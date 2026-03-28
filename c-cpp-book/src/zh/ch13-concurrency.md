[English Original](../en/ch13-concurrency.md)

# 13. 并发 (Concurrency) 🟢

Rust 的 **并发** 处理方法深受“无畏并发 (Fearless Concurrency)”哲学的引导。Rust 的所有权和类型系统提供了一套工具，可帮助你编写无微妙 Bug 且易于重构的代码。

### 1. 线程 (Threads)
Rust 提供 1:1 线程模型。你可以使用 `thread::spawn` 创建一个新线程。

```rust
use std::thread;
use std::time::Duration;

fn main() {
    thread::spawn(|| {
        for i in 1..10 {
            println!("来自生成的线程的数字 {}！", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("来自主线程的数字 {}！", i);
        thread::sleep(Duration::from_millis(1));
    }
}
```

---

### 2. 使用信道进行消息传递 (Message Passing with Channels)
确保安全并发的一个流行方法是 **消息传递**：在这种模式中，线程或参与者 (Actors) 通过互相发送包含数据的消息来进行通信。

```rust
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("你好");
        tx.send(val).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("收到：{}", received);
}
```

---

### 3. 共享状态并发 (Shared-State Concurrency)
Rust 还支持使用 **Mutex** (互斥锁) 和 **Arc** (原子引用计数) 在线程之间共享数据。

```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
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

    for handle in handles {
        handle.join().unwrap();
    }

    println!("结果：{}", *counter.lock().unwrap());
}
```

---

### 4. 通过 `Sync` 和 `Send` Trait 实现可扩展的并发
- **`Send`**：允许在线程之间转移值的所有权。
- **`Sync`**：允许通过共享引用让多个线程访问同一个值。

大多数 Rust 类型会自动实现这些 Trait，但某些类型（如 `Rc<T>`）则没有，这可以防止它们在多线程环境中被不安全地使用。

---

### 对 C/C++ 开发者的总结
- **In C++**：你使用 `std::thread`、`std::mutex` 和 `std::atomic`。由于疏忽而导致数据竞态（例如，在没有互斥锁的情况下共享一个非原子变量）是非常容易发生的。
- **In Rust**：数据竞态是 **编译时错误**。除非数据被包装在 `Arc` 或 `Mutex` 等线程安全原语中，否则你无法在线程之间共享数据。这使得 Rust 中的多线程编程更加安全和可靠。

***
