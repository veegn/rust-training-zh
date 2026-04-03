[English Original](../en/ch00-introduction.md)

# 面向 C/C++ 程序员的 Rust 入门强化课程 🟢

欢迎参加 **Rust 入门强化课程**。本课程专为具备 C/C++ 背景、希望利用其系统编程经验的高级开发者设计，旨在帮助你拥抱 Rust 的安全特性与现代功能。

## 课程概览

- **为什么选择 Rust**：C/C++ 开发者为何需要 Rust 以及它能消除哪些常见问题。
- **基础知识**：类型、函数、控制流以及模式匹配。
- **工具链**：模块、Cargo 以及工作空间（Workspaces）。
- **抽象能力**：Trait、泛型以及闭包。
- **内存与并发**：生命周期、智能指针以及线程安全。
- **系统与 FFI**：Unsafe Rust、外部函数接口（FFI）以及 `no_std`。
- **案例研究**：将 C++ 代码迁移到 Rust 的真实架构设计模式。

> [!NOTE]
> 本课程 **不** 涉及 `async` Rust。如需深入学习 Future、执行器（Executors）以及 `tokio`，请参阅配套的 [Async Rust Training](../../async-book/)。

---

## 自学指南

| 阶段 | 主题 | 建议时长 | 检查点 |
|-------|--------|----------------|------------|
| 1 | 环境搭建、类型、控制流 | 1 天 | 构建一个命令行计算器。 |
| 2 | 数据结构、所有权 | 1-2 天 | 理解为何 `let s2 = s1` 会移动 `s1`。 |
| 3 | 模块、错误处理 | 1 天 | 使用 `?` 操作符传播错误。 |
| 4 | Trait、泛型、闭包 | 1-2 天 | 编写带有 Trait 约束的泛型函数。 |
| 5 | 并发、Unsafe/FFI | 1 天 | 创建一个线程安全的计数器。 |

### 如何使用练习
每章都包含带难度标记的动手练习：
- 🟢 **入门 (Starter)**
- 🟡 **中级 (Intermediate)**
- 🔶 **挑战 (Challenge)**

**提示**：务必先独立尝试练习至少 15 分钟。与借用检查器（Borrow Checker）的“博弈”就是实际学习发生的地方。如果卡住了，请研究参考答案，然后尝试从头开始重写。

---

# 目录

## 第一部分 — 基础篇

