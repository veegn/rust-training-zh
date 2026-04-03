[English Original](../en/ch16-1-case-study-lifetime-borrowing.md)

# 案例研究 3：框架通信 → 生命周期借用

> **你将学到：** 如何将 C++ 的原始指针框架通信模式转换为 Rust 基于生命周期的借用系统，在保持零成本抽象的同时彻底消除悬垂指针风险。

## C++ 模式：指向框架的原始指针
```cpp
// C++ 原始代码：每个诊断模块都存储一个指向框架的原始指针
class DiagBase {
protected:
    DiagFramework* m_pFramework;  // 原始指针 —— 谁拥有它？
public:
    DiagBase(DiagFramework* fw) : m_pFramework(fw) {}
    
    void LogEvent(uint32_t code, const std::string& msg) {
        m_pFramework->GetEventLog()->Record(code, msg);  // 希望它还活着！
    }
};
// 问题：m_pFramework 是一个没有生命周期保证的原始指针。
// 如果框架在模块仍引用它时被销毁，将会导致未定义行为 (UB)。
```

## Rust 解决方案：带有生命周期借用的 DiagContext
```rust
// 示例：module.rs —— 借用，而不存储

/// 执行期间传递给诊断模块的上下文。
/// 生命周期 'a 保证了框架的存续时间长于该上下文。
pub struct DiagContext<'a> {
    pub der_log: &'a mut EventLogManager,
    pub config: &'a ModuleConfig,
    pub framework_opts: &'a HashMap<String, String>,
}

/// 模块将上下文作为参数接收 —— 绝不存储框架指针
pub trait DiagModule {
    fn id(&self) -> &str;
    fn execute(&mut self, ctx: &mut DiagContext) -> DiagResult<()>;
    fn pre_execute(&mut self, _ctx: &mut DiagContext) -> DiagResult<()> {
        Ok(())
    }
    fn post_execute(&mut self, _ctx: &mut DiagContext) -> DiagResult<()> {
        Ok(())
    }
}
```

### 关键洞察
- C++ 模块**存储**一个指向框架的指针（危险：如果框架先被销毁了怎么办？）。
- Rust 模块**接收**一个作为函数参数的上下文 —— 借用检查器保证了框架在调用期间是存活的。
- 无原始指针、无生命周期歧义、不再需要“祈祷它还活着”。

---

# 案例研究 4：上帝对象 → 可组合的状态

## C++ 模式：整体式框架类
```cpp
// C++ 原始代码：框架即为上帝对象 (God Object)
class DiagFramework {
    // 监控器陷阱处理 (Health-monitor trap processing)
    std::vector<AlertTriggerInfo> m_alertTriggers;
    std::vector<WarnTriggerInfo> m_warnTriggers;
    bool m_healthMonHasBootTimeError;
    uint32_t m_healthMonActionCounter;
    
    // GPU 诊断
    std::map<uint32_t, GpuPcieInfo> m_gpuPcieMap;
    bool m_isRecoveryContext;
    bool m_healthcheckDetectedDevices;
    // ... 30 多个其他 GPU 相关的字段
    
    // PCIe 树
    std::shared_ptr<CPcieTreeLinux> m_pPcieTree;
    
    // 事件日志
    CEventLogMgr* m_pEventLogMgr;
    
    // ... 一系列其他方法
    void HandleGpuEvents();
    void HandleNicEvents();
    void RunGpuDiag();
    // 一切都依赖于一切
};
```

## Rust 解决方案：可组合的状态结构体
```rust
// 示例：main.rs —— 状态被分解为聚焦的子结构体

#[derive(Default)]
struct HealthMonitorState {
    alert_triggers: Vec<AlertTriggerInfo>,
    warn_triggers: Vec<WarnTriggerInfo>,
    health_monitor_action_counter: u32,
    health_monitor_has_boot_time_error: bool,
    // 仅包含监控器相关的字段
}

#[derive(Default)]
struct GpuDiagState {
    gpu_pcie_map: HashMap<u32, GpuPcieInfo>,
    is_recovery_context: bool,
    healthcheck_detected_devices: bool,
    // 仅包含 GPU 相关的字段
}

/// 框架负责组合这些状态，而不是将其全盘扁平化地堆叠在一起
struct DiagFramework {
    ctx: DiagContext,             // 执行上下文
    args: Args,                   // 命令行参数 (CLI)
    pcie_tree: Option<DeviceTree>,  // 无需 shared_ptr
    event_log_mgr: EventLogManager,   // 所有权类型，而非原始指针
    fc_manager: FcManager,        // 故障代码管理
    health: HealthMonitorState,   // 监控状态 —— 分立的结构体
    gpu: GpuDiagState,           // GPU 状态 —— 分立的结构体
}
```

### 关键洞察
- **可测试性**：每个状态结构体都可以独立地进行单元测试。
- **可读性**：`self.health.alert_triggers` 对比 `m_alertTriggers` —— 所有权关系更加清晰。
- **大胆重构**：修改 `GpuDiagState` 不会意外地影响到监控状态的处理逻辑。
- **避免臃肿的方法库**：仅需要监控状态的函数仅携带 `&mut HealthMonitorState` 作为参数，而无需整个框架。

