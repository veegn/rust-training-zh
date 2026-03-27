# Rust Engineering Practices - Beyond `cargo build` / Rust 工程实践：超越 `cargo build`

## Speaker Intro / 讲师简介

- Principal Firmware Architect in Microsoft SCHIE (Silicon and Cloud Hardware Infrastructure Engineering) team / Microsoft SCHIE（Silicon and Cloud Hardware Infrastructure Engineering）团队首席固件架构师
- Industry veteran with expertise in security, systems programming (firmware, operating systems, hypervisors), CPU and platform architecture, and C++ systems / 在安全、系统编程（固件、操作系统、虚拟机监控器）、CPU 与平台架构以及 C++ 系统方面经验丰富
- Started programming in Rust in 2017 (@AWS EC2), and have been in love with the language ever since / 2017 年在 AWS EC2 开始使用 Rust，此后长期深度投入

---

> A practical guide to the Rust toolchain features that most teams discover too late: build scripts, cross-compilation, benchmarking, code coverage, and safety verification with Miri and Valgrind. Each chapter uses concrete examples drawn from a real hardware-diagnostics codebase - a large multi-crate workspace - so every technique maps directly to production code.
>
> 这是一本聚焦 Rust 工具链实践的实用指南，覆盖许多团队往往接触得太晚的关键能力：构建脚本、交叉编译、基准测试、代码覆盖率，以及借助 Miri 和 Valgrind 做安全验证。每章都基于真实硬件诊断代码库中的具体示例展开，该代码库是一个大型多 crate 工作区，因此书中的每项技巧都能直接映射到生产代码。

## How to Use This Book / 如何使用本书

This book is designed for **self-paced study or team workshops**. Each chapter is largely independent - read them in order or jump to the topic you need.

本书适合 **自定节奏学习或团队工作坊**。各章大体独立，你既可以按顺序阅读，也可以直接跳到当前最需要的主题。

### Difficulty Legend / 难度说明

| Symbol / 标记 | Level / 等级 | Meaning / 含义 |
|:------:|-------|---------|
| 🟢 | Starter / 入门 | Straightforward tools with clear patterns - useful on day one / 规则清晰、上手直接，第一天就能用到 |
| 🟡 | Intermediate / 中级 | Requires understanding of toolchain internals or platform concepts / 需要理解工具链内部机制或平台概念 |
| 🔶 | Advanced / 高级 | Deep toolchain knowledge, nightly features, or multi-tool orchestration / 涉及更深的工具链知识、nightly 特性或多工具协同 |

### Pacing Guide / 学习节奏建议

| Part / 部分 | Chapters / 章节 | Est. Time / 预计时间 | Key Outcome / 关键收获 |
|------|----------|:---------:|-------------|
| **I - Build & Ship / 构建与交付** | ch01-ch02 | 3-4 h / 3-4 小时 | Build metadata, cross-compilation, static binaries / 构建元数据、交叉编译、静态二进制 |
| **II - Measure & Verify / 度量与验证** | ch03-ch05 | 4-5 h / 4-5 小时 | Statistical benchmarking, coverage gates, Miri/sanitizers / 统计型基准测试、覆盖率门禁、Miri 与 sanitizer |
| **III - Harden & Optimize / 加固与优化** | ch06-ch10 | 6-8 h / 6-8 小时 | Supply chain security, release profiles, compile-time tools, `no_std`, Windows / 供应链安全、发布配置、编译期工具、`no_std` 与 Windows |
| **IV - Integrate / 集成** | ch11-ch13 | 3-4 h / 3-4 小时 | Production CI/CD pipeline, tricks, capstone exercise / 生产级 CI/CD 流水线、实践技巧与综合练习 |
| | | **16-21 h** | **Full production engineering pipeline / 完整生产工程流水线视角** |

### Working Through Exercises / 练习建议

Each chapter contains **exercises** with difficulty indicators. Solutions are provided in expandable `<details>` blocks - try the exercise first, then check your work.

