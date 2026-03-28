# 10. Traits and Generics 🟡

> **What you'll learn:**
> - Traits: Rust's version of explicit "Duck Typing"
> - `Protocol` (PEP 544) vs Rust Traits
> - Generic type bounds (T: Trait)
> - Common standard library traits (Display, Debug, Clone, etc.)

## Traits vs Duck Typing

Python follows **Duck Typing**: "If it walks like a duck, it's a duck." Rust follows **Trait Contracts**: "I'll tell you exactly which behaviors I need, and the compiler will verify them."

### Python: Implicit Duck Typing
```python
def total_area(shapes):
    return sum(s.area() for s in shapes)

# Crashes at runtime if an object doesn't have .area()
```

### Rust: Explicit Trait Contract
```rust
trait HasArea {
    fn area(&self) -> f64;
}

// The compiler ensures ONLY types with the HasArea trait are passed
fn total_area(shapes: &[&dyn HasArea]) -> f64 {
    shapes.iter().map(|s| s.area()).sum()
}
```

---

## Common Standard Library Traits

These are Rust's equivalent of Python's **"dunder methods"** (`__str__`, `__repr__`, etc.).

| Rust Trait | Python Dunder | Purpose |
|------------|--------------|---------|
| `Display` | `__str__` | Human-readable string |
| `Debug` | `__repr__` | Developer-friendly string (`{:?}`) |
| `Clone` | `copy.deepcopy` | Explicit deep copy |
| `PartialEq` | `__eq__` | Equality comparison (`==`) |
| `Add` | `__add__` | Operator overloading (`+`) |
| `Iterator` | `__iter__` / `__next__` | Loopable types |

### Deriving Traits
In Rust, you often don't have to write these manually. You can **derive** them:
```rust
#[derive(Debug, PartialEq, Clone)]
struct User {
    id: i32,
    username: String,
}
```

---

## Generics with Trait Bounds

Generics allow you to write code that works with any type `T`, as long as `T` follows certain rules (bounds).

```rust
// T can be anything, as long as it implements Display
fn print_it<T: std::fmt::Display>(item: T) {
    println!("Value is: {item}");
}

// Multiple bounds: T must be Display AND Debug
fn verbose_print<T>(item: T) 
where T: std::fmt::Display + std::fmt::Debug 
{
    println!("{item} (debug: {item:?})");
}
```

---

## Exercises

<details>
<summary><strong>🏋️ Exercise: Summary Trait</strong></summary>

**Challenge**: Define a trait `Summary` with a method `summarize(&self) -> String`. Implement it for `NewsArticle { headline: String }` and `Tweet { content: String }`. Write a function `notify(item: &impl Summary)` that calls it.

<details>
<summary>🔑 Solution</summary>

```rust
trait Summary {
    fn summarize(&self) -> String;
}

struct NewsArticle { headline: String }
struct Tweet { content: String }

impl Summary for NewsArticle {
    fn summarize(&self) -> String { format!("Headline: {}", self.headline) }
}

impl Summary for Tweet {
    fn summarize(&self) -> String { format!("Twitter: {}", self.content) }
}

fn notify(item: &impl Summary) {
    println!("Breaking: {}", item.summarize());
}

fn main() {
    let t = Tweet { content: "Rust is cool".to_string() };
    notify(&t);
}
```

</details>
</details>

***
