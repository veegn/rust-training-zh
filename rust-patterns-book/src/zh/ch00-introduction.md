[English Original](../en/ch00-introduction.md)

# Rust 设计模式与工程实践 (Rust Patterns & Engineering How-Tos)

## 讲师简介

- Microsoft SCHIE（硅与云硬件基础设施工程）团队首席固件架构师
- 资深行业专家，在安全、系统编程（固件、操作系统、管理程序）、CPU 与平台架构以及 C++ 系统领域拥有深厚背景
- 自 2017 年（于 AWS EC2）起开始使用 Rust 编程，从此便深深爱上了这门语言

---

这是一本针对中高级 Rust 模式的实战指南，涵盖了真实代码库中出现的各种案例。这不是一本语言入门教程 —— 它默认你已经掌握了 Rust 基础，并希望进一步提升。每一章节都剥离出一个核心概念，解释“何时”以及“为何”使用它，并提供可编译的示例及随堂练习。

## 适用对象

- 读完《Rust 原理（The Rust Programming Language）》但仍苦恼于“我该如何设计这个系统？”的开发者
- 正在将生产系统从 C++/C# 迁移至 Rust 的工程师
- 在泛型、Trait 约束或生命周期错误面前碰壁，希望获得系统性工具集的任何人

## 先决条件

在开始之前，你应该已经熟悉：
- 所有权（Ownership）、借用（Borrowing）和生命周期（Lifetimes）的基础知识
- 枚举（Enums）、模式匹配以及 `Option`/`Result`
- 结构体、方法以及基础 Trait（`Display`, `Debug`, `Clone`）
- Cargo 基础操作：`cargo build`, `cargo test`, `cargo run`

## 本书阅读指南

### 难度等级说明

每一章都标记了难度等级：

| 符号 | 等级 | 含义 |
|--------|-------|---------|
| 🟢 | 基础篇 | 每个 Rust 开发者都必须掌握的核心概念 |
| 🟡 | 进阶篇 | 生产级代码库中常用的模式 |
| 🔴 | 高级篇 | 语言底层的深刻机制 —— 可根据需要反复研读 |

### 学习路线图

| 章节 | 主题 | 建议耗时 | 重点核对 |
|----------|-------|----------------|------------|
| **第一部分：类型级模式** | | | |
| 1. 泛型 🟢 | 单态化、常量泛型、`const fn` | 1–2 小时 | 能解释何时 `dyn Trait` 优于泛型 |
| 2. Trait 🟡 | 关联类型、GATs、覆盖实现、虚表 | 3–4 小时 | 能设计带有关联类型的 Trait |
| 3. Newtype 与类型状态模式 🟡 | 零成本安全、编译时状态机 | 2–3 小时 | 能构建类型状态构造者（Builder）模式 |
| 4. PhantomData 🔴 | 生命周期烙印、型变（Variance）、Drop 检查 | 2–3 小时 | 能解释为何 `PhantomData<fn(T)>` 与 `PhantomData<T>` 不同 |
| **第二部分：并发与运行时** | | | |
| 5. 通道（Channels） 🟢 | `mpsc`, crossbeam, `select!`, Actor | 1–2 小时 | 能实现基于通道的工作线程池 |
| 6. 并发 🟡 | 线程、rayon、Mutex、RwLock、原子操作 | 2–3 小时 | 能根据场景选择正确的同步原语 |
| 7. 闭包 🟢 | `Fn`/`FnMut`/`FnOnce`, 组合器 | 1–2 小时 | 能编写接受闭包的高阶函数 |
| 8. 函数式 vs 命题式 🟡 | 组合器、迭代器适配器、函数式模式 | 2–3 小时 | 能解释何时函数式风格优于命题式 |
| 9. 智能指针 🟡 | Box, Rc, Arc, RefCell, Cow, Pin | 2–3 小时 | 能解释每种智能指针的适用场景 |
| **第三部分：系统与生产** | | | |
| 10. 错误处理 🟢 | thiserror, anyhow, `?` 操作符 | 1–2 小时 | 能设计错误类型层次结构 |
| 11. 序列化 🟡 | serde, 零拷贝, 二进制数据 | 2–3 小 byte | 能编写自定义的 serde 反序列化器 |
| 12. Unsafe 🔴 | 超能力、FFI、内存不安全陷阱、分配器 | 2–3 小时 | 能将 Unsafe 代码包装在健全的安全 API 中 |
| 13. 宏 🟡 | `macro_rules!`, 过程宏, `syn`/`quote` | 2–3 小时 | 能编写带有 `tt` 匹配的声明式宏 |
| 14. 测试 🟢 | 单元/集成/文档测试, proptest, criterion | 1–2 小时 | 能设置基于属性的测试（Property-based testing） |
| 15. API 设计 🟡 | 模块布局、人体工学 API、功能标志（Feature flags） | 2–3 小时 | 能应用“解析而非校验（parse, don't validate）”模式 |
| 16. 异步 🔴 | Future, Tokio, 常见陷阱 | 1–2 小时 | 能识别异步反模式（Anti-patterns） |
| **附录** | | | |
| 总结与速查表 | 快速查看 Trait 约束、生命周期及模式 | 随时查阅 | — |
| 终极实战项目 | 类型安全的任务调度器 | 4–6 小时 | 提交一份可运行的实现版本 |

