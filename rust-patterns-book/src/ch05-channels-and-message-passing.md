# 5. Channels and Message Passing / 5. 通道与消息传递 🟢

> **What you'll learn / 你将学到：**
> - `std::sync::mpsc` basics and when to upgrade to crossbeam-channel / `std::sync::mpsc` 的基础知识以及何时升级到 crossbeam-channel
> - Channel selection with `select!` for multi-source message handling / 使用 `select!` 进行多源消息处理的通道选择
> - Bounded vs unbounded channels and backpressure strategies / 有界与无界通道以及背压 (Backpressure) 策略
> - The actor pattern for encapsulating concurrent state / 用于封装并发状态的 Actor 模式

## std::sync::mpsc — The Standard Channel / std::sync::mpsc —— 标准通道

Rust's standard library provides a multi-producer, single-consumer channel:

Rust 的标准库提供了一个多生产者、单消费者 (MPSC) 的通道：

```rust
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    // Create a channel: tx (transmitter) and rx (receiver)
    // 创建一个通道：tx（发送端）和 rx（接收端）
    let (tx, rx) = mpsc::channel();

    // Spawn a producer thread
    // 启动一个生产者线程
    let tx1 = tx.clone(); // Clone for multiple producers / 为多个生产者克隆发送端
    thread::spawn(move || {
        for i in 0..5 {
            tx1.send(format!("producer-1: msg {i}")).unwrap();
            thread::sleep(Duration::from_millis(100));
        }
    });

    // Second producer
    // 第二个生产者
    thread::spawn(move || {
        for i in 0..5 {
            tx.send(format!("producer-2: msg {i}")).unwrap();
            thread::sleep(Duration::from_millis(150));
        }
    });

    // Consumer: receive all messages
    // 消费者：接收所有消息
    for msg in rx {
        // rx iterator ends when ALL senders are dropped
        // rx 迭代器在所有发送端都被丢弃 (Drop) 时结束
        println!("Received: {msg}");
    }
    println!("All producers done.");
}
```

> **Note:** `.unwrap()` on `.send()` is used for brevity. It panics if the receiver has been dropped. Production code should handle `SendError` gracefully.
>
> **注意**：在 `.send()` 上使用 `.unwrap()` 是为了简洁。如果接收端已被丢弃 (Drop)，它会产生恐慌。生产代码应当优雅地处理 `SendError`。

**Key properties / 关键特性**:
- **Unbounded / 无界** by default (can fill memory if consumer is slow) / 默认是无界的（如果消费者过慢，可能会填满内存）
- `mpsc::sync_channel(N)` creates a **bounded / 有界** channel with backpressure / `mpsc::sync_channel(N)` 创建一个带有背压的有界通道
- `rx.recv()` blocks the current thread until a message arrives / `rx.recv()` 阻塞当前线程直到消息到达
- `rx.try_recv()` returns immediately with `Err(TryRecvError::Empty)` if nothing is ready / `rx.try_recv()` 在没有就绪消息时立即返回 `Err(TryRecvError::Empty)`
- The channel closes when all `Sender`s are dropped / 当所有发送端 (`Sender`) 都被丢弃时，通道关闭

```rust
// Bounded channel with backpressure:
// 带有背压的有界通道：
let (tx, rx) = mpsc::sync_channel(10); // Buffer of 10 messages / 缓冲区容量为 10 条消息

thread::spawn(move || {
    for i in 0..1000 {
        tx.send(i).unwrap(); // BLOCKS if buffer is full — natural backpressure
                             // 如果缓冲区已满，则阻塞 —— 产生自然的背压
    }
});
```

> **Note:** `.unwrap()` is used for brevity. In production, handle `SendError` (receiver dropped) instead of panicking.
>
> **注意**：此处使用 `.unwrap()` 是为了简洁。在生产环境中，请处理 `SendError`（接收端已丢弃）而不是直接产生恐慌。

### crossbeam-channel — The Production Workhorse / crossbeam-channel —— 生产环境的主力军

`crossbeam-channel` is the de facto standard for production channel usage. It's faster than `std::sync::mpsc` and supports multi-consumer (`mpmc`):

