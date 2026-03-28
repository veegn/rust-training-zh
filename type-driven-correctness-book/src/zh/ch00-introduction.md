# Rust 中的类型驱动正确性

## 讲师简介

- Microsoft SCHIE（Silicon and Cloud Hardware Infrastructure Engineering）团队首席固件架构师
- 在安全、系统编程（固件、操作系统、虚拟机监控器）、CPU 与平台架构以及 C++ 系统方面经验丰富
- 2017 年在 AWS EC2 开始使用 Rust，此后长期深度投入

---

这是一本关于如何利用 Rust 类型系统，让整类 bug **在编译阶段就无法出现** 的实用指南。配套书籍 [Rust Patterns](../rust-patterns-book/src/SUMMARY.md) 讲解了相关机制（trait、关联类型、type-state），而本书则聚焦于如何把这些机制 **应用** 到真实领域中，例如硬件诊断、密码学、协议校验与嵌入式系统。

本书中的每个模式都遵循同一个原则：**把原本依赖运行时检查的不变量推进到类型系统中，让编译器来强制保证它们成立。**

## 如何使用本书

### 难度说明

| 标记 | 等级 | 适合读者 |
|:------:|-------|----------|
| 🟢 | 入门 | 已熟悉所有权与 trait |
| 🟡 | 中级 | 已熟悉泛型与关联类型 |
| 🔶 | 高级 | 准备学习 type-state、phantom type 与 session type |

### 学习节奏建议

| 目标 | 路径 | 时间 |
|------|------|------|
| **快速概览** | ch01、ch13（速查卡） | 30 分钟 |
| **IPMI / BMC 开发者** | ch02, ch05, ch07, ch10, ch17 | 2.5 小时 |
| **GPU / PCIe 开发者** | ch02, ch06, ch09, ch10, ch15 | 2.5 小时 |
| **Redfish 实现者** | ch02, ch05, ch07, ch08, ch17, ch18 | 3 小时 |
| **框架 / 架构师** | ch04, ch08, ch11, ch14, ch18 | 2.5 小时 |
| **初学“构造即正确”** | 按顺序阅读 ch01 到 ch10，再做 ch12 练习 | 4 小时 |
| **完整深潜** | 顺序阅读全部章节 | 7 小时 |

### 带说明的目录

| 章 | 标题 | 难度 | 核心思想 |
|----|-------|:----------:|----------|
| 1 | 核心理念：为什么类型优于测试 | 🟢 | 正确性的三个层次；类型作为编译器可检查的保证 |
| 2 | 类型化命令接口 | 🟡 | 通过关联类型将请求与响应绑定 |
| 3 | 单次使用类型 | 🟡 | 用移动语义为密码学场景提供线性类型保证 |
| 4 | 能力令牌 | 🟡 | 零大小的权限证明令牌 |
| 5 | 协议状态机 | 🔶 | 将 type-state 用于 IPMI 会话与 PCIe LTSSM |
| 6 | 量纲分析 | 🟢 | 用 newtype 包装器防止单位混淆 |
| 7 | 已验证边界 | 🟡 | 在边界处一次性解析，并通过类型携带证明 |
| 8 | 能力混入 | 🟡 | 组件化 trait 与 blanket impl |
| 9 | Phantom Type | 🟡 | 用 PhantomData 表示寄存器宽度、DMA 方向等信息 |
| 10 | 综合实战 | 🟡 | 在一个诊断平台中组合全部 7 类模式 |
| 11 | 来自一线的十四个技巧 | 🟡 | 哨兵值转 Option、sealed trait、builder 等技巧 |
| 12 | 练习 | 🟡 | 六个带答案的综合题 |
| 13 | 速查卡 | - | 模式目录与决策流程图 |
| 14 | 测试类型层保证 | 🟡 | trybuild, proptest, cargo-show-asm |
| 15 | `const fn` | 🔶 | 为内存映射、寄存器、位字段提供编译期证明 |
| 16 | `Send` 与 `Sync` | 🔶 | 编译期并发正确性证明 |
| 17 | Redfish 客户端实战讲解 | 🟡 | 将八种模式组合成类型安全的 Redfish 客户端 |
| 18 | Redfish 服务器实战讲解 | 🟡 | builder type-state、源令牌、健康汇总与 mixin |

## 前置知识

| 概念 | 建议学习位置 |
|---------|-------------------|
| 所有权与借用 | [Rust Patterns](../rust-patterns-book/src/SUMMARY.md), ch01 |
| Trait 与关联类型 | [Rust Patterns](../rust-patterns-book/src/SUMMARY.md), ch02 |
| Newtype 与 type-state | [Rust Patterns](../rust-patterns-book/src/SUMMARY.md), ch03 |
| PhantomData | [Rust Patterns](../rust-patterns-book/src/SUMMARY.md), ch04 |
| 泛型与 trait 约束 | [Rust Patterns](../rust-patterns-book/src/SUMMARY.md), ch01 |

## “构造即正确”光谱

```text
不够安全                                                       更安全

运行时检查           单元测试            属性测试           构造即正确
----------------    ----------        --------------      -----------------------

if temp > 100 {     #[test]           proptest! {         struct Celsius(f64);
  panic!("too       fn test_temp() {    |t in 0..200| {   // 类型层面上不会与 Rpm 混淆
  hot");              assert!(          assert!(...)      
}                     check(42));     }
                    }                 }

错误程序？          错误程序？        错误程序？          错误程序？
生产环境崩溃        CI 失败           CI 失败             无法编译
                                      (概率性的)          根本不存在
```

本书聚焦于最右侧的那一端：bug 之所以不存在，不是因为它们被测试捕获，而是因为类型系统 **根本无法表达这些错误程序**。

***
