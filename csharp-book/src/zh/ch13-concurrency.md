[English Original](../en/ch13-concurrency.md)

## 线程安全：约定原则 vs 类型系统保证

> **你将学到：** Rust 如何在编译时强制执行线程安全，对比 C# 基于约定的方式；`Arc<Mutex<T>>` 与 `lock` 的对比；通道 (Channels) 与 `ConcurrentQueue` 的对比；`Send`/`Sync` 特性；作用域线程 (Scoped threads)；以及通往 async/await 的桥梁。
>
> **难度：** 🔴 高级

> **深度探索**：关于生产环境下的异步模式（流处理、优雅停机、连接池、取消安全性），请参阅配套的 [异步 Rust 训练](../../async-book/src/summary.md) 指南。
>
> **先决条件**：[所有权与借用](ch07-ownership-and-borrowing.md) 以及 [智能指针](ch07-3-smart-pointers-beyond-single-ownership.md)（Rc 与 Arc 的决策树）。

### C# - 基于约定的线程安全
```csharp
// C# 集合默认不是线程安全的
public class UserService
{
    private readonly List<string> items = new();
    private readonly Dictionary<int, User> cache = new();

    // 这可能导致数据竞争：
    public void AddItem(string item)
    {
        items.Add(item);  // 非线程安全！
    }

    // 必须手动使用锁：
    private readonly object lockObject = new();

    public void SafeAddItem(string item)
    {
        lock (lockObject)
        {
            items.Add(item);  // 安全，但有运行时开销
        }
        // 在其他地方很容易忘记加锁
    }

    // ConcurrentCollection 有所帮助但功能有限：
    private readonly ConcurrentBag<string> safeItems = new();
    
    public void ConcurrentAdd(string item)
    {
        safeItems.Add(item);  // 线程安全但操作受限
    }

    // 复杂的共享状态管理
    private readonly ConcurrentDictionary<int, User> threadSafeCache = new();
    private volatile bool isShutdown = false;
    
    public async Task ProcessUser(int userId)
    {
        if (isShutdown) return;  // 可能存在竞争条件！
        
        var user = await GetUser(userId);
        threadSafeCache.TryAdd(userId, user);  // 必须记住哪些集合是安全的
    }

    // 线程本地存储 (Thread-local storage) 需要仔细管理
    private static readonly ThreadLocal<Random> threadLocalRandom = 
        new ThreadLocal<Random>(() => new Random());
        
    public int GetRandomNumber()
    {
        return threadLocalRandom.Value.Next();  // 安全但需手动管理
    }
}

// 带有潜在竞争条件的事件处理
public class EventProcessor
{
    public event Action<string> DataReceived;
    private readonly List<string> eventLog = new();
    
    public void OnDataReceived(string data)
    {
        // 竞争条件 —— 事件在检查与调用之间可能变为 null
        if (DataReceived != null)
        {
            DataReceived(data);
        }
        // 现代 C# (6+) 通过 DataReceived?.Invoke(data); 缓解了 null 竞争
        // 但底层的事件代理模型在下方的列表操作上依然允许竞争发生
        
        // 另一个竞争条件 —— 列表非线程安全
        eventLog.Add($"已处理: {data}");
    }
}
```

