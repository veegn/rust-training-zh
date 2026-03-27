## Avoiding excessive clone() / 避免过度使用 clone()
 
 > **What you'll learn / 你将学到：** Why `.clone()` is a code smell in Rust, how to restructure ownership to eliminate unnecessary copies, and the specific patterns that signal an ownership design problem.
 >
 > 为什么 `.clone()` 在 Rust 中被视为一种“代码坏味道”，如何重组所有权以消除不必要的拷贝，以及哪些特定模式预示着所有权设计存在问题。
 
- - Coming from C++, `.clone()` feels like a safe default — "just copy it". But excessive cloning hides ownership problems and hurts performance.
+ - 对于 C++ 开发者来说，`.clone()` 听起来像是一个安全的默认选择 —— “拷贝一份就行”。但过度的克隆会掩盖所有权问题并损害性能。
- - **Rule of thumb**: If you're cloning to satisfy the borrow checker, you probably need to restructure ownership instead.
+ - **经验法则**：如果你克隆是为了满足借用检查器，那么你可能需要重组所有权结构。
 
- ### When clone() is wrong
+ ### When clone() is wrong / 何时使用 clone() 是错误的
 
 ```rust
- // BAD: Cloning a String just to pass it to a function that only reads it
+ // BAD: Cloning a String / 坏习惯：为了传参给只读函数而克隆 String
 fn log_message(msg: String) {  // Takes ownership unnecessarily
     println!("[LOG] {}", msg);
 }
 let message = String::from("GPU test passed");
- log_message(message.clone());  // Wasteful: allocates a whole new String
+ log_message(message.clone());  // Wasteful / 浪费：分配了一个全新的 String
- log_message(message);           // Original consumed — clone was pointless
+ log_message(message);           // Original consumed / 原变量被消耗 —— 克隆毫无意义
 ```
 
 ```rust
- // GOOD: Accept a borrow — zero allocation
+ // GOOD: Accept a borrow / 好习惯：接收借用 —— 零分配
 fn log_message(msg: &str) {    // Borrows, doesn't own
     println!("[LOG] {}", msg);
 }
 let message = String::from("GPU test passed");
- log_message(&message);          // No clone, no allocation
+ log_message(&message);          // No clone / 无克隆，无分配
- log_message(&message);          // Can call again — message not consumed
+ log_message(&message);          // Can call again / 可以再次调用 —— 消息未被消耗
 ```
 
- ### Real example: returning `&str` instead of cloning
+ ### Real example: returning `&str` instead of cloning / 真实示例：返回 `&str` 而非克隆
 ```rust
- // Example: healthcheck.rs — returns a borrowed view, zero allocation
+ // Example: healthcheck.rs — returns borrowed view / 示例：返回借用视图，零分配
 pub fn serial_or_unknown(&self) -> &str {
     self.serial.as_deref().unwrap_or(UNKNOWN_VALUE)
 }
 
 pub fn model_or_unknown(&self) -> &str {
     self.model.as_deref().unwrap_or(UNKNOWN_VALUE)
 }
 ```
- The C++ equivalent would return `const std::string&` or `std::string_view` — but in C++ neither is lifetime-checked. In Rust, the borrow checker guarantees the returned `&str` can't outlive `self`.
+ C++ 的对应做法是返回 `const std::string&` 或 `std::string_view` —— 但在 C++ 中，这两者都没有经过生命周期检查。而在 Rust 中，借用检查器保证返回的 `&str` 存活时间不会超过 `self`。
 
- ### Real example: static string slices — no heap at all
+ ### Real example: static string slices — no heap at all / 真实示例：静态字符串切片 —— 完全不使用堆
 ```rust
- // Example: healthcheck.rs — compile-time string tables
+ // Example: healthcheck.rs — string tables / 示例：编译时字符串表
 const HBM_SCREEN_RECIPES: &[&str] = &[
     "hbm_ds_ntd", "hbm_ds_ntd_gfx", "hbm_dt_ntd", "hbm_dt_ntd_gfx",
     "hbm_burnin_8h", "hbm_burnin_24h",
 ];
 ```
