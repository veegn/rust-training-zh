## Async Programming: C# Task vs Rust Future | 异步编程：C# `Task` vs Rust `Future`

> **What you'll learn:** Rust's lazy `Future` vs C#'s eager `Task`, the executor model (tokio),
> cancellation via `Drop` + `select!` vs `CancellationToken`, and real-world patterns for concurrent requests.
>
> **你将学到什么：** Rust 惰性的 `Future` 与 C# 立即启动的 `Task` 有什么区别，执行器模型（tokio）是什么，
> `Drop` + `select!` 如何对应 `CancellationToken`，以及并发请求的常见实战模式。
>
> **Difficulty:** Advanced
>
> **难度：** 高级

C# developers are deeply familiar with `async`/`await`. Rust uses the same keywords but with a fundamentally different execution model.

C# 开发者通常对 `async`/`await` 非常熟悉。但 Rust 虽然用了同样的关键字，背后的执行模型却有根本差异。

### The Executor Model | 执行器模型

```csharp
// C# - The runtime provides a built-in thread pool and task scheduler
// async/await "just works" out of the box
public async Task<string> FetchDataAsync(string url)
{
    using var client = new HttpClient();
    return await client.GetStringAsync(url);  // Scheduled by .NET thread pool
}
// .NET manages the thread pool, task scheduling, and synchronization context
```

```rust
// Rust - No built-in async runtime. You choose an executor.
// The most popular is tokio.
async fn fetch_data(url: &str) -> Result<String, reqwest::Error> {
    let body = reqwest::get(url).await?.text().await?;
    Ok(body)
}

// You MUST have a runtime to execute async code:
#[tokio::main]  // This macro sets up the tokio runtime
async fn main() {
    let data = fetch_data("https://example.com").await.unwrap();
    println!("{}", &data[..100]);
}
```

### Future vs Task | `Future` vs `Task`

| | C# `Task<T>` | Rust `Future<Output = T>` |
|---|---|---|
| **Execution** | Starts immediately when created | **Lazy** - does nothing until `.await`ed |
| **执行时机** | 创建后立即开始 | **惰性** - 不被 `.await` 就不会执行 |
| **Runtime** | Built-in (CLR thread pool) | External (tokio, async-std, etc.) |
| **运行时** | 内置（CLR 线程池） | 外部提供（tokio、async-std 等） |
| **Cancellation** | `CancellationToken` | Drop the `Future` (or `tokio::select!`) |
| **取消机制** | `CancellationToken` | 直接丢弃 `Future`（或借助 `tokio::select!`） |
| **State machine** | Compiler-generated | Compiler-generated |
| **状态机** | 编译器生成 | 编译器生成 |
| **Size** | Heap-allocated | Stack-allocated until boxed |
| **内存形态** | 常见为堆分配 | 默认是栈上值，装箱后才上堆 |

```rust
// IMPORTANT: Futures are lazy in Rust!
async fn compute() -> i32 { println!("Computing!"); 42 }

let future = compute();  // Nothing printed! Future not polled yet.
let result = future.await; // NOW "Computing!" is printed
```

```csharp
// C# Tasks start immediately!
var task = ComputeAsync();  // "Computing!" printed immediately
var result = await task;    // Just waits for completion
```

### Cancellation: CancellationToken vs Drop / select! | 取消机制：`CancellationToken` vs `Drop` / `select!`

```csharp
// C# - Cooperative cancellation with CancellationToken
public async Task ProcessAsync(CancellationToken ct)
{
    while (!ct.IsCancellationRequested)
    {
        await Task.Delay(1000, ct);  // Throws if cancelled
        DoWork();
    }
}

var cts = new CancellationTokenSource(TimeSpan.FromSeconds(5));
await ProcessAsync(cts.Token);
```

```rust
// Rust - Cancellation by dropping the future, or with tokio::select!
use tokio::time::{sleep, Duration};

async fn process() {
    loop {
        sleep(Duration::from_secs(1)).await;
        do_work();
    }
}

// Timeout pattern with select!
async fn run_with_timeout() {
    tokio::select! {
        _ = process() => { println!("Completed"); }
        _ = sleep(Duration::from_secs(5)) => { println!("Timed out!"); }
    }
    // When select! picks the timeout branch, the process() future is DROPPED
    // - automatic cleanup, no CancellationToken needed
}
```

### Real-World Pattern: Concurrent Requests with Timeout | 实战模式：带超时的并发请求

```csharp
// C# - Concurrent HTTP requests with timeout
public async Task<string[]> FetchAllAsync(string[] urls, CancellationToken ct)
{
    var tasks = urls.Select(url => httpClient.GetStringAsync(url, ct));
    return await Task.WhenAll(tasks);
}
```

```rust
// Rust - Concurrent requests with tokio::join! or futures::join_all
use futures::future::join_all;

async fn fetch_all(urls: &[&str]) -> Vec<Result<String, reqwest::Error>> {
    let futures = urls.iter().map(|url| reqwest::get(*url));
    let responses = join_all(futures).await;

    let mut results = Vec::new();
    for resp in responses {
        results.push(resp?.text().await);
    }
    results
}

// With timeout:
async fn fetch_all_with_timeout(urls: &[&str]) -> Result<Vec<String>, &'static str> {
    tokio::time::timeout(
        Duration::from_secs(10),
        async {
            let futures: Vec<_> = urls.iter()
                .map(|url| async { reqwest::get(*url).await?.text().await })
                .collect();
            let results = join_all(futures).await;
            results.into_iter().collect::<Result<Vec<_>, _>>()
        }
    )
    .await
    .map_err(|_| "Request timed out")?
    .map_err(|_| "Request failed")
}
```

