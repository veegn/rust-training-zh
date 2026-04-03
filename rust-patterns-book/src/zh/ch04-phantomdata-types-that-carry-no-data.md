[English Original](../en/ch04-phantomdata-types-that-carry-no-data.md)

# 第 4 章：PhantomData —— 不携带数据的类型 🔴

> **你将学到：**
> - 为什么 `PhantomData<T>` 存在以及它解决的三个问题
> - 生命周期烙印 (Lifetime Branding)：用于编译时作用域强制执行
> - 单位量纲 (Unit-of-measure) 模式：用于量纲安全的算术运算
> - 型变 (Variance)（协变、逆变、不变）以及 `PhantomData` 如何控制它

## PhantomData 解决了什么

`PhantomData<T>` 是一种零大小类型，它告诉编译器：“这个结构体在逻辑上与 `T` 相关联，尽管它并不包含 `T`。”它会影响型变、Drop 检查以及 Auto-trait 推导 —— 且不占用任何内存。

```rust
use std::marker::PhantomData;

// 不使用 PhantomData：
struct Slice<'a, T> {
    ptr: *const T,
    len: usize,
    // 问题：编译器不知道这个结构体借用了 'a，
    // 也不知道为了 Drop 检查的目的它与 T 相关联。
}

// 使用 PhantomData：
struct Slice<'a, T> {
    ptr: *const T,
    len: usize,
    _marker: PhantomData<&'a T>,
    // 现在编译器知道了：
    // 1. 这个结构体借用了生命周期为 'a 的数据
    // 2. 它对 'a 是协变的（生命周期可以缩小）
    // 3. Drop 检查会考虑 T
}
```

**PhantomData 的三项职责**：

| 职责 | 示例 | 作用 |
|-----|---------|-------------|
| **生命周期绑定** | `PhantomData<&'a T>` | 结构体被视为借用了 `'a` |
| **所有权模拟** | `PhantomData<T>` | Drop 检查假设结构体拥有一个 `T` |
| **型变控制** | `PhantomData<fn(T)>` | 使结构体对 `T` 是逆变的 |

### 生命周期烙印 (Lifetime Branding)

使用 `PhantomData` 来防止混用来自不同“会话”或“上下文”的值：

```rust
use std::marker::PhantomData;

/// 仅在特定 Arena 的生命周期内有效的句柄
struct ArenaHandle<'arena> {
    index: usize,
    _brand: PhantomData<&'arena ()>,
}

struct Arena {
    data: Vec<String>,
}

impl Arena {
    fn new() -> Self {
        Arena { data: Vec::new() }
    }

    /// 分配一个字符串并返回一个带有烙印的句柄
    fn alloc<'a>(&'a mut self, value: String) -> ArenaHandle<'a> {
        let index = self.data.len();
        self.data.push(value);
        ArenaHandle { index, _brand: PhantomData }
    }

    /// 通过句柄查找 —— 仅接受来自 *此* Arena 的句柄
    fn get<'a>(&'a self, handle: ArenaHandle<'a>) -> &'a str {
        &self.data[handle.index]
    }
}

fn main() {
    let mut arena1 = Arena::new();
    let handle1 = arena1.alloc("hello".to_string());

    // 无法将 handle1 用于不同的 Arena —— 生命周期将不匹配
    // let mut arena2 = Arena::new();
    // arena2.get(handle1); // ❌ 生命周期不匹配

    println!("{}", arena1.get(handle1)); // ✅
}
```

### 单位量纲 (Unit-of-Measure) 模式

在编译时防止混用不兼容的单位，且运行时开销为零：

