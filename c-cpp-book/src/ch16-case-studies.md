# Case Study Overview: C++ to Rust Translation / 案例研究概览：C++ 到 Rust 的迁移

> **What you'll learn / 你将学到：** Lessons from a real-world translation of ~100K lines of C++ to ~90K lines of Rust across ~20 crates. Five key transformation patterns and the architectural decisions behind them.
>
> 这是一个真实项目的经验总结：将约 10 万行 C++ 代码迁移到约 9 万行 Rust 代码（分布在约 20 个 crate 中）。我们将探讨五个核心转换模式及其背后的架构决策。

*We translated a large C++ diagnostic system (~100K lines of C++) into a Rust implementation (~20 Rust crates, ~90K lines). This section shows the **actual patterns** used — not toy examples, but real production code.*

我们将一个大型 C++ 诊断系统（约 10 万行）转换为了 Rust 实现（分布在约 20 个 crate 中，共约 9 万行）。本节展示的是**实际使用的模式** —— 不是玩具示例，而是真实的生产代码。

| **#** | **C++ Pattern / C++ 模式** | **Rust Pattern / Rust 模式** | **Impact / 影响** |
|-------|----------------|-----------------|-----------|
| 1 | Class hierarchy + `dynamic_cast` / 类继承 | Enum dispatch + `match` / 枚举分发 | ~400 → 0 dynamic_casts |
| 2 | `shared_ptr` / `enable_shared_from_this` tree | Arena + index linkage / Arena + 索引关联 | No cycles / 无引用循环 |
| 3 | `Framework*` raw pointer / 裸指针 | `DiagContext<'a>` with lifetime | Compile-time validity / 编译时有效性 |
| 4 | God object / 上帝对象 | Composable state structs / 可组合状态结构体 | Testable / 可测试、模块化 |
| 5 | `vector<unique_ptr<Base>>` | Trait objects only if needed / 仅在必要时使用 Trait 对象 | Static dispatch / 默认静态分发 |

---

### Before and After Metrics / 迁前迁后指标对比

| **Metric / 指标** | **C++ (Original / 原版)** | **Rust (Rewrite / 重写版)** |
|------------|---------------------|------------------------|
| `dynamic_cast` / type downcasts | ~400 | 0 |
| `virtual` / `override` methods | ~900 | ~25 (`Box<dyn Trait>`) |
| Raw `new` allocations / 裸 `new` 分配 | ~200 | 0 (all owned / 全所有权类型) |
| `shared_ptr` / reference counting | ~10 (topology lib) | 0 (`Arc` only at FFI / 仅在 FFI 边界) |
| `enum class` definitions | ~60 | ~190 `pub enum` |
| Pattern matching / 模式匹配表达式 | N/A | ~750 `match` |
| God objects / 上帝对象 (>5000 行) | 2 | 0 |

---

# Case Study 1: Inheritance hierarchy → Enum dispatch / 案例研究 1：继承体系 → 枚举分发

## The C++ Pattern: Event Class Hierarchy / C++ 模式：事件类继承体系

```cpp
// C++ original: Every GPU event type is a class inheriting from GpuEventBase
// C++ 原版：每种 GPU 事件类型都是继承自 GpuEventBase 的一个类
class GpuEventBase {
public:
    virtual ~GpuEventBase() = default;
    virtual void Process(DiagFramework* fw) = 0;
    uint16_t m_recordId;
    uint8_t  m_sensorType;
    // ... common fields / 公共字段
};

class GpuPcieDegradeEvent : public GpuEventBase {
public:
    void Process(DiagFramework* fw) override;
    uint8_t m_linkSpeed;
    uint8_t m_linkWidth;
};

class GpuPcieFatalEvent : public GpuEventBase { /* ... */ }; // 致命事件
class GpuBootEvent : public GpuEventBase { /* ... */ };      // 启动事件
// ... 10+ event classes / 还有 10 多个继承自 GpuEventBase 的事件类

// Processing requires dynamic_cast / 处理时需要 dynamic_cast：
void ProcessEvents(std::vector<std::unique_ptr<GpuEventBase>>& events,
                   DiagFramework* fw) {
    for (auto& event : events) {
        if (auto* degrade = dynamic_cast<GpuPcieDegradeEvent*>(event.get())) {
            // handle degrade / 处理降级...
        } else if (auto* fatal = dynamic_cast<GpuPcieFatalEvent*>(event.get())) {
            // handle fatal / 处理致命错误...
        }
    }
}
```

## The Rust Solution: Enum Dispatch / Rust 解决方案：枚举分发

```rust
// Example: types.rs — No inheritance, no vtable, no dynamic_cast / 示例：无继承、无虚表、无 dynamic_cast
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum GpuEventKind {
    PcieDegrade,
    PcieFatal,
    PcieUncorr,
    Boot,
    BaseboardState,
    EccError,
    OverTemp,
    PowerRail,
    ErotStatus,
    Unknown,
}
```