- In C++ this would typically be `std::vector<std::string>` (heap-allocated on first use). Rust's `&'static [&'static str]` lives in read-only memory — zero runtime cost.
+ 在 C++ 中，这通常是 `std::vector<std::string>`（首次使用时在堆上分配）。Rust 的 `&'static [&'static str]` 存储在只读内存中 —— 运行时成本为零。
 
- ### When clone() IS appropriate
+ ### When clone() IS appropriate / 何时使用 clone() 是合理的
 
-| **Situation** | **Why clone is OK** | **Example** |
+| **Situation / 场景** | **Why clone is OK / 理由** | **Example / 示例** |
 |--------------|--------------------|-----------|
-| `Arc::clone()` for threading | Bumps ref count (~1 ns), doesn't copy data | `let flag = stop_flag.clone();` |
-| `Arc::clone()` for threading | Bumps ref count / 增加引用计数 (~1 ns)，不拷贝数据 | `let flag = Arc::clone(&flag);` |
-| Moving data into a spawned thread | Thread needs its own copy | `let ctx = ctx.clone(); thread::spawn(move \|\| { ... })` |
-| Moving data into thread | Thread needs copy / 线程需要自己的副本 | `let ctx = ctx.clone(); thread::spawn(...)` |
-| Extracting from `&self` fields | Can't move out of a borrow | `self.name.clone()` when returning owned `String` |
-| Extracting from fields / 从字段中提取 | Can't move out / 无法从借用中移除所有权 | 返回拥有所有权的 `String` 时 |
-| Small `Copy` types wrapped in `Option` | `.copied()` is clearer than `.clone()` | `opt.get(0).copied()` for `Option<&u32>` → `Option<u32>` |
-| Small `Copy` types in `Option` | `.copied()` is clearer / 更清晰 | 将 `Option<&u32>` 转换为 `Option<u32>` |
 
- ### Real example: Arc::clone for thread sharing
+ ### Real example: Arc::clone for thread sharing / 真实示例：用于线程间共享的 Arc::clone
 ```rust
- // Example: workload.rs — Arc::clone is cheap (ref count bump)
+ // Example: workload.rs — Arc::clone is cheap / 示例：Arc::clone 很廉价（仅增加引用计数）
 let stop_flag = Arc::new(AtomicBool::new(false));
- let stop_flag_clone = stop_flag.clone();   // ~1 ns, no data copied
+ let stop_flag_clone = stop_flag.clone();   // ~1 ns, no data copied / 不拷贝数据
- let ctx_clone = ctx.clone();               // Clone context for move into thread
+ let ctx_clone = ctx.clone();               // Clone context / 为进入线程克隆上下文
 
 let sensor_handle = thread::spawn(move || {
-    // ...uses stop_flag_clone and ctx_clone
+    // ...uses stop_flag_clone and ctx_clone / 使用副本
 });
 ```
 
- ### Checklist: Should I clone?
+ ### Checklist: Should I clone? / 检查清单：我应该克隆吗？
- 1. **Can I accept `&str` / `&T` instead of `String` / `T`?** → Borrow, don't clone
+ 1. **我能否接收 `&str` / `&T` 而非 `String` / `T`？** → 借用，不要克隆。
- 2. **Can I restructure to avoid needing two owners?** → Pass by reference or use scopes
+ 2. **我能否重构以避免需要两个所有者？** → 通过引用传递或使用作用域。
- 3. **Is this `Arc::clone()`?** → That's fine, it's O(1)
+ 3. **这是 `Arc::clone()` 吗？** → 没问题，它是 O(1) 的。
- 4. **Am I moving data into a thread/closure?** → Clone is necessary
+ 4. **我是否正在将数据移动到线程/闭包中？** → 克隆是必要的。
- 5. **Am I cloning in a hot loop?** → Profile and consider borrowing or `Cow<T>`
+ 5. **我是否在热点循环中克隆？** → 进行性能分析，考虑借用或 `Cow<T>`。
 
 ----
 
- ## `Cow<'a, T>`: Clone-on-Write — borrow when you can, clone when you must
+ ## `Cow<'a, T>`: Clone-on-Write — borrow when you can, clone when you must / 写时克隆 —— 能借则借，必克才隆
 
- `Cow` (Clone on Write) is an enum that holds **either** a borrowed reference **or**
+ `Cow`（Clone on Write，写时克隆）是一个枚举，它**既可以**持有借用引用，**也可以**
- an owned value. It's the Rust equivalent of "avoid allocation when possible, but
+ 持有拥有所有权的值。它是 Rust 中“尽可能避免分配，但在需要修改时才分配”的等价方案。
- allocate if you need to modify." C++ has no direct equivalent — the closest is a function
- that returns `const std::string&` sometimes and `std::string` other times.
+ C++ 没有直接的对应物 —— 最接近的是一个有时返回 `const std::string&`，有时返回 `std::string` 的函数。
 
- ### Why `Cow` exists
+ ### Why `Cow` exists / 为什么需要 `Cow`
 
 ```rust
- // Without Cow — you must choose: always borrow OR always clone
+ // Without Cow — you must choose / 不使用 Cow —— 你必须二选一：始终借用或始终克隆
 fn normalize(s: &str) -> String {          // Always allocates!
     if s.contains(' ') {
-        s.replace(' ', "_")               // New String (allocation needed)
+        s.replace(' ', "_")               // New String / 新 String（需要分配）
     } else {
-        s.to_string()                     // Unnecessary allocation!
+        s.to_string()                     // Unnecessary / 不必要的分配！
     }
 }
 
- // With Cow — borrow when unchanged, allocate only when modified
+ // With Cow — borrow when unchanged / 使用 Cow —— 未更改时借用，仅在修改时分配
 use std::borrow::Cow;
 
 fn normalize(s: &str) -> Cow<'_, str> {
     if s.contains(' ') {
-        Cow::Owned(s.replace(' ', "_"))    // Allocates (must modify)
+        Cow::Owned(s.replace(' ', "_"))    // Allocates / 分配（必须修改）
     } else {
-        Cow::Borrowed(s)                   // Zero allocation (passthrough)
+        Cow::Borrowed(s)                   // Zero alloc / 零分配（直传）
     }
 }
 ```
 
- ### How `Cow` works
+ ### How `Cow` works / `Cow` 如何工作
 
 ```rust
 use std::borrow::Cow;
 
- // Cow<'a, str> is essentially:
+ // Cow<'a, str> is essentially / Cow<'a, str> 本质上是：
 // enum Cow<'a, str> {
-//     Borrowed(&'a str),     // Zero-cost reference
+//     Borrowed(&'a str),     // Zero-cost / 零成本引用
-//     Owned(String),          // Heap-allocated owned value
+//     Owned(String),          // Managed / 堆分配的拥有所有权的值
 // }
 
 fn greet(name: &str) -> Cow<'_, str> {
     if name.is_empty() {
-        Cow::Borrowed("stranger")         // Static string — no allocation
+        Cow::Borrowed("stranger")         // Static / 静态字符串 —— 无分配
     } else if name.starts_with(' ') {
-        Cow::Owned(name.trim().to_string()) // Modified — allocation needed
+        Cow::Owned(name.trim().to_string()) // Modified / 已修改 —— 需要分配
     } else {
-        Cow::Borrowed(name)               // Passthrough — no allocation
+        Cow::Borrowed(name)               // Passthrough / 直传 —— 无分配
     }
 }
 
 fn main() {
     let g1 = greet("Alice");     // Cow::Borrowed("Alice")
     let g2 = greet("");          // Cow::Borrowed("stranger")
     let g3 = greet(" Bob ");     // Cow::Owned("Bob")
     
-    // Cow<str> implements Deref<Target = str>, so you can use it as &str:
+    // Cow<str> implements Deref / Cow<str> 实现了 Deref<Target = str>，因此你可以像使用 &str 一样使用它：
-    println!("Hello, {g1}!");    // Works — Cow auto-derefs to &str
+    println!("Hello, {g1}!");    // Works / 正常工作 —— Cow 自动解引用为 &str
     println!("Hello, {g2}!");
     println!("Hello, {g3}!");
 }
 ```
 
- ### Real-world use case: config value normalization
+ ### Real-world use case: config value normalization / 真实场景：配置值规范化
 
 ```rust
 use std::borrow::Cow;
 
- /// Normalize a SKU name: trim whitespace, lowercase.
+ /// Normalize SKU / 规范化 SKU 名称：修整空白符、转为小写。
- /// Returns Cow::Borrowed if already normalized (zero allocation).
+ /// Returns Borrowed if already ok / 如果已经是规范化的，则返回 Cow::Borrowed（零分配）。
 fn normalize_sku(sku: &str) -> Cow<'_, str> {
     let trimmed = sku.trim();
     if trimmed == sku && sku.chars().all(|c| c.is_lowercase() || !c.is_alphabetic()) {
-        Cow::Borrowed(sku)   // Already normalized — no allocation
+        Cow::Borrowed(sku)   // Already normalized / 已规范化 —— 无分配
     } else {
-        Cow::Owned(trimmed.to_lowercase())  // Needs modification — allocate
+        Cow::Owned(trimmed.to_lowercase())  // Needs / 需要修改 —— 进行分配
     }
 }
 
 fn main() {
-    let s1 = normalize_sku("server-x1");   // Borrowed — zero alloc
+    let s1 = normalize_sku("server-x1");   // Borrowed / 借用 —— 零分配
-    let s2 = normalize_sku("  Server-X1 "); // Owned — must allocate
+    let s2 = normalize_sku("  Server-X1 "); // Owned / 拥有所有权 —— 必须分配
     println!("{s1}, {s2}"); // "server-x1, server-x1"
 }
 ```
 
- ### When to use `Cow`
+ ### When to use `Cow` / 何时使用 `Cow`
 
-| **Situation** | **Use `Cow`?** |
+| **Situation / 场景** | **Use `Cow`? / 是否使用？** |
 |--------------|---------------|
-| Function returns input unchanged most of the time | ✅ Yes — avoid unnecessary clones |
+| Function returns unchanged mostly / 函数大部分时间返回原输入 | ✅ Yes / 是 —— 避免不必要的克隆 |
-| Parsing/normalizing strings (trim, lowercase, replace) | ✅ Yes — often input is already valid |
+| Parsing (trim, lowercase, replace) / 解析/规范化字符串 | ✅ Yes / 是 —— 通常输入已经是有效的 |
-| Always modifying — every code path allocates | ❌ No — just return `String` |
+| Always modifying / 总是修改 —— 每个代码路径都会分配 | ❌ No / 否 —— 直接返回 `String` |
-| Simple pass-through (never modifies) | ❌ No — just return `&str` |
+| Simple pass-through / 简单直传（从不修改） | ❌ No / 否 —— 直接返回 `&str` |
-| Data stored in a struct long-term | ❌ No — use `String` (owned) |
+| Stored in struct long-term / 长期存储在结构体中 | ❌ No / 否 —— 使用 `String` (拥有所有权) |
 
- > **C++ comparison**: `Cow<str>` is like a function that returns `std::variant<std::string_view, std::string>`
+ > **C++ 对比**：`Cow<str>` 就像一个返回 `std::variant<std::string_view, std::string>` 的函数
- > — except with automatic deref and no boilerplate to access the value.
+ > —— 不同之处在于它支持自动解引用，访问值时没有繁琐的样板代码。
 
 ----
 
- ## `Weak<T>`: Breaking Reference Cycles — Rust's `weak_ptr`
+ ## `Weak<T>`: Breaking Reference Cycles — Rust's `weak_ptr` / `Weak<T>`：打破引用循环 —— Rust 的 `weak_ptr`
 
- `Weak<T>` is the Rust equivalent of C++ `std::weak_ptr<T>`. It holds a non-owning
+ `Weak<T>` 是 C++ `std::weak_ptr<T>` 在 Rust 中的等价物。它持有一个对
- reference to an `Rc<T>` or `Arc<T>` value. The value can be deallocated while
+ `Rc<T>` 或 `Arc<T>` 值的非拥有性引用。值可以在 `Weak` 引用仍然存在时
- `Weak` references still exist — calling `upgrade()` returns `None` if the value is gone.
+ 被释放 —— 如果值已消失，调用 `upgrade()` 将返回 `None`。
 
- ### Why `Weak` exists
+ ### Why `Weak` exists / 为什么需要 `Weak`
 
- `Rc<T>` and `Arc<T>` create reference cycles if two values point to each
+ 如果两个值互相指向对方，`Rc<T>` 和 `Arc<T>` 会创建引用循环
- other — neither ever reaches refcount 0, so neither is dropped (memory leak).
+ —— 两者的引用计数永远不会达到 0，因此都不会被释放（导致内存泄漏）。
- `Weak` breaks the cycle:
+ `Weak` 可以打破这种循环：
 
 ```rust
 use std::rc::{Rc, Weak};
 use std::cell::RefCell;
 
 #[derive(Debug)]
 struct Node {
     value: String,
-    parent: RefCell<Weak<Node>>,      // Weak — doesn't prevent parent from dropping
+    parent: RefCell<Weak<Node>>,      // Weak / 弱引用 —— 不阻止父节点释放
-    children: RefCell<Vec<Rc<Node>>>,  // Strong — parent owns children
+    children: RefCell<Vec<Rc<Node>>>,  // Strong / 强引用 —— 父节点拥有子节点
 }
 
 impl Node {
     fn new(value: &str) -> Rc<Node> {
         Rc::new(Node {
             value: value.to_string(),
             parent: RefCell::new(Weak::new()),
             children: RefCell::new(Vec::new()),
         })
     }
 
     fn add_child(parent: &Rc<Node>, child: &Rc<Node>) {
-        // Child gets a weak reference to parent (no cycle)
+        // Child gets weak ref / 子节点获得父节点的弱引用（无循环）
         *child.parent.borrow_mut() = Rc::downgrade(parent);
-        // Parent gets a strong reference to child
+        // Parent gets strong ref / 父节点获得子节点的强引用
         parent.children.borrow_mut().push(Rc::clone(child));
     }
 }
 
 fn main() {
     let root = Node::new("root");
     let child = Node::new("child");
     Node::add_child(&root, &child);
 
-    // Access parent from child via upgrade()
+    // Access via upgrade() / 通过 upgrade() 访问父节点
     if let Some(parent) = child.parent.borrow().upgrade() {
         println!("Child's parent: {}", parent.value); // "root"
     }
     
     println!("Root strong count: {}", Rc::strong_count(&root));  // 1
     println!("Root weak count: {}", Rc::weak_count(&root));      // 1
 }
 ```
 
- ### C++ comparison
+ ### C++ comparison / C++ 对比
 
 ```cpp
 // C++ — weak_ptr to break shared_ptr cycle
+// C++ — 使用 weak_ptr 打破 shared_ptr 循环
 struct Node {
     std::string value;
-    std::weak_ptr<Node> parent;                  // Weak — no ownership
+    std::weak_ptr<Node> parent;                  // Weak / 弱引用 —— 无所有权
-    std::vector<std::shared_ptr<Node>> children;  // Strong — owns children
+    std::vector<std::shared_ptr<Node>> children;  // Strong / 强引用 —— 拥有子节点
 
     static auto create(const std::string& v) {
         return std::make_shared<Node>(Node{v, {}, {}});
     }
 };
 
 auto root = Node::create("root");
 auto child = Node::create("child");
- child->parent = root;          // weak_ptr assignment
+ child->parent = root;          // weak_ptr assignment / 弱引用赋值
 root->children.push_back(child);
 
- if (auto p = child->parent.lock()) {   // lock() → shared_ptr or null
+ if (auto p = child->parent.lock()) {   // lock() / lock() → shared_ptr 或 null
     std::cout << "Parent: " << p->value << std::endl;
 }
 ```
 
-| C++ | Rust | Notes |
+| **C++** | **Rust** | **Notes / 说明** |
 |-----|------|-------|
-| `shared_ptr<T>` | `Rc<T>` (single-thread) / `Arc<T>` (multi-thread) | Same semantics |
+| `shared_ptr<T>` | `Rc<T>` (单线程) / `Arc<T>` (多线程) | Same semantics / 语义相同 |
-| `weak_ptr<T>` | `Weak<T>` from `Rc::downgrade()` / `Arc::downgrade()` | Same semantics |
+| `weak_ptr<T>` | `Weak<T>` 来自 `downgrade()` | Same semantics / 语义相同 |
-| `weak_ptr::lock()` → `shared_ptr` or null | `Weak::upgrade()` → `Option<Rc<T>>` | `None` if dropped |
+| `lock()` → `shared_ptr` | `upgrade()` → `Option` | `None` / 若已释放则为 None |
-| `shared_ptr::use_count()` | `Rc::strong_count()` | Same meaning |
+| `use_count()` | `strong_count()` | Same / 含义相同 |
 
- ### When to use `Weak`
+ ### When to use `Weak` / 何时使用 `Weak`
 
-| **Situation** | **Pattern** |
+| **Situation / 场景** | **Pattern / 模式** |
 |--------------|-----------|
-| Parent ↔ child tree relationships | Parent holds `Rc<Child>`, child holds `Weak<Parent>` |
-| Parent ↔ child / 父子树关系 | Parent holds `Rc`, child holds `Weak` |
-| Observer pattern / event listeners | Event source holds `Weak<Observer>`, observer holds `Rc<Source>` |
-| Observer pattern / 观察者模式 | Source holds `Weak`, observer holds `Rc` |
-| Cache that doesn't prevent deallocation | `HashMap<Key, Weak<Value>>` — entries go stale naturally |
-| Cache / 缓存 | `HashMap<Key, Weak<Value>>` —— 条目会自然失效 |
-| Breaking cycles in graph structures | Cross-links use `Weak`, tree edges use `Rc`/`Arc` |
-| Graph cycles / 图循环 | Cross-links / 跨链使用 `Weak`，树边使用 `Rc`/`Arc` |
 
- > **Prefer the arena pattern** (Case Study 2) over `Rc/Weak` for tree structures in
+ > **首选 Arena 模式**（案例研究 2）：在新代码的树形结构中，优先使用 Arena 模式而非 `Rc/Weak`。
- > new code. `Vec<T>` + indices is simpler, faster, and has zero reference-counting
- > overhead. Use `Rc/Weak` when you need shared ownership with dynamic lifetimes.
+ > `Vec<T>` + 索引更简单、更快，且没有任何引用计数开销。只有当你确实需要具有动态生命周期的共享所有权时，才使用 `Rc/Weak`。
 
 ----
 
- ## Copy vs Clone, PartialEq vs Eq — when to derive what
+ ## Copy vs Clone, PartialEq vs Eq — when to derive what / Copy vs Clone，PartialEq vs Eq —— 何时派生什么
 
- - **Copy ≈ C++ trivially copyable (no custom copy ctor/dtor).** Types like `int`, `enum`, and simple POD structs — the compiler generates a bitwise `memcpy` automatically. In Rust, `Copy` is the same idea: assignment `let b = a;` does an implicit bitwise copy and both variables remain valid.
+ - **Copy ≈ C++ trivially copyable（无自定义拷贝构造/析构函数）。** 像 `int`、`enum` 和简单的 POD 结构体 —— 编译器会自动生成按位 `memcpy`。在 Rust 中，`Copy` 是同样的概念：赋值语句 `let b = a;` 会进行隐式按位拷贝，且两个变量都保持有效。
- - **Clone ≈ C++ copy constructor / `operator=` deep-copy.** When a C++ class has a custom copy constructor (e.g., to deep-copy a `std::vector` member), the equivalent in Rust is implementing `Clone`. You must call `.clone()` explicitly — Rust never hides an expensive copy behind `=`.
+ - **Clone ≈ C++ 拷贝构造函数 / `operator=` 深拷贝。** 当 C++ 类具有自定义拷贝构造函数（例如，为了深拷贝 `std::vector` 成员）时，Rust 中的等价物是实现 `Clone`。你必须显式调用 `.clone()` —— Rust 绝不会在 `=` 背后隐藏高昂的拷贝开销。
- - **Key distinction:** In C++, both trivial copies and deep copies happen implicitly via the same `=` syntax. Rust forces you to choose: `Copy` types copy silently (cheap), non-`Copy` types **move** by default, and you must opt in to an expensive duplicate with `.clone()`.
+ - **关键区别**：在 C++ 中，平凡拷贝和深拷贝都通过相同的 `=` 语法隐式发生。Rust 强制你做出选择：`Copy` 类型静默拷贝（廉价），非 `Copy` 类型默认进行**移动**（move），你必须通过 `.clone()` 显式选择进行昂贵的复制。
- - Similarly, C++ `operator==` doesn't distinguish between types where `a == a` always holds (like integers) and types where it doesn't (like `float` with NaN). Rust encodes this in `PartialEq` vs `Eq`.
+ - 类似地，C++ 的 `operator==` 不区分那些 `a == a` 总是成立的类型（如整数）和那些不成立的类型（如带有 NaN 的 `float`）。Rust 在 `PartialEq` 与 `Eq` 中体现了这一点。
 
- ### Copy vs Clone
+ ### Copy vs Clone / Copy vs Clone 对比
 
-| | **Copy** | **Clone** |
+| | **Copy** | **Clone** |
 |---|---------|----------|
-| **How it works** | Bitwise memcpy (implicit) | Custom logic (explicit `.clone()`) |
-| **How it works / 工作原理** | Bitwise / 按位 memcpy (隐式) | Custom / 自定义逻辑 (显式 `.clone()`) |
-| **When it happens** | On assignment: `let b = a;` | Only when you call `.clone()` |
-| **When it happens / 触发时机** | Assignment / 赋值时 | Explicit call / 显式调用时 |
-| **After copy/clone** | Both `a` and `b` are valid | Both `a` and `b` are valid |
-| **After / 之后** | Both valid / 两者均有效 | Both valid / 两者均有效 |
-| **Without either** | `let b = a;` **moves** `a` (a is gone) | `let b = a;` **moves** `a` (a is gone) |
-| **Without / 若无** | **Moves** `a` / 移动 `a` (a 消失) | **Moves** `a` / 移动 `a` (a 消失) |
-| **Allowed for** | Types with no heap data | Any type |
-| **Allowed for / 允许用于** | No heap / 无堆数据的类型 | Any type / 任意类型 |
-| **C++ analogy** | Trivially copyable / POD types (no custom copy ctor) | Custom copy constructor (deep copy) |
-| **C++ 类比** | POD 类型 (无自定义拷贝构造) | 拷贝构造函数 (深拷贝) |
 
- ### Real example: Copy — simple enums
+ ### Real example: Copy — simple enums / 真实示例：Copy —— 简单枚举
 ```rust
- // From fan_diag/src/sensor.rs — all unit variants, fits in 1 byte
+ // From fan_diag/src/sensor.rs — fits in 1 byte / 1 字节即可容纳
 #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
 pub enum FanStatus {
     #[default]
     Normal,
     Low,
     High,
     Missing,
     Failed,
     Unknown,
 }
 
 let status = FanStatus::Normal;
- let copy = status;   // Implicit copy — status is still valid
+ let copy = status;   // Implicit copy / 隐式拷贝 —— status 仍然有效
- println!("{:?} {:?}", status, copy);  // Both work
+ println!("{:?} {:?}", status, copy);  // Both work / 两者均可工作
 ```
 
- ### Real example: Copy — enum with integer payloads
+ ### Real example: Copy — enum with integer payloads / 真实示例：Copy —— 带有整数负载的枚举
 ```rust
- // Example: healthcheck.rs — u32 payloads are Copy, so the whole enum is too
+ // Example: healthcheck.rs — u32 is Copy / 示例：u32 是 Copy，所以整个枚举也是
 #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
 pub enum HealthcheckStatus {
     Pass,
     ProgramError(u32),
     DmesgError(u32),
     RasError(u32),
     OtherError(u32),
     Unknown,
 }
 ```
 
- ### Real example: Clone only — struct with heap data
+ ### Real example: Clone only — struct with heap data / 真实示例：仅 Clone —— 带有堆数据的结构体
 ```rust
- // Example: components.rs — String prevents Copy
+ // Example: components.rs — String prevents Copy / 示例：String 阻碍了 Copy
 #[derive(Debug, Clone, Serialize, Deserialize)]
 pub struct FruData {
     pub technology: DeviceTechnology,
-    pub physical_location: String,      // ← String: heap-allocated, can't Copy
+    pub physical_location: String,      // ← String / 堆分配，无法 Copy
     pub expected: bool,
     pub removable: bool,
 }
-// let a = fru_data;   → MOVES (a is gone)
-// let a = fru_data.clone();  → CLONES (fru_data still valid, new heap allocation)
+// let a = fru_data;   → MOVES / 移动 (fru_data 消失)
+// let a = fru_data.clone();  → CLONES / 克隆 (fru_data 仍有效，产生新堆分配)
 ```
 
- ### The rule: Can it be Copy?
+ ### The rule: Can it be Copy? / 规则：它可以是 Copy 吗？
 ```text
- Does the type contain String, Vec, Box, HashMap,
- Rc, Arc, or any other heap-owning type?
-     YES → Clone only (cannot be Copy)
-     NO  → You CAN derive Copy (and should, if the type is small)
+ 该类型是否包含 String、Vec、Box、HashMap、
+ Rc、Arc 或任何其他拥有堆数据的类型？
+     是 → 只能是 Clone (无法是 Copy)
+     否 → 你可以派生 Copy (如果类型很小，建议派生)
 ```
 
- ### PartialEq vs Eq
+ ### PartialEq vs Eq / PartialEq vs Eq 对比
 
-| | **PartialEq** | **Eq** |
+| | **PartialEq** | **Eq** |
 |---|--------------|-------|
-| **What it gives you** | `==` and `!=` operators | Marker: "equality is reflexive" |
-| **What it gives you / 带来的功能** | `==` and `!=` operators / 运算符 | Marker / 标记：“相等性具有自反性” |
-| **Reflexive? (a == a)** | Not guaranteed | **Guaranteed** |
-| **Reflexive? / 自反性？** | Not guaranteed / 不保证 | **Guaranteed** / 保证 |
-| **Why it matters** | `f32::NAN != f32::NAN` | `HashMap` keys **require** `Eq` |
-| **Why it matters / 重要性** | `f32::NAN != f32::NAN` | `HashMap` keys **require** Eq |
-| **When to derive** | Almost always | When the type has no `f32`/`f64` fields |
-| **When to derive / 何时派生** | Almost always / 绝大多数情况 | No `f32`/`f64` / 无浮点字段时 |
-| **C++ analogy** | `operator==` | No direct equivalent (C++ doesn't check) |
-| **C++ 类比** | `operator==` | 无直接对应 (C++ 不检查自反性) |
 
- ### Real example: Eq — used as HashMap key
+ ### Real example: Eq — used as HashMap key / 真实示例：Eq —— 用作 HashMap 的键
 ```rust
- // From hms_trap/src/cpu_handler.rs — Hash requires Eq
+ // From hms_trap/src/cpu_handler.rs — Hash requires Eq / 示例：Hash 需要 Eq
 #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
 pub enum CpuFaultType {
     InvalidFaultType,
     CpuCperFatalErr,
     CpuLpddr5UceErr,
     CpuC2CUceFatalErr,
-    // ...
+    // // ...
 }
-// Used as: HashMap<CpuFaultType, FaultHandler>
-// HashMap keys must be Eq + Hash — PartialEq alone won't compile
+// Usage / 用法：HashMap<CpuFaultType, FaultHandler>
+// HashMap keys must be Eq + Hash / 键必须是 Eq + Hash —— 仅 PartialEq 无法通过编译
 ```
 
- ### Real example: No Eq possible — type contains f32
+ ### Real example: No Eq possible — type contains f32 / 真实示例：无法实现 Eq —— 类型包含 f32
 ```rust
- // Example: types.rs — f32 prevents Eq
+ // Example: types.rs — f32 prevents Eq / 示例：f32 阻碍了 Eq
 #[derive(Debug, Clone, Serialize, Deserialize, Default)]
 pub struct TemperatureSensors {
-    pub warning_threshold: Option<f32>,   // ← f32 has NaN ≠ NaN
+    pub warning_threshold: Option<f32>,   // ← f32 has NaN / NaN ≠ NaN
-    pub critical_threshold: Option<f32>,  // ← can't derive Eq
+    pub critical_threshold: Option<f32>,  // ← can't / 无法派生 Eq
     pub sensor_names: Vec<String>,
 }
-// Cannot be used as HashMap key. Cannot derive Eq.
-// Because: f32::NAN == f32::NAN is false, violating reflexivity.
+// Cannot be key / 无法用作 HashMap 键。无法派生 Eq。
+// Because / 原因：f32::NAN == f32::NAN 为假，违反了自反性。
 ```
 
- ### PartialOrd vs Ord
+ ### PartialOrd vs Ord / PartialOrd vs Ord 对比
 
-| | **PartialOrd** | **Ord** |
+| | **PartialOrd** | **Ord** |
 |---|---------------|--------|
-| **What it gives you** | `<`, `>`, `<=`, `>=` | `.sort()`, `BTreeMap` keys |
-| **What it gives you / 带来的功能** | `<`, `>`, `<=`, `>=` / 比较运算符 | `.sort()`, `BTreeMap` keys / 排序支持 |
-| **Total ordering?** | No (some pairs may be incomparable) | **Yes** (every pair is comparable) |
-| **Total ordering? / 全序关系？** | No / 否 (部分值可能无法比较) | **Yes** / 是 (所有值均可比较) |
-| **f32/f64?** | PartialOrd only (NaN breaks ordering) | Cannot derive Ord |
-| **f32/f64?** | Only PartialOrd / 仅 PartialOrd | Cannot derive / 无法派生 Ord |
 
- ### Real example: Ord — severity ranking
+ ### Real example: Ord — severity ranking / 真实示例：Ord —— 严重性排名
 ```rust
- // From hms_trap/src/fault.rs — variant order defines severity
+ // From hms_trap/src/fault.rs — order defines severity / 示例：变体顺序定义了严重性
 #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
 pub enum FaultSeverity {
-    Info,      // lowest  (discriminant 0)
+    Info,      // lowest / 最低 (discriminant 0)
-    Warning,   //         (discriminant 1)
+    Warning,   //         (discriminant 1)
-    Error,     //         (discriminant 2)
+    Error,     //         (discriminant 2)
-    Critical,  // highest (discriminant 3)
+    Critical,  // highest / 最高 (discriminant 3)
 }
-// FaultSeverity::Info < FaultSeverity::Critical → true
-// Enables: if severity >= FaultSeverity::Error { escalate(); }
+// Condition / 启用：if severity >= FaultSeverity::Error { escalate(); }
 ```
 
- ### Real example: Ord — diagnostic levels for comparison
+ ### Real example: Ord — diagnostic levels / 真实示例：Ord —— 诊断级别
 ```rust
- // Example: orchestration.rs
+ // Example: orchestration.rs / 示例：编排逻辑
 #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
 pub enum GpuDiagLevel {
     #[default]
-    Quick,     // lowest
+    Quick,     // lowest / 最低
     Standard,
     Extended,
-    Full,      // highest
+    Full,      // highest / 最高
 }
-// Enables: if requested_level >= GpuDiagLevel::Extended { run_extended_tests(); }
+// Condition / 启用：if requested_level >= GpuDiagLevel::Extended { ... }
 ```
 
- ### Derive decision tree
+ ### Derive decision tree / 派生决策树
 
 ```text
-                        Your new type
-                             │
-                    Contains String/Vec/Box?
-                       /              \
-                     YES                NO
-                      │                  │
-               Clone only          Clone + Copy
-                      │                  │
-               Contains f32/f64?    Contains f32/f64?
-                 /          \         /          \
-               YES           NO     YES           NO
-                │             │      │             │
-          PartialEq       PartialEq  PartialEq  PartialEq
-          only            + Eq       only       + Eq
-                           │                      │
-                     Need sorting?           Need sorting?
-                       /       \               /       \
-                     YES        NO            YES        NO
-                      │          │              │          │
-                PartialOrd    Done        PartialOrd    Done
-                + Ord                     + Ord
-                      │                        │
-                Need as                  Need as
-                map key?                 map key?
-                   │                        │
-                 + Hash                   + Hash
+                          你的新类型
+                             │
+                    是否包含 String/Vec/Box?
+                       /              \
+                      是               否
+                      │                │
+                  仅 Clone         Clone + Copy
+                      │                │
+                 是否包含 f32/f64?   是否包含 f32/f64?
+                 /          \         /          \
+                是           否       是           否
+                │            │       │            │
+           仅 PartialEq  PartialEq  仅 PartialEq  PartialEq
+                         + Eq                 + Eq
+                          │                    │
+                      需要排序吗?            需要排序吗?
+                       /       \             /       \
+                      是        否           是        否
+                      │         │           │         │
+                PartialOrd     完成    PartialOrd    完成
+                + Ord                  + Ord
+                      │                     │
+                  需要作为                需要作为
+                   Map 键?               Map 键?
+                      │                     │
+                    + Hash                + Hash
 ```
 
- ### Quick reference: common derive combos from production Rust code
+ ### Quick reference: common derive combos / 快速参考：生产环境中的常见派生组合
 
-| **Type category** | **Typical derive** | **Example** |
+| **Type category / 类型类别** | **Typical derive / 典型派生** | **Example / 示例** |
 |-------------------|--------------------|------------|
-| Simple status enum | `Copy, Clone, PartialEq, Eq, Default` | `FanStatus` |
+| Simple status / 简单状态枚举 | `Copy, Clone, PartialEq, Eq, Default` | `FanStatus` |
-| Enum used as HashMap key | `Copy, Clone, PartialEq, Eq, Hash` | `CpuFaultType`, `SelComponent` |
+| Map key / 用作 HashMap 键 | `Copy, Clone, PartialEq, Eq, Hash` | `CpuFaultType` |
-| Sortable severity enum | `Copy, Clone, PartialEq, Eq, PartialOrd, Ord` | `FaultSeverity`, `GpuDiagLevel` |
-| Severity / 可排序的严重性 | `Copy, Clone, PartialEq, Eq, PartialOrd, Ord` | `FaultSeverity` |
-| Data struct with Strings | `Clone, Debug, Serialize, Deserialize` | `FruData`, `OverallSummary` |
-| Data / 带 String 的结构体 | `Clone, Debug, Serialize, Deserialize` | `FruData` |
-| Serializable config | `Clone, Debug, Default, Serialize, Deserialize` | `DiagConfig` |
-| Config / 序列化配置 | `Clone, Debug, Default, Serialize, ...` | `DiagConfig` |
