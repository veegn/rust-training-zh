# Async Rust：从 Future 到生产实践

## 讲师简介

- Microsoft SCHIE（Silicon and Cloud Hardware Infrastructure Engineering）团队首席固件架构师
- 在安全、系统编程（固件、操作系统、虚拟机监控器）、CPU 与平台架构以及 C++ 系统方面拥有丰富经验
- 2017 年在 AWS EC2 开始使用 Rust，此后一直深度投入并持续使用这门语言

---

这是一本关于 Rust 异步编程的深度指南。不同于许多从 `tokio::main` 直接入门、对内部机制一笔带过的教程，本书从第一性原理展开：`Future` trait、轮询、状态机，然后逐步过渡到真实世界中的模式、运行时选型以及生产环境常见陷阱。

## 适合谁阅读
- 能写同步 Rust，但对 async 仍感到困惑的 Rust 开发者
- 熟悉 C#、Go、Python 或 JavaScript 中 `async/await`，但不了解 Rust 模型的开发者
- 被 `Future is not Send`、`Pin<Box<dyn Future>>` 或“程序为什么卡住了”这些问题困扰过的人

## 前置知识

你应当熟悉以下内容：

- 所有权、借用和生命周期
- Trait 与泛型（包括 `impl Trait`）
- 使用 `Result<T, E>` 与 `?` 操作符
- 基础多线程（`std::thread::spawn`、`Arc`、`Mutex`）

不需要事先具备 async Rust 经验。

## 如何使用本书

**第一次阅读建议按顺序进行。** 第一部分和第二部分是逐层递进的集。每章都包含：

| 标记 | 含义 |
|--------|---------|
| 🟢 | 初级：基础概念 |
| 🟡 | 中级：依赖前文内容 |
| 🔶 | 高级：深入内部机制或生产模式 |

每章包括：

- 顶部的 **“你将学到什么”** 区块
- 适合视觉学习者的 **Mermaid 图示**
- 带隐藏答案的 **内联练习**
- 总结核心概念的 **关键要点**
- 指向相关章节的 **交叉引用**

## 学习节奏建议

| 章节 | 主题 | 建议时间 | 检查点 |
|----------|-------|----------------|------------|
| 1-5 | Async 如何工作 | 6-8 小时 | 你可以解释 `Future`、`Poll`、`Pin`，以及 Rust 为什么没有内建运行时 |
| 6-10 | 生态系统 | 6-8 小时 | 你可以手写 future、选择运行时并使用 tokio API |
| 11-13 | 生产级 Async | 6-8 小时 | 你可以编写包含流、正确错误处理和优雅关闭能力的生产级异步代码 |
| 综合项目 | 聊天服务器 | 4-6 小时 | 你已经构建出整合所有概念的真实异步应用 |

**预计总时长：22-30 小时**

## 练习建议

每个内容章节都包含内联练习。综合项目（第 16 章）会把所有内容整合进一个项目。为了获得最佳学习效果：

1. **先做题，再看答案**，真正的学习往往发生在卡住的时候
2. **手敲代码，不要复制粘贴**，Rust 语法需要肌肉记忆
3. **运行每个示例**，可以先用 `cargo new async-exercises` 边学边试

## 目录

### 第一部分：Async 如何工作

- [1. 为什么 Rust 中的 Async 与众不同](ch01-why-async-is-different-in-rust.md) 🟢 - 核心差异：Rust 没有内建运行时
- [2. Future Trait](ch02-the-future-trait.md) 🟡 - `poll()`、`Waker` 以及让一切运作起来的契约
- [3. poll 的工作机制](ch03-how-poll-works.md) 🟡 - 轮询状态机与一个最小执行器
- [4. Pin 与 Unpin](ch04-pin-and-unpin.md) 🔶 - 为什么自引用结构体需要 pin
- [5. 状态机真相](ch05-the-state-machine-reveal.md) 🟢 - 编译器究竟会从 `async fn` 生成什么

### 第二部分：生态系统

- [6. 手写 Future](ch06-building-futures-by-hand.md) 🟡 - 从零实现 TimerFuture、Join 与 Select
- [7. 执行器与运行时](ch07-executors-and-runtimes.md) 🟡 - tokio、smol、async-std、embassy 等运行时如何选择
- [8. Tokio 深入解析](ch08-tokio-deep-dive.md) 🟡 - 运行时类型、spawn、通道与同步原语
- [9. Tokio 不适用的场景](ch09-when-tokio-isnt-the-right-fit.md) 🟡 - LocalSet、FuturesUnordered 与运行时无关设计
- [10. 异步 Trait](ch10-async-traits.md) 🟡 - RPITIT、动态分发、trait_variant 与异步闭包

### 第三部分：生产级 Async

- [11. Stream 与 AsyncIterator](ch11-streams-and-asynciterator.md) 🟡 - 异步迭代、AsyncRead/Write 与流组合器
- [12. 常见陷阱](ch12-common-pitfalls.md) 🔶 - 9 类生产环境 bug 及其规避方式
- [13. 生产实践模式](ch13-production-patterns.md) 🔶 - 优雅关闭、背压与 Tower 中间件

### 附录

- [总结与速查卡](ch15-summary-and-reference-card.md) - 快速查询表与决策树
- [综合项目：异步聊天服务器](ch16-capstone-project.md) - 构建一个完整的异步应用

***
