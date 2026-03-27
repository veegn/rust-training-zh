# 2. Traits In Depth / 2. Trait 深入解析 🟡

> **What you'll learn / 你将学到：**
> - Associated types vs generic parameters — and when to use each / 关联类型 vs 泛型参数 —— 以及何时使用它们
> - GATs, blanket impls, marker traits, and trait object safety rules / GAT、blanket impl、标记 trait 以及 trait 对象安全规则
> - How vtables and fat pointers work under the hood / vtable 和脂肪指针的底层工作原理
> - Extension traits, enum dispatch, and typed command patterns / 扩展 trait、枚举分发以及类型化命令模式

## Associated Types vs Generic Parameters / 关联类型 vs 泛型参数

Both let a trait work with different types, but they serve different purposes:

二者都能让 trait 处理不同的类型，但它们的用途不同：

```rust
// --- ASSOCIATED TYPE: One implementation per type ---
// --- 关联类型：每个类型只有一个实现 ---
trait Iterator {
    type Item; // Each iterator produces exactly ONE kind of item / 每个迭代器只产生一种类型的项

    fn next(&mut self) -> Option<Self::Item>;
}

// A custom iterator that always yields i32 — there's no choice
// 一个总是产生 i32 的自定义迭代器 —— 没有其他选择
struct Counter { max: i32, current: i32 }

impl Iterator for Counter {
    type Item = i32; // Exactly one Item type per implementation / 每个实现只有一个 Item 类型
    fn next(&mut self) -> Option<i32> {
        if self.current < self.max {
            self.current += 1;
            Some(self.current)
        } else {
            None
        }
    }
}

// --- GENERIC PARAMETER: Multiple implementations per type ---
// --- 泛型参数：每个类型可以有多个实现 ---
trait Convert<T> {
    fn convert(&self) -> T;
}

// A single type can implement Convert for MANY target types:
// 一个类型可以为多种目标类型实现 Convert：
impl Convert<f64> for i32 {
    fn convert(&self) -> f64 { *self as f64 }
}
impl Convert<String> for i32 {
    fn convert(&self) -> String { self.to_string() }
}
```

**When to use which / 何时该用哪一个**：

| Use / 使用 | When / 何时 |
|-----|------|
| **Associated type** / **关联类型** | There's exactly ONE natural output/result per implementing type (e.g., `Iterator::Item`). / 每个实现类型恰好有一个自然的输出/结果（例如 `Iterator::Item`）。 |
| **Generic parameter** / **泛型参数** | A type can meaningfully implement the trait for MANY different types (e.g., `From<T>`). / 一个类型可以有意义地为许多不同的类型实现该 trait（例如 `From<T>`）。 |

**Intuition / 直觉解析**：If it makes sense to ask "what is the `Item` of this iterator?", use associated type. If it makes sense to ask "can this convert to `f64`? to `String`? to `bool`?", use a generic parameter.

**直觉解析**：如果问“这个迭代器的 `Item` 是什么？”是有意义的，请使用关联类型。如果问“这个类型能转换成 `f64` 吗？转换成 `String` 吗？转换成 `bool` 吗？”是有意义的，请使用泛型参数。

```rust
// Real-world example: std::ops::Add / 现实世界示例：std::ops::Add
trait Add<Rhs = Self> {
    type Output; // Associated type — addition has ONE result type / 关联类型 —— 加法只有一个结果类型
    fn add(self, rhs: Rhs) -> Self::Output;
}

// Rhs is a generic parameter — you can add different types to Meters:
// Rhs 是一个泛型参数 —— 你可以向 Meters 添加不同的类型：
struct Meters(f64);
struct Centimeters(f64);

impl Add<Meters> for Meters {
    type Output = Meters;
    fn add(self, rhs: Meters) -> Meters { Meters(self.0 + rhs.0) }
}
impl Add<Centimeters> for Meters {
    type Output = Meters;
    fn add(self, rhs: Centimeters) -> Meters { Meters(self.0 + rhs.0 / 100.0) }
}
```

### Generic Associated Types (GATs) / 泛型关联类型 (GAT)

Since Rust 1.65, associated types can have generic parameters of their own. This enables **lending iterators** — iterators that return references tied to the iterator rather than to the underlying collection:

从 Rust 1.65 开始，关联类型可以拥有自己的泛型参数。这使得 **借用迭代器（lending iterators）** 成为可能 —— 这种迭代器返回的引用绑定到迭代器本身，而不是底层的集合：

```rust
// Without GATs — impossible to express a lending iterator:
// 没有 GAT —— 无法表达借用迭代器：
// trait LendingIterator {
//     type Item<'a>;  // ← This was rejected before 1.65 / 1.65 之前被拒绝
// }

// With GATs (Rust 1.65+):
// 使用 GAT (Rust 1.65+):
trait LendingIterator {
    type Item<'a> where Self: 'a;

    fn next(&mut self) -> Option<Self::Item<'_>>;
}

// Example: an iterator that yields overlapping windows
// 示例：一个产生重叠窗口的迭代器
struct WindowIter<'data> {
    data: &'data [u8],
    pos: usize,
    window_size: usize,
}

impl<'data> LendingIterator for WindowIter<'data> {
    type Item<'a> = &'a [u8] where Self: 'a;

    fn next(&mut self) -> Option<&[u8]> {
        if self.pos + self.window_size <= self.data.len() {
            let window = &self.data[self.pos..self.pos + self.window_size];
            self.pos += 1;
            Some(window)
        } else {
            None
        }
    }
}
```

> **When you need GATs / 何时需要 GAT**：Lending iterators, streaming parsers, or any trait where the associated type's lifetime depends on the `&self` borrow. For most code, plain associated types are sufficient.
>
> **何时需要 GAT**：借用迭代器、流式解析器，或者任何关联类型的生命周期依赖于 `&self` 借用的 trait。对于大多数代码，普通的关联类型就足够了。

### Supertraits and Trait Hierarchies / Supertrait 与 Trait 层次结构

Traits can require other traits as prerequisites, forming hierarchies:

Trait 可以要求其他 trait 作为先决条件，从而形成层次结构：

```mermaid
graph BT
    Display["Display"]
    Debug["Debug"]
    Error["Error"]
    Clone["Clone"]
    Copy["Copy"]
    PartialEq["PartialEq"]
    Eq["Eq"]
    PartialOrd["PartialOrd"]
    Ord["Ord"]

    Error --> Display
    Error --> Debug
    Copy --> Clone
    Eq --> PartialEq
    Ord --> Eq
    Ord --> PartialOrd
    PartialOrd --> PartialEq

    style Display fill:#e8f4f8,stroke:#2980b9,color:#000
    style Debug fill:#e8f4f8,stroke:#2980b9,color:#000
    style Error fill:#fdebd0,stroke:#e67e22,color:#000
    style Clone fill:#d4efdf,stroke:#27ae60,color:#000
    style Copy fill:#d4efdf,stroke:#27ae60,color:#000
    style PartialEq fill:#fef9e7,stroke:#f1c40f,color:#000
    style Eq fill:#fef9e7,stroke:#f1c40f,color:#000
    style PartialOrd fill:#fef9e7,stroke:#f1c40f,color:#000
    style Ord fill:#fef9e7,stroke:#f1c40f,color:#000
```

> Arrows point from subtrait to supertrait: implementing `Error` requires `Display` + `Debug`.
>
> 箭头从子 trait 指向 supertrait：实现 `Error` 需要同时实现 `Display` 和 `Debug`。

A trait can require that implementors also implement other traits:

Trait 可以要求实现者同时实现其他 trait：

```rust
use std::fmt;

// Display is a supertrait of Error / Display 是 Error 的 supertrait
trait Error: fmt::Display + fmt::Debug {
    fn source(&self) -> Option<&(dyn Error + 'static)> { None }
}
// Any type implementing Error MUST also implement Display and Debug
// 任何实现 Error 的类型也必须实现 Display 和 Debug

// Build your own hierarchies / 构建你自己的层次结构：
trait Identifiable {
    fn id(&self) -> u64;
}

trait Timestamped {
    fn created_at(&self) -> chrono::DateTime<chrono::Utc>;
}

// Entity requires both / Entity 同时需要这两者：
trait Entity: Identifiable + Timestamped {
    fn is_active(&self) -> bool;
}

// Implementing Entity forces you to implement all three:
// 实现 Entity 会强制你实现全部这三个 trait：
struct User { id: u64, name: String, created: chrono::DateTime<chrono::Utc> }

impl Identifiable for User {
    fn id(&self) -> u64 { self.id }
}
impl Timestamped for User {
    fn created_at(&self) -> chrono::DateTime<chrono::Utc> { self.created }
}
impl Entity for User {
    fn is_active(&self) -> bool { true }
}
```

### Blanket Implementations / Blanket 实现

Implement a trait for ALL types that satisfy some bound:

为满足某些约束的所有类型实现一个 trait：

```rust
// std does this: any type that implements Display automatically gets ToString
// 标准库的工作方式：任何实现了 Display 的类型都会自动获得 ToString
impl<T: fmt::Display> ToString for T {
    fn to_string(&self) -> String {
        format!("{self}")
    }
}
// Now i32, &str, your custom types — anything with Display — gets to_string() for free.
// 现在 i32、&str 以及你的自定义类型 —— 只要有 Display，就能免费获得 to_string()。

// Your own blanket impl / 你自己的 blanket 实现：
trait Loggable {
    fn log(&self);
}

// Every Debug type is automatically Loggable / 每个 Debug 类型都会自动成为 Loggable：
impl<T: std::fmt::Debug> Loggable for T {
    fn log(&self) {
        eprintln!("[LOG] {self:?}");
    }
}

// Now ANY Debug type has .log() / 现在任何 Debug 类型都有了 .log() 方法：
// 42.log();              // [LOG] 42
// "hello".log();         // [LOG] "hello"
// vec![1, 2, 3].log();   // [LOG] [1, 2, 3]
```

> **Caution / 注意**：Blanket impls are powerful but irreversible — you can't add a
> more specific impl for a type that's already covered by a blanket impl
> (orphan rules + coherence). Design them carefully.
>
> **注意**：Blanket 实现非常强大，但也是不可逆的 —— 你不能为一个已经被 blanket 实现覆盖的类型添加更具体的实现（受限于孤儿规则和一致性）。请谨慎设计。

### Marker Traits / 标记 Trait (Marker Traits)

Traits with no methods — they mark a type as having some property:

不包含任何方法的 trait —— 它们将某个类型标记为具有特定属性：

```rust
// Standard library marker traits / 标准库中的标记 trait：
// Send    — safe to transfer between threads / 可以安全地在线程间转移
// Sync    — safe to share (&T) between threads / 可以安全地在线程间共享 (&T)
// Unpin   — safe to move after pinning / pin 后仍然可以安全移动
// Sized   — has a known size at compile time / 编译时具有已知大小
// Copy    — can be duplicated with memcpy / 可以通过 memcpy 复制

// Your own marker trait / 你自己的标记 trait：
/// Marker: this sensor has been factory-calibrated / 标记：该传感器已通过工厂校准
trait Calibrated {}

struct RawSensor { reading: f64 }
struct CalibratedSensor { reading: f64 }

impl Calibrated for CalibratedSensor {}

// Only calibrated sensors can be used in production:
// 只有经过校准的传感器才能在生产环境中使用：
fn record_measurement<S: Calibrated>(sensor: &S) {
    // ...
}
// record_measurement(&RawSensor { reading: 0.0 }); // ❌ Compile error / 编译错误
// record_measurement(&CalibratedSensor { reading: 0.0 }); // ✅
```

This connects directly to the **type-state pattern** in Chapter 3.

这与第 3 章中的 **状态类型模式 (type-state pattern)** 直接相关。

