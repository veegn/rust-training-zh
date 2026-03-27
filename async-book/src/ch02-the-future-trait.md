# 2. The Future Trait / 2. `Future` Trait 🟡

> **What you'll learn / 你将学到：**
> - The `Future` trait: `Output`, `poll()`, `Context`, `Waker` / `Future` trait 的组成：`Output`、`poll()`、`Context` 与 `Waker`
> - How a waker tells the executor "poll me again" / Waker 如何告知执行器“请再次轮询我”
> - The contract: never call `wake()` = your program silently hangs / 契约：如果不调用 `wake()`，程序就会静默挂起
> - Implementing a real future by hand (`Delay`) / 手动落实一个真实的 future（`Delay`）

## Anatomy of a Future / Future 的解剖

Everything in async Rust ultimately implements this trait:

Async Rust 中的一切最终都实现了这个 trait：

```rust
pub trait Future {
    type Output;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
}

pub enum Poll<T> {
    Ready(T),   // The future has completed with value T
    Pending,    // The future is not ready yet — call me back later
}
```

That's it. A `Future` is anything that can be *polled* — asked "are you done yet?" — and responds with either "yes, here's the result" or "not yet, I'll wake you up when I'm ready."

就这么简单。`Future` 是任何可以被 *poll*（轮询）的对象 —— 即被询问“你做完了吗？” —— 并回答“做完了，这是结果”或“还没呢，等我准备好了会叫醒你”。

### Output, poll(), Context, Waker / Output、poll()、Context 与 Waker

```mermaid
sequenceDiagram
    participant E as Executor
    participant F as Future
    participant R as Resource (I/O)

    E->>F: poll(cx)
    F->>R: Check: is data ready?
    R-->>F: Not yet
    F->>R: Register waker from cx
    F-->>E: Poll::Pending

    Note over R: ... time passes, data arrives ...

    R->>E: waker.wake() — "I'm ready!"
    E->>F: poll(cx) — try again
    F->>R: Check: is data ready?
    R-->>F: Yes! Here's the data
    F-->>E: Poll::Ready(data)
```

Let's break down each piece:

让我们分解每个部分：

```rust
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

// A future that returns 42 immediately
struct Ready42;

impl Future for Ready42 {
    type Output = i32; // What the future eventually produces

    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<i32> {
        Poll::Ready(42) // Always ready — no waiting
    }
}
```

**The components / 组成部分**:
- **`Output`** — the type of value produced when the future completes / Future 完成时产生的值的类型
- **`poll()`** — called by the executor to check progress; returns `Ready(value)` or `Pending` / 由执行器调用以检查进度；返回 `Ready(value)` 或 `Pending`
- **`Pin<&mut Self>`** — ensures the future won't be moved in memory (we'll cover why in Ch. 4) / 确保 future 不会在内存中被移动（我们将在第 4 章解释原因）
- **`Context`** — carries the `Waker` so the future can signal the executor when it's ready to make progress / 携带 `Waker`，以便 future 在准备好继续时通知执行器

### The Waker Contract / Waker 契约

The `Waker` is the callback mechanism. When a future returns `Pending`, it *must* arrange for `waker.wake()` to be called later — otherwise the executor will never poll it again and the program hangs.

`Waker` 是一种回调机制。当 future 返回 `Pending` 时，它 *必须* 安排在之后调用 `waker.wake()` —— 否则执行器永远不会再次轮询它，程序就会挂起。

