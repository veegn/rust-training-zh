# 总结与速查表 🟡

## 异步 Rust 快速参考卡

### 异步思维模型

```text
┌─────────────────────────────────────────────────────┐
│  async fn → 状态机 (enum) → 实现 Future trait        │
│  .await   → 轮询 (poll) 内部的 future                │
│  执行器   → 循环 { 轮询(); 睡眠直到被唤醒(); }         │
│  唤醒器   → “嘿执行器，请再次轮询我”                  │
│  Pin      → “承诺我不会在内存中移动”                 │
└─────────────────────────────────────────────────────┘
```

### 常用模式备忘录

| 目标 | 方案 |
|------|-----|
| 并发运行两个 future | `tokio::join!(a, b)` |
| 让两个 future 竞速 | `tokio::select! { ... }` |
| 派生后台任务 | `tokio::spawn(async { ... })` |
| 在异步中运行阻塞代码 | `tokio::task::spawn_blocking(|| { ... })` |
| 限制并发数 | `Semaphore::new(N)` |
| 收集多个任务结果 | `JoinSet` |
| 在任务间共享状态 | `Arc<Mutex<T>>` 或通道 |
| 优雅停机 | `watch::channel` + `select!` |
| 每次处理流中的 N 个项 | `.buffer_unordered(N)` |
| 为 future 设置超时 | `tokio::time::timeout(dur, fut)` |
| 退避重试 | 自定义组合器（参见第 13 章） |

### 固定 (Pinning) 快速指南

| 场景 | 方案 |
|-----------|-----|
| 在堆上固定 future | `Box::pin(fut)` |
| 在栈上固定 future | `tokio::pin!(fut)` |
| 固定一个 `Unpin` 类型 | `Pin::new(&mut val)` — 安全且无开销 |
| 返回固定的 trait 对象 | `-> Pin<Box<dyn Future + Send>>` |

### 通道选择指南

| 通道 | 生产者 | 消费者 | 值 | 适用场景 |
|---------|-----------|-----------|--------|----------|
| `mpsc` | 多 (N) | 1 | 流 | 工作队列、事件总线 |
| `oneshot` | 1 | 1 | 单个 | 请求/响应、完成通知 |
| `broadcast` | 多 (N) | 多 (N) | 全部 | 扇出通知、多点停机信号 |
| `watch` | 1 | 多 (N) | 最新值 | 配置分发、健康状态 |

### Mutex 选择指南

| Mutex | 适用场景 |
|-------|----------|
| `std::sync::Mutex` | 临界区极短，且**绝不**跨越 `.await` |
| `tokio::sync::Mutex` | 必须跨越 `.await` 持有锁 |
| `parking_lot::Mutex` | 高争用、无 `.await`、追求极致性能 |
| `tokio::sync::RwLock` | 多读少写，且需跨越 `.await` |

### 决策树概览

```text
需要并发吗？
├── I/O 密集型 → async/await
├── CPU 密集型 → rayon 或 std::thread
└── 混合型 → 对 CPU 部分使用 spawn_blocking

选择哪个运行时？
├── 服务器应用 → tokio
├── 库开发 → 运行时无关 (使用 futures 库)
├── 嵌入式 → embassy
└── 最小化实现 → smol

需要并发几个 Future？
├── 满足 'static + Send → tokio::spawn
├── 满足 'static + !Send → LocalSet
├── 不满足 'static → FuturesUnordered
└── 需要跟踪/批量关闭 → JoinSet
```

### 常见错误与对策

| 错误 | 原因 | 修复 |
|-------|-------|-----|
| `future is not Send` | 跨越 `.await` 持有了 `!Send` 类型 | 缩小作用域或切换到 `current_thread` 运行时 |
| `does not live long enough` | `tokio::spawn` 强制要求 `'static` | 用 `Arc` 包装、`clone()` 或改用 `FuturesUnordered` |
| `Future is not implemented` | 遗漏了 `.await` | 为异步函数后缀补充 `.await` |
| 程序静默挂起 | 忘记手动触发 `waker.wake()` | 确保每个 `Pending` 分支都有通路唤醒执行器 |

### 进一步学习资源

- [Tokio 官方系列教程](https://tokio.rs/tokio/tutorial)
- [Rust 官方异步指南 (Async Book)](https://rust-lang.github.io/async-book/)
- [Mini-Redis 源码实战项目](https://github.com/tokio-rs/mini-redis)
- [Tower 中间件架构文档](https://docs.rs/tower)

***
