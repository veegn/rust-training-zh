[English Original](../en/ch16-send-sync-compile-time-concurrency-proofs.md)

# `Send` 与 `Sync`：编译期并发证明 🔶

> **你将学到什么？** 如何利用 Rust 的 `Send` 和 `Sync` Trait 在编译期证明并发操作的安全性；如何利用类型系统强制执行对共享资源（如内存映射 IO、共享内存、PCIe 配置空间或跨线程访问的硬件上下文）的并发策略；以及为什么这相比于运行时出现的竞态条件（Race Condition），能大幅减少系统和驱动程序的 Debug 成本。

## 引言：脆弱的并发代码

在系统编程和底层的内核驱动中，资源往往是跨线程共享的。

考虑一个典型的、脆弱的共享资源交互：

```rust
struct HardwareContext {
    ptr: *mut u32,
}

fn thread_worker(ctx: &HardwareContext) {
    // 危险：如果多个线程同时写入 ptr 会发生什么？
    // 危险：如果该硬件不支持多线程访问怎么办？
}
```

这种方案的问题在于：你依然依赖程序运行到并发生竞态才报错。

## Rust 方案：具有语义的并发 Trait 证明

我们可以利用 `Send` 和 `Sync` 来标明一个特定类型是否可以被线程转移或共享。

### 1. 默认的不安全标记

原始指针类型 (`*mut T`) 在默认情况下是既不是 `Send` 也不是 `Sync` 的。

```rust
struct HardwareContext {
    ptr: *mut u32,
} // 默认不支持线程共享
```

### 2. 手动实现并发证明

只有在确认硬件对寄存器访问是线程安全的情况下：

```rust
unsafe impl Send for HardwareContext {}
unsafe impl Sync for HardwareContext {}
```

### 3. 在泛型函数中添加并发约束

```rust
fn spawn_task<T: Send>(val: T) {
    // 编译器会确保 val 可以安全地跨线程转移
}
```

如果你尝试传一个不支持 `Send` 的硬件上下文，编译器会直接报错。

## 现实应用：嵌入式外设与 PCIe 内存共享

在处理带有不同特性的 GPIO 或控制器时，通过 `Sync` 声明其是否支持并发访问 (`SyncShareable`)。
- 当你尝试在一个不支持并发访问的设备上启动多线程处理时，编译器会指出你的架构是不合法的。

## 为什么这种模式至关重要

1.  **排除竞态条件 (No Race Condition)**：违反并发策略的操作在编译期就被排除了。
2.  **强制遵守并发规格 (Enforced Concurrency Policy)**：通过代类型系统描述出硬件架构对并发的支持能力。
3.  **零运行时开销 (Zero-Cost)**：并发证明在由 `rustc` 生成二进制文件时已经完成，程序不再包含任何多余的并发校验逻辑。

在为具有多种访问级别或不同硬件规格的复杂 SoC 设计统一的驱动程序框架时，这些并发证明可以有效防御缓冲区溢出和逻辑漏洞。

***