### Trait Object Safety Rules / Trait 对象安全规则

Not every trait can be used as `dyn Trait`. A trait is **object-safe** only if:

并非所有的 trait 都可以作为 `dyn Trait` 使用。一个 trait 只有在满足以下条件时才是 **对象安全（object-safe）** 的：

1.  **No `Self: Sized` bound** on the trait itself / trait 本身没有 `Self: Sized` 约束
2.  **No generic type parameters** on methods / 方法上没有泛型参数
3.  **No use of `Self` in return position** (except via indirection like `Box<Self>`) / 在返回位置没有使用 `Self`（通过 `Box<Self>` 等间接方式除外）
4.  **No associated functions** (methods must have `&self`, `&mut self`, or `self`) / 没有关联函数（方法必须带有 `&self`、`&mut self` 或 `self`）

```rust
// ✅ Object-safe — can be used as dyn Drawable
// ✅ 对象安全 —— 可以用作 dyn Drawable
trait Drawable {
    fn draw(&self);
    fn bounding_box(&self) -> (f64, f64, f64, f64);
}

let shapes: Vec<Box<dyn Drawable>> = vec![/* ... */]; // ✅ Works / 行得通

// ❌ NOT object-safe — uses Self in return position
// ❌ 不对象安全 —— 在返回位置使用了 Self
trait Cloneable {
    fn clone_self(&self) -> Self;
    //                       ^^^^ Can't know the concrete size at runtime / 运行时无法知道具体大小
}
// let items: Vec<Box<dyn Cloneable>> = ...; // ❌ Compile error / 编译错误

// ❌ NOT object-safe — generic method
// ❌ 不对象安全 —— 泛型方法
trait Converter {
    fn convert<T>(&self) -> T;
    //        ^^^ The vtable can't contain infinite monomorphizations
}

// ❌ NOT object-safe — associated function (no self)
trait Factory {
    fn create() -> Self;
    // No &self — how would you call this through a trait object?
}
```

**Workarounds**:

```rust
// Add `where Self: Sized` to exclude a method from the vtable:
trait MyTrait {
    fn regular_method(&self); // Included in vtable

    fn generic_method<T>(&self) -> T
    where
        Self: Sized; // Excluded from vtable — can't be called via dyn MyTrait
}

// Now dyn MyTrait is valid, but generic_method can only be called
// when the concrete type is known.
// 现在 dyn MyTrait 是有效的，但 generic_method 只能在具体类型已知时调用。
```

> **Rule of thumb / 经验法则**：If you plan to use `dyn Trait`, keep methods simple — no generics, no `Self` in return types, no `Sized` bounds. When in doubt, try `let _: Box<dyn YourTrait>;` and let the compiler tell you.
>
> **经验法则**：如果你计划使用 `dyn Trait`，请保持方法简单 —— 不要使用泛型，不要在返回类型中使用 `Self`，不要使用 `Sized` 约束。如果不确定，试着写一行 `let _: Box<dyn YourTrait>;`，让编译器告诉你答案。

### Trait Objects Under the Hood — vtables and Fat Pointers / Trait 对象底层原理 —— vtable 与脂肪指针

A `&dyn Trait` (or `Box<dyn Trait>`) is a **fat pointer** — two machine words:

`&dyn Trait`（或 `Box<dyn Trait>`）是一个 **脂肪指针（fat pointer）** —— 包含两个机器字（machine words）：

```text
┌──────────────────────────────────────────────────┐
│  &dyn Drawable (on 64-bit: 16 bytes total)       │
│  &dyn Drawable (在 64 位系统上：总共 16 字节)      │
├──────────────┬───────────────────────────────────┤
│  data_ptr    │  vtable_ptr                       │
│  (8 bytes)   │  (8 bytes)                        │
│  ↓           │  ↓                                │
│  ┌─────────┐ │  ┌──────────────────────────────┐ │
│  │ Circle  │ │  │ vtable for <Circle as        │ │
│  │ {       │ │  │           Drawable>          │ │
│  │  r: 5.0 │ │  │ <Circle as Drawable> 的 vtable │ │
│  │ }       │ │  │                              │ │
│  │         │ │  │  drop_in_place: 0x7f...a0    │ │
│  └─────────┘ │  │  size:           8           │ │
│              │  │  align:          8           │ │
│              │  │  draw:          0x7f...b4    │ │
│              │  │  bounding_box:  0x7f...c8    │ │
│              │  └──────────────────────────────┘ │
└──────────────┴───────────────────────────────────┘
```

**How a vtable call works / vtable 调用是如何工作的** (e.g., `shape.draw()`):

1.  Load `vtable_ptr` from the fat pointer (second word) / 从脂肪指针中加载 `vtable_ptr`（第二个字）
2.  Index into the vtable to find the `draw` function pointer / 在 vtable 中索引以找到 `draw` 函数指针
3.  Call it, passing `data_ptr` as the `self` argument / 调用它，并将 `data_ptr` 作为 `self` 参数传递

This is similar to C++ virtual dispatch in cost (one pointer indirection per call), but Rust stores the vtable pointer in the fat pointer rather than inside the object — so a plain `Circle` on the stack carries no vtable pointer at all.

这在开销上与 C++ 的虚函数分发类似（每次调用一次指针间接跳转），但 Rust 将 vtable 指针存储在脂肪指针中，而不是对象内部 —— 因此栈上普通的 `Circle` 根本不携带 vtable 指针。

```rust
trait Drawable {
    fn draw(&self);
    fn area(&self) -> f64;
}

struct Circle { radius: f64 }

impl Drawable for Circle {
    fn draw(&self) { println!("Drawing circle r={}", self.radius); }
    fn area(&self) -> f64 { std::f64::consts::PI * self.radius * self.radius }
}

struct Square { side: f64 }

impl Drawable for Square {
    fn draw(&self) { println!("Drawing square s={}", self.side); }
    fn area(&self) -> f64 { self.side * self.side }
}

fn main() {
    let shapes: Vec<Box<dyn Drawable>> = vec![
        Box::new(Circle { radius: 5.0 }),
        Box::new(Square { side: 3.0 }),
    ];

    // Each element is a fat pointer: (data_ptr, vtable_ptr)
    // The vtable for Circle and Square are DIFFERENT
    // 每个元素都是一个脂肪指针：(data_ptr, vtable_ptr)
    // Circle 和 Square 的 vtable 是不同的
    for shape in &shapes {
        shape.draw();  // vtable dispatch → Circle::draw or Square::draw
        println!("  area = {:.2}", shape.area());
    }

    // Size comparison / 大小比较：
    println!("size_of::<&Circle>()        = {}", size_of::<&Circle>());
    // → 8 bytes (one pointer — the compiler knows the type) / 8 字节（一个指针 —— 编译器知道具体类型）
    println!("size_of::<&dyn Drawable>()  = {}", size_of::<&dyn Drawable>());
    // → 16 bytes (data_ptr + vtable_ptr) / 16 字节 (data_ptr + vtable_ptr)
}
```

**Performance cost model / 性能代价模型**：

| Aspect / 方面 | Static dispatch / 静态分发 (`impl Trait` / generics) | Dynamic dispatch / 动态分发 (`dyn Trait`) |
|--------|------------------------------------------|-------------------------------|
| Call overhead / 调用开销 | Zero — inlined by LLVM / 零 —— 由 LLVM 内联 | One pointer indirection per call / 每次调用一次指针间接跳转 |
| Inlining / 内联 | ✅ Compiler can inline / 编译器可以内联 | ❌ Opaque function pointer / 不透明的函数指针 |
| Binary size / 二进制大小 | Larger (one copy per type) / 较大（每个类型一份副本） | Smaller (one shared function) / 较小（一个共享函数） |
| Pointer size / 指针大小 | Thin (1 word) / 细指针（1 个字） | Fat (2 words) / 脂肪指针（2 个字） |
| Heterogeneous collections / 异构集合 | ❌ | ✅ `Vec<Box<dyn Trait>>` |

> **When vtable cost matters / 何时需要考虑 vtable 开销**：In tight loops calling a trait method millions of times, the indirection and inability to inline can be significant (2-10× slower). For cold paths, configuration, or plugin architectures, the flexibility of `dyn Trait` is worth the small cost.
>
> **何时需要考虑 vtable 开销**：在数百万次调用 trait 方法的紧凑循环中，间接跳转和无法内联的影响可能非常显著（慢 2-10 倍）。对于冷代码路径、配置或插件架构，`dyn Trait` 的灵活性值得这点微小的开销。

### Higher-Ranked Trait Bounds (HRTBs) / 高阶 Trait 约束 (HRTB)

Sometimes you need a function that works with references of *any* lifetime, not a specific one. This is where `for<'a>` syntax appears:

有时你需要一个能处理 *任何* 生命周期的引用的函数，而不仅仅是某个特定生命周期。这就是 `for<'a>` 语法的用武之地：

```rust
// Problem: this function needs a closure that can process
// references with ANY lifetime, not just one specific lifetime.
// 问题：该函数需要一个能够处理任何生命周期引用的闭包。

// ❌ This is too restrictive — 'a is fixed by the caller:
// fn apply<'a, F: Fn(&'a str) -> &'a str>(f: F, data: &'a str) -> &'a str

// ✅ HRTB: F must work for ALL possible lifetimes:
fn apply<F>(f: F, data: &str) -> &str
where
    F: for<'a> Fn(&'a str) -> &'a str,
{
    f(data)
}

fn main() {
    let result = apply(|s| s.trim(), "  hello  ");
    println!("{result}"); // "hello"
}
```

**When you encounter HRTBs / 何时会遇到 HRTB**：
- `Fn(&T) -> &U` traits — the compiler infers `for<'a>` automatically in most cases / `Fn(&T) -> &U` trait —— 编译器在大多数情况下会自动推断 `for<'a>`。
- Custom trait implementations that must work across different borrows / 必须跨不同借用工作的自定义 trait 实现。
- Deserialization with `serde`: `for<'de> Deserialize<'de>` / 使用 `serde` 进行反序列化：`for<'de> Deserialize<'de>`。

```rust,ignore
// serde's DeserializeOwned is defined as:
// trait DeserializeOwned: for<'de> Deserialize<'de> {}
// Meaning: "can be deserialized from data with ANY lifetime"
// (i.e., the result doesn't borrow from the input)
// 含义：“可以从具有任何生命周期的数据中反序列化”
// （即：结果不从输入中借用）

use serde::de::DeserializeOwned;

fn parse_json<T: DeserializeOwned>(input: &str) -> T {
    serde_json::from_str(input).unwrap()
}
```

