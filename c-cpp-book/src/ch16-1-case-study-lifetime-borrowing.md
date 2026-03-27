# Case Study 3: Framework communication → Lifetime borrowing / 案例研究 3：框架通信 → 生命周期借用
 
 > **What you'll learn / 你将学到：** How to convert C++ raw-pointer framework communication patterns to Rust's lifetime-based borrowing system, eliminating dangling pointer risks while maintaining zero-cost abstractions.
 >
 > 如何将 C++ 的裸指针框架通信模式转换为 Rust 基于生命周期的借用系统，在保持零成本抽象的同时消除悬空指针风险。
 
- ## The C++ Pattern: Raw Pointer to Framework
+ ## The C++ Pattern: Raw Pointer to Framework / C++ 模式：指向框架的裸指针
 ```cpp
 // C++ original: Every diagnostic module stores a raw pointer to the framework
+// C++ 原版：每个诊断模块都存储一个指向框架的裸指针
 class DiagBase {
 protected:
-    DiagFramework* m_pFramework;  // Raw pointer — who owns this?
+    DiagFramework* m_pFramework;  // Raw pointer / 裸指针 —— 谁拥有它？
 public:
     DiagBase(DiagFramework* fw) : m_pFramework(fw) {}
     
     void LogEvent(uint32_t code, const std::string& msg) {
-        m_pFramework->GetEventLog()->Record(code, msg);  // Hope it's still alive!
+        m_pFramework->GetEventLog()->Record(code, msg);  // Hope alive / 希望它还活着！
     }
 };
- // Problem: m_pFramework is a raw pointer with no lifetime guarantee
+ // Problem / 问题：m_pFramework 是一个没有生命周期保证的裸指针
- // If framework is destroyed while modules still reference it → UB
+ // 如果框架被销毁而模块仍在引用它 → 未定义行为（UB）
 ```
 
- ## The Rust Solution: DiagContext with Lifetime Borrowing
+ ## The Rust Solution: DiagContext with Lifetime Borrowing / Rust 解决方案：带生命周期借用的 DiagContext
 ```rust
- // Example: module.rs — Borrow, don't store
+ // Example: module.rs — Borrow, don't store / 示例：借用，不要存储
 
- /// Context passed to diagnostic modules during execution.
+ /// Context passed during execution / 执行期间传递给诊断模块的上下文。
- /// The lifetime 'a guarantees the framework outlives the context.
+ /// The lifetime 'a / 生命周期 'a 保证框架的存活时间长于上下文。
 pub struct DiagContext<'a> {
     pub der_log: &'a mut EventLogManager,
     pub config: &'a ModuleConfig,
     pub framework_opts: &'a HashMap<String, String>,
 }
 
- /// Modules receive context as a parameter — never store framework pointers
+ /// Modules receive context / 模块通过参数接收上下文 —— 绝不存储框架指针
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
 
- ### Key Insight
+ ### Key Insight / 核心洞察
- - C++ modules **store** a pointer to the framework (danger: what if the framework is destroyed first?)
+ - C++ 模块**存储**一个指向框架的指针（危险：如果框架先被销毁了怎么办？）。
- - Rust modules **receive** a context as a function parameter — the borrow checker guarantees the framework is alive during the call
+ - Rust 模块通过函数参数**接收**上下文 —— 借用检查器保证框架在调用期间是存活的。
- - No raw pointers, no lifetime ambiguity, no "hope it's still alive"
+ - 没有裸指针，没有生命周期模糊性，也就没有“希望它还活着”这种隐患。
 
 ----
 
- # Case Study 4: God object → Composable state
+ # Case Study 4: God object → Composable state / 案例研究 4：上帝对象 → 可组合状态
 
- ## The C++ Pattern: Monolithic Framework Class
+ ## The C++ Pattern: Monolithic Framework Class / C++ 模式：庞大的单体框架类
 ```cpp
 // C++ original: The framework is god object
