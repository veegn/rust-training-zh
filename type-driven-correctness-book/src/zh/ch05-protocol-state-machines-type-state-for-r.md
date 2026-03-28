[English Original](../en/ch05-protocol-state-machines-type-state-for-r.md)

# 协议状态机：面向真实硬件的 Type-State 🔶

> **你将学到什么？** 如何利用 Rust 的 Type-state 模式来表现复杂的硬件和通信协议（如 PCIe 的链接转换状态机 LTSSM、I2C/SPI 控制器的握手或 Redfish 交互）；如何防止调用在当前状态下无效的函数（例如在未完成三次握手时发送数据数据）；以及为什么这能大幅减少驱动程序中的 Bug。

## 引言：脆弱的状态机

在系统编程中，状态管理往往依赖于大量的 `if (state == READY)` 或 `switch (status)`。

考虑一个典型的、脆弱的状态驱动代码：

```rust
struct I2CDriver {
    state: State, // Disconnected, Idle, Transmitting, Error
}

fn send_byte(driver: &mut I2CDriver, data: u8) {
    if driver.state != State::Idle {
        panic!("状态错误！");
    }
    // 逻辑
}
```

这种方案的问题在于：你依然依赖程序运行到那一行逻辑才报错，且 `driver` 的类型在所有状态中都是一样的。

## Rust 方案：Type-state 协议描述

我们可以利用泛型和所有权来物理性地改变结构体的状态。

### 1. 定义状态类型

```rust
pub struct Disconnected;
pub struct Idle;
pub struct Transmitting;
```

### 2. 泛型驱动程序结构

```rust
pub struct I2CDriver<S> {
    _state: S,
    // 硬件寄存器映射等
}
```

### 3. 实现特定状态的功能

定义一个只能在 `Idle` 状态下调用的函数：

```rust
impl I2CDriver<Idle> {
    pub fn start_transaction(self, addr: u8) -> I2CDriver<Transmitting> {
        // 实际操作硬件寄存器进入传输状态
        I2CDriver { _state: Transmitting }
    }
}
```

### 4. 无法编译的调用

如果在此处尝试在 `Disconnected` 状态下直接 `start_transaction`，编译器会报错：
- `no method named start_transaction found for struct I2CDriver<Disconnected>`

## 现实应用：PCIe 链路状态管理 (LTSSM)

PCIe 的状态转换逻辑极其精细（Detect -> Polling -> Configuration -> L0）。通过 Type-state，你可以证明驱动程序中的每个寄存器写入都对应于当前合法的状态转换。

## 为什么这种模式至关重要

1.  **排除无效调用 (No Invalid Call)**：任何不符合规范的状态转换在编译期就被彻底排除了。
2.  **强制遵守协议 (Enforced Protocol Rules)**：通过类型映射出协议的拓扑结构。
3.  **自文档化 (Self-documenting API)**：函数的签名本身就描述了它的前提条件和结果状态。

在处理包含中断或异步事件的复杂驱动程序时，这种模式配合 ZST (零大小类型) 可以实现完全零成本的安全防护。

***
