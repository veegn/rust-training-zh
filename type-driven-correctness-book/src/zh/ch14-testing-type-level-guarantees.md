[English Original](../en/ch14-testing-type-level-guarantees.md)

# 测试类型层保证 🟡

> **你将学到什么？** 如何利用 Rust 的专门工具（如 `trybuild`）测试不应该通过编译的代码；如何结合属性测试（Proptest）验证类型层面的边界；如何通过查看汇编代码确认抽象的零成本属性；以及为什么这对于复杂类型级系统及其正确性证明至关重要。

## 引言：脆弱的测试体系

在处理复杂的类型系统时，一个常见的问题是：如何证明“本应报错的错误确实报错了”？

考虑一个典型的、脆弱的测试模式：

```rust
#[test]
fn test_invalid_state() {
    // 运行时检查：如果状态非法，预期会发生 Panic
    let res = std::panic::catch_unwind(|| {
        do_invalid_action();
    });
    assert!(res.is_err());
}
```

这种方案的问题在于：你依然依赖运行时行为，且无法利用编译器的静态分析。

## Rust 方案：编译失败测试与 Proptest

我们可以利用 Rust 专门的测试工具来测试“无法编译”。

### 1. 使用 trybuild 测试编译失败

```rust
#[test]
fn test_compile_fail() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/*.rs");
}
```

在 `tests/ui` 下的 `.rs` 文件中，你可以编写违反协议的代码。如果代码 **成功编译**，测试就会 **失败**。这是证明你的类型系统约束真实生效的最佳方法。

### 2. 利用 Proptest 在边界处进行自动化验证

```rust
proptest! {
    #[test]
    fn test_validated_port(p in 0u16..65535) {
        let res = Port::try_from(p);
        if p == 0 {
             prop_assert!(res.is_err());
        } else {
             prop_assert!(res.is_ok());
        }
    }
}
```

## 现实应用：驱动开发与协议库测试

在许多工业级驱动或协议库（如 PCIe、redfish 等）中，通过 `trybuild` 证明：
- 用户不能在未持有权限令牌的情况下写入寄存器；
- 用户不能在连接尚未建立时发送数据报文。

## 为什么这种测试至关重要

1.  **排除编译器回归 (No Compiler Regression)**：确保后续重构没有无意中放宽了类型约束。
2.  **强制遵守架构 (Enforced Architecture)**：通过测试驱动开发，锁定所有的非法访问路径。
3.  **零成本抽象验证 (High Performance Validation)**：查看生成的目标代码汇编，确保所有的 Newtype 和标记类型在优化后没有任何性能损耗。

在为具有多种访问级别或不同硬件规格的复杂系统编写统一的维护框架时，这些专门的测试内容可以提供极高的信心。

***