> **Practical advice / 建议**：You'll rarely write `for<'a>` yourself. It mostly appears in trait bounds on closure parameters, where the compiler handles it implicitly. But recognizing it in error messages ("expected a `for<'a> Fn(&'a ...)` bound") helps you understand what the compiler is asking for.
>
> 你很少需要亲手编写 `for<'a>`。它大多出现在闭包参数的 trait 约束中，编译器会隐式处理。但在错误消息中识别出它（“expected a `for<'a> Fn(&'a ...)` bound”）有助于你理解编译器的要求。

    serde_json::from_str(input).unwrap()
}
```

> **Practical advice / 建议**：You'll rarely write `for<'a>` yourself. It mostly appears in trait bounds on closure parameters, where the compiler handles it implicitly. But recognizing it in error messages ("expected a `for<'a> Fn(&'a ...)` bound") helps you understand what the compiler is asking for.
>
> 你很少需要亲手编写 `for<'a>`。它大多出现在闭包参数的 trait 约束中，编译器会隐式处理。但在错误消息中识别出它（“expected a `for<'a> Fn(&'a ...)` bound”）有助于你理解编译器的要求。

### `impl Trait` — Argument Position vs Return Position / `impl Trait` —— 参数位置 vs 返回位置

`impl Trait` appears in two positions with **different semantics**:

`impl Trait` 出现在两个位置，具有 **不同的语义**：

```rust
// --- Argument-Position impl Trait (APIT) ---
// --- 参数位置的 impl Trait (APIT) ---
// "Caller chooses the type" — syntactic sugar for a generic parameter
// “调用者选择类型” —— 泛型参数的语法糖
fn print_all(items: impl Iterator<Item = i32>) {
    for item in items { println!("{item}"); }
}
// Equivalent to / 等同于：
fn print_all_verbose<I: Iterator<Item = i32>>(items: I) {
    for item in items { println!("{item}"); }
}
// Caller decides / 调用者决定：
// print_all(vec![1,2,3].into_iter())
// print_all(0..10)

// --- Return-Position impl Trait (RPIT) ---
// --- 返回位置的 impl Trait (RPIT) ---
// "Callee chooses the type" — the function picks one concrete type
// “被调用者（函数）选择类型” —— 函数选择一个具体的类型
fn evens(limit: i32) -> impl Iterator<Item = i32> {
    (0..limit).filter(|x| x % 2 == 0)
    // The concrete type is Filter<Range<i32>, Closure>
    // but the caller only sees "some Iterator<Item = i32>"
    // 具体类型是 Filter<Range<i32>, Closure>
    // 但调用者只看到“某种 Iterator<Item = i32>”
}
```

**Key difference / 关键点比较**：

| | APIT (`fn foo(x: impl T)`) | RPIT (`fn foo() -> impl T`) |
|---|---|---|
| Who picks the type? / 谁来选择类型？ | Caller (调用者) | Callee (函数体) |
| Monomorphized? / 是否单态化？ | Yes — one copy per type / 是 —— 每个类型一份副本 | Yes — one concrete type / 是 —— 对应一个具体类型 |
| Turbofish? / Turbo 鱼语法？ | No (`foo::<X>()` not allowed) / 否 (不允许 `foo::<X>()`) | N/A |
| Equivalent to / 类似于 | `fn foo<X: T>(x: X)` | Existential type / 存在类型 |

#### RPIT in Trait Definitions (RPITIT) / Trait 定义中的 RPIT (RPITIT)

Since Rust 1.75, you can use `-> impl Trait` directly in trait definitions:

从 Rust 1.75 开始，你可以直接在 trait 定义中使用 `-> impl Trait`：

```rust
trait Container {
    fn items(&self) -> impl Iterator<Item = &str>;
    //                 ^^^^ Each implementor returns its own concrete type
    //                 ^^^^ 每个实现者返回其自己的具体类型
}

struct CsvRow {
    fields: Vec<String>,
}

impl Container for CsvRow {
    fn items(&self) -> impl Iterator<Item = &str> {
        self.fields.iter().map(String::as_str)
    }
}

struct FixedFields;

impl Container for FixedFields {
    fn items(&self) -> impl Iterator<Item = &str> {
        ["host", "port", "timeout"].into_iter()
    }
}
```

> **Before Rust 1.75**, you had to use `Box<dyn Iterator>` or an associated type to achieve this in traits. RPITIT removes the allocation.
>
> **在 Rust 1.75 之前**，你必须使用 `Box<dyn Iterator>` 或关联类型才能在 trait 中实现该功能。RPITIT 消除了堆内存分配。

#### `impl Trait` vs `dyn Trait` — Decision Guide / `impl Trait` vs `dyn Trait` —— 决策指南

```text
Do you know the concrete type at compile time? / 编译时是否知道具体类型？
├── YES (是) → Use impl Trait or generics (zero cost, inlinable) / 使用 impl Trait 或泛型（零开销，可内联）
└── NO (否)  → Do you need a heterogeneous collection? / 是否需要异构集合？
     ├── YES (是) → Use dyn Trait (Box<dyn T>, &dyn T)
     └── NO (否)  → Do you need the SAME trait object across an API boundary? / 是否需要在 API 边界使用相同的 trait 对象？
          ├── YES (是) → Use dyn Trait
          └── NO (否)  → Use generics / impl Trait
```

| Feature / 特性 | `impl Trait` | `dyn Trait` |
|---------|-------------|------------|
| Dispatch / 分发 | Static (monomorphized) / 静态 (单态化) | Dynamic (vtable) / 动态 (vtable) |
| Performance / 性能 | Best — inlinable / 极佳 —— 可内联 | One indirection per call / 每次调用一次间接跳转 |
| Heterogeneous collections / 异构集合 | ❌ | ✅ |
| Binary size per type / 每个类型的二进制大小 | One copy each / 每个类型一份副本 | Shared code / 代码共享 |
| Trait must be object-safe? / Trait 必须对象安全？ | No / 否 | Yes / 是 |
| Works in trait definitions / 在 trait 定义中可用 | ✅ (Rust 1.75+) | Always / 始终可用 |

***

## Type Erasure with `Any` and `TypeId` / 使用 `Any` 和 `TypeId` 进行类型擦除

Sometimes you need to store values of *unknown* types and downcast them later — a pattern familiar from `void*` in C or `object` in C#. Rust provides this through `std::any::Any`:

有时你需要存储 *未知* 类型的值并在稍后进行向下转型 (downcast) —— 这种模式在 C 语言的 `void*` 或 C# 的 `object` 中很常见。Rust 通过 `std::any::Any` 提供此功能：

```rust
use std::any::Any;

// Store heterogeneous values / 存储异构值：
fn log_value(value: &dyn Any) {
    if let Some(s) = value.downcast_ref::<String>() {
        println!("String: {s}");
    } else if let Some(n) = value.downcast_ref::<i32>() {
        println!("i32: {n}");
    } else {
        // TypeId lets you inspect the type at runtime / TypeId 允许在运行时检查类型：
        println!("Unknown type: {:?}", value.type_id());
    }
}

// Useful for plugin systems, event buses, or ECS-style architectures:
// 对插件系统、事件总线或 ECS 风格的架构非常有用：
struct AnyMap(std::collections::HashMap<std::any::TypeId, Box<dyn Any + Send>>);

impl AnyMap {
    fn new() -> Self { AnyMap(std::collections::HashMap::new()) }

    fn insert<T: Any + Send + 'static>(&mut self, value: T) {
        self.0.insert(std::any::TypeId::of::<T>(), Box::new(value));
    }

    fn get<T: Any + Send + 'static>(&self) -> Option<&T> {
        self.0.get(&std::any::TypeId::of::<T>())?
            .downcast_ref()
    }
}

fn main() {
    let mut map = AnyMap::new();
    map.insert(42_i32);
    map.insert(String::from("hello"));

    assert_eq!(map.get::<i32>(), Some(&42));
    assert_eq!(map.get::<String>().map(|s| s.as_str()), Some("hello"));
    assert_eq!(map.get::<f64>(), None); // Never inserted
}
```

> **When to use `Any` / 何时使用 `Any`**：Plugin/extension systems, type-indexed maps (`typemap`), error downcasting (`anyhow::Error::downcast_ref`). Prefer generics or trait objects when the set of types is known at compile time — `Any` is a last resort that trades compile-time safety for flexibility.
>
> **何时使用 `Any`**：插件/扩展系统、类型索引映射 (`typemap`)、错误向下转型 (`anyhow::Error::downcast_ref`)。如果在编译时已知类型集，请优先使用泛型或 trait 对象 —— `Any` 是最后的手段，它牺牲了编译时安全性以换取灵活性。

***

## Extension Traits — Adding Methods to Types You Don't Own / 扩展 Trait —— 为你不拥有的类型添加方法

Rust's orphan rule prevents you from implementing a foreign trait on a foreign type. Extension traits are the standard workaround: define a **new trait** in your crate whose methods have a blanket implementation for any type that meets a bound. The caller imports the trait and the new methods appear on existing types.

Rust 的孤儿规则（orphan rule）阻止你在不属于你的类型上实现不属于你的 trait。扩展 trait 是标准的解决方法：在你的 crate 中定义一个 **新 trait**，其方法为满足特定约束的任何类型提供 blanket 实现。调用者只需导入该 trait，新方法就会出现在现有类型上。

This pattern is pervasive in the Rust ecosystem: `itertools::Itertools`, `futures::StreamExt`, `tokio::io::AsyncReadExt`, `tower::ServiceExt`.

这种模式在 Rust 生态系统中非常普遍：`itertools::Itertools`、`futures::StreamExt`、`tokio::io::AsyncReadExt`、`tower::ServiceExt`。

### The Problem / 问题所在

```rust
// We want to add a .mean() method to all iterators that yield f64.
// But Iterator is defined in std and f64 is a primitive — orphan rule prevents:
// 我们想为所有产生 f64 的迭代器添加一个 .mean() 方法。
// 但 Iterator 定义在标准库中，f64 是原生类型 —— 孤儿规则阻止了这样做：
//
// impl<I: Iterator<Item = f64>> I {   // ❌ Cannot add inherent methods to a foreign type
//                                     // ❌ 无法为外部类型添加固有方法
//     fn mean(self) -> f64 { ... }
// }
```

### The Solution: An Extension Trait / 解决方案：扩展 Trait

```rust
/// Extension methods for iterators over numeric values / 数值类型迭代器的扩展方法。
pub trait IteratorExt: Iterator {
    /// Computes the arithmetic mean. Returns `None` for empty iterators.
    /// 计算算术平均值。如果是空迭代器则返回 `None`。
    fn mean(self) -> Option<f64>
    where
        Self: Sized,
        Self::Item: Into<f64>;
}

// Blanket implementation — automatically applies to ALL iterators
// Blanket 实现 —— 自动应用于所有迭代器
impl<I: Iterator> IteratorExt for I {
    fn mean(self) -> Option<f64>
    where
        Self: Sized,
        Self::Item: Into<f64>,
    {
        let mut sum: f64 = 0.0;
        let mut count: u64 = 0;
        for item in self {
            sum += item.into();
            count += 1;
        }
        if count == 0 { None } else { Some(sum / count as f64) }
    }
}

// Usage — just import the trait / 使用 —— 导入该 trait 即可：
use crate::IteratorExt;  // One import and the method appears on all iterators / 导入后，该方法就出现在所有迭代器上了

fn analyze_temperatures(readings: &[f64]) -> Option<f64> {
    readings.iter().copied().mean()  // .mean() is now available! / .mean() 现在可用了！
}

fn analyze_sensor_data(data: &[i32]) -> Option<f64> {
    data.iter().copied().mean()  // Works on i32 too (i32: Into<f64>) / 对 i32 同样有效
}
```

### Real-World Example: Diagnostic Result Extensions / 现实世界示例：诊断结果扩展

```rust
use std::collections::HashMap;

struct DiagResult {
    component: String,
    passed: bool,
    message: String,
}

/// Extension trait for Vec<DiagResult> — adds domain-specific analysis methods.
/// Vec<DiagResult> 的扩展 trait —— 添加特定领域分析方法。
pub trait DiagResultsExt {
    fn passed_count(&self) -> usize;
    fn failed_count(&self) -> usize;
    fn overall_pass(&self) -> bool;
    fn failures_by_component(&self) -> HashMap<String, Vec<&DiagResult>>;
}

impl DiagResultsExt for Vec<DiagResult> {
    fn passed_count(&self) -> usize {
        self.iter().filter(|r| r.passed).count()
    }

    fn failed_count(&self) -> usize {
        self.iter().filter(|r| !r.passed).count()
    }

    fn overall_pass(&self) -> bool {
        self.iter().all(|r| r.passed)
    }

    fn failures_by_component(&self) -> HashMap<String, Vec<&DiagResult>> {
        let mut map = HashMap::new();
        for r in self.iter().filter(|r| !r.passed) {
            map.entry(r.component.clone()).or_default().push(r);
        }
        map
    }
}

// Now any Vec<DiagResult> has these methods / 现在任何 Vec<DiagResult> 都有了这些方法：
fn report(results: Vec<DiagResult>) {
    if !results.overall_pass() {
        let failures = results.failures_by_component();
        for (component, fails) in &failures {
            eprintln!("{component}: {} failures", fails.len());
        }
    }
}
```

### Naming Convention / 命名规范

The Rust ecosystem uses a consistent `Ext` suffix / Rust 生态系统通常使用统一的 `Ext` 后缀：

| Crate / 包 | Extension Trait / 扩展 Trait | Extends / 扩展了 |
|-------|----------------|---------|
| `itertools` | `Itertools` | `Iterator` |
| `futures` | `StreamExt`, `FutureExt` | `Stream`, `Future` |
| `tokio` | `AsyncReadExt`, `AsyncWriteExt` | `AsyncRead`, `AsyncWrite` |
| `tower` | `ServiceExt` | `Service` |
| `bytes` | `BufMut` (partial) | `&mut [u8]` |
| Your crate / 你的包 | `DiagResultsExt` | `Vec<DiagResult>` |

### When to Use / 何时使用

| Situation / 场景 | Use Extension Trait? / 是否使用扩展 Trait？ |
|-----------|:---:|
| Adding convenience methods to a foreign type / 向外部类型添加便利方法 | ✅ |
| Grouping domain-specific logic on generic collections / 在泛型集合上分组特定领域逻辑 | ✅ |
| The method needs access to private fields / 方法需要访问私有字段 | ❌ (use a wrapper/newtype / 使用包装/newtype) |
| The method logically belongs on a new type you control / 方法在逻辑上属于你控制的新类型 | ❌ (just add it to your type / 直接加到你的类型上) |
| You want the method available without any import / 希望无需导入即可使用方法 | ❌ (inherent methods only / 仅限固有方法) |

***

## Enum Dispatch — Static Polymorphism Without `dyn` / 枚举分发 —— 无 `dyn` 的静态多态

When you have a **closed set** of types implementing a trait, you can replace `dyn Trait` with an enum whose variants hold the concrete types. This eliminates the vtable indirection and heap allocation while preserving the same caller-facing interface.

当你有一组 **封闭集合** 的类型实现某个 trait 时，可以用一个变体持有具体类型的枚举来替换 `dyn Trait`。这消除了 vtable 间接跳转和堆分配，同时保留了相同的面向调用者的接口。

### The Problem with `dyn Trait` / `dyn Trait` 的问题

```rust
trait Sensor {
    fn read(&self) -> f64;
    fn name(&self) -> &str;
}

struct Gps { lat: f64, lon: f64 }
struct Thermometer { temp_c: f64 }
struct Accelerometer { g_force: f64 }

impl Sensor for Gps {
    fn read(&self) -> f64 { self.lat }
    fn name(&self) -> &str { "GPS" }
}
impl Sensor for Thermometer {
    fn read(&self) -> f64 { self.temp_c }
    fn name(&self) -> &str { "Thermometer" }
}
impl Sensor for Accelerometer {
    fn read(&self) -> f64 { self.g_force }
    fn name(&self) -> &str { "Accelerometer" }
}

// Heterogeneous collection with dyn — works, but has costs:
// 使用 dyn 的异构集合 —— 虽然可行，但有开销：
fn read_all_dyn(sensors: &[Box<dyn Sensor>]) -> Vec<f64> {
    sensors.iter().map(|s| s.read()).collect()
    // Each .read() goes through a vtable indirection / 每次 .read() 都经过 vtable 间接跳转
    // Each Box allocates on the heap / 每个 Box 都在堆上分配
}
```

### The Enum Dispatch Solution / 枚举分发解决方案

```rust
// Replace the trait object with an enum / 用枚举替换 trait 对象：
enum AnySensor {
    Gps(Gps),
    Thermometer(Thermometer),
    Accelerometer(Accelerometer),
}

impl AnySensor {
    fn read(&self) -> f64 {
        match self {
            AnySensor::Gps(s) => s.read(),
            AnySensor::Thermometer(s) => s.read(),
            AnySensor::Accelerometer(s) => s.read(),
        }
    }

    fn name(&self) -> &str {
        match self {
            AnySensor::Gps(s) => s.name(),
            AnySensor::Thermometer(s) => s.name(),
            AnySensor::Accelerometer(s) => s.name(),
        }
    }
}

// Now: no heap allocation, no vtable, stored inline
// 现在：没有堆分配，没有 vtable，内联存储
fn read_all(sensors: &[AnySensor]) -> Vec<f64> {
    sensors.iter().map(|s| s.read()).collect()
    // Each .read() is a match branch — compiler can inline everything
    // 每个 .read() 都是一个 match 分支 —— 编译器可以内联所有内容
}

fn main() {
    let sensors = vec![
        AnySensor::Gps(Gps { lat: 47.6, lon: -122.3 }),
        AnySensor::Thermometer(Thermometer { temp_c: 72.5 }),
        AnySensor::Accelerometer(Accelerometer { g_force: 1.02 }),
    ];

    for sensor in &sensors {
        println!("{}: {:.2}", sensor.name(), sensor.read());
    }
}
```

### Implement the Trait on the Enum / 在枚举上实现 Trait

For interoperability, you can implement the original trait on the enum itself:

为了实现互操作性，你可以直接在枚举上实现原始 trait：

```rust
impl Sensor for AnySensor {
    fn read(&self) -> f64 {
        match self {
            AnySensor::Gps(s) => s.read(),
            AnySensor::Thermometer(s) => s.read(),
            AnySensor::Accelerometer(s) => s.read(),
        }
    }

    fn name(&self) -> &str {
        match self {
            AnySensor::Gps(s) => s.name(),
            AnySensor::Thermometer(s) => s.name(),
            AnySensor::Accelerometer(s) => s.name(),
        }
    }
}

// Now AnySensor works anywhere a Sensor is expected via generics:
// 现在 AnySensor 可以像泛型一样在任何需要 Sensor 的地方使用：
fn report<S: Sensor>(s: &S) {
    println!("{}: {:.2}", s.name(), s.read());
}
```

### Reducing Boilerplate with a Macro / 使用宏减少样板代码

The match-arm delegation is repetitive. A macro eliminates it:

match 分支的委托是重复性的。使用宏可以消除这一点：

```rust
macro_rules! dispatch_sensor {
    ($self:expr, $method:ident $(, $arg:expr)*) => {
        match $self {
            AnySensor::Gps(s) => s.$method($($arg),*),
            AnySensor::Thermometer(s) => s.$method($($arg),*),
            AnySensor::Accelerometer(s) => s.$method($($arg),*),
        }
    };
}

impl Sensor for AnySensor {
    fn read(&self) -> f64     { dispatch_sensor!(self, read) }
    fn name(&self) -> &str    { dispatch_sensor!(self, name) }
}
```

For larger projects, the `enum_dispatch` crate automates this entirely:

对于较大的项目，`enum_dispatch` crate 可以将此过程完全自动化：

```rust
use enum_dispatch::enum_dispatch;

#[enum_dispatch]
trait Sensor {
    fn read(&self) -> f64;
    fn name(&self) -> &str;
}

#[enum_dispatch(Sensor)]
enum AnySensor {
    Gps,
    Thermometer,
    Accelerometer,
}
// All delegation code is generated automatically / 所有的委托代码都会自动生成
```

### `dyn Trait` vs Enum Dispatch — Decision Guide / `dyn Trait` vs 枚举分发 —— 决策指南

```text
Is the set of types closed (known at compile time)? / 类型集是封闭的吗（在编译时已知）？
├── YES (是) → Prefer enum dispatch (faster, no heap allocation) / 优先选择枚举分发（更快、无堆分配）
│         ├── Few variants (< ~20)?     → Manual enum / 变体较少（< ~20）？ → 手动编写枚举
│         └── Many variants or growing? → enum_dispatch crate / 变体较多或不断增加？ → 使用 enum_dispatch
└── NO (否)  → Must use dyn Trait (plugins, user-provided types) / 必须使用 dyn Trait（插件、用户提供的类型）
```

| Property / 属性 | `dyn Trait` | Enum Dispatch / 枚举分发 |
|----------|:-----------:|:-------------:|
| Dispatch cost / 分发开销 | Vtable indirection (~2ns) / vtable 间接跳转 (~2ns) | Branch prediction (~0.3ns) / 分支预测 (~0.3ns) |
| Heap allocation / 堆分配 | Usually (Box) / 通常需要 (Box) | None (inline) / 无 (内联) |
| Cache-friendly / 缓存友好性 | No (pointer chasing) / 否 (指针追逐) | Yes (contiguous) / 是 (连续存储) |
| Open to new types / 是否支持新类型 | ✅ (anyone can impl) / 是 (任何人均可实现) | ❌ (closed set) / 否 (封闭集合) |
| Code size / 代码大小 | Shared / 共享 | One copy per variant / 每个变体一份副本 |
| Trait safe? / 是否必须对象安全？ | Yes / 是 | No / 否 |
| Adding a variant / 添加变体 | No code changes / 无需修改代码 | Update enum + match arms / 需要更新枚举和 match 分支 |

### When to Use Enum Dispatch / 何时使用枚举分发

| Scenario / 场景 | Recommendation / 建议 |
|----------|---------------|
| Diagnostic test types (CPU, GPU, NIC, Memory, ...) / 诊断测试类型 | ✅ Enum dispatch — closed set, known at compile time / 封闭集合，编译时已知 |
| Bus protocols (SPI, I2C, UART, ...) / 总线协议 | ✅ Enum dispatch or Config trait / 枚举分发或 Config trait |
| Plugin system (user loads .so at runtime) / 插件系统（运行时加载） | ❌ Use `dyn Trait` / 使用 `dyn Trait` |
| 2-3 variants / 2-3 个变体 | ✅ Manual enum dispatch / 手动枚举分发 |
| 10+ variants with many methods / 10 个以上变体且具有多个方法 | ✅ `enum_dispatch` crate |
| Performance-critical inner loop / 性能关键的内层循环 | ✅ Enum dispatch (eliminates vtable) / 消除 vtable |

***

## Capability Mixins — Associated Types as Zero-Cost Composition / 能力注入 (Capability Mixins) —— 作为零成本组合的关联类型

Ruby developers compose behaviour with **mixins** — `include SomeModule` injects methods into a class. Rust traits with **associated types + default methods + blanket impls** produce the same result, except:

Ruby 开发者使用 **mixins** 来组合行为 —— `include SomeModule` 会将方法注入到类中。具有 **关联类型 (associated types) + 默认方法 (default methods) + blanket 实现 (blanket impls)** 的 Rust trait 也能达到同样的效果，但有以下不同：

*   Everything resolves at **compile time** — no method-missing surprises / 所有的内容都在 **编译时** 解析 —— 不会有“方法缺失”的意外
*   Each associated type is a **knob** that changes what the default methods produce / 每个关联类型都是一个 **控制旋钮**，用来改变默认方法产生的结果
*   The compiler **monomorphises** each combination — zero vtable overhead / 编译器 **单态化** 每种组合 —— 零 vtable 开销

### The Problem: Cross-Cutting Bus Dependencies / 问题：横向切割的总线依赖

Hardware diagnostic routines share common operations — read an IPMI sensor, toggle a GPIO rail, sample a temperature over SPI — but different diagnostics need different combinations. Inheritance hierarchies don't exist in Rust. Passing every bus handle as a function argument creates unwieldy signatures. We need a way to **mix in** bus capabilities à la carte.

硬件诊断程序共享一些常见操作 —— 读取 IPMI 传感器、切换 GPIO 栏、通过 SPI 采样温度 —— 但不同的诊断需要不同的组合。Rust 中不存在继承层次结构。将每个总线句柄作为函数参数传递会产生笨重的签名。我们需要一种能够按需 **注入 (mix in)** 总线能力的方法。

### Step 1 — Define "Ingredient" Traits / 第 1 步 —— 定义“组件” Trait

Each ingredient provides one hardware capability via an associated type:

每个组件通过关联类型提供一种硬件能力：

```rust
use std::io;

// ── Bus abstractions (traits the hardware team provides) ──────────
// ── 总线抽象（硬件团队提供的 trait） ──────────
pub trait SpiBus {
    fn spi_transfer(&self, tx: &[u8], rx: &mut [u8]) -> io::Result<()>;
}

pub trait I2cBus {
    fn i2c_read(&self, addr: u8, reg: u8, buf: &mut [u8]) -> io::Result<()>;
    fn i2c_write(&self, addr: u8, reg: u8, data: &[u8]) -> io::Result<()>;
}

pub trait GpioPin {
    fn set_high(&self) -> io::Result<()>;
    fn set_low(&self) -> io::Result<()>;
    fn read_level(&self) -> io::Result<bool>;
}

pub trait IpmiBmc {
    fn raw_command(&self, net_fn: u8, cmd: u8, data: &[u8]) -> io::Result<Vec<u8>>;
    fn read_sensor(&self, sensor_id: u8) -> io::Result<f64>;
}

// ── Ingredient traits — one per bus, carries an associated type ───
// ── 组件 trait —— 每个总线一个，携带一个关联类型 ───
pub trait HasSpi {
    type Spi: SpiBus;
    fn spi(&self) -> &Self::Spi;
}

pub trait HasI2c {
    type I2c: I2cBus;
    fn i2c(&self) -> &Self::I2c;
}

pub trait HasGpio {
    type Gpio: GpioPin;
    fn gpio(&self) -> &Self::Gpio;
}

pub trait HasIpmi {
    type Ipmi: IpmiBmc;
    fn ipmi(&self) -> &Self::Ipmi;
}
```

Each ingredient is tiny, generic, and testable in isolation.

每个组件都非常微小、通用，且可以独立测试。

### Step 2 — Define "Mixin" Traits / 第 2 步 —— 定义“注入 (Mixin)” Trait

A mixin trait declares its required ingredients as supertraits, then provides all its methods via **defaults** — implementors get them for free:

注入 trait 将其所需的组件声明为 supertrait，然后通过 **默认方法 (defaults)** 提供其所有方法 —— 实现者可以免费获得它们：

```rust
/// Mixin: fan diagnostics — needs I2C (tachometer) + GPIO (PWM enable)
/// 注入：风扇诊断 —— 需要 I2C（转速计）+ GPIO（PWM 使能）
pub trait FanDiagMixin: HasI2c + HasGpio {
    /// Read fan RPM from the tachometer IC over I2C.
    /// 通过 I2C 从转速计 IC 读取风扇转速 (RPM)。
    fn read_fan_rpm(&self, fan_id: u8) -> io::Result<u32> {
        let mut buf = [0u8; 2];
        self.i2c().i2c_read(0x48 + fan_id, 0x00, &mut buf)?;
        Ok(u16::from_be_bytes(buf) as u32 * 60) // tach counts → RPM
    }

    /// Enable or disable the fan PWM output via GPIO.
    /// 通过 GPIO 启用或禁用风扇 PWM 输出。
    fn set_fan_pwm(&self, enable: bool) -> io::Result<()> {
        if enable { self.gpio().set_high() }
        else      { self.gpio().set_low() }
    }

    /// Full fan health check — read RPM + verify within threshold.
    /// 完整的风扇健康检查 —— 读取 RPM + 验证是否在阈值内。
    fn check_fan_health(&self, fan_id: u8, min_rpm: u32) -> io::Result<bool> {
        let rpm = self.read_fan_rpm(fan_id)?;
        Ok(rpm >= min_rpm)
    }
}

/// Mixin: temperature monitoring — needs SPI (thermocouple ADC) + IPMI (BMC sensors)
/// 注入：温度监测 —— 需要 SPI（热电偶 ADC）+ IPMI（BMC 传感器）
pub trait TempMonitorMixin: HasSpi + HasIpmi {
    /// Read a thermocouple via the SPI ADC (e.g. MAX31855).
    /// 通过 SPI ADC（例如 MAX31855）读取热电偶。
    fn read_thermocouple(&self) -> io::Result<f64> {
        let mut rx = [0u8; 4];
        self.spi().spi_transfer(&[0x00; 4], &mut rx)?;
        let raw = i32::from_be_bytes(rx) >> 18; // 14-bit signed
        Ok(raw as f64 * 0.25)
    }

    /// Read a BMC-managed temperature sensor via IPMI.
    /// 通过 IPMI 读取 BMC 管理的温度传感器。
    fn read_bmc_temp(&self, sensor_id: u8) -> io::Result<f64> {
        self.ipmi().read_sensor(sensor_id)
    }

    /// Cross-validate: thermocouple vs BMC must agree within delta.
    /// 交叉验证：热电偶与 BMC 的读数必须在 delta 范围内一致。
    fn validate_temps(&self, sensor_id: u8, max_delta: f64) -> io::Result<bool> {
        let tc = self.read_thermocouple()?;
        let bmc = self.read_bmc_temp(sensor_id)?;
        Ok((tc - bmc).abs() <= max_delta)
    }
}

/// Mixin: power sequencing — needs GPIO (rail enable) + IPMI (event logging)
/// 注入：电源时序 —— 需要 GPIO（导轨使能）+ IPMI（事件日志记录）
pub trait PowerSeqMixin: HasGpio + HasIpmi {
    /// Assert the power-good GPIO and verify via IPMI sensor.
    /// 断言电源良好 (power-good) GPIO 并通过 IPMI 传感器进行验证。
    fn enable_power_rail(&self, sensor_id: u8) -> io::Result<bool> {
        self.gpio().set_high()?;
        std::thread::sleep(std::time::Duration::from_millis(50));
        let voltage = self.ipmi().read_sensor(sensor_id)?;
        Ok(voltage > 0.8) // above 80% nominal = good
    }

    /// De-assert power and log shutdown via IPMI OEM command.
    /// 取消断言电源并通过 IPMI OEM 命令记录关机。
    fn disable_power_rail(&self) -> io::Result<()> {
        self.gpio().set_low()?;
        // Log OEM "power rail disabled" event to BMC
        self.ipmi().raw_command(0x2E, 0x01, &[0x00, 0x01])?;
        Ok(())
    }
}
```

### Step 3 — Blanket Impls Make It Truly "Mixin" / 第 3 步 —— Blanket 实现让它成为真正的“注入 (Mixin)”

The magic line — provide the ingredients, get the methods:

神奇的一行 —— 提供组件，获得方法：

```rust
impl<T: HasI2c + HasGpio>  FanDiagMixin    for T {}
impl<T: HasSpi  + HasIpmi>  TempMonitorMixin for T {}
impl<T: HasGpio + HasIpmi>  PowerSeqMixin   for T {}
```

Any struct that implements the right ingredient traits **automatically** gains every mixin method — no boilerplate, no forwarding, no inheritance.

任何实现了正确组件 trait 的结构体都会 **自动** 获得每个注入方法 —— 无样板代码、无转发、无继承。

### Step 4 — Wire Up Production / 第 4 步 —— 连接生产环境

```rust
// ── Concrete bus implementations (Linux platform) ────────────────
// ── 具体的总线实现（Linux 平台） ────────────────
struct LinuxSpi  { dev: String }
struct LinuxI2c  { dev: String }
struct SysfsGpio { pin: u32 }
struct IpmiTool  { timeout_secs: u32 }

impl SpiBus for LinuxSpi {
    fn spi_transfer(&self, _tx: &[u8], _rx: &mut [u8]) -> io::Result<()> {
        // spidev ioctl — omitted for brevity / 为了简洁起见省略
        Ok(())
    }
}
impl I2cBus for LinuxI2c {
    fn i2c_read(&self, _addr: u8, _reg: u8, _buf: &mut [u8]) -> io::Result<()> {
        // i2c-dev ioctl — omitted for brevity / 为了简洁起见省略
        Ok(())
    }
    fn i2c_write(&self, _addr: u8, _reg: u8, _data: &[u8]) -> io::Result<()> { Ok(()) }
}
impl GpioPin for SysfsGpio {
    fn set_high(&self) -> io::Result<()>  { /* /sys/class/gpio */ Ok(()) }
    fn set_low(&self) -> io::Result<()>   { Ok(()) }
    fn read_level(&self) -> io::Result<bool> { Ok(true) }
}
impl IpmiBmc for IpmiTool {
    fn raw_command(&self, _nf: u8, _cmd: u8, _data: &[u8]) -> io::Result<Vec<u8>> {
        // shells out to ipmitool — omitted for brevity / 为了简洁起见省略
        Ok(vec![])
    }
    fn read_sensor(&self, _id: u8) -> io::Result<f64> { Ok(25.0) }
}

// ── Production platform — all four buses ─────────────────────────
// ── 生产平台 —— 包含所有四个总线 ─────────────────────────
struct DiagPlatform {
    spi:  LinuxSpi,
    i2c:  LinuxI2c,
    gpio: SysfsGpio,
    ipmi: IpmiTool,
}

impl HasSpi  for DiagPlatform { type Spi  = LinuxSpi;  fn spi(&self)  -> &LinuxSpi  { &self.spi  } }
impl HasI2c  for DiagPlatform { type I2c  = LinuxI2c;  fn i2c(&self)  -> &LinuxI2c  { &self.i2c  } }
impl HasGpio for DiagPlatform { type Gpio = SysfsGpio; fn gpio(&self) -> &SysfsGpio { &self.gpio } }
impl HasIpmi for DiagPlatform { type Ipmi = IpmiTool;  fn ipmi(&self) -> &IpmiTool  { &self.ipmi } }

// DiagPlatform now has ALL mixin methods / DiagPlatform 现在拥有了所有注入方法：
fn production_diagnostics(platform: &DiagPlatform) -> io::Result<()> {
    let rpm = platform.read_fan_rpm(0)?;       // from FanDiagMixin / 来自 FanDiagMixin
    let tc  = platform.read_thermocouple()?;   // from TempMonitorMixin / 来自 TempMonitorMixin
    let ok  = platform.enable_power_rail(42)?;  // from PowerSeqMixin / 来自 PowerSeqMixin
    println!("Fan: {rpm} RPM, Temp: {tc}°C, Power: {ok}");
    Ok(())
}
```

### Step 5 — Test With Mocks (No Hardware Required) / 第 5 步 —— 使用 Mock 进行测试（无需硬件）

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;

    struct MockSpi  { temp: Cell<f64> }
    struct MockI2c  { rpm: Cell<u32> }
    struct MockGpio { level: Cell<bool> }
    struct MockIpmi { sensor_val: Cell<f64> }

    impl SpiBus for MockSpi {
        fn spi_transfer(&self, _tx: &[u8], rx: &mut [u8]) -> io::Result<()> {
            // Encode mock temp as MAX31855 format
            // 将模拟温度编码为 MAX31855 格式
            let raw = ((self.temp.get() / 0.25) as i32) << 18;
            rx.copy_from_slice(&raw.to_be_bytes());
            Ok(())
        }
    }
    impl I2cBus for MockI2c {
        fn i2c_read(&self, _addr: u8, _reg: u8, buf: &mut [u8]) -> io::Result<()> {
            let tach = (self.rpm.get() / 60) as u16;
            buf.copy_from_slice(&tach.to_be_bytes());
            Ok(())
        }
        fn i2c_write(&self, _: u8, _: u8, _: &[u8]) -> io::Result<()> { Ok(()) }
    }
    impl GpioPin for MockGpio {
        fn set_high(&self)  -> io::Result<()>   { self.level.set(true);  Ok(()) }
        fn set_low(&self)   -> io::Result<()>   { self.level.set(false); Ok(()) }
        fn read_level(&self) -> io::Result<bool> { Ok(self.level.get()) }
    }
    impl IpmiBmc for MockIpmi {
        fn raw_command(&self, _: u8, _: u8, _: &[u8]) -> io::Result<Vec<u8>> { Ok(vec![]) }
        fn read_sensor(&self, _: u8) -> io::Result<f64> { Ok(self.sensor_val.get()) }
    }

    // ── Partial platform: only fan-related buses ─────────────────
    // ── 部分平台：仅包含风扇相关的总线 ─────────────────
    struct FanTestRig {
        i2c:  MockI2c,
        gpio: MockGpio,
    }
    impl HasI2c  for FanTestRig { type I2c  = MockI2c;  fn i2c(&self)  -> &MockI2c  { &self.i2c  } }
    impl HasGpio for FanTestRig { type Gpio = MockGpio; fn gpio(&self) -> &MockGpio { &self.gpio } }
    // FanTestRig gets FanDiagMixin but NOT TempMonitorMixin or PowerSeqMixin
    // FanTestRig 获得了 FanDiagMixin，但没有获得 TempMonitorMixin 或 PowerSeqMixin

    #[test]
    fn fan_health_check_passes_above_threshold() {
        let rig = FanTestRig {
            i2c:  MockI2c  { rpm: Cell::new(6000) },
            gpio: MockGpio { level: Cell::new(false) },
        };
        assert!(rig.check_fan_health(0, 4000).unwrap());
    }

    #[test]
    fn fan_health_check_fails_below_threshold() {
        let rig = FanTestRig {
            i2c:  MockI2c  { rpm: Cell::new(2000) },
            gpio: MockGpio { level: Cell::new(false) },
        };
        assert!(!rig.check_fan_health(0, 4000).unwrap());
    }
}
```

Notice that `FanTestRig` only implements `HasI2c + HasGpio` — it gets `FanDiagMixin` automatically, but the compiler **refuses** `rig.read_thermocouple()` because `HasSpi` is not satisfied. This is mixin scoping enforced at compile time.

注意，`FanTestRig` 仅实现了 `HasI2c + HasGpio` —— 它自动获得了 `FanDiagMixin`，但编译器会 **拒绝** `rig.read_thermocouple()`，因为 `HasSpi` 未被满足。这是在编译时强制执行的注入作用域。

### Conditional Methods — Beyond What Ruby Can Do / 条件方法 —— 超越 Ruby 的能力

Add `where` bounds to individual default methods. The method only **exists** when the associated type satisfies the extra bound:

可以向单个默认方法添加 `where` 约束。方法仅在关联类型满足额外约束时才 **存在**：

```rust
/// Marker trait for DMA-capable SPI controllers / 支持 DMA 的 SPI 控制器的标记 trait
pub trait DmaCapable: SpiBus {
    fn dma_transfer(&self, tx: &[u8], rx: &mut [u8]) -> io::Result<()>;
}

