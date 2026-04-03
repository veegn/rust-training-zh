[English Original](../en/ch15-summary-and-reference-card.md)

# 总结与备忘卡 (Reference Card) 🟡

## 快速参考卡

### 异步思维模型

```text
┌─────────────────────────────────────────────────────┐
│  async fn → 状态机 (enum) → 实现 Future Trait        │
│  .await   → 调用内部 future 的 poll() 方法           │
│  执行器    → 循环 { poll(); 睡眠直至被唤醒; }         │
│  Waker    → “嘿，执行器，再次轮询我”                 │
│  Pin      → “我承诺不会在内存中移动位置”             │
└─────────────────────────────────────────────────────┘
```

### 常见模式速查表

| 目标 | 方法 |
|------|-----|
| 并发运行两个 future | `tokio::join!(a, b)` |
| 竞速运行两个 future | `tokio::select! { ... }` |
| 派生一个后台任务 | `tokio::spawn(async { ... })` |
| 在异步中运行阻塞代码 | `tokio::task::spawn_blocking(|| { ... })` |
| 限制并发数量 | `Semaphore::new(N)` |
| 收集多个任务的结果 | `JoinSet` |
| 跨任务共享状态 | `Arc<Mutex<T>>` 或使用通道 |
| 优雅停机 | `watch::channel` + `select!` |
| 每次并发处理 N 个流条目 | `.buffer_unordered(N)` |
| 为 future 设置超时 | `tokio::time::timeout(dur, fut)` |
| 带退避算法的重试 | 自定义组合器 (见第 13 章) |

### 固定 (Pinning) 快速参考

| 场景 | 方法 |
|-----------|-----|
| 在堆上固定 future | `Box::pin(fut)` |
| 在栈上固定 future | `tokio::pin!(fut)` |
| 固定一个 `Unpin` 类型 | `Pin::new(&mut val)` —— 安全且零开销 |
| 返回固定的 trait 对象 | `-> Pin<Box<dyn Future<Output = T> + Send>>` |

### 通道 (Channel) 选择指南

| 通道 | 生产者 | 消费者 | 传输内容 | 适用场景 |
|---------|-----------|-----------|--------|----------|
| `mpsc` | 多个 (N) | 1 | 流 | 工作队列、事件总线 |
| `oneshot` | 1 | 1 | 单个值 | 请求/响应模式、完成通知 |
| `broadcast` | 多个 (N) | 多个 (N) | 所有人收到所有 | 扇出通知、停机广播 |
| `watch` | 1 | 多个 (N) | 仅最新值 | 配置更新、健康状态检查 |

### Mutex 选择指南

| Mutex | 适用场景 |
|-------|----------|
| `std::sync::Mutex` | 持锁时间极短，决不跨越 `.await` |
| `tokio::sync::Mutex` | 必须跨越 `.await` 点持有锁 |
| `parking_lot::Mutex` | 高并发竞争，无 `.await`，极致性能 |
| `tokio::sync::RwLock` | 多读少写，且需要跨越 `.await` |

### 决策快速参考

```text
需要并发？
├── I/O 密集型 → 使用 async/await
├── CPU 密集型 → 使用 rayon / std::thread
└── 混合型 → 为 CPU 部分使用 spawn_blocking

选择运行时？
├── 服务器应用 → 使用 tokio
├── 类库项目 → 运行时无关设计 (使用 futures 包)
├── 嵌入式项目 → 使用 embassy
└── 极简项目 → 使用 smol

需要并发运行 Future？
├── 满足 'static + Send → 使用 tokio::spawn
├── 满足 'static + !Send → 使用 LocalSet
├── 不满足 'static → 使用 FuturesUnordered
└── 需要追踪/中止任务 → 使用 JoinSet
```

### 常见错误信息及修复方案

| 错误信息 | 原因 | 修复方法 |
|-------|-------|-----|
| `future is not Send` | 跨 `.await` 持有了 `!Send` 类型 | 收窄锁作用域或使用 `current_thread` |
| `borrowed value does not live long enough` (在 spawn 中) | `tokio::spawn` 要求 `'static` 生命周期 | 使用 `Arc`、克隆或 `FuturesUnordered` |
| `the trait Future is not implemented for ()` | 遗漏了 `.await` | 在异步调用后补上 `.await` |
| `cannot borrow as mutable` (在 poll 中) | 自引用借用问题 | 正确使用 `Pin<&mut Self>` (见第 4 章) |
| 程序发生静默挂起 | 忘记调用 `waker.wake()` | 确保每个 `Pending` 路径都注册了 waker |

### 延伸阅读

| 资源 | 推荐理由 |
|----------|-----|
| [Tokio 官方教程](https://tokio.rs/tokio/tutorial) | 官方出品 —— 入门首选指南 |
| [Async Book (官方文档)](https://rust-lang.github.io/async-book/) | 从语言层面涵盖 `Future`, `Pin`, `Stream` |
| [Jon Gjengset —— Crust of Rust: async/await](https://www.youtube.com/watch?v=ThjvMReOXYM) | 2 小时带源码实测深度解析 |
| [Alice Ryhl —— 使用 Tokio 构建 Actor](https://ryhl.io/blog/actors-with-tokio/) | 生产级有状态服务的架构模式 |
| [Without Boats —— Pin, Unpin, 及其设计初衷](https://without.boats/blog/pin/) | 核心设计者的原始设计逻辑 |
| [Tokio mini-Redis](https://github.com/tokio-rs/mini-redis) | 完整的异步项目 —— 极具参考价值的代码库 |
| [Tower 官方文档](https://docs.rs/tower) | axum/tonic 使用的中间件/服务模式 |

***
