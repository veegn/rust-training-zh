[English Original](../en/ch15-no_std-rust-without-the-standard-library.md)

# 15. `no_std` Rust 🟢

默认情况下，Rust 程序会链接到标准库 (`std`)，它提供了像 `Vec`、`String` 和网络等高级抽象。然而，在嵌入式系统或 OS 内核中，你可能没有操作系统或堆分配器。Rust 允许你使用 `#![no_std]` 属性来停用标准库。

### 1. `#![no_std]` 属性
当你使用 `#![no_std]` 时，你的程序将仅链接到 `core` 库，它包含了 Rust 中不依赖于 OS 的最基础部分。

```rust
#![no_std]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
```

---

### 2. `core` vs `alloc` vs `std`
- **`core`**：始终可用。包含 `Option`、`Result` 和基础数学运算等基础功能。没有堆分配。
- **`alloc`**：如果你有堆分配器，则可用。包含 `Vec`、`String`、`Box` 等。
- **`std`**：包含 `core` 和 `alloc`，以及像文件 I/O、网络和线程等 OS 特有的功能。

---

### 3. 实现一个 Panic 处理器 (Panic Handler)
在 `#![no_std]` 环境中，你必须定义当程序发生 Panic 时会发生什么，因为没有 OS 可以打印错误消息并终止进程。

```rust
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    // 对于嵌入式，你可能会在这里闪烁 LED 或重置 CPU
    loop {}
}
```

---

### 4. 嵌入式 Rust 生态系统
Rust 拥有一个充满活力的嵌入式开发生态系统：
- **`embedded-hal`**：一套用于硬件抽象的 Trait。
- **`cortex-m`**：对 ARM Cortex-M 微控制器的支持。
- **`riscv`**：对 RISC-V 微控制器的支持。

---

### 对 C/C++ 开发者的总结
- **在 C/C++ 中**：你通常通过不链接标准 C 库 (`libc`) 在“独立 (freestanding)”环境中工作。这类似于 Rust 中的 `#![no_std]`。
- **在 Rust 中**：在没有 `std` 的环境下工作是一项一流特性。许多库 (Crates) 被设计为“no_std”兼容，这意味着它们可以在托管环境和裸机环境下同时使用。这使得在嵌入式固件和桌面工具之间共享代码变得更加容易。

***
