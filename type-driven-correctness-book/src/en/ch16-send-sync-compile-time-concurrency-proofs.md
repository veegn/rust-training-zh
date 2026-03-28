# Send & Sync — Compile-Time Concurrency Proofs 🟠

> **What you'll learn:** How Rust's `Send` and `Sync` auto-traits turn the compiler into a concurrency auditor — proving at compile time which types can cross thread boundaries and which can be shared, with zero runtime cost.
>
> **Cross-references:** [ch04](ch04-capability-tokens-zero-cost-proof-of-aut.md) (tokens), [ch09](ch09-phantom-types-for-resource-tracking.md) (phantom types), [ch15](ch15-const-fn-compile-time-correctness-proofs.md) (const fn)

## The Problem: Data Races in C

In C, global buffers are often shared between the main loop and Interrupt Service Handlers (ISRs). The `volatile` keyword prevents compiler optimization, but it does **nothing** to prevent data races where two contexts read and write a shared variable (like a buffer index) simultaneously.

## What Send and Sync Prove

Rust uses two "marker traits" to categorize types based on thread safety:

| Trait | Meaning |
|-------|---------|
| `Send` | Safe to **move** to another thread. |
| `Sync` | Safe to **share** (via `&T`) between multiple threads. |

These are **auto-traits**. The compiler derives them by inspecting every field of your struct. If a struct contains a `!Send` field (like `Rc<T>` or a raw pointer `*const T`), the entire struct becomes `!Send`.

## Thread-Confined Hardware Handles

By wrapping a raw pointer in a struct, you make that struct `!Send` and `!Sync` by default. This ensures the hardware handle stays on the thread that created it (e.g., the main loop), preventing accidental access from background threads or ISRs.

```rust
pub struct Uart {
    regs: *const u32, // Raw pointer makes this !Send + !Sync
}

// Attempts to send `Uart` to another thread will fail at compile time.
```

## Creating Safe Shared State

- **`Mutex<T>`**: Transforms a `!Sync` type into a `Sync` type by forcing every access to go through a lock.
- **`Arc<T>`**: Allows shared ownership across threads for `Send + Sync` types.

## Key Takeaways

1. **The Compiler is the Auditor** — `Send` and `Sync` are derived automatically. You can't "forget" to mark a type as thread-unsafe if it contains non-thread-safe fields.
2. **Zero Runtime Cost** — these traits exist only in the type system. They vanish during compilation.
3. **Hardware Isolation** — use `!Send` handles to pin hardware access to a specific core or thread.
4. **Fearless Concurrency** — you get the performance of shared memory with the safety of a formal proof.

***
