[English Original](../en/ch00-introduction.md)

# 异步 Rust：从 Future 到生产实践

## 作者介绍

- Microsoft SCHIE（芯片与云硬件基础设施工程）团队首席固件架构师
- 行业资深人士，擅长安全、系统编程（固件、操作系统、管理程序）、CPU 及平台架构以及 C++ 系统
- 2017 年开始在 AWS EC2 使用 Rust 编程，并从此爱上了这门语言

---

这是一份关于 Rust 异步编程的深度指南。与大多数从 `tokio::main` 开始并对内部机制含糊其辞的异步教程不同，本指南从基本原理开始构建理解 —— 包括 `Future` trait、轮询（polling）以及状态机 —— 然后逐步深入到现实世界的模式、运行时选择和生产环境中的陷阱。

## 目标读者
- 能够编写同步 Rust 但觉得异步令人困惑的 Rust 开发者
- 来自 C#、Go、Python 或 JavaScript，了解 `async/await` 但不熟悉 Rust 模型的开发者
- 任何曾被 `Future is not Send`、`Pin<Box<dyn Future>>` 困扰，或疑惑“为什么我的程序挂起了？”的人

## 预备知识

你应该熟悉以下内容：
- 所有权、借用和生命周期
- Trait 和泛型（包括 `impl Trait`）
- 使用 `Result<T, E>` 和 `?` 运算符
- 基础多线程（`std::thread::spawn`、`Arc`、`Mutex`）

不需要先前的异步 Rust 经验。

## 如何阅读本书

**初次阅读请按顺序阅读。** 第一至第三部分环环相扣。每章都有：

| 符号 | 含义 |
|--------|---------|
| 🟢 | 初学者 —— 基础概念 |
| 🟡 | 中级 —— 需要先阅读之前的章节 |
| 🔴 | 高级 —— 深度内部机制或生产模式 |

每章包含：
- 顶部的 **“你将学到”** 区块
- 为视觉学习者准备的 **Mermaid 图表**
- 带有隐藏答案的 **行内练习**
- 总结核心思想的 **关键要诀**
- 相关章节的 **交叉引用**

## 进度指南

| 章节 | 主题 | 建议时间 | 检查点 |
|----------|-------|----------------|------------|
| 1–5 | 异步如何工作 | 6–8 小时 | 你可以解释 `Future`、`Poll`、`Pin` 以及为什么 Rust 没有内置运行时 |
| 6–10 | 生态系统 | 6–8 小时 | 你可以手动构建 Future，选择运行时并使用 tokio 的 API |
| 11–13 | 生产级异步 | 6–8 小时 | 你可以使用流（streams）、正确的错误处理和优雅停机编写生产级异步代码 |
| 案例实践 | 聊天服务器 | 4–6 小时 | 你已经构建了一个集成所有概念的真实异步应用 |

**总预估时间：22–30 小时**

## 完成练习

每个内容章节都有行内练习。案例实践（第 16 章）将所有内容整合到一个项目中。为了获得最佳学习效果：

1. **在展开答案前先尝试练习** —— 挣扎的过程正是学习发生的时候
2. **动手输入代码，不要复制粘贴** —— 肌肉记忆对 Rust 的语法很重要
3. **运行每个示例** —— `cargo new async-exercises` 并边做边测试

## 目录

### 第一部分：异步如何工作

- [1. 为什么异步在 Rust 中不同](ch01-why-async-is-different-in-rust.md) 🟢 —— 根本区别：Rust 没有内置运行时
- [2. Future Trait](ch02-the-future-trait.md) 🟡 —— `poll()`、`Waker` 以及使一切运转的契约
- [3. Poll 如何工作](ch03-how-poll-works.md) 🟡 —— 轮询状态机和最小执行器
- [4. Pin 与 Unpin](ch04-pin-and-unpin.md) 🔴 —— 为什么自引用结构体需要固定（Pinning）
- [5. 揭秘状态机](ch05-the-state-machine-reveal.md) 🟢 —— 编译器从 `async fn` 实际生成了什么

### 第二部分：生态系统

- [6. 手动构建 Future](ch06-building-futures-by-hand.md) 🟡 —— 从零开始实现 TimerFuture、Join、Select
- [7. 执行器与运行时](ch07-executors-and-runtimes.md) 🟡 —— tokio、smol、async-std、embassy —— 如何选择
- [8. Tokio 深度探索](ch08-tokio-deep-dive.md) 🟡 —— 运行时类型、spawn、通道（channels）、同步原语
- [9. 当 Tokio 不适用时](ch09-when-tokio-isnt-the-right-fit.md) 🟡 —— LocalSet、FuturesUnordered、运行时无关设计
- [10. 异步 Trait](ch10-async-traits.md) 🟡 —— RPITIT、动态分发、trait_variant、异步闭包

### 第三部分：生产级异步

- [11. 流与 AsyncIterator](ch11-streams-and-asynciterator.md) 🟡 —— 异步迭代、AsyncRead/Write、流组合器
- [12. 常见陷阱](ch12-common-pitfalls.md) 🔴 —— 9 个生产环境中的漏洞以及如何避免它们
- [13. 生产模式](ch13-production-patterns.md) 🔴 —— 优雅停机、背压（backpressure）、Tower 中间件
- [14. 异步是一种优化，而非架构](ch14-async-is-an-optimization-not-an-architecture.md) 🔴 —— 同步核心 / 异步外壳，以及函数着色的成本

### 附录

- [总结与参考卡片](ch16-summary-and-reference-card.md) —— 快速查询表和决策树
- [案例实践项目：异步聊天服务器](ch17-capstone-project.md) —— 构建一个完整的异步应用

***
