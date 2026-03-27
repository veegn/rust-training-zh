# Type-Driven Correctness in Rust / Rust 中的类型驱动正确性

## Speaker Intro / 讲师简介

- Principal Firmware Architect in Microsoft SCHIE (Silicon and Cloud Hardware Infrastructure Engineering) team / Microsoft SCHIE（Silicon and Cloud Hardware Infrastructure Engineering）团队首席固件架构师
- Industry veteran with expertise in security, systems programming (firmware, operating systems, hypervisors), CPU and platform architecture, and C++ systems / 在安全、系统编程（固件、操作系统、虚拟机监控器）、CPU 与平台架构以及 C++ 系统方面经验丰富
- Started programming in Rust in 2017 (@AWS EC2), and have been in love with the language ever since / 2017 年在 AWS EC2 开始使用 Rust，此后长期深度投入

---

A practical guide to using Rust's type system to make entire classes of bugs **impossible to compile**. While the companion [Rust Patterns](../rust-patterns-book/src/SUMMARY.md) book covers the mechanics (traits, associated types, type-state), this guide shows how to **apply** those mechanics to real-world domains - hardware diagnostics, cryptography, protocol validation, and embedded systems.

这是一本关于如何利用 Rust 类型系统，让整类 bug **在编译阶段就无法出现** 的实用指南。配套书籍 [Rust Patterns](../rust-patterns-book/src/SUMMARY.md) 讲解了相关机制（trait、关联类型、type-state），而本书则聚焦于如何把这些机制 **应用** 到真实领域中，例如硬件诊断、密码学、协议校验与嵌入式系统。

Every pattern here follows one principle: **push invariants from runtime checks into the type system so the compiler enforces them.**

本书中的每个模式都遵循同一个原则：**把原本依赖运行时检查的不变量推进到类型系统中，让编译器来强制保证它们成立。**

## How to Use This Book / 如何使用本书

### Difficulty Legend / 难度说明

| Symbol / 标记 | Level / 等级 | Audience / 适合读者 |
|:------:|-------|----------|
| 🟢 | Introductory / 入门 | Comfortable with ownership + traits / 已熟悉所有权与 trait |
| 🟡 | Intermediate / 中级 | Familiar with generics + associated types / 已熟悉泛型与关联类型 |
| 🔶 | Advanced / 高级 | Ready for type-state, phantom types, and session types / 准备学习 type-state、phantom type 与 session type |

### Pacing Guide / 学习节奏建议

| Goal / 目标 | Path / 路径 | Time / 时间 |
|------|------|------|
| **Quick overview / 快速概览** | ch01, ch13 (reference card) / ch01、ch13（速查卡） | 30 min / 30 分钟 |
| **IPMI / BMC developer** | ch02, ch05, ch07, ch10, ch17 | 2.5 hrs / 2.5 小时 |
| **GPU / PCIe developer** | ch02, ch06, ch09, ch10, ch15 | 2.5 hrs / 2.5 小时 |
| **Redfish implementer** | ch02, ch05, ch07, ch08, ch17, ch18 | 3 hrs / 3 小时 |
| **Framework / infrastructure** | ch04, ch08, ch11, ch14, ch18 | 2.5 hrs / 2.5 小时 |
| **New to correct-by-construction / 初学“构造即正确”** | ch01 to ch10 in order, then ch12 exercises / 按顺序阅读 ch01 到 ch10，再做 ch12 练习 | 4 hrs / 4 小时 |
| **Full deep dive / 完整深潜** | All chapters sequentially / 顺序阅读全部章节 | 7 hrs / 7 小时 |

### Annotated Table of Contents / 带说明的目录