`crossbeam-channel` 是生产环境中通道使用的事实标准。它比 `std::sync::mpsc` 更快，并且支持多消费者模式 (MPMC)：

```rust,ignore
// Cargo.toml:
//   [dependencies]
//   crossbeam-channel = "0.5"
use crossbeam_channel::{bounded, unbounded, select, Sender, Receiver};
use std::thread;
use std::time::Duration;

fn main() {
    // Bounded MPMC channel
    // 有界 MPMC 通道
    let (tx, rx) = bounded::<String>(100);

    // Multiple producers
    // 多个生产者
    for id in 0..4 {
        let tx = tx.clone();
        thread::spawn(move || {
            for i in 0..10 {
                tx.send(format!("worker-{id}: item-{i}")).unwrap();
            }
        });
    }
    drop(tx); // Drop the original sender so the channel can close
              // 丢弃原始发送端，以便通道能够关闭

    // Multiple consumers (not possible with std::sync::mpsc!)
    // 多个消费者（这在 std::sync::mpsc 中是不可能的！）
    let rx2 = rx.clone();
    let consumer1 = thread::spawn(move || {
        while let Ok(msg) = rx.recv() {
            println!("[consumer-1] {msg}");
        }
    });
    let consumer2 = thread::spawn(move || {
        while let Ok(msg) = rx2.recv() {
            println!("[consumer-2] {msg}");
        }
    });

    consumer1.join().unwrap();
    consumer2.join().unwrap();
}
```

### Channel Selection (select!) / 通道选择 (select!)

Listen on multiple channels simultaneously — like `select` in Go:

同时监听多个通道 —— 类似于 Go 语言中的 `select`：

```rust,ignore
use crossbeam_channel::{bounded, tick, after, select};
use std::time::Duration;

fn main() {
    let (work_tx, work_rx) = bounded::<String>(10);
    let ticker = tick(Duration::from_secs(1));        // Periodic tick / 周期性 Tick
    let deadline = after(Duration::from_secs(10));     // One-shot timeout / 一次性超时

    // Producer
    // 生产者
    let tx = work_tx.clone();
    std::thread::spawn(move || {
        for i in 0..100 {
            tx.send(format!("job-{i}")).unwrap();
            std::thread::sleep(Duration::from_millis(500));
        }
    });
    drop(work_tx);

    loop {
        select! {
            recv(work_rx) -> msg => {
                match msg {
                    Ok(job) => println!("Processing: {job}"), // 正在处理
                    Err(_) => {
                        println!("Work channel closed"); // 工作通道已关闭
                        break;
                    }
                }
            },
            recv(ticker) -> _ => {
                println!("Tick — heartbeat"); // Tick —— 心跳
            },
            recv(deadline) -> _ => {
                println!("Deadline reached — shutting down"); // 截止时间已到 —— 正在关闭
                break;
            },
        }
    }
}
```

> **Go comparison / 与 Go 的对比**: This is exactly like Go's `select` statement over channels. crossbeam's `select!` macro randomizes order to prevent starvation, just like Go.
>
> 这与 Go 语言在通道上的 `select` 语句完全相同。crossbeam 的 `select!` 宏会自动随机化执行顺序以此来防止饥饿 (Starvation) 现象，这同样与 Go 的行为一致。

### Bounded vs Unbounded and Backpressure / 有界与无界及背压

| Type / 类型 | Behavior When Full / 满载时的行为 | Memory / 内存 | Use Case / 使用场景 |
|------|-------------------|--------|----------|
| **Unbounded / 无界** | Never blocks (grows heap) / 永不阻塞（堆增长） | Unbounded ⚠️ / 无限制 ⚠️ | Rare — only when producer is slower than consumer / 罕见 —— 仅当生产者慢于消费者时 |
| **Bounded / 有界** | `send()` blocks until space / 阻塞直到有空间 | Fixed / 固定 | Production default — prevents OOM / 生产环境默认 —— 防止内存溢出 (OOM) |
| **Rendezvous / 交汇** (bounded(0)) | `send()` blocks until receiver is ready / 阻塞直到接收者准备就绪 | None / 无 | Synchronization / handoff / 同步或移交 |

