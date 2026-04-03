[English Original](../en/ch13-production-patterns.md)

# 13. 生产模式 🔴

> **你将学到：**
> - 使用 `watch` 通道与 `select!` 实现优雅停机（Graceful shutdown）
> - 背压（Backpressure）：有界通道防止内存溢出（OOM）
> - 结构化并发：`JoinSet` 与 `TaskTracker`
> - 超时、重试与指数退避算法
> - 错误处理：`thiserror` vs `anyhow`，以及双重 `?` 模式
> - Tower：axum、tonic 与 hyper 使用的中间件模式

## 优雅停机 (Graceful Shutdown)

生产级服务器必须能够干净地关闭 —— 完成正在进行的请求、冲刷缓冲区、关闭连接：

```rust
use tokio::signal;
use tokio::sync::watch;

async fn main_server() {
    // 创建一个停机信号通道
    let (shutdown_tx, shutdown_rx) = watch::channel(false);

    // 派生服务器任务
    let server_handle = tokio::spawn(run_server(shutdown_rx.clone()));

    // 等待 Ctrl+C 信号
    signal::ctrl_c().await.expect("监听 Ctrl+C 失败");
    println!("接收到停机信号，正在完成剩余请求...");

    // 通知所有任务进行停机
    shutdown_tx.send(true).unwrap();

    // 等待服务器完成（设置超时时间）
    match tokio::time::timeout(
        std::time::Duration::from_secs(30),
        server_handle,
    ).await {
        Ok(Ok(())) => println!("服务器已优雅停机"),
        Ok(Err(e)) => eprintln!("服务器错误: {e}"),
        Err(_) => eprintln!("服务器停机超时 —— 强制退出"),
    }
}

async fn run_server(mut shutdown: watch::Receiver<bool>) {
    loop {
        tokio::select! {
            // 接收新连接
            conn = accept_connection() => {
                let shutdown = shutdown.clone();
                tokio::spawn(handle_connection(conn, shutdown));
            }
            // 停机信号
            _ = shutdown.changed() => {
                if *shutdown.borrow() {
                    println!("停止接收新连接");
                    break;
                }
            }
        }
    }
    // 正在处理的连接将会自行完成
    // 因为它们持有各自的 shutdown_rx 克隆
}

async fn handle_connection(conn: Connection, mut shutdown: watch::Receiver<bool>) {
    loop {
        tokio::select! {
            request = conn.next_request() => {
                // 完整处理请求 —— 不要中途抛弃
                process_request(request).await;
            }
            _ = shutdown.changed() => {
                if *shutdown.borrow() {
                    // 完成当前请求后退出
                    break;
                }
            }
        }
    }
}
```

```mermaid
sequenceDiagram
    participant OS as 操作系统信号
    participant Main as 主任务
    participant WCH as watch 通道
    participant W1 as 工作单元 1
    participant W2 as 工作单元 2

    OS->>Main: SIGINT (Ctrl+C)
    Main->>WCH: send(true)
    WCH-->>W1: 触发 changed()
    WCH-->>W2: 触发 changed()

    Note over W1: 完成当前请求
    Note over W2: 完成当前请求

    W1-->>Main: 任务完成
    W2-->>Main: 任务完成
    Main->>Main: 所有工作单元就绪 → 退出
```

### 有界通道提供的背压 (Backpressure)

如果生产者的速度快于消费者，无界通道会导致内存溢出（OOM）。在生产环境中请务必使用有界通道：

