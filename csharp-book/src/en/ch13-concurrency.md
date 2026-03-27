# Concurrency: Safe Multi-Threading

> **What you'll learn:** How Rust enforces thread safety at compile time vs C#'s convention-based approach, `Arc<Mutex<T>>` vs `lock`, and `Send`/`Sync` traits.
>
> **Difficulty:** Advanced

In C#, you can share any object between threads, and it's your job to use `lock` correctly. In Rust, the compiler **guarantees** thread safety. If your code might cause a data race, it simply won't compile.

---

## Shared State: `Arc` and `Mutex`
In Rust, you can't just share a plain variable between threads because the compiler can't prove it's safe. You need two components:
1.  **`Arc<T>` (Atomic Reference Counter)**: Allows multiple threads to own the same data.
2.  **`Mutex<T>` (Mutual Exclusion)**: Ensures only one thread can modify the data at a time.

### Rust Example
```rust
use std::sync::{Arc, Mutex};
use std::thread;

let counter = Arc::new(Mutex::new(0));
let mut handles = vec![];

for _ in 0..10 {
    let counter = Arc::clone(&counter);
    let handle = thread::spawn(move || {
        let mut num = counter.lock().unwrap();
        *num += 1;
    });
    handles.push(handle);
}

for handle in handles { handle.join().unwrap(); }
println!("Result: {}", *counter.lock().unwrap());
```

---

## `Send` and `Sync`: The Magic Traits
Rust uses two special traits to track thread safety:
*   **`Send`**: This type can be **transferred** to another thread.
*   **`Sync`**: This type can be safely **shared** between threads (via references).

**The Killer Feature:** Most types are automatically `Send` and `Sync`. If you try to use a non-thread-safe type (like `Rc`) in a thread, the compiler will give you an error. In C#, this would be a silent runtime bug.

---

## Message Passing (Channels)
A common philosophy in Rust is: *"Do not communicate by sharing memory; instead, share memory by communicating."*

```rust
use std::sync::mpsc; // Multi-producer, single-consumer
let (tx, rx) = mpsc::channel();

thread::spawn(move || {
    tx.send("Hello from thread").unwrap();
});

let msg = rx.recv().unwrap();
```

---

## Summary for C# Developers
| **Concept** | **C# Approach** | **Rust Approach** |
| :--- | :--- | :--- |
| **Simple Lock** | `lock (obj) { ... }` | `MutexGuard` (RAII) |
| **Shared Ref** | Just pass the object | `Arc<T>` |
| **Thread-Local** | `[ThreadStatic]` | `thread_local!` macro |
| **Concurrently** | `Parallel.ForEach` | `rayon` crate |
| **Atomic Ops** | `Interlocked` class | `std::sync::atomic` |

---

## Exercise: Parallel Processing
**Challenge:** Use the `rayon` crate (the standard for data-parallelism) to square all numbers in a vector in parallel.

```rust
use rayon::prelude::*;

let mut nums = vec![1, 2, 3, 4, 5];
let squares: Vec<_> = nums.par_iter().map(|&x| x * x).collect();
```
**Takeaway:** Fearless concurrency is a reality in Rust. The compiler acts as a pair programmer, catching subtle race conditions before they ever reach production.
