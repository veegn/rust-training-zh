[English Original](../en/ch09-no-std-and-feature-verification.md)

# `no_std` 与特性验证 🔴

> **你将学到：**
> - 使用 `cargo-hack` 系统地验证特性 (features) 组合
> - Rust 的三个层次：`core` vs `alloc` vs `std` 及其适用场景
> - 构建包含自定义 panic handler 和分配器 (allocator) 的 `no_std` crate
> - 在宿主机和 QEMU 上测试 `no_std` 代码
>
> **相关章节：** [Windows 与条件编译](ch10-windows-and-conditional-compilation.md) — 该主题的平台篇 · [交叉编译](ch02-cross-compilation-one-source-many-target.md) — 交叉编译到 ARM 和嵌入式目标 · [Miri 与 Sanitizer](ch05-miri-valgrind-and-sanitizers-verifying-u.md) — 验证 `no_std` 环境中的 `unsafe` 代码 · [构建脚本](ch01-build-scripts-buildrs-in-depth.md) — 由 `build.rs` 发出的 `cfg` 标志

Rust 可以运行在从 8 位单片机到云端服务器的任何地方。本章涵盖了这一切的基础：通过 `#![no_std]` 剥离标准库，并验证你的特性组合是否能够正确编译。

### 使用 `cargo-hack` 验证特性组合

[`cargo-hack`](https://github.com/taiki-e/cargo-hack) 会系统地测试所有的特性组合 —— 对于包含大量 `#[cfg(...)]` 代码的 crate 来说至关重要：

```bash
# 安装并运行
cargo install cargo-hack
cargo hack check --each-feature --workspace
```

### `no_std` — 何时以及为何使用

`#![no_std]` 告诉编译器：“不要链接标准库”。你的 crate 只能使用 `core`（以及可选的 `alloc`）。

| 常见场景 | 使用理由 |
|----------|-------------|
| 嵌入式固件 (ARM Cortex-M, RISC-V) | 没有操作系统、没有堆、没有文件系统 |
| UEFI 诊断工具 | 预启动环境，无操作系统 API |
| 内核模块 | 内核空间无法使用用户态的 `std` |
| WebAssembly (WASM) | 极小化二进制体积，无 OS 依赖 |

### `core` vs `alloc` vs `std` — 三个层次

1. **`core`**：始终可用。包含原始类型、`Option`、`Result`、常用 trait 和原子操作。
2. **`alloc`**：如果有分配器则可用。包含 `Vec`、`String`、`Box`、`BTreeMap`。
3. **`std`**：全量标准库。包含文件 I/O、网络、线程、时间。

### 构建 `no_std` 库

```rust
// src/lib.rs
#![no_std]

pub struct Temperature {
    raw: u16,
}

impl Temperature {
    pub const fn from_raw(raw: u16) -> Self {
        Self { raw }
    }

    pub const fn millidegrees_c(&self) -> i32 {
        (self.raw as i32) * 625 / 10
    }
}
```

### 测试 `no_std` 代码

开发过程中的单元测试通常运行在宿主机上，宿主机是有 `std` 的。诀窍在于：你的库是 `no_std` 的，但你的测试钩子会自动使用 `std`。

```bash
cargo test --lib
```

### 🏋️ 练习

#### 🟡 练习 1：特性组合验证
安装 `cargo-hack` 并运行 `cargo hack check --each-feature --workspace`。

#### 🔴 练习 2：构建 `no_std` 库
创建一个能够使用 `#![no_std]` 编译的库。实现一个简单的基于栈分配的环形缓冲区。验证它能在 `thumbv7em-none-eabihf` (ARM Cortex-M) 目标下编译通过。

### 关键收获
- 对于带有交互特性的 crate，在 CI 中运行 `cargo-hack --each-feature` 是必须的。
- `core` → `alloc` → `std` 是层层递进的关系。
- 在裸机 `no_std` 二进制文件中，必须提供自定义的 panic handler。
- `no_std` 库的测试可以直接在宿主机上进行。

***
