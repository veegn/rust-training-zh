# 9. Error Handling 🟢

Rust groups errors into two major categories: **recoverable** and **unrecoverable** errors.

### 1. Unrecoverable Errors with `panic!`
For situations where the program cannot continue (e.g., bug, resource exhausted), Rust has the `panic!` macro.

```rust
fn main() {
    panic!("crash and burn");
}
```

When a panic occurs, the program prints a failure message, unwinds the stack, and quits. Common causes:
- Accessing an array out of bounds.
- Calling `.unwrap()` on a `None` or `Err`.

---

### 2. Recoverable Errors with `Result<T, E>`
Most errors aren't serious enough to require the program to stop. The `Result` enum represents success (`Ok`) or failure (`Err`).

```rust
use std::fs::File;

fn main() {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {error:?}"),
    };
}
```

---

### 3. Propagating Errors with the `?` Operator
Instead of handling the error immediately, you can return it to the caller using the `?` operator.

```rust
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();
    // If File::open fails, it returns the error immediately
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}
```

---

### 4. `Option<T>` for Nullable Values
In Rust, there are no null pointers. Instead, you use `Option<T>` to represent a value that might be absent.

```rust
fn find_word(text: &str, word: &str) -> Option<usize> {
    text.find(word)
}

fn main() {
    let index = find_word("hello world", "world");
    
    match index {
        Some(i) => println!("Found at index {i}"),
        None => println!("Not found"),
    }
}
```

---

### 5. `unwrap` and `expect`
These methods are shortcuts that either return the value or `panic!`.
- **`unwrap()`**: Returns the value or panics with a generic message.
- **`expect("msg")`**: Returns the value or panics with a custom message (recommended).

```rust
let f = File::open("hello.txt").expect("hello.txt should be included in this project");
```

---

### Summary for C/C++ Developers
- **In C/C++**: You check return codes (`if (ret != 0)`) or use `try/catch` for exceptions. It's easy to forget to check a return code.
- **In Rust**: You **must** handle the `Result` or `Option` types. The compiler will warn you if you ignore a returned `Result`. The `?` operator provides the convenience of exceptions without the hidden control flow.

***
