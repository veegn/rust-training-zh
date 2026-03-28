# 17. Rust Best Practices 🟢

Adopting best practices early on will help you write more idiomatic, maintainable, and efficient Rust code. This chapter summarizes key patterns and guidelines for C/C++ developers transitioning to Rust.

### 1. Error Handling: `Result` over Exceptions
Rust does not have exceptions. Instead, it uses the `Result<T, E>` type for operations that might fail. Use the `?` operator to propagate errors up the call stack gracefully.

```rust
use std::fs::File;
use std::io::{self, Read};

fn read_file_content(path: &str) -> io::Result<String> {
    let mut file = File::open(path)?; // Propagates error if file open fails
    let mut content = String::new();
    file.read_to_string(&mut content)?; // Propagates error if read fails
    Ok(content)
}
```

---

### 2. Composition over Inheritance
Rust does not support class-based inheritance. Use traits to define shared behavior and composition to build complex types.

```rust
trait Drawable {
    fn draw(&self);
}

struct Circle {
    radius: f64,
}

impl Drawable for Circle {
    fn draw(&self) {
        println!("Drawing a circle with radius {}", self.radius);
    }
}
```

---

### 3. Iterators and Functional Patterns
Rust's iterators are powerful and efficient. Use them instead of manual loops whenever possible to make your code more expressive and less error-prone.

```rust
fn main() {
    let numbers = vec![1, 2, 3, 4, 5];
    let sum: i32 = numbers.iter().filter(|&&x| x % 2 == 0).map(|&x| x * x).sum();
    println!("Sum of squares of even numbers: {}", sum);
}
```

---

### 4. Documentation and Testing
Rust has first-class support for documentation and testing. Use `///` for doc comments and place your tests in the same file or a `tests/` directory.

```rust
/// Adds two numbers together.
/// 
/// # Examples
/// 
/// ```
/// let result = my_library::add(2, 2);
/// assert_eq!(result, 4);
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(add(2, 2), 4);
    }
}
```

---

### Summary for C/C++ Developers
- **In C/C++**: Best practices are often tribal knowledge or enforced by external linting tools.
- **In Rust**: Many best practices are built directly into the language, compiler, and standard tools (`cargo fmt`, `cargo clippy`). Adhering to these conventions will make your code feel "idiomatic" and easier for other Rustacean to read and contribute to.

***