```rust
use std::task::{Context, Poll, Waker};
use std::pin::Pin;
use std::future::Future;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

/// A future that completes after a delay (toy implementation)
struct Delay {
    completed: Arc<Mutex<bool>>,
    waker_stored: Arc<Mutex<Option<Waker>>>,
    duration: Duration,
    started: bool,
}

impl Delay {
    fn new(duration: Duration) -> Self {
        Delay {
            completed: Arc::new(Mutex::new(false)),
            waker_stored: Arc::new(Mutex::new(None)),
            duration,
            started: false,
        }
    }
}

impl Future for Delay {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        // Check if already completed
        if *self.completed.lock().unwrap() {
            return Poll::Ready(());
        }

        // Store the waker so the background thread can wake us
        *self.waker_stored.lock().unwrap() = Some(cx.waker().clone());

        // Start the background timer on first poll
        if !self.started {
            self.started = true;
            let completed = Arc::clone(&self.completed);
            let waker = Arc::clone(&self.waker_stored);
            let duration = self.duration;

            thread::spawn(move || {
                thread::sleep(duration);
                *completed.lock().unwrap() = true;

                // CRITICAL: wake the executor so it polls us again
                if let Some(w) = waker.lock().unwrap().take() {
                    w.wake(); // "Hey executor, I'm ready — poll me again!"
                }
            });
        }

        Poll::Pending // Not done yet
    }
}
```

> **Key insight**: In C#, the TaskScheduler handles waking automatically.
> In Rust, **you** (or the I/O library you use) are responsible for calling
> `waker.wake()`. Forget it, and your program silently hangs.
>
> **关键洞察**：在 C# 中，TaskScheduler 会自动处理唤醒。而在 Rust 中，**你**（或者你使用的 I/O 库）负责调用 `waker.wake()`。如果忘了这一步，你的程序就会静默挂起。

### Exercise: Implement a CountdownFuture / 练习：实现一个倒计时 Future

<details>
<summary>🏋️ Exercise / 练习（点击展开）</summary>

**Challenge**: Implement a `CountdownFuture` that counts down from N to 0, printing the current count each time it's polled. When it reaches 0, it completes with `Ready("Liftoff!")`.

**挑战**：实现一个 `CountdownFuture`，从 N 倒数到 0，每次被轮询时打印当前数值。当达到 0 时，返回 `Ready("Liftoff!")` 完成。

*Hint*: The future needs to store the current count and decrement it on each poll. Remember to always re-register the waker!

*提示*：Future 需要存储当前计数并在每次轮询时递减。记得一定要重新注册 waker！

<details>
<summary>🔑 Solution / 参考答案</summary>

```rust
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

struct CountdownFuture {
    count: u32,
}

impl CountdownFuture {
    fn new(start: u32) -> Self {
        CountdownFuture { count: start }
    }
}

impl Future for CountdownFuture {
    type Output = &'static str;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.count == 0 {
            println!("Liftoff!");
            Poll::Ready("Liftoff!")
        } else {
            println!("{}...", self.count);
            self.count -= 1;
            cx.waker().wake_by_ref(); // Schedule re-poll immediately
            Poll::Pending
        }
    }
}
```

**Key takeaway**: This future is polled once per count. Each time it returns `Pending`, it immediately wakes itself to be polled again. In production, you'd use a timer instead of busy-polling.

**关键点**：这个 future 每一跳会被轮询一次。每次返回 `Pending` 时，它都会立即唤醒自己以便再次被轮询。在生产环境中，你会使用定时器而不是这种忙碌轮询。

</details>
</details>

> **Key Takeaways — The Future Trait / 关键要点：Future Trait**
> - `Future::poll()` returns `Poll::Ready(value)` or `Poll::Pending` / `Future::poll()` 返回 `Poll::Ready(value)` 或 `Poll::Pending`
> - A future must register a `Waker` before returning `Pending` — the executor uses it to know when to re-poll / Future 在返回 `Pending` 之前必须注册一个 `Waker` —— 执行器通过它知道何时重新轮询
> - `Pin<&mut Self>` guarantees the future won't be moved in memory (needed for self-referential state machines — see Ch 4) / `Pin<&mut Self>` 保证 future 不会在内存中移动（自引用状态机需要此特性 —— 见第 4 章）
> - Everything in async Rust — `async fn`, `.await`, combinators — is built on this one trait / Async Rust 中的一切 —— `async fn`、`.await`、组合器 —— 都构建在这一 trait 之上

> **See also / 延伸阅读：** [Ch 3 — How Poll Works / 第 3 章：poll 的工作机制](ch03-how-poll-works.md) for the executor loop, [Ch 6 — Building Futures by Hand / 第 6 章：手写 Future](ch06-building-futures-by-hand.md) for more complex implementations

***


