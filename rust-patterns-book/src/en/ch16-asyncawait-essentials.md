# 16. Async/Await Essentials 🔶

> **What you'll learn:**
> - How `Future` differs from goroutines.
> - Tokio quick-start: spawning and joining tasks.
> - Common async pitfalls and manual polling.
> - `spawn_blocking` for CPU-heavy work.

## Futures and Polling

- **Futures are lazy**: Calling an `async fn` only returns a state machine. Nothing is executed until you `.await` the future or spawn it onto a runtime.
- **Runtimes required**: The standard library provides the `Future` trait, but you need a crate like `Tokio` to actually run them.

```rust
async fn fetch_data() -> String {
    // .await yields control back to the executor
    let res = reqwest::get("url").await.unwrap();
    res.text().await.unwrap()
}
```

---

## Tokio Quick Start

```rust
#[tokio::main]
async fn main() {
    let handle = tokio::spawn(async {
        // Runs concurrently in the background
        "done"
    });

    // Wait for the spawned task
    let res = handle.await.unwrap();
}
```

### Concurrent Control

| Macro | Effect |
|-------|--------|
| `tokio::join!` | Waits for multiple futures to complete. |
| `tokio::select!` | Responds to the FIRST future that completes. |
| `tokio::time::timeout` | Fails if the future doesn't complete within a duration. |

---

## Common Pitfalls

### 1. Blocking the Executor
**DON'T** use `std::thread::sleep` or long CPU-bound loops in `async` blocks. This stops the executor from running other tasks.
**DO** use `tokio::task::spawn_blocking`.

```rust
let res = task::spawn_blocking(|| {
    // Heavy CPU or blocking I/O goes here
    std::thread::sleep(Duration::from_secs(5));
}).await.unwrap();
```

### 2. Holding MutexGuards across `.await`
**DON'T** hold `std::sync::MutexGuard` while calling `.await`. It's not `Send`, which will cause compilation errors.
**DO** use `tokio::sync::Mutex` or restructure to release the guard before the `.await` point.

---

## Send Bounds

Futures spawned with `tokio::spawn` must be `Send`. All variables captured or held across an `.await` point must also be `Send`.

***
