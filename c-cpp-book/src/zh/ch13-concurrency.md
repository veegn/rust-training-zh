[English Original](../en/ch13-concurrency.md)

# Rust 并发 (Concurrency)

> **你将学到：** Rust 的并发模型 —— 线程、`Send`/`Sync` 标记特性、`Mutex<T>`、`Arc<T>`、通道，以及编译器如何在编译时防止数据竞争。对于不使用的线程安全特性，Rust 不会引入任何运行时开销。

- Rust 内置了对并发的支持，类似于 C++ 中的 `std::thread`。
    - 关键区别：Rust 通过 `Send` 和 `Sync` 标记特性**在编译时防止数据竞争**。
    - 在 C++ 中，在没有互斥锁的情况下跨线程共享 `std::vector` 是未定义行为 (UB)，但可以顺利编译。而在 Rust 中，这根本无法通过编译。
    - Rust 中的 `Mutex<T>` 封装了**数据**本身，而不仅仅是访问权限 —— 若不加锁，你完全无法读取数据。
- 可以使用 `thread::spawn()` 创建一个单独的线程，并行执行闭包 `||`。
```rust
use std::thread;
use std::time::Duration;
fn main() {
    let handle = thread::spawn(|| {
        for i in 0..10 {
            println!("线程内计数: {i}!");
            thread::sleep(Duration::from_millis(5));
        }
    });

    for i in 0..5 {
        println!("主线程计数: {i}");
        thread::sleep(Duration::from_millis(5));
    }

    handle.join().unwrap(); // handle.join() 确保衍生的线程执行完毕后主线程才退出
}
```

---

# Rust 并发
- `thread::scope()` 常用于需要从环境中借用的场景。这之所以可行，是因为 `thread::scope` 会等待其内部线程返回。
- 尝试在不使用 `thread::scope` 的情况下执行此练习，看看会出现什么问题。
```rust
use std::thread;
fn main() {
  let a = [0, 1, 2];
  thread::scope(|scope| {
      scope.spawn(|| {
          for x in &a {
            println!("{x}");
          }
      });
  });
}
```

---

# Rust 并发
- 我们还可以使用 `move` 将所有权转移到线程。对于像 `[i32; 3]` 这样的 `Copy` 类型，`move` 关键字会将数据复制到闭包中，而原始数据仍然可用。
```rust
use std::thread;
fn main() {
  let mut a = [0, 1, 2];
  let handle = thread::spawn(move || {
      for x in a {
        println!("{x}");
      }
  });
  a[0] = 42;    // 不会影响发送到线程中的副本
  handle.join().unwrap();
}
```

---

# Rust 并发
- `Arc<T>` 可用于在多个线程之间共享*只读*引用。
    - `Arc` 代表原子引用计数（Atomic Reference Counted）。只有当引用计数降至 0 时，引用才会被释放。
    - `Arc::clone()` 仅仅增加引用计数，而不会克隆底层数据。
```rust
use std::sync::Arc;
use std::thread;
fn main() {
    let a = Arc::new([0, 1, 2]);
    let mut handles = Vec::new();
    for i in 0..2 {
        let arc = Arc::clone(&a);
        handles.push(thread::spawn(move || {
            println!("线程: {i} {arc:?}");
        }));
    }
    handles.into_iter().for_each(|h| h.join().unwrap());
}
```

---

# Rust 并发
- `Arc<T>` 可以与 `Mutex<T>` 结合使用，以提供可变的共享引用。
    - `Mutex` 负责保护受保护的数据，并确保只有持有锁的线程才能访问。
    - `MutexGuard` 在离开作用域时会自动释放（RAII）。注：虽然 `std::mem::forget` 仍可能导致守护者泄漏，但“不可能忘记解锁”比“不可能泄漏”更贴切。
```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = Vec::new();

    for _ in 0..5 {
        let counter = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
            // MutexGuard 在此处被销毁 —— 锁自动释放
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("最终计数: {}", *counter.lock().unwrap());
    // 输出: 最终计数: 5
}
```

---

# Rust 并发：RwLock
- `RwLock<T>` 允许多个并发读取者或一个独占写入者 —— 即 C++ 中的读写锁模式（`std::shared_mutex`）。
    - 当读取远多于写入时（例如配置、缓存），请使用 `RwLock`。
    - 当读写频率相近或临界区极短时，请使用 `Mutex`。
```rust
use std::sync::{Arc, RwLock};
use std::thread;

fn main() {
    let config = Arc::new(RwLock::new(String::from("v1.0")));
    let mut handles = Vec::new();

    // 衍生 5 个读取者 —— 全都可以并发运行
    for i in 0..5 {
        let config = Arc::clone(&config);
        handles.push(thread::spawn(move || {
            let val = config.read().unwrap();  // 允许多个读取者
            println!("读取者 {i}: {val}");
        }));
    }

    // 1 个写入者 —— 阻塞直至所有读取者完成任务
    {
        let config = Arc::clone(&config);
        handles.push(thread::spawn(move || {
            let mut val = config.write().unwrap();  // 独占访问
            *val = String::from("v2.0");
            println!("写入者: 已更新至 {val}");
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
```

