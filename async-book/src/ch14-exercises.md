# Exercises / 练习

### Exercise 1: Async Echo Server / 练习 1：异步 Echo 服务器

Build a TCP echo server that handles multiple clients concurrently.

构建一个可以并发处理多个客户端的 TCP echo（回显）服务器。

**Requirements / 要求**：
- Listen on `127.0.0.1:8080` / 监听 `127.0.0.1:8080`
- Accept connections and echo back each line / 接收连接并回显每一行内容
- Handle client disconnections gracefully / 优雅地处理客户端断开连接
- Print a log when clients connect/disconnect / 在客户端连接/断开时打印日志

<details>
<summary>🔑 Solution / 参考答案</summary>

```rust
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("Echo server listening on :8080");

    loop {
        let (socket, addr) = listener.accept().await?;
        println!("[{addr}] Connected");

        tokio::spawn(async move {
            let (reader, mut writer) = socket.into_split();
            let mut reader = BufReader::new(reader);
            let mut line = String::new();

            loop {
                line.clear();
                match reader.read_line(&mut line).await {
                    Ok(0) => {
                        println!("[{addr}] Disconnected");
                        break;
                    }
                    Ok(_) => {
                        print!("[{addr}] Echo: {line}");
                        if writer.write_all(line.as_bytes()).await.is_err() {
                            println!("[{addr}] Write error, disconnecting");
                            break;
                        }
                    }
                    Err(e) => {
                        eprintln!("[{addr}] Read error: {e}");
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

### Exercise 2: Concurrent URL Fetcher with Rate Limiting / 练习 2：带限流的并发 URL 抓取器

Fetch a list of URLs concurrently, with at most 5 concurrent requests.

并发抓取一组 URL，且限制最多同时进行 5 个并发请求。

<details>
<summary>🔑 Solution / 参考答案</summary>

```rust
use futures::stream::{self, StreamExt};
use tokio::time::{sleep, Duration};

async fn fetch_urls(urls: Vec<String>) -> Vec<Result<String, String>> {
    // buffer_unordered(5) ensures at most 5 futures are polled
    // concurrently — no separate Semaphore needed here.
    // buffer_unordered(5) 确保最多同时轮询 5 个 future
    // —— 此处不需要额外的信号量 (Semaphore)。
    let results: Vec<_> = stream::iter(urls)
        .map(|url| {
            async move {
                println!("Fetching: {url}");

                match reqwest::get(&url).await {
                    Ok(resp) => match resp.text().await {
                        Ok(body) => Ok(body),
                        Err(e) => Err(format!("{url}: {e}")),
                    },
                    Err(e) => Err(format!("{url}: {e}")),
                }
            }
        })
        .buffer_unordered(5) // ← This alone limits concurrency to 5 / 仅靠此项即可限制并发数为 5
        .collect()
        .await;

    results
}

// NOTE: Use Semaphore when you need to limit concurrency across
// independently spawned tasks (tokio::spawn). Use buffer_unordered
// when processing a stream. Don't combine both for the same limit.
// 注意：如果需要限制独立派生任务 (tokio::spawn) 的并发，请使用信号量。
// 如果处理流，请使用 buffer_unordered。不要为了同一个限制而混用两者。
```

</details>

---

### Exercise 3: Graceful Shutdown with Worker Pool / 练习 3：带有工作池的优雅停机

Build a task processor with:
- A channel-based work queue
- N worker tasks consuming from the queue
- Graceful shutdown on Ctrl+C: stop accepting, finish in-flight work

构建一个任务处理器，包含：
- 一个基于通道的工作队列
- N 个从队列中提取任务的工作任务
- 在收到 Ctrl+C 时实现优雅停机：停止接收新任务，并完成正在进行的工作

---

### Exercise 4: Build a Simple Async Mutex from Scratch / 练习 4：从零开始构建一个简单的异步 Mutex

Implement an async-aware mutex using channels (without using `tokio::sync::Mutex`).

使用通道实现一个支持异步的 Mutex（不要直接使用 `tokio::sync::Mutex`）。

*Hint / 提示*：可以使用容量为 1 的 `tokio::sync::mpsc` 通道作为信号量。

<details>
<summary>🔑 Solution / 参考答案</summary>

```rust
use std::cell::UnsafeCell;
use std::sync::Arc;
use tokio::sync::{OwnedSemaphorePermit, Semaphore};

pub struct SimpleAsyncMutex<T> {
    data: Arc<UnsafeCell<T>>,
    semaphore: Arc<Semaphore>,
}

// SAFETY: Access to T is serialized by the semaphore (max 1 permit).
unsafe impl<T: Send> Send for SimpleAsyncMutex<T> {}
unsafe impl<T: Send> Sync for SimpleAsyncMutex<T> {}

pub struct SimpleGuard<T> {
    data: Arc<UnsafeCell<T>>,
    _permit: OwnedSemaphorePermit, // Dropped on guard drop → releases lock
                                    // Guard 释放时 drop，从而释放锁
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

impl<T> std::ops::Deref for SimpleGuard<T> {
    type Target = T;
    fn deref(&self) -> &T {
        // SAFETY: We hold the only semaphore permit.
        // 安全性：我们持有唯一的信号量许可。
        unsafe { &*self.data.get() }
    }
}

impl<T> std::ops::DerefMut for SimpleGuard<T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut *self.data.get() }
    }
}
```

**Key takeaway**: Async mutexes are typically built on top of semaphores. The semaphore provides the async wait mechanism.

**关键要点**：异步 Mutex 通常构建在信号量之上。信号量提供了异步等待机制。

</details>

---

### Exercise 5: Stream Pipeline / 练习 5：流流水线

Build a data processing pipeline using streams:
1. Generate numbers 1..=100
2. Filter to even numbers
3. Map each to its square
4. Process 10 at a time concurrently
5. Collect results

使用流构建一个数据处理流水线：
1. 生成数字 1..=100
2. 过滤出偶数
3. 将每个数映射为其平方
4. 同时并发处理 10 个项
5. 收集结果

---

### Exercise 6: Implement Select with Timeout / 练习 6：实现带超时的 Select

Without using `tokio::select!` or `tokio::time::timeout`, implement a function that races a future against a deadline.

不使用 `tokio::select!` 或 `tokio::time::timeout`，实现一个让 future 与截止时间进行竞速的函数。

<details>
<summary>🔑 Solution / 参考答案</summary>

```rust
impl<F: Future + Unpin> Future for Timeout<F> {
    type Output = Either<F::Output, ()>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // Check if the main future is done / 检查主 future 是否完成
        if let Poll::Ready(val) = Pin::new(&mut self.future).poll(cx) {
            return Poll::Ready(Either::Left(val));
        }

        // Check if the timer expired / 检查定时器是否到期
        if let Poll::Ready(()) = Pin::new(&mut self.timer).poll(cx) {
            return Poll::Ready(Either::Right(()));
        }

        Poll::Pending
    }
}
```

**Key takeaway**: `select`/`timeout` is just polling two futures and seeing which completes first.

**关键要点**：`select`/`timeout` 本质上就是轮询两个 future 并观察哪一个先完成。

</details>

***

