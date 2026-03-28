[English Original](../en/ch10-1-generics.md)

# 10.1 泛型 🟢

**泛型 (Generics)** 是具体类型或其他属性的抽象代称。我们可以使用泛型来创建项（如函数签名或结构体）的定义，随后我们可以将这些定义与许多不同的具体数据类型结合使用。

### 1. 在函数定义中
在定义使用泛型的函数时，我们将泛型放置在函数签名中原先指定参数和返回值数据类型的地方。

```rust
fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}
```

---

### 2. 在结构体定义中
我们还可以使用 `<>` 语法定义结构体，以便在一个或多个字段上使用泛型类型参数。

```rust
struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
}
```

---

### 3. 在方法定义中
我们可以在结构体和枚举上实现方法，并在它们的定义中也使用泛型类型。

```rust
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}
```

---

### 4. 使用泛型的代码的性能
Rust 以一种确保你的代码运行速度不逊于使用具体类型的方式来实现泛型。Rust 通过在编译时对使用泛型的代码执行 **单态化 (Monomorphization)** 来实现这一点。

单态化是在编译时通过填充所使用的具体类型，将泛型代码转化为特定代码的过程。

---

### 对 C/C++ 开发者的总结
- **在 C++ 中**：你使用 **模板 (Templates)**。模板也会被单态化（实例化），因此性能是一致的。
- **在 Rust 中**：带有 **Trait 限定** (如 `<T: PartialOrd>`) 的泛型等同于 **C++20 Concepts**。它们提供了比传统 C++ 模板好得多的错误消息，因为要求是在声明处检查的，而不仅仅是在模板实例化时检查。

***