/// Marker trait for interrupt-capable GPIO pins / 支持中断的 GPIO 引脚的标记 trait
pub trait InterruptCapable: GpioPin {
    fn wait_for_edge(&self, timeout_ms: u32) -> io::Result<bool>;
}

pub trait AdvancedDiagMixin: HasSpi + HasGpio {
    // Always available / 始终可用
    fn basic_probe(&self) -> io::Result<bool> {
        let mut rx = [0u8; 1];
        self.spi().spi_transfer(&[0xFF], &mut rx)?;
        Ok(rx[0] != 0x00)
    }

    // Only exists when the SPI controller supports DMA
    // 仅在 SPI 控制器支持 DMA 时存在
    fn bulk_sensor_read(&self, buf: &mut [u8]) -> io::Result<()>
    where
        Self::Spi: DmaCapable,
    {
        self.spi().dma_transfer(&vec![0x00; buf.len()], buf)
    }

    // Only exists when the GPIO pin supports interrupts
    // 仅在 GPIO 引脚支持中断时存在
    fn wait_for_fault_signal(&self, timeout_ms: u32) -> io::Result<bool>
    where
        Self::Gpio: InterruptCapable,
    {
        self.gpio().wait_for_edge(timeout_ms)
    }
}

impl<T: HasSpi + HasGpio> AdvancedDiagMixin for T {}
```

If your platform's SPI doesn't support DMA, calling `bulk_sensor_read()` is a **compile error**, not a runtime crash. Ruby's `respond_to?` check is the closest equivalent — but it happens at deploy time, not compile time.

如果你的平台的 SPI 不支持 DMA，调用 `bulk_sensor_read()` 将会导致 **编译错误**，而不是运行时崩溃。Ruby 的 `respond_to?` 检查是最接近的等效项 —— 但它发生在部署时，而不是编译时。

### Composability: Stacking Mixins / 组合性：堆叠注入

Multiple mixins can share the same ingredient — no diamond problem:

多个注入可以共享相同的组件 —— 没有菱形继承问题：

```text
┌─────────────┐    ┌───────────┐    ┌──────────────┐
│ FanDiagMixin│    │TempMonitor│    │ PowerSeqMixin│
│  (I2C+GPIO) │    │ (SPI+IPMI)│    │  (GPIO+IPMI) │
└──────┬──────┘    └─────┬─────┘    └──────┬───────┘
       │                 │                 │
       │   ┌─────────────┴─────────────┐   │
       └──►│      DiagPlatform         │◄──┘
           │ HasSpi+HasI2c+HasGpio     │
           │        +HasIpmi           │
           └───────────────────────────┘