<details>
<summary><strong>Exercise: Async Timeout Pattern | 练习：异步超时模式</strong> (click to expand / 点击展开)</summary>

**Challenge**: Write an async function that fetches from two URLs concurrently, returns whichever responds first, and cancels the other. (This is `Task.WhenAny` in C#.)

**挑战：** 编写一个异步函数，并发请求两个 URL，返回先完成的那个结果，并取消另一个请求。（这相当于 C# 里的 `Task.WhenAny`。）

<details>
<summary>Solution | 参考答案</summary>

```rust
use tokio::time::{sleep, Duration};

// Simulated async fetch
async fn fetch(url: &str, delay_ms: u64) -> String {
    sleep(Duration::from_millis(delay_ms)).await;
    format!("Response from {url}")
}

async fn fetch_first(url1: &str, url2: &str) -> String {
    tokio::select! {
        result = fetch(url1, 200) => {
            println!("URL 1 won");
            result
        }
        result = fetch(url2, 500) => {
            println!("URL 2 won");
            result
        }
    }
    // The losing branch's future is automatically dropped (cancelled)
}

#[tokio::main]
async fn main() {
    let result = fetch_first("https://fast.api", "https://slow.api").await;
    println!("{result}");
}
```

**Key takeaway**: `tokio::select!` is Rust's equivalent of `Task.WhenAny` - it races multiple futures, completes when the first one finishes, and drops (cancels) the rest.

**关键结论：** `tokio::select!` 可以看作 Rust 对应 `Task.WhenAny` 的机制。它会让多个 future 竞速，谁先完成就返回谁，并自动丢弃其余 future。

</details>
</details>

### Spawning Independent Tasks with `tokio::spawn` | 用 `tokio::spawn` 启动独立任务

In C#, `Task.Run` launches work that runs independently of the caller. Rust's equivalent is `tokio::spawn`:

在 C# 中，`Task.Run` 会启动一个独立于调用方的任务。Rust 中最接近的对应物是 `tokio::spawn`：

```rust
use tokio::task;

async fn background_work() {
    // Runs independently - even if the caller's future is dropped
    let handle = task::spawn(async {
        tokio::time::sleep(Duration::from_secs(2)).await;
        42
    });

    // Do other work while the spawned task runs...
    println!("Doing other work");

    // Await the result when you need it
    let result = handle.await.unwrap(); // 42
}
```

```csharp
// C# equivalent
var task = Task.Run(async () => {
    await Task.Delay(2000);
    return 42;
});
// Do other work...
var result = await task;
```

**Key difference**: A regular `async {}` block is lazy - it does nothing until awaited. `tokio::spawn` launches it on the runtime immediately, like C#'s `Task.Run`.

**关键区别：** 普通的 `async {}` 代码块本身是惰性的，不被 await 就不会执行；而 `tokio::spawn` 会像 C# 的 `Task.Run` 一样，立刻把任务挂到运行时上开始执行。

### Pin: Why Rust Async Has a Concept C# Doesn't | `Pin`：为什么 Rust async 有而 C# 没有这个概念

C# developers never encounter `Pin` - the CLR's garbage collector moves objects freely and updates all references automatically. Rust has no GC. When the compiler transforms an `async fn` into a state machine, that struct may contain internal pointers to its own fields. Moving the struct would invalidate those pointers.

C# 开发者基本不会直接接触 `Pin`，因为 CLR 的垃圾回收器可以自由移动对象并自动更新引用。Rust 没有 GC。当编译器把 `async fn` 转换成状态机结构体时，这个结构体内部可能会包含指向自身字段的内部引用；如果再移动这个结构体，这些引用就会失效。

`Pin<T>` is a wrapper that says: **"this value will not be moved in memory."**

`Pin<T>`` 的含义可以简单理解为：**“这个值在内存中不会再被移动。”**

```rust
// You'll see Pin in these contexts:
trait Future {
    type Output;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
    //           ^^^^^^^^^^^^^^ pinned - internal references stay valid
}

// Returning a boxed future from a trait:
fn make_future() -> Pin<Box<dyn Future<Output = i32> + Send>> {
    Box::pin(async { 42 })
}
```

**In practice, you almost never write `Pin` yourself.** The `async fn` and `.await` syntax handles it. You'll encounter it only in:

**在实践里，你几乎不用手写 `Pin`。** `async fn` 和 `.await` 语法已经帮你处理了绝大部分场景。你通常只会在下面几类地方碰到它：

- Compiler error messages (follow the suggestion)
- 编译器报错信息里（按提示修）
- `tokio::select!` (use the `pin!()` macro)
- `tokio::select!` 里（通常配合 `pin!()` 宏）
- Trait methods returning `dyn Future` (use `Box::pin(async { ... })`)
- 返回 `dyn Future` 的 trait 方法中（通常用 `Box::pin(async { ... })`）

> **Want the deep dive?** The companion [Async Rust Training](../../async-book/src/ch04-pin-and-unpin.md) covers Pin, Unpin, self-referential structs, and structural pinning in full detail.
>
> **想看更深入的解释？** 配套的 [Async Rust Training](../../async-book/src/ch04-pin-and-unpin.md) 对 Pin、Unpin、自引用结构体和结构性 pinning 做了完整展开。

***
