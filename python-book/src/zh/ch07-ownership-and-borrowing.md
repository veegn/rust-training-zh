[English Original](../en/ch07-ownership-and-borrowing.md)

# 7. 所有权与借用 🟡

> **你将学到：**
> - 为什么 Rust 拥有所有权机制（且没有垃圾回收！）
> - 移动语义 (Move Semantics) vs Python 的引用计数 (Reference Counting)
> - 借用 (`&` 和 `&mut`) 及其单一写者原则
> - 生命周期概念入门与智能指针 (`Box`, `Rc`, `Arc`)

## 理解所有权

这是给 Python 开发者带来的最大冲击。在 Python 中，你从未考虑过谁“拥有”数据 —— 垃圾回收器 (GC) 处理这一切。在 Rust 中，每个值都有且仅有一个**所有者 (Owner)**，这种编译期的追踪使得 Rust 在无需 GC 的情况下实现内存安全。

### Python: 随处共享的引用
在 Python 中，赋值操作复制的是**引用**。
```python
a = [1, 2, 3]
b = a          # b 和 a 指向同一个列表
b.append(4)
print(a)        # [1, 2, 3, 4] — a 也被改了！
```

### Rust: 单一所有权 (移动语义)
在 Rust 中，赋值操作默认会发生**移动 (Move)**。
```rust
let a = vec![1, 2, 3];
let b = a;           // 所有权移交给 b。'a' 变得无效。
// println!("{:?}", a); // ❌ 编译报错！
```

## 借用规则

为了在不获取所有权的情况下使用数据，我们可以进行**借用 (Borrowing)**。借用分为两种：

1. **多重不可变借用 (`&T`)**：多个人可以同时读取数据，正如多个人可以同时读一本书。
2. **唯一可变借用 (`&mut T`)**：只有一个人可以修改数据，且此时不允许其他人读取或修改。

```rust
let mut data = vec![1, 2, 3];

let r1 = &data; // 没问题
let r2 = &data; // 没问题 (多个读取者)

// let r3 = &mut data; // ❌ 报错！不能在存在活跃读取者时进行修改。
```

## 移动语义 vs 引用计数

| 概念 | Python | Rust |
|---------|--------|------|
| 简单类型 | 自动复制 (int, float) | `Copy` 特征类型 (i32, f64) |
| 复合类型 | 共享引用 | **移动 (Move)** 所有权转移 |
| 内存清理 | GC (引用计数 + 循环检测) | **确定性** (离开作用域即清理) |
| 深拷贝 | `copy.deepcopy(x)` | `x.clone()` (显式深拷贝) |

## 智能指针 (显式选择共享所有权)

如果你**确实**需要像 Python 这种随处共享的所有权，你可以显式使用智能指针：

- **`Box<T>`**：最简单的堆内存分配。
- **`Rc<T>`**：引用计数 (Reference Counting)。仅用于单线程。
- **`Arc<T>`**：原子引用计数 (Atomic RC)。多线程环境下安全。
- **`RefCell<T>`**：内部可变性 —— 在运行时而非编译期进行借用检查。

---

## 练习

<details>
<summary><strong>🏋️ 练习：修复借用检查报错</strong> (点击展开)</summary>

**挑战**：在不使用 `.clone()` 的前提下修复以下代码。

```rust
fn main() {
    let mut names = vec!["张三".to_string()];
    let first = &names[0];     // 不可变借用发生
    names.push("李四".to_string()); // ❌ 在被借用时尝试修改！
    println!("{first}");
}
```

<details>
<summary>参考答案</summary>

```rust
fn main() {
    let mut names = vec!["张三".to_string()];
    {
        let first = &names[0];
        println!("{first}"); // 在冲突发生前完成借用行为
    } // first 在这里离开了作用域
    names.push("李四".to_string()); // 现在进行修改是安全的
}
```
</details>
</details>

***
