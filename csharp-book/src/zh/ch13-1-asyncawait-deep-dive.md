# 异步编程：C# Task vs Rust Future

> **你将学到什么：** Rust 惰性的 `Future` 与 C# 立即启动的 `Task` 有什么区别，执行器模型（Tokio）是什么，以及 `Drop` 取消机制。
>
> **难度：** 高级

C# 开发者对 `async`/`await` 通常非常熟悉。Rust 虽然使用了相同的关键字，但背后的执行模型有着根本性的差异。

---

## 执行器模型 (Executor Model)
在 C# 中，.NET 运行时内置了一个全功能的线程池和任务调度器。你只需要使用 `await`，一切都会“理所当然地正常工作”。

而在 Rust 中，**标准库并没有内置异步运行时。** 你必须选择一个库来提供“执行器 (Executor)”来实际运行这些异步任务。**Tokio** 目前是社区事实上的行业标准。

```rust
#[tokio::main] // 这个宏会为你配置好 Tokio 运行时
async fn main() {
    let result = fetch_data().await;
}
```

---

## Future vs Task
最大的区别在于：**惰性 (Laziness)**。
*   **C# `Task`**：一旦被创建，就开始在线程池里运行。
*   **Rust `Future`**：除非被 `.await`（或被轮询），否则它**什么都不会做**。

```rust
// C# - 任务立即启动
var task = DoWorkAsync(); 

// Rust - 此时还没有发生任何事！
let future = do_work_async(); 
// 现在才真正开始执行：
let result = future.await; 
```

---

## 取消机制：无需传递 Token
在 C# 中，你需要到处传递 `CancellationToken` 来取消任务。而在 Rust 中，取消机制被内建到了所有权系统里。如果你**丢弃（Drop）**了一个 Future（比如通过 `select!` 选择了其他分支），该异步任务就会立即停止运行。

```rust
tokio::select! {
    val = active_task() => println!("任务已完成: {}", val),
    _ = sleep(Duration::from_secs(5)) => println!("任务超时!"),
}
// 如果 'sleep' 先完成了，'active_task' 的 Future 就会被立即 Drop 掉，也就自动被取消了。
```

---

## C# 开发者总结表
| **概念** | **C# / .NET** | **Rust / Tokio** |
| :--- | :--- | :--- |
| **异步类型** | `Task<T>` | `impl Future<Output = T>` |
| **执行时机** | 急切 (立即启动) | 惰性 (直到被 `.await`) |
| **运行时环境** | 内置 (线程池) | 第三方库 (如 Tokio) |
| **取消方式** | `CancellationToken` | 直接 `Drop` 掉 Future |
| **并发等待** | `Task.WhenAll` | `tokio::join!` |
| **竞速等待** | `Task.WhenAny` | `tokio::select!` |

---

## 练习：并发请求
**挑战：** 使用 `tokio::join!` 同时获取两个不同的异步值，并输出它们的和。

```rust
async fn get_a() -> i32 { 10 }
async fn get_b() -> i32 { 20 }

#[tokio::main]
async fn main() {
    let (a, b) = tokio::join!(get_a(), get_b());
    println!("两数之和为 {}", a + b);
}
```
**关键理解：** Rust 的异步模型是基于拉取 (Pull-based) 的，这种惰性设计使其效率极高。你只为你真正用到的东西支付开销，而取消任务只需简单地“不再理会”那个异步值即可。
