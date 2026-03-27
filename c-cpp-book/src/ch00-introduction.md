# Rust Bootstrap Course for C/C++ Programmers / 面向 C/C++ 程序员的 Rust 入门强化课程

## Course Overview / 课程概览
- Course overview / 课程包含以下内容
    - The case for Rust (from both C and C++ perspectives) / 为什么选择 Rust（从 C 和 C++ 两个角度出发）
    - Local installation / 本地安装
    - Types, functions, control flow, pattern matching / 类型、函数、控制流、模式匹配
    - Modules, cargo / 模块与 Cargo
    - Traits, generics / Trait 与泛型
    - Collections, error handling / 集合与错误处理
    - Closures, memory management, lifetimes, smart pointers / 闭包、内存管理、生命周期、智能指针
    - Concurrency / 并发
    - Unsafe Rust, including Foreign Function Interface (FFI) / Unsafe Rust，包括外部函数接口（FFI）
    - `no_std` and embedded Rust essentials for firmware teams / 面向固件团队的 `no_std` 与嵌入式 Rust 核心内容
    - Case studies: real-world C++ to Rust translation patterns / 案例研究：真实世界中的 C++ 到 Rust 迁移模式
- We'll not cover `async` Rust in this course - see the companion [Async Rust Training](../async-book/) for a full treatment of futures, executors, `Pin`, tokio, and production async patterns
- 本课程**不**讲解 `async` Rust；如需系统学习 futures、执行器、`Pin`、tokio 与生产级异步模式，请参考配套的 [Async Rust Training](../async-book/)

---

# Self-Study Guide / 自学指南

This material works both as an instructor-led course and for self-study. If you're working through it on your own, here's how to get the most out of it:

本材料既适合讲师授课，也适合自学。如果你打算独立学习，下面的建议可以帮助你获得更好的学习效果：

**Pacing recommendations / 学习节奏建议：**

| Chapters / 章节 | Topic / 主题 | Suggested Time / 建议时间 | Checkpoint / 检查点 |
|----------|-------|---------------|------------|
| 1-4 | Setup, types, control flow / 环境、类型、控制流 | 1 day / 1 天 | You can write a CLI temperature converter / 你可以写出一个命令行温度转换器 |
| 5-7 | Data structures, ownership / 数据结构、所有权 | 1-2 days / 1-2 天 | You can explain *why* `let s2 = s1` invalidates `s1` / 你可以解释为什么 `let s2 = s1` 会使 `s1` 失效 |
| 8-9 | Modules, error handling / 模块、错误处理 | 1 day / 1 天 | You can create a multi-file project that propagates errors with `?` / 你可以创建一个多文件项目并用 `?` 传播错误 |
| 10-12 | Traits, generics, closures / Trait、泛型、闭包 | 1-2 days / 1-2 天 | You can write a generic function with trait bounds / 你可以写出带 trait 约束的泛型函数 |
| 13-14 | Concurrency, unsafe/FFI / 并发、unsafe/FFI | 1 day / 1 天 | You can write a thread-safe counter with `Arc<Mutex<T>>` / 你可以用 `Arc<Mutex<T>>` 写出线程安全计数器 |
| 15-16 | Deep dives / 深入专题 | At your own pace / 自定节奏 | Reference material - read when relevant / 作为参考材料，在需要时查阅 |
| 17-19 | Best practices & reference / 最佳实践与参考 | At your own pace / 自定节奏 | Consult as you write real code / 在实际写代码时按需查阅 |