---

# Rust 并发：Mutex 中毒 (Mutex poisoning)
- 如果一个线程在持有 `Mutex` 或 `RwLock` 的情况下**发生 panic**，该锁就会**中毒 (poisoned)**。
    - 随后对 `.lock()` 的调用将返回 `Err(PoisonError)` —— 这意味着数据可能处于不一致的状态。
    - 如果你确信数据仍然有效，可以使用 `.into_inner()` 进行恢复。
    - C++ 中没有等效的概念 —— `std::mutex` 没有中毒的概念，发生 panic 的线程只会导致锁仍处于持有的状态。
```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let data = Arc::new(Mutex::new(vec![1, 2, 3]));

    let data2 = Arc::clone(&data);
    let handle = thread::spawn(move || {
        let mut guard = data2.lock().unwrap();
        guard.push(4);
        panic!("糟了！");  // 此时锁已中毒
    });

    let _ = handle.join();  // 线程发生过 panic

    // 随后的加锁尝试将返回 Err(PoisonError)
    match data.lock() {
        Ok(guard) => println!("数据: {guard:?}"),
        Err(poisoned) => {
            println!("锁已经中毒！正在恢复...");
            let guard = poisoned.into_inner();  // 无论如何都要访问数据
            println!("恢复的数据: {guard:?}");  // [1, 2, 3, 4] —— 在 panic 发生前 push 已成功
        }
    }
}
```

---

# Rust 并发：Atomics
- 对于简单的计数器和标志，`std::sync::atomic` 类型可以避免 `Mutex` 的开销。
    - `AtomicBool`、`AtomicI32`、`AtomicU64`、`AtomicUsize` 等。
    - 等同于 C++ 中的 `std::atomic<T>` —— 二者具有相同的内存排序模型（`Relaxed`、`Acquire`、`Release`、`SeqCst`）。
```rust
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::thread;

fn main() {
    let counter = Arc::new(AtomicU64::new(0));
    let mut handles = Vec::new();

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            for _ in 0..1000 {
                counter.fetch_add(1, Ordering::Relaxed);
            }
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("计数器: {}", counter.load(Ordering::SeqCst));
    // 输出: 计数器: 10000
}
```

| 原语 | 使用场景 | C++ 等价物 |
|-----------|-------------|----------------|
| `Mutex<T>` | 通用的可变共享状态 | `std::mutex` + 手动数据关联 |
| `RwLock<T>` | 读取密集的负载 | `std::shared_mutex` |
| `Atomic*` | 简单的计数器、标志、无锁模式 | `std::atomic<T>` |
| `Condvar` | 等待条件成立 | `std::condition_variable` |

---

# Rust 并发：Condvar
- `Condvar`（条件变量）可以让一个线程**进入睡眠状态，直到另一个线程发出信号**通知条件已改变。
    - 始终与 `Mutex` 配对使用 —— 其模式为：锁定、检查条件、若未就绪则等待、就绪后执行操作。
    - 等同于 C++ 中的 `std::condition_variable` / `std::condition_variable::wait`。
    - 处理**虚假唤醒 (spurious wakeups)** —— 始终在循环中重新检查条件（或者使用 `wait_while`/`wait_until`）。
```rust
use std::sync::{Arc, Condvar, Mutex};
use std::thread;

fn main() {
    let pair = Arc::new((Mutex::new(false), Condvar::new()));

    // 衍生一个等待信号的工作线程
    let pair2 = Arc::clone(&pair);
    let worker = thread::spawn(move || {
        let (lock, cvar) = &*pair2;
        let mut ready = lock.lock().unwrap();
        // wait: 进入睡眠直至收到信号（始终在循环中重新检查以防虚假唤醒）
        while !*ready {
            ready = cvar.wait(ready).unwrap();
        }
        println!("工作线程：条件已满足，继续执行！");
    });

    // 主线程执行一些工作，然后向工作线程发送信号
    thread::sleep(std::time::Duration::from_millis(100));
    {
        let (lock, cvar) = &*pair;
        let mut ready = lock.lock().unwrap();
        *ready = true;
        cvar.notify_one();  // 唤醒一个等待中的线程（notify_all() 会唤醒所有线程）
    }

    worker.join().unwrap();
}
```

> **何时使用 Condvar 与通道 (channels)：** 当线程共享可变状态且需要等待该状态下的某个条件（例如“缓冲区非空”）时，请使用 `Condvar`。而当线程需要传递*消息*时，请使用通道（`mpsc`）。通道通常更容易理解和推理。

