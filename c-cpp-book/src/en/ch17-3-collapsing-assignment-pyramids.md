# 17.3 Collapsing Assignment Pyramids 🟢

In languages like C++, you often find yourself writing deeply nested `if-else` or `switch` statements to assign a value to a variable based on multiple conditions. This is sometimes called an "assignment pyramid." Rust's expression-based syntax allows you to collapse these pyramids into much cleaner and more readable code.

### 1. Variables as the Result of an Expression
In Rust, almost everything is an expression that returns a value. This means you can use `if`, `match`, and even blocks (`{}`) to assign values directly to variables.

```rust
fn main() {
    let score = 85;

    // GOOD: use an if expression for assignment
    let grade = if score >= 90 {
        "A"
    } else if score >= 80 {
        "B"
    } else if score >= 70 {
        "C"
    } else {
        "F"
    };

    println!("Grade: {}", grade);
}
```

---

### 2. Collapsing with `match`
The `match` expression is even more powerful for collapsing complex conditional logic, especially when dealing with enums or multiple variables.

```rust
enum Status {
    Success,
    Warning(u32),
    Error(String),
}

fn main() {
    let status = Status::Warning(404);

    // GOOD: use match to extract values and assign
    let message = match status {
        Status::Success => String::from("Operation successful"),
        Status::Warning(code) => format!("Warning with code: {}", code),
        Status::Error(err) => format!("Error occurred: {}", err),
    };

    println!("{}", message);
}
```

---

### 3. Using Closures and `?` for Optional Chains
If you have a sequence of operations that might fail (returning `Option` or `Result`), you can use closures and the `?` operator to avoid nested `if let` or `match` statements.

```rust
fn get_user_id() -> Option<u32> { Some(123) }
fn get_user_name(id: u32) -> Option<String> { Some(String::from("Alice")) }

fn main() {
    // GOOD: collapse the chain using and_then
    let name = get_user_id()
        .and_then(|id| get_user_name(id))
        .unwrap_or_else(|| String::from("Unknown"));

    println!("User name: {}", name);
}
```

---

### 4. Expression Blocks
You can use a block of code `{}` as an expression. This is useful for complex initializations that require temporary variables.

```rust
fn main() {
    let config_val = {
        let temp = 10 * 2;
        let offset = 5;
        temp + offset // The last expression in the block is the return value
    };

    println!("Config value: {}", config_val); // 25
}
```

---

### Summary for C/C++ Developers
- **In C++**: You might use the ternary operator (`condition ? a : b`) for simple assignments, but for anything more complex, you're stuck with multiple `if-else` blocks and potentially uninitialized variables.
- **In Rust**: Expression-based syntax makes your code more declarative. It also helps the compiler ensure that variables are always initialized, eliminating a common class of bugs found in C/C++.

***
