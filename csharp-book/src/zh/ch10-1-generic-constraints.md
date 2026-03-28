[English Original](../en/ch10-1-generic-constraints.md)

# 泛型约束：where 与 trait bound

> **你将学到什么：** Rust 的 trait bound 与 C# `where` 约束的区别，`where` 子句语法，以及条件 trait 实现。
>
> **难度：** 高级

在 C# 中，泛型约束用于指定类型参数的要求。而在 Rust 中，是通过 **Trait Bounds** 来实现同样的目的。

---

## 基本语法
Rust 中有两种书写泛型约束的方式：

### 1. 行内 Trait Bound (In-line Trait Bounds)
适用于简单的约束场景。
```rust
fn print_debug<T: std::fmt::Debug>(value: T) {
    println!("{:?}", value);
}
```

### 2. `where` 子句 (`where` Clause)
对于复杂的约束或涉及多个类型参数的场景，推荐使用 `where` 子句。它能让你的函数签名保持整洁。
```rust
fn compare_and_print<T, U>(a: T, b: U)
where
    T: std::fmt::Display + Clone,
    U: std::fmt::Debug,
{
    println!("A: {}, B: {:?}", a, b);
}
```

---

## C# 与 Rust 对应关系
| **C# 约束** | **Rust Trait Bound** | **说明** |
| :--- | :--- | :--- |
| **`where T : class`** | 无 | Rust 中没有直接对应“必须是堆分配类”的约束。 |
| **`where T : struct`** | `T: Copy` | 最接近的对应物，用于表示栈分配、可拷贝的类型。 |
| **`where T : new()`** | `T: Default` | `Default` trait 提供了一个标准的 `default()` 构造方法。 |
| **`where T : IInterface`** | `T: Trait` | 最直接的映射。 |

---

## 条件实现 (Conditional Implementations)
Rust 允许你**仅在**满足某些条件时为泛型类型实现特定的 Trait。这是 C# 目前尚未提供的强大特性。

```rust
struct Pair<T> { x: T, y: T }

// 这个方法对所有的 Pair 都有效
impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self { Self { x, y } }
}

// 只有在 T 实现了 Display 和 PartialOrd 两个 trait 时，这些方法才有效
impl<T: std::fmt::Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("最大的成员是 x = {}", self.x);
        } else {
            println!("最大的成员是 y = {}", self.y);
        }
    }
}
```

---

## C# 开发者总结表
| **特性** | **C#** | **Rust** |
| :--- | :--- | :--- |
| **关键字** | `where` | `where` 或 `: Trait` |
| **多重 Trait** | `where T : IA, IB` | `T: TraitA + TraitB` |
| **构造器** | `new()` 约束 | `Default` trait |
| **静态方法** | 不容易被约束 | Trait 可以包含静态（关联）方法 |

---

## 练习：编写一个泛型函数
**挑战：** 编写一个名为 `print_and_clone` 的函数，要求其类型参数 `T` 必须同时满足可打印 (`Display`) 和可克隆 (`Clone`)。请使用 `where` 子句。

```rust
fn print_and_clone<T>(value: &T) -> T
where
    T: std::fmt::Display + Clone,
{
    println!("正在克隆：{}", value);
    value.clone()
}
```
**关键理解：** `where` 子句能保持泛型逻辑的可读性，特别是当你开始将多个 Trait 甚至生命周期联合在一起使用时。
