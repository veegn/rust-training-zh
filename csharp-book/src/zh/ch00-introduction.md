# 面向 C# 程序员的 Rust：完整训练指南

这是一份为具有 C# 开发经验的程序员量身定制的 Rust 学习指南。本书涵盖了从基础语法到高级模式的所有内容，重点关注两种语言之间的概念转变以及实践差异。

## 课程概览
- **为什么要学习 Rust** —— Rust 对 C# 开发者的意义：性能、安全性和正确性。
- **快速上手** —— 安装、工具链以及你的第一个 Rust 程序。
- **基础构建模块** —— 类型、变量、控制流。
- **数据结构** —— 数组、元组、结构体、集合。
- **模式匹配与枚举** —— 代数数据类型与穷尽式匹配。
- **所有权与借用** —— Rust 的内存管理模型。
- **模块与 Crates** —— 代码组织与依赖管理。
- **错误处理** —— 基于 Result 的错误传播。
- **特性 (Traits) 与泛型** —— Rust 的类型系统。
- **闭包与迭代器** —— 函数式编程模式。
- **并发编程** —— 拥有类型系统保证的“无畏并发”，以及异步/等待 (Async/Await) 深度解析。
- **Unsafe Rust 与 FFI** —— 何时以及如何超越“安全 Rust”的边界。
- **迁移模式** —— 现实世界中 C# 到 Rust 的模式转换与渐进式引入。
- **最佳实践** —— 专为 C# 开发者准备的惯用 Rust 写法。

---

# 自学指南

本教材既可以作为讲师引导的课程，也适用于自学。如果你是独自学习，以下是如何获得最佳学习效果的建议。

**进度建议：**

| 章节 | 主题 | 建议时间 | 阶段目标 (Checkpoint) |
|----------|-------|---------------|------------|
| 1–4 | 环境搭建、类型、控制流 | 1 天 | 你能用 Rust 编写一个命令行温度转换器 |
| 5–6 | 数据结构、枚举、模式匹配 | 1–2 天 | 你能定义带数据的枚举并对其进行穷尽式 `match` |
| 7 | 所有权与借用 | 1–2 天 | 你能解释*为什么* `let s2 = s1` 会导致 `s1` 失效 |
| 8–9 | 模块、错误处理 | 1 天 | 你能创建一个多文件项目并使用 `?` 传播错误 |
| 10–12 | 特性、泛型、闭包、迭代器 | 1–2 天 | 你能将 LINQ 链翻译为 Rust 迭代器 |
| 13 | 并发与异步 | 1 天 | 你能使用 `Arc<Mutex<T>>` 编写线程安全的计数器 |
| 14 | Unsafe Rust, FFI, 测试 | 1 天 | 你能通过 P/Invoke 从 C# 调用 Rust 函数 |
| 15–16 | 迁移、最佳实践、工具链 | 自主节奏 | 参考材料 —— 在编写实际代码时查阅 |
| 17 | 终极项目实战 | 1–2 天 | 你完成了一个能获取天气数据的命令行工具 |