```

`DiagPlatform` implements `HasGpio` **once**, and both `FanDiagMixin` and `PowerSeqMixin` use the same `self.gpio()`. In Ruby, this would be two modules both calling `self.gpio_pin` — but if they expected different pin numbers, you'd discover the conflict at runtime. In Rust, you can disambiguate at the type level.

`DiagPlatform` 只需实现 **一次** `HasGpio`，而 `FanDiagMixin` 和 `PowerSeqMixin` 都会使用同一个 `self.gpio()`。在 Ruby 中，这将是两个模块都调用 `self.gpio_pin` —— 但如果它们期望不同的引脚编号，你只会在运行时发现冲突。而在 Rust 中，你可以在类型级别消除歧义。

### Comparison: Ruby Mixins vs Rust Capability Mixins / 比较：Ruby Mixin vs Rust 能力注入

| Dimension / 维度 | Ruby Mixins | Rust Capability Mixins / 能力注入 |
|-----------|-------------|------------------------|
| Dispatch / 分发 | Runtime (method table lookup) / 运行时（方法表查找） | Compile-time (monomorphised) / 编译时（单态化） |
| Safe composition / 安全组合 | MRO linearisation hides conflicts / MRO 线性化隐藏了冲突 | Compiler rejects ambiguity / 编译器拒绝歧义 |
| Conditional methods / 条件方法 | `respond_to?` at runtime / 运行时 `respond_to?` | `where` bounds at compile time / 编译时 `where` 约束 |
| Overhead / 开销 | Method dispatch + GC / 方法分发 + GC | Zero-cost (inlined) / 零成本（内联） |
| Testability / 可测试性 | Stub/mock via metaprogramming / 通过元编程进行 Stub/mock | Generic over mock types / 对 Mock 类型泛型化 |
| Adding new buses / 添加新总线 | `include` at runtime / 运行时 `include` | Add ingredient trait, recompile / 添加组件 trait，重新编译 |
| Runtime flexibility / 运行时灵活性 | `extend`, `prepend`, open classes | None (fully static) / 无（完全静态） |

### When to Use Capability Mixins / 何时使用能力注入

| Scenario / 场景 | Use Mixins? / 是否使用注入？ |
|----------|:-----------:|
| Multiple diagnostics share bus-reading logic / 多个诊断程序共享总线读取逻辑 | ✅ |
| Test harness needs different bus subsets / 测试夹具需要不同的总线子集 | ✅ (partial ingredient structs / 部分组件结构体) |
| Methods only valid for certain bus capabilities (DMA, IRQ) / 仅对特定总线能力有效的方法 | ✅ (conditional `where` bounds / 条件 `where` 约束) |
| You need runtime module loading (plugins) / 需要运行时模块加载（插件） | ❌ (use `dyn Trait` or enum dispatch / 使用 `dyn Trait` 或枚举分发) |
| Single struct with one bus — no sharing needed / 只有一个总线的单个结构体 | ❌ (keep it simple / 保持简单) |
| Cross-crate ingredients with coherence issues / 存在一致性问题的跨 crate 组件 | ⚠️ (use newtype wrappers / 使用 newtype 包装) |

> **Key Takeaways — Capability Mixins / 核心要点 —— 能力注入**
>
> 1.  **Ingredient trait** = associated type + accessor method (e.g., `HasSpi`) / **组件 trait** = 关联类型 + 访问器方法（例如 `HasSpi`）
> 2.  **Mixin trait** = supertrait bounds on ingredients + default method bodies / **注入 trait** = 组件上的 supertrait 约束 + 默认方法体
> 3.  **Blanket impl** = `impl<T: HasX + HasY> Mixin for T {}` — auto-injects methods / **Blanket 实现** = `impl<T: HasX + HasY> Mixin for T {}` —— 自动注入方法
> 4.  **Conditional methods** = `where Self::Spi: DmaCapable` on individual defaults / **条件方法** = 在单个默认方法上使用 `where Self::Spi: DmaCapable`
> 5.  **Partial platforms** = test structs that only impl the needed ingredients / **部分平台** = 仅实现所需组件的测试结构体
> 6.  **No runtime cost** — the compiler generates specialised code for each platform type / **无运行时开销** —— 编译其会为每个平台类型生成专门的代码

***

## Typed Commands — GADT-Style Return Type Safety / 类型化命令 —— GADT 风格的返回类型安全性

In Haskell, **Generalised Algebraic Data Types (GADTs)** let each constructor of a data type refine the type parameter — so `Expr Int` and `Expr Bool` are enforced by the type checker. Rust has no direct GADT syntax, but **traits with associated types** achieve the same guarantee: the command type **determines** the response type, and mixing them up is a compile error.

在 Haskell 中，**广义代数数据类型 (GADTs)** 允许数据类型的每个构造函数细化类型参数 —— 这样 `Expr Int` 和 `Expr Bool` 就能由类型检查器强制执行。Rust 没有直接的 GADT 语法，但 **具有关联类型的 trait** 可以实现同样的保证：命令类型 **决定** 了响应类型，而混淆它们会导致编译错误。

This pattern is particularly powerful for hardware diagnostics, where IPMI commands, register reads, and sensor queries each return different physical quantities that should never be confused.

这种模式对于硬件诊断特别强大，在这种场景下，IPMI 命令、寄存器读取和传感器查询各自返回不同的物理量，且永远不应该被混淆。

### The Problem: The Untyped `Vec<u8>` Swamp / 问题：无类型的 `Vec<u8>` 沼泽

Most C/C++ IPMI stacks — and naïve Rust ports — use raw bytes everywhere:

大多数 C/C++ IPMI 栈 —— 以及幼稚的 Rust 移植版本 —— 到处都在使用原始字节：

```rust
use std::io;

