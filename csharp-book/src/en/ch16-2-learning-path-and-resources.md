# Learning Path and Resources

> **What you'll learn:** A structured learning roadmap for your first 90 days, recommended books, and how to avoid the "Borrow Checker Wall."
>
> **Difficulty:** Beginner

Every C# developer follows a predictable path when learning Rust. You start with excitement, hit the "Borrow Checker Wall" around week 3, and then—if you persist—everything clicks around month 3. This chapter is your roadmap.

---

## 1. The 90-Day Roadmap

### Weeks 1-2: Foundations
*   **Goal**: Get code to compile.
*   **Focus**: Basic syntax, Structs, Enums, and simple Ownership (passing data by reference vs. value).
*   **Tasks**: Complete the first 20 [Rustlings](https://github.com/rust-lang/rustlings) exercises.

### Month 1: The Standard Library
*   **Goal**: Handle data efficiently.
*   **Focus**: `Vec`, `HashMap`, and the **Iterator** pattern (Rust's version of LINQ).
*   **Tasks**: Build a small CLI tool that reads a file and counts word frequencies.

### Month 2: Traits and Generics
*   **Goal**: Write reusable code.
*   **Focus**: Implementing common traits (`Debug`, `Clone`, `Default`) and creating your own traits (Interfaces).
*   **Tasks**: Refactor your CLI tool to use a `Trait` for data input (e.g., File vs. Memory).

### Month 3: Async and Ecosystem
*   **Goal**: Build production-ready services.
*   **Focus**: `Tokio` for async/await, `Serde` for JSON, and `Axum` for web APIs.
*   **Tasks**: Build a small REST API that connects to a database using `SQLx`.

---

## 2. Recommended Resources
| **Resource** | **Type** | **Why?** |
| :--- | :--- | :--- |
| **"The Book"** | Free Online | The official Rust Bible. Read it cover to cover. |
| **Rustlings** | Interactive | Small exercises that fix compiler errors. |
| **Rust by Example** | Reference | Great for "How do I do X in Rust?" |
| **Programming Rust** | Paid Book | Best for deep-dives into memory management. |

---

## 3. Top Tips for C# Developers
1.  **Read the Errors**: Rust compiler errors are literal instructions on how to fix your code. Read them carefully!
2.  **Don't skip Ownership**: It's tempting to use `.clone()` everywhere to make the error go away. Don't. Stop and understand *why* the loan was rejected.
3.  **Use `rust-analyzer`**: This VS Code extension is non-negotiable. It's like having ReSharper/IntelliCode for Rust.
4.  **Join the Community**: The Rust Discord and Forums are incredibly welcoming to newcomers.

---

## Summary for C# Developers
*   **It's a marathon, not a sprint**: Rust has a steeper learning curve than C#, but the payoff in code quality and performance is immense.
*   **Focus on one thing at a time**: Don't try to learn `Unsafe` or `Macros` in your first month. Stick to the basics.
*   **Learn by doing**: Rust is a "handheld" language. You won't learn it by just reading; you have to write code and let the compiler teach you.

---

## Exercise: Start Your Journey
**Challenge:** Install Rust using `rustup`, install the `rust-analyzer` extension in your IDE, and run `cargo new my_first_app`. Congratulations, you're officially a Rustacean!

**Takeaway:** Every expert was once a beginner who refused to quit when the borrow checker got tough. Stick with it!