**如何使用练习：**
- 各个章节在可折叠的 `<details>` 块中包含了动手练习及其参考答案。
- **在展开答案之前，务必先尝试自己动手完成练习。** 与借用检查器“搏斗”是学习过程的一部分 —— 编译器的错误提示就是你最好的老师。
- 如果你卡住超过 15 分钟，可以展开答案进行研究，然后关闭它，尝试再次从零开始编写。
- [Rust Playground](https://play.rust-lang.org/) 让你无需本地安装即可运行代码。

**难度分级：**
- 🟢 **入门** —— 直接从 C# 概念对应转化。
- 🟡 **中级** —— 需要理解所有权或特性 (Traits)。
- 🔴 **进阶** —— 涉及生命周期、异步原理或不安全代码。

**当你遇到困难时：**
- 仔细阅读编译器的错误信息 —— Rust 的报错信息具有极高的指导意义。
- 重新阅读相关章节；像所有权（第 7 章）这样的概念通常在第二次阅读时才会豁然开朗。
- [Rust 标准库文档](https://doc.rust-lang.org/std/) 非常出色 —— 可以搜索任何类型或方法。
- 若需深入学习异步模式，请参阅配套的 [异步 Rust 训练](../async-book/) 指南。

---

# 目录

## 第一部分 — 基础篇

### 1. 简介与动机 🟢
- [C# 开发者为什么要学习 Rust](ch01-introduction-and-motivation.md#c-开发者为什么要学习-rust)
- [Rust 解决的常见 C# 痛点](ch01-introduction-and-motivation.md#rust-解决的常见-c-痛点)
- [何时选择 Rust 而非 C#](ch01-introduction-and-motivation.md#何时选择-rust-而非-c)
- [语言哲学对比](ch01-introduction-and-motivation.md#语言哲学对比)
- [快速参考：Rust vs C#](ch01-introduction-and-motivation.md#快速参考rust-vs-c)

### 2. 快速上手 🟢
- [安装与环境搭建](ch02-getting-started.md#安装与环境搭建)
- [你的第一个 Rust 程序](ch02-getting-started.md#你的第一个-rust-程序)
- [Cargo vs NuGet/MSBuild](ch02-getting-started.md#cargo-vs-nugetmsbuild)
- [读取输入与命令行参数](ch02-getting-started.md#读取输入与命令行参数)
- [核心 Rust 关键字 *(可选参考 —— 视需要查阅)*](ch02-1-essential-keywords-reference.md#c-开发者核心-rust-关键字)

### 3. 内置类型与变量 🟢
- [变量与可变性](ch03-built-in-types-and-variables.md#变量与可变性)
- [原生类型对比](ch03-built-in-types-and-variables.md#原生类型对比)
- [字符串类型：String vs &str](ch03-built-in-types-and-variables.md#字符串类型string-vs-str)
- [打印与字符串格式化](ch03-built-in-types-and-variables.md#打印与字符串格式化)
- [类型转换与强制转换](ch03-built-in-types-and-variables.md#类型转换与强制转换)
- [真正的不可变性 vs Records 幻想](ch03-1-true-immutability-vs-record-illusions.md#真正的不可变性-vs-records-幻想)

### 4. 控制流 🟢
- [函数 vs 方法](ch04-control-flow.md#函数-vs-方法)
- [表达式 vs 语句 (重点！)](ch04-control-flow.md#表达式-vs-语句-重点)
- [条件语句](ch04-control-flow.md#条件语句)
- [循环与迭代](ch04-control-flow.md#循环)

### 5. 数据结构与集合 🟢
- [元组与解构](ch05-data-structures-and-collections.md#元组与解构)
- [数组与切片](ch05-data-structures-and-collections.md#数组与切片)
- [结构体 vs 类](ch05-data-structures-and-collections.md#结构体-vs-类)
- [构造函数模式](ch05-1-constructor-patterns.md#构造函数模式)
- [`Vec<T>` vs `List<T>`](ch05-2-collections-vec-hashmap-and-iterators.md#vect-vs-listt)
- [HashMap vs Dictionary](ch05-2-collections-vec-hashmap-and-iterators.md#hashmap-vs-dictionary)

### 6. 枚举与模式匹配 🟡
- [代数数据类型 vs C# 联合体](ch06-enums-and-pattern-matching.md#代数数据类型-vs-c-联合体)
- [穷尽式模式匹配](ch06-1-exhaustive-matching-and-null-safety.md#穷尽式模式匹配编译器保证-vs-运行时错误)
- [用于空安全的 `Option<T>`](ch06-1-exhaustive-matching-and-null-safety.md#空安全nullablet-vs-optiont)
- [守卫 (Guards) 与高级模式](ch06-enums-and-pattern-matching.md#守卫与高级模式)

### 7. 所有权与借用 🟡
- [理解所有权](ch07-ownership-and-borrowing.md#理解所有权)
- [移动语义 vs 引用语义](ch07-ownership-and-borrowing.md#移动语义)
- [借用与引用](ch07-ownership-and-borrowing.md#借用基础)
- [内存安全深度解析](ch07-1-memory-safety_deep-dive.md#引用-vs-指针)
- [生命周期深度解析](ch07-2-lifetimes-deep-dive.md#生命周期指示编译器引用的存活时间) 🔴
- [智能指针, Drop 和 Deref](ch07-3-smart-pointers-beyond-single-ownership.md#智能指针当单一所有权力有不逮时) 🔴

### 8. Crates 与 模块 🟢
- [Rust 模块 vs C# 命名空间](ch08-crates-and-modules.md#rust-模块-vs-c-命名空间)
- [Crates vs .NET 程序集](ch08-crates-and-modules.md#crates-vs-net-程序集)
- [包管理：Cargo vs NuGet](ch08-1-package-management-cargo-vs-nuget.md#包管理cargo-vs-nuget)

### 9. 错误处理 🟡
- [异常 (Exceptions) vs `Result<T, E>`](ch09-error-handling.md#异常-vs-resultt-e)
- [? 运算符](ch09-error-handling.md#--运算符简洁地传播错误)
- [自定义错误类型](ch06-1-exhaustive-matching-and-null-safety.md#自定义错误类型)
- [Crate 级错误类型与 Result 别名](ch09-1-crate-level-error-types-and-result-alias.md#crate-级错误类型与-result-别名)
- [错误恢复模式](ch09-1-crate-level-error-types-and-result-alias.md#错误恢复模式)

### 10. 特性 (Traits) 与 泛型 🟡
- [特性 vs 接口](ch10-traits-and-generics.md#特性---rust-的接口)
- [继承 vs 组合](ch10-2-inheritance-vs-composition.md#继承-vs-组合)
- [泛型约束：where vs 特性限界](ch10-1-generic-constraints.md#泛型约束where-vs-特性限界-trait-bounds)
- [常用标准库特性](ch10-traits-and-generics.md#常见标准库特性)

### 11. From 与 Into 特性 🟡
- [Rust 中的类型转换](ch11-from-and-into-traits.md#rust-中的类型转换)
- [为自定义类型实现 From](ch11-from-and-into-traits.md#rust-中的-from-与-into)

### 12. 闭包与迭代器 🟡
- [Rust 闭包](ch12-closures-and-iterators.md#rust-闭包)
- [LINQ vs Rust 迭代器](ch12-closures-and-iterators.md#linq-vs-rust-迭代器)
- [宏入门](ch12-1-macros-primer.md#宏编写代码的代码)

---

## 第二部分 — 并发与系统编程

### 13. 并发编程 🔴
- [线程安全：约定原则 vs 类型系统保证](ch13-concurrency.md#线程安全约定原则-vs-类型系统保证)
- [async/await: C# Task vs Rust Future](ch13-1-asyncawait-deep-dive.md#异步编程c-task-vs-rust-future)
- [取消模式](ch13-1-asyncawait-deep-dive.md#取消机制cancellationtoken-vs-drop--select)
- [Pin 与 tokio::spawn](ch13-1-asyncawait-deep-dive.md#pin为什么-rust-异步拥有一个-c-所没有的概念)

### 14. Unsafe Rust, FFI 与 测试 🟡
- [为何以及何时需要使用 Unsafe](ch14-unsafe-rust-and-ffi.md#为何需要内核-unsafe)
- [通过 FFI 与 C# 进行互操作](ch14-unsafe-rust-and-ffi.md#通过-ffi-与-c-进行互操作)
- [Rust 与 C# 的测试对比](ch14-1-testing.md#rust-与-c-中的测试)
- [属性测试与 Mocking](ch14-1-testing.md#属性测试在大规模下证明正确性)

---

## 第三部分 — 迁移与最佳实践

### 15. 迁移模式与案例研究 🟡
- [Rust 中的常用 C# 设计模式对照](ch15-migration-patterns-and-case-studies.md#rust-中的常用-c-设计模式对照)
- [C# 开发者核心 Crate 指南](ch15-1-essential-crates-for-c-developers.md#c-开发者核心-crate-指南)
- [渐进式引入策略](ch15-2-incremental-adoption-strategy.md#渐进式引入策略)

### 16. 最佳实践与参考 🟡
- [C# 开发者的最佳实践指南](ch16-best-practices.md#c-开发者的最佳实践指南)
- [性能对比：托管代码 vs 原生代码](ch16-1-performance-comparison-and-migration.md#性能对比托管代码-vs-原生代码)
- [C# 开发者常见的坑](ch16-2-learning-path-and-resources.md#c-开发者常见的坑)
- [学习路线图与后续步骤](ch16-2-learning-path-and-resources.md#学习路线图与后续步骤)
- [C# 开发者的核心 Rust 工具链指南](ch16-3-rust-tooling-ecosystem.md#c-开发者的核心-rust-工具链指南)

---

## 终极项目实战

### 17. 终极项目 🟡
- [构建命令行天气工具](ch17-capstone-project.md#项目实战构建命令行天气工具) —— 综合运用结构体、特性、错误处理、异步、模块、serde 以及测试，构建一个真实可用的应用程序。
