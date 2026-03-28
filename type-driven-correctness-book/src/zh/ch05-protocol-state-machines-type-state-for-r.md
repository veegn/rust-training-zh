# 协议状态机 — 真实硬件中的 Type-State 🔴

> **你将学到：** Type-state 编码如何将协议违规（错误顺序的命令、close 后使用）转变为编译错误，并应用于 IPMI 会话生命周期和 PCIe 链路训练。
>
> **相关章节：** [ch01](ch01-the-philosophy-why-types-beat-tests.md)（第 2 层 —— 状态正确性）、[ch04](ch04-capability-tokens-zero-cost-proof-of-aut.md)（令牌）、[ch09](ch09-phantom-types-for-resource-tracking.md) (phantom type)、[ch11](ch11-fourteen-tricks-from-the-trenches.md)（技巧 4 — typestate builder，技巧 8 — async type-state）

## 问题：协议违规

硬件协议（如 IPMI 或 PCIe 链路训练）拥有严格的状态机。在错误的状态下发送命令（例如在认证前发送数据）会导致会话损坏或总线挂起。

## Type-State 模式

在 Rust 中，我们将每个协议状态表示为一个 **独立的类型**。状态转换是消耗一个状态并返回另一个状态的方法。这使得在错误的状态下调用方法变得不可能，因为在该类型上根本不存在该方法。

```rust
pub struct IpmiSession<State> {
    _state: PhantomData<State>,
}

impl IpmiSession<Idle> {
    pub fn authenticate(self) -> Result<IpmiSession<Authenticated>, Error> { ... }
}

impl IpmiSession<Active> {
    pub fn send_command(&mut self) { ... }
    pub fn close(self) -> IpmiSession<Closed> { ... }
}
```

编译器强制执行：
- 激活前必须进行认证。
- 发送命令前必须激活。
- 关闭后不能再发送命令。

## PCIe 链路训练 (LTSSM)

链路训练会经历 `Detect → Polling → Configuration → L0` 等阶段。Type-state 可以确保 `send_tlp()` 仅在 `L0` 状态下可用。

```rust
impl PcieLink<L0> {
    pub fn send_tlp(&mut self, tlp: &[u8]) { ... }
}
```

## 组合：状态 + 能力

你可以将 type-state 与能力令牌（见 ch04）结合使用，以要求同时满足“活动会话”和“管理员权限”：

```rust
pub fn firmware_update(
    session: &mut IpmiSession<Active>,
    _admin: &AdminToken,
    image: &[u8],
) { ... }
```

## 何时使用 Type-State

| 协议 | 是否值得？ |
|----------|:----:|
| IPMI/USB/TLS 握手 | ✅ 是 |
| PCIe LTSSM | ✅ 是 |
| 固件更新生命周期 | ✅ 是 |
| 简单的 2 状态请求/响应 | ⚠️ 可能不值得 |

## 关键收获

1. **使错误顺序的调用变得不可能** —— 方法仅在合法的状态下存在。
2. **转换消耗 `self`** —— 防止使用过时的旧状态。
3. **可组合性** —— 可以同时强制执行状态和特权。
4. **可扩展性** —— 适用于简单的会话，也适用于复杂的固件生命周期。

***
