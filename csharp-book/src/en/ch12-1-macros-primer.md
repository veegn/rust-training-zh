# Macros: Code That Writes Code

> **What you'll learn:** Why Rust needs macros (no overloading, no variadic args), `macro_rules!` basics, the `!` suffix convention, and common derive macros.
>
> **Difficulty:** Intermediate

C# has no direct equivalent to Rust macros. Understanding why they exist and how they work removes a major source of confusion for C# developers.

---

## Why Macros?
In C#, you have features that make macros unnecessary, such as:
1.  **Method Overloading**: `void Print(int x)` and `void Print(string s)`.
2.  **Variadic Arguments**: `void Print(params object[] args)`.

**Rust has neither.** Macros fill these gaps by allowing code to handle a variable number of arguments or different types at compile time.

---

## The `!` Suffix
An invocation ending in `!` is a macro, not a function.
*   **`println!("Hello")`**: Handles formatting at compile time.
*   **`vec![1, 2, 3]`**: Expands to code that creates and populates a `Vec`.
*   **`panic!("Error")`**: Stops execution with a message.

---

## Declarative Macros (`macro_rules!`)
These use pattern matching to transform code. It's like a `match` statement for your source code.

```rust
macro_rules! say_hello {
    () => {
        println!("Hello!");
    };
    ($name:expr) => {
        println!("Hello, {}!", $name);
    };
}

say_hello!();        // Prints "Hello!"
say_hello!("Alice"); // Prints "Hello, Alice!"
```

---

## Derive Macros: Auto-Implementing Traits
This is the most common macro you'll use. It's like C#'s `record` feature but more flexible.

```rust
#[derive(Debug, Clone, PartialEq)]
struct User {
    name: String,
    age: u32,
}
```
The compiler automatically generates code for `Debug` (printing), `Clone` (copying), and `PartialEq` (comparison).

---

## `dbg!()`: Quick Debugging
Instead of `Console.WriteLine`, use `dbg!`. It prints the file name, line number, the expression, and its value, then returns the value.

```rust
let x = 5;
let y = dbg!(x * 2) + 1; // Prints [src/main.rs:2] x * 2 = 10
```

---

## Summary for C# Developers
| **Concept** | **C# Equivalent** | **Rust Reality** |
| :--- | :--- | :--- |
| **Overloading** | Multiple methods | Macro or Trait |
| **Variadic Args** | `params` keyword | Macro (e.g., `vec![]`) |
| **Boilerplate** | Manual implementation | `#[derive(...)]` |
| **Source Gen** | Source Generators | Procedural Macros |

---

## Exercise: Use a Macro
**Challenge:** Use the `vec!` macro to create a vector of integers, then use the `dbg!` macro to print its length.

```rust
let v = vec![1, 2, 3, 4, 5];
dbg!(v.len());
```
**Takeaway:** Macros are a powerful tool for reducing boilerplate and adding features like compile-time formatting that aren't possible with regular functions.
