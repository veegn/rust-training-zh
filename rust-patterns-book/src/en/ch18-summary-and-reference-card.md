# 18. Quick Reference Card 🟢

### Pattern Selection Guide

- **Type Safety**: Newtype Pattern ([Ch 03](ch03-the-newtype-and-type-state-patterns.md))
- **Compile-time States**: Type-State Pattern ([Ch 03](ch03-the-newtype-and-type-state-patterns.md))
- **Unit Metadata**: PhantomData ([Ch 04](ch04-phantomdata-types-that-carry-no-data.md))
- **Shared State**: Arc + Mutex/RwLock ([Ch 06](ch06-concurrency-vs-parallelism-vs-threads.md))
- **Concurrent Messaging**: MPSC Channels ([Ch 05](ch05-channels-and-message-passing.md))
- **Abstraction**: Traits vs dyn Trait ([Ch 02](ch02-traits-in-depth.md))
- **Low-level Control**: Unsafe Rust & Pin ([Ch 09](ch09-smart-pointers-and-interior-mutability.md), [Ch 12](ch12-unsafe-rust-controlled-danger.md))
- **Error Handling**: Library (thiserror) vs App (anyhow) ([Ch 10](ch10-error-handling-patterns.md))

---

### Trait Bounds Cheat Sheet

| Bound | Meaning |
|-------|---------|
| `T: Clone` | Can be duplicated. |
| `T: Copy` | Bitwise copy (implicit). |
| `T: Send` | Can be moved to another thread. |
| `T: Sync` | `&T` can be shared between threads. |
| `T: 'static` | No non-static references. |
| `T: Sized` | Size known at compile time (default). |
| `T: ?Sized` | Opt-out of Sized requirement (e.g., `[T]`). |

---

### Lifetime Elision Rules

1. Each input reference gets its own lifetime: `fn(x: &i32, y: &i32) -> fn<'a, 'b>(x: &'a i32, y: &'b i32)`.
2. If there is exactly one input lifetime, it is assigned to all outputs.
3. If there is a `&self` or `&mut self` parameter, its lifetime is assigned to all outputs.

---

### Visibility Modifiers

- `pub`: Visible everywhere.
- `pub(crate)`: Visible in this crate.
- `pub(super)`: Visible in the parent module.
- `None`: Visible only in the current module.

***