```rust
// Rendezvous channel — zero capacity, direct handoff
// 交汇通道 —— 零容量，直接移交
let (tx, rx) = crossbeam_channel::bounded(0);
// tx.send(x) blocks until rx.recv() is called, and vice versa.
// This synchronizes the two threads precisely.

// tx.send(x) 会阻塞直到 rx.recv() 被调用，反之亦然。
// 这可以在两个线程之间实现精确同步。
```

**Rule / 规则**: Always use bounded channels in production unless you can prove the producer will never outpace the consumer.

**规则**：除非你能证明生产者永远不会超过消费者的处理速度，否则请在生产环境中始终使用有界通道。

### Actor Pattern with Channels / 使用通道的 Actor 模式

The actor pattern uses channels to serialize access to mutable state — no mutexes needed:

Actor 模式利用通道来序列化对可变状态的访问 —— 无需使用互斥锁 (Mutex)：

```rust
use std::sync::mpsc;
use std::thread;

// Messages the actor can receive
// Actor 可以接收的消息
enum CounterMsg {
    Increment,
    Decrement,
    Get(mpsc::Sender<i64>), // Reply channel / 用于回复的通道
}

struct CounterActor {
    count: i64,
    rx: mpsc::Receiver<CounterMsg>,
}

impl CounterActor {
    fn new(rx: mpsc::Receiver<CounterMsg>) -> Self {
        CounterActor { count: 0, rx }
    }

    fn run(mut self) {
        while let Ok(msg) = self.rx.recv() {
            match msg {
                CounterMsg::Increment => self.count += 1,
                CounterMsg::Decrement => self.count -= 1,
                CounterMsg::Get(reply) => {
                    let _ = reply.send(self.count);
                }
            }
        }
    }
}

// Actor handle — cheap to clone, Send + Sync
// Actor 句柄 —— 克隆成本低，支持 Send + Sync
#[derive(Clone)]
struct Counter {
    tx: mpsc::Sender<CounterMsg>,
}

impl Counter {
    fn spawn() -> Self {
        let (tx, rx) = mpsc::channel();
        thread::spawn(move || CounterActor::new(rx).run());
        Counter { tx }
    }

    fn increment(&self) { let _ = self.tx.send(CounterMsg::Increment); }
    fn decrement(&self) { let _ = self.tx.send(CounterMsg::Decrement); }

    fn get(&self) -> i64 {
        let (reply_tx, reply_rx) = mpsc::channel();
        self.tx.send(CounterMsg::Get(reply_tx)).unwrap();
        reply_rx.recv().unwrap()
    }
}

fn main() {
    let counter = Counter::spawn();

    // Multiple threads can safely use the counter — no mutex!
    // 多个线程可以安全地使用计数器 —— 无需互斥锁！
    let handles: Vec<_> = (0..10).map(|_| {
        let counter = counter.clone();
        thread::spawn(move || {
            for _ in 0..1000 {
                counter.increment();
            }
        })
    }).collect();

    for h in handles { h.join().unwrap(); }
    println!("Final count: {}", counter.get()); // 10000
}
```

> **When to use actors vs mutexes / 何时使用 Actor 而非互斥锁**: Actors are great when the state has complex invariants, operations take a long time, or you want to serialize access without thinking about lock ordering. Mutexes are simpler for short critical sections.
>
> **何时使用 Actor 而非互斥锁**：当状态具有复杂的不可变式 (Invariants)、操作耗时较长、或者你希望无需考虑锁顺序 (Lock Ordering) 就实现序列化访问时，Actor 是绝佳选择。而对于简短的临界区 (Critical Sections)，互斥锁则更为简单。

