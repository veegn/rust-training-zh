## Avoiding excessive clone() / 避免过度使用 clone()

> **What you'll learn / 你将学到：** Why `.clone()` is a code smell in Rust, how to restructure ownership to eliminate unnecessary copies, and the specific patterns that signal an ownership design problem.
>
> 为什么 `.clone()` 在 Rust 中被视为一种“代码坏味道”，如何重组所有权以消除不必要的拷贝，以及哪些特定模式预示着所有权设计存在问题。

*Coming from C++, `.clone()` feels like a safe default — "just copy it". But excessive cloning hides ownership problems and hurts performance. **Rule of thumb**: If you're cloning to satisfy the borrow checker, you probably need to restructure ownership instead.*

对于 C++ 开发者来说，`.clone()` 听起来像是一个安全的默认选择 —— “拷贝一份就行”。但过度的克隆会掩盖所有权问题并损害性能。**经验法则**：如果你克隆是为了满足借用检查器，那么你可能需要重组所有权结构。

---

### When clone() is wrong / 何时使用 clone() 是错误的

```rust
// BAD: Cloning a String / 坏习惯：为了传参给只读函数而克隆 String
fn log_message(msg: String) {  // Takes ownership unnecessarily / 不必要地夺取了所有权
    println!("[LOG] {}", msg);
}

let message = String::from("GPU test passed");
log_message(message.clone());  // Wasteful / 浪费：分配了一个全新的 String
log_message(message);           // Original consumed / 原变量被消耗 —— 克隆毫无意义
```

```rust
// GOOD: Accept a borrow / 好习惯：接收借用 —— 零分配
fn log_message(msg: &str) {    // Borrows, doesn't own / 借用，不拥有
    println!("[LOG] {}", msg);
}

let message = String::from("GPU test passed");
log_message(&message);          // No clone / 无克隆，无分配
log_message(&message);          // Can call again / 可以再次调用 —— 消息未被消耗
```

### Real example: returning `&str` instead of cloning / 真实示例：返回 `&str` 而非克隆
```rust
// Example: healthcheck.rs — returns borrowed view / 示例：返回借用视图，零分配
pub fn serial_or_unknown(&self) -> &str {
    self.serial.as_deref().unwrap_or(UNKNOWN_VALUE)
}

pub fn model_or_unknown(&self) -> &str {
    self.model.as_deref().unwrap_or(UNKNOWN_VALUE)
}
```
*The C++ equivalent would return `const std::string&` or `std::string_view` — but in C++ neither is lifetime-checked. In Rust, the borrow checker guarantees the returned `&str` can't outlive `self`.*

C++ 的对应做法是返回 `const std::string&` 或 `std::string_view` —— 但在 C++ 中，这两者都没有经过生命周期检查。而在 Rust 中，借用检查器保证返回的 `&str` 存活时间不会超过 `self`。

---

### When clone() IS appropriate / 何时使用 clone() 是合理的

| **Situation / 场景** | **Why clone is OK / 理由** | **Example / 示例** |
|--------------|--------------------|-----------|
| `Arc::clone()` for threading | Bumps ref count / 增加引用计数 (~1 ns)，不拷贝数据 | `let flag = Arc::clone(&flag);` |
| Moving data into thread | Thread needs copy / 线程需要自己的副本 | `let ctx = ctx.clone(); thread::spawn(...)` |
| Extracting from fields / 从字段中提取 | Can't move out / 无法从借用中移除所有权 | 返回拥有所有权的 `String` 时 |
| Small `Copy` types in `Option` | `.copied()` is clearer / 更清晰 | 将 `Option<&u32>` 转换为 `Option<u32>` |

---

### Checklist: Should I clone? / 检查清单：我应该克隆吗？
1. **Can I accept `&str` / `&T` instead of `String` / `T`?** → Borrow, don't clone. / **我能否接收 `&str` / `&T` 而非 `String` / `T`？** → 借用，不要克隆。
2. **Can I restructure to avoid needing two owners?** → Pass by reference or use scopes. / **我能否重构以避免需要两个所有者？** → 通过引用传递或使用作用域。
3. **Is this `Arc::clone()`?** → That's fine, it's O(1). / **这是 `Arc::clone()` 吗？** → 没问题，它是 O(1) 的。
4. **Am I moving data into a thread/closure?** → Clone is necessary. / **我是否正在将数据移动到线程/闭包中？** → 克隆是必要的。
5. **Am I cloning in a hot loop?** → Profile and consider borrowing or `Cow<T>`. / **我是否在热点循环中克隆？** → 进行性能分析，考虑借用或 `Cow<T>`。

---

## `Cow<'a, T>`: Clone-on-Write / 写时克隆 —— 能借则借，必克才隆

*`Cow` (Clone on Write) is an enum that holds **either** a borrowed reference **or** an owned value. It's the Rust equivalent of "avoid allocation when possible, but allocate if you need to modify."*

`Cow`（Clone on Write，写时克隆）是一个枚举，它**既可以**持有借用引用，**也可以**持有拥有所有权的值。它是 Rust 中“尽可能避免分配，但在需要修改时才分配”的等价方案。

### Why `Cow` exists / 为什么需要 `Cow`

```rust
// Without Cow — you must choose / 不使用 Cow —— 你必须二选一：始终借用或始终克隆

// With Cow — borrow when unchanged / 使用 Cow —— 未更改时借用，仅在修改时分配
use std::borrow::Cow;

fn normalize(s: &str) -> Cow<'_, str> {
    if s.contains(' ') {
        Cow::Owned(s.replace(' ', "_"))    // Allocates / 分配（必须修改）
    } else {
        Cow::Borrowed(s)                   // Zero alloc / 零分配（直传）
    }
}
```