| Ch / 章 | Title / 标题 | Difficulty / 难度 | Key Idea / 核心思想 |
|----|-------|:----------:|----------|
| 1 | The Philosophy - Why Types Beat Tests / 核心理念：为什么类型优于测试 | 🟢 | Three levels of correctness; types as compiler-checked guarantees / 正确性的三个层次；类型作为编译器可检查的保证 |
| 2 | Typed Command Interfaces / 类型化命令接口 | 🟡 | Associated types bind request to response / 通过关联类型将请求与响应绑定 |
| 3 | Single-Use Types / 单次使用类型 | 🟡 | Move semantics as linear types for crypto / 用移动语义为密码学场景提供线性类型保证 |
| 4 | Capability Tokens / 能力令牌 | 🟡 | Zero-sized proof-of-authority tokens / 零大小的权限证明令牌 |
| 5 | Protocol State Machines / 协议状态机 | 🔶 | Type-state for IPMI sessions + PCIe LTSSM / 将 type-state 用于 IPMI 会话与 PCIe LTSSM |
| 6 | Dimensional Analysis / 量纲分析 | 🟢 | Newtype wrappers prevent unit mix-ups / 用 newtype 包装器防止单位混淆 |
| 7 | Validated Boundaries / 已验证边界 | 🟡 | Parse once at the edge, carry proof in types / 在边界处一次性解析，并通过类型携带证明 |
| 8 | Capability Mixins / 能力混入 | 🟡 | Ingredient traits + blanket impls / 组件化 trait 与 blanket impl |
| 9 | Phantom Types / Phantom Type | 🟡 | PhantomData for register width, DMA direction / 用 PhantomData 表示寄存器宽度、DMA 方向等信息 |
| 10 | Putting It All Together / 综合实战 | 🟡 | All 7 patterns in one diagnostic platform / 在一个诊断平台中组合全部 7 类模式 |
| 11 | Fourteen Tricks from the Trenches / 来自一线的十四个技巧 | 🟡 | Sentinel to Option, sealed traits, builders, etc. / 哨兵值转 Option、sealed trait、builder 等技巧 |
| 12 | Exercises / 练习 | 🟡 | Six capstone problems with solutions / 六个带答案的综合题 |
| 13 | Reference Card / 速查卡 | - | Pattern catalogue + decision flowchart / 模式目录与决策流程图 |
| 14 | Testing Type-Level Guarantees / 测试类型层保证 | 🟡 | trybuild, proptest, cargo-show-asm / trybuild、proptest、cargo-show-asm |
| 15 | Const Fn / `const fn` | 🔶 | Compile-time proofs for memory maps, registers, bitfields / 为内存映射、寄存器、位字段提供编译期证明 |
| 16 | Send & Sync / `Send` 与 `Sync` | 🔶 | Compile-time concurrency proofs / 编译期并发正确性证明 |
| 17 | Redfish Client Walkthrough / Redfish 客户端实战讲解 | 🟡 | Eight patterns composed into a type-safe Redfish client / 将八种模式组合成类型安全的 Redfish 客户端 |
| 18 | Redfish Server Walkthrough / Redfish 服务器实战讲解 | 🟡 | Builder type-state, source tokens, health rollup, mixins / builder type-state、源令牌、健康汇总与 mixin |

## Prerequisites / 前置知识

| Concept / 概念 | Where to learn it / 建议学习位置 |
|---------|-------------------|
| Ownership and borrowing / 所有权与借用 | [Rust Patterns](../rust-patterns-book/src/SUMMARY.md), ch01 |
| Traits and associated types / Trait 与关联类型 | [Rust Patterns](../rust-patterns-book/src/SUMMARY.md), ch02 |
| Newtypes and type-state / Newtype 与 type-state | [Rust Patterns](../rust-patterns-book/src/SUMMARY.md), ch03 |
| PhantomData / PhantomData | [Rust Patterns](../rust-patterns-book/src/SUMMARY.md), ch04 |
| Generics and trait bounds / 泛型与 trait 约束 | [Rust Patterns](../rust-patterns-book/src/SUMMARY.md), ch01 |

## The Correct-by-Construction Spectrum / “构造即正确”光谱

```text
Less Safe                                                    More Safe

Runtime checks      Unit tests        Property tests      Correct by Construction
----------------    ----------        --------------      -----------------------

if temp > 100 {     #[test]           proptest! {         struct Celsius(f64);
  panic!("too       fn test_temp() {    |t in 0..200| {   // Can't confuse with Rpm
  hot");              assert!(          assert!(...)       // at the type level
}                     check(42));     }
                    }                 }

Invalid program?    Invalid program?  Invalid program?    Invalid program?
Crashes in prod.    Fails in CI.      Fails in CI         Won't compile.
                                      (probabilistic).    Never exists.
```

This guide operates at the rightmost position - where bugs don't exist because the type system **cannot express them**.

本书聚焦于最右侧的那一端：bug 之所以不存在，不是因为它们被测试捕获，而是因为类型系统 **根本无法表达这些错误程序**。

---
