## Rust Macros: From Preprocessor to Metaprogramming / Rust 宏：从预处理器到元编程

> **What you'll learn / 你将学到：** How Rust macros work, when to use them instead of functions or generics, and how they replace the C/C++ preprocessor. By the end of this chapter you can write your own `macro_rules!` macros and understand what `#[derive(Debug)]` does under the hood.
>
> Rust 宏的工作原理、何时使用它们而非函数或泛型，以及它们如何替代 C/C++ 预处理器。在本章结束时，你将能编写自己的 `macro_rules!` 宏，并理解 `#[derive(Debug)]` 底层的运作机制。

*Macros are one of the first things you encounter in Rust (`println!("hello")` on line one) but one of the last things most courses explain. This chapter fixes that.*

宏是你最早接触到的 Rust 特性之一（第一行代码通常就是 `println!("hello")`），但却是大多数课程最后才解释的内容。本章将填补这一空白。

---

### Why Macros Exist / 为什么需要宏

*Functions and generics handle most code reuse in Rust. Macros fill the gaps where the type system can't reach:*

在 Rust 中，函数和泛型处理了大部分代码复用。宏则填补了类型系统无法触及的空白：

| **Need / 需求** | **Function/Generic? / 函数/泛型？** | **Macro? / 宏？** | **Why / 原因** |
|------|-------------------|--------|-----|
| Compute value / 计算值 | ✅ `fn max<T: Ord>(...)` | — | 类型系统足以处理 |
| Variadic args / 变长参数 | ❌ Rust 无变长参数函数 | ✅ `println!` | 宏可以接收任意数量的 Token |
| Repetitive `impl` blocks | ❌ 仅靠泛型无法实现 | ✅ `macro_rules!` | 宏在编译时生成代码 |
| Compile-time code execution | ❌ `const fn` 限制较多 | ✅ Procedural macros | 完整的 Rust 代码可在编译时运行 |

---

## Declarative Macros with `macro_rules!` / 使用 `macro_rules!` 的声明式宏

*Declarative macros (also called "macros by example") use pattern matching on syntax, similar to `match` on values.*

声明式宏（也称为“示例宏”）在语法上使用模式匹配，类似于对值进行的 `match` 操作。

### Basic syntax / 基础语法

```rust
macro_rules! say_hello {
    () => {
        println!("Hello!");
    };
}

fn main() {
    say_hello!();  // Expands / 展开为：println!("Hello!");
}
```

### Fragment specifiers / 片段指示符参考表

| **Specifier / 指示符** | **Matches / 匹配项** | **Example / 示例** |
|-----------|---------|---------|
| `$x:expr` | 任意表达式 | `42`, `a + b`, `foo()` |
| `$x:ty` | 类型 | `i32`, `Vec<String>`, `&str` |
| `$x:ident` | 标识符 | `foo`, `my_var`, `MyStruct` |
| `$x:path` | 路径 | `std::collections::HashMap` |
| `$x:tt` | 单个 Token 树 | 任意内容 —— 通配符 |

---

### Repetition / 重复 —— 宏的核心杀手锏

*C/C++ macros can't loop. Rust macros can repeat patterns:*

C/C++ 宏无法循环。Rust 宏则可以重复模式：

```rust
macro_rules! make_vec {
    ( $( $element:expr ),* ) => {
        {
            let mut v = Vec::new();
            $( v.push($element); )*  // Repeat for each matched element
            v
        }
    };
}
```

| **Operator / 操作符** | **Meaning / 含义** | **Example / 示例** |
|----------|---------|---------|
| `$( ... )*` | 零个或多个 | `vec![]`, `vec![1, 2]` |
| `$( ... )+` | 一个或多个 | 至少需要一个元素 |
| `$( ... )?` | 零个或一个 | 可选元素 |

---

## Common Standard Library Macros / 常用标准库宏

| **Macro / 宏** | **What it does / 作用** | **Expands to / 展开为** |
|-------|-------------|------------------------|
| `println!` | Format and print to stdout | `std::io::_print(...)` |
| `format!` | Format into a `String` | Allocates and returns a `String` |
| `vec!` | Create a `Vec` with elements | `Vec::from(...)` |
| `todo!` | Mark unfinished code | `panic!("not yet implemented")` |
| `assert!` | Panic if condition is false | `if !cond { panic!(...) }` |
| `dbg!` | Print expr + value, return value | Debug-only inspection |

### `dbg!` — The debugging champion / `dbg!` —— 调试王者

*`dbg!` returns the value it wraps, so you can insert it anywhere without changing program behavior. It prints to stderr with file:line info.*

`dbg!` 会返回它包裹的值，且带有文件名和行号。它打印到 stderr，因此不会干扰 stdout。

---

## Derive Macros / 派生宏

*`#[derive(Debug)]` is a **derive macro** — it generates trait implementations automatically.*

`#[derive(Debug)]` 是一个**派生宏** —— 它会自动为结构体或枚举生成 Trait 实现代码。

| **Derive / 派生** | **What it generates / 生成内容** | **When to use / 场景** |
|--------|-------------------|-------------|
| `Debug` | `{:?}` 格式化 | 几乎所有场景 —— 启用调试打印 |
| `Clone` | `.clone()` 方法 | 需要复制值时 |
| `PartialEq` | `==` 和 `!=` 运算符 | 需要相等性比较时 |
| `Default` | `Type::default()` | 具有合理的“零值”或空值时 |

---

## Attribute Macros / 属性宏

*Attribute macros transform the item they're attached to.*

属性宏会转换它们所附加的项。它们是语言语法的一部分，替代了 `#pragma` 和编译器扩展。

| **Attribute / 属性** | **Purpose / 用途** |
|-----------|---------|
| `#[test]` | 标记为测试函数 |
| `#[cfg(...)]` | 条件编译（替代 `#ifdef`） |
| `#[must_use]` | 警告未使用的返回值 |
| `#[repr(C)]` | C 兼容的内存布局 (用于 FFI) |

---

# Exercises / 练习

### 🟢 Exercise 1: `min!` macro / 练习 1：`min!` 宏
Write a `min!` macro that returns the smaller of two or three values.
```rust
macro_rules! min {
    ($a:expr, $b:expr) => { if $a < $b { $a } else { $b } };
    ($a:expr, $b:expr, $c:expr) => { min!(min!($a, $b), $c) };
}
```

### 🟡 Exercise 2: `hashmap!` / 练习 2：`hashmap!`
Create a `hashmap!` macro that supports `key => value` syntax and trailing commas.
```rust
macro_rules! hashmap {
    ( $( $key:expr => $value:expr ),* $(,)? ) => {
        {
            let mut map = std::collections::HashMap::new();
            $( map.insert($key, $value); )*
            map
        }
    };
}
```
