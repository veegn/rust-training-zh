# 14. Unsafe Rust and FFI 🔴

> **What you'll learn:**
> - When to use `unsafe` (and why you probably won't need to)
> - **PyO3**: The bridge for writing Rust extensions for Python
> - Built-in testing vs `pytest`
> - Mocking and Benchmarking

## What is Unsafe?

In Rust, `unsafe` is a keyword that tells the compiler: "I know what I'm doing, please let me take care of the memory safety checks myself." You use it for things the compiler can't verify, like:
- Dereferencing raw pointers.
- Calling functions from another language (C/Python).

**Rule of thumb:** 99% of your code should be safe Rust. Use `unsafe` only when you must interface with other languages or need extreme, low-level performance tuning.

---

## PyO3: Rust for Python 🐍

PyO3 is the "killer feature" for Python developers. It allows you to write Rust functions and classes that can be imported directly into Python as a module.

### A Rust Function for Python
```rust
use pyo3::prelude::*;

#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

#[pymodule]
fn my_rust_lib(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    Ok(())
}
```

### Why use PyO3?
1. **Performance**: Replace slow Python loops with zero-cost Rust.
2. **Safety**: Python extensions in C are notoriously hard to get right. Rust makes them memory-safe.
3. **Ecosystem**: Use any Rust crate (like `serde` or `tokio`) inside your Python project.

---

## Testing: Rust vs Pytest

Rust has a built-in test runner. Tests are usually written in the same file as the code, inside a `tests` module.

| Feature | Pytest | Rust |
|---------|--------|------|
| Run all tests | `pytest` | `cargo test` |
| Exact match | `assert x == y` | `assert_eq!(x, y)` |
| Expect error | `pytest.raises(Err)`| `#[should_panic]` |
| Fast/Slow tests| `pytest.mark.slow` | `#[ignore]` |

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(2 + 2, 4);
    }
}
```

---

## Exercises

<details>
<summary><strong>🏋️ Exercise: First PyO3 Module</strong></summary>

**Challenge**: Research the `maturin` tool. How do you initialize a new Rust-Python project from the command line?

<details>
<summary>🔑 Solution</summary>

Use `maturin init`. It will set up a project structure including a `Cargo.toml` with `pyo3` and a `pyproject.toml` ready for building.

</details>
</details>

***
