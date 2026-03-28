# 4. PhantomData —— 不携带数据的类型 🔶

> **你将学到：**
> - 为什么 `PhantomData<T>` 存在以及它解决的三个问题
> - 生命周期烙印 (Lifetime Branding) 用于编译时作用域强制执行
> - 单位测量模式 (Unit-of-measure Pattern) 用于维度安全算术
> - 型变（协变、逆变、不变性）以及 PhantomData 如何控制它

## PhantomData 解决了什么

`PhantomData<T>` 是一种零大小的类型，它告诉编译器：“尽管该结构体不包含 `T`，但在逻辑上它与 `T` 相关联。”它会影响型变 (Variance)、析构检查 (Drop Checking) 和自动 trait 推导 (Auto-trait Inference) —— 且不占用任何内存。

```rust
use std::marker::PhantomData;

struct Slice<'a, T> {
    ptr: *const T,
    len: usize,
    _marker: PhantomData<&'a T>,
    // 现在编译器知道了：
    // 1. 该结构体借用了生命周期为 'a 的数据
    // 2. 它对 'a 是协变的（生命周期可以缩小）
    // 3. 析构检查会考虑 T
}
```

### 1. 生命周期烙印 (Lifetime Branding)

使用 `PhantomData` 来防止混用来自不同“会话 (Sessions)”或“上下文 (Contexts)”的值：

```rust
struct ArenaHandle<'arena> {
    index: usize,
    _brand: PhantomData<&'arena ()>,
}
```

这确保了即使两个 Arena（内存池）内部表示相同，你也无法在 `Arena B` 中使用来自 `Arena A` 的句柄。

### 2. 单位测量模式 (Unit-of-measure Pattern)

通过零运行时成本，在编译时防止混用不兼容的单位：

```rust
struct Meters;
struct Seconds;

struct Quantity<Unit> {
    value: f64,
    _unit: PhantomData<Unit>,
}

let dist = Quantity::<Meters>::new(100.0);
let time = Quantity::<Seconds>::new(10.0);
// let sum = dist + time; // ❌ 编译错误：米 != 秒
```

---

## 型变 (Variance)：为什么它很重要

**型变 (Variance)** 决定了泛型类型是否可以被其子类型或超类型替换。

| 型变 (Variance) | “是否可以替换……” | Rust 示例 |
|----------|-----------------|--------------|
| **协变 (Covariant)** | 在期望 `'short` 的地方使用 `'long` ✅ | `&'a T` |
| **逆变 (Contravariant)** | 在期望 `'long` 的地方使用 `'short` ✅ | `fn(T)` (作为参数) |
| **不变 (Invariant)** | 不允许任何替换 ❌ | `&mut T`, `Cell<T>` |

### PhantomData 型变速查表

| PhantomData 类型 | 针对 `T` 的型变 | 针对 `'a` 的型变 |
|------------------|----------------|----------------|
| `PhantomData<T>` | 协变 | — |
| `PhantomData<&'a T>` | 协变 | 协变 |
| `PhantomData<&'a mut T>` | **不变** | 协变 |
| `PhantomData<*mut T>` | **不变** | — |
| `PhantomData<fn(T)>` | **逆变** | — |

> **实践建议**：默认从 `PhantomData<&'a T>` (协变) 开始。只有当你需要通过此抽象分发对内部数据的可变访问权限时，才切换到 `&'a mut T` (不变性)。

***
