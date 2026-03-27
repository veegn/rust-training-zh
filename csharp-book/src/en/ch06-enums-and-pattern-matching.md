# Enums and Pattern Matching

> **What you'll learn:** Rust's algebraic data types (enums with data) vs C#'s limited enums/unions, `match` expressions with exhaustive checking, and nested pattern destructuring.
>
> **Difficulty:** 🟡 Intermediate

## Algebraic Data Types (Enums)
Rust enums are much more powerful than C# enums. They are **Algebraic Data Types (ADTs)**, meaning each variant can carry its own unique data.

### C# vs Rust Enums
In C#, an enum is just a set of named integer constants. To associate data, you usually need an inheritance hierarchy of classes or records.

```csharp
// C# - Requires inheritance to hold different data
public abstract record Shape;
public record Circle(double Radius) : Shape;
public record Rectangle(double Width, double Height) : Shape;
```

In Rust, this is a single, cohesive type:
```rust
pub enum Shape {
    Circle { radius: f64 },
    Rectangle { width: f64, height: f64 },
    Triangle { base: f64, height: f64 },
}
```

---

## Pattern Matching with `match`
The `match` expression is the primary way to consume an enum. Unlike C#'s `switch`, Rust's `match` is **exhaustive** — the compiler will refuse to build your code if you miss a single variant.

```rust
impl Shape {
    pub fn area(&self) -> f64 {
        match self {
            Shape::Circle { radius } => 3.14 * radius * radius,
            Shape::Rectangle { width, height } => width * height,
            Shape::Triangle { base, height } => 0.5 * base * height,
        }
    }
}
```

### Advanced Patterns
Rust's pattern matching goes far beyond simple equality:
*   **Guards**: `match x { n if n < 0 => ... }`
*   **Ranges**: `match age { 0..=12 => "child", ... }`
*   **Destructuring**: Extract values directly from nested structures.

---

## Comparison Summary

| **Feature** | **C# Enums / Records** | **Rust Enums** |
| :--- | :--- | :--- |
| **Data Association** | Requires inheritance/records | Built-in to variants |
| **Exhaustiveness** | Optional (runtime risk) | **Mandatory** (compile-time safety) |
| **Memory** | Heap (for classes) | Stack (optimized) |
| **Pattern Matching** | `switch` (since C# 8) | `match` (core language feature) |

---

## Exercise: Command Parser
**Challenge:** Model a simple CLI command system where `Quit`, `Echo(String)`, and `Move { x, y }` are different variants.

```rust
enum Command {
    Quit,
    Echo(String),
    Move { x: i32, y: i32 },
}

fn execute(cmd: Command) {
    match cmd {
        Command::Quit => println!("Bye!"),
        Command::Echo(s) => println!("{s}"),
        Command::Move { x, y } => println!("Moving to {x}, {y}"),
    }
}
```
**Takeaway:** Enums allow you to group related but structurally different data into a single type, while `match` ensures you never forget to handle a specific case.
