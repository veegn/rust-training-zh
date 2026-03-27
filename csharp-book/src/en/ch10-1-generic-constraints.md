# Generic Constraints: where vs trait bounds

> **What you'll learn:** Rust's trait bounds vs C#'s `where` constraints, the `where` clause syntax, and conditional trait implementations.
>
> **Difficulty:** Advanced

In C#, generic constraints are used to specify requirements for type parameters. Rust achieves the same via **Trait Bounds**.

---

## Basic Syntaxes
There are two ways to write generic constraints in Rust:

### 1. In-line Trait Bounds
Good for simple constraints.
```rust
fn print_debug<T: std::fmt::Debug>(value: T) {
    println!("{:?}", value);
}
```

### 2. The `where` Clause
Recommended for complex constraints or when multiple type parameters are involved. It keeps the function signature clean.
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

## C# to Rust Comparison
| **C# Constraint** | **Rust Trait Bound** | **Notes** |
| :--- | :--- | :--- |
| **`where T : class`** | N/A | Rust doesn't have a direct equivalent for "must be a heap-allocated class". |
| **`where T : struct`** | `T: Copy` | Closest equivalent for stack-allocated, copyable types. |
| **`where T : new()`** | `T: Default` | The `Default` trait provides a standard `default()` constructor. |
| **`where T : IInterface`** | `T: Trait` | The most direct mapping. |

---

## Conditional Implementations
Rust allows you to implement a trait for a generic type **only if** certain conditions are met. This is a powerful feature not found in C#.

```rust
struct Pair<T> { x: T, y: T }

// This method exists for ALL Pairs
impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self { Self { x, y } }
}

// These methods exist ONLY if T implements Display and PartialOrd
impl<T: std::fmt::Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}
```

---

## Summary for C# Developers
| **Feature** | **C#** | **Rust** |
| :--- | :--- | :--- |
| **Keyword** | `where` | `where` or `: Trait` |
| **Multiple Traits** | `where T : IA, IB` | `T: TraitA + TraitB` |
| **Constructor** | `new()` constraint | `Default` trait |
| **Static Methods** | Not easily constrained | Traits can have static methods |

---

## Exercise: Write a Generic Function
**Challenge:** Write a function `print_and_clone` that takes a type `T` which must be printable (Display) and cloneable (Clone). Use the `where` clause.

```rust
fn print_and_clone<T>(value: &T) -> T
where
    T: std::fmt::Display + Clone,
{
    println!("Cloning: {}", value);
    value.clone()
}
```
**Takeaway:** `where` clauses keep your generic logic readable, especially when you start combining multiple traits and lifetimes.
