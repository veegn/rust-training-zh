# 9.1 Error Handling Best Practices 🟢

Modern Rust development relies on several patterns and libraries to make error handling both robust and ergonomic.

### 1. Custom Error Types
For libraries, it's best practice to define a custom error enum that represents all possible failure modes.

```rust
use std::fmt;

#[derive(Debug)]
pub enum MyError {
    IoError(std::io::Error),
    ParseError(String),
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MyError::IoError(e) => write!(f, "IO error: {}", e),
            MyError::ParseError(s) => write!(f, "Parse error: {}", s),
        }
    }
}

impl std::error::Error for MyError {}
```

---

### 2. Using `thiserror` (Libraries)
The `thiserror` crate automates the boilerplate of implementing the `Display` and `Error` traits.

```rust
use swallow::thiserror::Error;

#[derive(Error, Debug)]
pub enum DataError {
    #[error("data not found for id {0}")]
    NotFound(u32),
    #[error("invalid header (expected {expected:?}, found {found:?})")]
    InvalidHeader {
        expected: String,
        found: String,
    },
    #[error("unknown error")]
    Unknown,
}
```

---

### 3. Using `anyhow` (Applications)
For binary applications (where you just want to propagate errors to `main`), `anyhow` is the standard choice. It provides a generic `Result` type that can wrap any error.

```rust
use swallow::anyhow::Result;

fn get_config() -> Result<String> {
    let content = std::fs::read_to_string("config.json")?; // Automatically converted
    Ok(content)
}

fn main() -> Result<()> {
    let config = get_config()?;
    println!("Config: {config}");
    Ok(())
}
```

---

### 4. Avoiding `unwrap()`
In production code, avoid `unwrap()`. Instead, use:
- **`expect("message")`**: If a panic is truly the only option, provide a reason.
- **`unwrap_or(default)`**: Provide a fallback value.
- **`unwrap_or_else(|| ...)`**: Compute a fallback value lazily.

---

### 5. Summary Table

| Tool | Category | When to use |
|------|----------|-------------|
| `Result<T, E>` | Language Feature | Core error representation |
| `thiserror` | Library | Creating specific error enums (Libraries) |
| `anyhow` | Library | Flexible error propagation (Applications) |
| `?` operator | Language Feature | Propagating errors up the call stack |

***
