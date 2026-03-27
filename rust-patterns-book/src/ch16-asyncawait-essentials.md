# 16. Async/Await Essentials / 16. Async/Await 核心要点 🔶

> **What you'll learn / 你将学到：**
> - How Rust's `Future` trait differs from Go's goroutines and Python's asyncio / Rust 的 `Future` trait 与 Go 的 goroutine 以及 Python 的 asyncio 有何不同
> - Tokio quick-start: spawning tasks, `join!`, and runtime configuration / Tokio 快速上手：生成任务、`join!` 以及运行时的配置
> - Common async pitfalls and how to fix them / 常见的异步陷阱及其解决方法
> - When to offload blocking work with `spawn_blocking` / 何时使用 `spawn_blocking` 来卸载阻塞性工作

## Futures, Runtimes, and `async fn` / Future、运行时与 `async fn`

Rust's async model is *fundamentally different* from Go's goroutines or Python's `asyncio`. Understanding three concepts is enough to get started:

Rust 的异步模型与 Go 的 goroutine 或 Python 的 `asyncio` 有着 *本质上的不同*。了解以下三个概念就足以入门：

1. **A `Future` is a lazy state machine / `Future` 是一个惰性状态机** — calling `async fn` doesn't execute anything; it returns a `Future` that must be polled.
   调用 `async fn` 不会执行任何操作；它会返回一个必须被轮询（poll）的 `Future`。
2. **You need a runtime / 你需要一个运行时** to poll futures — `tokio`, `async-std`, or `smol`. The standard library defines `Future` but provides no runtime.
   来轮询 future —— 例如 `tokio`、`async-std` 或 `smol`。标准库定义了 `Future` 但不提供运行时。
3. **`async fn` is sugar / `async fn` 是语法糖** — the compiler transforms it into a state machine that implements `Future`.
   编译器会将其转换为实现 `Future` 的状态机。

```rust
// A Future is just a trait:
// Future 只是一个 trait：
pub trait Future {
    type Output;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
}

// async fn desugars to:
// async fn 会脱糖（desugar）为：
// fn fetch_data(url: &str) -> impl Future<Output = Result<Vec<u8>, Error>>
async fn fetch_data(url: &str) -> Result<Vec<u8>, reqwest::Error> {
    let response = reqwest::get(url).await?;  // .await yields until ready
                                              // .await 会在未就绪时让出控制权
    let bytes = response.bytes().await?;
    Ok(bytes.to_vec())
}
```

### Tokio Quick Start / Tokio 快速上手

```toml
# Cargo.toml
[dependencies]
tokio = { version = "1", features = ["full"] }
```

```rust,ignore
use tokio::time::{sleep, Duration};
use tokio::task;

#[tokio::main]
async fn main() {
    // Spawn concurrent tasks (like lightweight threads):
    // 生成并发任务（就像轻量级线程）：
    let handle_a = task::spawn(async {
        sleep(Duration::from_millis(100)).await;
        "task A done"
    });

    let handle_b = task::spawn(async {
        sleep(Duration::from_millis(50)).await;
        "task B done"
    });

    // .await both — they run concurrently, not sequentially:
    // 等待（.await）两者 —— 它们是并发运行的，而不是顺序运行：
    let (a, b) = tokio::join!(handle_a, handle_b);
    println!("{}, {}", a.unwrap(), b.unwrap());
}
```

### Async Common Pitfalls / 异步常见陷阱

| Pitfall / 陷阱 | Why It Happens / 原因 | Fix / 解决方法 |
|---------|---------------|-----|
| Blocking in async / 在异步中阻塞 | `std::thread::sleep` or CPU work blocks the executor / `std::thread::sleep` 或 CPU 运算阻塞了执行器 | Use `tokio::task::spawn_blocking` or `rayon` / 使用 `tokio::task::spawn_blocking` 或 `rayon` |
| `Send` bound errors / `Send` 约束错误 | Future held across `.await` contains `!Send` type (e.g., `Rc`, `MutexGuard`) / 在 `.await` 处跨越持有的 Future 包含 `!Send` 类型（如 `Rc`、`MutexGuard`） | Restructure to drop non-Send values before `.await` / 重新调整结构，在 `.await` 前丢弃非 Send 值 |
| Future not polled / Future 未被轮询 | Calling `async fn` without `.await` or spawning — nothing happens / 调用 `async fn` 后未进行 `.await` 或 spawn —— 导致没有任何反应 | Always `.await` or `tokio::spawn` the returned future / 始终对返回的 future 进行 `.await` 或使用 `tokio::spawn` |
| Holding `MutexGuard` across `.await` / 跨 `.await` 持有 `MutexGuard` | `std::sync::MutexGuard` is `!Send`; async tasks may resume on different thread / `std::sync::MutexGuard` 是 `!Send`；异步任务可能会在不同线程恢复执行 | Use `tokio::sync::Mutex` or drop the guard before `.await` / 使用 `tokio::sync::Mutex` 或在 `.await` 前丢弃 guard |
| Accidental sequential execution / 意外的顺序执行 | `let a = foo().await; let b = bar().await;` runs sequentially / `let a = foo().await; let b = bar().await;` 会依次运行 | Use `tokio::join!` or `tokio::spawn` for concurrency / 使用 `tokio::join!` 或 `tokio::spawn` 实现并发 |

