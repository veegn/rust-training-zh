[English Original](../en/ch19-capstone-project.md)

# 19. 综合项目：类型安全的任务调度器 ★★★ 🟢

本项目将本书所学的所有模式整合到一个生产级别的系统当中。你将构建一个 **类型安全的并发任务调度器**。

### 项目特性
1. **类型化生命周期**：使用 **类型状态模式 (Type-State Pattern)** 使任务经历 `Pending → Running → Completed` 状态。
2. **并发工作线程**：一组工作线程池从共享通道中拉取任务。
3. **安全性优先**：无效的状态转换（例如运行一个已完成的任务）将导致 **编译时错误**。
4. **错误传播**：使用 `thiserror` 为调度器维护一个结构化的错误层级。

---

### 第一步：任务状态机
为各状态定义标记 (markers) 并创建一个泛型的 `Task` 结构体。使用 `PhantomData` 在无运行时开销的情况下跟踪任务状态。

```rust
struct Pending;
struct Running;
struct Completed;

struct Task<S, R> {
    id: u64,
    _state: PhantomData<S>,
    _result: PhantomData<R>,
}

impl<R> Task<Pending, R> {
    fn start(self) -> Task<Running, R> { ... }
}
```

---

### 第二步：调度器 (Scheduler)
调度器管理一个 `Sender` 用于派发 `WorkItem`，以及一个 `Receiver` 用于收集结果。

```rust
struct Scheduler<R> {
    sender: mpsc::Sender<WorkItem<R>>,
    results: mpsc::Receiver<TaskResult<R>>,
}
```

---

### 第三步：工作线程实现 (Worker Implementation)
每个工作线程在各自的线程中运行，通过锁定共享的接收端 (receiver) 来拉取下一个可用任务。

---

### 第四步：验证
编写一组测试用例：
1. 提交 10 个并发任务并验证其结果。
2. 确保内部失败的任务返回结构化的 `Err` 结果，而不是发生 panic。
3. 使用 `proptest` 对包含不同任务数量的任务调度器进行模糊测试。

***
