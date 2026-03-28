# Summary / 概览

[Introduction / 简介](en/ch00-introduction.md) ([ZH](zh/ch00-introduction.md))

---

# Part I - Foundations / 第一部分：基础

- [1. Introduction and Motivation / 1. 引言与动机](en/ch01-introduction-and-motivation.md) ([ZH](zh/ch01-introduction-and-motivation.md))
- [2. Getting Started / 2. 快速开始](en/ch02-getting-started.md) ([ZH](zh/ch02-getting-started.md))
    - [Essential Keywords Reference / 核心关键字速查](en/ch02-1-essential-keywords-reference.md) ([ZH](zh/ch02-1-essential-keywords-reference.md))
- [3. Built-in Types and Variables / 3. 内建类型与变量](en/ch03-built-in-types-and-variables.md) ([ZH](zh/ch03-built-in-types-and-variables.md))
    - [True Immutability vs Record Illusions / 真正的不可变性与 Record 的“不可变幻觉”](en/ch03-1-true-immutability-vs-record-illusions.md) ([ZH](zh/ch03-1-true-immutability-vs-record-illusions.md))
- [4. Control Flow / 4. 控制流](en/ch04-control-flow.md) ([ZH](zh/ch04-control-flow.md))
- [5. Data Structures and Collections / 5. 数据结构与集合](en/ch05-data-structures-and-collections.md) ([ZH](zh/ch05-data-structures-and-collections.md))
    - [Constructor Patterns / 构造器模式](en/ch05-1-constructor-patterns.md) ([ZH](zh/ch05-1-constructor-patterns.md))
    - [Collections - Vec, HashMap, and Iterators / 集合：Vec、HashMap 与迭代器](en/ch05-2-collections-vec-hashmap-and-iterators.md) ([ZH](zh/ch05-2-collections-vec-hashmap-and-iterators.md))
- [6. Enums and Pattern Matching / 6. 枚举与模式匹配](en/ch06-enums-and-pattern-matching.md) ([ZH](zh/ch06-enums-and-pattern-matching.md))
    - [Exhaustive Matching and Null Safety / 穷尽匹配与空安全](en/ch06-1-exhaustive-matching-and-null-safety.md) ([ZH](zh/ch06-1-exhaustive-matching-and-null-safety.md))
- [7. Ownership and Borrowing / 7. 所有权与借用](en/ch07-ownership-and-borrowing.md) ([ZH](zh/ch07-ownership-and-borrowing.md))
    - [Memory Safety Deep Dive / 内存安全深入解析](en/ch07-1-memory-safety-deep-dive.md) ([ZH](zh/ch07-1-memory-safety-deep-dive.md))
    - [Lifetimes Deep Dive / 生命周期深入解析](en/ch07-2-lifetimes-deep-dive.md) ([ZH](zh/ch07-2-lifetimes-deep-dive.md))
    - [Smart Pointers - Beyond Single Ownership / 智能指针：超越单一所有权](en/ch07-3-smart-pointers-beyond-single-ownership.md) ([ZH](zh/ch07-3-smart-pointers-beyond-single-ownership.md))
- [8. Crates and Modules / 8. Crate 与模块](en/ch08-crates-and-modules.md) ([ZH](zh/ch08-crates-and-modules.md))
    - [Package Management / 包管理：Cargo 与 NuGet](en/ch08-1-package-management-cargo-vs-nuget.md) ([ZH](zh/ch08-1-package-management-cargo-vs-nuget.md))
- [9. Error Handling / 9. 错误处理](en/ch09-error-handling.md) ([ZH](zh/ch09-error-handling.md))
    - [Crate-Level Error Types and Result Aliases / Crate 级错误类型与 Result 别名](en/ch09-1-crate-level-error-types-and-result-alias.md) ([ZH](zh/ch09-1-crate-level-error-types-and-result-alias.md))
- [10. Traits and Generics / 10. Trait 与泛型](en/ch10-traits-and-generics.md) ([ZH](zh/ch10-traits-and-generics.md))
    - [Generic Constraints / 泛型约束](en/ch10-1-generic-constraints.md) ([ZH](zh/ch10-1-generic-constraints.md))
    - [Inheritance vs Composition / 继承与组合](en/ch10-2-inheritance-vs-composition.md) ([ZH](zh/ch10-2-inheritance-vs-composition.md))
- [11. From and Into Traits / 11. From 与 Into Trait](en/ch11-from-and-into-traits.md) ([ZH](zh/ch11-from-and-into-traits.md))
- [12. Closures and Iterators / 12. 闭包与迭代器](en/ch12-closures-and-iterators.md) ([ZH](zh/ch12-closures-and-iterators.md))
    - [Macros Primer / 宏入门](en/ch12-1-macros-primer.md) ([ZH](zh/ch12-1-macros-primer.md))

---

# Part II - Concurrency & Systems / 第二部分：并发与系统

- [13. Concurrency / 13. 并发](en/ch13-concurrency.md) ([ZH](zh/ch13-concurrency.md))
    - [Async/Await Deep Dive / Async/Await 深入解析](en/ch13-1-asyncawait-deep-dive.md) ([ZH](zh/ch13-1-asyncawait-deep-dive.md))
- [14. Unsafe Rust and FFI / 14. Unsafe Rust 与 FFI](en/ch14-unsafe-rust-and-ffi.md) ([ZH](zh/ch14-unsafe-rust-and-ffi.md))
    - [Testing / 测试](en/ch14-1-testing.md) ([ZH](zh/ch14-1-testing.md))

---

# Part III - Migration & Best Practices / 第三部分：迁移与最佳实践

- [15. Migration Patterns and Case Studies / 15. 迁移模式与案例研究](en/ch15-migration-patterns-and-case-studies.md) ([ZH](zh/ch15-migration-patterns-and-case-studies.md))
    - [Essential Crates for C# Developers / C# 开发者必备 Crate](en/ch15-1-essential-crates-for-c-developers.md) ([ZH](zh/ch15-1-essential-crates-for-c-developers.md))
    - [Incremental Adoption Strategy / 渐进式采用策略](en/ch15-2-incremental-adoption-strategy.md) ([ZH](zh/ch15-2-incremental-adoption-strategy.md))
- [16. Best Practices / 16. 最佳实践](en/ch16-best-practices.md) ([ZH](zh/ch16-best-practices.md))
    - [Performance Comparison and Migration / 性能对比与迁移](en/ch16-1-performance-comparison-and-migration.md) ([ZH](zh/ch16-1-performance-comparison-and-migration.md))
    - [Learning Path and Resources / 学习路径与资源](en/ch16-2-learning-path-and-resources.md) ([ZH](zh/ch16-2-learning-path-and-resources.md))
    - [Rust Tooling Ecosystem / Rust 工具链生态](en/ch16-3-rust-tooling-ecosystem.md) ([ZH](zh/ch16-3-rust-tooling-ecosystem.md))

---

# Capstone / 综合项目

- [17. Capstone Project / 综合项目](en/ch17-capstone-project.md) ([ZH](zh/ch17-capstone-project.md))
