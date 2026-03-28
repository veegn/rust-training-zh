# 3. Built-in Rust Types 🟢

### Fundamental Types
Rust provides a variety of built-in types that should be familiar to C/C++ developers.

| Category | Types | Examples |
|----------|-------|----------|
| **Signed Integers** | `i8`, `i16`, `i32`, `i64`, `i128`, `isize` | `-1`, `42`, `1_000_000` |
| **Unsigned Integers** | `u8`, `u16`, `u32`, `u64`, `u128`, `usize` | `0`, `42u32`, `42u64` |
| **Floating Point** | `f32`, `f64` | `0.0`, `3.14159` |
| **Boolean** | `bool` | `true`, `false` |
| **Character** | `char` | `'a'`, `'😊'` (4-byte Unicode) |

---

### Variables and Mutability
In Rust, variables are **immutable by default**. To make a variable mutable, you must explicitly use the `mut` keyword.

```rust
fn main() {
    let x = 5;      // x is immutable
    // x = 6;       // ❌ Compile Error

    let mut y = 5;  // y is mutable
    y = 6;          // ✅ Works
}
```

---

### Type Specification and Inference
Rust features powerful type inference but also allows for explicit type annotations.

```rust
fn main() {
    let a: i32 = 42;    // Explicit annotation
    let b = 42u32;     // Suffix annotation
    let c = 42;        // Inferred from usage (defaults to i32)
}
```

#### Function Signatures
Unlike variables, function parameters and return values **must** have explicit types.
```rust
fn add(x: i32, y: i32) -> i32 {
    x + y // No semicolon = return expression
}
```

---

### Shadowing
Rust allows you to declare a new variable with the same name as a previous one, essentially "shadowing" the original. This is useful for changing a variable's type or mutability without creating a new name.

```rust
fn main() {
    let x = 5;
    let x = x + 1;  // Shadowing: the first 'x' is now hidden
    let x = "hello"; // You can even change the type
}
```

***
