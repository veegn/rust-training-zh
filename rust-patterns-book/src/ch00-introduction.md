# Rust Patterns & Engineering How-Tos / Rust 模式与工程实践手册

## Speaker Intro / 讲师简介

- Principal Firmware Architect in Microsoft SCHIE (Silicon and Cloud Hardware Infrastructure Engineering) team / Microsoft SCHIE（Silicon and Cloud Hardware Infrastructure Engineering）团队首席固件架构师
- Industry veteran with expertise in security, systems programming (firmware, operating systems, hypervisors), CPU and platform architecture, and C++ systems / 在安全、系统编程（固件、操作系统、虚拟机监控器）、CPU 与平台架构以及 C++ 系统方面经验丰富
- Started programming in Rust in 2017 (@AWS EC2), and have been in love with the language ever since / 2017 年在 AWS EC2 开始使用 Rust，此后长期深度投入

---

A practical guide to intermediate-and-above Rust patterns that arise in real codebases. This is not a language tutorial - it assumes you can write basic Rust and want to level up. Each chapter isolates one concept, explains when and why to use it, and provides compilable examples with inline exercises.

这是一本聚焦真实代码库中常见中高级 Rust 模式的实用指南。它不是语言入门教程，而是假设你已经能编写基础 Rust，并希望继续进阶。每章聚焦一个概念，解释何时使用、为何使用，并提供可编译示例与内联练习。

## Who This Is For / 适合谁阅读

- Developers who have finished *The Rust Programming Language* but struggle with "how do I actually design this?" / 已经读完 *The Rust Programming Language*，但仍困惑“真实系统到底该怎么设计”的开发者
- C++/C# engineers translating production systems into Rust / 正在把生产系统从 C++/C# 迁移到 Rust 的工程师
- Anyone who has hit a wall with generics, trait bounds, or lifetime errors and wants a systematic toolkit / 被泛型、trait 约束或生命周期报错卡住，希望建立系统化工具箱的人

## Prerequisites / 前置知识

Before starting, you should be comfortable with:

开始之前，你应当熟悉以下内容：

- Ownership, borrowing, and lifetimes (basic level) / 所有权、借用与生命周期（基础层面）
- Enums, pattern matching, and `Option`/`Result` / 枚举、模式匹配以及 `Option`/`Result`
- Structs, methods, and basic traits (`Display`, `Debug`, `Clone`) / 结构体、方法与基础 trait（如 `Display`、`Debug`、`Clone`）
- Cargo basics: `cargo build`, `cargo test`, `cargo run` / Cargo 基础：`cargo build`、`cargo test`、`cargo run`

## How to Use This Book / 如何使用本书

### Difficulty Legend / 难度说明

Each chapter is tagged with a difficulty level:

每章都带有难度标记：

| Symbol / 标记 | Level / 等级 | Meaning / 含义 |
|--------|-------|---------|
| 🟢 | Fundamentals / 基础 | Core concepts every Rust developer needs / 每个 Rust 开发者都需要掌握的核心概念 |
| 🟡 | Intermediate / 中级 | Patterns used in production codebases / 生产代码中常见的模式 |
| 🔶 | Advanced / 高级 | Deep language mechanics - revisit as needed / 深入语言机制，建议按需反复回看 |

### Pacing Guide / 学习节奏建议

