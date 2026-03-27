# Async Rust: From Futures to Production / Async Rust：从 Future 到生产实践

## Speaker Intro / 讲师简介

- Principal Firmware Architect in Microsoft SCHIE (Silicon and Cloud Hardware Infrastructure Engineering) team / Microsoft SCHIE（Silicon and Cloud Hardware Infrastructure Engineering）团队首席固件架构师
- Industry veteran with expertise in security, systems programming (firmware, operating systems, hypervisors), CPU and platform architecture, and C++ systems / 在安全、系统编程（固件、操作系统、虚拟机监控器）、CPU 与平台架构以及 C++ 系统方面拥有丰富经验
- Started programming in Rust in 2017 (@AWS EC2), and have been in love with the language ever since / 2017 年在 AWS EC2 开始使用 Rust，此后一直深度投入并持续使用这门语言

---

A deep-dive guide to asynchronous programming in Rust. Unlike most async tutorials that start with `tokio::main` and hand-wave the internals, this guide builds understanding from first principles - the `Future` trait, polling, state machines - then progresses to real-world patterns, runtime selection, and production pitfalls.

这是一本关于 Rust 异步编程的深度指南。不同于许多从 `tokio::main` 直接入门、对内部机制一笔带过的教程，本书从第一性原理展开：`Future` trait、轮询、状态机，然后逐步过渡到真实世界中的模式、运行时选型以及生产环境常见陷阱。

## Who This Is For / 适合谁阅读
- Rust developers who can write synchronous Rust but find async confusing / 能写同步 Rust，但对 async 仍感到困惑的 Rust 开发者
- Developers from C#, Go, Python, or JavaScript who know `async/await` but not Rust's model / 熟悉 C#、Go、Python 或 JavaScript 中 `async/await`，但不了解 Rust 模型的开发者
- Anyone who's been bitten by `Future is not Send`, `Pin<Box<dyn Future>>`, or "why does my program hang?" / 被 `Future is not Send`、`Pin<Box<dyn Future>>` 或“程序为什么卡住了”这些问题困扰过的人

## Prerequisites / 前置知识

You should be comfortable with:

你应当熟悉以下内容：

- Ownership, borrowing, and lifetimes / 所有权、借用和生命周期
- Traits and generics (including `impl Trait`) / Trait 与泛型（包括 `impl Trait`）
- Using `Result<T, E>` and the `?` operator / 使用 `Result<T, E>` 与 `?` 操作符
- Basic multi-threading (`std::thread::spawn`, `Arc`, `Mutex`) / 基础多线程（`std::thread::spawn`、`Arc`、`Mutex`）

No prior async Rust experience is needed.

不需要事先具备 async Rust 经验。

## How to Use This Book / 如何使用本书

**Read linearly the first time.** Parts I-II build on each other. Each chapter has:

**第一次阅读建议按顺序进行。** 第一部分和第二部分是逐层递进的。每章都包含：

| Symbol / 标记 | Meaning / 含义 |
|--------|---------|
| 🟢 | Beginner - foundational concept / 初级：基础概念 |
| 🟡 | Intermediate - requires earlier chapters / 中级：依赖前文内容 |
| 🔶 | Advanced - deep internals or production patterns / 高级：深入内部机制或生产模式 |

Each chapter includes:

每章包括：

- A **"What you'll learn"** block at the top / 顶部的 **“你将学到什么”** 区块
- **Mermaid diagrams** for visual learners / 适合视觉学习者的 **Mermaid 图示**
- An **inline exercise** with a hidden solution / 带隐藏答案的 **内联练习**
- **Key Takeaways** summarizing the core ideas / 总结核心概念的 **关键要点**
- **Cross-references** to related chapters / 指向相关章节的 **交叉引用**

## Pacing Guide / 学习节奏建议

| Chapters / 章节 | Topic / 主题 | Suggested Time / 建议时间 | Checkpoint / 检查点 |
|----------|-------|----------------|------------|
| 1-5 | How Async Works / Async 如何工作 | 6-8 hours / 6-8 小时 | You can explain `Future`, `Poll`, `Pin`, and why Rust has no built-in runtime / 你可以解释 `Future`、`Poll`、`Pin`，以及 Rust 为什么没有内建运行时 |
| 6-10 | The Ecosystem / 生态系统 | 6-8 hours / 6-8 小时 | You can build futures by hand, choose a runtime, and use tokio's API / 你可以手写 future、选择运行时并使用 tokio API |
| 11-13 | Production Async / 生产级 Async | 6-8 hours / 6-8 小时 | You can write production-grade async code with streams, proper error handling, and graceful shutdown / 你可以编写包含流、正确错误处理和优雅关闭能力的生产级异步代码 |
| Capstone / 综合项目 | Chat Server / 聊天服务器 | 4-6 hours / 4-6 小时 | You've built a real async application integrating all concepts / 你已经构建出整合所有概念的真实异步应用 |

