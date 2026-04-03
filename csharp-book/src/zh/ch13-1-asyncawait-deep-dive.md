[English Original](../en/ch13-1-asyncawait-deep-dive.md)

## 异步编程：C# Task vs Rust Future

> **你将学到：** Rust 的延迟加载型 `Future` 与 C# 的及早求值型 `Task` 对比；执行器模型 (tokio)；通过 `Drop` + `select!` 还是 `CancellationToken` 进行取消操作；以及并发请求的实际应用模式。
>
> **难度：** 🔴 高级

C# 开发者对 `async`/`await` 非常熟悉。Rust 虽然使用了相同的关键字，但其底层执行模型却截然不同。

### 执行器模型 (The Executor Model)

```csharp
// C# —— 运行时提供了内置的线程池和任务调度器
// async/await 在开箱即用的情况下即能正常运行
public async Task<string> FetchDataAsync(string url)
{
    using var client = new HttpClient();
    return await client.GetStringAsync(url);  // 由 .NET 线程池进行调度
}
// .NET 负责管理线程池、任务调度以及同步上下文 (Synchronization Context)
```

```rust
// Rust —— 没有内置的异步运行时。你需要自行选择执行器。
// 目前最流行的是 tokio。
async fn fetch_data(url: &str) -> Result<String, reqwest::Error> {
    let body = reqwest::get(url).await?.text().await?;
    Ok(body)
}

// 你“必须”拥有一个运行时来执行异步代码：
#[tokio::main]  // 此宏用于设置 tokio 运行时
async fn main() {
    let data = fetch_data("https://example.com").await.unwrap();
    println!("{}", &data[..100]);
}
```

### Future vs Task

| | C# `Task<T>` | Rust `Future<Output = T>` |
|---|---|---|
| **执行方式** | 创建后立即开始执行 | **延迟加载 (Lazy)** —— 在调用 `.await` 之前什么都不做 |
| **运行时** | 内置 (CLR 线程池) | 外部库 (tokio, async-std 等) |
| **取消操作** | 通过 `CancellationToken` | 丢弃 `Future` (或使用 `tokio::select!`) |
| **状态机** | 由编译器生成 | 由编译器生成 |
| **内容分配** | 在堆 (Heap) 上分配 | 在栈 (Stack) 上分配，除非被装箱 (Boxed) |

```rust
// 重要提示：Rust 中的 Future 是延迟加载的！
async fn compute() -> i32 { println!("正在计算！"); 42 }

let future = compute();  // 没有任何输出！Future 尚未被轮询 (Poll)。
let result = future.await; // “现在”才会打印 “正在计算！”
```

```csharp
// C# 的 Task 在创建时立即开始运行！
var task = ComputeAsync();  // 立即打印 “正在计算！”
var result = await task;    // 仅仅是等待任务完成
```

### 取消操作：CancellationToken vs Drop / select!

```csharp
// C# —— 通过 CancellationToken 进行协作式取消
public async Task ProcessAsync(CancellationToken ct)
{
    while (!ct.IsCancellationRequested)
    {
        await Task.Delay(1000, ct);  // 如果取消则抛出异常
        DoWork();
    }
}

var cts = new CancellationTokenSource(TimeSpan.FromSeconds(5));
await ProcessAsync(cts.Token);
```

```rust
// Rust —— 通过丢弃 Future 或使用 tokio::select! 进行取消
use tokio::time::{sleep, Duration};

async fn process() {
    loop {
        sleep(Duration::from_secs(1)).await;
        do_work();
    }
}

// 使用 select! 实现超时模式
async fn run_with_timeout() {
    tokio::select! {
        _ = process() => { println!("已完成"); }
        _ = sleep(Duration::from_secs(5)) => { println!("已超时！"); }
    }
    // 当 select! 选择了超时分支时，process() 的 future 会被“丢弃 (Dropped)”
    // —— 自动执行清理逻辑，无需 CancellationToken
}
```

### 实际应用模式：带超时的并发请求

```csharp
// C# —— 带有超时的并发 HTTP 请求
public async Task<string[]> FetchAllAsync(string[] urls, CancellationToken ct)
{
    var tasks = urls.Select(url => httpClient.GetStringAsync(url, ct));
    return await Task.WhenAll(tasks);
}
```

