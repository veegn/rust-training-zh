# 19. Macros 🟢

Macros in Rust are a powerful feature that allow you to write code that writes other code, which is known as metaprogramming. Rust has two main types of macros: **declarative macros** (defined with `macro_rules!`) and **procedural macros**.

### 1. Declarative Macros with `macro_rules!`
Declarative macros are the most common type of macro in Rust. They use pattern matching to transform a piece of code into another piece of code.

```rust
macro_rules! say_hello {
    () => {
        println!("Hello!");
    };
}

fn main() {
    say_hello!(); // Swaps with println!("Hello!") at compile time
}
```

---

### 2. Macros with Arguments
You can define macros that take arguments, allowing you to create more flexible and reusable patterns.

```rust
macro_rules! create_vec {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

fn main() {
    let v = create_vec![1, 2, 3];
    println!("{:?}", v);
}
```

---

### 3. Procedural Macros
Procedural macros are more complex and act more like functions. They take a stream of tokens as input, operate on that stream, and produce a stream of tokens as output. There are three types of procedural macros:
- **Custom `#[derive]` macros**: Specify code that is added with the `derive` attribute.
- **Attribute-like macros**: Define custom attributes that can be used on any item.
- **Function-like macros**: Look like function calls but operate on tokens rather than values.

---

### 4. Macros vs. Functions
- Use **functions** for standard logic that doesn't require manipulating the structure of the code itself.
- Use **macros** for tasks that functions cannot perform, such as:
    - Creating variadic interfaces (like `println!`).
    - Reducing boilerplate code (like `#[derive(Debug)]`).
    - Defining domain-specific languages (DSLs).

---

### Summary for C/C++ Developers
- **In C/C++**: Macros are handled by the preprocessor and are essentially simple text substitution. This can lead to many subtle bugs (e.g., missing parentheses in a macro definition).
- **In Rust**: Macros are part of the compiler and work on the abstract syntax tree (AST) or token streams. This makes them much safer and more powerful than C/C++ preprocessor macros. Rust macros are "hygienic," meaning they don't accidentally capture variables from the scope where they are called.

***
