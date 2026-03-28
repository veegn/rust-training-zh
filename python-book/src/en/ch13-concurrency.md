# 13. Concurrency 🔴

> **What you'll learn:**
> - Why Rust has NO GIL (Global Interpreter Lock)
> - True parallelism vs Python's threading limitations
> - Thread safety via `Arc<Mutex<T>>`
> - Async/Await: Python's `asyncio` vs Rust's `tokio`

## No GIL: True Parallelism

Python's Global Interpreter Lock (GIL) is its biggest bottleneck for CPU-bound tasks. Since only one thread can execute Python bytecode at a time, multi-threading in Python doesn't actually speed up CPU tasks.

**Rust has no GIL.** Threads run truly in parallel, and the type system ensures you don't have data races (two threads writing to the same memory at once).

### Python: GIL Bottleneck
```python
import threading

def cpu_bound():
    sum(range(10_000_000))

# Spawning 4 threads will NOT be 4x faster due to the GIL.
threads = [threading.Thread(target=cpu_bound) for _ in range(4)]
```

### Rust: Fearless Parallelism
```rust
use std::thread;

fn cpu_bound() {
    (0..10_000_000).sum::<u64>();
}

fn main() {
    let handles: Vec<_> = (0..4).map(|_| {
        thread::spawn(|| cpu_bound())
    }).collect();

    for h in handles { h.join().unwrap(); }
    // Truly runs on 4 different CPU cores simultaneously!
}
```

---

## Shared State: Arc and Mutex

To share data between threads, you need a way to **protect** it and **reference-count** it across threads.

- **`Mutex<T>`**: Ensures only one thread can access the data at a time (Mutual Exclusion).
- **`Arc<T>`**: Atomic Reference Counting. Allows multiple threads to "own" a reference to the same data.

```rust
use std::sync::{Arc, Mutex};
use std::thread;

let counter = Arc::new(Mutex::new(0));
let mut handles = vec![];

for _ in 0..10 {
    let counter = Arc::clone(&counter);
    handles.push(thread::spawn(move || {
        let mut num = counter.lock().unwrap(); // Lock is required!
        *num += 1;
    }));
}
```

---

## Async/Await: I/O Bound Work

Both languages use `async/await`, but they differ:
- **Python's `asyncio`**: Single-threaded event loop. If you run a CPU-bound task, you block the whole loop.
- **Rust's `tokio`**: Multi-threaded runtime. `tokio::spawn` can run tasks on any available CPU core.

### Quick Mapping:
| Python | Rust (Tokio) |
|--------|--------------|
| `asyncio.run(main())` | `#[tokio::main] async fn main()` |
| `asyncio.gather(*tasks)` | `futures::future::join_all(tasks).await` |
| `asyncio.sleep(1)` | `tokio::time::sleep(Duration::from_secs(1)).await` |

---

## Exercises

<details>
<summary><strong>🏋️ Exercise: Rayon Parallel Map</strong></summary>

**Challenge**: In Python, you'd use `multiprocessing.Pool` for parallel mapping. In Rust, you can use the `rayon` crate. Write a snippet using Rayon's `.par_iter()` to square a vector of numbers in parallel.

<details>
<summary>🔑 Solution</summary>

```rust
use rayon::prelude::*; // You'll need the rayon crate

fn main() {
    let mut nums = vec![1, 2, 3, 4, 5, 6];
    let squares: Vec<_> = nums.par_iter().map(|&x| x * x).collect();
    println!("{squares:?}");
}
```

</details>
</details>

***
