[English Original](../en/ch16-asyncawait-essentials.md)

# 第 16 章：Async/Await 核心要点 🔴

> **你将学到：**
> - Rust 的 `Future` 特性与 Go 的 goroutine 以及 Python 的 asyncio 有何不同。
> - **Tokio 快速上手**：生成任务、`join!` 以及运行时配置。
> - 常见的异步陷阱及其修复方法。
> - 何时使用 `spawn_blocking` 转移阻塞性工作。

## 16.1 Future、运行时与 `async fn`

Rust 的异步模型与 Go 的 goroutine 或 Python 的 `asyncio` 有着 **本质的区别**。了解以下三个概念即可开始上手：

1. **`Future` 是一个惰性状态机** —— 调用 `async fn` 不会执行任何操作；它会返回一个必须进行轮询 (poll) 的 `Future`。
2. **你需要一个运行时** 来轮询 Future —— 例如 `tokio`、`async-std` 或 `smol`。标准库定义了 `Future` 但不提供运行时。
3. **`async fn` 是语法糖** —— 编译器会将其转换为一个实现了 `Future` 的状态机。

```rust
// Future 只是一个特性：
pub trait Future {
    type Output;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
}

// async fn 脱糖后变为：
// fn fetch_data(url: &str) -> impl Future<Output = Result<Vec<u8>, Error>>
async fn fetch_data(url: &str) -> Result<Vec<u8>, reqwest::Error> {
    let response = reqwest::get(url).await?;  // .await 会让出控制权直至就绪
    let bytes = response.bytes().await?;
    Ok(bytes.to_vec())
}
```

### Tokio 快速上手

```toml
# Cargo.toml
[dependencies]
tokio = { version = "1", features = ["full"] }
```

```rust,ignore
use tokio::time::{sleep, Duration};
use tokio::task;

#[tokio::main]
async fn main() {
    // 生成并发任务 (类似轻量级线程)：
    let handle_a = task::spawn(async {
        sleep(Duration::from_millis(100)).await;
        "任务 A 已完成"
    });

    let handle_b = task::spawn(async {
        sleep(Duration::from_millis(50)).await;
        "任务 B 已完成"
    });

    // 对二者执行 .await —— 它们并发运行，而非顺序运行：
    let (a, b) = tokio::join!(handle_a, handle_b);
    println!("{}, {}", a.unwrap(), b.unwrap());
}
```

### 异步常见陷阱

| 陷阱 | 发生原因 | 修复方法 |
|---------|---------------|-----|
| **异步中阻塞** | `std::thread::sleep` 或 CPU 密集任务阻塞了执行器 | 使用 `tokio::task::spawn_blocking` 或 `rayon` |
| **`Send` 约束错误** | 跨 `.await` 持有的 Future 包含非 Send 类型 (如 `Rc`、`MutexGuard`) | 重构代码，在 `.await` 之前丢弃非 Send 的值 |
| **Future 未轮询** | 调用 `async fn` 但未执行 `.await` 或 spawn —— 导致没有任何反应 | 务必对返回的 Future 执行 `.await` 或 `tokio::spawn` |
| **跨 `.await` 持有 `MutexGuard`** | `std::sync::MutexGuard` 是 `!Send`；异步任务可能会在不同线程上恢复执行 | 使用 `tokio::sync::Mutex` 或在 `.await` 之前手动 drop |
| **意外的顺序执行** | `let a = foo().await; let b = bar().await;` 会由于依次等待而顺序执行 | 使用 `tokio::join!` 或 `tokio::spawn` 来实现并发 |

```rust
// ❌ 阻塞异步执行器：
async fn bad() {
    std::thread::sleep(std::time::Duration::from_secs(5)); // 阻塞整个线程！
}

// ✅ 转移阻塞工作：
async fn good() {
    tokio::task::spawn_blocking(|| {
        std::thread::sleep(std::time::Duration::from_secs(5)); // 在阻塞线程池中运行
    }).await.unwrap();
}
```

> **深度异步覆盖**：关于 `Stream`、`select!`、取消安全性、结构化并发以及 `tower` 中间件，请参阅我们的 **Async Rust 进阶指南**。本节仅涵盖阅读和编写基础异步代码所需的核心内容。

### 任务生成与结构化并发

Tokio 的 `spawn` 会创建一个新的异步任务 —— 类似于 `thread::spawn` 但更轻量：

```rust,ignore
use tokio::task;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    // 生成三个并发任务
    let h1 = task::spawn(async {
        sleep(Duration::from_millis(200)).await;
        "已获取用户个人资料"
    });

    let h2 = task::spawn(async {
        sleep(Duration::from_millis(100)).await;
        "已获取订单历史"
    });

    let h3 = task::spawn(async {
        sleep(Duration::from_millis(150)).await;
        "已获取推荐信息"
    });

    // 并发（而非顺序！）等待所有三项任务
    let (r1, r2, r3) = tokio::join!(h1, h2, h3);
    println!("{}", r1.unwrap());
    println!("{}", r2.unwrap());
    println!("{}", r3.unwrap());
}
```