### How `Cow` works / `Cow` 如何工作

```rust
use std::borrow::Cow;

// Cow<'a, str> is essentially / Cow<'a, str> 本质上是：
// enum Cow<'a, str> {
//     Borrowed(&'a str),     // Zero-cost / 零成本引用
//     Owned(String),          // Managed / 堆分配的拥有所有权的值
// }

fn greet(name: &str) -> Cow<'_, str> {
    if name.is_empty() {
        Cow::Borrowed("stranger")         // Static / 静态字符串 —— 无分配
    } else if name.starts_with(' ') {
        Cow::Owned(name.trim().to_string()) // Modified / 已修改 —— 需要分配
    } else {
        Cow::Borrowed(name)               // Passthrough / 直传 —— 无分配
    }
}
```

---

## `Weak<T>`: Breaking Reference Cycles / `Weak<T>`：打破引用循环 —— Rust 的 `weak_ptr`

*`Weak<T>` is the Rust equivalent of C++ `std::weak_ptr<T>`. It holds a non-owning reference to an `Rc<T>` or `Arc<T>` value. `Weak` references break cycles that would otherwise cause memory leaks.*

`Weak<T>` 是 C++ `std::weak_ptr<T>` 在 Rust 中的等价物。它持有一个对 `Rc<T>` 或 `Arc<T>` 值的非拥有性引用。`Weak` 引用可以打破导致内存泄漏的循环。

### Why `Weak` exists / 为什么需要 `Weak`

```rust
use std::rc::{Rc, Weak};
use std::cell::RefCell;

#[derive(Debug)]
struct Node {
    value: String,
    parent: RefCell<Weak<Node>>,      // Weak / 弱引用 —— 不阻止父节点释放
    children: RefCell<Vec<Rc<Node>>>,  // Strong / 强引用 —— 父节点拥有子节点
}

impl Node {
    fn add_child(parent: &Rc<Node>, child: &Rc<Node>) {
        // Child gets weak ref / 子节点获得父节点的弱引用（无循环）
        *child.parent.borrow_mut() = Rc::downgrade(parent);
        // Parent gets strong ref / 父节点获得子节点的强引用
        parent.children.borrow_mut().push(Rc::clone(child));
    }
}
```

---

## Copy vs Clone, PartialEq vs Eq — when to derive what / Copy vs Clone，PartialEq vs Eq —— 何时派生什么

- **Copy ≈ C++ trivially copyable**. Compiler generates a bitwise `memcpy` automatically. assignment `let b = a;` does an implicit bitwise copy. / **Copy ≈ C++ trivially copyable（无自定义拷贝构造/析构函数）。**
- **Clone ≈ C++ copy constructor / `operator=` deep-copy.** Implementing `Clone` means you must call `.clone()` explicitly — Rust never hides an expensive copy. / **Clone ≈ C++ 拷贝构造函数 / `operator=` 深拷贝。**

### Copy vs Clone / Copy vs Clone 对比

| | **Copy** | **Clone** |
|---|---------|----------|
| **How it works / 工作原理** | Bitwise / 按位 memcpy (隐式) | Custom / 自定义逻辑 (显式 `.clone()`) |
| **When it happens / 触发时机** | Assignment / 消耗时触发隐式复制 | Explicit call / 显式调用时 |
| **After / 之后** | Both valid / 两者均有效 | Both valid / 两者均有效 |
| **Without / 若无** | **Moves** `a` / 移动 `a` (a 消失) | **Moves** `a` / 移动 `a` (a 消失) |
| **Allowed for / 允许用于** | No heap / 无堆数据的类型 | Any type / 任意类型 |

---

### PartialEq vs Eq / PartialEq vs Eq 对比

| | **PartialEq** | **Eq** |
|---|--------------|-------|
| **What it gives / 带来的功能** | `==` and `!=` operators / 运算符 | Marker / 标记：“相等性具有自反性” |
| **Reflexive? / 自反性？** | Not guaranteed / 不保证 | **Guaranteed** / 保证 |
| **Why it matters / 重要性** | `f32::NAN != f32::NAN` | `HashMap` keys **require** Eq |
| **When to derive / 何时派生** | Almost always / 绝大多数情况 | No `f32`/`f64` / 无浮点字段时 |

---

### Real example: Ord — severity ranking / 真实示例：Ord —— 严重性排名
```rust
// From hms_trap/src/fault.rs — order defines severity / 示例：变体顺序定义了严重性
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum FaultSeverity {
    Info,      // lowest / 最低 (discriminant 0)
    Warning,   //         (discriminant 1)
    Error,     //         (discriminant 2)
    Critical,  // highest / 最高 (discriminant 3)
}
// Enables: if severity >= FaultSeverity::Error { escalate(); }
```

### Quick reference: common derive combos / 快速参考：生产环境中的常见派生组合

| **Type category / 类型类别** | **Typical derive / 典型派生** | **Example / 示例** |
|-------------------|--------------------|------------|
| Simple status / 简单状态枚举 | `Copy, Clone, PartialEq, Eq, Default` | `FanStatus` |
| Map key / 用作 HashMap 键 | `Copy, Clone, PartialEq, Eq, Hash` | `CpuFaultType` |
| Severity / 可排序的严重性 | `Copy, Clone, PartialEq, Eq, PartialOrd, Ord` | `FaultSeverity` |
| Data / 带 String 的结构体 | `Clone, Debug, Serialize, Deserialize` | `FruData` |
| Config / 序列化配置 | `Clone, Debug, Default, Serialize, ...` | `DiagConfig` |
