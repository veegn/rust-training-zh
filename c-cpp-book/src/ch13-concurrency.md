# Rust concurrency / Rust 并发
 
 > **What you'll learn / 你将学到：** Rust's concurrency model — threads, `Send`/`Sync` marker traits, `Mutex<T>`, `Arc<T>`, channels, and how the compiler prevents data races at compile time. No runtime overhead for thread safety you don't use.
 >
 > Rust 的并发模型 —— 线程、`Send` / `Sync` 标记 trait、`Mutex<T>`、`Arc<T>`、信道（channels），以及编译器如何通过编译时检查来防止数据竞态（data races）。对于你未使用的线程安全特性，不会产生任何运行时开销。
 
 - Rust has built-in support for concurrency, similar to `std::thread` in C++
+ - Rust 内置了对并发的支持，类似于 C++ 中的 `std::thread`。
-     - Key difference: Rust **prevents data races at compile time** through `Send` and `Sync` marker traits
+     - 关键区别：Rust 通过 `Send` 和 `Sync` 标记 trait 在**编译时防止数据竞态**。
-     - In C++, sharing a `std::vector` across threads without a mutex is UB but compiles fine. In Rust, it won't compile.
+     - 在 C++ 中，在没有互斥锁的情况下跨线程共享 `std::vector` 是未定义行为（UB），但编译是正常的。而在 Rust 中，这无法通过编译。
-     - `Mutex<T>` in Rust wraps the **data**, not just the access — you literally cannot read the data without locking
+     - Rust 中的 `Mutex<T>` 包装的是**数据**本身，而不仅仅是访问权限 —— 字面上，如果不加锁，你甚至无法读取数据。
- - The `thread::spawn()` can be used to create a separate thread that executes the closure `||` in parallel
+ - `thread::spawn()` 可用于创建一个运行闭包 `||` 的独立线程。
 ```rust
 use std::thread;
 use std::time::Duration;
 fn main() {
     let handle = thread::spawn(|| {
         for i in 0..10 {
             println!("Count in thread: {i}!");
             thread::sleep(Duration::from_millis(5));
         }
     });
 
     for i in 0..5 {
         println!("Main thread: {i}");
         thread::sleep(Duration::from_millis(5));
     }
 
-    handle.join().unwrap(); // The handle.join() ensures that the spawned thread exits
+    handle.join().unwrap(); // Ensures the spawned thread exits / 确保生成的线程已退出
 }
 ```
 
- # Rust concurrency
+ # Rust concurrency continued / Rust 并发（续）
- - ```thread::scope()``` can be used in cases where it is necessary to borrow from the environment. This works because ```thread::scope``` waits until the internal thread returns
+ - 在需要从环境中进行借用的情况下，可以使用 ```thread::scope()```。这是可行的，因为 ```thread::scope``` 会等待内部线程返回。
- - Try executing this exercise without ```thread::scope``` to see the issue
+ - 尝试在没有 ```thread::scope``` 的情况下执行此练习以查看问题所在。
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
 ----
- # Rust concurrency
+ # Rust concurrency continued / Rust 并发（续）
- - We can also use ```move``` to transfer ownership to the thread. For `Copy` types like `[i32; 3]`, the `move` keyword copies the data into the closure, and the original remains usable
+ - 我们还可以使用 ```move``` 将所有权转移到线程中。对于像 `[i32; 3]` 这样实现了 `Copy` 的类型，`move` 关键字会将数据拷贝到闭包中，原始变量仍然可用。
 ```rust
 use std::thread;
 fn main() {
   let mut a = [0, 1, 2];
-  let handle = thread::spawn(move || {
+  let handle = thread::spawn(move || { // 使用 move 关键字
       for x in a {
         println!("{x}");
       }
   });
   a[0] = 42;    // Doesn't affect the copy sent to the thread / 不会影响发送到线程的拷贝
   handle.join().unwrap();
 }
 ```
 
- # Rust concurrency
+ # Rust concurrency: Arc / Rust 并发：Arc
- - ```Arc<T>``` can be used to share *read-only* references between multiple threads
+ - ```Arc<T>``` 可用于在多个线程之间共享**只读**引用。
-     - ```Arc``` stands for Atomic Reference Counted. The reference isn't released until the reference count reaches 0
+     - ```Arc``` 代表“原子引用计数（Atomic Reference Counted）”。直到引用计数达到 0 时，引用才会被释放。
-     - ```Arc::clone()``` simply increases the reference count without cloning the data
+     - ```Arc::clone()``` 仅增加引用计数，而不克隆数据本身。
 ```rust
 use std::sync::Arc;
 use std::thread;
 fn main() {
     let a = Arc::new([0, 1, 2]);
     let mut handles = Vec::new();
     for i in 0..2 {
-        let arc = Arc::clone(&a);
+        let arc = Arc::clone(&a); // 增加引用计数
         handles.push(thread::spawn(move || {
             println!("Thread: {i} {arc:?}");
         }));
     }
     handles.into_iter().for_each(|h| h.join().unwrap());
 }
 ```
 