**`join!` vs `try_join!` vs `select!`**：

| 宏 | 行为 | 使用场景 |
|-------|----------|----------|
| `join!` | 等待所有 Future 完成 | 所有任务都必须完成的情况 |
| `try_join!` | 等待所有任务，但在遇到第一个 `Err` 时短路 | 任务返回 `Result` 的情况 |
| `select!` | 在第一个 Future 完成时返回 | 用于超时处理、任务取消 |

```rust,ignore
use tokio::time::{timeout, Duration};

async fn fetch_with_timeout() -> Result<String, Box<dyn std::error::Error>> {
    let result = timeout(Duration::from_secs(5), async {
        // 模拟慢速网络调用
        tokio::time::sleep(Duration::from_millis(100)).await;
        Ok::<_, Box<dyn std::error::Error>>("数据".to_string())
    }).await??; // 第一个 ? 解包 Elapsed (超时)，第二个 ? 解包内部 Result

    Ok(result)
}
```

### `Send` 约束以及为何 Future 必须满足 `Send`

当你使用 `tokio::spawn` 生成一个 Future 时，它可能会在不同的 OS 线程上恢复执行。这意味着该 Future 必须满足 `Send` 约束。常见陷阱如下：

```rust,ignore
use std::rc::Rc;

async fn not_send() {
    let rc = Rc::new(42); // Rc 不满足 Send
    tokio::time::sleep(std::time::Duration::from_millis(10)).await;
    println!("{}", rc); // rc 跨 .await 被持有 —— 导致 Future 不满足 Send
}

// 修复 1：在 .await 之前丢弃 (Drop)
async fn fixed_drop() {
    let data = {
        let rc = Rc::new(42);
        *rc // 拷贝出其值
    }; // rc 在此处被丢弃
    tokio::time::sleep(std::time::Duration::from_millis(10)).await;
    println!("{}", data); // 只是一个满足 Send 的 i32
}

// 修复 2：使用 Arc 替代 Rc
async fn fixed_arc() {
    let arc = std::sync::Arc::new(42); // Arc 满足 Send
    tokio::time::sleep(std::time::Duration::from_millis(10)).await;
    println!("{}", arc); // ✅ Future 满足 Send
}
```

> **另请参阅：** [第 5 章](ch05-channels-and-message-passing.md) 了解同步通道。[第 6 章](ch06-concurrency-vs-parallelism-vs-threads.md) 了解 OS 线程与异步任务的对比。

> **关键要点 —— 异步**
> - `async fn` 返回一个 **惰性 Future** —— 除非你执行 `.await` 或 spawn 它，否则不会运行任何代码。
> - 在异步上下文中使用 `tokio::task::spawn_blocking` 处理重型 CPU 任务或阻塞性工作。
> - **不要跨 `.await` 持有 `std::sync::MutexGuard`** —— 请改用 `tokio::sync::Mutex`。
> - 被生成 (spawn) 的 Future 必须满足 `Send` 约束 —— 在跨 `.await` 点之前应丢弃非 `Send` 类型。

---

### 练习：带超时的并发获取器 ★★ (~25 分钟)

编写一个异步函数 `fetch_all`，它生成三个 `tokio::spawn` 任务，每个任务都使用 `tokio::time::sleep` 模拟网络调用。使用 `tokio::try_join!` 将三个任务组合在一起，并将其包装在 `tokio::time::timeout(Duration::from_secs(5), ...)` 中。返回 `Result<Vec<String>, ...>`，或者在任何任务失败或截止时间到期时返回错误。

<details>
<summary>🔑 参考答案</summary>

```rust,ignore
use tokio::time::{sleep, timeout, Duration};

async fn fake_fetch(name: &'static str, delay_ms: u64) -> Result<String, String> {
    sleep(Duration::from_millis(delay_ms)).await;
    Ok(format!("{name}: OK"))
}

async fn fetch_all() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let deadline = Duration::from_secs(5);

    let (a, b, c) = timeout(deadline, async {
        let h1 = tokio::spawn(fake_fetch("svc-a", 100));
        let h2 = tokio::spawn(fake_fetch("svc-b", 200));
        let h3 = tokio::spawn(fake_fetch("svc-c", 150));
        tokio::try_join!(h1, h2, h3)
    })
    .await??;

    Ok(vec![a?, b?, c?])
}

#[tokio::main]
async fn main() {
    let results = fetch_all().await.unwrap();
    for r in &results {
        println!("{r}");
    }
}
```

</details>

***