每章都包含带难度标记的 **练习**。答案放在可展开的 `<details>` 区块中，建议先做题，再核对答案。

- 🟢 exercises can often be done in 10-15 minutes / 🟢 练习通常可在 10-15 分钟内完成
- 🟡 exercises require 20-30 minutes and may involve running tools locally / 🟡 练习通常需要 20-30 分钟，并可能需要本地运行工具
- 🔶 exercises require significant setup and experimentation (1+ hour) / 🔶 练习通常需要较多环境准备与实验时间（1 小时以上）

## Prerequisites / 前置知识

| Concept / 概念 | Where to learn it / 建议学习位置 |
|---------|-------------------|
| Cargo workspace layout / Cargo 工作区结构 | [Rust Book ch14.3](https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html) |
| Feature flags / Feature 标志 | [Cargo Reference - Features](https://doc.rust-lang.org/cargo/reference/features.html) |
| `#[cfg(test)]` and basic testing / `#[cfg(test)]` 与基础测试 | Rust Patterns ch12 / Rust Patterns 第 12 章 |
| `unsafe` blocks and FFI basics / `unsafe` 代码块与 FFI 基础 | Rust Patterns ch10 / Rust Patterns 第 10 章 |

## Chapter Dependency Map / 章节依赖图

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

**Read in any order**: ch01, ch03, ch04, ch05, ch06, ch09 are independent.  
**Read after prerequisites**: ch02 (needs ch01), ch07-ch08 (benefit from ch03-ch06), ch10 (benefits from ch09).  
**Read last**: ch11 (ties everything together), ch12 (tricks), ch13 (reference).

**可任意顺序阅读**：ch01、ch03、ch04、ch05、ch06、ch09 相互独立。  
**建议在具备前置知识后阅读**：ch02（依赖 ch01），ch07-ch08（先学 ch03-ch06 效果更好），ch10（最好先看 ch09）。  
**建议最后阅读**：ch11（综合收束全书）、ch12（技巧汇总）、ch13（参考速查）。

## Annotated Table of Contents / 带说明的目录

### Part I - Build & Ship / 第一部分：构建与交付

| # | Chapter / 章节 | Difficulty / 难度 | Description / 说明 |
|---|---------|:----------:|-------------|
| 1 | [Build Scripts - `build.rs` in Depth / 构建脚本：深入理解 `build.rs`](ch01-build-scripts-buildrs-in-depth.md) | 🟢 | Compile-time constants, compiling C code, protobuf generation, system library linking, anti-patterns / 编译期常量、编译 C 代码、protobuf 生成、系统库链接与反模式 |
| 2 | [Cross-Compilation - One Source, Many Targets / 交叉编译：一份源码，多种目标](ch02-cross-compilation-one-source-many-target.md) | 🟡 | Target triples, musl static binaries, ARM cross-compile, `cross`, `cargo-zigbuild`, GitHub Actions / 目标三元组、musl 静态二进制、ARM 交叉编译、`cross`、`cargo-zigbuild` 与 GitHub Actions |

### Part II - Measure & Verify / 第二部分：度量与验证

| # | Chapter / 章节 | Difficulty / 难度 | Description / 说明 |
|---|---------|:----------:|-------------|
| 3 | [Benchmarking - Measuring What Matters / 基准测试：衡量真正重要的指标](ch03-benchmarking-measuring-what-matters.md) | 🟡 | Criterion.rs, Divan, `perf` flamegraphs, PGO, continuous benchmarking in CI / Criterion.rs、Divan、`perf` 火焰图、PGO 与 CI 中的持续基准测试 |
| 4 | [Code Coverage - Seeing What Tests Miss / 代码覆盖率：发现测试遗漏](ch04-code-coverage-seeing-what-tests-miss.md) | 🟢 | `cargo-llvm-cov`, `cargo-tarpaulin`, `grcov`, Codecov/Coveralls CI integration / `cargo-llvm-cov`、`cargo-tarpaulin`、`grcov` 与 Codecov/Coveralls 集成 |
| 5 | [Miri, Valgrind, and Sanitizers / Miri、Valgrind 与 Sanitizer](ch05-miri-valgrind-and-sanitizers-verifying-u.md) | 🔶 | MIR interpreter, Valgrind memcheck/Helgrind, ASan/MSan/TSan, cargo-fuzz, loom / MIR 解释器、Valgrind memcheck/Helgrind、ASan/MSan/TSan、cargo-fuzz 与 loom |

### Part III - Harden & Optimize / 第三部分：加固与优化

| # | Chapter / 章节 | Difficulty / 难度 | Description / 说明 |
|---|---------|:----------:|-------------|
| 6 | [Dependency Management and Supply Chain Security / 依赖管理与供应链安全](ch06-dependency-management-and-supply-chain-s.md) | 🟢 | `cargo-audit`, `cargo-deny`, `cargo-vet`, `cargo-outdated`, `cargo-semver-checks` / `cargo-audit`、`cargo-deny`、`cargo-vet`、`cargo-outdated` 与 `cargo-semver-checks` |
| 7 | [Release Profiles and Binary Size / 发布配置与二进制体积](ch07-release-profiles-and-binary-size.md) | 🟡 | Release profile anatomy, LTO trade-offs, `cargo-bloat`, `cargo-udeps` / 发布配置结构、LTO 权衡、`cargo-bloat` 与 `cargo-udeps` |
| 8 | [Compile-Time and Developer Tools / 编译期与开发者工具](ch08-compile-time-and-developer-tools.md) | 🟡 | `sccache`, `mold`, `cargo-nextest`, `cargo-expand`, `cargo-geiger`, workspace lints, MSRV / `sccache`、`mold`、`cargo-nextest`、`cargo-expand`、`cargo-geiger`、工作区 lint 与 MSRV |
| 9 | [`no_std` and Feature Verification / `no_std` 与特性验证](ch09-no-std-and-feature-verification.md) | 🔶 | `cargo-hack`, `core`/`alloc`/`std` layering, custom panic handlers, testing `no_std` code / `cargo-hack`、`core`/`alloc`/`std` 分层、自定义 panic handler 与 `no_std` 代码测试 |
| 10 | [Windows and Conditional Compilation / Windows 与条件编译](ch10-windows-and-conditional-compilation.md) | 🟡 | `#[cfg]` patterns, `windows-sys`/`windows` crates, `cargo-xwin`, platform abstraction / `#[cfg]` 模式、`windows-sys`/`windows` crate、`cargo-xwin` 与平台抽象 |

### Part IV - Integrate / 第四部分：集成

| # | Chapter / 章节 | Difficulty / 难度 | Description / 说明 |
|---|---------|:----------:|-------------|
| 11 | [Putting It All Together - A Production CI/CD Pipeline / 综合实战：生产级 CI/CD 流水线](ch11-putting-it-all-together-a-production-cic.md) | 🟡 | GitHub Actions workflow, `cargo-make`, pre-commit hooks, `cargo-dist`, capstone / GitHub Actions 工作流、`cargo-make`、pre-commit hook、`cargo-dist` 与综合实战 |
| 12 | [Tricks from the Trenches / 一线实践技巧](ch12-tricks-from-the-trenches.md) | 🟡 | 10 battle-tested patterns: `deny(warnings)` trap, cache tuning, dep dedup, RUSTFLAGS, more / 10 个经验证的实战模式：`deny(warnings)` 陷阱、缓存调优、依赖去重、RUSTFLAGS 等 |
| 13 | [Quick Reference Card / 速查卡](ch13-quick-reference-card.md) | - | Commands at a glance, 60+ decision table entries, further reading links / 命令速览、60+ 条决策表条目以及延伸阅读链接 |
