[English Original](../en/ch05-channels-and-message-passing.md)

# 第 5 章：信道 (Channels) 与消息传递 🟢

> **你将学到：**
> - `std::sync::mpsc` 基础，以及何时升级到 `crossbeam-channel`
> - 使用 `select!` 进行多源消息处理的信道选择
> - 有界与无界信道及背压 (Backpressure) 策略
> - 用于封装并发状态的 Actor 模式

## std::sync::mpsc —— 标准信道

Rust 标准库提供了一个多生产者、单消费者 (MPSC) 信道：

```rust
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    // 创建一个信道：tx (发送者) 和 rx (接收者)
    let (tx, rx) = mpsc::channel();

    // 派生一个生产者线程
    let tx1 = tx.clone(); // 为多个生产者克隆发送者
    thread::spawn(move || {
        for i in 0..5 {
            tx1.send(format!("生产者-1: 消息 {i}")).unwrap();
            thread::sleep(Duration::from_millis(100));
        }
    });

    // 第二个生产者
    thread::spawn(move || {
        for i in 0..5 {
            tx.send(format!("生产者-2: 消息 {i}")).unwrap();
            thread::sleep(Duration::from_millis(150));
        }
    });

    // 消费者：接收所有消息
    for msg in rx {
        // 当所有发送者都被 drop 时，rx 迭代器结束
        println!("收到: {msg}");
    }
    println!("所有生产者已完成。");
}
```

> **注意**：为了简明起见，我们在 `.send()` 上使用了 `.unwrap()`。如果接收者已被 drop，它会触发 panic。生产环境代码应优雅地处理 `SendError`。

**关键属性**：
- 默认是 **无界 (Unbounded)** 的（如果消费者速度较慢，可能会填满内存）
- `mpsc::sync_channel(N)` 创建一个带有背压的 **有界 (Bounded)** 信道
- `rx.recv()` 会阻塞当前线程，直到有消息到达
- 如果没有任何消息准备就绪，`rx.try_recv()` 会立即返回 `Err(TryRecvError::Empty)`
- 当所有 `Sender` 都被 drop 时，信道关闭

```rust
// 带有背压的有界信道：
let (tx, rx) = mpsc::sync_channel(10); // 缓冲区容量为 10 条消息

thread::spawn(move || {
    for i in 0..1000 {
        tx.send(i).unwrap(); // 如果缓冲区已满则阻塞 —— 实现自然的背压
    }
});
```

> **注意**：为了简明起见使用了 `.unwrap()`。在生产环境中，请处理 `SendError`（接收者已 drop）而不是直接 panic。

### crossbeam-channel —— 工业级利器

`crossbeam-channel` 是生产环境中信道使用的行业标准。它比 `std::sync::mpsc` 更快，且支持多消费者 (MPMC)：

```rust,ignore
// Cargo.toml:
//   [dependencies]
//   crossbeam-channel = "0.5"
use crossbeam_channel::{bounded, unbounded, select, Sender, Receiver};
use std::thread;
use std::time::Duration;

fn main() {
    // 有界 MPMC 信道
    let (tx, rx) = bounded::<String>(100);

    // 多个生产者
    for id in 0..4 {
        let tx = tx.clone();
        thread::spawn(move || {
            for i in 0..10 {
                tx.send(format!("工作者-{id}: 条目-{i}")).unwrap();
            }
        });
    }
    drop(tx); // drop 原始发送者，以便信道可以关闭

    // 多个消费者（这在 std::sync::mpsc 中是不可能的！）
    let rx2 = rx.clone();
    let consumer1 = thread::spawn(move || {
        while let Ok(msg) = rx.recv() {
            println!("[消费者-1] {msg}");
        }
    });
    let consumer2 = thread::spawn(move || {
        while let Ok(msg) = rx2.recv() {
            println!("[消费者-2] {msg}");
        }
    });

    consumer1.join().unwrap();
    consumer2.join().unwrap();
}
```

### 信道选择 (select!)

同时监听多个信道 —— 类似于 Go 中的 `select`：

```rust,ignore
use crossbeam_channel::{bounded, tick, after, select};
use std::time::Duration;

fn main() {
    let (work_tx, work_rx) = bounded::<String>(10);
    let ticker = tick(Duration::from_secs(1));        // 周期性滴答
    let deadline = after(Duration::from_secs(10));     // 一次性超时

    // 生产者
    let tx = work_tx.clone();
    std::thread::spawn(move || {
        for i in 0..100 {
            tx.send(format!("任务-{i}")).unwrap();
            std::thread::sleep(Duration::from_millis(500));
        }
    });
    drop(work_tx);

    loop {
        select! {
            recv(work_rx) -> msg => {
                match msg {
                    Ok(job) => println!("处理中: {job}"),
                    Err(_) => {
                        println!("工作信道已关闭");
                        break;
                    }
                }
            },
            recv(ticker) -> _ => {
                println!("滴答 — 心跳");
            },
            recv(deadline) -> _ => {
                println!("截止时间已到 — 正在关闭");
                break;
            },
        }
    }
}
```