- # Rust concurrency
+ # Rust concurrency: Mutex / Rust 并发：Mutex
- - ```Arc<T>``` can be combined with ```Mutex<T>``` to provide mutable references.
+ - ```Arc<T>``` 可以与 ```Mutex<T>``` 结合使用以提供可变引用。
-     - ```Mutex``` guards the protected data and ensures that only the thread holding the lock has access.
+     - ```Mutex```（互斥锁）保护受保护的数据，并确保只有持有锁的线程才能访问。
-     - The `MutexGuard` is automatically released when it goes out of scope (RAII). Note: `std::mem::forget` can still leak a guard — so "impossible to forget to unlock" is more accurate than "impossible to leak."
+     - 当 `MutexGuard` 超出作用域时（RAII），锁会自动释放。注意：`std::mem::forget` 仍然可能导致 guard 泄漏 —— 因此，“不可能忘记解锁”比“不可能泄漏锁”更准确。
 ```rust
 use std::sync::{Arc, Mutex};
 use std::thread;
 
 fn main() {
     let counter = Arc::new(Mutex::new(0));
     let mut handles = Vec::new();
 
     for _ in 0..5 {
         let counter = Arc::clone(&counter);
         handles.push(thread::spawn(move || {
-            let mut num = counter.lock().unwrap();
+            let mut num = counter.lock().unwrap(); // 获取锁
             *num += 1;
-            // MutexGuard dropped here — lock released automatically
+            // MutexGuard dropped here / MutexGuard 在此处被丢弃 —— 自动释放锁
         }));
     }
 
     for handle in handles {
         handle.join().unwrap();
     }
 
     println!("Final count: {}", *counter.lock().unwrap());
-    // Output: Final count: 5
+    // Output / 输出：Final count: 5
 }
 ```
 
- # Rust concurrency: RwLock
+ # Rust concurrency: RwLock / Rust 并发：RwLock
- - `RwLock<T>` allows **multiple concurrent readers** or **one exclusive writer** — the read/write lock pattern from C++ (`std::shared_mutex`)
+ - `RwLock<T>` 允许**多个并发读者**或**一个排他性作者** —— 这是来自 C++ 的读写锁模式（`std::shared_mutex`）。
-     - Use `RwLock` when reads far outnumber writes (e.g., configuration, caches)
+     - 当读取操作远多于写入操作时（例如配置、缓存），请使用 `RwLock`。
-     - Use `Mutex` when read/write frequency is similar or critical sections are short
+     - 当读写频率相近或临界区很短时，请使用 `Mutex`。
 ```rust
 use std::sync::{Arc, RwLock};
 use std::thread;
 
 fn main() {
     let config = Arc::new(RwLock::new(String::from("v1.0")));
     let mut handles = Vec::new();
 
-    // Spawn 5 readers — all can run concurrently
+    // Spawn 5 readers — all can run concurrently / 生成 5 个读者 —— 它们可以并发运行
     for i in 0..5 {
         let config = Arc::clone(&config);
         handles.push(thread::spawn(move || {
-            let val = config.read().unwrap();  // Multiple readers OK
+            let val = config.read().unwrap();  // Multiple readers / 允许多个读者
             println!("Reader {i}: {val}");
         }));
     }
 
-    // One writer — blocks until all readers finish
+    // One writer — blocks until all readers finish / 一个作者 —— 阻塞直到所有读者完成
     {
         let config = Arc::clone(&config);
         handles.push(thread::spawn(move || {
-            let mut val = config.write().unwrap();  // Exclusive access
+            let mut val = config.write().unwrap();  // Exclusive / 排他性访问
             *val = String::from("v2.0");
             println!("Writer: updated to {val}");
         }));
     }
 
     for handle in handles {
         handle.join().unwrap();
     }
 }
 ```
 