**How to use the exercises / 如何使用练习：**
- Every chapter has hands-on exercises marked with difficulty: 🟢 Starter, 🟡 Intermediate, 🔶 Challenge / 每章都包含带难度标记的动手练习：🟢 入门、🟡 中级、🔶 挑战
- **Always try the exercise before expanding the solution.** Struggling with the borrow checker is part of learning - the compiler's error messages are your teacher / **总是先做题，再看答案。** 与借用检查器“拉扯”是学习的一部分，编译器的报错就是你的老师
- If you're stuck for more than 15 minutes, expand the solution, study it, then close it and try again from scratch / 如果卡住超过 15 分钟，就先看答案、理解思路，再关闭答案重新独立完成
- The [Rust Playground](https://play.rust-lang.org/) lets you run code without a local install / [Rust Playground](https://play.rust-lang.org/) 允许你无需本地安装就运行代码

**When you hit a wall / 遇到难点时：**
- Read the compiler error message carefully - Rust's errors are exceptionally helpful / 仔细阅读编译器错误信息，Rust 的错误提示通常非常有帮助
- Re-read the relevant section; concepts like ownership (ch7) often click on the second pass / 重读相关章节，像所有权这类概念常常在第二遍时真正理解
- The [Rust standard library docs](https://doc.rust-lang.org/std/) are excellent - search for any type or method / [Rust 标准库文档](https://doc.rust-lang.org/std/) 非常优秀，任何类型或方法都值得查阅
- For async patterns, see the companion [Async Rust Training](../async-book/) / 如需学习异步模式，请参考配套的 [Async Rust Training](../async-book/)

---

# Table of Contents / 目录

## Part I - Foundations / 第一部分：基础

### 1. Introduction and Motivation / 1. 引言与动机
- [Speaker intro and general approach / 讲师介绍与整体方法](ch01-introduction-and-motivation.md#speaker-intro-and-general-approach)
- [The case for Rust / 为什么选择 Rust](ch01-introduction-and-motivation.md#the-case-for-rust)
- [How does Rust address these issues? / Rust 如何解决这些问题？](ch01-introduction-and-motivation.md#how-does-rust-address-these-issues)
- [Other Rust USPs and features / Rust 的其他独特优势与特性](ch01-introduction-and-motivation.md#other-rust-usps-and-features)
- [Quick Reference: Rust vs C/C++ / 速查：Rust 与 C/C++ 对比](ch01-introduction-and-motivation.md#quick-reference-rust-vs-cc)
- [Why C/C++ Developers Need Rust / 为什么 C/C++ 开发者需要 Rust](ch01-1-why-c-cpp-developers-need-rust.md)
  - [What Rust Eliminates - The Complete List / Rust 消除了什么：完整清单](ch01-1-why-c-cpp-developers-need-rust.md#what-rust-eliminates--the-complete-list)
  - [The Problems Shared by C and C++ / C 与 C++ 共同存在的问题](ch01-1-why-c-cpp-developers-need-rust.md#the-problems-shared-by-c-and-c)
  - [C++ Adds More Problems on Top / C++ 额外引入的问题](ch01-1-why-c-cpp-developers-need-rust.md#c-adds-more-problems-on-top)
  - [How Rust Addresses All of This / Rust 如何解决这一切](ch01-1-why-c-cpp-developers-need-rust.md#how-rust-addresses-all-of-this)

### 2. Getting Started / 2. 快速开始
- [Enough talk already: Show me some code / 少说多练：先看代码](ch02-getting-started.md#enough-talk-already-show-me-some-code)
- [Rust Local installation / Rust 本地安装](ch02-getting-started.md#rust-local-installation)
- [Rust packages (crates) / Rust 包（crate）](ch02-getting-started.md#rust-packages-crates)
- [Example: cargo and crates / 示例：cargo 与 crate](ch02-getting-started.md#example-cargo-and-crates)

### 3. Basic Types and Variables / 3. 基础类型与变量
- [Built-in Rust types / Rust 内建类型](ch03-built-in-types.md#built-in-rust-types)
- [Rust type specification and assignment / Rust 类型标注与赋值](ch03-built-in-types.md#rust-type-specification-and-assignment)
- [Rust type specification and inference / Rust 类型标注与推断](ch03-built-in-types.md#rust-type-specification-and-inference)
- [Rust variables and mutability / Rust 变量与可变性](ch03-built-in-types.md#rust-variables-and-mutability)

### 4. Control Flow / 4. 控制流
- [Rust if keyword / Rust `if` 关键字](ch04-control-flow.md#rust-if-keyword)
- [Rust loops using while and for / 使用 while 与 for 的 Rust 循环](ch04-control-flow.md#rust-loops-using-while-and-for)
- [Rust loops using loop / 使用 loop 的 Rust 循环](ch04-control-flow.md#rust-loops-using-loop)
- [Rust expression blocks / Rust 表达式块](ch04-control-flow.md#rust-expression-blocks)

### 5. Data Structures and Collections / 5. 数据结构与集合
- [Rust array type / Rust 数组类型](ch05-data-structures.md#rust-array-type)
- [Rust tuples / Rust 元组](ch05-data-structures.md#rust-tuples)
- [Rust references / Rust 引用](ch05-data-structures.md#rust-references)
- [C++ References vs Rust References - Key Differences / C++ 引用与 Rust 引用的关键区别](ch05-data-structures.md#c-references-vs-rust-references--key-differences)
- [Rust slices / Rust 切片](ch05-data-structures.md#rust-slices)
- [Rust constants and statics / Rust 常量与静态项](ch05-data-structures.md#rust-constants-and-statics)
- [Rust strings: String vs &str / Rust 字符串：String 与 &str](ch05-data-structures.md#rust-strings-string-vs-str)
- [Rust structs / Rust 结构体](ch05-data-structures.md#rust-structs)
- [Rust Vec<T> / Rust Vec<T>](ch05-data-structures.md#rust-vec-type)
- [Rust HashMap / Rust HashMap](ch05-data-structures.md#rust-hashmap-type)
- [Exercise: Vec and HashMap / 练习：Vec 与 HashMap](ch05-data-structures.md#exercise-vec-and-hashmap)

### 6. Pattern Matching and Enums / 6. 模式匹配与枚举
- [Rust enum types / Rust 枚举类型](ch06-enums-and-pattern-matching.md#rust-enum-types)
- [Rust match statement / Rust `match` 语句](ch06-enums-and-pattern-matching.md#rust-match-statement)
- [Exercise: Implement add and subtract using match and enum / 练习：使用 match 和 enum 实现加减法](ch06-enums-and-pattern-matching.md#exercise-implement-add-and-subtract-using-match-and-enum)

### 7. Ownership and Memory Management / 7. 所有权与内存管理
- [Rust memory management / Rust 内存管理](ch07-ownership-and-borrowing.md#rust-memory-management)
- [Rust ownership, borrowing and lifetimes / Rust 所有权、借用与生命周期](ch07-ownership-and-borrowing.md#rust-ownership-borrowing-and-lifetimes)
- [Rust move semantics / Rust 移动语义](ch07-ownership-and-borrowing.md#rust-move-semantics)
- [Rust Clone / Rust Clone](ch07-ownership-and-borrowing.md#rust-clone)
- [Rust Copy trait / Rust Copy trait](ch07-ownership-and-borrowing.md#rust-copy-trait)
- [Rust Drop trait / Rust Drop trait](ch07-ownership-and-borrowing.md#rust-drop-trait)
- [Exercise: Move, Copy and Drop / 练习：Move、Copy 与 Drop](ch07-ownership-and-borrowing.md#exercise-move-copy-and-drop)
- [Rust lifetime and borrowing / Rust 生命周期与借用](ch07-1-lifetimes-and-borrowing-deep-dive.md#rust-lifetime-and-borrowing)
- [Rust lifetime annotations / Rust 生命周期标注](ch07-1-lifetimes-and-borrowing-deep-dive.md#rust-lifetime-annotations)
- [Exercise: Slice storage with lifetimes / 练习：带生命周期的切片存储](ch07-1-lifetimes-and-borrowing-deep-dive.md#exercise-slice-storage-with-lifetimes)
- [Lifetime Elision Rules Deep Dive / 生命周期省略规则深入解析](ch07-1-lifetimes-and-borrowing-deep-dive.md#lifetime-elision-rules-deep-dive)
- [Rust Box<T> / Rust Box<T>](ch07-2-smart-pointers-and-interior-mutability.md#rust-boxt)
- [Interior Mutability: Cell<T> and RefCell<T> / 内部可变性：Cell<T> 与 RefCell<T>](ch07-2-smart-pointers-and-interior-mutability.md#interior-mutability-cellt-and-refcellt)
- [Shared Ownership: Rc<T> / 共享所有权：Rc<T>](ch07-2-smart-pointers-and-interior-mutability.md#shared-ownership-rct)
- [Exercise: Shared ownership and interior mutability / 练习：共享所有权与内部可变性](ch07-2-smart-pointers-and-interior-mutability.md#exercise-shared-ownership-and-interior-mutability)

### 8. Modules and Crates / 8. 模块与 Crate
- [Rust crates and modules / Rust crate 与模块](ch08-crates-and-modules.md#rust-crates-and-modules)
- [Exercise: Modules and functions / 练习：模块与函数](ch08-crates-and-modules.md#exercise-modules-and-functions)
- [Workspaces and crates (packages) / 工作区与 crate（包）](ch08-crates-and-modules.md#workspaces-and-crates-packages)
- [Exercise: Using workspaces and package dependencies / 练习：使用工作区与包依赖](ch08-crates-and-modules.md#exercise-using-workspaces-and-package-dependencies)
- [Using community crates from crates.io / 使用 crates.io 上的社区 crate](ch08-crates-and-modules.md#using-community-crates-from-cratesio)
- [Crates dependencies and SemVer / Crate 依赖与 SemVer](ch08-crates-and-modules.md#crates-dependencies-and-semver)
- [Exercise: Using the rand crate / 练习：使用 rand crate](ch08-crates-and-modules.md#exercise-using-the-rand-crate)
- [Cargo.toml and Cargo.lock / Cargo.toml 与 Cargo.lock](ch08-crates-and-modules.md#cargotoml-and-cargolock)
- [Cargo test feature / Cargo test 功能](ch08-crates-and-modules.md#cargo-test-feature)
- [Other Cargo features / 其他 Cargo 功能](ch08-crates-and-modules.md#other-cargo-features)
- [Testing Patterns / 测试模式](ch08-1-testing-patterns.md)

### 9. Error Handling / 9. 错误处理
- [Connecting enums to Option and Result / 将枚举与 Option、Result 关联起来](ch09-error-handling.md#connecting-enums-to-option-and-result)
- [Rust Option type / Rust Option 类型](ch09-error-handling.md#rust-option-type)
- [Rust Result type / Rust Result 类型](ch09-error-handling.md#rust-result-type)
- [Exercise: log() function implementation with Option / 练习：使用 Option 实现 `log()` 函数](ch09-error-handling.md#exercise-log-function-implementation-with-option)
- [Rust error handling / Rust 错误处理](ch09-error-handling.md#rust-error-handling)
- [Exercise: error handling / 练习：错误处理](ch09-error-handling.md#exercise-error-handling)
- [Error Handling Best Practices / 错误处理最佳实践](ch09-1-error-handling-best-practices.md)

### 10. Traits and Generics / 10. Trait 与泛型
- [Rust traits / Rust trait](ch10-traits.md#rust-traits)
- [C++ Operator Overloading to Rust std::ops Traits / C++ 运算符重载到 Rust `std::ops` trait](ch10-traits.md#c-operator-overloading--rust-stdops-traits)
- [Exercise: Logger trait implementation / 练习：实现 Logger trait](ch10-traits.md#exercise-logger-trait-implementation)
- [When to use enum vs dyn Trait / 何时使用 enum，何时使用 dyn Trait](ch10-traits.md#when-to-use-enum-vs-dyn-trait)
- [Exercise: Think Before You Translate / 练习：翻译代码前先思考](ch10-traits.md#exercise-think-before-you-translate)
- [Rust generics / Rust 泛型](ch10-1-generics.md#rust-generics)
- [Exercise: Generics / 练习：泛型](ch10-1-generics.md#exercise-generics)
- [Combining Rust traits and generics / 结合 Rust trait 与泛型](ch10-1-generics.md#combining-rust-traits-and-generics)
- [Rust traits constraints in data types / 数据类型中的 Rust trait 约束](ch10-1-generics.md#rust-traits-constraints-in-data-types)
- [Exercise: Trait constraints and generics / 练习：trait 约束与泛型](ch10-1-generics.md#exercise-traits-constraints-and-generics)
- [Rust type state pattern and generics / Rust type-state 模式与泛型](ch10-1-generics.md#rust-type-state-pattern-and-generics)
- [Rust builder pattern / Rust builder 模式](ch10-1-generics.md#rust-builder-pattern)

### 11. Type System Advanced Features / 11. 类型系统高级特性
- [Rust From and Into traits / Rust From 与 Into trait](ch11-from-and-into-traits.md#rust-from-and-into-traits)
- [Exercise: From and Into / 练习：From 与 Into](ch11-from-and-into-traits.md#exercise-from-and-into)
- [Rust Default trait / Rust Default trait](ch11-from-and-into-traits.md#rust-default-trait)
- [Other Rust type conversions / 其他 Rust 类型转换](ch11-from-and-into-traits.md#other-rust-type-conversions)

### 12. Functional Programming / 12. 函数式编程
- [Rust closures / Rust 闭包](ch12-closures.md#rust-closures)
- [Exercise: Closures and capturing / 练习：闭包与捕获](ch12-closures.md#exercise-closures-and-capturing)
- [Rust iterators / Rust 迭代器](ch12-closures.md#rust-iterators)
- [Exercise: Rust iterators / 练习：Rust 迭代器](ch12-closures.md#exercise-rust-iterators)
- [Iterator Power Tools Reference / 迭代器进阶工具速查](ch12-1-iterator-power-tools.md#iterator-power-tools-reference)

### 13. Concurrency / 13. 并发
- [Rust concurrency / Rust 并发](ch13-concurrency.md#rust-concurrency)
- [Why Rust prevents data races: Send and Sync / Rust 为什么能防止数据竞争：Send 与 Sync](ch13-concurrency.md#why-rust-prevents-data-races-send-and-sync)
- [Exercise: Multi-threaded word count / 练习：多线程词频统计](ch13-concurrency.md#exercise-multi-threaded-word-count)

### 14. Unsafe Rust and FFI / 14. Unsafe Rust 与 FFI
- [Unsafe Rust / Unsafe Rust](ch14-unsafe-rust-and-ffi.md#unsafe-rust)
- [Simple FFI example / 简单 FFI 示例](ch14-unsafe-rust-and-ffi.md#simple-ffi-example-rust-library-function-consumed-by-c)
- [Complex FFI example / 复杂 FFI 示例](ch14-unsafe-rust-and-ffi.md#complex-ffi-example)
- [Ensuring correctness of unsafe code / 确保 unsafe 代码正确性](ch14-unsafe-rust-and-ffi.md#ensuring-correctness-of-unsafe-code)
- [Exercise: Writing a safe FFI wrapper / 练习：编写安全的 FFI 包装器](ch14-unsafe-rust-and-ffi.md#exercise-writing-a-safe-ffi-wrapper)

## Part II - Deep Dives / 第二部分：深入专题

### 15. no_std - Rust for Bare Metal / 15. no_std：面向裸机的 Rust
- [What is no_std? / 什么是 no_std？](ch15-no_std-rust-without-the-standard-library.md#what-is-no_std)
- [When to use no_std vs std / 何时使用 no_std，何时使用 std](ch15-no_std-rust-without-the-standard-library.md#when-to-use-no_std-vs-std)
- [Exercise: no_std ring buffer / 练习：no_std 环形缓冲区](ch15-no_std-rust-without-the-standard-library.md#exercise-no_std-ring-buffer)
- [Embedded Deep Dive / 嵌入式深入解析](ch15-1-embedded-deep-dive.md)

### 16. Case Studies: Real-World C++ to Rust Translation / 16. 案例研究：真实世界中的 C++ 到 Rust 迁移
- [Case Study 1: Inheritance hierarchy to Enum dispatch / 案例 1：继承层次到枚举分发](ch16-case-studies.md#case-study-1-inheritance-hierarchy--enum-dispatch)
- [Case Study 2: shared_ptr tree to Arena/index pattern / 案例 2：shared_ptr 树到 Arena/索引模式](ch16-case-studies.md#case-study-2-shared_ptr-tree--arenaindex-pattern)
- [Case Study 3: Framework communication to Lifetime borrowing / 案例 3：框架通信到生命周期借用](ch16-1-case-study-lifetime-borrowing.md#case-study-3-framework-communication--lifetime-borrowing)
- [Case Study 4: God object to Composable state / 案例 4：上帝对象到可组合状态](ch16-1-case-study-lifetime-borrowing.md#case-study-4-god-object--composable-state)
- [Case Study 5: Trait objects - when they ARE right / 案例 5：Trait 对象何时才是正确选择](ch16-1-case-study-lifetime-borrowing.md#case-study-5-trait-objects--when-they-are-right)

## Part III - Best Practices & Reference / 第三部分：最佳实践与参考

### 17. Best Practices / 17. 最佳实践
- [Rust Best Practices Summary / Rust 最佳实践总结](ch17-best-practices.md#rust-best-practices-summary)
- [Avoiding excessive clone() / 避免过度使用 clone()](ch17-1-avoiding-excessive-clone.md#avoiding-excessive-clone)
- [Avoiding unchecked indexing / 避免未检查索引](ch17-2-avoiding-unchecked-indexing.md#avoiding-unchecked-indexing)
- [Collapsing assignment pyramids / 精简赋值金字塔](ch17-3-collapsing-assignment-pyramids.md#collapsing-assignment-pyramids)
- [Capstone Exercise: Diagnostic Event Pipeline / 综合练习：诊断事件流水线](ch17-3-collapsing-assignment-pyramids.md#capstone-exercise-diagnostic-event-pipeline)
- [Logging and Tracing Ecosystem / 日志与追踪生态](ch17-4-logging-and-tracing-ecosystem.md#logging-and-tracing-ecosystem)

### 18. C++ to Rust Semantic Deep Dives / 18. C++ 到 Rust 的语义深入对比
- [Casting, Preprocessor, Modules, volatile, static, constexpr, SFINAE, and more / 转型、预处理器、模块、volatile、static、constexpr、SFINAE 等主题](ch18-cpp-rust-semantic-deep-dives.md)

### 19. Rust Macros / 19. Rust 宏
- [Declarative macros (`macro_rules!`) / 声明式宏（`macro_rules!`）](ch19-macros.md#declarative-macros-with-macro_rules)
- [Common standard library macros / 常见标准库宏](ch19-macros.md#common-standard-library-macros)
- [Derive macros / 派生宏](ch19-macros.md#derive-macros)
- [Attribute macros / 属性宏](ch19-macros.md#attribute-macros)
- [Procedural macros / 过程宏](ch19-macros.md#procedural-macros-conceptual-overview)
- [When to use what: macros vs functions vs generics / 何时用宏、函数或泛型](ch19-macros.md#when-to-use-what-macros-vs-functions-vs-generics)
- [Exercises / 练习](ch19-macros.md#exercises)