> **与 Go 语言对比**：这与 Go 语言中跨信道的 `select` 语句完全一致。crossbeam 的 `select!` 宏会通过随机化顺序来防止饥饿，这一点也与 Go 相同。

### 有界 vs 无界与背压 (Backpressure)

| 类型 | 缓冲区满时的行为 | 内存占用 | 使用场景 |
|------|-------------------|--------|----------|
| **无界 (Unbounded)** | 从不阻塞（在堆上增长） | 无限制 ⚠️ | 罕见 —— 仅当生产者速度明显慢于消费者时使用 |
| **有界 (Bounded)** | `send()` 会阻塞直到有空位 | 固定 | 生产环境默认选择 —— 防止内存溢出 (OOM) |
| **会合 (Rendezvous)** (bounded(0)) | `send()` 阻塞直到有接收者就绪 | 无 | 用于同步或移交 (Handoff) |

```rust
// 会合信道 —— 零容量，直接移交
let (tx, rx) = crossbeam_channel::bounded(0);
// tx.send(x) 会阻塞直到有人调用 rx.recv()，反之亦然。
// 这能让两个线程实现精确同步。
```

**规则**：生产环境中应始终使用有界信道，除非你能证明生产者绝对不会超过消费者的处理速度。

### 使用信道的 Actor 模式

Actor 模式利用信道来串行化对可变状态的访问 —— 这种方式不需要互斥锁：

```rust
use std::sync::mpsc;
use std::thread;

// Actor 能够接收的消息类型
enum CounterMsg {
    Increment,
    Decrement,
    Get(mpsc::Sender<i64>), // 响应信道
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

// Actor 句柄 —— 克隆成本低，且支持 Send + Sync
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
    println!("最终计数值: {}", counter.get()); // 10000
}
```

> **何时使用 Actor vs 互斥锁**：当状态具有复杂的不变量、操作耗时较长，或者你希望在不考虑加锁顺序的情况下串行化访问时，Actor 是非常出色的选择。互斥锁则在处理简单的临界区时更为直接。

> **关键要点 —— 信道**
> - `crossbeam-channel` 是生产环境中的主力 —— 它比 `std::sync::mpsc` 更快且功能更丰富。
> - `select!` 取代了复杂的多元轮询，提供声明式的信道选择。
> - 有界信道提供自然的背压；无界信道存在内存溢出 (OOM) 风险。

> **另请参阅：** [第 6 章 —— 并发](ch06-concurrency-vs-parallelism-vs-threads.md) 了解线程、Mutex 和共享状态。[第 15 章 —— 异步](ch16-asyncawait-essentials.md) 了解异步信道 (`tokio::sync::mpsc`)。

***

### 练习：基于信道的工作者池 (Worker Pool) ★★★ (~45 分钟)

构建一个使用信道的工作者池，要求如下：
- 调度器通过信道发送 `Job` 结构体
- N 个工作者消耗任务并将结果发回
- 使用 `std::sync::mpsc` 配合 `Arc<Mutex<Receiver>>` 来实现共享的工作队列

<details>
<summary>🔑 参考答案</summary>

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

    let job_rx = std::sync::Arc::new(std::sync::Mutex::new(job_rx));

    let mut handles = Vec::new();
    for worker_id in 0..num_workers {
        let job_rx = job_rx.clone();
        let result_tx = result_tx.clone();
        handles.push(thread::spawn(move || {
            loop {
                let job = {
                    let rx = job_rx.lock().unwrap();
                    rx.recv()
                };
                match job {
                    Ok(job) => {
                        let output = format!("由工作者 {worker_id} 处理了 '{}'", job.data);
                        result_tx.send(JobResult {
                            job_id: job.id, output, worker_id,
                        }).unwrap();
                    }
                    Err(_) => break, // 信道已关闭
                }
            }
        }));
    }
    drop(result_tx); // 必须在发送端全部 drop 之后，rx 的迭代器才会停止

    let num_jobs = jobs.len();
    for job in jobs {
        job_tx.send(job).unwrap();
    }
    drop(job_tx); // 关闭工作队列

    let results: Vec<_> = result_rx.into_iter().collect();
    assert_eq!(results.len(), num_jobs);

    for h in handles { h.join().unwrap(); }
    results
}

fn main() {
    let jobs: Vec<Job> = (0..20).map(|i| Job {
        id: i, data: format!("任务-{i}"),
    }).collect();

    let results = worker_pool(jobs, 4);
    for r in &results {
        println!("[工作者 {}] 任务 {}: {}", r.worker_id, r.job_id, r.output);
    }
}
```

</details>

***