```rust
// ❌ Blocking the async executor:
// ❌ 阻塞异步执行器：
async fn bad() {
    std::thread::sleep(std::time::Duration::from_secs(5)); // Blocks entire thread!
                                                        // 会阻塞整个线程！
}

// ✅ Offload blocking work:
// ✅ 卸载阻塞性工作：
async fn good() {
    tokio::task::spawn_blocking(|| {
        std::thread::sleep(std::time::Duration::from_secs(5)); // Runs on blocking pool
                                                               // 在阻塞池中运行
    }).await.unwrap();
}
```

> **Comprehensive async coverage**: For `Stream`, `select!`, cancellation safety,
> structured concurrency, and `tower` middleware, see our dedicated
> **Async Rust Training** guide. This section covers just enough to read and
> write basic async code.

### Spawning and Structured Concurrency / 任务生成与结构化并发

Tokio's `spawn` creates a new asynchronous task — similar to `thread::spawn` but much lighter:

Tokio 的 `spawn` 会创建一个新的异步任务 —— 类似于 `thread::spawn` 但要轻量得多：

```rust,ignore
use tokio::task;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    // Spawn three concurrent tasks
    // 生成三个并发任务
    let h1 = task::spawn(async {
        sleep(Duration::from_millis(200)).await;
        "fetched user profile"
    });

    let h2 = task::spawn(async {
        sleep(Duration::from_millis(100)).await;
        "fetched order history"
    });

    let h3 = task::spawn(async {
        sleep(Duration::from_millis(150)).await;
        "fetched recommendations"
    });

    // Wait for all three concurrently (not sequentially!)
    // 同时等待这三个任务（并发而非顺序等待！）
    let (r1, r2, r3) = tokio::join!(h1, h2, h3);
    println!("{}", r1.unwrap());
    println!("{}", r2.unwrap());
    println!("{}", r3.unwrap());
}
```

**`join!` vs `try_join!` vs `select!`**：

| Macro / 宏 | Behavior / 行为 | Use when / 适用场景 |
|-------|----------|----------|
| `join!` | Waits for ALL futures / 等待所有 future | 所有任务都必须完成 |
| `try_join!` | Waits for all, short-circuits on first `Err` / 等待所有任务，遇错则立即短路 | 任务返回 `Result` |
| `select!` | Returns when FIRST future completes / 第一个 future 完成时即返回 | 超时、取消操作 |

```rust,ignore
use tokio::time::{timeout, Duration};

async fn fetch_with_timeout() -> Result<String, Box<dyn std::error::Error>> {
    let result = timeout(Duration::from_secs(5), async {
        // Simulate slow network call
        // 模拟慢速网络调用
        tokio::time::sleep(Duration::from_millis(100)).await;
        Ok::<_, Box<dyn std::error::Error>>("data".to_string())
    }).await??; // First ? unwraps Elapsed, second ? unwraps inner Result
                // 第一个 ? 解包 Elapsed 超时，第二个 ? 解包内部 Result
    Ok(result)
}
```

### `Send` Bounds and Why Futures Must Be `Send` / `Send` 约束以及为何 Future 必须满足 `Send`

When you `tokio::spawn` a future, it may resume on a different OS thread. This means the future must be `Send`. Common pitfalls:

当你通过 `tokio::spawn` 生成一个 future 时，它可能会在不同的系统线程上恢复执行。这意味着此 future 必须满足 `Send`。常见的陷阱有：