+// C++ 原版：框架是一个“上帝对象（God Object）”
 class DiagFramework {
-    // Health-monitor trap processing
+    // Health-monitor / 健康监测 Trap 处理
     std::vector<AlertTriggerInfo> m_alertTriggers;
     std::vector<WarnTriggerInfo> m_warnTriggers;
     bool m_healthMonHasBootTimeError;
     uint32_t m_healthMonActionCounter;
     
-    // GPU diagnostics
+    // GPU diagnostics / GPU 诊断
     std::map<uint32_t, GpuPcieInfo> m_gpuPcieMap;
     bool m_isRecoveryContext;
     bool m_healthcheckDetectedDevices;
-    // ... 30+ more GPU-related fields
+    // ... 30+ more GPU fields / 还有 30 多个 GPU 相关字段
     
-    // PCIe tree
+    // PCIe tree / PCIe 树
     std::shared_ptr<CPcieTreeLinux> m_pPcieTree;
     
-    // Event logging
+    // Event logging / 事件日志
     CEventLogMgr* m_pEventLogMgr;
     
-    // ... several other methods
+    // ... several other methods / 还有其他若干个方法
     void HandleGpuEvents();
     void HandleNicEvents();
     void RunGpuDiag();
-    // Everything depends on everything
+    // Everything depends on everything / 牵一发而动全身
 };
 ```
 
- ## The Rust Solution: Composable State Structs
+ ## The Rust Solution: Composable State Structs / Rust 解决方案：可组合状态结构体
 ```rust
- // Example: main.rs — State decomposed into focused structs
+ // Example: main.rs — Decomposed state / 示例：分解为聚焦的小型结构体
 
- #[derive(Default)]
+ #[derive(Default)] // 派生 Default
 struct HealthMonitorState {
     alert_triggers: Vec<AlertTriggerInfo>,
     warn_triggers: Vec<WarnTriggerInfo>,
     health_monitor_action_counter: u32,
     health_monitor_has_boot_time_error: bool,
-    // Only health-monitor-related fields
+    // Only health-monitor fields / 仅包含健康监测相关字段
 }
 
 #[derive(Default)]
 struct GpuDiagState {
     gpu_pcie_map: HashMap<u32, GpuPcieInfo>,
     is_recovery_context: bool,
     healthcheck_detected_devices: bool,
-    // Only GPU-related fields
+    // Only GPU fields / 仅包含 GPU 相关字段
 }
 
- /// The framework composes these states rather than owning everything flat
+ /// Framework composes states / 框架组合这些状态，而不是扁平化地拥有全部字段
 struct DiagFramework {
-    ctx: DiagContext,             // Execution context
+    ctx: DiagContext,             // Context / 执行上下文
-    args: Args,                   // CLI arguments
+    args: Args,                   // Args / 命令行参数
-    pcie_tree: Option<DeviceTree>,  // No shared_ptr needed
+    pcie_tree: Option<DeviceTree>,  // Tree / 不需要 shared_ptr
-    event_log_mgr: EventLogManager,   // Owned, not raw pointer
+    event_log_mgr: EventLogManager,   // Log / 拥有所有权的管理器，而非裸指针
     fc_manager: FcManager,        // Fault code management
-    health: HealthMonitorState,   // Health-monitor state — its own struct
+    health: HealthMonitorState,   // Health / 独立的子结构体
-    gpu: GpuDiagState,           // GPU state — its own struct
+    gpu: GpuDiagState,           // GPU / 独立的子结构体
 }
 ```
 
- ### Key Insight
+ ### Key Insight / 核心洞察
- - **Testability**: Each state struct can be unit-tested independently
+ - **可测试性**：每个状态结构体都可以独立进行单元测试。
- - **Readability**: `self.health.alert_triggers` vs `m_alertTriggers` — clear ownership
+ - **可读性**：`self.health.alert_triggers` 与 `m_alertTriggers` 相比，所有权关系更清晰。
- - **Fearless refactoring**: Changing `GpuDiagState` can't accidentally affect health-monitor processing
+ - **无畏重构**：修改 `GpuDiagState` 不会意外影响到健康监测的处理流程。
- - **No method soup**: Functions that only need health-monitor state take `&mut HealthMonitorState`, not the entire framework
+ - **没有“方法大杂烩”**：只需要健康监测状态的函数只需接收 `&mut HealthMonitorState`，而不是整个框架对象。
 
 ----
 
- # Case Study 5: Trait objects — when they ARE right
+ # Case Study 5: Trait objects — when they ARE right / 案例研究 5：Trait 对象 —— 何时它们才是正确的选择
 
- - Not everything should be an enum! The **diagnostic module plugin system** is a genuine use case for trait objects
+ - 并非所有东西都该用枚举！**诊断模块插件系统**是 Trait 对象的真实应用场景。
- - Why? Because diagnostic modules are **open for extension** — new modules can be added without modifying the framework
+ - 为什么？因为诊断模块需要“**对扩展开放**” —— 可以在不修改框架的前提下添加新模块。
 
 ```rust
