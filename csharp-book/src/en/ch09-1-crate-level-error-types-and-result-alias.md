# Production Error Patterns

> **What you'll learn:** The production pattern of defining a per-crate error enum with `thiserror`, creating a `Result<T>` type alias, and when to choose `thiserror` (libraries) vs `anyhow` (applications).
>
> **Difficulty:** Intermediate

In production Rust, we avoid using `String` or `Box<dyn Error>` for errors. Instead, we define structured enums to represent every failure mode our code can encounter.

---

## The Crate-Level Error Pattern
A common practice is to define a single `Error` enum for your crate and a `Result` alias.

```rust
// error.rs
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("I/O failure: {0}")]
    Io(#[from] std::io::Error), // Automatically convert std::io::Error to AppError

    #[error("Database error: {0}")]
    Sql(String),

    #[error("Validation failed: {message}")]
    Validation { message: String },
}

pub type Result<T> = std::result::Result<T, AppError>;
```

### Benefits
1.  **Cleaner Signatures:** Your functions return `Result<User>` instead of `Result<User, AppError>`.
2.  **Automatic Propagation:** Using `#[from]` allows the `?` operator to automatically convert low-level errors (like `io::Error`) into your high-level `AppError`.

---

## `thiserror` vs `anyhow`
Choosing between these two crates is the first decision you'll make in any project.

| **Crate** | **Best For** | **Philosophy** |
| :--- | :--- | :--- |
| **`thiserror`** | **Libraries** | For code that others will call and need to handle specific error cases (`match`). |
| **`anyhow`** | **Applications** | For final binary programs. It provides an opaque `Error` type that is easy to add context to. |

### Using `anyhow` in Applications
```rust
use anyhow::{Context, Result};

fn main() -> Result<()> {
    let config = std::fs::read_to_string("config.toml")
        .context("Missing config.toml file")?; // Adds a human-readable message to the error
    Ok(())
}
```

---

## Summary for C# Developers
*   **`thiserror`** is like defining custom `Exception` classes in your library.
*   **`anyhow`** is like using a catch-all `Exception` in your `Main` method and wrapping it with helpful messages: `throw new Exception("...", innerException)`.

---

## Exercise: Design a Crate Error
**Challenge:** Create a `RegistrationError` using `thiserror` that supports `DuplicateEmail`, `WeakPassword`, and `DatabaseError`.

```rust
#[derive(Error, Debug)]
pub enum RegistrationError {
    #[error("Email {0} is already taken")]
    DuplicateEmail(String),
    #[error("Database failed")]
    Database(#[from] sqlx::Error),
}
```
**Takeaway:** A well-designed error enum serves as documentation for your API, telling callers exactly what can go wrong and giving them the tools to handle it gracefully.
