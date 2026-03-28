[English Original](../en/ch04-capability-tokens-zero-cost-proof-of-aut.md)

# 能力令牌：零成本权限证明 🟡

> **你将学到什么？** 如何利用 Rust 的所有权和可见性来模拟操作系统级的能力（Capability）；如何实现编译期的权限检查，而不需要任何运行时开销；以及如何用这个模式来保护对原始硬件寄存器、内存页或敏感 Redfish 操作的直接写入。

## 引言：隐形的不变量

考虑一个读写硬件寄存器的函数：

```rust
fn write_to_register(offset: u32, value: u8) {
    // 危险：如果尚未获得访问权限怎么办？
    // 危险：如果尚未进行解锁 (Unlock) 怎么办？
}
```

传统的防御性编程是在函数开头检查运行时权限。然而，这会在每个频繁调用的写入操作中产生微小的开销。

## 能力令牌解决方案

我们可以在编译期证明，如果一个函数被调用，那么它 **一定** 已经获得了所需的权限。

### 1. 定义私有令牌

```rust
pub struct RegisterAccessPrivilege { _internal: () }
```

确保这个结构体是不可 `Clone` 和 `Copy` 的，且你只能通过特定的 `authorize()` 函数获取它。

### 2. 获取权限的代码路径

```rust
pub fn authorize(creds: Credentials) -> Option<RegisterAccessPrivilege> {
    if creds.is_admin() {
        Some(RegisterAccessPrivilege { _internal: () })
    } else {
        None
    }
}
```

### 3. 需要令牌作为“入场券”

将函数签名更改为：

```rust
fn write_to_register(token: &RegisterAccessPrivilege, offset: u32, value: u8) {
    // 只有持有令牌的调用者才能进入
}
```

## 为什么这种模式至关重要

1.  **零成本权限证明 (Zero-Cost)**：`RegisterAccessPrivilege` 是一个 ZST（零大小类型）。它在生成的二进制文件中 **不存在**，不占用任何内存空间，也没有运行时检查開销；编译器只在编译期验证它的存在。
2.  **受控的导出 (Visibility Control)**：通过 Rust 的模块系统，你可以控制谁能获取这个令牌。
3.  **防止权限提升 (Privilege Escalation)**：所有的权限分发路径在源代码中都是明确且受控的。

在嵌入式编程中，这种模式通常与 DMA (直接存储器存取) 令牌结合使用。如果程序没有持有某个内存区域的“所有权令牌”，它就无法启动指向该区域的 DMA 传输。

***
