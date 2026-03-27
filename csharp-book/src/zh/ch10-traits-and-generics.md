# Trait：Rust 的接口机制

> **你将学到什么：** Trait 与 C# 接口的对应关系、默认方法实现、trait object (`dyn Trait`) 与泛型约束 (`impl Trait`) 的区别，以及标准库中的常见 trait。
>
> **难度：** 中级

Trait 是 Rust 中定义共享行为的方式。概念上它们和 C# 的接口非常像，但被用在了一种更加灵活且解耦的方式上。

---

## Trait vs 接口
在 C# 中，一个类（Class）必须明确声明它实现了某个接口。而在 Rust 中，你可以为任意类型实现一个 Trait，甚至是那些不是由你定义的类型（但需遵循一定规则）。

### C# 接口
```csharp
public interface IShape {
    double Area();
}

public class Circle : IShape {
    public double Radius { get; set; }
    public double Area() => Math.PI * Radius * Radius;
}
```

### Rust Trait
```rust
trait Shape {
    fn area(&self) -> f64;
}

struct Circle { radius: f64 }

impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}
```

---

## 默认实现
就像现代 C# (8.0+) 一样，Rust 的 Trait 可以拥有默认的方法实现。

```rust
trait Summary {
    fn summarize(&self) -> String {
        String::from("(点击阅读更多...)")
    }
}
```

---

## 泛型与 Trait 约束 (Trait Bounds)
你可以使用 Trait 来限制哪些类型可以被传入一个泛型函数。

```rust
fn print_area<T: Shape>(shape: &T) {
    println!("该形状的面积是 {}", shape.area());
}
```
**静态分发 (Static Dispatch):** 编译器会为你调用的每一种具体类型，生成一个专门的 `print_area` 函数副本。这非常快（零运行时成本），但可能会增加二进制文件的体积。

---

## Trait Object (`dyn Trait`)
有些时候，你需要在同一个集合中存放不同的类型。由于 Rust 需要在编译期知道类型的大小，你必须通过指针（如 `Box` 或 `&`）来使用 Trait Object。

```rust
let shapes: Vec<Box<dyn Shape>> = vec![
    Box::new(Circle { radius: 1.0 }),
    Box::new(Square { side: 2.0 }),
];
```
**动态分发 (Dynamic Dispatch):** 与泛型不同，这会在运行时通过一个“虚函数表 (vtable)”来寻找对应的正确方法。虽然速度稍慢，但它支持异类集合（Heterogeneous Collection）。

---

## C# 开发者总结表
| **概念** | **C# 对应物** | **Rust 现实** |
| :--- | :--- | :--- |
| **接口** | `interface I` | `trait T` |
| **实现** | `class C : I` | `impl T for C` |
| **泛型** | `where T : I` | `<T: T>` (静态分发) |
| **多态** | `List<I>` | `Vec<Box<dyn T>>` (动态分发) |
| **隐式实现** | 不可以 | 可以 (但需遵循孤儿规则) |

---

## 练习：实现一个 Trait
**挑战：** 定义一个包含 `draw` 方法的 `Drawable` trait。为 `Point` 结构体实现它，并编写一个能够接收 `impl Drawable` 参数的渲染函数。

```rust
trait Drawable {
    fn draw(&self);
}

struct Point { x: i32, y: i32 }

impl Drawable for Point {
    fn draw(&self) { println!("({}, {})", self.x, self.y); }
}

fn render(item: impl Drawable) {
    item.draw();
}
```
**关键理解：** Trait 是将 Rust 代码粘合在一起的关键。它们提供了如同接口般的抽象能力，同时又没有继承体系中僵化的层级链。
