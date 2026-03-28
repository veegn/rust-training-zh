# Capability Tokens â€?Zero-Cost Proof of Authority ðŸŸ¡

> **What you'll learn:** How zero-sized types (ZSTs) act as compile-time proof tokens, enforcing privilege hierarchies, power sequencing, and revocable authority â€?all at zero runtime cost.
>
> **Cross-references:** [ch03](ch03-single-use-types-cryptographic-guarantee.md) (single-use types), [ch05](ch05-protocol-state-machines-type-state-for-r.md) (type-state), [ch08](ch08-capability-mixins-compile-time-hardware-.md) (mixins), [ch10](ch10-putting-it-all-together-a-complete-diagn.md) (integration)

## The Problem: Dangerous Operations

In hardware diagnostics, certain operations (firmware flashing, resets, high-voltage modes) are dangerous. Guarding them with runtime checks results in repetitive code and potential privilege escalation bugs.

## Zero-Sized Types as Proof Tokens

A **capability token** is a zero-sized type (ZST) that proves authority. It costs **zero bytes** at runtime.

```rust
pub struct AdminToken {
    _private: (),   // construction restricted to this module
}

impl BmcController {
    pub fn authenticate_admin(&mut self) -> Result<AdminToken, Error> {
        // ... validate ...
        Ok(AdminToken { _private: () })
    }

    pub fn reset_pcie_link(
        &mut self,
        _admin: &AdminToken, // zero-cost proof
        slot: u32,
    ) -> Result<(), Error> { ... }
}
```

The `AdminToken` is a **proof obligation**. The compiler confirms that a token was obtained via `authenticate_admin()` before allowing the reset.

## Hierarchical Capabilities

Use trait hierarchies to model "Admin can do everything an Operator can":

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

## Scoped Capabilities

Lifetime-bounded tokens ensure privileges cannot outlive the session:

```rust
pub struct ScopedAdminToken<'session> {
    _session: &'session AdminSession,
}
```

## Cost and Benefits

| Feature | Cost | Benefit |
|---------|:----:|---------|
| ZST Tokens | 0 bytes | Compile-time proof |
| Hierarchy | 0 cost | Inherited permissions |
| Lifetimes | 0 cost | Automatic expiration |

**Total runtime overhead: zero.**

## Key Takeaways

1. **ZST tokens cost zero bytes** â€?they are compile-time-only proof objects.
2. **Private constructors = unforgeable** â€?tokens can only be minted by authorized logic.
3. **Trait hierarchies model RBAC** â€?clean, inherited permission sets.
4. **Lifetime bounds = revocable** â€?privileges expire automatically when the session ends.

***

