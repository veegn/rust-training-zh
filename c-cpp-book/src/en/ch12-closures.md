# 12. Closures 🟢

Rust’s **closures** are anonymous functions you can save in a variable or pass as arguments to other functions. You can create the closure in one place and then call the closure to evaluate it in a different context.

### 1. Basic Syntax
Closures use vertical bars `||` instead of parentheses `()` for parameters.

```rust
fn main() {
    let add_one = |x: i32| x + 1;
    let result = add_one(5);
    println!("The result is {result}");
}
```

---

### 2. Capturing the Environment
Unlike functions, closures can capture values from the scope in which they’re defined.

```rust
fn main() {
    let x = 4;
    let equal_to_x = |z| z == x; // Captures `x`

    let y = 4;
    assert!(equal_to_x(y));
}
```

---

### 3. Closure Traits: `Fn`, `FnMut`, and `FnOnce`
Closures capture values from their environment in three ways, which map to the three ways a function can take a parameter:
- **`FnOnce`**: Consumes the variables it captures from its enclosing scope (can only be called once).
- **`FnMut`**: Mutably borrows values from its environment (can change the environment).
- **`Fn`**: Immutably borrows values from its environment.

---

### 4. Moving Ownership with `move`
If you want to force the closure to take ownership of the values it uses in the environment, you can use the `move` keyword before the parameter list.

```rust
use std::thread;

fn main() {
    let x = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", x);
    });

    handle.join().unwrap();
}
```

---

### Summary for C/C++ Developers
- **In C++**: You use **Lambdas** (`[=](int x) { ... }`). You must manually specify how to capture variables (by value `[=]`, by reference `[&]`, etc.).
- **In Rust**: Closures automatically infer which capture trait to use based on how the environment variables are used. The `move` keyword is used for moving ownership into the closure, similar to `[x = std::move(x)]` in C++.

***
