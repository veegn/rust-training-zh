[English Original](../en/ch03-single-use-types-cryptographic-guarantee.md)

# 单次使用类型：通过所有权提供密码学级保证 🟡

> **你将学到什么？** 如何利用 Rust 的移动语义（Move Semantics）实现线性类型（Linear Types）的等效物；如何保证敏感数据（如私钥）在内存中只使用一次后即被销毁；以及为什么这能防止重放攻击和重用过期的会话令牌。

## 引言：状态泄露的风险

在密码学和协议中，“重用”是常见的危险。例如，如果您在两个不同的连接中重用同一个一次性随机数（Nonce），你可能面临会话劫持或加密破解的风险。

传统的 C++ 方法通常使用 `bool is_used` 或 `State enum` 进行运行时检查：

```cpp
class Token {
    bool used = false;
    void consume() { 
        if (used) throw error; 
        used = true; 
    }
};
```

这种方案的问题在于：你依然依赖程序运行到那一行逻辑才报错。

## Rust 方案：单次使用所有权

我们可以利用 Rust 的所有权模型：**消耗（Consume）** 该对象。

### 1. 定义私有令牌

```rust
pub struct SessionToken {
    id: Vec<u8>
}
```

### 2. 消耗性操作

定义一个接受 `self` 而不是 `&self` 的函数：

```rust
pub fn authenticate(token: SessionToken) -> Result<SecureSession, Error> {
    // 逻辑
} 
```

一旦 `authenticate` 被调用，`SessionToken` 就会被移动并失效。

### 3. 禁止复制

**千万不要** 为此类结构体实现 `Clone` 或 `Copy`。这是这种模式的核心。

```rust
let token = get_token();
authenticate(token); // 正常

// 如果在此处尝试再次使用 token，编译器会报错：
// authenticate(token); // 错误：使用已移动的值
```

## 现实应用：TLS 握手和随机数 (Nonces)

在实现 TLS 握手的状态机时，可以将握手报文（Handshake Message）封装在只能使用一次的类型中：
- `ClientHello` 令牌一旦转换为 `ServerHello` 或 `EncryptedHandshake`，原有的上下文就在类型层面上彻底消失了。

## 为什么这种模式至关重要

1.  **无法重放 (No Replay)**：任何依赖于已经失效的状态的操作在编译期就被排除了。
2.  **强制清理 (Enforced Cleanup)**：如果一个类型必须被消耗才能产生下一个结果，那么开发者就无法“跳过”某些清理或验证步骤。
3.  **确定性生命周期**：所有敏感数据都在业务需要的最后一刻结束其生命周期。

在使用 `Pin` 和 `Future` 的异步场景中，这种单次使用模式甚至能防止某些并发 Bug。

***