```rust
// Example: manager.rs — Separate typed Vecs, no downcasting / 示例：分离的类型化 Vec，理论上不需要向下转型
pub struct GpuEventManager {
    sku: SkuVariant,
    degrade_events: Vec<GpuPcieDegradeEvent>,   // Concrete type / 具体类型，而非 Box<dyn>
    fatal_events: Vec<GpuPcieFatalEvent>,
    uncorr_events: Vec<GpuPcieUncorrEvent>,
    boot_events: Vec<GpuBootEvent>,
    baseboard_events: Vec<GpuBaseboardEvent>,
    ecc_events: Vec<GpuEccEvent>,
    // ... each event type / 每个事件类型都有自己的 Vec
}

// Accessors return typed slices / 访问器返回具体类型的切片 —— 零歧义
impl GpuEventManager {
    pub fn degrade_events(&self) -> &[GpuPcieDegradeEvent] {
        &self.degrade_events
    }
    pub fn fatal_events(&self) -> &[GpuPcieFatalEvent] {
        &self.fatal_events
    }
}
```

### Why Not `Vec<Box<dyn GpuEvent>>`? / 为什么不使用 `Vec<Box<dyn GpuEvent>>`？

- **The Wrong Approach (literal translation) / 错误的方法（字面翻译）**: Put all events in one heterogeneous collection, then downcast — this is what C++ does with `vector<unique_ptr<Base>>`.
- **The Right Approach / 正确的方法**: Separate typed Vecs eliminate *all* downcasting. Each consumer asks for exactly the event type it needs.
- **Performance / 性能**: Separate Vecs give better cache locality (all degrade events are contiguous in memory).

---

# Case Study 2: shared_ptr tree → Arena/index pattern / 案例研究 2：shared_ptr 树 → Arena/索引模式

## The C++ Pattern: Reference-Counted Tree / C++ 模式：引用计数树

```cpp
// C++ topology library: PcieDevice uses enable_shared_from_this
// C++ 拓扑库：PcieDevice 使用 enable_shared_from_this
// because parent and child nodes both need to reference each other
// 因为父节点和子节点需要互相引用
class PcieDevice : public std::enable_shared_from_this<PcieDevice> {
public:
    std::shared_ptr<PcieDevice> m_upstream;
    std::vector<std::shared_ptr<PcieDevice>> m_downstream;
    // ... device data / 设备数据
    
    void AddChild(std::shared_ptr<PcieDevice> child) {
        child->m_upstream = shared_from_this();  // Parent ↔ child cycle / 父子引用循环！
        m_downstream.push_back(child);
    }
};
// Problem / 问题：父→子和子→父引用创建了引用循环。需要用 weak_ptr 来打破循环，但很容易忘记。
```

## The Rust Solution: Arena with Index Linkage / Rust 解决方案：带索引关联的 Arena

```rust
// Example: components.rs — Flat Vec owns all devices / 示例：扁平 Vec 拥有所有设备
pub struct PcieDevice {
    pub base: PcieDeviceBase,
    pub kind: PcieDeviceKind,

    // Tree linkage / 通过索引关联树 —— 无引用计数，无循环
    pub upstream_idx: Option<usize>,      // Index / Arena Vec 中的索引
    pub downstream_idxs: Vec<usize>,      // Indices / Arena Vec 中的索引
}

// The "arena" is simply a Vec<PcieDevice> owned by the tree / “arena” 本质上是树拥有的一个 Vec<PcieDevice>：
pub struct DeviceTree {
    devices: Vec<PcieDevice>,  // Flat ownership / 扁平化所有权 —— 一个 Vec 拥有所有内容
}

impl DeviceTree {
    pub fn parent(&self, device_idx: usize) -> Option<&PcieDevice> {
        self.devices[device_idx].upstream_idx
            .map(|idx| &self.devices[idx])
    }
    
    pub fn children(&self, device_idx: usize) -> Vec<&PcieDevice> {
        self.devices[device_idx].downstream_idxs
            .iter()
            .map(|&idx| &self.devices[idx])
            .collect()
    }
}
```

### Key Insight / 核心洞察

- **No `shared_ptr`, no `weak_ptr`, no `enable_shared_from_this`**.
- **No reference cycles possible** — indices are just `usize` values.
- **Better cache performance** — all devices in contiguous memory.
- **Simpler reasoning** — one owner (the Vec), many viewers (indices).

```mermaid
graph LR
    subgraph "C++ shared_ptr Tree / C++ shared_ptr 树"
        A1["shared_ptr<Device>"] -->|"shared_ptr"| B1["shared_ptr<Device>"]
        B1 -->|"shared_ptr (parent)"| A1
        A1 -->|"shared_ptr"| C1["shared_ptr<Device>"]
        C1 -->|"shared_ptr (parent)"| A1
        style A1 fill:#ff6b6b,color:#000
        style B1 fill:#ffa07a,color:#000
        style C1 fill:#ffa07a,color:#000
    end

    subgraph "Rust Arena + Index / Rust Arena + 索引"
        V["Vec<PcieDevice>"]
        V --> D0["[0] Root / 根<br/>upstream: None<br/>down: [1,2]"]
        V --> D1["[1] Child / 子<br/>upstream: Some(0)<br/>down: []"]
        V --> D2["[2] Child / 子<br/>upstream: Some(0)<br/>down: []"]
        style V fill:#51cf66,color:#000
        style D0 fill:#91e5a3,color:#000
        style D1 fill:#91e5a3,color:#000
        style D2 fill:#91e5a3,color:#000
    end
```