```rust
use std::marker::PhantomData;
use std::ops::{Add, Mul};

// 单位标记类型 (零大小)
struct Meters;
struct Seconds;
struct MetersPerSecond;

#[derive(Debug, Clone, Copy)]
struct Quantity<Unit> {
    value: f64,
    _unit: PhantomData<Unit>,
}

impl<U> Quantity<U> {
    fn new(value: f64) -> Self {
        Quantity { value, _unit: PhantomData }
    }
}

// 只能相加相同的单位：
impl<U> Add for Quantity<U> {
    type Output = Quantity<U>;
    fn add(self, rhs: Self) -> Self::Output {
        Quantity::new(self.value + rhs.value)
    }
}

// 米 / 秒 = 米每秒 (自定义 Trait)
impl std::ops::Div<Quantity<Seconds>> for Quantity<Meters> {
    type Output = Quantity<MetersPerSecond>;
    fn div(self, rhs: Quantity<Seconds>) -> Quantity<MetersPerSecond> {
        Quantity::new(self.value / rhs.value)
    }
}

fn main() {
    let dist = Quantity::<Meters>::new(100.0);
    let time = Quantity::<Seconds>::new(9.58);
    let speed = dist / time; // Quantity<MetersPerSecond>
    println!("速度: {:.2} m/s", speed.value); // 10.44 m/s

    // let nonsense = dist + time; // ❌ 编译错误：无法将“米”与“秒”相加
}
```

> **这是纯粹的类型系统魔法** —— `PhantomData<Meters>` 是零大小的，因此 `Quantity<Meters>` 的内存布局与 `f64` 完全相同。在运行时没有包装开销，但在编译时具备完全的单位安全性。

### PhantomData 与 Drop 检查

当编译器检查结构体的析构函数是否可能访问已过时的数据时，它会使用 `PhantomData` 来做出决定：

```rust
use std::marker::PhantomData;

// PhantomData<T> — 编译器假设我们 *可能* 会 drop 一个 T
// 这意味着 T 的生命周期必须比我们的结构体更长
struct OwningSemantic<T> {
    ptr: *const T,
    _marker: PhantomData<T>,  // “我在逻辑上拥有一个 T”
}

// PhantomData<*const T> — 编译器假设我们 *不* 拥有 T
// 要求更宽松 —— T 的生命周期不需要比我们更长
struct NonOwningSemantic<T> {
    ptr: *const T,
    _marker: PhantomData<*const T>,  // “我只是指向 T”
}
```

**实践规则**：在包装原始指针时，请深思熟虑地选择 PhantomData：
- 编写一个拥有其数据的容器？ → `PhantomData<T>`
- 编写一个视图/引用类型？ → `PhantomData<&'a T>` 或 `PhantomData<*const T>`

### 型变 (Variance) —— 为什么 PhantomData 的类型参数很重要

**型变** 决定了一个泛型类型是否可以用其子类型或超类型进行代换（在 Rust 中，“子类型”意味着“具有更长的生命周期”）。搞错型变会导致编译器要么拒绝本应安全的代码 (rejected-good-code)，要么接受实际上不安全的代码 (unsound-accepted-code)。

```mermaid
graph LR
    subgraph Covariant (协变)
        direction TB
        A1["&'long T"] -->|"可以变为"| A2["&'short T"]
    end

    subgraph Contravariant (逆变)
        direction TB
        B1["fn(&'short T)"] -->|"可以变为"| B2["fn(&'long T)"]
    end

    subgraph Invariant (不变)
        direction TB
        C1["&'a mut T"] ---|"不允许代换"| C2["&'b mut T"]
    end

    style A1 fill:#d4efdf,stroke:#27ae60,color:#000
    style A2 fill:#d4efdf,stroke:#27ae60,color:#000
    style B1 fill:#e8daef,stroke:#8e44ad,color:#000
    style B2 fill:#e8daef,stroke:#8e44ad,color:#000
    style C1 fill:#fadbd8,stroke:#e74c3c,color:#000
    style C2 fill:#fadbd8,stroke:#e74c3c,color:#000
```

#### 三种型变

