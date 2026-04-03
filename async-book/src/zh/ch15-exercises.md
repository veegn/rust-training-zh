[English Original](../en/ch15-exercises.md)

# 15. 练习题 🟡

> **通过实践巩固所学：**
> - 构建异步 Echo 服务器
> - 带速率限制的并发 URL 获取器
> - 带工作池（Worker Pool）的优雅停机
> - 从零实现简单的异步 Mutex
> - 流（Stream）处理管道
> - 实现带超时的 select

### 练习 1：异步 Echo 服务器

构建一个能并发处理多个客户端的 TCP echo 服务器。

**要求**：
- 监听 `127.0.0.1:8080`
- 接收连接并回显每一行内容
- 优雅地处理客户端断开连接
- 在客户端连接/断开时打印日志

<details>
<summary>🔑 参考方案</summary>

```rust
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("Echo 服务器已在 8080 端口监听");

    loop {
        let (socket, addr) = listener.accept().await?;
        println!("[{addr}] 已连接");

        tokio::spawn(async move {
            let (reader, mut writer) = socket.into_split();
            let mut reader = BufReader::new(reader);
            let mut line = String::new();

            loop {
                line.clear();
                match reader.read_line(&mut line).await {
                    Ok(0) => {
                        println!("[{addr}] 已断开连接");
                        break;
                    }
                    Ok(_) => {
                        print!("[{addr}] 回显: {line}");
                        if writer.write_all(line.as_bytes()).await.is_err() {
                            break;
                        }
                    }
                    Err(e) => {
                        eprintln!("[{addr}] 读取错误: {e}");
                        break;
                    }
                }
            }
        });
    }
}
```

</details>

---

### 练习 2：带限速的并发 URL 获取器

并发获取一组 URL，确保同时进行的请求不超过 5 个。

<details>
<summary>🔑 参考方案</summary>

```rust
use futures::stream::{self, StreamExt};

async fn fetch_urls(urls: Vec<String>) -> Vec<Result<String, String>> {
    // buffer_unordered(5) 确保最多同时轮询 5 个 future —— 
    // 在这里不需要额外的 Semaphore（信号量）。
    let results: Vec<_> = stream::iter(urls)
        .map(|url| {
            async move {
                println!("正在获取: {url}");
                match reqwest::get(&url).await {
                    Ok(resp) => match resp.text().await {
                        Ok(body) => Ok(body),
                        Err(e) => Err(format!("{url}: {e}")),
                    },
                    Err(e) => Err(format!("{url}: {e}")),
                }
            }
        })
        .buffer_unordered(5) // ← 仅此一行即可限制并发数为 5
        .collect()
        .await;

    results
}

// 注意：当你需要限制跨多个独立派生任务（tokio::spawn）的并发时，请使用 Semaphore。
// 在处理流（Stream）时，请直接使用 buffer_unordered。不要为了同一个限制目标混用两者。
```

</details>

---

### 练习 3：带工作池的优雅停机

构建一个包含以下功能的任务处理器：
- 基于通道（channel）的任务队列
- N 个从队列中消费任务的工作任务（worker tasks）
- 在按下 Ctrl+C 时实现优雅停机：停止接收新任务，完成已在进行的任务

<details>
<summary>🔑 参考方案</summary>

```rust
use tokio::sync::{mpsc, watch};
use tokio::time::{sleep, Duration};

struct WorkItem { id: u64, payload: String }

#[tokio::main]
async fn main() {
    let (work_tx, work_rx) = mpsc::channel::<WorkItem>(100);
    let (shutdown_tx, shutdown_rx) = watch::channel(false);

    // 派生 4 个工作单元
    let mut worker_handles = Vec::new();
    let work_rx = std::sync::Arc::new(tokio::sync::Mutex::new(work_rx));

    for id in 0..4 {
        let rx = work_rx.clone();
        let mut shutdown = shutdown_rx.clone();
        let handle = tokio::spawn(async move {
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
                        println!("工作单元 {id}: 处理中 {}", work.id);
                        sleep(Duration::from_millis(200)).await; // 模拟耗时操作
                    }
                    None => break,
                }
            }
        });
        worker_handles.push(handle);
    }

    // 生产者：提交一些任务
    let producer = tokio::spawn(async move {
        for i in 0..20 {
            let _ = work_tx.send(WorkItem { id: i, payload: "...".into() }).await;
            sleep(Duration::from_millis(50)).await;
        }
    });

    // 等待 Ctrl+C
    tokio::signal::ctrl_c().await.unwrap();
    println!("\n接收到停机信号！");
    shutdown_tx.send(true).unwrap();
    producer.abort(); // 取消生产者任务

    // 等待所有工作单元完成
    for handle in worker_handles { let _ = handle.await; }
    println!("所有工作单元均已停止。再见！");
}
```

