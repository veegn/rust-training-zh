# 18. 快速参考卡 🟢

### 模式选择指南

- **类型安全**：新类型模式 (Newtype Pattern) ([第 3 章](ch03-newtype-and-type-state-patterns.md))
- **编译时状态强加**：类型状态模式 (Type-State Pattern) ([第 3 章](ch03-newtype-and-type-state-patterns.md))
- **单位元数据**：PhantomData ([第 4 章](ch04-phantomdata-types-that-carry-no-data.md))
- **共享状态**：Arc + Mutex/RwLock ([第 6 章](ch06-concurrency-vs-parallelism-vs-threads.md))
- **并发消息传递**：MPSC 通道 ([第 5 章](ch05-channels-and-message-passing.md))
- **抽象策略**：泛型 (Generics) vs 动态分发 (dyn Trait) ([第 2 章](ch02-traits-in-depth-the-soul-of-rust.md))
- **底层控制**：Unsafe Rust 与 Pin ([第 9 章](ch09-smart-pointers-and-interior-mutability.md), [第 12 章](ch12-unsafe-rust-controlled-danger.md))
- **错误处理**：库 (thiserror) vs 应用 (anyhow) ([第 10 章](ch10-error-handling-patterns.md))

---

### Trait 约束速查表

| 约束 | 含义 |
|-------|---------|
| `T: Clone` | 可被克隆（显式复制）。 |
| `T: Copy` | 按位拷贝（隐式复制）。 |
| `T: Send` | 可被移动到另一个线程。 |
| `T: Sync` | 其引用 `&T` 可在多线程间共享。 |
| `T: 'static` | 不包含非静态引用。 |
| `T: Sized` | 编译时大小已知（默认）。 |
| `T: ?Sized` | 放宽 Sized 限制（如 `[T]`）。 |

---

### 生命周期省略规则

1. 每个输入引用获得独立的生命周期：`fn(x: &i32, y: &i32) -> fn<'a, 'b>(x: &'a i32, y: &'b i32)`。
2. 若恰好只有一个输入生命周期，它将被分配给所有输出。
3. 若存在 `&self` 或 `&mut self` 参数，该生命周期将被分配给所有输出。

---

### 可见性修饰符

- `pub`：全局可见。
- `pub(crate)`：仅在当前 crate 内可见。
- `pub(super)`：仅对父模块可见。
- `无`：仅对当前模块可见。

***
