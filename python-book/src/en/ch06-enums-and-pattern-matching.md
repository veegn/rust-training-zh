# 6. Enums and Pattern Matching 🟡

> **What you'll learn:**
> - Rust enums with data vs Python `Union` types
> - Exhaustive `match` vs Python's `match/case`
> - `Option<T>`: The type-safe replacement for `None`
> - How enums replace multiple Python patterns (Tagged Unions, Constants)

## Algebraic Data Types vs Union Types

Python 3.10 introduced `match` and type unions (`Union[Circle, Rectangle]`). Rust's enums go further: each variant can carry **different data**, and the compiler **enforces** that you handle every case.

### Rust Enums — Data-Carrying Variants
```rust
enum Shape {
    Circle(f64),                           // Radius
    Rectangle(f64, f64),                   // Width, Height
    Triangle { base: f64, height: f64 },    // Named fields
}

fn area(shape: &Shape) -> f64 {
    match shape {
        Shape::Circle(r) => std::f64::consts::PI * r * r,
        Shape::Rectangle(w, h) => w * h,
        Shape::Triangle { base, height } => 0.5 * base * height,
        // ❌ Compiler error if a variant is missing!
    }
}
```

---

## Exhaustive Pattern Matching

In Python, if you miss a `case` in a `match` statement, it returns `None` silently. In Rust, **it won't even compile**.

```rust
enum Status { Pending, Active, Closed }

fn describe(s: Status) -> &'static str {
    match s {
        Status::Pending => "Waiting...",
        Status::Active => "Running!",
        Status::Closed => "Finished",
        // No wildcard (_) needed because all variants are covered.
    }
}
```

### Pattern Matching Features:
- **Ranges**: `1..=10 => println!("small")`
- **Multiple values**: `1 | 2 | 3 => println!("low")`
- **Guards**: `t if t > 100 => println!("boiling")`

---

## Option for None Safety

`Option<T>` is arguably the most important type for Python developers. It replaces `None` with a variant called `None`, and it **forces** you to check for its existence.

```rust
fn find_user(id: i32) -> Option<String> {
    if id == 1 { Some("Alice".to_string()) } else { None }
}

let user = find_user(1);
// user.to_uppercase(); // ❌ Error: user is Option<String>, not String!

// You MUST handle it:
match user {
    Some(name) => println!("Hello, {name}"),
    None => println!("User not found"),
}

// Or use if let:
if let Some(name) = find_user(1) {
    println!("{name}");
}
```

---

## Exercises

<details>
<summary><strong>🏋️ Exercise: Message Processor</strong></summary>

**Challenge**: Define an enum `Message` with variants `Quit`, `Move { x: i32, y: i32 }`, and `Write(String)`. Implement a function `process(m: Message)` that uses `match` to print different actions.

<details>
<summary>🔑 Solution</summary>

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
}

fn process(m: Message) {
    match m {
        Message::Quit => println!("Shutting down..."),
        Message::Move { x, y } => println!("Moving to ({x}, {y})"),
        Message::Write(text) => println!("Message: {text}"),
    }
}

fn main() {
    process(Message::Move { x: 10, y: 20 });
    process(Message::Write("Hello Rust".to_string()));
    process(Message::Quit);
}
```

</details>
</details>

***
