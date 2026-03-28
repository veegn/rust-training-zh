# 9. Tokio 不适用的场景 🟡

> **你将学到：**
> - `'static` 难题：当 `tokio::spawn` 强制你在各处使用 `Arc` 时
> - 针对 `!Send` future 的 `LocalSet`
> - 借用友好的并发工具：`FuturesUnordered`（无需 spawn）
> - 用于管理任务组的 `JoinSet`
> - 编写运行时无关（Runtime-agnostic）的库

```mermaid
graph TD
    START["需要并发运行 Future？"] --> STATIC{"Future 能满足 'static 吗？"}
    STATIC -->|可以| SEND{"Future 满足 Send 吗？"}
    STATIC -->|不行| FU["FuturesUnordered<br/>在当前任务中运行"]
    SEND -->|是| SPAWN["tokio::spawn<br/>多线程并行"]
    SEND -->|否| LOCAL["LocalSet<br/>单线程并行"]
    SPAWN --> MANAGE{"需要跟踪/批量取消任务？"}
    MANAGE -->|是| JOINSET["JoinSet / TaskTracker"]
    MANAGE -->|否| HANDLE["JoinHandle"]

    style START fill:#f5f5f5,stroke:#333,color:#000
    style FU fill:#d4efdf,stroke:#27ae60,color:#000
    style SPAWN fill:#e8f4f8,stroke:#2980b9,color:#000
    style LOCAL fill:#fef9e7,stroke:#f39c12,color:#000
    style JOINSET fill:#e8daef,stroke:#8e44ad,color:#000
    style HANDLE fill:#e8f4f8,stroke:#2980b9,color:#000
```

## 'static Future 难题

Tokio 的 `spawn` 强制要求 future 必须是 `'static`。这意味着你不能在派生任务中借用局部变量：

```rust
async fn process_items(items: &[String]) {
    // ❌ 错误：items 是借用的，不满足 'static
    // for item in items {
    //     tokio::spawn(async {
    //         process(item).await; // 尝试借用 item
    //     });
    // }

    // 😐 方案 1：到处克隆
    for item in items {
        let item = item.clone();
        tokio::spawn(async move {
            process(&item).await;
        });
    }

    // 😐 方案 2：包装 Arc
    let items = Arc::new(items.to_vec());
    for i in 0..items.len() {
        let items = Arc::clone(&items);
        tokio::spawn(async move {
            process(&items[i]).await;
        });
    }
}
```

这确实很累人。在 Go 语言中，你可以随手写一个闭包直接 `go` 出去。但在 Rust 中，所有权系统强制你时刻关注数据的生命周期和归属。

### 局部并发与替代方案

针对 `'static` 带来的繁琐，我们有几套常用的替代方案：

```rust
// 1. tokio::task::LocalSet —— 在当前线程运行 !Send 的任务
use tokio::task::LocalSet;

let local_set = LocalSet::new();
local_set.run_until(async {
    tokio::task::spawn_local(async {
        // 在这里可以使用 Rc、Cell 等非线程安全的类型
        let rc = std::rc::Rc::new(42);
        println!("{rc}");
    }).await.unwrap();
}).await;

// 2. FuturesUnordered —— 无需 spawn 也能实现并发
use futures::stream::{FuturesUnordered, StreamExt};

async fn process_items(items: &[String]) {
    let futures: FuturesUnordered<_> = items
        .iter()
        .map(|item| async move {
            // ✅ 可以自由借用 item —— 没用 spawn，所以不需要 'static！
            process(item).await
        })
        .collect();

    // 驱动所有 future 完成
    futures.for_each(|result| async {
        println!("得到结果: {result:?}");
    }).await;
}

// 3. JoinSet (Tokio 1.21+) —— 任务组管理助手
use tokio::task::JoinSet;

async fn with_joinset() {
    let mut set = JoinSet::new();

    for i in 0..10 {
        set.spawn(async move {
            tokio::time::sleep(Duration::from_millis(100)).await;
            i * 2
        });
    }

    // 逐个获取完成的数据
    while let Some(result) = set.join_next().await {
        println!("任务完成: {:?}", result.unwrap());
    }
}
```

### 编写轻量级、运行时无关的库

如果你在写一个库（Crate），不要强行把用户绑死在 Tokio 上：

```rust
// ❌ 差评：库内部写死了 tokio
pub async fn my_lib_function() {
    tokio::time::sleep(Duration::from_secs(1)).await;
}

// ✅ 好评：库是运行时无关的
pub async fn my_lib_function() {
    // 仅使用 std::future 或 futures crate 提供的通用原语
    do_computation().await;
}
```

> **经验法则**：库应该依赖 `futures` crate。应用程序才去选定具体的运行时（如 `tokio`）。

<details>
<summary><strong>🏋️ 练习：并发与借用</strong> (点击展开)</summary>

**挑战**：写一个函数并发获取字符串列表的长度，要求不克隆原字符串。

<details>
<summary>🔑 参考答案</summary>

```rust
use futures::stream::{FuturesUnordered, StreamExt};

async fn get_lengths(items: &[String]) -> Vec<usize> {
    let futures: FuturesUnordered<_> = items
        .iter()
        .map(|item| async move {
            // 这里我们借用了局部引用 &String
            item.len()
        })
        .collect();

    futures.collect().await
}
```

**关键点**：`FuturesUnordered` 是在当前任务（Task）里推进所有子 future。它没有跨越线程，因此不要求子任务必须是 `'static`。这是处理局部数据并发的最佳利器。

</details>
</details>

> **关键要点：Tokio 不适用的场景**
> - `FuturesUnordered` 允许在当前任务中并发推进多个 future，且支持借用。
> - `LocalSet` 专门用于处理 `!Send` 类型（如 `Rc`）。
> - `JoinSet` 提供了更现代的任务集合管理 API。
> - 编写库时，尽量保持运行时无关，仅依赖 `std::future`。

> **延伸阅读：** [第 8 章：Tokio 深入解析](ch08-tokio-deep-dive.md) 了解什么时候该用 spawn；[第 11 章：流](ch11-streams-and-asynciterator.md) 了解 `buffer_unordered()`。

***
