# Exercises 🟡

### Exercise 1: Async Echo Server

Build a TCP echo server that handles multiple clients concurrently.

**Requirements**:
- Listen on `127.0.0.1:8080`
- Accept connections and echo back each line
- Handle client disconnections gracefully
- Print a log when clients connect/disconnect

<details>
<summary>🔑 Solution</summary>

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

### Exercise 2: Concurrent URL Fetcher with Rate Limiting

Fetch a list of URLs concurrently, with at most 5 concurrent requests.

<details>
<summary>🔑 Solution</summary>

```rust
use futures::stream::{self, StreamExt};
use tokio::time::{sleep, Duration};

async fn fetch_urls(urls: Vec<String>) -> Vec<Result<String, String>> {
    // buffer_unordered(5) ensures at most 5 futures are polled
    // concurrently — no separate Semaphore needed here.
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
        .buffer_unordered(5) // ← This alone limits concurrency to 5
        .collect()
        .await;

    results
}

// NOTE: Use Semaphore when you need to limit concurrency across
// independently spawned tasks (tokio::spawn). Use buffer_unordered
// when processing a stream. Don't combine both for the same limit.
```

</details>

---

### Exercise 3: Graceful Shutdown with Worker Pool

Build a task processor with:
- A channel-based work queue
- N worker tasks consuming from the queue
- Graceful shutdown on Ctrl+C: stop accepting, finish in-flight work

---

### Exercise 4: Build a Simple Async Mutex from Scratch

Implement an async-aware mutex using channels (without using `tokio::sync::Mutex`).

*Hint*: You can use a `tokio::sync::mpsc` channel of capacity 1 as a semaphore.

<details>
<summary>🔑 Solution</summary>

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

</details>

---

### Exercise 5: Stream Pipeline

Build a data processing pipeline using streams:
1. Generate numbers 1..=100
2. Filter to even numbers
3. Map each to its square
4. Process 10 at a time concurrently
5. Collect results

---

### Exercise 6: Implement Select with Timeout

Without using `tokio::select!` or `tokio::time::timeout`, implement a function that races a future against a deadline.

<details>
<summary>🔑 Solution</summary>

```rust
impl<F: Future + Unpin> Future for Timeout<F> {
    type Output = Either<F::Output, ()>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // Check if the main future is done
        if let Poll::Ready(val) = Pin::new(&mut self.future).poll(cx) {
            return Poll::Ready(Either::Left(val));
        }

        // Check if the timer expired
        if let Poll::Ready(()) = Pin::new(&mut self.timer).poll(cx) {
            return Poll::Ready(Either::Right(()));
        }

        Poll::Pending
    }
}
```

**Key takeaway**: `select`/`timeout` is just polling two futures and seeing which completes first.

</details>

***
