# Single-Use Types — Cryptographic Guarantees via Ownership 🟡

> **What you'll learn:** How Rust's move semantics act as a linear type system, making nonce reuse, double key-agreement, and accidental fuse re-programming impossible at compile time.
>
> **Cross-references:** [ch01](ch01-the-philosophy-why-types-beat-tests.md) (philosophy), [ch04](ch04-capability-tokens-zero-cost-proof-of-aut.md) (capability tokens), [ch05](ch05-protocol-state-machines-type-state-for-r.md) (type-state), [ch14](ch14-testing-type-level-guarantees.md) (testing compile-fail)

## The Nonce Reuse Catastrophe

In authenticated encryption (AES-GCM, ChaCha20-Poly1305), reusing a nonce with the same key is **catastrophic**. It leaks information and authentication keys. In C, a nonce is just an array; nothing stops you from using it twice.

## Move Semantics as Linear Types

Rust's ownership system is effectively a **linear type system** — a value can be used exactly once (moved) unless it implements `Copy`.

```rust
pub struct Nonce(/* private */);

impl Nonce {
    // No Clone, no Copy — can only be used once
}

fn seal_in_place(
    key: &SealingKey,
    nonce: Nonce,       // ← moved, not borrowed
    data: &mut Vec<u8>,
) { ... }
```

Attempting to reuse a `Nonce` results in a **compile error**:

```rust
let nonce = Nonce::new();
seal_in_place(key, nonce, data1); // ✅ nonce moved
seal_in_place(key, nonce, data2); // ❌ compile error: use of moved value
```

## Hardware Application: One-Time Fuse Programming

Writing OTP (one-time programmable) fuses is irreversible. Move semantics prevent accidental double-writes:

```rust
pub struct FusePayload { ... }

impl FuseController {
    pub fn program(&mut self, payload: FusePayload) -> io::Result<()> {
        // ... write to OTP hardware ...
        // payload is consumed here
        Ok(())
    }
}
```

## When to Use Single-Use Types

| Scenario | Use single-use semantics? |
|----------|:------:|
| Cryptographic nonces | ✅ Always |
| Ephemeral keys (DH) | ✅ Always |
| OTP fuse writes | ✅ Always |
| Calibration tokens | ✅ Usually |
| General data buffers | ❌ No — need reuse |

## Key Takeaways

1. **Move = linear use** — non-Clone/non-Copy types are consumed exactly once.
2. **Structural prevention** — Rust prevents nonce reuse via ownership, not discipline.
3. **Broad applicability** — works for crypto, fuses, calibration, and more.
4. **Forward secrecy** — ephemeral keys vanish from memory after use.

***