| Chapters / 章节 | Topic / 主题 | Suggested Time / 建议时间 | Checkpoint / 检查点 |
|----------|-------|----------------|------------|
| **Part I: Type-Level Patterns / 类型层模式** | | | |
| 1. Generics 🟢 | Monomorphization, const generics, `const fn` / 单态化、const 泛型、`const fn` | 1-2 hours / 1-2 小时 | Can explain when `dyn Trait` beats generics / 能解释何时 `dyn Trait` 比泛型更合适 |
| 2. Traits 🟡 | Associated types, GATs, blanket impls, vtables / 关联类型、GAT、blanket impl、vtable | 3-4 hours / 3-4 小时 | Can design a trait with associated types / 能设计带关联类型的 trait |
| 3. Newtype & Type-State 🟡 | Zero-cost safety, compile-time FSMs / 零成本安全、编译期有限状态机 | 2-3 hours / 2-3 小时 | Can build a type-state builder pattern / 能实现 type-state builder 模式 |
| 4. PhantomData 🔶 | Lifetime branding, variance, drop check / 生命周期品牌化、变型、drop check | 2-3 hours / 2-3 小时 | Can explain why `PhantomData<fn(T)>` differs from `PhantomData<T>` / 能解释 `PhantomData<fn(T)>` 与 `PhantomData<T>` 的区别 |
| **Part II: Concurrency & Runtime / 并发与运行时** | | | |
| 5. Channels 🟢 | `mpsc`, crossbeam, `select!`, actors / `mpsc`、crossbeam、`select!`、actor | 1-2 hours / 1-2 小时 | Can implement a channel-based worker pool / 能实现基于 channel 的 worker pool |
| 6. Concurrency 🟡 | Threads, rayon, Mutex, RwLock, atomics / 线程、rayon、Mutex、RwLock、原子类型 | 2-3 hours / 2-3 小时 | Can pick the right sync primitive for a scenario / 能为具体场景选择合适的同步原语 |
| 7. Closures 🟢 | `Fn`/`FnMut`/`FnOnce`, combinators / `Fn`/`FnMut`/`FnOnce`、组合器 | 1-2 hours / 1-2 小时 | Can write a higher-order function that accepts closures / 能编写接受闭包的高阶函数 |
| 8. Smart Pointers 🟡 | Box, Rc, Arc, RefCell, Cow, Pin / Box、Rc、Arc、RefCell、Cow、Pin | 2-3 hours / 2-3 小时 | Can explain when to use each smart pointer / 能解释每种智能指针的适用场景 |
| **Part III: Systems & Production / 系统与生产实践** | | | |
| 9. Error Handling 🟢 | thiserror, anyhow, `?` operator / thiserror、anyhow、`?` 操作符 | 1-2 hours / 1-2 小时 | Can design an error type hierarchy / 能设计错误类型层次结构 |
| 10. Serialization 🟡 | serde, zero-copy, binary data / serde、零拷贝、二进制数据 | 2-3 hours / 2-3 小时 | Can write a custom serde deserializer / 能写自定义 serde 反序列化器 |
| 11. Unsafe 🔶 | Superpowers, FFI, UB pitfalls, allocators / 五种“超能力”、FFI、UB 陷阱、分配器 | 2-3 hours / 2-3 小时 | Can wrap unsafe code in a sound safe API / 能把 unsafe 代码封装成健全的安全 API |
| 12. Macros 🟡 | `macro_rules!`, proc macros, `syn`/`quote` / `macro_rules!`、过程宏、`syn`/`quote` | 2-3 hours / 2-3 小时 | Can write a declarative macro with `tt` munching / 能写出使用 `tt` munching 的声明式宏 |
| 13. Testing 🟢 | Unit/integration/doc tests, proptest, criterion / 单元测试、集成测试、文档测试、proptest、criterion | 1-2 hours / 1-2 小时 | Can set up property-based tests / 能搭建属性测试 |
| 14. API Design 🟡 | Module layout, ergonomic APIs, feature flags / 模块布局、易用 API、feature 标志 | 2-3 hours / 2-3 小时 | Can apply the "parse, don't validate" pattern / 能应用“先解析，不要事后校验”的模式 |
| 15. Async 🔶 | Futures, Tokio, common pitfalls / Future、Tokio、常见陷阱 | 1-2 hours / 1-2 小时 | Can identify async anti-patterns / 能识别 async 反模式 |
| **Appendices / 附录** | | | |
| Reference Card / 速查卡 | Quick-look trait bounds, lifetimes, patterns / trait 约束、生命周期、模式速查 | As needed / 按需查阅 | - |
| Capstone Project / 综合项目 | Type-safe task scheduler / 类型安全任务调度器 | 4-6 hours / 4-6 小时 | Submit a working implementation / 完成一个可运行实现 |

**Total estimated time**: 30-45 hours for thorough study with exercises.

**预计总时长**：若完整学习并完成练习，大约需要 30-45 小时。

### Working Through Exercises / 练习建议

Every chapter ends with a hands-on exercise. For maximum learning:

每章末尾都有动手练习。为了获得最佳学习效果：

1. **Try it yourself first** - spend at least 15 minutes before opening the solution / **先自己尝试**，至少坚持 15 分钟再打开答案
2. **Type the code** - don't copy-paste; typing builds muscle memory / **手敲代码**，不要复制粘贴，输入本身会强化记忆
3. **Modify the solution** - add a feature, change a constraint, break something on purpose / **修改答案**，比如增加功能、调整约束，或者故意改坏再修复
4. **Check cross-references** - most exercises combine patterns from multiple chapters / **查看交叉引用**，大多数练习都会结合多章模式

The capstone project (Appendix) ties together patterns from across the book into a single, production-quality system.

综合项目（附录）会把整本书中的多个模式整合成一个具有生产质量的完整系统。

## Table of Contents / 目录

### Part I: Type-Level Patterns / 第一部分：类型层模式

**[1. Generics - The Full Picture / 1. 泛型全景图](ch01-generics-the-full-picture.md)** 🟢  
Monomorphization, code bloat trade-offs, generics vs enums vs trait objects, const generics, `const fn`.  
单态化、代码膨胀权衡、泛型与枚举和 trait 对象的取舍、const 泛型、`const fn`。

**[2. Traits In Depth / 2. Trait 深入解析](ch02-traits-in-depth.md)** 🟡  
Associated types, GATs, blanket impls, marker traits, vtables, HRTBs, extension traits, enum dispatch.  
关联类型、GAT、blanket impl、标记 trait、vtable、HRTB、扩展 trait、枚举分发。

**[3. The Newtype and Type-State Patterns / 3. Newtype 与 Type-State 模式](ch03-the-newtype-and-type-state-patterns.md)** 🟡  
Zero-cost type safety, compile-time state machines, builder patterns, config traits.  
零成本类型安全、编译期状态机、builder 模式、配置 trait。