</details>

---

### 练习 4：从零实现简单的异步 Mutex

利用通道实现一个异步感知的 Mutex（不直接使用 `tokio::sync::Mutex`）。

*提示*：使用容量为 1 的 `tokio::sync::mpsc` 通道作为信号量。

<details>
<summary>🔑 参考方案</summary>

```rust
use std::cell::UnsafeCell;
use std::sync::Arc;
use tokio::sync::{OwnedSemaphorePermit, Semaphore};

pub struct SimpleAsyncMutex<T> {
    data: Arc<UnsafeCell<T>>,
    semaphore: Arc<Semaphore>,
}

pub struct SimpleGuard<T> {
    data: Arc<UnsafeCell<T>>,
    _permit: OwnedSemaphorePermit, // 丢弃 guard 时会释放锁
}

impl<T> SimpleAsyncMutex<T> {
    pub fn new(value: T) -> Self {
        SimpleAsyncMutex {
            data: Arc::new(UnsafeCell::new(value)),
            semaphore: Arc::new(Semaphore::new(1)),
        }
    }

    pub async fn lock(&self) -> SimpleGuard<T> {
        let permit = self.semaphore.clone().acquire_owned().await.unwrap();
        SimpleGuard {
            data: self.data.clone(),
            _permit: permit,
        }
    }
}

// 还要实现 Deref 和 DerefMut...
```

**核心总结**：异步 Mutex 通常构建在信号量之上。信号量提供了异步等待机制 —— 当锁定时，`acquire()` 会挂起任务直到有空出的许可证。这正是 `tokio::sync::Mutex` 的内部工作原理。

</details>

---

### 练习 5：流处理管道

使用流（Stream）构建数据处理管道：
1. 生成数字 1 到 100
2. 过滤出偶数
3. 对每个数字求平方
4. 并发处理：一次处理 10 个（使用 sleep 模拟耗时异步操作）
5. 收集结果

<details>
<summary>🔑 参考方案</summary>

```rust
use futures::stream::{self, StreamExt};
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    let results: Vec<u64> = stream::iter(1u64..=100)
        .filter(|x| futures::future::ready(x % 2 == 0))
        .map(|x| x * x)
        .map(|x| async move {
            sleep(Duration::from_millis(50)).await;
            println!("已处理: {x}");
            x
        })
        .buffer_unordered(10) // 10 路并发
        .collect()
        .await;

    println!("得到 {} 个结果，总和为 {}", results.len(), results.iter().sum::<u64>());
}
```

</details>

---

### 练习 6：实现带超时的 Select

在不直接使用 `tokio::select!` 或 `tokio::time::timeout` 的前提下，实现一个函数让 Future 与某个截止时间竞速，并在超时后返回 `Either::Right(())`。

*提示*：基于第 6 章的 `Select` 组合器和 `TimerFuture` 实现。

<details>
<summary>🔑 参考方案</summary>

```rust
pub enum Either<A, B> { Left(A), Right(B) }

pub struct Timeout<F> {
    future: F,
    timer: TimerFuture, // 来自第 6 章
}

impl<F: Future + Unpin> Future for Timeout<F> {
    type Output = Either<F::Output, ()>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if let Poll::Ready(val) = Pin::new(&mut self.future).poll(cx) {
            return Poll::Ready(Either::Left(val));
        }
        if let Poll::Ready(()) = Pin::new(&mut self.timer).poll(cx) {
            return Poll::Ready(Either::Right(()));
        }
        Poll::Pending
    }
}
```

***