---

# Rust 并发
- Rust 通道可用于在发送者 (`Sender`) 和接收者 (`Receiver`) 之间交换消息。
    - 这里使用的是名为 `mpsc` 或“多生产者，单消费者 (Multi-producer, Single-Consumer)”的范式。
    - `send()` 和 `recv()` 都会对线程产生阻塞。
```rust
use std::sync::mpsc;

fn main() {
    let (tx, rx) = mpsc::channel();
    
    tx.send(10).unwrap();
    tx.send(20).unwrap();
    
    println!("接收到: {:?}", rx.recv());
    println!("接收到: {:?}", rx.recv());

    let tx2 = tx.clone();
    tx2.send(30).unwrap();
    println!("接收到: {:?}", rx.recv());
}
```

---

# Rust 并发
- 通道可以与线程结合使用
```rust
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();
    for _ in 0..2 {
        let tx2 = tx.clone();
        thread::spawn(move || {
            let thread_id = thread::current().id();
            for i in 0..10 {
                tx2.send(format!("消息 {i}")).unwrap();
                println!("{thread_id:?}: 发送了消息 {i}");
            }
            println!("{thread_id:?}: 完成");
        });
    }

    // 丢弃原始发送者，以便当所有克隆的发送者也被丢弃时 rx.iter() 能够正常终止
    drop(tx);

    thread::sleep(Duration::from_millis(100));

    for msg in rx.iter() {
        println!("主线程：获取到 {msg}");
    }
}
```

---

## 为什么 Rust 能防止数据竞争：Send 与 Sync

- Rust 使用两个标记特性在编译时强制实施线程安全：
    - `Send`：如果一个类型可以安全地**转移**到另一个线程，它就是 `Send`。
    - `Sync`：如果一个类型可以安全地在线程之间（通过 `&T`）**共享**，它就是 `Sync`。
- 大多数类型都是自动实现 `Send + Sync` 的。一些显著的例外包括：
    - `Rc<T>` **既不是** Send 也不是 Sync（在线程中请使用 `Arc<T>`）。
    - `Cell<T>` 和 `RefCell<T>` **不是** Sync（请使用 `Mutex<T>` 或 `RwLock<T>`）。
    - 裸指针（`*const T`、`*mut T`）**既不是** Send 也不是 Sync。
- 这就是为什么编译器会阻止你跨线程使用 `Rc<T>` —— 它根本没有实现 `Send`。
- `Arc<Mutex<T>>` 是 `Rc<RefCell<T>>` 的线程安全版本。

> **直观理解** *(Jon Gjengset)*：把值想象成玩具。
> **`Send`** = 你可以**把你的玩具送给**另一个孩子（线程）—— 转移所有权是安全的。
> **`Sync`** = 你可以**让其他孩子同时玩你的玩具** —— 共享引用是安全的。
> `Rc<T>` 有一个脆弱的（非原子）引用计数器；送出或共享它都会破坏计数，因此它既不是 `Send` 也不是 `Sync`。

---

# 练习：多线程词频统计

🔴 **极度挑战** —— 融合线程、Arc、Mutex 以及 HashMap

- 给定一组由 `Vec<String>` 组成的文本行，为每一行衍生一个线程来统计该行中的单词。
- 使用 `Arc<Mutex<HashMap<String, usize>>>` 收集统计结果。
- 打印所有行的单词总数。
- **加分项**：尝试使用通道 (`mpsc`) 而非共享状态来实现。

<details><summary>参考答案 (点击展开)</summary>

```rust
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let lines = vec![
        "the quick brown fox".to_string(),
        "jumps over the lazy dog".to_string(),
        "the fox is quick".to_string(),
    ];

    let word_counts: Arc<Mutex<HashMap<String, usize>>> =
        Arc::new(Mutex::new(HashMap::new()));

    let mut handles = vec![];
    for line in &lines {
        let line = line.clone();
        let counts = Arc::clone(&word_counts);
        handles.push(thread::spawn(move || {
            for word in line.split_whitespace() {
                let mut map = counts.lock().unwrap();
                *map.entry(word.to_lowercase()).or_insert(0) += 1;
            }
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let counts = word_counts.lock().unwrap();
    let total: usize = counts.values().sum();
    println!("词频统计: {counts:#?}");
    println!("单词总数: {total}");
}
```
**输出示例 (顺序可能有所不同):**
```text
词频统计: {
    "the": 3,
    "quick": 2,
    "brown": 1,
    "fox": 2,
    "jumps": 1,
    "over": 1,
    "lazy": 1,
    "dog": 1,
    "is": 1,
}
单词总数: 13
```

</details>

---