### 1. 引言与动机
- [讲师介绍与通用方法](ch01-introduction-and-motivation.md#speaker-intro-and-general-approach)
- [为什么要用 Rust](ch01-introduction-and-motivation.md#the-case-for-rust)
- [Rust 如何解决这些问题？](ch01-introduction-and-motivation.md#how-does-rust-address-these-issues)
- [Rust 的其他核心卖点与特性](ch01-introduction-and-motivation.md#other-rust-usps-and-features)
- [快速参考：Rust vs C/C++](ch01-introduction-and-motivation.md#quick-reference-rust-vs-cc)
- [为什么 C/C++ 开发者需要 Rust](ch01-1-why-c-cpp-developers-need-rust.md)
  - [Rust 消除的问题清单](ch01-1-why-c-cpp-developers-need-rust.md#what-rust-eliminates--the-complete-list)
  - [C 与 C++ 共同的问题](ch01-1-why-c-cpp-developers-need-rust.md#the-problems-shared-by-c-and-c)
  - [C++ 额外引入的问题](ch01-1-why-c-cpp-developers-need-rust.md#c-adds-more-problems-on-top)
  - [Rust 如何应对这一切](ch01-1-why-c-cpp-developers-need-rust.md#how-rust-addresses-all-of-this)

### 2. 快速开始
- [少说废话：直接看代码](ch02-getting-started.md#enough-talk-already-show-me-some-code)
- [Rust 本地安装指南](ch02-getting-started.md#rust-local-installation)
- [Rust 软件包 (Crates)](ch02-getting-started.md#rust-packages-crates)
- [案例：Cargo 与 Crates](ch02-getting-started.md#example-cargo-and-crates)

### 3. 基础类型与变量
- [Rust 内建类型](ch03-built-in-types.md#built-in-rust-types)
- [类型指定与赋值](ch03-built-in-types.md#rust-type-specification-and-assignment)
- [类型指定与推导](ch03-built-in-types.md#rust-type-specification-and-inference)
- [变量与可变性](ch03-built-in-types.md#rust-variables-and-mutability)

### 4. 控制流
- [if 关键字](ch04-control-flow.md#rust-if-keyword)
- [使用 while 和 for 实现循环](ch04-control-flow.md#rust-loops-using-while-and-for)
- [使用 loop 实现循环](ch04-control-flow.md#rust-loops-using-loop)
- [表达式块](ch04-control-flow.md#rust-expression-blocks)

### 5. 数据结构与集合
- [数组类型](ch05-data-structures.md#rust-array-type)
- [元组 (Tuples)](ch05-data-structures.md#rust-tuples)
- [引用 (References)](ch05-data-structures.md#rust-references)
- [C++ 引用 vs Rust 引用 — 关键区别](ch05-data-structures.md#c-references-vs-rust-references--key-differences)
- [切片 (Slices)](ch05-data-structures.md#rust-slices)
- [常量与静态变量](ch05-data-structures.md#rust-constants-and-statics)
- [Rust 字符串：String vs &str](ch05-data-structures.md#rust-strings-string-vs-str)
- [结构体 (Structs)](ch05-data-structures.md#rust-structs)
- [Vec\<T\> 类型](ch05-data-structures.md#rust-vec-type)
- [HashMap 类型](ch05-data-structures.md#rust-hashmap-type)
- [练习：Vec 与 HashMap](ch05-data-structures.md#exercise-vec-and-hashmap)

### 6. 模式匹配与枚举
- [枚举类型](ch06-enums-and-pattern-matching.md#rust-enum-types)
- [match 语句](ch06-enums-and-pattern-matching.md#rust-match-statement)
- [练习：使用 match 与 enum 实现加减法](ch06-enums-and-pattern-matching.md#exercise-implement-add-and-subtract-using-match-and-enum)

### 7. 所有权与内存管理
- [内存管理概览](ch07-ownership-and-borrowing.md#rust-memory-management)
- [所有权、借用与生命周期](ch07-ownership-and-borrowing.md#rust-ownership-borrowing-and-lifetimes)
- [移动语义 (Move Semantics)](ch07-ownership-and-borrowing.md#rust-move-semantics)
- [Clone 特征](ch07-ownership-and-borrowing.md#rust-clone)
- [Copy 特征](ch07-ownership-and-borrowing.md#rust-copy-trait)
- [Drop 特征](ch07-ownership-and-borrowing.md#rust-drop-trait)
- [练习：Move, Copy 与 Drop](ch07-ownership-and-borrowing.md#exercise-move-copy-and-drop)
- [生命周期与借用深入解析](ch07-1-lifetimes-and-borrowing-deep-dive.md#rust-lifetime-and-borrowing)
- [生命周期标注](ch07-1-lifetimes-and-borrowing-deep-dive.md#rust-lifetime-annotations)
- [练习：带生命周期的切片存储](ch07-1-lifetimes-and-borrowing-deep-dive.md#exercise-slice-storage-with-lifetimes)
- [生命周期省略规则深挖](ch07-1-lifetimes-and-borrowing-deep-dive.md#lifetime-elision-rules-deep-dive)
- [Box\<T\> 智能指针](ch07-2-smart-pointers-and-interior-mutability.md#rust-boxt)
- [内部可变性：Cell\<T\> 与 RefCell\<T\>](ch07-2-smart-pointers-and-interior-mutability.md#interior-mutability-cellt-and-refcellt)
- [共享所有权：Rc\<T\>](ch07-2-smart-pointers-and-interior-mutability.md#shared-ownership-rct)
- [练习：共享所有权与内部可变性](ch07-2-smart-pointers-and-interior-mutability.md#exercise-shared-ownership-and-interior-mutability)

### 8. 模块与 Crate
- [Crates 与模块概览](ch08-crates-and-modules.md#rust-crates-and-modules)
- [练习：模块与函数](ch08-crates-and-modules.md#exercise-modules-and-functions)
- [工作空间 (Workspaces) 与软件包](ch08-crates-and-modules.md#workspaces-and-crates-packages)
- [练习：使用工作空间与包依赖](ch08-crates-and-modules.md#exercise-using-workspaces-and-package-dependencies)
- [使用来自 crates.io 的社区 Crate](ch08-crates-and-modules.md#using-community-crates-from-cratesio)
- [依赖管理与语义化版本 (SemVer)](ch08-crates-and-modules.md#crates-dependencies-and-semver)
- [练习：使用 rand Crate](ch08-crates-and-modules.md#exercise-using-the-rand-crate)
- [Cargo.toml 与 Cargo.lock](ch08-crates-and-modules.md#cargotoml-and-cargolock)
- [Cargo Test 功能](ch08-crates-and-modules.md#cargo-test-feature)
- [Cargo 的其他功能](ch08-crates-and-modules.md#other-cargo-features)
- [测试模式](ch08-1-testing-patterns.md)

### 9. 错误处理
- [将枚举连接到 Option 与 Result](ch09-error-handling.md#connecting-enums-to-option-and-result)
- [Option 类型](ch09-error-handling.md#rust-option-type)
- [Result 类型](ch09-error-handling.md#rust-result-type)
- [练习：使用 Option 实现 log() 函数](ch09-error-handling.md#exercise-log-function-implementation-with-option)
- [Rust 错误处理机制](ch09-error-handling.md#rust-error-handling)
- [练习：错误处理实战](ch09-error-handling.md#exercise-error-handling)
- [错误处理最佳实践](ch09-1-error-handling-best-practices.md)

### 10. Trait 与泛型
- [Rust Traits (特征)](ch10-traits.md#rust-traits)
- [C++ 运算符重载 → Rust std::ops Traits](ch10-traits.md#c-operator-overloading--rust-stdops-traits)
- [练习：Logger Trait 实现](ch10-traits.md#exercise-logger-trait-implementation)
- [何时使用 enum vs dyn Trait](ch10-traits.md#when-to-use-enum-vs-dyn-trait)
- [练习：翻译前的思考](ch10-traits.md#exercise-think-before-you-translate)
- [Rust 泛型](ch10-1-generics.md#rust-generics)
- [练习：泛型实战](ch10-1-generics.md#exercise-generics)
- [结合使用 Trait 与泛型](ch10-1-generics.md#combining-rust-traits-and-generics)
- [数据类型中的 Trait 约束](ch10-1-generics.md#rust-traits-constraints-in-data-types)
- [练习：Trait 约束与泛型](ch10-1-generics.md#exercise-traits-constraints-and-generics)
- [类型状态模式 (Type State Pattern)](ch10-1-generics.md#rust-type-state-pattern-and-generics)
- [构建者模式 (Builder Pattern)](ch10-1-generics.md#rust-builder-pattern)

### 11. 类型系统高级特性
- [From 与 Into Traits](ch11-from-and-into-traits.md#rust-from-and-into-traits)
- [练习：From 与 Into 实战](ch11-from-and-into-traits.md#exercise-from-and-into)
- [Default Trait](ch11-from-and-into-traits.md#rust-default-trait)
- [其他常用的类型转换](ch11-from-and-into-traits.md#other-rust-type-conversions)

### 12. 函数式编程
- [闭包 (Closures)](ch12-closures.md#rust-closures)
- [练习：闭包与捕获](ch12-closures.md#exercise-closures-and-capturing)
- [迭代器 (Iterators)](ch12-closures.md#rust-iterators)
- [练习：迭代器实战](ch12-closures.md#exercise-rust-iterators)
- [迭代器进阶工具参考](ch12-1-iterator-power-tools.md#iterator-power-tools-reference)

### 13. 并发编程
- [Rust 并发概览](ch13-concurrency.md#rust-concurrency)
- [防止数据竞态：Send 与 Sync](ch13-concurrency.md#why-rust-prevents-data-races-send-and-sync)
- [练习：多线程词频统计](ch13-concurrency.md#exercise-multi-threaded-word-count)

### 14. Unsafe Rust 与 FFI
- [非安全 (Unsafe) Rust](ch14-unsafe-rust-and-ffi.md#unsafe-rust)
- [简单 FFI 案例：C 调用 Rust 库](ch14-unsafe-rust-and-ffi.md#simple-ffi-example-rust-library-function-consumed-by-c)
- [复杂 FFI 案例](ch14-unsafe-rust-and-ffi.md#complex-ffi-example)
- [确保 Unsafe 代码的正确性](ch14-unsafe-rust-and-ffi.md#ensuring-correctness-of-unsafe-code)
- [练习：编写安全的 FFI 封装](ch14-unsafe-rust-and-ffi.md#exercise-writing-a-safe-ffi-wrapper)

## 第二部分 — 专题深入

### 15. no_std — 裸机环境下的 Rust
- [什么是 no_std？](ch15-no_std-rust-without-the-standard-library.md#what-is-no_std)
- [何时使用 no_std vs std](ch15-no_std-rust-without-the-standard-library.md#when-to-use-no_std-vs-std)
- [练习：no_std 环形缓冲区](ch15-no_std-rust-without-the-standard-library.md#exercise-no_std-ring-buffer)
- [嵌入式开发深入解析](ch15-1-embedded-deep-dive.md)

### 16. 案例研究：真实的 C++ 到 Rust 迁移
- [案例 1：继承层级 → Enum 派发](ch16-case-studies.md#case-study-1-inheritance-hierarchy--enum-dispatch)
- [案例 2：shared_ptr 树 → Arena/索引 模式](ch16-case-studies.md#case-study-2-shared_ptr-tree--arenaindex-pattern)
- [案例 3：框架通信 → 生命周期借用](ch16-1-case-study-lifetime-borrowing.md#case-study-3-framework-communication--lifetime-borrowing)
- [案例 4：上帝对象 → 可组合状态](ch16-1-case-study-lifetime-borrowing.md#case-study-4-god-object--composable-state)
- [案例 5：Trait 对象 — 何时才是正确选择](ch16-1-case-study-lifetime-borrowing.md#case-study-5-trait-objects--when-they-are-right)

## 第三部分 — 最佳实践与参考

### 17. 最佳实践
- [Rust 最佳实践总结](ch17-best-practices.md#rust-best-practices-summary)
- [避免过度使用 clone()](ch17-1-avoiding-excessive-clone.md#avoiding-excessive-clone)
- [避免未检查的索引访问](ch17-2-avoiding-unchecked-indexing.md#avoiding-unchecked-indexing)
- [精简赋值“金字塔”](ch17-3-collapsing-assignment-pyramids.md#collapsing-assignment-pyramids)
- [结业练习：诊断事件流水线](ch17-3-collapsing-assignment-pyramids.md#capstone-exercise-diagnostic-event-pipeline)
- [日志与追踪生态系统](ch17-4-logging-and-tracing-ecosystem.md#logging-and-tracing-ecosystem)

### 18. C++ → Rust 语义深挖
- [类型转换、预处理器、模块、volatile、static、constexpr、SFINAE 等](ch18-cpp-rust-semantic-deep-dives.md)

### 19. Rust 宏
- [声明式宏 (`macro_rules!`)](ch19-macros.md#declarative-macros-with-macro_rules)
- [标准库中的常用宏](ch19-macros.md#common-standard-library-macros)
- [派生宏 (Derive Macros)](ch19-macros.md#derive-macros)
- [属性宏 (Attribute Macros)](ch19-macros.md#attribute-macros)
- [过程宏 (Procedural Macros) 概念概览](ch19-macros.md#procedural-macros-conceptual-overview)
- [选型建议：宏 vs 函数 vs 泛型](ch19-macros.md#when-to-use-what-macros-vs-functions-vs-generics)
- [练习](ch19-macros.md#exercises)
