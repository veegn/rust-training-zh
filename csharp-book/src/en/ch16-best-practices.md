# Best Practices for C# Developers

> **What you'll learn:** Five critical mindset shifts, idiomatic project organization, and common mistakes to avoid.
>
> **Difficulty:** Intermediate

Transitioning from C# to Rust isn't just about syntax—it's about how you approach problems. This chapter outlines the best practices that will help you write "idiomatic" Rust.

---

## 1. The Mindset Shift
| **From (C#)** | **To (Rust)** | **Why?** |
| :--- | :--- | :--- |
| **Garbage Collection** | **Ownership/Borrowing** | Predictive performance & memory safety without a GC. |
| **Exceptions** | **Result Types** | Explicit error handling makes code more robust. |
| **Class Inheritance** | **Trait Composition** | Composition is more flexible and avoids deep hierarchies. |
| **Nullable Types** | **Option Types** | Eliminates the "Billion Dollar Mistake" of null references. |

---

## 2. Project Organization
Structure your Rust projects to feel familiar but idiomatic.
*   **`main.rs`**: Your entry point (like `Program.cs`).
*   **`lib.rs`**: Your library logic (like a separate Class Library project).
*   **`models/`**: For your data structures (POCOs).
*   **`services/`**: For your business logic.
*   **`tests/`**: For integration tests.

---

## 3. Error Handling Strategy
Don't use `unwrap()` in production code. Use the **Question Mark Operator (`?`)** to propagate errors upward.

```rust
// GOOD: Propagating errors
pub fn load_config() -> Result<Config, io::Error> {
    let content = fs::read_to_string("config.json")?; // Returns Err immediately if file missing
    let config = serde_json::from_str(&content)?;
    Ok(config)
}
```

---

## 4. Avoid Common C# Pitfalls
### 1. Stop Cloning Everything
In C#, everything is a reference, so "cloning" (copying the reference) is cheap. In Rust, `.clone()` creates a full deep copy of the data.
*   **Bad**: `process_data(my_string.clone())`
*   **Good**: `process_data(&my_string)`

### 2. Don't Fight the Borrow Checker
If you find yourself stuck on a borrow checker error, it's often a sign that your **data ownership model** is wrong.
*   **Tip**: Instead of trying to have multiple parts of your app "own" the same object, use an **Owner/Worker** model or use `Arc` if sharing across threads is required.

---

## Summary for C# Developers
*   **Be Explicit**: Rust prefers explicit code over hidden "magic" (like implicit GC or background exceptions).
*   **Embrace the Compiler**: The compiler is your friend. If it's complaining, it's trying to save you from a runtime crash.
*   **Traits > Classes**: Use traits to define shared behavior, not base classes.

---

## Exercise: Refactor a Code Block
**Challenge:** Take a piece of code that uses `unwrap()` and refactor it to use `Result` and the `?` operator.

**Takeaway:** Idiomatic Rust code is safe, predictable, and incredibly fast. By following these best practices, you'll move from "writing C# in Rust" to writing true, high-performance Rust.