```rust
use tokio::sync::mpsc;

async fn backpressure_example() {
    // 有界通道：最大缓冲 100 个条目
    let (tx, mut rx) = mpsc::channel::<WorkItem>(100);

    // 生产者：当缓冲区满时会自动减速
    let producer = tokio::spawn(async move {
        for i in 0..1_000_000 {
            // send() 是异步的 —— 如果缓冲区满，它会等待
            // 这自然地产生了“背压”！
            tx.send(WorkItem { id: i }).await.unwrap();
        }
    });

    // 消费者：按自己的节奏处理条目
    let consumer = tokio::spawn(async move {
        while let Some(item) = rx.recv().await {
            process(item).await; // 处理慢一点也没关系 —— 生产者会等待
        }
    });

    let _ = tokio::join!(producer, consumer);
}

// 对比无界通道 —— 这是危险的：
// let (tx, rx) = mpsc::unbounded_channel(); // 无背压！
// 生产者可能会无限填满内存
```

### 结构化并发：JoinSet 与 TaskTracker

`JoinSet` 用于将相关的任务分组，并确保它们全部完成：

```rust
use tokio::task::JoinSet;

async fn structured_concurrency() {
    let mut set = JoinSet::new();

    // 派生一批任务
    for url in get_urls() {
        set.spawn(async move {
            fetch_and_process(url).await
        });
    }

    // 收集所有结果（顺序不保证）
    let mut results = Vec::new();
    while let Some(result) = set.join_next().await {
        match result {
            Ok(Ok(data)) => results.push(data),
            Ok(Err(e)) => eprintln!("任务错误: {e}"),
            Err(e) => eprintln!("任务发生 panic: {e}"),
        }
    }

    // 到这里所有任务都已完成 —— 不会有残留的后台工作
    println!("已处理 {} 个条目", results.len());
}
```

### 超时、重试与指数退避

```rust
use tokio::time::{timeout, sleep, Duration};

// 指数退避重试
async fn retry_with_backoff<F, Fut, T, E>(
    max_attempts: u32,
    base_delay_ms: u64,
    operation: F,
) -> Result<T, E>
where
    F: Fn() -> Fut,
    Fut: std::future::Future<Output = Result<T, E>>,
    E: std::fmt::Display,
{
    let mut delay = Duration::from_millis(base_delay_ms);

    for attempt in 1..=max_attempts {
        match operation().await {
            Ok(result) => return Ok(result),
            Err(e) => {
                if attempt == max_attempts {
                    return Err(e);
                }
                sleep(delay).await;
                delay *= 2; // 指数倍增延迟
            }
        }
    }
    unreachable!()
}
```

> **生产提示 —— 加入抖动 (Jitter)**：上面的函数是纯粹的指数退避，但在生产环境中，如果大量客户端同时失败并重试，会导致“惊群效应（thundering herd）”。请加入随机的 *抖动* —— 比如 `sleep(delay + rand_jitter)`，让重试请求在时间上分散开。

### 异步代码中的错误处理

异步引入了独特的错误传播挑战 —— 派生任务创建了错误边界，超时错误会包装内部错误，当 Future 跨越任务边界时，`?` 操作符会有不同的交互。

**`thiserror` vs `anyhow`** —— 选对工具：

```rust
// thiserror：为类库和公共 API 定义强类型错误
// 每个变体都是显式的 —— 调用者可以对特定错误进行 match
use thiserror::Error;

#[derive(Error, Debug)]
enum DiagError {
    #[error("传感器 {sensor} 超出范围: {value}°C")]
    OverTemp { sensor: String, value: f64 },

    #[error("操作在 {0:?} 后超时")]
    Timeout(std::time::Duration),
}

// anyhow：为应用程序和原型提供快速错误处理
// 包装任意错误 —— 无需为每种情况定义类型
use anyhow::{Context, Result};

async fn run_diagnostics() -> Result<()> {
    let _ = load_config()
        .await
        .context("加载配置失败")?; // 增加上下文信息信息信息
    Ok(())
}
```

| Crate | 适用场景 | 错误类型 | 是否支持匹配 |
|-------|----------|-----------|----------|
| `thiserror` | 类库代码、公共 API | `enum MyError` | 支持 `match err` |
| `anyhow` | 应用程序、CLI 工具、脚本 | `anyhow::Error` | 需使用 `downcast` |

**双重 `?` 模式**：

