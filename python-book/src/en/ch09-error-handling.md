# 9. Error Handling 🟡

> **What you'll learn:**
> - `Result<T, E>` vs Python's Exceptions
> - The `?` operator for clear error propagation
> - Defining custom error enums with `thiserror`
> - Why explicit error handling prevents silent production bugs

## Exceptions vs Result

In Python, errors are **thrown** and can be caught anywhere (or not at all). In Rust, errors are **values** returned by functions. They are visible in the function signature and **must** be handled.

### Python: Implicit Exceptions
```python
def load_config(path):
    with open(path) as f:
        return json.load(f) 
# This might raise FileNotFoundError or JSONDecodeError. 
# You can't tell from the signature!
```

### Rust: Explicit Result
```rust
fn load_config(path: &str) -> Result<Config, ConfigError> {
    let s = std::fs::read_to_string(path)?; // Returns early on failure
    let config = serde_json::from_str(&s)?; // Returns early on failure
    Ok(config)
}
```

---

## The ? Operator: Visible Propagation

The `?` operator is Rust's way of saying: "If this succeeded, give me the value. If it failed, return the error from this function immediately."

It's like Python's exception propagation, but:
1. It's **visible** (you see the `?` in the code).
2. It's **typed** (it must match the return type of the current function).

```rust
fn process() -> Result<(), io::Error> {
    step_one()?; // If this fails, process() returns Err
    step_two()?; 
    Ok(())
}
```

---

## custom Error Types with `thiserror`

Instead of generic strings, Rust uses enums to categorize errors. The `thiserror` crate makes this extremely ergonomic.

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("User {0} was not found")]
    NotFound(i32),

    #[error("Network error occurred: {0}")]
    Network(#[from] std::io::Error), // Auto-convert from IO errors!
}

fn fetch_user(id: i32) -> Result<User, AppError> {
    // ...
    Err(AppError::NotFound(id))
}
```

### Quick Mapping:
- `try / except` → `match result { Ok(v) => ..., Err(e) => ... }`
- `raise Exception("...")` → `return Err(AppError::...)`
- `finally` → `Drop` trait (happens automatically when variables go out of scope)

---

## Exercises

<details>
<summary><strong>🏋️ Exercise: Safe Division</strong></summary>

**Challenge**: Write a function `divide(a: f64, b: f64) -> Result<f64, String>` that returns an error if `b` is zero. Test it using `match`.

<details>
<summary>🔑 Solution</summary>

```rust
fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        return Err("Cannot divide by zero".to_string());
    }
    Ok(a / b)
}

fn main() {
    match divide(10.0, 0.0) {
        Ok(val) => println!("Result: {val}"),
        Err(e) => println!("Error: {e}"),
    }
}
```

</details>
</details>

***
