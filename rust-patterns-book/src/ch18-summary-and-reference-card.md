## Quick Reference Card / 快速参考卡

### Pattern Decision Guide / 模式决策指南

```text
Need type safety for primitives? / 需要原始类型的类型安全性？
└── Newtype pattern (Ch3) / 新类型模式（第 3 章）

Need compile-time state enforcement? / 需要在编译时强制执行状态检查？
└── Type-state pattern (Ch3) / 类型状态模式（第 3 章）

Need a "tag" with no runtime data? / 需要一个不带运行时数据的“标签”？
└── PhantomData (Ch4) / PhantomData（第 4 章 / 原书为 Ch4）

Need to break Rc/Arc reference cycles? / 需要打破 Rc/Arc 的循环引用？
└── Weak<T> / sync::Weak<T> (Ch8) / 弱引用（第 8 章）

Need to wait for a condition without busy-looping? / 需要在不忙轮询的情况下等待某个条件？
└── Condvar + Mutex (Ch6) / 条件变量 + 互斥锁（第 6 章）

Need to handle "one of N types"? / 需要处理“N 选 1”的类型？
├── Known closed set → Enum / 已知的封闭集合 → 枚举
├── Open set, hot path → Generics / 开放集合、热点路径 → 泛型
├── Open set, cold path → dyn Trait / 开放集合、非热点路径 → 动态分发（dyn Trait）
└── Completely unknown types → Any + TypeId (Ch2) / 完全未知的类型 → Any + TypeId（第 2 章）

Need shared state across threads? / 需要跨线程共享状态？
├── Simple counter/flag → Atomics / 简单的计数器/标志 → 原子操作
├── Short critical section → Mutex / 较短的临界区 → 互斥锁
├── Read-heavy → RwLock / 读多写少 → 读写锁
├── Lazy one-time init → OnceLock / LazyLock (Ch6) / 惰性的一次性初始化 → OnceLock / LazyLock（第 6 章）
└── Complex state → Actor + Channels / 复杂状态 → Actor 模式 + 通道

Need to parallelize computation? / 需要将计算并行化？
├── Collection processing → rayon::par_iter / 集合处理 → rayon::par_iter
├── Background task → thread::spawn / 后台任务 → thread::spawn
└── Borrow local data → thread::scope / 借用本地数据 → thread::scope

Need async I/O or concurrent networking? / 需要异步 I/O 或并发网络？
├── Basic → tokio + async/await (Ch16) / 基础 → tokio + async/await（第 16 章）
└── Advanced (streams, middleware) → see Async Rust Training / 进阶（流、中间件）→ 参见 Async Rust 进阶指南

Need error handling? / 需要错误处理？
├── Library → thiserror (#[derive(Error)]) / 库 → thiserror
└── Application → anyhow (Result<T>) / 应用程序 → anyhow

Need to prevent a value from being moved? / 需要防止某个值被移动？
└── Pin<T> (Ch8) / Pin（第 8 章）— required for Futures, self-referential types / Future 及自引用类型所需
```

### Trait Bounds Cheat Sheet / Trait 约束速查表

| Bound / 约束 | Meaning / 含义 |
|-------|---------|
| `T: Clone` | Can be duplicated / 可被复制 |
| `T: Send` | Can be moved to another thread / 可被移动到另一个线程 |
| `T: Sync` | `&T` can be shared between threads / 其不可变引用 `&T` 可跨线程共享 |
| `T: 'static` | Contains no non-static references / 不包含非静态引用的生命周期 |
| `T: Sized` | Size known at compile time (default) / 编译时大小已知（默认） |
| `T: ?Sized` | Size may not be known (`[T]`, `dyn Trait`) / 大小可能未知 |
| `T: Unpin` | Safe to move after pinning / 被固定（pin）后仍可安全地移动 |
| `T: Default` | Has a default value / 具有默认值 |
| `T: Into<U>` | Can be converted to `U` / 可以转换为类型 `U` |
| `T: AsRef<U>` | Can be borrowed as `&U` / 可以作为 `&U` 被借用 |
| `T: Deref<Target = U>` | Auto-derefs to `&U` / 自动解引用为 `&U` |
| `F: Fn(A) -> B` | Callable, borrows state immutably / 可调用，以不可变方式借用状态 |
| `F: FnMut(A) -> B` | Callable, may mutate state / 可调用，可能会修改状态 |
| `F: FnOnce(A) -> B` | Callable exactly once, may consume state / 仅可被调用一次，可能会消耗状态 |