### Rust - 由类型系统保证的线程安全
```rust
use std::sync::{Arc, Mutex, RwLock};
use std::thread;
use std::collections::HashMap;
use tokio::sync::{mpsc, broadcast};

// Rust 在编译时阻止数据竞争
pub struct UserService {
    items: Arc<Mutex<Vec<String>>>,
    cache: Arc<RwLock<HashMap<i32, User>>>,
}

impl UserService {
    pub fn new() -> Self {
        UserService {
            items: Arc::new(Mutex::new(Vec::new())),
            cache: Arc::new(RwLock::new(HashMap::new())),
        }
    }
    
    pub fn add_item(&self, item: String) {
        let mut items = self.items.lock().unwrap();
        items.push(item);
        // 当 `items` 离开作用域时，锁会自动释放
    }
    
    // 多读者、单写者 —— 自动强制执行
    pub async fn get_user(&self, user_id: i32) -> Option<User> {
        let cache = self.cache.read().unwrap();
        cache.get(&user_id).cloned()
    }
    
    pub async fn cache_user(&self, user_id: i32, user: User) {
        let mut cache = self.cache.write().unwrap();
        cache.insert(user_id, user);
    }
    
    // 克隆 Arc 以供线程间共享
    pub fn process_in_background(&self) {
        let items = Arc::clone(&self.items);
        
        thread::spawn(move || {
            let items = items.lock().unwrap();
            for item in items.iter() {
                println!("正在处理: {}", item);
            }
        });
    }
}

// 基于通道 (Channel) 的通信 —— 无需共享状态
pub struct MessageProcessor {
    sender: mpsc::UnboundedSender<String>,
}

impl MessageProcessor {
    pub fn new() -> (Self, mpsc::UnboundedReceiver<String>) {
        let (tx, rx) = mpsc::unbounded_channel();
        (MessageProcessor { sender: tx }, rx)
    }
    
    pub fn send_message(&self, message: String) -> Result<(), mpsc::error::SendError<String>> {
        self.sender.send(message)
    }
}

// 这段代码无法通过编译 —— Rust 阻止了不安全的共享可变数据：
fn impossible_data_race() {
    let mut items = vec![1, 2, 3];
    
    // 无法通过编译 —— 不能将 `items` 同时移动到多个闭包中
    /*
    thread::spawn(move || {
        items.push(4);  // 错误：使用了已移动的值
    });
    
    thread::spawn(move || {
        items.push(5);  // 错误：使用了已移动的值  
    });
    */
}

// 安全的并发数据处理
use rayon::prelude::*;

fn parallel_processing() {
    let data = vec![1, 2, 3, 4, 5];
    
    // 并行迭代 —— 保证线程安全
    let results: Vec<i32> = data
        .par_iter()
        .map(|&x| x * x)
        .collect();
        
    println!("{:?}", results);
}

// 带有消息传递的异步并发
async fn async_message_passing() {
    let (tx, mut rx) = mpsc::channel(100);
    
    // 生产者任务
    let producer = tokio::spawn(async move {
        for i in 0..10 {
            if tx.send(i).await.is_err() {
                break;
            }
        }
    });
    
    // 消费者任务  
    let consumer = tokio::spawn(async move {
        while let Some(value) = rx.recv().await {
            println!("接收到: {}", value);
        }
    });
    
    // 等待两个任务完成
    let (producer_result, consumer_result) = tokio::join!(producer, consumer);
    producer_result.unwrap();
    consumer_result.unwrap();
}

#[derive(Clone)]
struct User {
    id: i32,
    name: String,
}
```

```mermaid
graph TD
    subgraph "C# 线程安全挑战"
        CS_MANUAL["手动同步"]
        CS_LOCKS["lock 语句"]
        CS_CONCURRENT["并发集合 (ConcurrentCollections)"]
        CS_VOLATILE["volatile 字段"]
        CS_FORGET["😰 容易忘记加锁"]
        CS_DEADLOCK["💀 可能发生死锁"]
        CS_RACE["🏃 竞争条件"]
        CS_OVERHEAD["⚡ 运行时开销"]
        
        CS_MANUAL --> CS_LOCKS
        CS_MANUAL --> CS_CONCURRENT
        CS_MANUAL --> CS_VOLATILE
        CS_LOCKS --> CS_FORGET
        CS_LOCKS --> CS_DEADLOCK
        CS_FORGET --> CS_RACE
        CS_LOCKS --> CS_OVERHEAD
    end
    
    subgraph "Rust 类型系统保证"
        RUST_OWNERSHIP["所有权系统"]
        RUST_BORROWING["借用检查器"]
        RUST_SEND["Send 特性"]
        RUST_SYNC["Sync 特性"]
        RUST_ARC["Arc&lt;Mutex&lt;T&gt;&gt;"]
        RUST_CHANNELS["消息传递"]
        RUST_SAFE["✅ 杜绝数据竞争"]
        RUST_FAST["⚡ 零成本抽象"]
        
        RUST_OWNERSHIP --> RUST_BORROWING
        RUST_BORROWING --> RUST_SEND
        RUST_SEND --> RUST_SYNC
        RUST_SYNC --> RUST_ARC
        RUST_ARC --> RUST_CHANNELS
        RUST_CHANNELS --> RUST_SAFE
        RUST_SAFE --> RUST_FAST
    end
    
    style CS_FORGET fill:#ffcdd2,color:#000
    style CS_DEADLOCK fill:#ffcdd2,color:#000
    style CS_RACE fill:#ffcdd2,color:#000
    style RUST_SAFE fill:#c8e6c9,color:#000
    style RUST_FAST fill:#c8e6c9,color:#000
```