```rust
// Rust —— 使用 tokio::join! 或 futures::join_all 进行并发请求
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

// 带有超时的版本：
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
    .map_err(|_| "请求超时")?
    .map_err(|_| "请求失败")
}
```

---

<details>
<summary><strong>🏋️ 练习：异步超时模式</strong> (点击展开)</summary>

**挑战**：编写一个异步函数，同时从两个 URL 获取数据，返回最先响应的那个，并取消另一个请求。（这等同于 C# 中的 `Task.WhenAny`。）

<details>
<summary>🔑 参考答案</summary>

```rust
use tokio::time::{sleep, Duration};

// 模拟异步获取数据
async fn fetch(url: &str, delay_ms: u64) -> String {
    sleep(Duration::from_millis(delay_ms)).await;
    format!("来自 {url} 的响应")
}

async fn fetch_first(url1: &str, url2: &str) -> String {
    tokio::select! {
        result = fetch(url1, 200) => {
            println!("URL 1 胜出");
            result
        }
        result = fetch(url2, 500) => {
            println!("URL 2 胜出");
            result
        }
    }
    // “输掉”的分支其 future 会被自动丢弃 (即取消)
}

#[tokio::main]
async fn main() {
    let result = fetch_first("https://fast.api", "https://slow.api").await;
    println!("{result}");
}
```

**关键收获**：`tokio::select!` 是 Rust 中对应 `Task.WhenAny` 的功能 —— 它让多个 future 进行竞争，在第一个完成后结束，并丢弃（取消）其余的 future。

</details>
</details>

### 使用 `tokio::spawn` 衍生独立任务

在 C# 中，`Task.Run` 会启动一个独立于调用者的任务。Rust 中对应的功能是 `tokio::spawn`：

```rust
use tokio::task;

async fn background_work() {
    // 独立运行 —— 即便调用者的 future 被丢弃，它也会继续运行
    let handle = task::spawn(async {
        tokio::time::sleep(Duration::from_secs(2)).await;
        42
    });

    // 在衍生任务运行期间执行其他工作...
    println!("正在执行其他工作");

    // 在需要结果时进行 await
    let result = handle.await.unwrap(); // 42
}
```

```csharp
// C# 等效写法
var task = Task.Run(async () => {
    await Task.Delay(2000);
    return 42;
});
// 执行其他工作...
var result = await task;
```

**关键区别**：普通的 `async {}` 代码块是延迟加载的 —— 在被 await 之前什么都不做。而 `tokio::spawn` 会立即将其发布到运行时并启动，类似于 C# 的 `Task.Run`。

### 固定 (Pin)：为什么 Rust 异步有 C# 没有的概念

C# 开发者从未遇到过 `Pin` —— CLR 的垃圾回收器 (GC) 会自由移动对象并自动更新所有引用。而 Rust 没有 GC。当编译器将 `async fn` 转换为状态机时，该结构体可能包含指向其自身字段的内部指针。移动该结构体会导致这些指针失效。

`Pin<T>` 是一种包装器，它声明：**“此数值在内存中的位置不会被移动。”**

```rust
// 你会在这些语境中看到 Pin ：
trait Future {
    type Output;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
    //           ^^^^^^^^^^^^^^ 固定 (Pinned) —— 内部引用保持有效
}

// 从特性中返回一个装箱的 future ：
fn make_future() -> Pin<Box<dyn Future<Output = i32> + Send>> {
    Box::pin(async { 42 })
}
```

**在实践中，你几乎不需要亲自编写 `Pin`。** `async fn` 和 `.await` 语法会自动处理它。你只会在以下情况遇到它：
- 编译器错误信息中（按照建议操作即可）。
- 在 `tokio::select!` 中（使用 `pin!()` 宏）。
- 特性方法返回 `dyn Future` 时（使用 `Box::pin(async { ... })`）。

> **想深入了解吗？** 配套的 [异步 Rust 训练](../../async-book/src/ch04-pin-and-unpin.md) 详细介绍了 Pin, Unpin, 自引用结构体 (Self-referential structs) 以及结构化固定 (Structural pinning)。
