# 能力令牌 — 零成本权限证明 🟡

> **你将学到：** 零大小类型 (Zero-Sized Types, ZSTs) 如何作为编译期的证明令牌，用以强制执行特权层次结构、上电时序以及撤销授权 —— 且这一切都是零 runtime 成本的。
>
> **相关章节：** [ch03](ch03-single-use-types-cryptographic-guarantee.md)（单次使用类型）、[ch05](ch05-protocol-state-machines-type-state-for-r.md) (type-state)、[ch08](ch08-capability-mixins-compile-time-hardware-.md) (mixin)、[ch10](ch10-putting-it-all-together-a-complete-diagn.md)（集成）

## 问题：危险的操作

在硬件诊断中，某些操作（固件烧录、复位、高电压模式）是危险的。使用运行时检查来防御它们会导致大量冗余代码，并可能存在特权提升的 bug。

## 零大小类型即证明令牌

**能力令牌 (Capability Token)** 是一种证明权限的零大小类型 (ZST)。它在运行时的开销是 **零字节**。

```rust
pub struct AdminToken {
    _private: (),   // 仅限本模块构造
}

impl BmcController {
    pub fn authenticate_admin(&mut self) -> Result<AdminToken, Error> {
        // ... 验证过程 ...
        Ok(AdminToken { _private: () })
    }

    pub fn reset_pcie_link(
        &mut self,
        _admin: &AdminToken, // 零成本证明
        slot: u32,
    ) -> Result<(), Error> { ... }
}
```

`AdminToken` 是一种 **证明义务 (proof obligation)** —— 编译器会确认你在执行复位之前已经通过 `authenticate_admin()` 获得了一个令牌。

## 特权层次结构

使用 trait 层级来模拟“Admin 可以执行 Operator 能做的一切”：

```rust
pub trait Authenticated {}
pub trait Operator: Authenticated {}
pub trait Admin: Operator {}

pub struct AdminToken;
impl Authenticated for AdminToken {}
impl Operator for AdminToken {}
impl Admin for AdminToken {}

pub fn run_diag(_who: &impl Operator) { ... }
pub fn flash_fw(_who: &impl Admin) { ... }
```

## 有作用域的能力

通过生命周期绑定的令牌，可以确保特权不会超出当前会话的范围：

```rust
pub struct ScopedAdminToken<'session> {
    _session: &'session AdminSession,
}
```

## 成本与优势

| 功能 | 成本 | 优势 |
|---------|:----:|---------|
| ZST 令牌 | 0 字节 | 编译期证明 |
| 层级结构 | 0 成本 | 权限继承 |
| 生命周期 | 0 成本 | 自动失效 |

**总 Runtime 开销：零。**

## 关键收获

1. **ZST 令牌开销为零字节** —— 它们仅作为编译期的证明对象存在。
2. **私有构造函数 = 不可伪造** —— 令牌仅能由授权逻辑签发。
3. **Trait 层级模拟 RBAC** —— 实现简洁、可继承的权限集合。
4. **生命周期绑定 = 可撤销** —— 特权会在会话结束时自动过期。

***
