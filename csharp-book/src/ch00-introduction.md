# <span class="lang-en">Rust for C# Programmers: Training Guide</span><span class="lang-zh">面向 C# 程序员的 Rust 培训指南</span>

<div class="lang-en">
A comprehensive guide to learning Rust for developers with C# experience. This guide covers everything from basic syntax to advanced patterns, focusing on the conceptual shifts and practical differences between the two languages.
</div>

<div class="lang-zh">
这是一本面向具有 C# 背景开发者的 Rust 学习指南，覆盖从基础语法到高级模式的完整内容，重点讲解两门语言在思维方式和实际用法上的差异。
</div>

---

## <span class="lang-en">Course Overview</span><span class="lang-zh">课程概览</span>

<div class="lang-en">
*   **The case for Rust** - Why Rust matters for C# developers: performance, safety, and correctness.
*   **Getting started** - Installation, tooling, and your first program.
*   **Basic building blocks** - Types, variables, control flow.
*   **Data structures** - Arrays, tuples, structs, collections.
*   **Pattern matching and enums** - Algebraic data types and exhaustive matching.
*   **Ownership and borrowing** - Rust's memory management model.
*   **Modules and crates** - Code organization and dependencies.
*   **Error handling** - Result-based error propagation.
*   **Traits and generics** - Rust's type system.
*   **Closures and iterators** - Functional programming patterns.
*   **Concurrency** - Fearless concurrency with type-system guarantees and async/await deep dive.
*   **Unsafe Rust and FFI** - When and how to go beyond safe Rust.
*   **Migration patterns** - Real-world C# to Rust patterns and incremental adoption.
*   **Best practices** - Idiomatic Rust for C# developers.
</div>

<div class="lang-zh">
*   **为什么选择 Rust**：性能、安全与正确性。
*   **快速开始**：安装、工具链与第一个程序。
*   **基础构件**：类型、变量、控制流。
*   **数据结构**：数组、元组、结构体、集合。
*   **模式匹配与枚举**：代数数据类型与穷尽匹配。
*   **所有权与借用**：Rust 的内存管理模型。
*   **模块与 crate**：代码组织与依赖管理。
*   **错误处理**：基于 `Result` 的错误传播。
*   **Trait 与泛型**：Rust 类型系统。
*   **闭包与迭代器**：函数式编程模式。
*   **并发**：由类型系统保证的无畏并发，以及 async/await 深入解析。
*   **Unsafe Rust 与 FFI**：何时以及如何超越安全 Rust。
*   **迁移模式**：真实世界中的 C# 到 Rust 模式与渐进迁移。
*   **最佳实践**：适合 C# 开发者的 Rust 惯用法。
</div>

---

## <span class="lang-en">Self-Study Guide</span><span class="lang-zh">自学指南</span>

<div class="lang-en">
This material works both as an instructor-led course and for self-study. If you're working through it on your own, here's how to get the most out of it.
</div>

<div class="lang-zh">
本材料既适合讲师授课，也适合自学。如果你打算自行学习，下面的建议能帮助你更高效地使用这套内容。
</div>

### <span class="lang-en">Pacing Recommendations</span><span class="lang-zh">学习节奏建议</span>

| **Chapters / 章节** | **Topic / 主题** | **Time / 时间** | **Checkpoint / 检查点** |
| :--- | :--- | :--- | :--- |
| 1-4 | <span class="lang-en">Setup, types, control flow</span><span class="lang-zh">环境、类型、控制流</span> | 1 day | <span class="lang-en">Write a CLI temp converter</span><span class="lang-zh">能写一个命令行温度转换器</span> |
| 5-6 | <span class="lang-en">Data, enums, pattern matching</span><span class="lang-zh">数据结构、枚举、模式匹配</span> | 1-2 days | <span class="lang-en">Understand algebraic types</span><span class="lang-zh">能理解并使用代数数据类型</span> |
| 7 | <span class="lang-en">Ownership & Borrowing</span><span class="lang-zh">所有权与借用</span> | 1-2 days | <span class="lang-en">Explain move semantics</span><span class="lang-zh">能解释移动语义</span> |
| 10-12 | <span class="lang-en">Traits, Generics, LINQ to Rust</span><span class="lang-zh">Trait、泛型、迭代器</span> | 1-2 days | <span class="lang-en">Translate LINQ to Iterators</span><span class="lang-zh">能将 LINQ 转换为迭代器</span> |

---

## <span class="lang-en">Table of Contents</span><span class="lang-zh">目录</span>

<div class="lang-en">
### Part I - Foundations
*   [1. Introduction and Motivation](ch01-introduction-and-motivation.md)
*   [2. Getting Started](ch02-getting-started.md)
*   [3. Built-in Types and Variables](ch03-built-in-types-and-variables.md)
*   [4. Control Flow](ch04-control-flow.md)
*   [5. Data Structures and Collections](ch05-data-structures-and-collections.md)
*   [6. Enums and Pattern Matching](ch06-enums-and-pattern-matching.md)
*   [7. Ownership and Borrowing](ch07-ownership-and-borrowing.md)
</div>

<div class="lang-zh">
### 第一部分：基础
*   [1. 引言与动机](ch01-introduction-and-motivation.md)
*   [2. 快速开始](ch02-getting-started.md)
*   [3. 内建类型与变量](ch03-built-in-types-and-variables.md)
*   [4. 控制流](ch04-control-flow.md)
*   [5. 数据结构与集合](ch05-data-structures-and-collections.md)
*   [6. 枚举与模式匹配](ch06-enums-and-pattern-matching.md)
*   [7. 所有权与借用](ch07-ownership-and-borrowing.md)
</div>