| 型变 | 含义 | “我能否用……代换？” | Rust 示例 |
|----------|---------|---------------------|--------------|
| **协变 (Covariant)** | 子类型关系流向一致 | 在需要 `'short` 的地方使用 `'long` ✅ | `&'a T`, `Vec<T>`, `Box<T>` |
| **逆变 (Contravariant)** | 子类型关系流向相反 | 在需要 `'long` 的地方使用 `'short` ✅ | `fn(T)` (在参数位置) |
| **不变 (Invariant)** | 不允许代换 | 两个方向都不允许 ✅ | `&mut T`, `Cell<T>`, `UnsafeCell<T>` |

#### 为什么 `&'a T` 对 `'a` 是协变的

```rust
fn print_str(s: &str) {
    println!("{s}");
}

fn main() {
    let owned = String::from("hello");
    // owned 的生命周期贯穿整个函数 ('long)
    // print_str 预期 &'_ str ('short — 仅在调用期间有效)
    print_str(&owned); // ✅ 协变：'long → 'short 是安全的
    // 长生命周期的引用总是可以在需要短生命周期引用的地方使用。
}
```

#### 为什么 `&mut T` 对 `T` 是不变的

```rust
// 如果 &mut T 对 T 是协变的，那么这段代码就能编译：
fn evil(s: &mut &'static str) {
    // 我们本可以将一个短生命周期的 &str 写入预留给 &'static str 的位置！
    let local = String::from("temporary");
    // *s = &local; // ← 这将创建一个悬垂的 &'static str
}

// “不变性”防止了这种情况：在变动 (mutating) 时，&'static str ≠ &'a str。
// 编译器会完全拒绝这种代换。
```

#### PhantomData 如何控制型变

`PhantomData<X>` 会赋予你的结构体与 `X` **相同的型变**：

```rust
use std::marker::PhantomData;

// 对 'a 是协变的 — Ref<'long> 可以作为 Ref<'short> 使用
struct Ref<'a, T> {
    ptr: *const T,
    _marker: PhantomData<&'a T>,  // 对 'a 协变, 对 T 协变
}

// 对 T 是不变的 — 防止对 T 执行不安全的生命周期缩短
struct MutRef<'a, T> {
    ptr: *mut T,
    _marker: PhantomData<&'a mut T>,  // 对 'a 协变, 对 T 不变
}

// 对 T 是逆变的 — 在回调容器中很有用
struct CallbackSlot<T> {
    _marker: PhantomData<fn(T)>,  // 对 T 逆变
}
```

**PhantomData 型变速查表**：

| PhantomData 类型 | 对 `T` 的型变 | 对 `'a` 的型变 | 何时使用 |
|------------------|--------------------|--------------------|-----------|
| `PhantomData<T>` | 协变 | — | 你在逻辑上拥有一个 `T` |
| `PhantomData<&'a T>` | 协变 | 协变 | 你借用一个生命周期为 `'a` 的 `T` |
| `PhantomData<&'a mut T>` | **不变** | 协变 | 你可变地借用 `T` |
| `PhantomData<*const T>` | 协变 | — | 指向 `T` 的非拥有指针 |
| `PhantomData<*mut T>` | **不变** | — | 非拥有且可变的指针 |
| `PhantomData<fn(T)>` | **逆变** | — | `T` 出现在参数位置 |
| `PhantomData<fn() -> T>` | 协变 | — | `T` 出现在返回位置 |
| `PhantomData<fn(T) -> T>` | **不变** | — | `T` 在两个位置上互相抵消 |

#### 案例分析：为什么这在实践中很重要