- # Rust concurrency: Mutex poisoning
+ # Rust concurrency: Mutex poisoning / Rust 并发：Mutex 中毒
- - If a thread **panics** while holding a `Mutex` or `RwLock`, the lock becomes **poisoned**
+ - 如果一个线程在持有 `Mutex` 或 `RwLock` 时发生 **panic**，该锁就会变得“**中毒（poisoned）**”。
-     - Subsequent calls to `.lock()` return `Err(PoisonError)` — the data may be in an inconsistent state
+     - 后续对 `.lock()` 的调用将返回 `Err(PoisonError)` —— 因为数据可能处于不一致的状态。
-     - You can recover with `.into_inner()` if you're confident the data is still valid
+     - 如果你确信数据仍然有效，可以使用 `.into_inner()` 进行恢复。
-     - This has no C++ equivalent — `std::mutex` has no poisoning concept; a panicking thread just leaves the lock held
+     - 这在 C++ 中没有等价物 —— `std::mutex` 没有中毒概念；发生 panic 的线程只会让锁一直保持被持有状态。
 ```rust
 use std::sync::{Arc, Mutex};
 use std::thread;
 
 fn main() {
     let data = Arc::new(Mutex::new(vec![1, 2, 3]));
 
     let data2 = Arc::clone(&data);
     let handle = thread::spawn(move || {
         let mut guard = data2.lock().unwrap();
         guard.push(4);
-        panic!("oops!");  // Lock is now poisoned
+        panic!("oops!");  // Lock is poisoned / 锁在此处中毒
     });
 
-    let _ = handle.join();  // Thread panicked
+    let _ = handle.join();  // Thread panicked / 线程发生了 panic
 
-    // Subsequent lock attempts return Err(PoisonError)
+    // Subsequent attempts return Err / 后续尝试将返回错误
     match data.lock() {
         Ok(guard) => println!("Data: {guard:?}"),
         Err(poisoned) => {
             println!("Lock was poisoned! Recovering...");
-            let guard = poisoned.into_inner();  // Access data anyway
+            let guard = poisoned.into_inner();  // Access anyway / 依然访问数据
-            println!("Recovered data: {guard:?}");  // [1, 2, 3, 4] — push succeeded before panic
+            println!("Recovered data: {guard:?}");  // 成功恢复的数据：[1, 2, 3, 4]
         }
     }
 }
 ```
 
- # Rust concurrency: Atomics
+ # Rust concurrency: Atomics / Rust 并发：原子操作
- - For simple counters and flags, `std::sync::atomic` types avoid the overhead of a `Mutex`
+ - 对于简单的计数器和标志位，`std::sync::atomic` 类型可以避免 `Mutex` 的开销。
-     - `AtomicBool`, `AtomicI32`, `AtomicU64`, `AtomicUsize`, etc.
+     - 包括 `AtomicBool`、`AtomicI32`、`AtomicU64`、`AtomicUsize` 等。
-     - Equivalent to C++ `std::atomic<T>` — same memory ordering model (`Relaxed`, `Acquire`, `Release`, `SeqCst`)
+     - 等价于 C++ 的 `std::atomic<T>` —— 具有相同的内存顺序模型（`Relaxed`、`Acquire`、`Release`、`SeqCst`）。
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
-                counter.fetch_add(1, Ordering::Relaxed);
+                counter.fetch_add(1, Ordering::Relaxed); // 原子加法
             }
         }));
     }
 
     for handle in handles {
         handle.join().unwrap();
     }
 
     println!("Counter: {}", counter.load(Ordering::SeqCst));
