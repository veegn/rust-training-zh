# Summary / 概览

[Introduction](en/ch00-introduction.md) / [简介](zh/ch00-introduction.md)

---

# Part I - Foundations / 第一部分：基础

- [1. Introduction and Motivation](en/ch01-introduction-and-motivation.md) / [1. 引言与动机](zh/ch01-introduction-and-motivation.md)
- [2. Getting Started](en/ch02-getting-started.md) / [2. 快速开始](zh/ch02-getting-started.md)
    - [Essential Keywords Reference](en/ch02-1-essential-keywords-reference.md) / [核心关键字速查](zh/ch02-1-essential-keywords-reference.md)
- [3. Built-in Types and Variables](en/ch03-built-in-types-and-variables.md) / [3. 内建类型与变量](zh/ch03-built-in-types-and-variables.md)
    - [True Immutability vs Record Illusions](en/ch03-1-true-immutability-vs-record-illusions.md) / [真正的不可变性与 Record 的“不可变幻觉”](zh/ch03-1-true-immutability-vs-record-illusions.md)
- [4. Control Flow](en/ch04-control-flow.md) / [4. 控制流](zh/ch04-control-flow.md)
- [5. Data Structures and Collections](en/ch05-data-structures-and-collections.md) / [5. 数据结构与集合](zh/ch05-data-structures-and-collections.md)
    - [Constructor Patterns](en/ch05-1-constructor-patterns.md) / [构造器模式](zh/ch05-1-constructor-patterns.md)
    - [Collections - Vec, HashMap, and Iterators](en/ch05-2-collections-vec-hashmap-and-iterators.md) / [集合：Vec、HashMap 与迭代器](zh/ch05-2-collections-vec-hashmap-and-iterators.md)
- [6. Enums and Pattern Matching](en/ch06-enums-and-pattern-matching.md) / [6. 枚举与模式匹配](zh/ch06-enums-and-pattern-matching.md)
    - [Exhaustive Matching and Null Safety](en/ch06-1-exhaustive-matching-and-null-safety.md) / [穷尽匹配与空安全](zh/ch06-1-exhaustive-matching-and-null-safety.md)
- [7. Ownership and Borrowing](en/ch07-ownership-and-borrowing.md) / [7. 所有权与借用](zh/ch07-ownership-and-borrowing.md)
    - [Memory Safety Deep Dive](en/ch07-1-memory-safety-deep-dive.md) / [内存安全深入解析](zh/ch07-1-memory-safety-deep-dive.md)
    - [Lifetimes Deep Dive](en/ch07-2-lifetimes-deep-dive.md) / [生命周期深入解析](zh/ch07-2-lifetimes-deep-dive.md)
    - [Smart Pointers - Beyond Single Ownership](en/ch07-3-smart-pointers-beyond-single-ownership.md) / [智能指针：超越单一所有权](zh/ch07-3-smart-pointers-beyond-single-ownership.md)
- [8. Crates and Modules](en/ch08-crates-and-modules.md) / [8. Crate 与模块](zh/ch08-crates-and-modules.md)
    - [Package Management](en/ch08-1-package-management-cargo-vs-nuget.md) / [包管理：Cargo 与 NuGet](zh/ch08-1-package-management-cargo-vs-nuget.md)
- [9. Error Handling](en/ch09-error-handling.md) / [9. 错误处理](zh/ch09-error-handling.md)
    - [Crate-Level Error Types and Result Aliases](en/ch09-1-crate-level-error-types-and-result-alias.md) / [Crate 级错误类型与 Result 别名](zh/ch09-1-crate-level-error-types-and-result-alias.md)
- [10. Traits and Generics](en/ch10-traits-and-generics.md) / [10. Trait 与泛型](zh/ch10-traits-and-generics.md)
    - [Generic Constraints](en/ch10-1-generic-constraints.md) / [泛型约束](zh/ch10-1-generic-constraints.md)
    - [Inheritance vs Composition](en/ch10-2-inheritance-vs-composition.md) / [继承与组合](zh/ch10-2-inheritance-vs-composition.md)
- [11. From and Into Traits](en/ch11-from-and-into-traits.md) / [11. From 与 Into Trait](zh/ch11-from-and-into-traits.md)
- [12. Closures and Iterators](en/ch12-closures-and-iterators.md) / [12. 闭包与迭代器](zh/ch12-closures-and-iterators.md)
    - [Macros Primer](en/ch12-1-macros-primer.md) / [宏入门](zh/ch12-1-macros-primer.md)

---

# Part II - Concurrency & Systems / 第二部分：并发与系统

- [13. Concurrency](en/ch13-concurrency.md) / [13. 并发](zh/ch13-concurrency.md)
    - [Async/Await Deep Dive](en/ch13-1-asyncawait-deep-dive.md) / [Async/Await 深入解析](zh/ch13-1-asyncawait-deep-dive.md)
- [14. Unsafe Rust and FFI](en/ch14-unsafe-rust-and-ffi.md) / [14. Unsafe Rust 与 FFI](zh/ch14-unsafe-rust-and-ffi.md)
    - [Testing](en/ch14-1-testing.md) / [测试](zh/ch14-1-testing.md)

---

# Part III - Migration & Best Practices / 第三部分：迁移与最佳实践

- [15. Migration Patterns and Case Studies](en/ch15-migration-patterns-and-case-studies.md) / [15. 迁移模式与案例研究](zh/ch15-migration-patterns-and-case-studies.md)
    - [Essential Crates for C# Developers](en/ch15-1-essential-crates-for-c-developers.md) / [C# 开发者必备 Crate](zh/ch15-1-essential-crates-for-c-developers.md)
    - [Incremental Adoption Strategy](en/ch15-2-incremental-adoption-strategy.md) / [渐进式采用策略](zh/ch15-2-incremental-adoption-strategy.md)
- [16. Best Practices](en/ch16-best-practices.md) / [16. 最佳实践](zh/ch16-best-practices.md)
    - [Performance Comparison and Migration](en/ch16-1-performance-comparison-and-migration.md) / [性能对比与迁移](zh/ch16-1-performance-comparison-and-migration.md)
    - [Learning Path and Resources](en/ch16-2-learning-path-and-resources.md) / [学习路径与资源](zh/ch16-2-learning-path-and-resources.md)
    - [Rust Tooling Ecosystem](en/ch16-3-rust-tooling-ecosystem.md) / [Rust 工具链生态](zh/ch16-3-rust-tooling-ecosystem.md)

---

# Capstone / 综合项目

- [17. Capstone Project](en/ch17-capstone-project.md) / [17. 综合项目](zh/ch17-capstone-project.md)