```rust
use std::marker::PhantomData;

// 一个用会话生命周期“标记”值的令牌。
// 必须对 'a 是协变的 —— 否则调用者在将其传递给需要
// 更短生命周期的函数时，将无法缩短生命周期。
struct SessionToken<'a> {
    id: u64,
    _brand: PhantomData<&'a ()>,  // ✅ 协变 — 调用者可以缩短 'a
    // _brand: PhantomData<fn(&'a ())>,  // ❌ 逆变 — 破坏易用性
}

fn use_token(token: &SessionToken<'_>) {
    println!("使用令牌 {}", token.id);
}

fn main() {
    let token = SessionToken { id: 42, _brand: PhantomData };
    use_token(&token); // ✅ 之所以可行，是因为 SessionToken 对 'a 是协变的
}
```

> **决策规则**：默认先使用 `PhantomData<&'a T>`（协变）。只有当你的抽象层会分发对 `T` 的可变访问权限时，才切换到 `PhantomData<&'a mut T>`（不变）。几乎 **永远不要** 使用 `PhantomData<fn(T)>`（逆变）—— 它仅在回调存储等特定场景下才是正确的。

> **关键要点 —— PhantomData**
> - `PhantomData<T>` 在不产生运行时开销的情况下携带类型/生命周期信息。
> - 使用它来实现生命周期烙印、型变控制以及单位量纲模式。
> - Drop 检查：`PhantomData<T>` 告诉编译器你的类型在逻辑上拥有一个 `T`。

> **另请参阅：** [第 3 章 —— Newtype 与类型状态](ch03-the-newtype-and-type-state-patterns.md) 了解使用 PhantomData 的类型状态模式。[第 11 章 —— 不安全 Rust](ch12-unsafe-rust-controlled-danger.md) 了解 PhantomData 如何与原始指针交互。

***

### 练习：使用 PhantomData 实现单位量纲 ★★ (~30 分钟)

扩展单位量纲模式，以支持：
- `Meters` (米)、`Seconds` (秒)、`Kilograms` (千克)
- 相同单位的加法
- 乘法：`Meters * Meters = SquareMeters`
- 除法：`Meters / Seconds = MetersPerSecond`

<details>
<summary>🔑 参考答案</summary>

```rust
use std::marker::PhantomData;
use std::ops::{Add, Mul, Div};

#[derive(Clone, Copy)]
struct Meters;
#[derive(Clone, Copy)]
struct Seconds;
#[derive(Clone, Copy)]
struct Kilograms;
#[derive(Clone, Copy)]
struct SquareMeters;
#[derive(Clone, Copy)]
struct MetersPerSecond;

#[derive(Debug, Clone, Copy)]
struct Qty<U> {
    value: f64,
    _unit: PhantomData<U>,
}

impl<U> Qty<U> {
    fn new(v: f64) -> Self { Qty { value: v, _unit: PhantomData } }
}

impl<U> Add for Qty<U> {
    type Output = Qty<U>;
    fn add(self, rhs: Self) -> Self::Output { Qty::new(self.value + rhs.value) }
}

impl Mul<Qty<Meters>> for Qty<Meters> {
    type Output = Qty<SquareMeters>;
    fn mul(self, rhs: Qty<Meters>) -> Qty<SquareMeters> {
        Qty::new(self.value * rhs.value)
    }
}

impl Div<Qty<Seconds>> for Qty<Meters> {
    type Output = Qty<MetersPerSecond>;
    fn div(self, rhs: Qty<Seconds>) -> Qty<MetersPerSecond> {
        Qty::new(self.value / rhs.value)
    }
}

fn main() {
    let width = Qty::<Meters>::new(5.0);
    let height = Qty::<Meters>::new(3.0);
    let area = width * height; // Qty<SquareMeters>
    println!("面积: {:.1} m²", area.value);

    let dist = Qty::<Meters>::new(100.0);
    let time = Qty::<Seconds>::new(9.58);
    let speed = dist / time;
    println!("速度: {:.2} m/s", speed.value);

    let sum = width + height; // 相同单位 ✅
    println!("总和: {:.1} m", sum.value);

    // let bad = width + time; // ❌ 编译错误：无法将“米”与“秒”相加
}
```

</details>

***
