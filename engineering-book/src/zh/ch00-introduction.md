# Rust 工程实践：超越 `cargo build` 🟢

## 讲师简介

- Microsoft SCHIE（Silicon and Cloud Hardware Infrastructure Engineering）团队首席固件架构师
- 在安全、系统编程（固件、操作系统、虚拟机监控器）、CPU 与平台架构以及 C++ 系统方面经验丰富
- 2017 年在 AWS EC2 开始使用 Rust，此后长期深度投入

---

> 这是一本聚焦 Rust 工具链实践的实用指南，覆盖许多团队往往接触得太晚的关键能力：构建脚本、交叉编译、基准测试、代码覆盖率，以及借助 Miri 和 Valgrind 做安全验证。每章都基于真实硬件诊断代码库中的具体示例展开，该代码库是一个大型多 crate 工作区，因此书中的每项技巧都能直接映射到生产代码。

## 如何使用本书

本书适合 **自定节奏学习或团队工作坊**。各章大体独立，你既可以按顺序阅读，也可以直接跳到当前最需要的主题。

### 难度说明

| 标记 | 等级 | 含义 |
|:------:|-------|---------|
| 🟢 | 入门 | 规则清晰、上手直接，第一天就能用到 |
| 🟡 | 中级 | 需要理解工具链内部机制或平台概念 |
| 🔶 | 高级 | 涉及更深的工具链知识、nightly 特性或多工具协同 |

### 学习节奏建议

| 部分 | 章节 | 预计时间 | 关键收获 |
|------|----------|:---------:|-------------|
| **I - 构建与交付** | ch01-ch02 | 3-4 小时 | 构建元数据、交叉编译、静态二进制 |
| **II - 度量与验证** | ch03-ch05 | 4-5 小时 | 统计型基准测试、覆盖率门禁、Miri 与 sanitizer |
| **III - 加固与优化** | ch06-ch10 | 6-8 小时 | 供应链安全、发布配置、编译期工具、`no_std` 与 Windows |
| **IV - 集成** | ch11-ch13 | 3-4 小时 | 生产级 CI/CD 流水线、实践技巧与综合练习 |
| | | **16-21 小时** | **完整生产工程流水线视角** |

### 练习建议

每章都包含带难度标记的 **练习**。答案放在可展开的 `<details>` 区块中，建议先做题，再核对答案。

- 🟢 练习通常可在 10-15 分钟内完成
- 🟡 练习通常需要 20-30 分钟，并可能需要本地运行工具
- 🔶 练习通常需要较多环境准备与实验时间（1 小时以上）

## 前置知识

| 概念 | 建议学习位置 |
|---------|-------------------|
| Cargo 工作区结构 | [Rust Book ch14.3](https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html) |
| Feature 标志 | [Cargo Reference - Features](https://doc.rust-lang.org/cargo/reference/features.html) |
| `#[cfg(test)]` 与基础测试 | Rust Patterns 第 12 章 |
| `unsafe` 代码块与 FFI 基础 | Rust Patterns 第 10 章 |

## 章节依赖图

```text
                 +-----------------+
                 | ch00            |
                 | Intro           |
                 +----+-----+------+
        +--------+----+---+--+---+---------+------+
        |        |        |      |         |      |
      ch01     ch03     ch04   ch05      ch06   ch09
      Build    Bench    Cov    Miri      Deps   no_std
        |        |       |      |         |      |
        |        +-------+------+         |      |
        |                |                |    ch10
       ch02             ch07             ch07  Windows
       Cross            RelProf          RelProf
        |                |                |      |
        |               ch08              |      |
        |             CompTime            |      |
        +----------------+----------------+------+
                         |
                        ch11
                      CI/CD Pipeline
                         |
                        ch12 ---- ch13
                       Tricks   Quick Ref
```

**可任意顺序阅读**：ch01、ch03、ch04、ch05、ch06、ch09 相互独立。  
**建议在具备前置知识后阅读**：ch02（依赖 ch01），ch07-ch08（先学 ch03-ch06 效果更好），ch10（最好先看 ch09）。  
**建议最后阅读**：ch11（综合收束全书）、ch12（技巧汇总）、ch13（参考速查）。

## 带说明的目录

### 第一部分：构建与交付

| # | 章节 | 难度 | 说明 |
|---|---------|:----------:|-------------|
| 1 | [构建脚本：深入理解 `build.rs`](ch01-build-scripts-buildrs-in-depth.md) | 🟢 | 编译期常量、编译 C 代码、protobuf 生成、系统库链接与反模式 |
| 2 | [交叉编译：一份源码，多种目标](ch02-cross-compilation-one-source-many-target.md) | 🟡 | 目标三元组、musl 静态二进制、ARM 交叉编译、`cross`、`cargo-zigbuild` 与 GitHub Actions |

### 第二部分：度量与验证

| # | 章节 | 难度 | 说明 |
|---|---------|:----------:|-------------|
| 3 | [基准测试：衡量真正重要的指标](ch03-benchmarking-measuring-what-matters.md) | 🟡 | Criterion.rs、Divan、`perf` 火焰图、PGO 与 CI 中的持续基准测试 |
| 4 | [代码覆盖率：发现测试遗漏](ch04-code-coverage-seeing-what-tests-miss.md) | 🟢 | `cargo-llvm-cov`、`cargo-tarpaulin`、`grcov` 与 Codecov/Coveralls 集成 |
| 5 | [Miri、Valgrind 与 Sanitizer](ch05-miri-valgrind-and-sanitizers-verifying-u.md) | 🔶 | MIR 解释器、Valgrind memcheck/Helgrind、ASan/MSan/TSan、cargo-fuzz 与 loom |

### 第三部分：加固与优化

| # | 章节 | 难度 | 说明 |
|---|---------|:----------:|-------------|
| 6 | [依赖管理与供应链安全](ch06-dependency-management-and-supply-chain-s.md) | 🟢 | `cargo-audit`、`cargo-deny`、`cargo-vet`、`cargo-outdated` 与 `cargo-semver-checks` |
| 7 | [发布配置与二进制体积](ch07-release-profiles-and-binary-size.md) | 🟡 | 发布配置结构、LTO 权衡、`cargo-bloat` 与 `cargo-udeps` |
| 8 | [编译期与开发者工具](ch08-compile-time-and-developer-tools.md) | 🟡 | `sccache`、`mold`、`cargo-nextest`、`cargo-expand`、`cargo-geiger`、工作区 lint 与 MSRV |
| 9 | [`no_std` 与特性验证](ch09-no-std-and-feature-verification.md) | 🔶 | `cargo-hack`、`core`/`alloc`/`std` 分层、自定义 panic handler 与 `no_std` 代码测试 |
| 10 | [Windows 与条件编译](ch10-windows-and-conditional-compilation.md) | 🟡 | `#[cfg]` 模式、`windows-sys`/`windows` crate、`cargo-xwin` 与平台抽象 |

### 第四部分：集成

| # | 章节 | 难度 | 说明 |
|---|---------|:----------:|-------------|
| 11 | [综合实战：生产级 CI/CD 流水线](ch11-putting-it-all-together-a-production-cic.md) | 🟡 | GitHub Actions 工作流、`cargo-make`、pre-commit hook、`cargo-dist` 与综合实战 |
| 12 | [一线实践技巧](ch12-tricks-from-the-trenches.md) | 🟡 | 10 个经验证的实战模式：`deny(warnings)` 陷阱、缓存调优、依赖去重、RUSTFLAGS 等 |
| 13 | [速查卡](ch13-quick-reference-card.md) | - | 命令速览、60+ 条决策表条目以及延伸阅读链接 |
