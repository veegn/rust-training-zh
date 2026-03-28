# Summary and Reference Card 🟡

## Quick Reference Card

### Async Mental Model

```text
┌─────────────────────────────────────────────────────┐
│  async fn → State Machine (enum) → impl Future      │
│  .await   → poll() the inner future                 │
│  executor → loop { poll(); sleep_until_woken(); }   │
│  waker    → "hey executor, poll me again"           │
│  Pin      → "promise I won't move in memory"        │
└─────────────────────────────────────────────────────┘
```

### Common Patterns Cheat Sheet

| Goal | Use |
|------|-----|
| Run two futures concurrently | `tokio::join!(a, b)` |
| Race two futures | `tokio::select! { ... }` |
| Spawn a background task | `tokio::spawn(async { ... })` |
| Run blocking code in async | `tokio::task::spawn_blocking(|| { ... })` |
| Limit concurrency | `Semaphore::new(N)` |
| Collect many task results | `JoinSet` |
| Share state across tasks | `Arc<Mutex<T>>` or channels |
| Graceful shutdown | `watch::channel` + `select!` |
| Process a stream N-at-a-time | `.buffer_unordered(N)` |
| Timeout a future | `tokio::time::timeout(dur, fut)` |
| Retry with backoff | Custom combinator (see Ch 13) |

### Pinning Quick Reference

| Situation | Use |
|-----------|-----|
| Pin a future on the heap | `Box::pin(fut)` |
| Pin a future on the stack | `tokio::pin!(fut)` |
| Pin an `Unpin` type | `Pin::new(&mut val)` — safe & zero cost |
| Return a pinned trait object | `-> Pin<Box<dyn Future<Output = T> + Send>>` |

### Channel Selection Guide

| Channel | Producers | Consumers | Values | Use When |
|---------|-----------|-----------|--------|----------|
| `mpsc` | Many (N) | 1 | Stream | Work queues, event buses |
| `oneshot` | 1 | 1 | Single | Request/Response, completion |
| `broadcast` | Many (N) | Many (N) | All see all | Fan-out notifications, shutdown |
| `watch` | 1 | Many (N) | Only latest | Config updates, health status |

### Mutex Selection Guide

| Mutex | Use When |
|-------|----------|
| `std::sync::Mutex` | Locked section is tiny, never crosses `.await` |
| `tokio::sync::Mutex` | Must hold the lock across an `.await` point |
| `parking_lot::Mutex` | High contention, no `.await`, extreme perf |
| `tokio::sync::RwLock` | Many readers/few writers, crosses `.await` |

### Decision Quick Reference

```text
Need concurrency?
├── I/O-bound → async/await
├── CPU-bound → rayon / std::thread
└── Mixed → spawn_blocking for CPU parts

Choosing runtime?
├── Server app → tokio
├── Library → runtime-agnostic (futures crate)
├── Embedded → embassy
└── Minimal → smol

Need concurrent futures?
├── Can be 'static + Send → tokio::spawn
├── Can be 'static + !Send → LocalSet
├── Can't be 'static → FuturesUnordered
└── Need to track/abort → JoinSet
```

### Common Error Messages and Fixes

| Error | Cause | Fix |
|-------|-------|-----|
| `future is not Send` | Held `!Send` type across `.await` | Scope tightly or use `current_thread` |
| `borrowed value does not live long enough` (in spawn) | `tokio::spawn` needs `'static` life | Use `Arc`, `clone()`, or `FuturesUnordered` |
| `the trait Future is not implemented for ()` | Missed an `.await` | Add `.await` to async calls |
| `cannot borrow as mutable` (in poll) | Self-referential borrow | Proper use of `Pin<&mut Self>` (Ch 4) |
| Program hangs silently | Forgot to call `waker.wake()` | Ensure every `Pending` path registers waker |

### Further Reading

| Resource | Why |
|----------|-----|
| [Tokio Tutorial](https://tokio.rs/tokio/tutorial) | Official guide — great for your first project |
| [Async Book (official)](https://rust-lang.github.io/async-book/) | Covers `Future`, `Pin`, `Stream` at language level |
| [Jon Gjengset — Crust of Rust: async/await](https://www.youtube.com/watch?v=ThjvMReOXYM) | 2h deep dive with live coding |
| [Alice Ryhl — Actors with Tokio](https://ryhl.io/blog/actors-with-tokio/) | Patterns for stateful production services |
| [Without Boats — Pin, Unpin, and why Rust needs them](https://without.boats/blog/pin/) | Original design logic from the designers |
| [Tokio mini-Redis](https://github.com/tokio-rs/mini-redis) | Complete async project — study-worthy code |
| [Tower documentation](https://docs.rs/tower) | Middleware/service pattern used by axum/tonic |

***
