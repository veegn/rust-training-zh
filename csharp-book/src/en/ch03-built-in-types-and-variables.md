# Variables and Mutability

> **What you'll learn:** Rust's variable declaration and mutability model vs C#'s `var`/`const`, primitive type mappings, the critical `String` vs `&str` distinction, type inference, and how Rust handles casting and conversions differently from C#.
>
> **Difficulty:** 🟢 Beginner

Variables are one of the first places where Rust feels familiar on the surface but behaves very differently underneath.

---

## Variable Declaration

### C# Variable Declaration
```csharp
// C# - Variables are mutable by default
int count = 0;           // Mutable
count = 5;               // ✅ Works

const int BUFFER_SIZE = 1024; // Compile-time constant
```

### Rust Variable Declaration
```rust
// Rust - Variables are immutable by default
let count = 0;           // Immutable by default
// count = 5;            // ❌ Compile error

let mut count = 0;       // Explicitly mutable
count = 5;               // ✅ Works

const BUFFER_SIZE: usize = 1024; // Compile-time constant
```

### Key Mental Shift
Think of `let` as C#'s `readonly` field semantics applied to all variables.
- **Shadowing**: Rust allows you to declare a new variable with the same name as a previous one, effectively "reusing" the name for a different type or value.

---

## Data Types Comparison

### Primitive Types

| **C# Type** | **Rust Type** | **Size** | **Notes** |
| :--- | :--- | :--- | :--- |
| `byte` | `u8` | 8 bits | Unsigned |
| `int` | `i32` | 32 bits | Default integer |
| `long` | `i64` | 64 bits | |
| `float` | `f32` | 32 bits | IEEE 754 |
| `double` | `f64` | 64 bits | |
| `char` | `char` | 32 bits | Unicode scalar |

### Size Types (isize/usize)
In Rust, `usize` and `isize` match the pointer size of the architecture (32 or 64-bit). They are primarily used for indexing collections, similar to `size_t` in C++.

---

## String Types: String vs &str

This is a critical distinction for C# developers.

*   **`&str` (string slice)**: An immutable reference to a string buffer. Like a `ReadOnlySpan<char>` in C#. String literals are always `&str`.
*   **`String`**: An owned, heap-allocated, growable string. Like a `StringBuilder` or a regular `string` that you can modify.

```rust
let literal: &str = "Hello";           // Borrowed
let mut owned: String = literal.to_string(); // Owned
owned.push_str(" World");
```

---

## Printing and Formatting

Rust uses macros (ending in `!`) for output.

*   `println!("{name} is {age}")`: Captured variables (Rust 1.58+).
*   `println!("{:?}", object)`: Debug printing (requires `#[derive(Debug)]`).
*   `println!("{}", object)`: Display printing (requires implementing the `Display` trait).

---

## Type Casting

Rust has **no implicit numeric conversions**. You must always be explicit.

```rust
let x: i32 = 42;
let y: f64 = x as f64; // Explicit cast

// Safe conversion
let result = u8::try_from(x); // Returns Result
```
