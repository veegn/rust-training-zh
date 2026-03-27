# Traits: Rust's Interfaces

> **What you'll learn:** Traits vs C# interfaces, default method implementations, trait objects (`dyn Trait`) vs generic bounds (`impl Trait`), and common standard library traits.
>
> **Difficulty:** Intermediate

Traits are Rust's way of defining shared behavior. They are conceptually similar to C# interfaces but used in a more flexible, decoupled way.

---

## Traits vs Interfaces
In C#, a class must declare that it implements an interface. In Rust, you can implement a trait for any type, even types you didn't define (with some rules).

### C# Interface
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

## Default Implementations
Just like modern C# (8.0+), Rust traits can have default method implementations.

```rust
trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}
```

---

## Generics and Trait Bounds
You use traits to constrain what types can be passed to a generic function.

```rust
fn print_area<T: Shape>(shape: &T) {
    println!("The area is {}", shape.area());
}
```
**Static Dispatch:** The compiler creates a version of `print_area` for every type you call it with. This is fast (zero runtime cost) but can increase binary size.

---

## Trait Objects (`dyn Trait`)
Sometimes you need to store different types in a single collection. Since Rust needs to know the size of types, you must use a trait object with a pointer (like `Box` or `&`).

```rust
let shapes: Vec<Box<dyn Shape>> = vec![
    Box::new(Circle { radius: 1.0 }),
    Box::new(Square { side: 2.0 }),
];
```
**Dynamic Dispatch:** Unlike generics, this uses a "vtable" at runtime to find the right method. This is slower but allows for heterogeneous collections.

---

## Summary for C# Developers
| **Concept** | **C# Equivalent** | **Rust Reality** |
| :--- | :--- | :--- |
| **Interface** | `interface I` | `trait T` |
| **Implementation** | `class C : I` | `impl T for C` |
| **Generics** | `where T : I` | `<T: T>` (Static Dispatch) |
| **Polymorphism** | `List<I>` | `Vec<Box<dyn T>>` (Dynamic) |
| **Implicit Implementation** | Not possible | Possible (Orphan Rule applies) |

---

## Exercise: Implement a Trait
**Challenge:** Define a `Drawable` trait with a `draw` method. Implement it for a `Point` struct and write a function that takes an `impl Drawable`.

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
**Takeaway:** Traits are the glue that holds Rust code together. They provide the abstraction power of interfaces without the rigid hierarchy of inheritance.