struct BmcConnectionUntyped { timeout_secs: u32 }

impl BmcConnectionUntyped {
    fn raw_command(&self, net_fn: u8, cmd: u8, data: &[u8]) -> io::Result<Vec<u8>> {
        // ... shells out to ipmitool ...
        Ok(vec![0x00, 0x19, 0x00]) // stub
    }
}

fn diagnose_thermal_untyped(bmc: &BmcConnectionUntyped) -> io::Result<()> {
    // Read CPU temperature — sensor ID 0x20
    // 读取 CPU 温度 —— 传感器 ID 0x20
    let raw = bmc.raw_command(0x04, 0x2D, &[0x20])?;
    let cpu_temp = raw[0] as f64;  // 🤞 hope byte 0 is the reading / 🤞 希望第 0 个字节就是读数

    // Read fan speed — sensor ID 0x30
    // 读取风扇速度 —— 传感器 ID 0x30
    let raw = bmc.raw_command(0x04, 0x2D, &[0x30])?;
    let fan_rpm = raw[0] as u32;  // 🐛 BUG: fan speed is 2 bytes LE / 🐛 错误：风扇速度是 2 字节小端序

    // Read inlet voltage — sensor ID 0x40
    // 读取入口电压 —— 传感器 ID 0x40
    let raw = bmc.raw_command(0x04, 0x2D, &[0x40])?;
    let voltage = raw[0] as f64;  // 🐛 BUG: need to divide by 1000 / 🐛 错误：需要除以 1000

    // 🐛 Comparing °C to RPM — compiles, but nonsensical
    // 🐛 比较摄氏度与 RPM —— 虽然能编译通过，但毫无意义
    if cpu_temp > fan_rpm as f64 {
        println!("uh oh");
    }

    // 🐛 Passing Volts as temperature — compiles fine
    // 🐛 将电压作为温度传递 —— 编译正常
    log_temp_untyped(voltage);
    log_volts_untyped(cpu_temp);

    Ok(())
}

