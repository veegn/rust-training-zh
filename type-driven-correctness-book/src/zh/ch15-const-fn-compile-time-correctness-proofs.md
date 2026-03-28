[English Original](../en/ch15-const-fn-compile-time-correctness-proofs.md)

# `const fn`：编译期正确性证明 🔶

> **你将学到什么？** 如何利用 Rust 的 `const fn` 在编译阶段执行复杂的验证逻辑；如何定义可以在硬编码时就进行校验的硬件寄存器偏移、位掩码或内存映射（Memory-mapped IO）；以及为什么这相比于传统的运行时检查，能彻底防止在固件或驱动开发中出现初始化错误。

## 引言：脆弱的运行时配置

在底层的固件和核心驱动程序开发中，配置（如寄存器基址或偏移）往往在系统启动时通过硬编码值确定。

考虑一个典型的、脆弱的内存映射寄存器模型：

```rust
const REG_OFFSET: u32 = 0x1234;

fn init_register() {
    if REG_OFFSET % 4 != 0 {
        panic!("对齐错误！");
    }
}
```

这种方案的问题在于：你的程序在启动并运行到那一行逻辑之后才会挂掉。

## Rust 方案：编译期验证逻辑

我们可以利用 `const fn` 将运行时检查提升到编译过程中。

### 1. 定义 const 验证函数

```rust
const fn validate_alignment(offset: u32) -> u32 {
    if offset % 4 != 0 {
         panic!("寄存器偏移必须按 4 字节对齐！");
    }
    offset
}
```

### 2. 在常量定义时进行链式验证

```rust
const VALID_REG_OFFSET: u32 = validate_alignment(0x1234);
```

如果你尝试传入一个无效的值（如 `0x1235`），编译器会直接报错：
- `error: any use of this value will cause an error`

## 现实应用：嵌入式外设与 PCIe 内存映射

在处理带有不同特性的 GPIO 或控制器时，通过 `const fn` 检查：
- 全局基址是否在该特定型号硬件的合法范围内；
- 位掩码 (`Bitmask`) 是否与硬件文档中定义的位宽兼容。

## 为什么这种模式至关重要

1.  **排除无效配置 (No Invalid Config)**：硬件不支持的偏移或非法的对齐在编译期就被排除了。
2.  **强制遵守硬件规格 (Enforced Specs)**：通过代码将硬件文档中的物理限制转化为编译期的防火墙。
3.  **零运行时开销 (Zero-Cost)**：所有的验证逻辑在由 `rustc` 生成二进制文件时已经完成，程序不再包含任何配置校验开销。

在为具有多种访问级别或不同硬件规格的复杂 SoC 设计统一的驱动程序框架时，这种模式可以显著提升代码的鲁棒性。

***
