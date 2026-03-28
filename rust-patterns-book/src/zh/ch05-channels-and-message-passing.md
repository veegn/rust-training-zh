# 5. 通道与消息传递 🟢

> **你将学到：**
> - `std::sync::mpsc` 的基础知识以及何时升级到 crossbeam-channel
> - 使用 `select!` 进行多源消息处理的通道选择
> - 有界与无界通道以及背压 (Backpressure) 策略
> - 用于封装并发状态的 Actor 模式

## std::sync::mpsc — 标准通道

Rust 的标准库提供了一个多生产者、单消费者 (MPSC) 的通道：

```rust
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone(); 
    thread::spawn(move || {
        tx1.send("来自 p1 的消息").unwrap();
    });

    thread::spawn(move || {
        tx.send("来自 p2 的消息").unwrap();
    });

    for msg in rx {
        println!("收到: {msg}");
    }
}
```

**关键特性**：
- **无界 (Unbounded)** 是默认值（如果消费者过慢，可能会填满内存）。
- `mpsc::sync_channel(N)` 创建一个带有背压的 **有界 (Bounded)** 通道。
- `rx.recv()` 是阻塞的；而 `rx.try_recv()` 不是。

---

## crossbeam-channel — 生产环境主力军

对于大多数生产用例，请使用 `crossbeam-channel`。它更快，支持 **多消费者 (MPMC)** 模式，且 API 设计更好。

```rust
let (tx, rx) = crossbeam_channel::bounded(100);
// rx.clone() 是有效的！你可以有多个线程同时从同一个通道接收数据。
```

### 通道选择 (select!)

类似 Go 语言中的 `select`，它可以同时监听多个通道：

```rust
loop {
    select! {
        recv(work_rx) -> msg => println!("任务: {msg:?}"),
        recv(ticker) -> _ => println!("心跳信号"),
        recv(deadline) -> _ => break,
    }
}
```

---

## Actor 模式

利用通道来序列化对可变状态的访问，以此取代 `Mutex`。这能有效防止复杂的锁顺序冲突问题。

```rust
enum Msg { Increment, Get(Sender<i64>) }

fn actor(rx: Receiver<Msg>) {
    let mut count = 0;
    while let Ok(msg) = rx.recv() {
        match msg {
            Msg::Increment => count += 1,
            Msg::Get(tx) => tx.send(count).unwrap(),
        }
    }
}
```

> **经验法则**： 
> - 对于临界区非常短且数据共享简单的场景，使用 **Mutex**。
> - 对于状态逻辑复杂、存在耗时操作或涉及分布式系统的场景，使用 **Actors/Channels**。

***