---

<details>
<summary><strong>🏋️ 练习：线程安全计数器</strong> (点击展开)</summary>

**挑战**：实现一个线程安全的计数器，要求可以被 10 个线程同时递增。每个线程执行 1000 次递增操作。最终计数结果应精确为 10,000。

<details>
<summary>🔑 参考答案</summary>

```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0u64));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            for _ in 0..1000 {
                let mut count = counter.lock().unwrap();
                *count += 1;
            }
        }));
    }

    for h in handles { h.join().unwrap(); }
    assert_eq!(*counter.lock().unwrap(), 10_000);
    println!("最终计数: {}", counter.lock().unwrap());
}
```

**或者使用原子类型 (更高效，无锁)：**
```rust
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::thread;

fn main() {
    let counter = Arc::new(AtomicU64::new(0));
    let handles: Vec<_> = (0..10).map(|_| {
        let counter = Arc::clone(&counter);
        thread::spawn(move || {
            for _ in 0..1000 {
                counter.fetch_add(1, Ordering::Relaxed);
            }
        })
    }).collect();

    for h in handles { h.join().unwrap(); }
    assert_eq!(counter.load(Ordering::SeqCst), 10_000);
}
```

**关键收获**：`Arc<Mutex<T>>` 是通用模式。对于简单的计数器，使用 `AtomicU64` 可以完全避免锁的开销。

</details>
</details>

### 为什么 Rust 能阻止数据竞争：Send 与 Sync

Rust 使用两个标记特性 (Marker traits) 在**编译时**强制执行线程安全 —— C# 中没有与之对应的概念：

- `Send`：类型可以安全地在线程间**转移**所有权（例如，移动到传递给 `thread::spawn` 的闭包中）。
- `Sync`：类型可以安全地在线程间通过引用 (`&T`) **共享**。

大多数类型会自动实现 `Send + Sync`。显著的例外包括：
- `Rc<T>` **既不**满足 Send 也不满足 Sync —— 编译器会拒绝让你将其传递给 `thread::spawn`（请改用 `Arc<T>`）。
- `Cell<T>` 和 `RefCell<T>` **不满足** Sync —— 请使用 `Mutex<T>` 或 `RwLock<T>` 来实现线程安全的内部可变性。
- 原生指针 (`*const T`, `*mut T`) **既不**满足 Send 也不满足 Sync。

在 C# 中，`List<T>` 不是线程安全的，但编译器不会阻止你在线程间共享它。在 Rust 中，类似的错误会导致**编译错误**，而非运行时的竞争条件。

### 作用域线程 (Scoped threads)：从栈上借用数据

`thread::scope()` 允许派生的线程借用局部变量 —— 无需使用 `Arc`：

```rust
use std::thread;

fn main() {
    let data = vec![1, 2, 3, 4, 5];
    
    // 作用域线程可以借用 'data' —— 作用域会等待所有线程结束
    thread::scope(|s| {
        s.spawn(|| println!("线程 1: {data:?}"));
        s.spawn(|| println!("线程 2: sum = {}", data.iter().sum::<i32>()));
    });
    // 'data' 在此处依然有效 —— 已保证所有线程都已执行完毕
}
```

这类似于 C# 的 `Parallel.ForEach`，调用代码会等待完成，但 Rust 的借用检查器是在**编译时证明**了不存在数据竞争。

### 通往 async/await 的桥梁

C# 开发者通常倾向于使用 `Task` 和 `async/await` 而非原生线程。Rust 同样支持这两种范式：

| C# | Rust | 何时使用 |
|----|------|-------------|
| `Thread` | `std::thread::spawn` | CPU 密集型任务，每个任务对应一个 OS 线程 |
| `Task.Run` | `tokio::spawn` | 在运行时上运行的异步任务 |
| `async/await` | `async/await` | I/O 密集型并发 |
| `lock` | `Mutex<T>` | 同步互斥锁 |
| `SemaphoreSlim` | `tokio::sync::Semaphore` | 异步并发限制 |
| `Interlocked` | `std::sync::atomic` | 无锁原子操作 |
| `CancellationToken` | `tokio_util::sync::CancellationToken` | 协作式取消 |

> 下一章（[Async/Await 深度解析](ch13-1-asyncawait-deep-dive.md)）将详细介绍 Rust 的异步模型 —— 包括它与 C# 基于 `Task` 的模型有何不同。
