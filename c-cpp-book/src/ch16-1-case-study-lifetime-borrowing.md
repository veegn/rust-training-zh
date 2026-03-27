# Case Study 3: Framework communication → Lifetime borrowing / 案例研究 3：框架通信 → 生命周期借用

> **What you'll learn / 你将学到：** How to convert C++ raw-pointer framework communication patterns to Rust's lifetime-based borrowing system, eliminating dangling pointer risks while maintaining zero-cost abstractions.
>
> 如何将 C++ 的裸指针框架通信模式转换为 Rust 基于生命周期的借用系统，在保持零成本抽象的同时消除悬空指针风险。

## The C++ Pattern: Raw Pointer to Framework / C++ 模式：指向框架的裸指针

```cpp
// C++ original: Every diagnostic module stores a raw pointer to the framework
// C++ 原版：每个诊断模块都存储一个指向框架的裸指针
class DiagBase {
protected:
    DiagFramework* m_pFramework;  // Raw pointer / 裸指针 —— 谁拥有它？
public:
    DiagBase(DiagFramework* fw) : m_pFramework(fw) {}
    
    void LogEvent(uint32_t code, const std::string& msg) {
        m_pFramework->GetEventLog()->Record(code, msg);  // Hope alive / 希望它还活着！
    }
};
// Problem / 问题：m_pFramework 是一个没有生命周期保证的裸指针。
// 如果框架被销毁而模块仍在引用它 → 未定义行为（UB）。
```

## The Rust Solution: DiagContext with Lifetime Borrowing / Rust 解决方案：带生命周期借用的 DiagContext

```rust
// Example: module.rs — Borrow, don't store / 示例：借用，不要存储

/// Context passed during execution / 执行期间传递给诊断模块的上下文。
/// The lifetime 'a / 生命周期 'a 保证框架的存活时间长于上下文。
pub struct DiagContext<'a> {
    pub der_log: &'a mut EventLogManager,
    pub config: &'a ModuleConfig,
    pub framework_opts: &'a HashMap<String, String>,
}

/// Modules receive context / 模块通过参数接收上下文 —— 绝不存储框架指针
pub trait DiagModule {
    fn id(&self) -> &str;
    fn execute(&mut self, ctx: &mut DiagContext) -> DiagResult<()>;
    fn pre_execute(&mut self, _ctx: &mut DiagContext) -> DiagResult<()> { Ok(()) }
    fn post_execute(&mut self, _ctx: &mut DiagContext) -> DiagResult<()> { Ok(()) }
}
```

### Key Insight / 核心洞察

- **C++ modules store a pointer** to the framework (danger: what if the framework is destroyed first?).
- **Rust modules receive a context** as a function parameter — the borrow checker guarantees the framework is alive during the call.
- **No raw pointers**, no lifetime ambiguity, no "hope it's still alive" hidden risks.

---

# Case Study 4: God object → Composable state / 案例研究 4：上帝对象 → 可组合状态

## The C++ Pattern: Monolithic Framework Class / C++ 模式：庞大的单体框架类

```cpp
// C++ original: The framework is god object
// C++ 原版：框架是一个“上帝对象（God Object）”
class DiagFramework {
    // Health-monitor / 健康监测 Trap 处理
    std::vector<AlertTriggerInfo> m_alertTriggers;
    std::vector<WarnTriggerInfo> m_warnTriggers;
    bool m_healthMonHasBootTimeError;
    uint32_t m_healthMonActionCounter;
    
    // GPU diagnostics / GPU 诊断
    std::map<uint32_t, GpuPcieInfo> m_gpuPcieMap;
    bool m_isRecoveryContext;
    bool m_healthcheckDetectedDevices;
    // ... 30+ more GPU fields / 还有 30 多个 GPU 相关字段
    
    // PCIe tree / PCIe 树
    std::shared_ptr<CPcieTreeLinux> m_pPcieTree;
    
    // Event logging / 事件日志
    CEventLogMgr* m_pEventLogMgr;
    
    // Everything depends on everything / 牵一发而动全身
};
```

## The Rust Solution: Composable State Structs / Rust 解决方案：可组合状态结构体

```rust
// Example: main.rs — Decomposed state / 示例：分解为聚焦的小型结构体

#[derive(Default)] // 派生 Default
struct HealthMonitorState {
    alert_triggers: Vec<AlertTriggerInfo>,
    warn_triggers: Vec<WarnTriggerInfo>,
    health_monitor_action_counter: u32,
    health_monitor_has_boot_time_error: bool,
    // Only health-monitor fields / 仅包含健康监测相关字段
}

#[derive(Default)]
struct GpuDiagState {
    gpu_pcie_map: HashMap<u32, GpuPcieInfo>,
    is_recovery_context: bool,
    healthcheck_detected_devices: bool,
    // Only GPU fields / 仅包含 GPU 相关字段
}

/// Framework composes states / 框架组合这些状态，而不是扁平化地拥有全部字段
struct DiagFramework {
    ctx: DiagContext,             // Context / 执行上下文
    args: Args,                   // Args / 命令行参数
    pcie_tree: Option<DeviceTree>,  // Tree / 不需要 shared_ptr
    event_log_mgr: EventLogManager,   // Log / 拥有所有权的管理器，而非裸指针
    fc_manager: FcManager,        // Fault code management
    health: HealthMonitorState,   // Health / 独立的子结构体
    gpu: GpuDiagState,           // GPU / 独立的子结构体
}
```

