# Async/Await: C# Task vs Rust Future

> **What you'll learn:** Rust's lazy `Future` vs C#'s eager `Task`, the executor model (Tokio), and cancellation via `Drop`.
>
> **Difficulty:** Advanced

C# developers are deeply familiar with `async`/`await`. Rust uses the same keywords but with a fundamentally different execution model.

---

## The Executor Model
In C#, the .NET runtime provides a built-in thread pool and task scheduler. You just `await`, and it "just works."

In Rust, **the standard library has no async runtime.** You must choose a library to provide the "executor" (the code that actually runs the tasks). **Tokio** is the industry standard.

```rust
#[tokio::main] // This macro sets up the Tokio runtime
async fn main() {
    let result = fetch_data().await;
}
```

---

## Future vs Task
The biggest difference is **Laziness**.
*   **C# `Task`**: Starts running as soon as it's created.
*   **Rust `Future`**: Does **nothing** until it is `.await`ed.

```rust
// C# - Starts immediately
var task = DoWorkAsync(); 

// Rust - Nothing happens yet!
let future = do_work_async(); 
// Now it starts:
let result = future.await; 
```

---

## Cancellation: No Tokens Needed
In C#, you pass a `CancellationToken` around to cancel tasks. In Rust, cancellation is built into the ownership system. If you **Drop** a Future (e.g., it goes out of scope or you stop awaiting it), it simply stops running immediately.

```rust
tokio::select! {
    val = active_task() => println!("Task finished: {}", val),
    _ = sleep(Duration::from_secs(5)) => println!("Timed out!"),
}
// If 'sleep' finishes first, 'active_task' is DROPPED and cancelled automatically.
```

---

## Summary for C# Developers
| **Concept** | **C# / .NET** | **Rust / Tokio** |
| :--- | :--- | :--- |
| **Type** | `Task<T>` | `impl Future<Output = T>` |
| **Execution** | Eager (Starts now) | Lazy (Starts on `.await`) |
| **Runtime** | Built-in (Thread Pool) | Library (e.g., Tokio) |
| **Cancellation** | `CancellationToken` | `Drop` the Future |
| **Multiple Tasks** | `Task.WhenAll` | `tokio::join!` |
| **Race Tasks** | `Task.WhenAny` | `tokio::select!` |

---

## Exercise: Concurrent Requests
**Challenge:** Use `tokio::join!` to fetch two different values concurrently and print their sum.

```rust
async fn get_a() -> i32 { 10 }
async fn get_b() -> i32 { 20 }

#[tokio::main]
async fn main() {
    let (a, b) = tokio::join!(get_a(), get_b());
    println!("Sum is {}", a + b);
}
```
**Takeaway:** Rust's async model is pull-based and lazy, which makes it incredibly efficient. You don't pay for what you don't use, and cancellation is as simple as "letting go" of a value.