-    // Output: Counter: 10000
+    // Output / 输出：Counter: 10000
 }
 ```
 
-| Primitive | When to use | C++ equivalent |
+| **Primitive / 原语** | **When to use / 何时使用** | **C++ equivalent / C++ 等价物** |
 |-----------|-------------|----------------|
-| `Mutex<T>` | General mutable shared state | `std::mutex` + manual data association |
+| `Mutex<T>` | General mutable shared state / 普通可变共享状态 | `std::mutex` + 手动数据关联 |
-| `RwLock<T>` | Read-heavy workloads | `std::shared_mutex` |
+| `RwLock<T>` | Read-heavy workloads / 读多写少的工作负载 | `std::shared_mutex` |
-| `Atomic*` | Simple counters, flags, lock-free patterns | `std::atomic<T>` |
+| `Atomic*` | Simple counters, flags / 简单计数器、标志位 | `std::atomic<T>` |
-| `Condvar` | Wait for a condition to become true | `std::condition_variable` |
+| `Condvar` | Wait for condition / 等待某个条件成立 | `std::condition_variable` |
 
- # Rust concurrency: Condvar
+ # Rust concurrency: Condvar / Rust 并发：Condvar
- - `Condvar` (condition variable) lets a thread **sleep until another thread signals** that a condition has changed
+ - `Condvar`（条件变量）允许线程**睡眠直到另一个线程发出信号**表示条件已改变。
-     - Always paired with a `Mutex` — the pattern is: lock, check condition, wait if not ready, act when ready
+     - 它总是与 `Mutex` 成对出现 —— 其模式是：加锁、检查条件、如果不就绪则等待、就绪后采取行动。
-     - Equivalent to C++ `std::condition_variable` / `std::condition_variable::wait`
+     - 等价于 C++ 的 `std::condition_variable` / `std::condition_variable::wait`。
-     - Handles **spurious wakeups** — always re-check the condition in a loop (or use `wait_while`/`wait_until`)
+     - 处理**虚假唤醒（spurious wakeups）** —— 始终在循环中重新检查条件（或者使用 `wait_while` / `wait_until`）。
 ```rust
 use std::sync::{Arc, Condvar, Mutex};
 use std::thread;
 
 fn main() {
     let pair = Arc::new((Mutex::new(false), Condvar::new()));
 
-    // Spawn a worker that waits for a signal
+    // Spawn a worker / 生成一个等待信号的工人线程
     let pair2 = Arc::clone(&pair);
     let worker = thread::spawn(move || {
         let (lock, cvar) = &*pair2;
         let mut ready = lock.lock().unwrap();
-        // wait: sleeps until signaled (always re-check in a loop for spurious wakeups)
+        // wait: sleeps until signaled / wait：睡眠直到收到信号（必须在循环中重检）
         while !*ready {
             ready = cvar.wait(ready).unwrap();
         }
         println!("Worker: condition met, proceeding!");
     });
 
-    // Main thread does some work, then signals the worker
+    // Main thread signals / 主线程发出信号
     thread::sleep(std::time::Duration::from_millis(100));
     {
         let (lock, cvar) = &*pair;
         let mut ready = lock.lock().unwrap();
         *ready = true;
-        cvar.notify_one();  // Wake one waiting thread (notify_all() wakes all)
+        cvar.notify_one();  // Wake one / 唤醒一个等待中的线程
     }
 
     worker.join().unwrap();
 }
 ```
 
- > **When to use Condvar vs channels:** Use `Condvar` when threads share mutable state and need to wait for a condition on that state (e.g., "buffer not empty"). Use channels (`mpsc`) when threads need to pass *messages*. Channels are generally easier to reason about.
+ > **何时使用 Condvar vs 信道（Channels）：** 当线程间共享可变状态并需要等待该状态满足某个条件（例如，“缓冲区不为空”）时，请使用 `Condvar`。当线程需要传递**消息**时，请使用信道（`mpsc`）。信道通常更容易理解和推理。
 
- # Rust concurrency
+ # Rust concurrency: Channels / Rust 并发：信道（Channels）
- - Rust channels can be used to exchange messages between ```Sender``` and ```Receiver```
+ - Rust 信道可用于在 ```发送端（Sender）``` 和 ```接收端（Receiver）``` 之间交换消息。
-     - This uses a paradigm called ```mpsc``` or ```Multi-producer, Single-Consumer```
+     - 它采用了一种称为 ```mpsc``` 或 ```多生产者单消费者（Multi-producer, Single-Consumer）``` 的范式。
-     - Both ```send()``` and ```recv()``` can block the thread
+     - ```send()``` 和 ```recv()``` 都有可能阻塞线程。
 ```rust
 use std::sync::mpsc;
 
 fn main() {
-    let (tx, rx) = mpsc::channel();
+    let (tx, rx) = mpsc::channel(); // 创建信道
     
     tx.send(10).unwrap();
     tx.send(20).unwrap();
     
     println!("Received: {:?}", rx.recv());
     println!("Received: {:?}", rx.recv());
 
-    let tx2 = tx.clone();
+    let tx2 = tx.clone(); // 克隆发送端
     tx2.send(30).unwrap();
     println!("Received: {:?}", rx.recv());
 }
 ```
 
- # Rust concurrency
+ # Rust concurrency: Channels continued / Rust 并发：信道（续）
- - Channels can be combined with threads
+ - 信道可以与线程结合使用：
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
                 tx2.send(format!("Message {i}")).unwrap();
                 println!("{thread_id:?}: sent Message {i}");
             }
             println!("{thread_id:?}: done");
         });
     }
 
-        // Drop the original sender so rx.iter() terminates when all cloned senders are dropped
+    // Drop the original sender / 丢弃原始发送端，以便 rx.iter() 结束
     drop(tx);
 
     thread::sleep(Duration::from_millis(100));
 
     for msg in rx.iter() {
         println!("Main: got {msg}");
     }
 }
 ```
 
