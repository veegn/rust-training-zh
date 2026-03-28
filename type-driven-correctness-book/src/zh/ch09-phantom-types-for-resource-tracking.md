[English Original](../en/ch09-phantom-types-for-resource-tracking.md)

# 用 Phantom Type 追踪资源 🟡

> **你将学到什么？** 如何利用 Rust 的 `PhantomData` 在类型层面上存储元数据（如寄存器宽度、DMA 方向、IO 总线或内核模式）；如何利用这些隐式标记来防止在 `u8` 寄存器上执行 `u32` 访问；以及为什么这对于处理多种硬件规格（如 PCIe 寄存器，或者处理不同型号的内核页表）至关重要。

## 引言：通用的、脆弱的指针

在硬件驱动和低级系统编程中，大量操作都是通过裸指针 (Raw Pointer) 进行的。

考虑一个典型的、脆弱的通用寄存器操作模型：

```rust
struct Register {
    ptr: *mut u64,
}

fn write(reg: &Register, value: u64) {
    if reg.is_8bit() && value > 0xFF {
        panic!("值溢出！");
    }
    // 逻辑
}
```

这种方案的问题在于：你的寄存器操作在所有设备上都是一样的，你依然依赖程序运行到那一行逻辑才报错。

## Rust 方案：带有标记的 PhantomData

我们可以利用泛型和 `PhantomData` 在不增加运行时开销的情况下，把属性标注在类型上。

### 1. 定义标记类型

```rust
pub struct Width8;
pub struct Width32;
```

### 2. 泛型寄存器与标记

```rust
use std::marker::PhantomData;

pub struct Register<W, Direction> {
    ptr: *mut u64,
    _width: PhantomData<W>,
    _direction: PhantomData<Direction>,
}
```

### 3. 实现特定宽度的写入

为 `Width8` 实现写入：

```rust
impl<D> Register<Width8, D> {
    pub fn write_byte(&self, val: u8) {
        // 实际写入
    }
}
```

现在，如果你尝试在一个 `Register<Width8, ReadOnly>` 上调用写入，编译器会直接报错。

## 现实应用：PCIe 寄存器与 DMA 管理

在许多 PCIe 驱动中，同一个指针可以映射到不同的偏移地址，具备不同的读写规则 (`ReadOnly`, `WriteOnly`, `ReadWrite`)。
- 当你尝试对一个标记为 `ReadOnly` 的 DMA 描述符执行写入时，编译器会指出你的选择是错误的。

## 为什么这种模式至关重要

1.  **排除无效操作 (No Invalid Access)**：在类型层面直接表达读写属性和宽度属性。
2.  **零成本抽象 (Zero-Cost)**：`PhantomData` 没有任何运行时占用空间。
3.  **强制遵守硬件规范 (Enforced Policy)**：所有的外部操作在编译期都被限制在预定义的路径内。

在处理包含多种寄存器类型或不同总线协议的复杂驱动程序时，这种模式可以有效防御缓冲区溢出和逻辑漏洞。

***
