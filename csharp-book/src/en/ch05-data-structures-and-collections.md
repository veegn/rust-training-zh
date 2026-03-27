# Tuples and Data Structures

> **What you'll learn:** Rust tuples vs C# `ValueTuple`, arrays and slices, and the newtype pattern for domain modeling with zero-cost type safety.
>
> **Difficulty:** 🟢 Beginner

## Tuples and Destructuring
C# has `ValueTuple` (since C# 7). Rust tuples are similar but more deeply integrated into the language.

### Rust Tuples
```rust
// Rust tuples are immutable by default and don't support named elements.
let point = (10, 20); // (i32, i32)
let (x, y) = point;    // Destructuring

// Access by index
println!("x={}, y={}", point.0, point.1);

// Tuple as return type
fn divide(a: i32, b: i32) -> (i32, i32) {
    (a / b, a % b)
}
```

### Tuple Structs (Newtypes)
When a plain tuple isn't descriptive enough, or you want to prevent Mixing units, use a tuple struct:
```rust
struct Meters(f64);
struct Celsius(f64);

let d = Meters(100.0);
let t = Celsius(36.6);
// d == t; // ❌ Compile error: different types!
```

---

## The Newtype Pattern: Domain Modeling with Zero Cost
One of Rust's primary tools for encoding business rules into the type system is the **Newtype pattern**.

### C# Baseline (Runtime Guards)
In C#, you often use strings for everything and rely on runtime validation.
```csharp
public void SendEmail(string email) {
    if (!email.Contains('@')) throw new ArgumentException("Invalid email");
    // ...
}
```

### Rust Newtype Approach (Compile-Time Proof)
In Rust, you can make the type system guarantee validity.
```rust
pub struct Email(String);

impl Email {
    pub fn new(raw: &str) -> Result<Self, &'static str> {
        if raw.contains('@') {
            Ok(Email(raw.to_string()))
        } else {
            Err("invalid email")
        }
    }
}

// Any function taking Email is GUARANTEED to have a valid email.
fn send_email(to: Email) { ... }
```
**Zero-cost:** Newtypes compile down to the same machine code as the inner type.

---

## Arrays and Slices

### 1. Arrays
Fixed size, stack allocated.
```rust
let numbers: [i32; 5] = [1, 2, 3, 4, 5];
```

### 2. Slices
A reference to a contiguous sequence of elements. Slices are "views" into memory.
```rust
let slice: &[i32] = &numbers[1..4]; // Elements 1, 2, 3
```

### 3. Slices as Parameters
Functionally similar to `ReadOnlySpan<T>` in C#.
```rust
fn process(data: &[i32]) { ... }

// Works with both arrays and vectors!
process(&array);
process(&vec);
```

---

## Structs vs Classes

| **Feature** | **C# Class** | **Rust Struct** |
| :--- | :--- | :--- |
| **Location** | Always on the Heap | Stack by default |
| **Methods** | Defined inside class | Defined in `impl` block |
| **Privacy** | Keywords per member | Default private |

### Rust Struct Example
```rust
pub struct Person {
    pub name: String,
    age: u32, // Private
}

impl Person {
    pub fn new(name: String, age: u32) -> Self {
        Self { name, age }
    }
    
    pub fn get_info(&self) -> String {
        format!("{} is {}", self.name, self.age)
    }
}
```
**Core Insight:** C# objects always involve a reference and a heap allocation. Rust structs stay on the stack unless you explicitly move them to the heap (e.g., via `Box`).