```rust
async fn spawn_with_errors() -> Result<String, AppError> {
    let handle = tokio::spawn(async {
        let resp = reqwest::get("https://example.com").await?;
        Ok::<_, reqwest::Error>(resp.text().await?)
    });

    // 双重 ?: 第一个 ? 解出 JoinError (任务 panic)，第二个 ? 解出内部的 Result
    let result = handle.await??;
    Ok(result)
}
```

### Tower：中间件模式

[Tower](https://docs.rs/tower) 定义了一个可组合的 `Service` trait —— 它是 Rust 异步中间件的骨架（被 `axum`, `tonic`, `hyper` 所采用）：

```rust
// Tower 核心 trait (简化版):
pub trait Service<Request> {
    type Response;
    type Error;
    type Future: Future<Output = Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>>;
    fn call(&mut self, req: Request) -> Self::Future;
}
```

中间件通过包装一个 `Service` 来添加跨切面行为（日志、超时、限流），而无需修改内部逻辑：

```rust
let service = ServiceBuilder::new()
    .layer(TimeoutLayer::new(Duration::from_secs(10)))       // 最外层：超时
    .layer(RateLimitLayer::new(100, Duration::from_secs(1))) // 随后：限流
    .service(my_handler);                                     // 最内层：你的业务逻辑
```

<details>
<summary><strong>🏋️ 实践任务：带工作池的优雅停机</strong> (点击展开)</summary>

**挑战**：构建一个基于通道的工作队列，包含 N 个工作任务，并在按下 Ctrl+C 时实现优雅停机。工作任务应在退出前完成当前正在处理的工作。

<details>
<summary>🔑 参考方案</summary>

```rust
use tokio::sync::{mpsc, watch};
use tokio::time::{sleep, Duration};

struct WorkItem { id: u64 }

#[tokio::main]
async fn main() {
    let (work_tx, work_rx) = mpsc::channel::<WorkItem>(100);
    let (shutdown_tx, shutdown_rx) = watch::channel(false);
    let work_rx = std::sync::Arc::new(tokio::sync::Mutex::new(work_rx));

    let mut handles = Vec::new();
    for id in 0..4 {
        let rx = work_rx.clone();
        let mut shutdown = shutdown_rx.clone();
        handles.push(tokio::spawn(async move {
            loop {
                let item = {
                    let mut rx = rx.lock().await;
                    tokio::select! {
                        item = rx.recv() => item,
                        _ = shutdown.changed() => {
                            if *shutdown.borrow() { None } else { continue }
                        }
                    }
                };
                match item {
                    Some(work) => {
                        println!("工作单元 {id}: 正在处理 {}", work.id);
                        sleep(Duration::from_millis(200)).await;
                    }
                    None => break,
                }
            }
        }));
    }

    // 提交一些任务
    for i in 0..20 {
        let _ = work_tx.send(WorkItem { id: i }).await;
    }

    // 处理停机
    tokio::signal::ctrl_c().await.unwrap();
    shutdown_tx.send(true).unwrap();
    for h in handles { let _ = h.await; }
    println!("已干净地停机。");
}
```

</details>
</details>

> **关键要诀 —— 生产模式**
> - 使用 `watch` 通道 + `select!` 实现多组件协调的优雅停机。
> - 有界通道 (`mpsc::channel(N)`) 提供了 **背压** 机制 —— 当缓冲区满时发送者会挂起等待。
> - `JoinSet` 与 `TaskTracker` 提供 **结构化并发**：以便追踪、中止和等待任务组。
> - 为所有网络操作添加超时处理 —— `tokio::time::timeout(dur, fut)`。
> - Tower 的 `Service` trait 是 Rust 异步中间件事实上的标准。

> **另请参阅：** [第 8 章 —— Tokio 深度探索](ch08-tokio-deep-dive.md) 了解通道与同步原语，[第 12 章 —— 常见陷阱](ch12-common-pitfalls.md) 了解停机过程中的取消陷阱。

***
