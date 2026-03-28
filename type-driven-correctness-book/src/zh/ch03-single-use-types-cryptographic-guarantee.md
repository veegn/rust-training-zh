# 单次使用类型 — 通过所有权实现的密码学保证 🟡

> **你将学到：** Rust 的移动语义 (move semantics) 如何充当线性类型系统 (linear type system)，在编译期使 Nonce 重用、重复密钥协商以及意外的熔丝 (fuse) 二次编程变得不可能。
>
> **相关章节：** [ch01](ch01-the-philosophy-why-types-beat-tests.md)（理念）、[ch04](ch04-capability-tokens-zero-cost-proof-of-aut.md)（能力令牌）、[ch05](ch05-protocol-state-machines-type-state-for-r.md) (type-state)、[ch14](ch14-testing-type-level-guarantees.md) (编译失败测试)

## Nonce 重用的灾难

在认证加密 (AES-GCM, ChaCha20-Poly1305) 中，对同一个密钥重用 Nonce 是 **灾难性的** —— 这会泄露明文的 XOR 结果，通常还会泄露认证密钥本身。在 C 语言中，Nonce 仅仅是一个 `uint8_t[12]`；没有什么能阻止你使用它两次。

## 移动语义即线性类型

Rust 的所有权系统实际上就是一个 **线性类型系统 (linear type system)** —— 除非实现了 `Copy`，否则一个值只能被使用一次（即发生移动）。

```rust
pub struct Nonce(/* 私有字段 */);

impl Nonce {
    // 无 Clone, 无 Copy — 只能使用一次
}

fn seal_in_place(
    key: &SealingKey,
    nonce: Nonce,       // ← 发生移动，而非借用
    data: &mut Vec<u8>,
) { ... }
```

尝试重用 `Nonce` 会导致 **编译错误**：

```rust
let nonce = Nonce::new();
seal_in_place(key, nonce, data1); // ✅ nonce 移动到此
seal_in_place(key, nonce, data2); // ❌ 编译报错：使用了已移动的值
```

## 硬件应用：一次性熔丝编程 (OTP)

OTP (one-time programmable) 熔丝的写入是不可逆的。移动语义可以防止意外的二次写入：

```rust
pub struct FusePayload { ... }

impl FuseController {
    pub fn program(&mut self, payload: FusePayload) -> io::Result<()> {
        // ... 写入 OTP 硬件 ...
        // payload 在此被消耗掉
        Ok(())
    }
}
```

## 何时使用单次使用类型

| 场景 | 是否使用移动语义？ |
|----------|:------:|
| 密码学 Nonce | ✅ 始终（重用是灾难性的） |
| 临时密钥 (DH) | ✅ 始终 |
| OTP 熔丝写入 | ✅ 始终（二次写入会导致硬件损坏） |
| 校准令牌 | ✅ 通常（确保每次会话仅校准一次） |
| 普通数据缓冲区 | ❌ 否（需要重用） |

## 关键收获

1. **Move = 线性化使用** — 非 Clone/Copy 类型在编译期被保证仅能消耗一次。
2. **结构化预防** — Rust 通过所有权结构来预防 Nonce 重用，而非依赖开发者的纪律。
3. **广泛适用性** — 适用于密码学、熔丝、校准等多种场景。
4. **前向安全性 (Forward Secrecy)** — 临时密钥在使用后即从内存中彻底消失。

***