- // Example: framework.rs — Vec<Box<dyn DiagModule>> is correct here
+ // Example: framework.rs — Box<dyn> is correct / 示例：此处的 Box<dyn DiagModule> 是正确的
 pub struct DiagFramework {
-    modules: Vec<Box<dyn DiagModule>>,        // Runtime polymorphism
+    modules: Vec<Box<dyn DiagModule>>,        // Runtime poly / 运行时多态
     pre_diag_modules: Vec<Box<dyn DiagModule>>,
     event_log_mgr: EventLogManager,
-    // ...
+    // // ...
 }
 
 impl DiagFramework {
-    /// Register a diagnostic module — any type implementing DiagModule
+    /// Register module / 注册诊断模块 —— 任何实现了 DiagModule 的类型
     pub fn register_module(&mut self, module: Box<dyn DiagModule>) {
         info!("Registering module: {}", module.id());
         self.modules.push(module);
     }
 }
 ```
 
- ### When to Use Each Pattern
+ ### When to Use Each Pattern / 各模式适用场景
 
-| **Use Case** | **Pattern** | **Why** |
+| **Use Case / 使用场景** | **Pattern / 模式** | **Why / 理由** |
 |-------------|-----------|--------|
-| Fixed set of variants known at compile time | `enum` + `match` | Exhaustive checking, no vtable |
+| Fixed known at compile time / 编译时确定的固定变体集 | `enum` + `match` | Exhaustive / 穷尽性检查，无虚表 |
-| Hardware event types (Degrade, Fatal, Boot, ...) | `enum GpuEventKind` | All variants known, performance matters |
+| HW events / 硬件事件类型 (Degrade, Fatal, ...) | `enum GpuEventKind` | Performance / 所有变体已知，性能至关重要 |
-| PCIe device types (GPU, NIC, Switch, ...) | `enum PcieDeviceKind` | Fixed set, each variant has different data |
+| PCIe devices / PCIe 设备类型 (GPU, NIC, ...) | `enum PcieDeviceKind` | Fixed set / 固定集合，各变体数据结构不同 |
-| Plugin/module system (open for extension) | `Box<dyn Trait>` | New modules added without modifying framework |
+| Plugins/modules / 插件/模块系统 (对扩展开放) | `Box<dyn Trait>` | Extension / 可在不改动框架的情况下添加新模块 |
-| Test mocking | `Box<dyn Trait>` | Inject test doubles |
+| Test mocking / 测试 Mock | `Box<dyn Trait>` | Test doubles / 注入测试替身 |
 
- ### Exercise: Think Before You Translate
+ ### Exercise: Think Before You Translate / 练习：翻译前的思考
- Given this C++ code:
+ 给定以下 C++ 代码：
 ```cpp
 class Shape { public: virtual double area() = 0; };
 class Circle : public Shape { double r; double area() override { return 3.14*r*r; } };
 class Rect : public Shape { double w, h; double area() override { return w*h; } };
 std::vector<std::unique_ptr<Shape>> shapes;
 ```
- **Question**: Should the Rust translation use `enum Shape` or `Vec<Box<dyn Shape>>`?
+ **问题**：Rust 翻译应该使用 `enum Shape` 还是 `Vec<Box<dyn Shape>>`？
 
- <details><summary>Solution (click to expand)</summary>
+ <details><summary>Solution (click to expand) / 解决方案（点击展开）</summary>
 
- **Answer**: `enum Shape` — because the set of shapes is **closed** (known at compile time). You'd only use `Box<dyn Shape>` if users could add new shape types at runtime.
+ **答案**：`enum Shape` —— 因为形状的集合是**封闭的**（在编译时已知）。只有当用户可以在运行时添加新的形状类型时，才会使用 `Box<dyn Shape>`。
 
 ```rust
