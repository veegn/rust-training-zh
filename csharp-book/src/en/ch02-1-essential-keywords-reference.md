# Essential Rust Keywords for C# Developers

> **What you'll learn:** A quick-reference mapping of Rust keywords to their C# equivalents — visibility modifiers, ownership keywords, control flow, type definitions, and pattern matching syntax.
>
> **Difficulty:** 🟢 Beginner

Understanding Rust's keywords and their purposes helps C# developers navigate the language more effectively.

---

## Visibility and Access Control

### C# Access Modifiers
```csharp
public int PublicField;     // Everywhere
private int privateField;   // Only this class
internal int internalField; // This assembly
```

### Rust Visibility Keywords
```rust
pub struct PublicStruct {
    pub public_field: i32,     // Public
    private_field: i32,        // Private by default
}

pub(crate) fn internal_fn() {} // Public within current crate (like internal)
```

---

## Memory and Ownership

### C# Memory Keywords
```csharp
public void Method(ref int val); // Pass by reference
public void Method(in int val);  // Readonly reference
```

### Rust Ownership Keywords
```rust
fn read_only(data: &Vec<i32>);   // Immutable reference (&)
fn modify(data: &mut Vec<i32>); // Mutable reference (&mut)

let closure = move || { ... };   // Force move capture
let boxed = Box::new(42);        // Heap allocation
```

---

## Control Flow

### C# vs Rust
*   **`return`**: Optional in Rust if the last expression calculates the value.
*   **`loop`**: Infinite loop (`while(true)`).
*   **`break` / `continue`**: Standard usage, but `break` can return a value from a `loop`.

---

## Type Definition

| **C#** | **Rust** | **Notes** |
| :--- | :--- | :--- |
| `class` / `struct` | `struct` | Data structure |
| `interface` | `trait` | Shared behavior |
| `enum` | `enum` | Enums are algebraic data types in Rust |
| `using alias` | `type` | Type alias |

---

## Keywords Summary Table

| **Purpose** | **C#** | **Rust** | **Key Difference** |
| :--- | :--- | :--- | :--- |
| **Visibility** | `public`, `private` | `pub`, default private | More granular with `pub(crate)` |
| **Variables** | `var`, `readonly` | `let`, `let mut` | Immutable by default |
| **Functions** | `method()` | `fn` | Standalone functions |
| **Patterns** | `switch`, `is` | `match`, `if let` | Exhaustive matching required |
| **References** | `ref`, `in` | `&mut`, `&` | Compile-time borrow checking |