### Key Insight / 核心洞察

- **Testability**: Each state struct can be unit-tested independently.
- **Readability**: `self.health.alert_triggers` vs `m_alertTriggers` — clear ownership.
- **Fearless refactoring**: Changing `GpuDiagState` can't accidentally affect health-monitor processing.
- **No "method soup"**: Functions that only need health-monitor state take `&mut HealthMonitorState`, not the entire framework.

---

# Case Study 5: Trait objects — when they ARE right / 案例研究 5：Trait 对象 —— 何时它们才是正确的选择

*Not everything should be an enum! The **diagnostic module plugin system** is a genuine use case for trait objects because diagnostic modules are **open for extension** — new modules can be added without modifying the framework.*

并非所有东西都该用枚举！**诊断模块插件系统**是 Trait 对象的真实应用场景，因为诊断模块需要“**对扩展开放**” —— 可以在不修改框架的前提下添加新模块。

```rust
// Example: framework.rs — Box<dyn> is correct / 示例：此处的 Box<dyn DiagModule> 是正确的
pub struct DiagFramework {
    modules: Vec<Box<dyn DiagModule>>,        // Runtime poly / 运行时多态
    pre_diag_modules: Vec<Box<dyn DiagModule>>,
    event_log_mgr: EventLogManager,
}

impl DiagFramework {
    /// Register module / 注册诊断模块 —— 任何实现了 DiagModule 的类型
    pub fn register_module(&mut self, module: Box<dyn DiagModule>) {
        info!("Registering module: {}", module.id());
        self.modules.push(module);
    }
}
```

### When to Use Each Pattern / 各模式适用场景

| **Use Case / 使用场景** | **Pattern / 模式** | **Why / 理由** |
|-------------|-----------|--------|
| Fixed known at compile time / 编译时确定的固定变体集 | `enum` + `match` | Exhaustive / 穷尽性检查，无虚表 |
| HW events / 硬件事件类型 (Degrade, Fatal, ...) | `enum GpuEventKind` | Performance / 所有变体已知，性能至关重要 |
| PCIe devices / PCIe 设备类型 (GPU, NIC, ...) | `enum PcieDeviceKind` | Fixed set / 固定集合，各变体数据结构不同 |
| Plugins/modules / 插件/模块系统 (对扩展开放) | `Box<dyn Trait>` | Extension / 可在不改动框架的情况下添加新模块 |
| Test mocking / 测试 Mock | `Box<dyn Trait>` | Test doubles / 注入测试替身 |

---

### Exercise: Think Before You Translate / 练习：翻译前的思考

Given this C++ code:
```cpp
class Shape { public: virtual double area() = 0; };
class Circle : public Shape { double r; double area() override { return 3.14*r*r; } };
class Rect : public Shape { double w, h; double area() override { return w*h; } };
std::vector<std::unique_ptr<Shape>> shapes;
```
**Question**: Should the Rust translation use `enum Shape` or `Vec<Box<dyn Shape>>`?
**问题**：Rust 翻译应该使用 `enum Shape` 还是 `Vec<Box<dyn Shape>>`？

<details><summary>Solution / 解决方案（点击展开）</summary>

**Answer**: `enum Shape` — because the set of shapes is **closed** (known at compile time). You'd only use `Box<dyn Shape>` if users could add new shape types at runtime.

```rust
// Correct Rust translation / 正确的 Rust 翻译：
enum Shape {
    Circle { r: f64 },
    Rect { w: f64, h: f64 },
}

impl Shape {
    fn area(&self) -> f64 {
        match self {
            Shape::Circle { r } => std::f64::consts::PI * r * r,
            Shape::Rect { w, h } => w * h,
        }
    }
}
```
</details>

---

# Translation metrics and lessons learned / 翻译指标与经验教训总结

## What We Learned / 我们的收获
1. **Default to enum dispatch**: In ~100K lines of C++, only ~25 uses of `Box<dyn Trait>` were genuinely needed (plugin systems, test mocks).
2. **Arena pattern eliminates reference cycles**: `shared_ptr` and `enable_shared_from_this` are symptoms of unclear ownership.
3. **Pass context, don't store pointers**: Lifetime-bounded `DiagContext<'a>` is safer and clearer than storing `Framework*` in every module.
4. **Decompose god objects**: If a struct has 30+ fields, it's probably 3-4 structs wearing a trenchcoat.
5. **The compiler is your pair programmer**: Zero `dynamic_cast` equivalents in Rust means zero runtime type errors.

## The Hardest Parts / 最艰难的部分
- **Lifetime annotations**: Getting borrows right takes time when you're used to raw pointers — but once it compiles, it's correct.
- **Fighting the borrow checker**: Wanting `&mut self` in two places at once. Solution: decompose state into separate structs.
- **Resisting literal translation**: The temptation to write `Vec<Box<dyn Base>>` everywhere. Ask: "Is this set of variants closed?"

## Recommendation for C++ Teams / 给 C++ 团队的建议
1. Start with a small, self-contained module (not the god object).
2. Translate data structures first, then behavior.
3. Let the compiler guide you — its error messages are excellent.
4. Reach for `enum` before `dyn Trait`.
5. Use the [Rust playground](https://play.rust-lang.org/) to prototype patterns before integrating.
