# 变量与可变性

> **你将学到什么：** Rust 的变量声明与可变性模型与 C# 的 `var`/`const` 有什么不同，常见基础类型如何映射，为什么 `String` 与 `&str` 的区别至关重要，以及 Rust 在类型推断、类型转换和强制转换上的思路为何不同于 C#。
>
> **难度：** 🟢 初级

变量是 Rust 中最容易让 C# 开发者“看起来熟悉、实际上差异很大”的地方之一。

---

## 变量声明

### C# 变量声明
```csharp
// C# - 变量默认是可变的
int count = 0;           // 可变
count = 5;               // ✅ 正常运行

const int BUFFER_SIZE = 1024; // 编译时常量
```

### Rust 变量声明
```rust
// Rust - 变量默认是不可变的
let count = 0;           // 默认不可变
// count = 5;            // ❌ 编译错误

let mut count = 0;       // 显式可变
count = 5;               // ✅ 正常运行

const BUFFER_SIZE: usize = 1024; // 编译时常量
```

### 关键思想转变
可以把 `let` 理解为将 C# 的 `readonly` 语义应用到所有局部变量。
- **变量遮蔽 (Shadowing)**: Rust 允许你用相同的名称声明一个新变量，从而有效地“复用”该名称来代表不同的类型或值。

---

## 数据类型对比

### 基础类型

| **C# 类型** | **Rust 类型** | **大小** | **备注** |
| :--- | :--- | :--- | :--- |
| `byte` | `u8` | 8 位 | 无符号 |
| `int` | `i32` | 32 位 | 默认整数 |
| `long` | `i64` | 64 位 | |
| `float` | `f32` | 32 位 | IEEE 754 |
| `double` | `f64` | 64 位 | |
| `char` | `char` | 32 位 | Unicode 标量 |

### 大小相关的类型 (isize/usize)
在 Rust 中，`usize` 和 `isize` 的大小取决于具体的硬件架构（32 位或 64 位）。它们主要用于集合索引，类似于 C++ 中的 `size_t`。

---

## 字符串类型：String 与 &str

这是 C# 开发者的一个核心难点。

*   **`&str` (字符串切片)**: 对字符串缓冲区的不可变引用。类似于 C# 中的 `ReadOnlySpan<char>`。字符串字面量总是 `&str`。
*   **`String`**: 拥有的、堆分配的、可增长的字符串。类似于 `StringBuilder` 或者是你可以随意修改的常规 `string`。

```rust
let literal: &str = "Hello";           // 借用 (Borrowed)
let mut owned: String = literal.to_string(); // 拥有 (Owned)
owned.push_str(" World");
```

---

## 输出与格式化

Rust 使用宏（以 `!` 结尾）进行输出。

*   `println!("{name} is {age}")`: 直接捕获变量 (Rust 1.58+)。
*   `println!("{:?}", object)`: 调试输出 (需要 `#[derive(Debug)]`)。
*   `println!("{}", object)`: 展示输出 (需要手动实现 `Display` trait)。

---

## 类型转换

Rust **没有隐式的数值转换**。你必须始终保持显式。

```rust
let x: i32 = 42;
let y: f64 = x as f64; // 显式转换

// 安全转换
let result = u8::try_from(x); // 返回 Result
```
