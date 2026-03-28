[English Original](../en/ch08-capability-mixins-compile-time-hardware-.md)

# 能力混入：编译期硬件契约 🟡

> **你将学到什么？** 如何利用 Rust 的 Trait 和泛型机制实现“能力混入”（Capability Mixins）；如何通过 Trait 声明硬件组件具备的特定功能（如 `SupportsSleep`、`SupportsReset`）；以及为什么这相比于通用的 `GenericDevice` 结构体，能将硬件误操作限制在编译期。

## 引言：通用的、脆弱的驱动

在硬件驱动设计中，一个常见的问题是：为了一致性，通常使用一个通用的 `Device` 结构体来管理所有的硬件。

考虑一个典型的、脆弱的通用驱动：

```rust
struct Device {
    id: u32,
}

fn sleep(device: &Device) {
    if !device.supports_sleep() {
        panic!("不支持睡眠！");
    }
    // 逻辑
}
```

这种方案的问题在于：你的驱动程序在所有硬件上都是一样的，你依然依赖程序运行到那一行逻辑才报错。

## Rust 方案：Trait 驱动的能力声明

我们可以利用 Trait 来标明一个特定硬件型号是否具备某种能力。

### 1. 定义能力 Trait

```rust
pub trait SupportsSleep {}
pub trait SupportsReset {}
```

### 2. 泛型设备与其能力声明

```rust
pub struct Device<C> {
    _capability: C,
}

pub struct SleepOnly;
impl SupportsSleep for SleepOnly {}
```

### 3. 实现功能时增加 Trait 约束

```rust
fn puts_to_sleep<C: SupportsSleep>(device: Device<C>) {
    // 实际操作硬件寄存器进入传输状态
}
```

现在，如果你尝试在一个 `Device<ResetOnly>` 上调用 `puts_to_sleep`，编译器会直接报错。

## 现实应用：嵌入式外设与 I/O 选项

在处理带有不同特性的 GPIO 或控制器时，通过 Trait 声明其是否支持中断 (`HasInterrupt`) 或是否支持 DMA (`SupportsDMA`)。
- 当你尝试把一个不支持中断的引脚传递给中断控制器时，编译器会指出你的选择是错误的。

## 为什么这种模式至关重要

1.  **排除无效操作 (No Unsupported Action)**：硬件不支持的操作在编译期就被排除了。
2.  **强制遵循硬件规格 (Enforced Hardware Specs)**：通过类型系统直接表达硬件文档中的功能列表。
3.  **高性能 (High Performance)**：Trait 约束在编译期完成分发；生成的汇编代码不再包含任何 `if !supports_xxx` 的检查逻辑。

在为具有多种变体但高度相似的 SoC 分支编写统一的驱动程序框架时，这种模式可以有效解决“通用框架导致的脆弱性”问题。

***