**建议总耗时**：完成所有学习与练习约需 30–45 小时。

---

## 目录

### 第一部分：类型级模式

**[1. 泛型：全景概览](ch01-generics-the-full-picture.md)** 🟢
单态化、代码膨胀的权衡、泛型 vs 枚举 vs Trait 对象、常量泛型、`const fn`。

**[2. 深入理解 Trait](ch02-traits-in-depth.md)** 🟡
关联类型、GATs、覆盖实现（Blanket impls）、标记 Trait、虚表、HRTBs、扩展 Trait、枚举分发。

**[3. Newtype 与类型状态（Type-State）模式](ch03-the-newtype-and-type-state-patterns.md)** 🟡
零成本类型安全、编译时状态机、构造者模式、配置 Trait。

**[4. PhantomData：不携带数据的类型](ch04-phantomdata-types-that-carry-no-data.md)** 🔴
生命周期烙印、单位度量模式、Drop 检查、型变（Variance）。

### 第二部分：并发与运行时

**[5. 通道与消息传递](ch05-channels-and-message-passing.md)** 🟢
`std::sync::mpsc`, crossbeam, `select!`, 背压控制, Actor 模式。

**[6. 并发 vs 并行 vs 线程](ch06-concurrency-vs-parallelism-vs-threads.md)** 🟡
OS 线程、作用域线程、rayon、Mutex/RwLock/原子操作、条件变量、OnceLock、无锁模式。

**[7. 闭包与高阶函数](ch07-closures-and-higher-order-functions.md)** 🟢
`Fn`/`FnMut`/`FnOnce`, 闭包作为参数/返回值, 组合器, 高阶 API。

**[8. 函数式 vs 命题式：何为优雅？](ch08-functional-vs-imperative-when-elegance-wins.md)** 🟡
组合器、迭代器适配器、函数式模式。

**[9. 智能指针与内部可变性](ch09-smart-pointers-and-interior-mutability.md)** 🟡
Box, Rc, Arc, Weak, Cell/RefCell, Cow, Pin, ManuallyDrop。

### 第三部分：系统与生产

**[10. 错误处理模式](ch10-error-handling-patterns.md)** 🟢
thiserror vs anyhow, `#[from]`, `.context()`, `?` 操作符, Panics。

**[11. 序列化、零拷贝与二进制数据](ch11-serialization-zero-copy-and-binary-data.md)** 🟡
serde 基础, 枚举表示, 零拷贝反序列化, `repr(C)`, `bytes::Bytes`。

**[12. Unsafe Rust：受控的危险](ch12-unsafe-rust-controlled-danger.md)** 🔴
五大超能力、健全的抽象、FFI、内存不安全陷阱、Arena/Slab 分配器。

**[13. 宏：编写代码的代码](ch13-macros-code-that-writes-code.md)** 🟡
`macro_rules!`, 何时（及何时不）使用宏, 过程宏, Derive 宏, `syn`/`quote`。

**[14. 测试与基准测试模式](ch14-testing-and-benchmarking-patterns.md)** 🟢
单元/集成/文档测试, proptest, criterion, Mock 策略。

**[15. Crate 架构与 API 设计](ch15-crate-architecture-and-api-design.md)** 🟡
模块布局、API 设计清单、符合人体工学的参数、功能标志（Feature flags）、工作区（Workspaces）。

**[16. 异步基础](ch16-asyncawait-essentials.md)** 🔴
Future, Tokio 快速入门, 常见陷阱。（关于更深度的异步覆盖，请参阅我们的《异步 Rust 训练》。）

### 附录

**[总结与速查表](ch18-summary-and-reference-card.md)**
模式决策指南、Trait 约束速查、生命周期洗去规则、延伸阅读。

**[实战项目：类型安全的任务调度器](ch19-capstone-project.md)**
将泛型、Trait、类型状态、通道、错误处理和测试整合到一个完整的系统中。

***
