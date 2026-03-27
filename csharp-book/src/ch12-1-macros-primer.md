## Macros: Code That Writes Code | 宏：生成代码的代码

> **What you'll learn:** Why Rust needs macros (no overloading, no variadic args), `macro_rules!` basics,
> the `!` suffix convention, common derive macros, and `dbg!()` for quick debugging.
>
> **你将学到什么：** 为什么 Rust 需要宏（没有重载、没有可变参数），`macro_rules!` 的基础写法，
> `!` 后缀约定、常见 derive 宏，以及如何用 `dbg!()` 快速调试。
>
> **Difficulty:** Intermediate
>
> **难度：** 中级

C# has no direct equivalent to Rust macros. Understanding why they exist and how they work removes a major source of confusion for C# developers.

C# 没有与 Rust 宏完全对应的机制。理解宏为什么存在、它到底做了什么，可以消除 C# 开发者在学习 Rust 时的一大困惑来源。

### Why Macros Exist in Rust | Rust 为什么需要宏

```mermaid
graph LR
    SRC["vec![1, 2, 3]"] -->|"compile time"| EXP["{
  let mut v = Vec::new();
  v.push(1);
  v.push(2);
  v.push(3);
  v
}"]
    EXP -->|"compiles to"| BIN["machine code"]

    style SRC fill:#fff9c4,color:#000
    style EXP fill:#c8e6c9,color:#000
```

```csharp
// C# has features that make macros unnecessary:
Console.WriteLine("Hello");           // Method overloading (1-16 params)
Console.WriteLine("{0}, {1}", a, b);  // Variadic via params array
var list = new List<int> { 1, 2, 3 }; // Collection initializer syntax
```

```rust
// Rust has NO function overloading, NO variadic arguments, NO special syntax.
// Macros fill these gaps:
println!("Hello");                    // Macro - handles 0+ args at compile time
println!("{}, {}", a, b);             // Macro - type-checked at compile time
let list = vec![1, 2, 3];            // Macro - expands to Vec::new() + push()
```

### Recognizing Macros: The `!` Suffix | 如何识别宏：`!` 后缀

Every macro invocation ends with `!`. If you see `!`, it's a macro, not a function:

每一次宏调用后面都会带 `!`。只要你看到 `!`，基本就可以判断它是宏，而不是普通函数：

```rust
println!("hello");     // macro - generates format string code at compile time
format!("{x}");        // macro - returns String, compile-time format checking
vec![1, 2, 3];         // macro - creates and populates a Vec
todo!();               // macro - panics with "not yet implemented"
dbg!(expression);      // macro - prints file:line + expression + value, returns value
assert_eq!(a, b);      // macro - panics with diff if a != b
cfg!(target_os = "linux"); // macro - compile-time platform detection
```

### Writing a Simple Macro with `macro_rules!` | 用 `macro_rules!` 写一个简单宏
```rust
// Define a macro that creates a HashMap from key-value pairs
macro_rules! hashmap {
    // Pattern: key => value pairs separated by commas
    ( $( $key:expr => $value:expr ),* $(,)? ) => {{
        let mut map = std::collections::HashMap::new();
        $( map.insert($key, $value); )*
        map
    }};
}

fn main() {
    let scores = hashmap! {
        "Alice" => 100,
        "Bob"   => 85,
        "Carol" => 92,
    };
    println!("{scores:?}");
}
```

### Derive Macros: Auto-Implementing Traits | Derive 宏：自动实现 Trait
```rust
// #[derive] is a procedural macro that generates trait implementations
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct User {
    name: String,
    age: u32,
}
// The compiler generates Debug::fmt, Clone::clone, PartialEq::eq, etc.
// automatically by examining the struct fields.
```

```csharp
// C# equivalent: none - you'd manually implement IEquatable, ICloneable, etc.
// Or use records: public record User(string Name, int Age);
// Records auto-generate Equals, GetHashCode, ToString - similar idea!
```

### Common Derive Macros | 常见 Derive 宏

| Derive | Purpose | C# Equivalent |
|--------|---------|---------------|
| `Debug` | `{:?}` format string output | `ToString()` override |
| `Debug` | 用于 `{:?}` 调试输出 | 重写 `ToString()` |
| `Clone` | Deep copy via `.clone()` | `ICloneable` |
| `Clone` | 通过 `.clone()` 进行深拷贝 | `ICloneable` |
| `Copy` | Implicit bitwise copy (no `.clone()` needed) | Value type (`struct`) semantics |
| `Copy` | 隐式按位复制（不需要 `.clone()`） | 值类型（`struct`）语义 |
| `PartialEq`, `Eq` | `==` comparison | `IEquatable<T>` |
| `PartialEq`, `Eq` | 用于 `==` 比较 | `IEquatable<T>` |
| `PartialOrd`, `Ord` | `<`, `>` comparison + sorting | `IComparable<T>` |
| `PartialOrd`, `Ord` | 支持 `<`、`>` 比较和排序 | `IComparable<T>` |
| `Hash` | Hashing for `HashMap` keys | `GetHashCode()` |
| `Hash` | 用于作为 `HashMap` 键的哈希实现 | `GetHashCode()` |
| `Default` | Default values via `Default::default()` | Parameterless constructor |
| `Default` | 通过 `Default::default()` 提供默认值 | 无参构造 |
| `Serialize`, `Deserialize` | JSON/TOML/etc. (serde) | `[JsonProperty]` attributes |
| `Serialize`, `Deserialize` | JSON/TOML 等序列化（serde） | `[JsonProperty]` 这类属性 |

