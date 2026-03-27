# 宏：生成代码的代码

> **你将学到什么：** 为什么 Rust 需要宏（没有重载、没有可变参数），`macro_rules!` 的基础写法，`!` 后缀约定，以及常见 derive 宏。
>
> **难度：** 中级

C# 没有与 Rust 宏完全对应的机制。理解宏为什么存在、它到底做了什么，可以消除 C# 开发者在学习 Rust 时的一大困惑来源。

---

## 为什么需要宏？
在 C# 中，你拥有一些使得宏显得并不那么必要的特性，例如：
1.  **方法重载 (Method Overloading)**：可以同时定义 `void Print(int x)` 和 `void Print(string s)`。
2.  **可变参数 (Variadic Arguments)**：使用 `void Print(params object[] args)`。

**而 Rust 两者都没有。** 宏填补了这些空白，使得代码能在编译期处理可变数量或不同类型的参数。

---

## `!` 后缀约定
以 `!` 结尾的调用通常都是宏调用，而非普通的函数调用。
*   **`println!("你好")`**：在编译期处理格式化。
*   **`vec![1, 2, 3]`**：会展开为创建并填充一个 `Vec` 的代码。
*   **`panic!("发生错误")`**：停止执行并输出消息。

---

## 声明式宏 (`macro_rules!`)
这些宏使用模式匹配（Pattern Matching）来转换代码。它就像是面向你源代码的 `match` 语句。

```rust
macro_rules! say_hello {
    () => {
        println!("你好!");
    };
    ($name:expr) => {
        println!("你好, {}!", $name);
    };
}

say_hello!();        // 输出 "你好!"
say_hello!("Alice"); // 输出 "你好, Alice!"
```

---

## Derive 宏：自动实现 Trait
由于这是你最常接触到的宏。它有些类似于 C# 的 `record` 特性，但比它更灵活。

```rust
#[derive(Debug, Clone, PartialEq)]
struct User {
    name: String,
    age: u32,
}
```
编译器会自动为你生成 `Debug`（用于打印）、`Clone`（用于拷贝）和 `PartialEq`（用于比较）的代码。

---

## `dbg!()`：快速调试时的好帮手
在调试时，别再只用 `Console.WriteLine` 了，试试 `dbg!` 吧。它不仅能打印出文件名和行号，还能打印出表达式的内容及其值，最后还会返回该值。

```rust
let x = 5;
let y = dbg!(x * 2) + 1; // 打印 [src/main.rs:2] x * 2 = 10
```

---

## C# 开发者总结表
| **概念** | **C# 对应物** | **Rust 现实** |
| :--- | :--- | :--- |
| **重载** | 多个同名方法 | 宏或 Trait |
| **可变参数** | `params` 关键字 | 宏 (例如 `vec![]`) |
| **样板代码** | 手动编写实现 | `#[derive(...)]` 自动生成 |
| **代码生成** | C# 源代码生成器 | 过程宏 (Procedural Macros) |

---

## 练习：使用宏
**挑战：** 使用 `vec!` 宏创建一个包含若干整数的动态数组，并使用 `dbg!` 宏打印出它的长度。

```rust
let v = vec![1, 2, 3, 4, 5];
dbg!(v.len());
```
**关键理解：** 宏是减少样板代码、实现编译期格式化等普通函数无法完成功能的强大工具。