> **Key Takeaways — Channels / 核心要点 —— 通道**
> - `crossbeam-channel` is the production workhorse — faster and more feature-rich than `std::sync::mpsc` / `crossbeam-channel` 是生产环境中的主力军 —— 比 `std::sync::mpsc` 更快且功能更丰富
> - `select!` replaces complex multi-source polling with declarative channel selection / `select!` 通过声明式的通道选择，取代了复杂的多个源的轮询
> - Bounded channels provide natural backpressure; unbounded channels risk OOM / 有界通道提供自然的背压；无界通道则存在内存溢出 (OOM) 的风险

> **See also / 另请参阅：** [Ch 6 — Concurrency](ch06-concurrency-vs-parallelism-vs-threads.md) for threads, Mutex, and shared state. [Ch 15 — Async](ch15-asyncawait-essentials.md) for async channels (`tokio::sync::mpsc`).
>
> 参见 [Ch 6 —— 并发](ch06-concurrency-vs-parallelism-vs-threads.md) 了解线程、互斥锁和共享状态。参见 [Ch 15 —— Async](ch15-asyncawait-essentials.md) 了解异步通道 (`tokio::sync::mpsc`)。

---

### Exercise: Channel-Based Worker Pool ★★★ (~45 min) / 练习：基于通道的工作池 ★★★（约 45 分钟）

Build a worker pool using channels where:
- A dispatcher sends `Job` structs through a channel
- N workers consume jobs and send results back
- Use `std::sync::mpsc` with `Arc<Mutex<Receiver>>` for work-stealing

构建一个使用通道的工作池，其中：
- 调度器 (Dispatcher) 通过通道发送 `Job` 结构体
- N 个工作者 (Workers) 消费这些任务并将结果发回
- 使用 `std::sync::mpsc` 配合 `Arc<Mutex<Receiver>>` 实现任务窃取 (Work-stealing) 机制

<details>
<summary>🔑 Solution / 参考答案</summary>

```rust
use std::sync::mpsc;
use std::thread;

struct Job {
    id: u64,
    data: String,
}

struct JobResult {
    job_id: u64,
    output: String,
    worker_id: usize,
}

fn worker_pool(jobs: Vec<Job>, num_workers: usize) -> Vec<JobResult> {
    let (job_tx, job_rx) = mpsc::channel::<Job>();
    let (result_tx, result_rx) = mpsc::channel::<JobResult>();

    // Arc<Mutex<_>> allows sharing the single Receiver among all workers
    // Arc<Mutex<_>> 允许在所有工作者之间共享单个接收端
    let job_rx = std::sync::Arc::new(std::sync::Mutex::new(job_rx));

    let mut handles = Vec::new();
    for worker_id in 0..num_workers {
        let job_rx = job_rx.clone();
        let result_tx = result_tx.clone();
        handles.push(thread::spawn(move || {
            loop {
                // Workers compete for the lock to receive a job
                // 工作者通过竞争锁来接收任务
                let job = {
                    let rx = job_rx.lock().unwrap();
                    rx.recv()
                };
                match job {
                    Ok(job) => {
                        let output = format!("processed '{}' by worker {worker_id}", job.data);
                        result_tx.send(JobResult {
                            job_id: job.id, output, worker_id,
                        }).unwrap();
                    }
                    Err(_) => break, // Channel closed / 通道已关闭
                }
            }
        }));
    }
    // Very important: drop the result_tx in the dispatcher thread
    // Otherwise result_rx.into_iter() will never end!
    
    // 非常重要：在调度器线程中丢弃 result_tx
    // 否则 result_rx.into_iter() 永远不会结束！
    drop(result_tx);

    let num_jobs = jobs.len();
    for job in jobs {
        job_tx.send(job).unwrap();
    }
    drop(job_tx); // Signals workers to exit when done / 通知工作者完成后退出

    let results: Vec<_> = result_rx.into_iter().collect();
    assert_eq!(results.len(), num_jobs);

    for h in handles { h.join().unwrap(); }
    results
}

fn main() {
    let jobs: Vec<Job> = (0..20).map(|i| Job {
        id: i, data: format!("task-{i}"),
    }).collect();

    let results = worker_pool(jobs, 4);
    for r in &results {
        println!("[worker {}] job {}: {}", r.worker_id, r.job_id, r.output);
    }
}
```

</details>

***