> **Rule of thumb:** Start with `#[derive(Debug)]` on every type. Add `Clone`, `PartialEq` when needed. Add `Serialize, Deserialize` for any type that crosses a boundary (API, file, database).
>
> **经验法则：** 几乎每个类型一开始都可以先加上 `#[derive(Debug)]`。需要复制时再加 `Clone`，需要比较时加 `PartialEq`。任何跨边界传输的类型（API、文件、数据库）通常都值得加上 `Serialize, Deserialize`。

### Procedural & Attribute Macros (Awareness Level) | 过程宏与属性宏（了解层面）

Derive macros are one kind of **procedural macro** - code that runs at compile time to generate code. You'll encounter two other forms:

Derive 宏只是**过程宏**的一种。过程宏本质上是在编译期运行、用来生成代码的代码。你还会遇到另外两种形式：

**Attribute macros** - attached to items with `#[...]`:

**属性宏**：通过 `#[...]` 附着在条目上：

```rust
#[tokio::main]          // turns main() into an async runtime entry point
async fn main() { }

#[test]                 // marks a function as a unit test
fn it_works() { assert_eq!(2 + 2, 4); }

#[cfg(test)]            // conditionally compile this module only during testing
mod tests { /* ... */ }
```

**Function-like macros** - look like function calls:

**函数式宏**：外观看起来像函数调用：

```rust
// sqlx::query! verifies your SQL against the database at compile time
let users = sqlx::query!("SELECT id, name FROM users WHERE active = $1", true)
    .fetch_all(&pool)
    .await?;
```

> **Key insight for C# developers:** You rarely *write* procedural macros - they're an advanced library-author tool. But you *use* them constantly (`#[derive(...)]`, `#[tokio::main]`, `#[test]`). Think of them like C# source generators: you benefit from them without implementing them.
>
> **给 C# 开发者的关键理解：** 你通常不会亲自去*写*过程宏，那是偏底层、偏库作者的工作；但你会频繁地*使用*它们，比如 `#[derive(...)]`、`#[tokio::main]`、`#[test]`。可以把它们理解为类似 C# source generator 的东西：你大量受益，但很少自己实现。

### Conditional Compilation with `#[cfg]` | 用 `#[cfg]` 做条件编译

Rust's `#[cfg]` attributes are like C#'s `#if DEBUG` preprocessor directives, but type-checked:

Rust 的 `#[cfg]` 属性有点像 C# 的 `#if DEBUG` 预处理指令，但它和类型系统、编译器规则是协同工作的：

```rust
// Compile this function only on Linux
#[cfg(target_os = "linux")]
fn platform_specific() {
    println!("Running on Linux");
}

// Debug-only assertions (like C# Debug.Assert)
#[cfg(debug_assertions)]
fn expensive_check(data: &[u8]) {
    assert!(data.len() < 1_000_000, "data unexpectedly large");
}

// Feature flags (like C# #if FEATURE_X, but declared in Cargo.toml)
#[cfg(feature = "json")]
pub fn to_json<T: Serialize>(val: &T) -> String {
    serde_json::to_string(val).unwrap()
}
```

```csharp
// C# equivalent
#if DEBUG
    Debug.Assert(data.Length < 1_000_000);
#endif
```

### `dbg!()` - Your Best Friend for Debugging | `dbg!()`：调试时的高效助手
```rust
fn calculate(x: i32) -> i32 {
    let intermediate = dbg!(x * 2);     // prints: [src/main.rs:3] x * 2 = 10
    let result = dbg!(intermediate + 1); // prints: [src/main.rs:4] intermediate + 1 = 11
    result
}
// dbg! prints to stderr, includes file:line, and returns the value
// Far more useful than Console.WriteLine for debugging!
```

<details>
<summary><strong>Exercise: Write a min! Macro | 练习：实现一个 `min!` 宏</strong> (click to expand / 点击展开)</summary>

**Challenge**: Write a `min!` macro that accepts 2 or more arguments and returns the smallest.

**挑战：** 编写一个 `min!` 宏，接收 2 个或更多参数，并返回其中最小的值。

```rust
// Should work like:
let smallest = min!(5, 3, 8, 1, 4); // -> 1
let pair = min!(10, 20);             // -> 10
```

<details>
<summary>Solution | 参考答案</summary>

```rust
macro_rules! min {
    // Base case: single value
    ($x:expr) => ($x);
    // Recursive: compare first with min of rest
    ($x:expr, $($rest:expr),+) => {{
        let first = $x;
        let rest = min!($($rest),+);
        if first < rest { first } else { rest }
    }};
}

fn main() {
    assert_eq!(min!(5, 3, 8, 1, 4), 1);
    assert_eq!(min!(10, 20), 10);
    assert_eq!(min!(42), 42);
    println!("All assertions passed!");
}
```

**Key takeaway**: `macro_rules!` uses pattern matching on token trees - it's like `match` but for code structure instead of values.

**关键理解：** `macro_rules!` 本质上是在对 token tree 做模式匹配。你可以把它理解成“不是匹配值，而是匹配代码结构”的 `match`。

</details>
</details>

***
