# 16. Async/Await 核心要点 🔶

> **你将学到：**
> - `Future` 与 goroutine 有何不同。
> - Tokio 快速上手：生成与等待任务。
> - 常见的异步陷阱。
> - 用于 CPU 密集型工作的 `spawn_blocking`。

## Future 与 轮询 (Polling)

- **Future 是惰性的**：调用 `async fn` 仅返回一个状态机。在对其执行 `.await` 或将其提交到运行时（runtime）之前，什么都不会发生。
- **需要运行时**：标准库提供了 `Future` trait，但你需要像 `Tokio` 这样的 crate 才能真正运行它们。

```rust
async fn fetch_data() -> String {
    // .await 将控制权让回给执行器
    let res = reqwest::get("url").await.unwrap();
    res.text().await.unwrap()
}
```

---

## Tokio 快速上手

```rust
#[tokio::main]
async fn main() {
    let handle = tokio::spawn(async {
        // 在后台并发运行
        "已完成"
    });

    // 等待生成的任务
    let res = handle.await.unwrap();
}
```

### 并发控制

| 宏 | 效果 |
|-------|--------|
| `tokio::join!` | 等待多个 future 全部完成。 |
| `tokio::select!` | 响应第一个完成的 future。 |
| `tokio::time::timeout` | 如果 future 在规定时间内未完成，则返回错误。 |

---

## 常见陷阱

### 1. 阻塞执行器 (Blocking the Executor)
**不要** 在 `async` 代码块中使用 `std::thread::sleep` 或长时间的 CPU 密集型循环。这会阻塞执行器处理其他任务。
**请使用** `tokio::task::spawn_blocking`。

```rust
let res = task::spawn_blocking(|| {
    // 处理重型 CPU 运算或阻塞型 I/O
    std::thread::sleep(Duration::from_secs(5));
}).await.unwrap();
```

### 2. 跨 `.await` 持有 MutexGuard
**不要** 在调用 `.await` 时持有 `std::sync::MutexGuard`。由于它不满足 `Send` 约束，会引发编译错误。
**请使用** `tokio::sync::Mutex` 或调整代码结构以在 `.await` 前释放 guard。

---

## Send 约束

通过 `tokio::spawn` 生成的 Future 必须满足 `Send` 约束。这意味着在 `.await` 点被捕获或持有的所有变量也必须满足 `Send`。

***
