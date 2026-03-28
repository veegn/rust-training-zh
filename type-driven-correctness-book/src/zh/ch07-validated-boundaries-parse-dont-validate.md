[English Original](../en/ch07-validated-boundaries-parse-dont-validate.md)

# 已验证边界：解析，而不是事后校验 🟡

> **你将学到什么？** 如何利用 Rust 的所有权和类型系统在系统边界处进行强一致性检查；为什么不应在函数内部各处散落 `if (data.is_valid())` 而应采用“解析，而不是后续校验”（Parse, don't validate）的策略；以及如何用这个模式保护来自 PCIe、Redfish 或网络接口（如 IPv4 地址、JSON Payload）的原始数据。

## 引言：脆弱的验证逻辑

在系统编程中，我们经常从外部世界接收原始数据、报文或配置。传统的模式是在函数内部各处进行按需检查。

考虑一个典型的、脆弱的数据处理模型：

```rust
struct NetworkPacket {
    raw_data: Vec<u8>,
}

fn process_packet(packet: &NetworkPacket) {
    if packet.raw_data.len() < 4 {
        panic!("无效报文！");
    }
    // 逻辑
}
```

这种方案的问题在于：你依然依赖程序运行到那一行逻辑才报错，且 `NetworkPacket` 在整个生命周期中都可能处于“非法”状态。

## Rust 方案：Parse, don't validate (解析，而不是持续验证)

我们可以将“验证”操作看作是“从原始类型转到已验证类型”的过程。

### 1. 定义私有数据

```rust
pub struct ValidatedPacket {
    payload: Vec<u8>
}
```

### 2. 通过解析构造类型

```rust
impl ValidatedPacket {
    pub fn try_from_raw(raw: Vec<u8>) -> Result<Self, Error> {
        if raw.len() < 4 {
             return Err(Error::TooSmall);
        }
        // 成功解析即代表“已被永久验证”
        Ok(ValidatedPacket { payload: raw })
    }
}
```

### 3. 在下游函数中只接受“已验证”的类型

```rust
fn handle_packet(packet: ValidatedPacket) {
    // 这里没有任何 if (len < 4) 检查，
    // 因为类型本身就是一种“已被验证”的证明。
}
```

## 现实应用：IPv4 地址与 JSON 负载

在很多底层库或 Redfish 客户端中，如果一个函数接受一个字符串作为 IPv4 地址：
- 不要在函数内部用正则表达式验证它；
- 而是要求传入 `std::net::Ipv4Addr`。
当调用者不得不先完成一次 `Ipv4Addr::parse()` 时，他们就已经在类型层面上完成了验证。

## 为什么这种模式至关重要

1.  **排除非法状态 (No Illegal State)**：一旦一个对象具有 `ValidatedPacket` 类型，它在全局范围内都代表“合法”，不再需要任何多余的后续检查。
2.  **强制遵守规范 (Enforced Policy)**：所有的外部数据在被系统核心消费之前，都必须经过特定的“净化”程序。
3.  **高性能 (High Performance)**：复杂的逻辑校验只在边界处发生一次；在系统内部的所有数据流转都是零开销的。

在处理不可信的第三方数据线或来自内核的原始结构体时，这种模式可以有效防御缓冲区溢出和逻辑漏洞。

***