- ## Why Rust prevents data races: Send and Sync
+ ## Why Rust prevents data races: Send and Sync / 为什么 Rust 能防止数据竞态：Send 与 Sync
 
- - Rust uses two marker traits to enforce thread safety at compile time:
+ - Rust 使用两个标记 trait 在编译时强制执行线程安全：
-     - `Send`: A type is `Send` if it can be safely **transferred** to another thread
+     - `Send`：如果一个类型可以安全地**转移**到另一个线程，则该类型是 `Send`。
-     - `Sync`: A type is `Sync` if it can be safely **shared** (via `&T`) between threads
+     - `Sync`：如果一个类型可以安全地在线程间**共享**（通过 `&T`），则该类型是 `Sync`。
- - Most types are automatically `Send + Sync`. Notable exceptions:
+ - 大多数类型都是自动满足 `Send + Sync` 的。值得注意的例外包括：
-     - `Rc<T>` is **neither** Send nor Sync (use `Arc<T>` for threads)
+     - `Rc<T>` **既不是** Send 也不是 Sync（线程中请使用 `Arc<T>`）。
-     - `Cell<T>` and `RefCell<T>` are **not** Sync (use `Mutex<T>` or `RwLock<T>`)
+     - `Cell<T>` 和 `RefCell<T>` **不是** Sync（请使用 `Mutex<T>` 或 `RwLock<T>`）。
-     - Raw pointers (`*const T`, `*mut T`) are **neither** Send nor Sync
+     - 裸指针（`*const T`、`*mut T`）**既不是** Send 也不是 Sync。
- - This is why the compiler stops you from using `Rc<T>` across threads -- it literally doesn't implement `Send`
+ - 这就是为什么编译器会阻止你跨线程使用 `Rc<T>` —— 它字面上就没有实现 `Send`。
- - `Arc<Mutex<T>>` is the thread-safe equivalent of `Rc<RefCell<T>>`
+ - `Arc<Mutex<T>>` 是 `Rc<RefCell<T>>` 的线程安全对应物。
 
- > **Intuition** *(Jon Gjengset)*: Think of values as toys.
+ > **直观理解** *(Jon Gjengset)*：把值想象成玩具。
- > **`Send`** = you can **give your toy away** to another child (thread) — transferring ownership is safe.
+ > **`Send`** = 你可以**把玩具送给**另一个孩子（线程）—— 转移所有权是安全的。
- > **`Sync`** = you can **let others play with your toy at the same time** — sharing a reference is safe.
+ > **`Sync`** = 你可以**让别人同时玩你的玩具** —— 共享引用是安全的。
- > An `Rc<T>` has a fragile (non-atomic) reference counter; handing it off or sharing it would corrupt the count, so it is neither `Send` nor `Sync`.
+ > `Rc<T>` 的引用计数是脆弱的（非原子性的）；转手或共享它都会破坏计数，因此它既不是 `Send` 也不是 `Sync`。
 
- # Exercise: Multi-threaded word count
+ # Exercise: Multi-threaded word count / 练习：多线程单词统计
 
- 🔴 **Challenge** — combines threads, Arc, Mutex, and HashMap
+ 🔴 **Challenge / 挑战题** —— 综合运用线程、Arc、Mutex 和 HashMap
 
- - Given a `Vec<String>` of text lines, spawn one thread per line to count the words in that line
+ - 给定一个存储文本行的 `Vec<String>`，为每一行生成一个线程来统计该行中的单词。
- - Use `Arc<Mutex<HashMap<String, usize>>>` to collect results
+ - 使用 `Arc<Mutex<HashMap<String, usize>>>` 来收集结果。
- - Print the total word count across all lines
+ - 打印所有行中的总词数。
- - **Bonus**: Try implementing this with channels (`mpsc`) instead of shared state
+ - **加分项**：尝试用信道（`mpsc`）而非共享状态来实现。
 
- <details><summary>Solution (click to expand)</summary>
+ <details><summary>Solution (click to expand) / 解决方案（点击展开）</summary>
 
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
     println!("Word frequencies: {counts:#?}");
-    println!("Total words: {total}");
+    println!("Total words: {total}"); // 总词数
 }
- // Output:
+ // Output / 输出（顺序可能不同）：
 // Word frequencies: {
 //     "the": 3,
 //     "quick": 2,
 //     "brown": 1,
 //     "fox": 2,
 //     "jumps": 1,
 //     "over": 1,
 //     "lazy": 1,
 //     "dog": 1,
 //     "is": 1,
 // }
 // Total words: 13
 ```
 
 </details>