- // Correct Rust translation:
+ // Correct Rust translation / 正确的 Rust 翻译：
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
         println!("Area: {:.2}", shape.area());
     }
 }
- // Output:
+ // Output / 输出：
 // Area: 78.54
 // Area: 12.00
 ```
 
 </details>
 
 ----
 
- # Translation metrics and lessons learned
+ # Translation metrics and lessons learned / 翻译指标与经验教训总结
 
- ## What We Learned
+ ## What We Learned / 我们的收获
- 1. **Default to enum dispatch** — In ~100K lines of C++, only ~25 uses of `Box<dyn Trait>` were genuinely needed (plugin systems, test mocks). The other ~900 virtual methods became enums with match
+ 1. **默认使用枚举分发** —— 在约 10 万行 C++ 代码中，只有约 25 处真正需要 `Box<dyn Trait>`（插件系统、测试 Mock）。其余约 900 个虚方法都转换成了带 match 的枚举。
- 2. **Arena pattern eliminates reference cycles** — `shared_ptr` and `enable_shared_from_this` are symptoms of unclear ownership. Think about who **owns** the data first
+ 2. **Arena 模式消除了引用循环** —— `shared_ptr` 和 `enable_shared_from_this` 是所有权关系不明确的征兆。请先思考谁才是数据的**拥有者**。
- 3. **Pass context, don't store pointers** — Lifetime-bounded `DiagContext<'a>` is safer and clearer than storing `Framework*` in every module
+ 3. **传递上下文，而不是存储指针** —— 带生命周期限定的 `DiagContext<'a>` 比在每个模块中存储一个 `Framework*` 更安全、更清晰。
- 4. **Decompose god objects** — If a struct has 30+ fields, it's probably 3-4 structs wearing a trenchcoat
+ 4. **分解上帝对象** —— 如果一个结构体有 30 多个字段，它大概率是三四个小结构体套在了一件大衣里。
- 5. **The compiler is your pair programmer** — ~400 `dynamic_cast` calls meant ~400 potential runtime failures. Zero `dynamic_cast` equivalents in Rust means zero runtime type errors
+ 5. **编译器是你的结对程序员** —— 约 400 处 `dynamic_cast` 调用意味着 400 处潜在的运行时失败。而 Rust 中零等效项意味着零运行时类型错误。
 
- ## The Hardest Parts
+ ## The Hardest Parts / 最艰难的部分
- - **Lifetime annotations**: Getting borrows right takes time when you're used to raw pointers — but once it compiles, it's correct
+ - **生命周期标注**：当你习惯了裸指针时，正确处理借用需要一些时间 —— 但一旦编过，它就是正确的。
- - **Fighting the borrow checker**: Wanting `&mut self` in two places at once. Solution: decompose state into separate structs
+ - **与借用检查器“搏斗”**：想在两处同时持有 `&mut self`。解决方案：将状态分解为独立的结构体。
- - **Resisting literal translation**: The temptation to write `Vec<Box<dyn Base>>` everywhere. Ask: "Is this set of variants closed?" → If yes, use enum
+ - **抵制字面翻译**：到处都想写 `Vec<Box<dyn Base>>` 的诱惑。请自问：“这个变体集是封闭的吗？” → 如果是，请用枚举。
 
- ## Recommendation for C++ Teams
+ ## Recommendation for C++ Teams / 给 C++ 团队的建议
- 1. Start with a small, self-contained module (not the god object)
+ 1. 从小型、自包含的模块开始（不要从上帝对象开始）。
- 2. Translate data structures first, then behavior
+ 2. 先翻译数据结构，再翻译行为。
- 3. Let the compiler guide you — its error messages are excellent
+ 3. 让编译器引导你 —— 它的错误提示非常出色。
- 4. Reach for `enum` before `dyn Trait`
+ 4. 在考虑 `dyn Trait` 之前，先考虑 `enum`。
- 5. Use the [Rust playground](https://play.rust-lang.org/) to prototype patterns before integrating
+ 5. 在集成之前，使用 [Rust Playground](https://play.rust-lang.org/) 对模式进行原型设计。
