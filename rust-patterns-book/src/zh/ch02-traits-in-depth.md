[English Original](../en/ch02-traits-in-depth.md)

# 2. Trait 深入解析 🟡

> **你将学到：**
> - 关联类型 vs 泛型参数 —— 以及何时使用它们
> - GAT、blanket impl、标记 trait 以及 trait 对象安全规则
> - vtable 和脂肪指针的底层工作原理
> - 扩展 trait、枚举分发以及类型化命令模式

## 关联类型 vs 泛型参数

二者都能让 trait 处理不同的类型，但它们的用途不同：

```rust
// --- 关联类型：每个类型只有一个实现 ---
trait Iterator {
    type Item; // 每个迭代器只产生一种类型的项

    fn next(&mut self) -> Option<Self::Item>;
}

// 一个总是产生 i32 的自定义迭代器 —— 没有其他选择
struct Counter { max: i32, current: i32 }

impl Iterator for Counter {
    type Item = i32; // 每个实现只有一个 Item 类型
    fn next(&mut self) -> Option<i32> {
        if self.current < self.max {
            self.current += 1;
            Some(self.current)
        } else {
            None
        }
    }
}

// --- 泛型参数：每个类型可以有多个实现 ---
trait Convert<T> {
    fn convert(&self) -> T;
}

// 一个类型可以为多种目标类型实现 Convert：
impl Convert<f64> for i32 {
    fn convert(&self) -> f64 { *self as f64 }
}
impl Convert<String> for i32 {
    fn convert(&self) -> String { self.to_string() }
}
```

**何时该用哪一个**：

| 使用 | 何时 |
|-----|------|
| **关联类型** | 每个实现类型恰好有一个自然的输出/结果（例如 `Iterator::Item`）。 |
| **泛型参数** | 一个类型可以有意义地为许多不同的类型实现该 trait（例如 `From<T>`）。 |

**直觉解析**：如果问“这个迭代器的 `Item` 是什么？”是有意义的，请使用关联类型。如果问“这个类型能转换成 `f64` 吗？转换成 `String` 吗？转换成 `bool` 吗？”是有意义的，请使用泛型参数。

```rust
// 现实世界示例：std::ops::Add
trait Add<Rhs = Self> {
    type Output; // 关联类型 —— 加法只有一个结果类型
    fn add(self, rhs: Rhs) -> Self::Output;
}

// Rhs 是一个泛型参数 —— 你可以向 Meters 添加不同的类型：
struct Meters(f64);
struct Centimeters(f64);

impl Add<Meters> for Meters {
    type Output = Meters;
    fn add(self, rhs: Meters) -> Meters { Meters(self.0 + rhs.0) }
}
impl Add<Centimeters> for Meters {
    type Output = Meters;
    fn add(self, rhs: Centimeters) -> Meters { Meters(self.0 + rhs.0 / 100.0) }
}
```

---

## 性能对比：静态分发 vs 动态分发

| 特性 | 静态分发 (`impl Trait`) | 动态分发 (`dyn Trait`) |
|---------|-------------------------|------------------------|
| **机制** | 单态化 (专门化) | vtable (脂肪指针) |
| **调用开销** | 零 (可内联) | 一次间接跳转 (指针跳转) |
| **内联** | ✅ 支持 | ❌ 不支持 |
| **二进制体积**| 较大 (多份副本) | 较小 (单份副本) |
| **混合类型** | ❌ 不支持 | ✅ 支持 (`Vec<Box<dyn Trait>>`) |

---

## 高级主题：GAT (泛型关联类型)

从 Rust 1.65 开始，关联类型可以拥有自己的泛型参数。这使得 **借用迭代器 (lending iterators)** 成为可能：

```rust
trait LendingIterator {
    type Item<'a> where Self: 'a;
    fn next(&mut self) -> Option<Self::Item<'_>>;
}
```

这让迭代器能产出那些生命周期绑定在迭代器自身上的项。

---

## 关键要点：Trait

- **Supertraits** (`trait B: A`)：当实现 B 的前提是必须先实现 A 时使用。
- **Blanket Implementations** (`impl<T: A> B for T`)：自动为所有拥有 A 特征的类型赋予 B 特解。
- **Extension Traits**：为你不拥有的类型添加新方法（例如：为 `std::vec::Vec` 添加 `.toJson()`）。
- **Trait 对象安全**：并非所有 trait 都能作为 `dyn Trait` 使用（不能有泛型方法，不能在返回类型中使用 `Self`）。

***
