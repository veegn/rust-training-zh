[English Original](../en/ch00-introduction.md)

# 面向 Python 程序员的 Rust 完整培训指南 🟢

这是一本面向 Python 开发者的 Rust 学习指南，涵盖从基础语法到高级模式的内容，重点讲解从动态类型、垃圾回收语言迁移到具备编译期内存安全保证的静态类型系统语言时所需要的思维转变。

## 如何使用本书

**自学建议**：先学习第一部分（第 1-6 章），这些内容与 Python 中已有概念最接近。第二部分（第 7-12 章）会引入 Rust 特有概念，如所有权和 trait。第三部分（第 13-16 章）讨论高级主题与迁移问题。

**学习节奏建议：**

| 章节 | 主题 | 建议时间 | 检查点 |
|----------|-------|---------------|------------|
| 1-4 | 环境、类型、控制流 | 1 天 | 你可以用 Rust 写出命令行温度转换器 |
| 5-6 | 数据结构、枚举、模式匹配 | 1-2 天 | 你可以定义携带数据的枚举并用 `match` 完整匹配 |
| 7 | 所有权与借用 | 1-2 天 | 你可以解释为什么 `let s2 = s1` 会使 `s1` 失效 |
| 8-9 | 模块、错误处理 | 1 天 | 你可以创建一个多文件项目并用 `?` 传播错误 |
| 10-12 | Trait、泛型、闭包、迭代器 | 1-2 天 | 你可以把列表推导式翻译成迭代器链 |
| 13 | 并发 | 1 天 | 你可以用 `Arc<Mutex<T>>` 写出线程安全计数器 |
| 14 | Unsafe、PyO3、测试 | 1 天 | 你可以通过 PyO3 从 Python 调用 Rust 函数 |
| 15-16 | 迁移、最佳实践 | 自定节奏 | 作为参考材料，在实际开发时按需查阅 |
| 17 | 综合项目 | 2-3 天 | 构建一个整合各章节内容的完整命令行应用 |