fn log_temp_untyped(t: f64)  { println!("Temp: {t}°C"); }
fn log_volts_untyped(v: f64) { println!("Voltage: {v}V"); }
```

**Every reading is `f64`** — the compiler has no idea that one is a temperature, another is RPM, another is voltage. Four distinct bugs compile without warning:

**每个读数都是 `f64`** —— 编译器不知道一个是温度、另一个是 RPM、还有一个是电压。四个不同的错误在没有任何警告的情况下通过了编译：

| # | Bug / 错误 | Consequence / 后果 | Discovered / 何时被发现 |
|---|-----|-------------|------------|
| 1 | Fan RPM parsed as 1 byte instead of 2 / 风扇 RPM 解析为 1 字节而非 2 字节 | Reads 25 RPM instead of 6400 / 读数为 25 RPM 而非 6400 | Production, 3 AM fan-failure flood / 生产环境凌晨三点的风扇故障报警潮 |
| 2 | Voltage not divided by 1000 / 电压没有除以 1000 | 12000V instead of 12.0V / 读数为 12000V 而非 12.0V | Threshold check flags every PSU / 阈值检查标记了每个电源 |
| 3 | Comparing °C to RPM / 比较摄氏度与 RPM | Meaningless boolean / 毫无意义的布尔值 | Possibly never / 可能永远不会发现 |
| 4 | Voltage passed to `log_temp_untyped()` / 电压被传递给 `log_temp_untyped()` | Silent data corruption in logs / 日志中无声的数据损坏 | 6 months later, reading history / 6 个月后查阅历史记录时 |

### The Solution: Typed Commands via Associated Types / 解决方案：通过关联类型实现类型化命令

#### Step 1 — Domain newtypes / 第 1 步 —— 领域 Newtype

```rust
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
struct Celsius(f64);

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
struct Rpm(u32);

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
struct Volts(f64);

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
struct Watts(f64);
```

#### Step 2 — The command trait (the GADT equivalent) / 第 2 步 —— 命令 Trait（GADT 的等效实现）

The associated type `Response` is the key — it binds each command to its return type:

关联类型 `Response` 是关键 —— 它将每个命令与其返回类型绑定在一起：

```rust
trait IpmiCmd {
    /// The GADT "index" — determines what execute() returns.
    /// GADT “索引” —— 决定了 execute() 的返回值。
    type Response;

    fn net_fn(&self) -> u8;
    fn cmd_byte(&self) -> u8;
    fn payload(&self) -> Vec<u8>;

    /// Parsing is encapsulated HERE — each command knows its own byte layout.
    /// 解析被封装在这里 —— 每个命令都知道自己的字节布局。
    fn parse_response(&self, raw: &[u8]) -> io::Result<Self::Response>;
}
```

#### Step 3 — One struct per command, parsing written once / 第 3 步 —— 每个命令一个结构体，解析代码只需写一次

```rust
struct ReadTemp { sensor_id: u8 }
impl IpmiCmd for ReadTemp {
    type Response = Celsius;  // ← "this command returns a temperature" / ← “该命令返回温度”
    fn net_fn(&self) -> u8 { 0x04 }
    fn cmd_byte(&self) -> u8 { 0x2D }
    fn payload(&self) -> Vec<u8> { vec![self.sensor_id] }
    fn parse_response(&self, raw: &[u8]) -> io::Result<Celsius> {
        // Signed byte per IPMI SDR — written once, tested once
        // 根据 IPMI SDR 定义的有符号字节 —— 编写一次，测试一次
        Ok(Celsius(raw[0] as i8 as f64))
    }
}

struct ReadFanSpeed { fan_id: u8 }
impl IpmiCmd for ReadFanSpeed {
    type Response = Rpm;     // ← "this command returns RPM" / ← “该命令返回 RPM”
    fn net_fn(&self) -> u8 { 0x04 }
    fn cmd_byte(&self) -> u8 { 0x2D }
    fn payload(&self) -> Vec<u8> { vec![self.fan_id] }
    fn parse_response(&self, raw: &[u8]) -> io::Result<Rpm> {
        // 2-byte LE — the correct layout, encoded once
        // 2 字节小端序 —— 正确的布局，编码一次即可
        Ok(Rpm(u16::from_le_bytes([raw[0], raw[1]]) as u32))
    }
}

struct ReadVoltage { rail: u8 }
impl IpmiCmd for ReadVoltage {
    type Response = Volts;   // ← "this command returns voltage" / ← “该命令返回电压”
    fn net_fn(&self) -> u8 { 0x04 }
    fn cmd_byte(&self) -> u8 { 0x2D }
    fn payload(&self) -> Vec<u8> { vec![self.rail] }
    fn parse_response(&self, raw: &[u8]) -> io::Result<Volts> {
        // Millivolts → Volts, always correct
        // 毫伏 → 伏特，始终正确
        Ok(Volts(u16::from_le_bytes([raw[0], raw[1]]) as f64 / 1000.0))
    }
}

struct ReadFru { fru_id: u8 }
impl IpmiCmd for ReadFru {
    type Response = String;
    fn net_fn(&self) -> u8 { 0x0A }
    fn cmd_byte(&self) -> u8 { 0x11 }
    fn payload(&self) -> Vec<u8> { vec![self.fru_id, 0x00, 0x00, 0xFF] }
    fn parse_response(&self, raw: &[u8]) -> io::Result<String> {
        Ok(String::from_utf8_lossy(raw).to_string())
    }
}
```

#### Step 4 — The executor (zero `dyn`, monomorphised) / 第 4 步 —— 执行器（零 `dyn`，单态化）

```rust
struct BmcConnection { timeout_secs: u32 }

impl BmcConnection {
    /// Generic over any command — compiler generates one version per command type.
    /// 对任何命令泛型化 —— 编译器为每种命令类型生成一个版本。
    fn execute<C: IpmiCmd>(&self, cmd: &C) -> io::Result<C::Response> {
        let raw = self.raw_send(cmd.net_fn(), cmd.cmd_byte(), &cmd.payload())?;
        cmd.parse_response(&raw)
    }

    fn raw_send(&self, _nf: u8, _cmd: u8, _data: &[u8]) -> io::Result<Vec<u8>> {
        Ok(vec![0x19, 0x00]) // stub — real impl calls ipmitool / 存根 —— 实际实现会调用 ipmitool
    }
}
```

#### Step 5 — Caller code: all four bugs become compile errors / 第 5 步 —— 调用者代码：所有四个错误都变成了编译错误

```rust
fn diagnose_thermal(bmc: &BmcConnection) -> io::Result<()> {
    let cpu_temp: Celsius = bmc.execute(&ReadTemp { sensor_id: 0x20 })?;
    let fan_rpm:  Rpm     = bmc.execute(&ReadFanSpeed { fan_id: 0x30 })?;
    let voltage:  Volts   = bmc.execute(&ReadVoltage { rail: 0x40 })?;

    // Bug #1 — IMPOSSIBLE: parsing lives in ReadFanSpeed::parse_response
    // 错误 #1 —— 不可能发生：解析逻辑在 ReadFanSpeed::parse_response 中
    // Bug #2 — IMPOSSIBLE: scaling lives in ReadVoltage::parse_response
    // 错误 #2 —— 不可能发生：缩放逻辑在 ReadVoltage::parse_response 中

    // Bug #3 — COMPILE ERROR / 错误 #3 —— 编译错误：
    // if cpu_temp > fan_rpm { }
    //    ^^^^^^^^   ^^^^^^^
    //    Celsius    Rpm      → "mismatched types" ❌

    // Bug #4 — COMPILE ERROR / 错误 #4 —— 编译错误：
    // log_temperature(voltage);
    //                 ^^^^^^^  Volts, expected Celsius ❌

    // Only correct comparisons compile / 只有正确的比较才能编译：
    if cpu_temp > Celsius(85.0) {
        println!("CPU overheating: {:?}", cpu_temp);
    }
    if fan_rpm < Rpm(4000) {
        println!("Fan too slow: {:?}", fan_rpm);
    }

    Ok(())
}

fn log_temperature(t: Celsius) { println!("Temp: {:?}", t); }
fn log_voltage(v: Volts)       { println!("Voltage: {:?}", v); }
```

### Macro DSL for Diagnostic Scripts / 诊断脚本的宏 DSL

For large diagnostic routines that run many commands in sequence, a macro gives concise declarative syntax while preserving full type safety:

对于需要按顺序运行许多命令的大型诊断程序，宏可以提供简明的声明式语法，同时保留完整的类型安全性：

```rust
/// Execute a series of typed IPMI commands, returning a tuple of results.
/// Each element of the tuple has the command's own Response type.
/// 执行一系列类型化的 IPMI 命令，返回一个结果元组。
/// 元组的每个元素都具有命令自己的 Response 类型。
macro_rules! diag_script {
    ($bmc:expr; $($cmd:expr),+ $(,)?) => {{
        ( $( $bmc.execute(&$cmd)?, )+ )
    }};
}

fn full_pre_flight(bmc: &BmcConnection) -> io::Result<()> {
    // Expands to: (Celsius, Rpm, Volts, String) — every type tracked
    // 展开为：(Celsius, Rpm, Volts, String) —— 每个类型都被追踪
    let (temp, rpm, volts, board_pn) = diag_script!(bmc;
        ReadTemp     { sensor_id: 0x20 },
        ReadFanSpeed { fan_id:    0x30 },
        ReadVoltage  { rail:      0x40 },
        ReadFru      { fru_id:    0x00 },
    );

    println!("Board: {:?}", board_pn);
    println!("CPU: {:?}, Fan: {:?}, 12V: {:?}", temp, rpm, volts);

    // Type-safe threshold checks / 类型安全的阈值检查：
    assert!(temp  < Celsius(95.0), "CPU too hot");
    assert!(rpm   > Rpm(3000),     "Fan too slow");
    assert!(volts > Volts(11.4),   "12V rail sagging");

    Ok(())
}
```

The macro is just syntactic sugar — the tuple type `(Celsius, Rpm, Volts, String)` is fully inferred by the compiler. Swap two commands and the destructuring breaks at compile time, not at runtime.

宏仅仅是语法糖 —— 元组类型 `(Celsius, Rpm, Volts, String)` 完全由编译器推导。交换两个命令，解构逻辑将在编译时报错，而不是在运行时。

### Enum Dispatch for Heterogeneous Command Lists / 异构命令列表的枚举分发

When you need a `Vec` of mixed commands (e.g., a configurable script loaded from JSON), use enum dispatch to stay `dyn`-free:

当你需要一个混合命令的 `Vec`（例如从 JSON 加载的可配置脚本）时，请使用枚举分发以保持无 `dyn`：

```rust
enum AnyReading {
    Temp(Celsius),
    Rpm(Rpm),
    Volt(Volts),
    Text(String),
}

