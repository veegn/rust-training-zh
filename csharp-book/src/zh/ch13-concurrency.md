# 并发：安全的并行计算

> **你将学到什么：** Rust 如何在编译期强制保证线程安全，而 C# 更多依赖约定式做法；`Arc<Mutex<T>>` 与 `lock` 的对比，以及 `Send`/`Sync` trait。
>
> **难度：** 高级

在 C# 中，你可以随意在线程间共享任何对象，而你的责任则是确保正确地使用了 `lock`。而在 Rust 中，编译器会**强制保证**线程安全。如果你的代码可能会导致数据竞争，它根本就无法通过编译。

---

## 共享状态：`Arc` 与 `Mutex`
在 Rust 中，你不能直接在线程间共享一个普通变量，因为编译器无法证明它是安全的。你需要用到这两个组件：
1.  **`Arc<T>` (原子引用计数器)**：允许数据在多个线程间共享“所有权”。
2.  **`Mutex<T>` (互斥锁)**：确保在同一时间内，只能有一个线程可以修改数据。

### Rust 示例
```rust
use std::sync::{Arc, Mutex};
use std::thread;

let counter = Arc::new(Mutex::new(0));
let mut handles = vec![];

for _ in 0..10 {
    let counter = Arc::clone(&counter);
    let handle = thread::spawn(move || {
        let mut num = counter.lock().unwrap();
        *num += 1;
    });
    handles.push(handle);
}

for handle in handles { handle.join().unwrap(); }
println!("最终计数结果：{}", *counter.lock().unwrap());
```

---

## `Send` 与 `Sync`：神奇的标记 Trait
Rust 使用两个特殊的 Trait 来追踪线程安全性：
*   **`Send`**：表示该类型可以从一个线程**转移**到另一个线程。
*   **`Sync`**：表示该类型可以安全地在多个线程间通过引用进行**共享**。

**杀手锏：** 大多数类型会自动实现 `Send` 和 `Sync`。但如果你尝试在线程中使用非线程安全的类型（比如 `Rc`），编译器会直接报错。在 C# 中，这类错误通常是无声无息的运行时灾难。

---

## 消息传递 (Channels)
Rust 中有一个非常著名的设计哲学：*“不要通过共享内存来通信；相反，要通过通信来共享内存。”*

```rust
use std::sync::mpsc; // 多生产者，单消费者
let (tx, rx) = mpsc::channel();

thread::spawn(move || {
    tx.send("来自线程的消息").unwrap();
});

let msg = rx.recv().unwrap();
```

---

## C# 开发者总结表
| **概念** | **C# 方式** | **Rust 方式** |
| :--- | :--- | :--- |
| **简单的锁** | `lock (obj) { ... }` | `MutexGuard` (RAII) |
| **共享引用** | 直接传对象引用 | `Arc<T>` |
| **线程本地存储** | `[ThreadStatic]` | `thread_local!` 宏 |
| **并发迭代** | `Parallel.ForEach` | `rayon` crate |
| **原子操作** | `Interlocked` 类 | `std::sync::atomic` |

---

## 练习：并行处理
**挑战：** 使用 `rayon` 库（Rust 中进行数据并行的标准库），对一个整数数组中的所有元素并行求平方。

```rust
use rayon::prelude::*;

let mut nums = vec![1, 2, 3, 4, 5];
let squares: Vec<_> = nums.par_iter().map(|&x| x * x).collect();
```
**关键理解：** “无畏并发 (Fearless Concurrency)”在 Rust 中已成为了现实。编译器就像是一个和你结对编程的资深程序员，在那些微妙的竞态条件甚至还没进入生产阶段之前，它就会把它们全部抓出来。