**如何使用练习：**
- 各章包含可折叠 `<details>` 区块中的动手练习及答案
- **总是先尝试练习，再展开答案。** 与借用检查器斗争本身就是学习过程，编译器的报错就是老师
- 如果卡住超过 15 分钟，就展开答案学习，然后收起并重新独立完成一次
- [Rust Playground](https://play.rust-lang.org/) 允许你在未本地安装 Rust 的情况下运行代码

**难度标记：**
- 🟢 **初级**：可以直接从 Python 概念迁移
- 🟡 **中级**：需要理解所有权或 trait
- 🔶 **高级**：生命周期、async 内部机制或 unsafe 代码

**遇到卡点时：**
- 仔细阅读编译器错误信息，Rust 的错误提示通常非常有帮助
- 重读相关小节，像所有权这样的概念往往第二遍才真正理解
- [Rust 标准库文档](https://doc.rust-lang.org/std/) 非常优秀，遇到类型或方法都值得去查
- 如需更深入的异步内容，请参考配套的 [Async Rust Training](../../async-book/)

---

## 目录

### 第一部分：基础

#### 1. 引言与动机 🟢
- [Rust 对 Python 开发者的价值](ch01-introduction-and-motivation.md#the-case-for-rust-for-python-developers)
- [Rust 能解决的 Python 常见痛点](ch01-introduction-and-motivation.md#common-python-pain-points-that-rust-addresses)
- [何时选择 Rust 而不是 Python](ch01-introduction-and-motivation.md#when-to-choose-rust-over-python)

#### 2. 快速开始 🟢
- [安装与环境配置](ch02-getting-started.md#installation-and-setup)
- [你的第一个 Rust 程序](ch02-getting-started.md#your-first-rust-program)
- [Cargo 与 pip/Poetry 对比](ch02-getting-started.md#cargo-vs-pippoetry)

#### 3. 内建类型与变量 🟢
- [变量与可变性](ch03-built-in-types-and-variables.md#variables-and-mutability)
- [基本类型对比](ch03-built-in-types-and-variables.md#primitive-types-comparison)
- [字符串类型：String 与 &str](ch03-built-in-types-and-variables.md#string-types-string-vs-str)

#### 4. 控制流 🟢
- [条件语句](ch04-control-flow.md#conditional-statements)
- [循环与迭代](ch04-control-flow.md#loops-and-iteration)
- [表达式块](ch04-control-flow.md#expression-blocks)
- [函数与类型签名](ch04-control-flow.md#functions-and-type-signatures)

#### 5. 数据结构与集合 🟢
- [元组、数组与切片](ch05-data-structures-and-collections.md#tuples-and-destructuring)
- [结构体与类](ch05-data-structures-and-collections.md#structs-vs-classes)
- [Vec 与 list，HashMap 与 dict](ch05-data-structures-and-collections.md#vec-vs-list)

#### 6. 枚举与模式匹配 🟡
- [代数数据类型与联合类型](ch06-enums-and-pattern-matching.md#algebraic-data-types-vs-union-types)
- [穷尽模式匹配](ch06-enums-and-pattern-matching.md#exhaustive-pattern-matching)
- [用 Option 实现 None 安全](ch06-enums-and-pattern-matching.md#option-for-none-safety)

### 第二部分：核心概念

#### 7. 所有权与借用 🟡
- [理解所有权](ch07-ownership-and-borrowing.md#understanding-ownership)
- [移动语义与引用计数](ch07-ownership-and-borrowing.md#move-semantics-vs-reference-counting)
- [借用与生命周期](ch07-ownership-and-borrowing.md#borrowing-and-lifetimes)
- [智能指针](ch07-ownership-and-borrowing.md#smart-pointers)

#### 8. Crate 与模块 🟢
- [Rust 模块与 Python 包](ch08-crates-and-modules.md#rust-modules-vs-python-packages)
- [Crate 与 PyPI 包](ch08-crates-and-modules.md#crates-vs-pypi-packages)

#### 9. 错误处理 🟡
- [异常与 Result](ch09-error-handling.md#exceptions-vs-result)
- [The ? Operator](ch09-error-handling.md#the--operator)
- [使用 thiserror 自定义错误类型](ch09-error-handling.md#custom-error-types-with-thiserror)

#### 10. Trait 与泛型 🟡
- [Trait 与 鸭子类型](ch10-traits-and-generics.md#traits-vs-duck-typing)
- [Protocol（PEP 544）与 Trait](ch10-traits-and-generics.md#protocols-pep-544-vs-traits)
- [泛型约束](ch10-traits-and-generics.md#generic-constraints)

#### 11. `From` 与 `Into` Trait 🟡
- [Rust 中的类型转换](ch11-from-and-into-traits.md#type-conversions-in-rust)
- [From、Into 与 TryFrom](ch11-from-and-into-traits.md#rust-frominto)
- [字符串转换模式](ch11-from-and-into-traits.md#string-conversions)

#### 12. 闭包与迭代器 🟡
- [闭包与 Lambda](ch12-closures-and-iterators.md#rust-closures-vs-python-lambdas)
- [迭代器与生成器](ch12-closures-and-iterators.md#iterators-vs-generators)
- [宏：生成代码的代码](ch12-closures-and-iterators.md#why-macros-exist-in-rust)

### 第三部分：高级主题与迁移

#### 13. 并发 🔶
- [没有 GIL：真正的并行](ch13-concurrency.md#no-gil-true-parallelism)
- [线程安全：由类型系统保证](ch13-concurrency.md#thread-safety-type-system-guarantees)
- [async/await 对比](ch13-concurrency.md#asyncawait-comparison)

#### 14. Unsafe Rust、FFI 与测试 🔶
- [何时以及为何使用 Unsafe](ch14-unsafe-rust-and-ffi.md#when-and-why-to-use-unsafe)
- [PyO3：为 Python 编写 Rust 扩展](ch14-unsafe-rust-and-ffi.md#pyo3-rust-extensions-for-python)
- [单元测试与 pytest](ch14-unsafe-rust-and-ffi.md#unit-tests-vs-pytest)

#### 15. 迁移模式 🟡
- [Rust 中的常见 Python 模式](ch15-migration-patterns.md#common-python-patterns-in-rust)
- [Python 开发者必备 Crate](ch08-crates-and-modules.md#essential-crates-for-python-developers)
- [渐进式采用策略](ch15-migration-patterns.md#incremental-adoption-strategy)

#### 16. 最佳实践 🟡
- [面向 Python 开发者的 Rust 惯用法](ch16-best-practices.md#idiomatic-rust-for-python-developers)
- [常见陷阱与解决方案](ch16-best-practices.md#common-pitfalls-and-solutions)
- [Python 到 Rust 对照速查](ch16-best-practices.md#rosetta-stone-python-to-rust)
- [学习路径与资源](ch16-best-practices.md#learning-path-and-resources)

---

### 第四部分：综合项目

#### 17. 综合项目：命令行任务管理器 🔶
- [项目：`rustdo`](ch17-capstone-project.md#the-project-rustdo)
- [数据模型、存储、命令与业务逻辑](ch17-capstone-project.md#step-1-define-the-data-model-ch-3-6-10-11)
- [测试与扩展目标](ch17-capstone-project.md#step-7-tests-ch-14)

***
