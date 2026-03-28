# 8.1 Testing Patterns 🟢

Rust has a built-in test harness that makes it easy to write and run tests without needing a third-party library.

### 1. Unit Tests
Unit tests are typically written in the same file as the code they are testing, inside a `tests` module marked with `#[cfg(test)]`.

```rust
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*; // Bring the parent items into scope

    #[test]
    fn it_works() {
        assert_eq!(add(2, 2), 4);
    }
}
```

---

### 2. Integration Tests
Integration tests are entirely external to your library. They live in a `tests` directory at the root of your project.

File structure:
```text
├── src/
│   └── lib.rs
└── tests/
    └── integration_test.rs
```

In `tests/integration_test.rs`:
```rust
use adder; // Import your library crate

#[test]
fn test_add() {
    assert_eq!(adder::add(3, 2), 5);
}
```

---

### 3. Common Test Macros
- `assert!(condition)`: Panics if the condition is false.
- `assert_eq!(left, right)`: Panics if left != right.
- `assert_ne!(left, right)`: Panics if left == right.

---

### 4. Handling Expected Panics
You can test that a function panics when it should by adding the `#[should_panic]` attribute.

```rust
#[test]
#[should_panic(expected = "Guess value must be less than or equal to 100")]
fn greater_than_100() {
    Guess::new(200);
}
```

---

### 5. Running Tests
Use Cargo to run all tests in your project:
```bash
cargo test
```

Useful flags:
- `cargo test -- --nocapture`: Shows output (like `println!`) from passing tests.
- `cargo test test_name`: Runs only tests that match the specified name.

---

### Summary for C/C++ Developers
- **In C/C++**: You often use frameworks like GTest or Catch2. You have to set up your build system (CMake/Make) to compile and run them.
- **In Rust**: Testing is a first-class citizen. `cargo test` handles everything, and the syntax is part of the language itself.

***
