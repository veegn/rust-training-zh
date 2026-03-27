# Summary and Reference Card / 总结与速查表

## Quick Reference Card / 快速速查表

### Async Mental Model / 异步思维模型

```text
┌─────────────────────────────────────────────────────┐
│  async fn → State Machine (enum) → impl Future     │
│  异步函数 → 状态机 (枚举值) → 实现 Future trait        │
│  .await   → poll() the inner future                 │
│  .await   → 轮询 (poll) 内部的 future                │
│  executor → loop { poll(); sleep_until_woken(); }   │
│  执行器   → 循环 { 轮询(); 睡眠直到被唤醒(); }         │
│  waker    → "hey executor, poll me again"           │
│  唤醒器   → “嘿执行器，请再次轮询我”                  │
│  Pin      → "promise I won't move in memory"        │
│  固定 (Pin) → “承诺我不会在内存中移动”                 │
└─────────────────────────────────────────────────────┘
```

### Common Patterns Cheat Sheet / 常用模式备忘录

| Goal / 目标 | Use / 方案 |
|------|-----|
| Run two futures concurrently / 并发运行两个 future | `tokio::join!(a, b)` |
| Race two futures / 让两个 future 竞速 | `tokio::select! { ... }` |
| Spawn a background task / 派生后台任务 | `tokio::spawn(async { ... })` |
| Run blocking code in async / 在异步中运行阻塞代码 | `tokio::task::spawn_blocking(|| { ... })` |
| Limit concurrency / 限制并发数 | `Semaphore::new(N)` |
| Collect many task results / 收集多个任务结果 | `JoinSet` |
| Share state across tasks / 在任务间共享状态 | `Arc<Mutex<T>>` 或通道 |
| Graceful shutdown / 优雅停机 | `watch::channel` + `select!` |
| Process a stream N-at-a-time / 每次处理流中的 N 个项 | `.buffer_unordered(N)` |
| Timeout a future / 为 future 设置超时 | `tokio::time::timeout(dur, fut)` |
| Retry with backoff / 退避重试 | 自定义组合器（参见第 13 章） |

### Pinning Quick Reference / 固定 (Pinning) 快速指南

| Situation / 场景 | Use / 方案 |
|-----------|-----|
| Pin a future on the heap / 在堆上固定 future | `Box::pin(fut)` |
| Pin a future on the stack / 在栈上固定 future | `tokio::pin!(fut)` |
| Pin an `Unpin` type / 固定一个 `Unpin` 类型 | `Pin::new(&mut val)` — 安全且无开销 |
| Return a pinned trait object / 返回固定的 trait 对象 | `-> Pin<Box<dyn Future<Output = T> + Send>>` |

### Channel Selection Guide / 通道选择指南

| Channel / 通道 | Producers / 生产者 | Consumers / 消费者 | Values / 值 | Use When / 适用场景 |
|---------|-----------|-----------|--------|----------|
| `mpsc` | 多 (N) | 1 | 流 (Stream) | 工作队列、事件总线 |
| `oneshot` | 1 | 1 | 单个 (Single) | 请求/响应、完成通知 |
| `broadcast` | 多 (N) | 多 (N) | 全部接收全部 | 扇出通知、停机信号 |
| `watch` | 1 | 多 (N) | 仅最新值 | 配置更新、健康状态 |

### Mutex Selection Guide / Mutex 选择指南

| Mutex / 互斥锁 | Use When / 适用场景 |
|-------|----------|
| `std::sync::Mutex` | 锁持有时间极短，且绝不跨越 `.await` |
| `tokio::sync::Mutex` | 必须跨越 `.await` 点持有锁 |
| `parking_lot::Mutex` | 高争用、无 `.await`、追求极致性能 |
| `tokio::sync::RwLock` | 多读少写，且锁需跨越 `.await` |

### Decision Quick Reference / 决策指南

```text
Need concurrency? / 需要并发吗？
├── I/O-bound → async/await / I/O 密集型 → async/await
├── CPU-bound → rayon / std::thread / CPU 密集型 → rayon 或 std::thread
└── Mixed → spawn_blocking for CPU parts / 混合型 → 对 CPU 部分使用 spawn_blocking

Choosing runtime? / 选择哪种运行时？
├── Server app → tokio / 服务器应用 → tokio
├── Library → runtime-agnostic (futures crate) / 库 → 运行时无关 (使用 futures 库)
├── Embedded → embassy / 嵌入式 → embassy
└── Minimal → smol / 最小化 → smol

Need concurrent futures? / 需要并发 future 吗？
├── Can be 'static + Send → tokio::spawn / 满足 'static + Send → tokio::spawn
├── Can be 'static + !Send → LocalSet / 满足 'static + !Send → LocalSet
├── Can't be 'static → FuturesUnordered / 不满足 'static → FuturesUnordered
└── Need to track/abort → JoinSet / 需要跟踪/中止 → JoinSet
```

### Common Error Messages and Fixes / 常见错误消息与修复

| Error / 错误 | Cause / 原因 | Fix / 修复 |
|-------|-------|-----|
| `future is not Send` | 跨越 `.await` 持有了 `!Send` 类型 | 缩小作用域使其在 `.await` 前被释放，或使用 `current_thread` 运行时 |
| `borrowed value does not live long enough` (in spawn) | `tokio::spawn` 要求 `'static` 生命周期 | 使用 `Arc`、`clone()` 或 `FuturesUnordered` |
| `the trait Future is not implemented for ()` | 遗漏了 `.await` | 为异步调用添加 `.await` |
| `cannot borrow as mutable` (in poll) | 自引用借用 | 正确使用 `Pin<&mut Self>`（参见第 4 章） |
| Program hangs silently / 程序静默挂起 | 忘记调用 `waker.wake()` | 确保每个 `Pending` 路径都注册并触发了唤醒器 |

### Further Reading / 延伸阅读

| Resource / 资源 | Why / 理由 |
|----------|-----|
| [Tokio Tutorial](https://tokio.rs/tokio/tutorial) | 官方上手指南 —— 对第一个项目来说非常棒 |
| [Async Book (official)](https://rust-lang.github.io/async-book/) | 在语言层面涵盖 `Future`、`Pin` 和 `Stream` |
| [Jon Gjengset — Crust of Rust: async/await](https://www.youtube.com/watch?v=ThjvMReOXYM) | 2 小时的深入解析，包含现场编码 |
| [Alice Ryhl — Actors with Tokio](https://ryhl.io/blog/actors-with-tokio/) | 有状态生产服务的架构模式 |
| [Without Boats — Pin, Unpin, and why Rust needs them](https://without.boats/blog/pin/) | 语言设计者的原始设计思路 |
| [Tokio mini-Redis](https://github.com/tokio-rs/mini-redis) | 完整的异步 Rust 项目 —— 研究级的生产代码 |
| [Tower documentation](https://docs.rs/tower) | axum、tonic 和 hyper 使用的中间件/服务架构 |

***

*End of Async Rust Training Guide / Async Rust 培训指南结束*

