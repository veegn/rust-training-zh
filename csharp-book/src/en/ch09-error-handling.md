# Error Handling: Result vs Exceptions

> **What you'll learn:** Why Rust replaces exceptions with `Result<T, E>`, the `?` operator for concise error propagation, and how explicit error handling eliminates hidden control flow.
>
> **Difficulty:** Intermediate

In C#, errors are handled via exceptions. In Rust, errors are part of the type system.

---

## The Core Philosophy
*   **C#**: Control flow is implicit. A method might throw an exception at any time, and you have to remember to catch it.
*   **Rust**: Control flow is explicit. If a function can fail, it **must** return a `Result<T, E>`. The caller is then forced by the compiler to handle both the success and error cases.

---

## Result and Option
Rust uses two main enums for handling "non-perfect" situations:
1.  **`Option<T>`**: Used when a value might be missing (e.g., `None` instead of `null`).
2.  **`Result<T, E>`**: Used when an operation might fail (e.g., `Err(e)` instead of `throw`).

```rust
fn get_user(id: i32) -> Result<User, String> {
    if id < 0 {
        Err("Invalid ID".to_string())
    } else {
        Ok(User { id })
    }
}
```

---

## The `?` Operator
The `?` operator is Rust's way of making error handling concise. It says: "If the result is `Ok`, give me the value; if it's `Err`, return from this function immediately with that error."

```rust
fn process_user(id: i32) -> Result<(), String> {
    let user = get_user(id)?; // Returns early on error
    println!("Processing {}", user.id);
    Ok(())
}
```
**C# Equivalent:** This is roughly like calling a method that throws—the exception bubbles up automatically. The difference is the `?` makes it obvious where the "bubbles" can happen.

---

## Handling Errors
You can handle errors using `match`, `if let`, or functional combinators like `unwrap_or`.

```rust
let email = get_email(10).unwrap_or("default@example.com".to_string());

match get_user(1) {
    Ok(user) => println!("Hello, {}", user.id),
    Err(e) => eprintln!("Error: {}", e),
}
```

---

## Summary for C# Developers
| **Concept** | **C# Approach** | **Rust Approach** |
| :--- | :--- | :--- |
| **Error Type** | `Exception` class | `Result<T, E>` enum |
| **Propagation** | Automatic (implicit) | `?` operator (explicit) |
| **Failure Case** | `throw new X()` | `return Err(X)` |
| **"Nothing"** | `null` | `None` |

---

## Exercise: Propagate an Error
**Challenge:** Write a function that reads a file, parses it as an integer, and returns the result. Use the `?` operator to propagate both I/O and Parse errors.

```rust
fn read_id(path: &str) -> Result<i32, Box<dyn std::error::Error>> {
    let content = std::fs::read_to_string(path)?;
    let id = content.trim().parse::<i32>()?;
    Ok(id)
}
```
**Takeaway:** `Result` makes error handling a first-class citizen. You can't ignore errors, and you don't need expensive stack unwinding for expected failures.