enum AnyCmd {
    Temp(ReadTemp),
    Fan(ReadFanSpeed),
    Voltage(ReadVoltage),
    Fru(ReadFru),
}

impl AnyCmd {
    fn execute(&self, bmc: &BmcConnection) -> io::Result<AnyReading> {
        match self {
            AnyCmd::Temp(c)    => Ok(AnyReading::Temp(bmc.execute(c)?)),
            AnyCmd::Fan(c)     => Ok(AnyReading::Rpm(bmc.execute(c)?)),
            AnyCmd::Voltage(c) => Ok(AnyReading::Volt(bmc.execute(c)?)),
            AnyCmd::Fru(c)     => Ok(AnyReading::Text(bmc.execute(c)?)),
        }
    }
}

/// Dynamic diagnostic script — commands loaded at runtime
/// 动态诊断脚本 —— 在运行时加载命令
fn run_script(bmc: &BmcConnection, script: &[AnyCmd]) -> io::Result<Vec<AnyReading>> {
    script.iter().map(|cmd| cmd.execute(bmc)).collect()
}
```

You lose per-element type tracking (everything is `AnyReading`), but you gain runtime flexibility — and the parsing is still encapsulated in each `IpmiCmd` impl.

你失去了对每个元素的类型追踪（所有内容都是 `AnyReading`），但你获得了运行时的灵活性 —— 并且解析逻辑仍然封装在每个 `IpmiCmd` 实现中。

### Testing Typed Commands / 测试类型化命令

```rust
#[cfg(test)]
mod tests {
    use super::*;

    struct StubBmc {
        responses: std::collections::HashMap<u8, Vec<u8>>,
    }

    impl StubBmc {
        fn execute<C: IpmiCmd>(&self, cmd: &C) -> io::Result<C::Response> {
            let key = cmd.payload()[0]; // sensor ID as key / 传感器 ID 作为 key
            let raw = self.responses.get(&key)
                .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "no stub"))?;
            cmd.parse_response(raw)
        }
    }

    #[test]
    fn read_temp_parses_signed_byte() {
        let bmc = StubBmc {
            responses: [( 0x20, vec![0xE7] )].into() // -25 as i8 = 0xE7
        };
        let temp = bmc.execute(&ReadTemp { sensor_id: 0x20 }).unwrap();
        assert_eq!(temp, Celsius(-25.0));
    }

    #[test]
    fn read_fan_parses_two_byte_le() {
        let bmc = StubBmc {
            responses: [( 0x30, vec![0x00, 0x19] )].into() // 0x1900 = 6400
        };
        let rpm = bmc.execute(&ReadFanSpeed { fan_id: 0x30 }).unwrap();
        assert_eq!(rpm, Rpm(6400));
    }

    #[test]
    fn read_voltage_scales_millivolts() {
        let bmc = StubBmc {
            responses: [( 0x40, vec![0xE8, 0x2E] )].into() // 0x2EE8 = 12008 mV
        };
        let v = bmc.execute(&ReadVoltage { rail: 0x40 }).unwrap();
        assert!((v.0 - 12.008).abs() < 0.001);
    }
}
```

Each command's parsing is tested independently. If `ReadFanSpeed` changes from 2-byte LE to 4-byte BE in a new IPMI spec revision, you update **one** `parse_response` and the test catches regressions.

每个命令的解析都是独立测试的。如果在新的 IPMI 规范修订版中，`ReadFanSpeed` 从 2 字节小端序变为 4 字节大端序，你只需更新 **一个** `parse_response`，测试即可捕获回归。

### How This Maps to Haskell GADTs / 这与 Haskell GADT 的映射关系

| Haskell GADT | Rust Equivalent / Rust 等效实现 |
| ──────────────── | ─────────────────────── |
| `data Cmd a where` | `trait IpmiCmd { type Response; ... }` |
| `ReadTemp :: SensorId -> Cmd Temp` | `struct ReadTemp { .. }` |
| `ReadFan :: FanId -> Cmd Rpm` | `struct ReadFanSpeed { .. }` |
| `eval :: Cmd a -> IO a` | `fn execute<C: IpmiCmd>(&self, cmd: &C) -> io::Result<C::Response>` |
| Type refinement in case branches / case 分支中的类型细化 | Monomorphisation: compiler generates / 单态化：编译器生成具体版本 |

Both guarantee: **the command determines the return type**. Rust achieves it through generic monomorphisation instead of type-level case analysis — same safety, zero runtime cost.

两者都保证：**命令决定了返回类型**。Rust 通过泛型单态化而不是类型级的 case 分析来实现这一点 —— 安全性相同，且零运行时成本。

### Before vs After Summary / 修改前 vs 修改后总结

| Dimension / 维度 | Untyped (`Vec<u8>`) / 无类型 (`Vec<u8>`) | Typed Commands / 类型化命令 |
|-----------|:---:|:---:|
| Lines per sensor / 每个传感器的代码行数 | ~3 (duplicated at every call site / 每个调用处重复) | ~15 (written and tested once / 编写和测试一次) |
| Parsing errors possible / 可能出现解析错误 | At every call site / 在每个调用处 | In one `parse_response` impl / 在一个 `parse_response` 实现中 |
| Unit confusion bugs / 单位混淆错误 | Unlimited / 无限制 | Zero (compile error) / 零（编译错误） |
| Adding a new sensor / 添加新传感器 | Touch N files, copy-paste parsing / 修改 N 个文件，复制粘贴解析逻辑 | Add 1 struct + 1 impl / 添加 1 个结构体 + 1 个实现 |
| Runtime cost / 运行时开销 | — | Identical (monomorphised) / 相同（单态化） |
| IDE autocomplete / IDE 自动补全 | `f64` everywhere | `Celsius`, `Rpm`, `Volts` — self-documenting |
| Code review burden / 代码审查负担 | Must verify every raw byte parse / 必须核实每一个原始字节解析 | Verify one `parse_response` per sensor / 每个传感器核实一次 |
| Macro DSL | N/A | `diag_script!(bmc; ReadTemp{..}, ReadFan{..})` → `(Celsius, Rpm)` |
| Dynamic scripts / 动态脚本 | Manual dispatch / 手动分发 | `AnyCmd` enum — still `dyn`-free / 仍然无 `dyn` |

### When to Use Typed Commands / 何时使用类型化命令

| Scenario / 场景 | Recommendation / 建议 |
|----------|:--------------:|
| IPMI sensor reads with distinct physical units / 具有不同物理单位的 IPMI 传感器读取 | ✅ Typed commands |
| Register map with different-width fields / 具有不同宽度字段的寄存器映射 | ✅ Typed commands |
| Network protocol messages (request → response) / 网络协议消息（请求 → 响应） | ✅ Typed commands |
| Single command type with one return format / 只有一种返回格式的单个命令类型 | ❌ Overkill — just return the type directly / 过度设计 —— 直接返回类型即可 |
| Prototyping / exploring an unknown device / 原型设计 / 探索未知设备 | ❌ Raw bytes first, type later / 先使用原始字节，稍后再类型化 |
| Plugin system where commands aren't known at compile time / 编译时未知的插件系统 | ⚠️ Use `AnyCmd` enum dispatch / 使用 `AnyCmd` 枚举分发 |

> **Key Takeaways — Traits / 核心要点 —— Trait**
> - Associated types = one impl per type; generic parameters = many impls per type / 关联类型 = 每个类型一个实现；泛型参数 = 每个类型多个实现
> - GATs unlock lending iterators and async-in-traits patterns / GAT 开启了 lending iterator 和 async-in-traits 模式
> - Use enum dispatch for closed sets (fast); `dyn Trait` for open sets (flexible) / 对封闭集使用枚举分发（快）；对开放集使用 `dyn Trait`（灵活）
> - `Any` + `TypeId` is the escape hatch when compile-time types are unknown / 当编译时类型未知时，`Any` + `TypeId` 是逃生舱
>
> **See also / 另请参阅：** [Ch 1 — Generics](ch01-generics-the-full-picture.md) for monomorphization and when generics cause code bloat. [Ch 3 — Newtype & Type-State](ch03-the-newtype-and-type-state-patterns.md) for using traits with the config trait pattern.
>
> 查看 [Ch 1 —— 泛型](ch01-generics-the-full-picture.md) 了解单态化以及泛型何时会导致代码膨胀。查看 [Ch 3 —— Newtype 与类型状态模式](ch03-the-newtype-and-type-state-patterns.md) 了解如何将 trait 与 config trait 模式结合使用。

---

### Exercise: Repository with Associated Types ★★★ (~40 min) / 练习：具有关联类型的 Respository ★★★（约 40 分钟）

Design a `Repository` trait with associated `Error`, `Id`, and `Item` types. Implement it for an in-memory store and demonstrate compile-time type safety.

设计一个具有关联类型 `Error`、`Id` 和 `Item` 的 `Repository` trait。为一个内存存储（in-memory store）实现它，并演示编译时类型安全性。

<details>
<summary>🔑 Solution / 🔑 解决方案</summary>

```rust
use std::collections::HashMap;

trait Repository {
    type Item;
    type Id;
    type Error;

    fn get(&self, id: &Self::Id) -> Result<Option<&Self::Item>, Self::Error>;
    fn insert(&mut self, item: Self::Item) -> Result<Self::Id, Self::Error>;
    fn delete(&mut self, id: &Self::Id) -> Result<bool, Self::Error>;
}

#[derive(Debug, Clone)]
struct User {
    name: String,
    email: String,
}

struct InMemoryUserRepo {
    data: HashMap<u64, User>,
    next_id: u64,
}

impl InMemoryUserRepo {
    fn new() -> Self {
        InMemoryUserRepo { data: HashMap::new(), next_id: 1 }
    }
}

impl Repository for InMemoryUserRepo {
    type Item = User;
    type Id = u64;
    type Error = std::convert::Infallible;

    fn get(&self, id: &u64) -> Result<Option<&User>, Self::Error> {
        Ok(self.data.get(id))
    }

    fn insert(&mut self, item: User) -> Result<u64, Self::Error> {
        let id = self.next_id;
        self.next_id += 1;
        self.data.insert(id, item);
        Ok(id)
    }

    fn delete(&mut self, id: &u64) -> Result<bool, Self::Error> {
        Ok(self.data.remove(id).is_some())
    }
}

fn create_and_fetch<R: Repository>(repo: &mut R, item: R::Item) -> Result<(), R::Error>
where
    R::Item: std::fmt::Debug,
    R::Id: std::fmt::Debug,
{
    let id = repo.insert(item)?;
    println!("Inserted with id: {id:?}");
    let retrieved = repo.get(&id)?;
    println!("Retrieved: {retrieved:?}");
    Ok(())
}

fn main() {
    let mut repo = InMemoryUserRepo::new();
    create_and_fetch(&mut repo, User {
        name: "Alice".into(),
        email: "alice@example.com".into(),
    }).unwrap();
}
```

</details>

***
