# 10. Error Handling Patterns 🟢

> **What you'll learn:**
> - When to use `thiserror` (libraries) vs `anyhow` (applications).
> - Error conversion chains with `#[from]` and `.context()`.
> - How the `?` operator desugars.
> - When to panic vs return errors.

## thiserror vs anyhow

| | `thiserror` | `anyhow` |
|---|---|---|
| **Use in** | Libraries, shared crates | Applications, binaries |
| **Error types** | Concrete enums (matchable) | `anyhow::Error` (opaque) |
| **Effort** | Define your error enum | Just use `Result<T>` |

---

## Library Error Pattern (thiserror)

Use `thiserror` to define meaningful, matchable error types for your library's users.

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MyError {
    #[error("I/O failed: {0}")]
    Io(#[from] std::io::Error),

    #[error("not found: {id}")]
    NotFound { id: u64 },
}
```

---

## Application Error Pattern (anyhow)

Use `anyhow` for top-level code where you want to add human-readable context to errors as they propagate.

```rust
use anyhow::{Context, Result};

fn load_config() -> Result<Config> {
    let s = std::fs::read_to_string("cfg.json")
        .context("failed to read config file")?;
    
    serde_json::from_str(&s)
        .context("failed to parse JSON")
}
```

---

## The `?` Operator

The `?` operator is sugar for a `match` that automatically converts error types using the `From` trait and returns early on error.

```rust
// This:
let value = op()?;

// Desugars to:
let value = match op() {
    Ok(v) => v,
    Err(e) => return Err(From::from(e)),
};
```

> **Note**: `?` also works with `Option` in functions that return `Option`.

---

## When to Panic

- **Use `Result`** for *expected* errors (file not found, network timeout).
- **Use `panic!`** for *bugs* (index out of bounds, internal invariants violated).
- **Use `catch_unwind`** only at major boundaries (FFI or thread pool workers).

***