**[4. PhantomData - Types That Carry No Data / 4. `PhantomData`：不承载数据的类型](ch04-phantomdata-types-that-carry-no-data.md)** 🔶  
Lifetime branding, unit-of-measure pattern, drop check, variance.  
生命周期品牌化、计量单位模式、drop check 与变型。

### Part II: Concurrency & Runtime / 第二部分：并发与运行时

**[5. Channels and Message Passing / 5. Channel 与消息传递](ch05-channels-and-message-passing.md)** 🟢  
`std::sync::mpsc`, crossbeam, `select!`, backpressure, actor pattern.  
`std::sync::mpsc`、crossbeam、`select!`、背压与 actor 模式。

**[6. Concurrency vs Parallelism vs Threads / 6. 并发、并行与线程](ch06-concurrency-vs-parallelism-vs-threads.md)** 🟡  
OS threads, scoped threads, rayon, Mutex/RwLock/Atomics, Condvar, OnceLock, lock-free patterns.  
操作系统线程、作用域线程、rayon、Mutex/RwLock/原子类型、Condvar、OnceLock 与无锁模式。

**[7. Closures and Higher-Order Functions / 7. 闭包与高阶函数](ch07-closures-and-higher-order-functions.md)** 🟢  
`Fn`/`FnMut`/`FnOnce`, closures as parameters/return values, combinators, higher-order APIs.  
`Fn`/`FnMut`/`FnOnce`、闭包作为参数和返回值、组合器与高阶 API。

**[8. Smart Pointers and Interior Mutability / 8. 智能指针与内部可变性](ch08-smart-pointers-and-interior-mutability.md)** 🟡  
Box, Rc, Arc, Weak, Cell/RefCell, Cow, Pin, ManuallyDrop.  
Box、Rc、Arc、Weak、Cell/RefCell、Cow、Pin、ManuallyDrop。

### Part III: Systems & Production / 第三部分：系统与生产实践

**[9. Error Handling Patterns / 9. 错误处理模式](ch09-error-handling-patterns.md)** 🟢  
thiserror vs anyhow, `#[from]`, `.context()`, `?` operator, panics.  
thiserror 与 anyhow 的对比、`#[from]`、`.context()`、`?` 操作符与 panic。

**[10. Serialization, Zero-Copy, and Binary Data / 10. 序列化、零拷贝与二进制数据](ch10-serialization-zero-copy-and-binary-data.md)** 🟡  
serde fundamentals, enum representations, zero-copy deserialization, `repr(C)`, `bytes::Bytes`.  
serde 基础、枚举表示方式、零拷贝反序列化、`repr(C)`、`bytes::Bytes`。

**[11. Unsafe Rust - Controlled Danger / 11. Unsafe Rust：受控的危险](ch11-unsafe-rust-controlled-danger.md)** 🔶  
Five superpowers, sound abstractions, FFI, UB pitfalls, arena/slab allocators.  
五种“超能力”、健全抽象、FFI、UB 陷阱、arena/slab 分配器。

**[12. Macros - Code That Writes Code / 12. 宏：生成代码的代码](ch12-macros-code-that-writes-code.md)** 🟡  
`macro_rules!`, when (not) to use macros, proc macros, derive macros, `syn`/`quote`.  
`macro_rules!`、宏的适用与不适用场景、过程宏、派生宏、`syn`/`quote`。

**[13. Testing and Benchmarking Patterns / 13. 测试与基准模式](ch13-testing-and-benchmarking-patterns.md)** 🟢  
Unit/integration/doc tests, proptest, criterion, mocking strategies.  
单元测试、集成测试、文档测试、proptest、criterion 与 mock 策略。

**[14. Crate Architecture and API Design / 14. Crate 架构与 API 设计](ch14-crate-architecture-and-api-design.md)** 🟡  
Module layout, API design checklist, ergonomic parameters, feature flags, workspaces.  
模块布局、API 设计清单、易用参数设计、feature 标志与工作区。

**[15. Async/Await Essentials / 15. Async/Await 核心要点](ch15-asyncawait-essentials.md)** 🔶  
Futures, Tokio quick-start, common pitfalls. (For deep async coverage, see our Async Rust Training.)  
Future、Tokio 快速入门与常见陷阱。（若需深入异步内容，请参考 Async Rust Training。）

### Appendices / 附录

**[Summary and Reference Card / 总结与速查卡](ch17-summary-and-reference-card.md)**  
Pattern decision guide, trait bounds cheat sheet, lifetime elision rules, further reading.  
模式决策指南、trait 约束速查、生命周期省略规则与延伸阅读。

**[Capstone Project: Type-Safe Task Scheduler / 综合项目：类型安全任务调度器](ch18-capstone-project.md)**  
Integrate generics, traits, typestate, channels, error handling, and testing into a complete system.  
将泛型、trait、typestate、channel、错误处理与测试整合为完整系统。

***
