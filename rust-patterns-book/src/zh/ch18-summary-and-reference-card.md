## 快速参考卡片

### 模式决策指南

```text
需要为原生类型提供类型安全？
└── 新类型 (Newtype) 模式 (第 3 章)

需要编译时状态约束？
└── 类型状态 (Type-state) 模式 (第 3 章)

需要一个不含运行时数据的“标签”？
└── PhantomData (第 4 章)

需要打破 Rc/Arc 的引用循环？
└── Weak<T> / sync::Weak<T> (第 9 章)

需要等待某个条件而不进行忙碌轮询 (busy-looping)？
└── Condvar + Mutex (第 6 章)

需要处理“N 种类型之一”？
├── 已知的封闭集 → 枚举 (Enum)
├── 开放集，热点路径 → 泛型 (Generics)
├── 开放集，冷点路径 → dyn Trait
└── 完全未知的类型 → Any + TypeId (第 2 章)

需要跨线程共享状态？
├── 简单的计数器/标志 → 原子操作 (Atomics)
├── 短临界区 → 互斥锁 (Mutex)
├── 读多写少 → 读写锁 (RwLock)
├── 延迟一次性初始化 → OnceLock / LazyLock (第 6 章)
└── 复杂状态 → Actor + 通道 (Channels)

需要并行化计算？
├── 集合处理 → rayon::par_iter
├── 后台任务 → thread::spawn
└── 借用本地数据 → thread::scope (线程作用域)

需要异步 I/O 或并发网络？
├── 基础 → tokio + async/await (第 16 章)
└── 高阶 (流、中间件) → 请参阅 Async Rust 进阶指南

需要进行错误处理？
├── 库 (Library) → thiserror (#[derive(Error)])
└── 应用程序 (Application) → anyhow (Result<T>)

需要防止某个值被移动 (Move)？
└── Pin<T> (第 9 章) —— Future 和自引用类型所必需
```

### 特性约束 (Trait Bounds) 速查表

| 约束 | 含义 |
|-------|---------|
| `T: Clone` | 可被克隆 |
| `T: Send` | 可被移动到另一个线程 |
| `T: Sync` | `&T` 可在线程间共享 |
| `T: 'static` | 不包含非静态引用 |
| `T: Sized` | 编译时大小已知 (默认情况) |
| `T: ?Sized` | 大小可能未知 (`[T]`、`dyn Trait`) |
| `T: Unpin` | 在固定 (Pin) 后仍可安全移动 |
| `T: Default` | 拥有默认值 |
| `T: Into<U>` | 可转换为类型 `U` |
| `T: AsRef<U>` | 可被借用为 `&U` |
| `T: Deref<Target = U>` | 自动解引用为 `&U` |
| `F: Fn(A) -> B` | 可调用，以不可变方式借用状态 |
| `F: FnMut(A) -> B` | 可调用，可能会修改状态 |
| `F: FnOnce(A) -> B` | 仅限调用一次，可能会消耗状态 |

### 生命周期消除 (Elision) 规则

在以下三种情况下，编译器会自动插入生命周期（这样你就不必手动编写）：

```rust
// 规则 1：每个引用参数都有自己的生命周期
// fn foo(x: &str, y: &str)  →  fn foo<'a, 'b>(x: &'a str, y: &'b str)

// 规则 2：如果正好只有一个输入生命周期，它将用于所有输出
// fn foo(x: &str) -> &str   →  fn foo<'a>(x: &'a str) -> &'a str

// 规则 3：如果有一个参数是 &self 或 &mut self，则使用其生命周期
// fn foo(&self, x: &str) -> &str  →  fn foo<'a>(&'a self, x: &str) -> &'a str
```

**当你 必须 编写显式生命周期时**：
- 存在多个输入引用且存在引用输出（编译器无法推断应使用哪个输入）。
- 结构体字段包含引用：`struct Ref<'a> { data: &'a str }`。
- 当你需要不包含借用引用的数据时，使用 `'static` 约束。

### 常用的派生特性

```rust
#[derive(
    Debug,          // {:?} 格式化
    Clone,          // .clone() 方法
    Copy,           // 隐式复制 (仅限简单类型)
    PartialEq, Eq,  // == 比较
    PartialOrd, Ord, // < > 比较 + 排序
    Hash,           // HashMap/HashSet 的键
    Default,        // Type::default() 方法
)]
struct MyType { /* ... */ }
```

### 模块可见性速查参考

```text
pub           → 到处可见
pub(crate)    → 仅在当前 crate 内可见
pub(super)    → 对父模块可见
pub(in path)  → 在特定路径内可见
(不写)        → 仅对当前模块及其子模块私有
```

### 进一步阅读

| 资源 | 推荐理由 |
|----------|-----|
| [Rust 设计模式](https://rust-unofficial.github.io/patterns/) | 惯用法与反面模式（anti-patterns）目录 |
| [Rust API 指南](https://rust-lang.github.io/api-guidelines/) | 完善的公共 API 设计官方清单 |
| [Rust 原子操作与锁](https://marabos.nl/atomics/) | Mara Bos 对并发原语的深入探讨 |
| [Rustonomicon (死灵书)](https://doc.rust-lang.org/nomicon/) | 针对 Unsafe Rust 与底层细节的官方指南 |
| [Rust 错误处理](https://blog.burntsushi.net/rust-error-handling/) | Andrew Gallant 撰写的详尽指南 |
| [Jon Gjengset —— Crust of Rust 系列视频](https://www.youtube.com/playlist?list=PLqbS7AVVErFiWDOAVrPt7aYmnuuOLYvOa) | 对迭代器、生命周期、通道等内容的深度剖析 |
| [Effective Rust](https://www.lurklurk.org/effective-rust/) | 35 种改进 Rust 代码的具体方式 |

***

*《Rust 设计模式与工程实践》 完*
