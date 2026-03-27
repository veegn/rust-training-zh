# Memory Safety: Compile-Time vs Runtime

> **What you'll learn:** Rust references vs C# pointers, lifetime basics, and why compile-time safety proofs are stronger than C#'s runtime checks.
>
> **Difficulty:** Intermediate

## References vs Pointers
In C#, you rarely use pointers unless you are in an `unsafe` block. In Rust, you use references (`&`) everywhere, and they are **safe by default**.

### C# Unsafe Pointer
```csharp
unsafe {
    int value = 42;
    int* ptr = &value;
    *ptr = 100;
}
```

### Rust Safe Reference
```rust
let mut value = 42;
let r = &mut value; // No 'unsafe' keyword needed
*r = 100;
```
The **Borrow Checker** ensures that `r` never points to invalid memory, so you get the performance of pointers with the safety of high-level references.

---

## Lifetime Basics
A **lifetime** is a construct the compiler uses to ensure all borrows are valid for as long as they are used.

### The Dangling Reference Problem
In C#, returning a pointer to a local variable is a recipe for disaster. Rust's compiler catches this immediately.
```rust
fn invalid_reference() -> &String {
    let s = String::from("hello");
    &s // ❌ ERROR: `s` does not live long enough
}
```
In this example, `s` is dropped at the end of the function, so the reference would be "dangling" (pointing to garbage). Rust prevents you from even compiling this.

---

## Runtime Checks vs Compile-Time Proofs

| **Feature** | **C# (Runtime)** | **Rust (Compile-Time)** |
| :--- | :--- | :--- |
| **Bounds Check** | Throws `IndexOutOfRange` | Proven safe or Panic |
| **Null Access** | Throws `NullReference` | Impossible (No Null) |
| **Data Races** | Possible (lock/mutex) | **Impossible** (Borrow Rules) |
| **Memory Leaks** | Rare (GC handles) | Prevented (Ownership) |

### Use-After-Free
In C#, you might dispose of a resource (like a `FileStream`) and then accidentally try to use it again, causing an `ObjectDisposedException`. In Rust, the ownership system makes this a compile error.

```rust
// C#
var stream = new FileStream(...);
stream.Dispose();
stream.Write(...); // ❌ Runtime Exception

// Rust
let file = File::open(...)?;
drop(file);
// file.write(...); // ❌ Compile Error: value used after move
```

---

## Exercise: Spot the Safety Bug
**Challenge:** Identify why modifying a collection while iterating is dangerous in C# and how Rust prevents it.

```rust
fn filter_evens(numbers: &mut Vec<i32>) {
    for n in numbers.iter() {
        if n % 2 == 0 {
            // numbers.remove(...); // ❌ ERROR
        }
    }
}
```
**Takeaway:** In C#, modifying a list during a `foreach` loop triggers an `InvalidOperationException` at runtime. In Rust, the iterator holds an immutable borrow on the vector, so any attempt to mutate it (like `remove`) is a compile-time error.
