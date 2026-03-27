# Understanding Ownership

> **What you'll learn:** Rust's ownership system - why `let s2 = s1` invalidates `s1` (unlike C# reference copying), the three ownership rules, `Copy` vs `Move` types, borrowing with `&` and `&mut`, and how the borrow checker replaces garbage collection.
>
> **Difficulty:** Intermediate

Ownership is Rust's most unique feature and the biggest conceptual shift for C# developers.

---

## C# vs Rust Memory Models

### C# Memory Model
In C#, objects (classes) live on the heap. When you assign one variable to another, you copy the reference. Both variables point to the same object.
```csharp
var original = new List<int> { 1, 2, 3 };
var reference = original; // Both point to the same list
original.Add(4);
Console.WriteLine(reference.Count); // 4 - same object
```

### Rust Ownership Model
In Rust, every value has a single owner. When you assign a value to a new variable, ownership is **moved** by default.
```rust
let original = vec![1, 2, 3];
let moved = original; // Ownership transferred to 'moved'
// println!("{:?}", original); // ❌ Compile error: original no longer owns the data
```

---

## The Three Ownership Rules
1.  **Each value has exactly one owner.**
2.  **When the owner goes out of scope, the value is dropped.** (Deterministic cleanup)
3.  **Ownership can be transferred (moved).**

---

## Copy Types vs Move Types
*   **Copy Types**: Simple values like integers (`i32`), booleans (`bool`), and floats (`f64`). These are copied instead of moved because they are cheap to duplicate on the stack.
*   **Move Types**: Complex types that manage heap memory like `String` or `Vec<T>`. These are moved to prevent double-free errors.

---

## Borrowing Basics
Borrowing allows you to access data without taking ownership.

### Immutable Borrowing (`&`)
You can have as many immutable references as you want.
```rust
fn read_value(value: &Vec<i32>) {
    println!("Length: {}", value.len());
}
```

### Mutable Borrowing (`&mut`)
You can have **exactly one** mutable reference at a time. This prevents data races at compile time.
```rust
fn modify_value(value: &mut Vec<i32>) {
    value.push(42);
}
```

---

## The Borrow Checker Rules
1.  **Multiple immutable borrows** are OK.
2.  **Only one mutable borrow** at a time.
3.  **No immutable borrows allowed while a mutable borrow exists.**

---

## RAII vs Garbage Collection
In C#, the Garbage Collector (GC) periodically scans memory to find unused objects. In Rust, memory is cleaned up the moment the owner goes out of scope. This is called **RAII** (Resource Acquisition Is Initialization).

```rust
{
    let file = std::fs::File::open("data.txt")?;
    // File is automatically closed when 'file' goes out of scope.
    // No 'using' statement or manual 'Dispose()' needed.
}
```
**Core Insight:** Rust provides the automation of a GC with the performance and predictability of manual memory management.