```rust,ignore
use std::rc::Rc;

async fn not_send() {
    let rc = Rc::new(42); // Rc is !Send / Rc 是非 Send 的
    tokio::time::sleep(std::time::Duration::from_millis(10)).await;
    println!("{}", rc); // rc is held across .await — future is !Send
                         // rc 在 .await 期间被持有 —— future 为非 Send
}

// Fix 1: Drop before .await / 解决方法 1：在 .await 前丢弃
async fn fixed_drop() {
    let data = {
        let rc = Rc::new(42);
        *rc // Copy the value out / 将值拷贝出来
    }; // rc dropped here / rc 在此处被丢弃
    tokio::time::sleep(std::time::Duration::from_millis(10)).await;
    println!("{}", data); // Just an i32, which is Send / 只是一个满足 Send 的 i32
}

// Fix 2: Use Arc instead of Rc / 解决方法 2：使用 Arc 代替 Rc
async fn fixed_arc() {
    let arc = std::sync::Arc::new(42); // Arc is Send / Arc 是 Send 的
    tokio::time::sleep(std::time::Duration::from_millis(10)).await;
    println!("{}", arc); // ✅ Future is Send / ✅ Future 满足 Send
}
```

> **Comprehensive async coverage / 异步内容的全面覆盖**：For `Stream`, `select!`, cancellation safety, structured concurrency, and `tower` middleware, see our dedicated **Async Rust Training** guide. This section covers just enough to read and write basic async code.
>
> 有关 `Stream`、`select!`、取消安全性（cancellation safety）、结构化并发以及 `tower` 中间件的详细内容，请参阅我们专门的 **Async Rust 进阶指南**。本节仅涵盖阅读和编写基础异步代码所需的知识。

> **See also / 延伸阅读**：[Ch 05 — Channels](ch05-channels-and-message-passing.md) 了解同步通道。[Ch 06 — Concurrency](ch06-concurrency-vs-parallelism-vs-threads.md) 了解操作系统线程与异步任务的对比。

> **Key Takeaways — Async / 关键要点：异步**
> - `async fn` returns a lazy `Future` — nothing runs until you `.await` or spawn it / `async fn` 返回一个惰性的 `Future` —— 除非对其进行 `.await` 或 spawn 否则什么都不会发生
> - Use `tokio::task::spawn_blocking` for CPU-heavy or blocking work inside async contexts / 在异步上下文中使用 `tokio::task::spawn_blocking` 来处理 CPU 密集型或阻塞型工作
> - Don't hold `std::sync::MutexGuard` across `.await` — use `tokio::sync::Mutex` instead / 不要跨 `.await` 持有 `std::sync::MutexGuard` —— 请改用 `tokio::sync::Mutex`
> - Futures must be `Send` when spawned — drop `!Send` types before `.await` points / 被 spawn 的 Future 必须满足 `Send` —— 在 `.await` 点之前丢弃所有非 Send 类型

---

### Exercise: Concurrent Fetcher with Timeout ★★ (~25 min) / 练习：带有超时的并发获取器

Write an async function `fetch_all` that spawns three `tokio::spawn` tasks, each simulating a network call with `tokio::time::sleep`. Join all three with `tokio::try_join!` wrapped in `tokio::time::timeout(Duration::from_secs(5), ...)`. Return `Result<Vec<String>, ...>` or an error if any task fails or the deadline expires.

编写一个异步函数 `fetch_all`，使用 `tokio::spawn` 生成三个任务，每个任务都使用 `tokio::time::sleep` 模拟网络调用。使用 `tokio::try_join!` 同时等待这三个任务，并将其包装在 `tokio::time::timeout(Duration::from_secs(5), ...)` 中。如果任何一个任务失败或超过截止时间，则返回错误，否则返回 `Result<Vec<String>, ...>`。

<details>
<summary>🔑 Solution / 参考答案</summary>

```rust,ignore
use tokio::time::{sleep, timeout, Duration};

async fn fake_fetch(name: &'static str, delay_ms: u64) -> Result<String, String> {
    sleep(Duration::from_millis(delay_ms)).await;
    Ok(format!("{name}: OK"))
}

async fn fetch_all() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let deadline = Duration::from_secs(5);

    let (a, b, c) = timeout(deadline, async {
        let h1 = tokio::spawn(fake_fetch("svc-a", 100));
        let h2 = tokio::spawn(fake_fetch("svc-b", 200));
        let h3 = tokio::spawn(fake_fetch("svc-c", 150));
        tokio::try_join!(h1, h2, h3)
    })
    .await??; // First ? for Timeout, second ? for JoinError / 第一个 ? 处理超时，第二个 ? 处理 JoinError

    Ok(vec![a?, b?, c?]) // Propagate any inner Result errors / 传播任何内部 Result 错误
}

#[tokio::main]
async fn main() {
    let results = fetch_all().await.unwrap();
    // Print results / 打印结果
    for r in &results {
        println!("{r}");
    }
}
```

</details>

***