### Lifetime Elision Rules / 生命周期省略规则

The compiler inserts lifetimes automatically in three cases (so you don't have to):

编译器会在以下三种情况下自动插入生命周期（无需手动标注）：

```rust
// Rule 1: Each reference parameter gets its own lifetime
// 规则 1：每一个引用类型的参数都会获得其各自的生命周期
// fn foo(x: &str, y: &str)  →  fn foo<'a, 'b>(x: &'a str, y: &'b str)

// Rule 2: If there's exactly ONE input lifetime, it's used for all outputs
// 规则 2：如果恰好只有一个输入参数的生命周期，它将被用于所有的输出。
// fn foo(x: &str) -> &str   →  fn foo<'a>(x: &'a str) -> &'a str

// Rule 3: If one parameter is &self or &mut self, its lifetime is used
// 规则 3：如果包含 &self 或 &mut self 参数，则该生命周期将用于输出。
// fn foo(&self, x: &str) -> &str  →  fn foo<'a>(&'a self, x: &str) -> &'a str
```

**When you MUST write explicit lifetimes / 必须手动编写显式生命周期的情况**：
- Multiple input references and a reference output (compiler can't guess which input) / 存在多个输入引用参数且有一个返回引用结果（编译器无法推断应遵循哪个输入）
- Struct fields that hold references: `struct Ref<'a> { data: &'a str }` / 结构体持有引用类型的字段：`struct Ref<'a> { data: &'a str }`
- `'static` bounds when you need data without borrowed references / 当需要不带任何被借用引用的数据时，使用 `'static` 约束

### Common Derive Traits / 常用的 Derive Trait

```rust
#[derive(
    Debug,          // {:?} formatting / 格式化输出
    Clone,          // .clone()
    Copy,           // Implicit copy (only for simple types) / 隐式拷贝（仅适用于简单类型）
    PartialEq, Eq,  // == comparison / 等值比较
    PartialOrd, Ord, // < > comparison + sorting / 大小比较 + 排序
    Hash,           // HashMap/HashSet key / HashMap/HashSet 键名
    Default,        // Type::default() / 默认值
)]
struct MyType { /* ... */ }
```

### Module Visibility Quick Reference / 模块可见性快速参考

```text
pub           → visible everywhere / 到处可见
pub(crate)    → visible within the crate / 在当前 crate 内可见
pub(super)    → visible to parent module / 对父模块可见
pub(in path)  → visible within a specific path / 在特定路径内可见
(nothing)     → private to current module + children / 对当前模块及其子模块私有
```

### Further Reading / 延伸阅读
麻

| Resource / 资源 | Why / 推荐理由 |
|----------|-----|
| [Rust Design Patterns](https://rust-unofficial.github.io/patterns/) | Catalog of idiomatic patterns and anti-patterns / 惯用模式与反模式的百科目录 |
| [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/) | Official checklist for polished public APIs / 官方发布的公共 API 完善检查清单 |
| [Rust Atomics and Locks](https://marabos.nl/atomics/) | Mara Bos's deep dive into concurrency primitives / Mara Bos 对并发原语的深入探讨 |
| [The Rustonomicon](https://doc.rust-lang.org/nomicon/) | Official guide to unsafe Rust and dark corners / 关于 Unsafe Rust 与黑暗角落的官方指南 |
| [Error Handling in Rust](https://blog.burntsushi.net/rust-error-handling/) | Andrew Gallant's comprehensive guide / Andrew Gallant 撰写的错误处理全面指南 |
| [Jon Gjengset — Crust of Rust series](https://www.youtube.com/playlist?list=PLqbS7AVVErFiWDOAVrPt7aYmnuuOLYvOa) | Deep dives into iterators, lifetimes, channels, etc. / 深入探讨迭代器、生命周期、通道等专题 |
| [Effective Rust](https://www.lurklurk.org/effective-rust/) | 35 specific ways to improve your Rust code / 35 个改进 Rust 代码的具体方法 |

***

*End of Rust Patterns & Engineering How-Tos*

*《Rust 模式与工程实务指南》—— 完*