**Total estimated time: 22-30 hours**

**预计总时长：22-30 小时**

## Working Through Exercises / 练习建议

Every content chapter has an inline exercise. The capstone (Ch 16) integrates everything into a single project. For maximum learning:

每个内容章节都包含内联练习。综合项目（第 16 章）会把所有内容整合进一个项目。为了获得最佳学习效果：

1. **Try the exercise before expanding the solution** - struggling is where learning happens / **先做题，再看答案**，真正的学习往往发生在卡住的时候
2. **Type the code, don't copy-paste** - muscle memory matters for Rust's syntax / **手敲代码，不要复制粘贴**，Rust 语法需要肌肉记忆
3. **Run every example** - `cargo new async-exercises` and test as you go / **运行每个示例**，可以先用 `cargo new async-exercises` 边学边试

## Table of Contents / 目录

### Part I: How Async Works / 第一部分：Async 如何工作

- [1. Why Async is Different in Rust / 1. 为什么 Rust 中的 Async 与众不同](ch01-why-async-is-different-in-rust.md) 🟢 - The fundamental difference: Rust has no built-in runtime / 核心差异：Rust 没有内建运行时
- [2. The Future Trait / 2. `Future` Trait](ch02-the-future-trait.md) 🟡 - `poll()`, `Waker`, and the contract that makes it all work / `poll()`、`Waker` 以及让一切运作起来的契约
- [3. How Poll Works / 3. `poll` 的工作机制](ch03-how-poll-works.md) 🟡 - The polling state machine and a minimal executor / 轮询状态机与一个最小执行器
- [4. Pin and Unpin / 4. `Pin` 与 `Unpin`](ch04-pin-and-unpin.md) 🔶 - Why self-referential structs need pinning / 为什么自引用结构体需要 pin
- [5. The State Machine Reveal / 5. 状态机真相](ch05-the-state-machine-reveal.md) 🟢 - What the compiler actually generates from `async fn` / 编译器究竟会从 `async fn` 生成什么

### Part II: The Ecosystem / 第二部分：生态系统

- [6. Building Futures by Hand / 6. 手写 Future](ch06-building-futures-by-hand.md) 🟡 - TimerFuture, Join, Select from scratch / 从零实现 TimerFuture、Join 与 Select
- [7. Executors and Runtimes / 7. 执行器与运行时](ch07-executors-and-runtimes.md) 🟡 - tokio, smol, async-std, embassy - how to choose / tokio、smol、async-std、embassy 等运行时如何选择
- [8. Tokio Deep Dive / 8. Tokio 深入解析](ch08-tokio-deep-dive.md) 🟡 - Runtime flavors, spawn, channels, sync primitives / 运行时类型、spawn、通道与同步原语
- [9. When Tokio Isn't the Right Fit / 9. Tokio 不适用的场景](ch09-when-tokio-isnt-the-right-fit.md) 🟡 - LocalSet, FuturesUnordered, runtime-agnostic design / LocalSet、FuturesUnordered 与运行时无关设计
- [10. Async Traits / 10. 异步 Trait](ch10-async-traits.md) 🟡 - RPITIT, dyn dispatch, trait_variant, async closures / RPITIT、动态分发、trait_variant 与异步闭包

### Part III: Production Async / 第三部分：生产级 Async

- [11. Streams and AsyncIterator / 11. Stream 与 AsyncIterator](ch11-streams-and-asynciterator.md) 🟡 - Async iteration, AsyncRead/Write, stream combinators / 异步迭代、AsyncRead/Write 与流组合器
- [12. Common Pitfalls / 12. 常见陷阱](ch12-common-pitfalls.md) 🔶 - 9 production bugs and how to avoid them / 9 类生产环境 bug 及其规避方式
- [13. Production Patterns / 13. 生产实践模式](ch13-production-patterns.md) 🔶 - Graceful shutdown, backpressure, Tower middleware / 优雅关闭、背压与 Tower 中间件

### Appendices / 附录

- [Summary and Reference Card / 总结与速查卡](ch15-summary-and-reference-card.md) - Quick-lookup tables and decision trees / 快速查询表与决策树
- [Capstone Project: Async Chat Server / 综合项目：异步聊天服务器](ch16-capstone-project.md) - Build a complete async application / 构建一个完整的异步应用

***