---

# 案例研究 5：Trait 对象 —— 它们在何时是正确的

- 并不是所有的东西都应该是枚举！**诊断模块插件系统**就是一个真正需要使用 Trait 对象的案例。
- 为什么？因为诊断模块需要对**扩展开放** —— 开发者可以在不修改框架核心代码的情况下添加新的模块。

```rust
// 示例：framework.rs —— 在此处使用 Vec<Box<dyn DiagModule>> 是正确的
pub struct DiagFramework {
    modules: Vec<Box<dyn DiagModule>>,        // 运行时多态
    pre_diag_modules: Vec<Box<dyn DiagModule>>,
    event_log_mgr: EventLogManager,
    // ...
}

impl DiagFramework {
    /// 注册诊断模块 —— 任何实现了 DiagModule trait 的类型
    pub fn register_module(&mut self, module: Box<dyn DiagModule>) {
        info!("正在注册模块: {}", module.id());
        self.modules.push(module);
    }
}
```

### 何时使用何种模式

| **用例** | **模式** | **原因** |
|-------------|-----------|--------|
| 编译器已知的固定变体集合 | `enum` + `match` | 完备性检查，无虚函数表开销 |
| 硬件事件类型（降级、致命、引导……） | `enum GpuEventKind` | 所有变体均为已知，注重性能 |
| PCIe 设备类型（GPU、网卡、交换机……） | `enum PcieDeviceKind` | 集合固定，每个变体包含不同数据 |
| 插件/模块系统（针对扩展开放） | `Box<dyn Trait>` | 无需修改框架即可添加新模块 |
| 测试模拟 (Mocking) | `Box<dyn Trait>` | 注入测试替身 |

---

### 练习：翻译前的思考
给定如下 C++ 代码：
```cpp
class Shape { public: virtual double area() = 0; };
class Circle : public Shape { double r; double area() override { return 3.14*r*r; } };
class Rect : public Shape { double w, h; double area() override { return w*h; } };
std::vector<std::unique_ptr<Shape>> shapes;
```
**问题**：在 Rust 翻译中应该使用 `enum Shape` 还是 `Vec<Box<dyn Shape>>`？

<details><summary>答案 (点击展开)</summary>

**答案**：应该使用 `enum Shape` —— 因为形状的集合是**封闭的**（在编译时已知）。只有当用户可以在运行时添加新的形状类型时，才需要使用 `Box<dyn Shape>`。

```rust
// 正确的 Rust 翻译：
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

fn main() {
    let shapes: Vec<Shape> = vec![
        Shape::Circle { r: 5.0 },
        Shape::Rect { w: 3.0, h: 4.0 },
    ];
    for shape in &shapes {
        println!("面积: {:.2}", shape.area());
    }
}
// 输出：
// 面积: 78.54
// 面积: 12.00
```

</details>

---

# 迁移指标与经验总结

## 我们的收获
1. **优先使用枚举分发** —— 在 10 万行 C++ 代码中，仅有约 25 处真正需要使用 `Box<dyn Trait>`（如插件系统、测试模拟）。其余约 900 处虚函数都转为了带有模式匹配的枚举。
2. **Arena 模式消除了引用循环** —— `shared_ptr` 和 `enable_shared_from_this` 是权责不明的所有权的典型症状。首先思考谁**拥有**数据。
3. **传递上下文，而非存储指针** —— 带有生命周期限制的 `DiagContext<'a>` 比在每个模块中存储 `Framework*` 原始指针更加安全且清晰。
4. **分解上帝对象** —— 如果一个结构体拥有 30 多个字段，它很可能是三四个结构体套在一起的产物。
5. **编译器是你的结对编程伙伴** —— 约 400 次 `dynamic_cast` 调用意味着约 400 次潜在的运行时失败。在 Rust 中实现零 `dynamic_cast` 等效项意味着零运行时类型错误。

## 最困难的部分
- **生命周期标注**：当你习惯了原始指针后，正确处理借用需要一些时间 —— 但一旦代码通过编译，它就是正确的。
- **与借用检查器“搏斗”**：想要在两处地方同时使用 `&mut self`。解决方案：将状态分解为独立的结构体。
- **抵制生搬硬套式的翻译**：容易禁不住诱惑在各处都写上 `Vec<Box<dyn Base>>`。问问自己：“这个变体的集合是封闭的吗？” → 如果是，请使用枚举。

## 给 C++ 团队的建议
1. 从一个小型的、自包含的模块开始（不要从上帝对象开始）。
2. 先翻译数据结构，再翻译行为。
3. 让编译器引导你 —— 它的错误信息非常出色。
4. 在使用 `dyn Trait` 之前，先行尝试使用 `enum`。
5. 在集成之前，使用 [Rust playground](https://play.rust-lang.org/) 对模式进行原型设计。

---
