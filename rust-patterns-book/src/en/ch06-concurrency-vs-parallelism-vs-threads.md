# 6. Concurrency vs Parallelism vs Threads 🟡

> **What you'll learn:**
> - The distinction between concurrency and parallelism
> - OS threads, scoped threads, and rayon for data parallelism
> - Shared state primitives: Arc, Mutex, RwLock, Atomics, Condvar
> - Lazy initialization with OnceLock/LazyLock

## Terminology: Concurrency ≠ Parallelism

| | Concurrency | Parallelism |
|---|---|---|
| **Definition** | Managing multiple tasks that can make progress | Executing multiple tasks simultaneously |
| **Hardware** | One core is enough | Requires multiple cores |
| **Analogy** | One cook, multiple dishes (switching) | Multiple cooks, each working on a dish |

## std::thread — OS Threads

Rust threads map 1:1 to OS threads.

```rust
use std::thread;

fn main() {
    let handle = thread::spawn(|| {
        println!("Spawned thread");
        42
    });

    println!("Main thread");
    let result = handle.join().unwrap();
}
```

### Scoped Threads (std::thread::scope)

Scoped threads allow you to borrow local data without needing an `Arc`. The compiler ensures all threads finish before the scope ends.

```rust
let data = vec![1, 2, 3];
thread::scope(|s| {
    s.spawn(|| println!("Thread 1: {:?}", data));
    s.spawn(|| println!("Thread 2: {:?}", data));
}); // Guaranteed to join here
```

---

## rayon — Data Parallelism

For data-intensive tasks, use `rayon`. It automatically distributes work across a thread pool.

```rust
use rayon::prelude::*;

let sum: u64 = data.par_iter() // Parallel iterator
    .map(|x| x * x)
    .sum();
```

---

## Shared State Primitives

| Primitive | Use Case |
|-----------|----------|
| `Mutex<T>` | Exclusive access (lock/unlock). |
| `RwLock<T>` | Multiple readers OR one writer. |
| `AtomicU64` | Lock-free counters and flags (hardware level). |
| `OnceLock<T>` | One-time lazy initialization (safe for globals). |

### Lazy Initialization

```rust
static CONFIG: LazyLock<Config> = LazyLock::new(|| load_config());

fn main() {
    let cfg = &*CONFIG; // Initialized on first access
}
```

***
