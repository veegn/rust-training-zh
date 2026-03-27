# Rust for C# Programmers: Complete Training Guide / 面向 C# 程序员的 Rust 完整培训指南

A comprehensive guide to learning Rust for developers with C# experience. This guide covers everything from basic syntax to advanced patterns, focusing on the conceptual shifts and practical differences between the two languages.

这是一本面向具有 C# 背景开发者的 Rust 学习指南，覆盖从基础语法到高级模式的完整内容，重点讲解两门语言在思维方式和实际用法上的差异。

## Course Overview / 课程概览
- **The case for Rust** - Why Rust matters for C# developers: performance, safety, and correctness / **为什么选择 Rust**：Rust 为什么值得 C# 开发者学习，重点在性能、安全与正确性
- **Getting started** - Installation, tooling, and your first program / **快速开始**：安装、工具链与第一个程序
- **Basic building blocks** - Types, variables, control flow / **基础构件**：类型、变量、控制流
- **Data structures** - Arrays, tuples, structs, collections / **数据结构**：数组、元组、结构体、集合
- **Pattern matching and enums** - Algebraic data types and exhaustive matching / **模式匹配与枚举**：代数数据类型与穷尽匹配
- **Ownership and borrowing** - Rust's memory management model / **所有权与借用**：Rust 的内存管理模型
- **Modules and crates** - Code organization and dependencies / **模块与 crate**：代码组织与依赖管理
- **Error handling** - Result-based error propagation / **错误处理**：基于 `Result` 的错误传播
- **Traits and generics** - Rust's type system / **Trait 与泛型**：Rust 类型系统
- **Closures and iterators** - Functional programming patterns / **闭包与迭代器**：函数式编程模式
- **Concurrency** - Fearless concurrency with type-system guarantees, async/await deep dive / **并发**：由类型系统保证的无畏并发，以及 async/await 深入解析
- **Unsafe Rust and FFI** - When and how to go beyond safe Rust / **Unsafe Rust 与 FFI**：何时以及如何超越安全 Rust
- **Migration patterns** - Real-world C# to Rust patterns and incremental adoption / **迁移模式**：真实世界中的 C# 到 Rust 模式与渐进迁移
- **Best practices** - Idiomatic Rust for C# developers / **最佳实践**：适合 C# 开发者的 Rust 惯用法

---

# Self-Study Guide / 自学指南

This material works both as an instructor-led course and for self-study. If you're working through it on your own, here's how to get the most out of it.

本材料既适合讲师授课，也适合自学。如果你打算自行学习，下面的建议能帮助你更高效地使用这套内容。

**Pacing recommendations / 学习节奏建议：**

| Chapters / 章节 | Topic / 主题 | Suggested Time / 建议时间 | Checkpoint / 检查点 |
|----------|-------|---------------|------------|
| 1-4 | Setup, types, control flow / 环境、类型、控制流 | 1 day / 1 天 | You can write a CLI temperature converter in Rust / 你可以用 Rust 写一个命令行温度转换器 |
| 5-6 | Data structures, enums, pattern matching / 数据结构、枚举、模式匹配 | 1-2 days / 1-2 天 | You can define an enum with data and `match` exhaustively on it / 你可以定义携带数据的枚举并用 `match` 进行穷尽匹配 |
| 7 | Ownership and borrowing / 所有权与借用 | 1-2 days / 1-2 天 | You can explain *why* `let s2 = s1` invalidates `s1` / 你可以解释为什么 `let s2 = s1` 会使 `s1` 失效 |
| 8-9 | Modules, error handling / 模块、错误处理 | 1 day / 1 天 | You can create a multi-file project that propagates errors with `?` / 你可以创建一个多文件项目并用 `?` 传播错误 |
| 10-12 | Traits, generics, closures, iterators / Trait、泛型、闭包、迭代器 | 1-2 days / 1-2 天 | You can translate a LINQ chain to Rust iterators / 你可以把一段 LINQ 链改写成 Rust 迭代器 |
| 13 | Concurrency and async / 并发与异步 | 1 day / 1 天 | You can write a thread-safe counter with `Arc<Mutex<T>>` / 你可以用 `Arc<Mutex<T>>` 写出线程安全计数器 |
| 14 | Unsafe Rust, FFI, testing / Unsafe Rust、FFI、测试 | 1 day / 1 天 | You can call a Rust function from C# via P/Invoke / 你可以通过 P/Invoke 从 C# 调用 Rust 函数 |
| 15-16 | Migration, best practices, tooling / 迁移、最佳实践、工具链 | At your own pace / 自定节奏 | Reference material - consult as you write real code / 作为参考材料，在实际写代码时查阅 |
| 17 | Capstone project / 综合项目 | 1-2 days / 1-2 天 | You have a working CLI tool that fetches weather data / 你拥有一个可用的命令行天气工具 |

**How to use the exercises / 如何使用练习：**
- Chapters include hands-on exercises in collapsible `<details>` blocks with solutions / 各章包含可折叠 `<details>` 区块中的动手练习与答案
- **Always try the exercise before expanding the solution.** Struggling with the borrow checker is part of learning - the compiler's error messages are your teacher / **总是先做题，再展开答案。** 与借用检查器“较劲”本身就是学习过程，编译器报错就是你的老师
- If you're stuck for more than 15 minutes, expand the solution, study it, then close it and try again from scratch / 如果卡住超过 15 分钟，就先看答案，理解后再关掉并从头重做
- The [Rust Playground](https://play.rust-lang.org/) lets you run code without a local install / [Rust Playground](https://play.rust-lang.org/) 允许你在不本地安装的情况下运行代码

**Difficulty indicators / 难度标记：**
- 🟢 **Beginner** - Direct translation from C# concepts / **初级**：可以直接从 C# 概念迁移过来
- 🟡 **Intermediate** - Requires understanding ownership or traits / **中级**：需要理解所有权或 trait
- 🔶 **Advanced** - Lifetimes, async internals, or unsafe code / **高级**：生命周期、async 内部机制或 unsafe 代码

**When you hit a wall / 遇到难点时：**
- Read the compiler error message carefully - Rust's errors are exceptionally helpful / 仔细阅读编译器错误信息，Rust 的报错通常非常有帮助
- Re-read the relevant section; concepts like ownership (ch7) often click on the second pass / 重读相关章节，像所有权这类概念通常第二遍会更容易理解
- The [Rust standard library docs](https://doc.rust-lang.org/std/) are excellent - search for any type or method / [Rust 标准库文档](https://doc.rust-lang.org/std/) 非常优秀，遇到任何类型或方法都值得查
- For deeper async patterns, see the companion [Async Rust Training](../async-book/) / 如果想深入学习异步模式，请参考配套的 [Async Rust Training](../async-book/)

---

# Table of Contents / 目录

## Part I - Foundations / 第一部分：基础

### 1. Introduction and Motivation / 1. 引言与动机 🟢
- [The Case for Rust for C# Developers / Rust 对 C# 开发者的价值](ch01-introduction-and-motivation.md#the-case-for-rust-for-c-developers)
- [Common C# Pain Points That Rust Addresses / Rust 能解决的 C# 常见痛点](ch01-introduction-and-motivation.md#common-c-pain-points-that-rust-addresses)
- [When to Choose Rust Over C# / 何时选择 Rust 而不是 C#](ch01-introduction-and-motivation.md#when-to-choose-rust-over-c)
- [Language Philosophy Comparison / 语言设计理念对比](ch01-introduction-and-motivation.md#language-philosophy-comparison)
- [Quick Reference: Rust vs C# / 速查：Rust 与 C# 对比](ch01-introduction-and-motivation.md#quick-reference-rust-vs-c)

### 2. Getting Started / 2. 快速开始 🟢
- [Installation and Setup / 安装与环境配置](ch02-getting-started.md#installation-and-setup)
- [Your First Rust Program / 你的第一个 Rust 程序](ch02-getting-started.md#your-first-rust-program)
- [Cargo vs NuGet/MSBuild / Cargo 与 NuGet/MSBuild 对比](ch02-getting-started.md#cargo-vs-nugetmsbuild)
- [Reading Input and CLI Arguments / 读取输入与命令行参数](ch02-getting-started.md#reading-input-and-cli-arguments)
- [Essential Rust Keywords *(optional reference - consult as needed)* / Rust 核心关键字（可选参考，按需查阅）](ch02-1-essential-keywords-reference.md#essential-rust-keywords-for-c-developers)

### 3. Built-in Types and Variables / 3. 内建类型与变量 🟢
- [Variables and Mutability / 变量与可变性](ch03-built-in-types-and-variables.md#variables-and-mutability)
- [Primitive Types Comparison / 基本类型对比](ch03-built-in-types-and-variables.md#primitive-types)
- [String Types: String vs &str / 字符串类型：String 与 &str](ch03-built-in-types-and-variables.md#string-types-string-vs-str)
- [Printing and String Formatting / 输出与字符串格式化](ch03-built-in-types-and-variables.md#printing-and-string-formatting)
- [Type Casting and Conversions / 类型转换](ch03-built-in-types-and-variables.md#type-casting-and-conversions)
- [True Immutability vs Record Illusions / 真正的不可变性与 Record 的“不可变幻觉”](ch03-1-true-immutability-vs-record-illusions.md#true-immutability-vs-record-illusions)

### 4. Control Flow / 4. 控制流 🟢
- [Functions vs Methods / 函数与方法](ch04-control-flow.md#functions-vs-methods)
- [Expression vs Statement (Important!) / 表达式与语句（非常重要）](ch04-control-flow.md#expression-vs-statement-important)
- [Conditional Statements / 条件语句](ch04-control-flow.md#conditional-statements)
- [Loops and Iteration / 循环与迭代](ch04-control-flow.md#loops)

### 5. Data Structures and Collections / 5. 数据结构与集合 🟢
- [Tuples and Destructuring / 元组与解构](ch05-data-structures-and-collections.md#tuples-and-destructuring)
- [Arrays and Slices / 数组与切片](ch05-data-structures-and-collections.md#arrays-and-slices)
- [Structs vs Classes / 结构体与类](ch05-data-structures-and-collections.md#structs-vs-classes)
- [Constructor Patterns / 构造器模式](ch05-1-constructor-patterns.md#constructor-patterns)
- [`Vec<T>` vs `List<T>` / `Vec<T>` 与 `List<T>`](ch05-2-collections-vec-hashmap-and-iterators.md#vect-vs-listt)
- [HashMap vs Dictionary / HashMap 与 Dictionary](ch05-2-collections-vec-hashmap-and-iterators.md#hashmap-vs-dictionary)

### 6. Enums and Pattern Matching / 6. 枚举与模式匹配 🟡
- [Algebraic Data Types vs C# Unions / 代数数据类型与 C# Union 对比](ch06-enums-and-pattern-matching.md#algebraic-data-types-vs-c-unions)
- [Exhaustive Pattern Matching / 穷尽模式匹配](ch06-1-exhaustive-matching-and-null-safety.md#exhaustive-pattern-matching-compiler-guarantees-vs-runtime-errors)
- [`Option<T>` for Null Safety / 使用 `Option<T>` 实现空安全](ch06-1-exhaustive-matching-and-null-safety.md#null-safety-nullablet-vs-optiont)
- [Guards and Advanced Patterns / 守卫与高级模式](ch06-enums-and-pattern-matching.md#guards-and-advanced-patterns)

### 7. Ownership and Borrowing / 7. 所有权与借用 🟡
- [Understanding Ownership / 理解所有权](ch07-ownership-and-borrowing.md#understanding-ownership)
- [Move Semantics vs Reference Semantics / 移动语义与引用语义](ch07-ownership-and-borrowing.md#move-semantics)
- [Borrowing and References / 借用与引用](ch07-ownership-and-borrowing.md#borrowing-basics)
- [Memory Safety Deep Dive / 内存安全深入解析](ch07-1-memory-safety-deep-dive.md#references-vs-pointers)
- [Lifetimes Deep Dive / 生命周期深入解析](ch07-2-lifetimes-deep-dive.md#lifetimes-telling-the-compiler-how-long-references-live) 🔶
- [Smart Pointers, Drop, and Deref / 智能指针、Drop 与 Deref](ch07-3-smart-pointers-beyond-single-ownership.md#smart-pointers-when-single-ownership-isnt-enough) 🔶

### 8. Crates and Modules / 8. Crate 与模块 🟢
- [Rust Modules vs C# Namespaces / Rust 模块与 C# 命名空间](ch08-crates-and-modules.md#rust-modules-vs-c-namespaces)
- [Crates vs .NET Assemblies / Crate 与 .NET 程序集](ch08-crates-and-modules.md#crates-vs-net-assemblies)
- [Package Management: Cargo vs NuGet / 包管理：Cargo 与 NuGet](ch08-1-package-management-cargo-vs-nuget.md#package-management-cargo-vs-nuget)

### 9. Error Handling / 9. 错误处理 🟡
- [Exceptions vs `Result<T, E>` / 异常与 `Result<T, E>`](ch09-error-handling.md#exceptions-vs-resultt-e)
- [The ? Operator / `?` 操作符](ch09-error-handling.md#the--operator-propagating-errors-concisely)
- [Custom Error Types / 自定义错误类型](ch06-1-exhaustive-matching-and-null-safety.md#custom-error-types)
- [Crate-Level Error Types and Result Aliases / Crate 级错误类型与 `Result` 别名](ch09-1-crate-level-error-types-and-result-alias.md#crate-level-error-types-and-result-aliases)
- [Error Recovery Patterns / 错误恢复模式](ch09-1-crate-level-error-types-and-result-alias.md#error-recovery-patterns)

### 10. Traits and Generics / 10. Trait 与泛型 🟡
- [Traits vs Interfaces / Trait 与接口](ch10-traits-and-generics.md#traits---rusts-interfaces)
- [Inheritance vs Composition / 继承与组合](ch10-2-inheritance-vs-composition.md#inheritance-vs-composition)
- [Generic Constraints: where vs trait bounds / 泛型约束：where 与 trait bound](ch10-1-generic-constraints.md#generic-constraints-where-vs-trait-bounds)
- [Common Standard Library Traits / 常见标准库 Trait](ch10-traits-and-generics.md#common-standard-library-traits)

### 11. From and Into Traits / 11. `From` 与 `Into` Trait 🟡
- [Type Conversions in Rust / Rust 中的类型转换](ch11-from-and-into-traits.md#type-conversions-in-rust)
- [Implementing From for Custom Types / 为自定义类型实现 From](ch11-from-and-into-traits.md#rust-from-and-into)

### 12. Closures and Iterators / 12. 闭包与迭代器 🟡
- [Rust Closures / Rust 闭包](ch12-closures-and-iterators.md#rust-closures)
- [LINQ vs Rust Iterators / LINQ 与 Rust 迭代器](ch12-closures-and-iterators.md#linq-vs-rust-iterators)
- [Macros Primer / 宏入门](ch12-1-macros-primer.md#macros-code-that-writes-code)

---

## Part II - Concurrency & Systems / 第二部分：并发与系统

### 13. Concurrency / 13. 并发 🔶
- [Thread Safety: Convention vs Type System Guarantees / 线程安全：约定式保障与类型系统保障](ch13-concurrency.md#thread-safety-convention-vs-type-system-guarantees)
- [async/await: C# Task vs Rust Future / async/await：C# Task 与 Rust Future](ch13-1-asyncawait-deep-dive.md#async-programming-c-task-vs-rust-future)
- [Cancellation Patterns / 取消模式](ch13-1-asyncawait-deep-dive.md#cancellation-cancellationtoken-vs-drop--select)
- [Pin and tokio::spawn / Pin 与 `tokio::spawn`](ch13-1-asyncawait-deep-dive.md#pin-why-rust-async-has-a-concept-c-doesnt)

### 14. Unsafe Rust, FFI, and Testing / 14. Unsafe Rust、FFI 与测试 🟡
- [When and Why to Use Unsafe / 何时以及为何使用 Unsafe](ch14-unsafe-rust-and-ffi.md#when-you-need-unsafe)
- [Interop with C# via FFI / 通过 FFI 与 C# 互操作](ch14-unsafe-rust-and-ffi.md#interop-with-c-via-ffi)
- [Testing in Rust vs C# / Rust 测试与 C# 测试](ch14-1-testing.md#testing-in-rust-vs-c)
- [Property Testing and Mocking / 属性测试与 Mock](ch14-1-testing.md#property-testing-proving-correctness-at-scale)

---

## Part III - Migration & Best Practices / 第三部分：迁移与最佳实践

### 15. Migration Patterns and Case Studies / 15. 迁移模式与案例研究 🟡
- [Common C# Patterns in Rust / Rust 中的常见 C# 模式](ch15-migration-patterns-and-case-studies.md#common-c-patterns-in-rust)
- [Essential Crates for C# Developers / C# 开发者必备 Crate](ch15-1-essential-crates-for-c-developers.md#essential-crates-for-c-developers)
- [Incremental Adoption Strategy / 渐进式采用策略](ch15-2-incremental-adoption-strategy.md#incremental-adoption-strategy)

### 16. Best Practices and Reference / 16. 最佳实践与参考 🟡
- [Idiomatic Rust for C# Developers / 面向 C# 开发者的 Rust 惯用法](ch16-best-practices.md#best-practices-for-c-developers)
- [Performance Comparison: Managed vs Native / 性能对比：托管与原生](ch16-1-performance-comparison-and-migration.md#performance-comparison-managed-vs-native)
- [Common Pitfalls and Solutions / 常见陷阱与解决方案](ch16-2-learning-path-and-resources.md#common-pitfalls-for-c-developers)
- [Learning Path and Resources / 学习路径与资源](ch16-2-learning-path-and-resources.md#learning-path-and-next-steps)
- [Rust Tooling Ecosystem / Rust 工具体系生态](ch16-3-rust-tooling-ecosystem.md#essential-rust-tooling-for-c-developers)

---

## Capstone / 综合项目

### 17. Capstone Project / 17. 综合项目 🟡
- [Build a CLI Weather Tool / 构建命令行天气工具](ch17-capstone-project.md#capstone-project-build-a-cli-weather-tool) - combines structs, traits, error handling, async, modules, serde, and testing into a working application / 将结构体、trait、错误处理、async、模块、serde 和测试整合为一个可运行应用

***
