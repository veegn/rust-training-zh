[English Original](../en/ch09-smart-pointers-and-interior-mutability.md)

# 9. 智能指针与内部可变性 🟡

> **你将学到：**
> - 用于堆分配和共享所有权的 `Box`、`Rc`、`Arc`
> - 用于打破引用循环的 `Weak` 引用
> - 用于内部可变性的 `Cell`、`RefCell` 和 `Cow`
> - 用于自引用类型的 `Pin`

## Box, Rc, Arc — 堆分配与共享

| 指针 | 使用场景 | 可变性 | 线程安全 |
|---------|----------|------------|-------------|
| `Box<T>` | 堆上的单一所有者。 | 通过 `&mut` | ✅ (若 T: Send) |
| `Rc<T>` | 共享所有权（单线程）。 | 无（需包裹在 RefCell 中） | ❌ |
| `Arc<T>` | 共享所有权（多线程）。 | 无（需包裹在 Mutex 中） | ✅ |

### Weak 引用

`Weak<T>` 是一种不具有所有权的句柄。使用它来打破循环（例如，子节点指向父节点的指针）并防止内存泄漏。

```rust
let parent = Rc::new(Node { ... });
let child = Rc::new(Node {
    parent: Rc::downgrade(&parent), // Weak 连接
});
```

---

## 内部可变性

内部可变性允许你通过将借用检查移至运行时，来修改隐藏在共享（`&`）引用之后的数据。

- **`Cell<T>`**：永远不会触发 Panic，主要通过替换/复制值来工作。仅适用于 `Copy` 类型。
- **`RefCell<T>`**：适用于任何类型，但如果在运行时违反借用规则，将会触发 Panic。

```rust
let data = RefCell::new(vec![1, 2, 3]);
data.borrow_mut().push(4); // 对 &RefCell 执行修改
```

---

## Cow — 写时克隆

`Cow` (Clone on Write) 允许你在不需要实际修改数据时，避免分配新的 `String` 或 `Vec`。

```rust
fn normalize(s: &str) -> Cow<'_, str> {
    if s.contains('\t') {
        Cow::Owned(s.replace('\t', " "))
    } else {
        Cow::Borrowed(s)
    }
}
```

---

## Pin — 防止移动

`Pin<P>` 保证被指向的值永远不会在内存中被移动。这对于 **自引用类型**（如 `async/await` 生成的类型）是必不可少的。

- **`Unpin`**：大多数类型默认都是 `Unpin` 的（可以安全移动）。
- **`!Unpin`**：一旦初始化后就必须固定在某个内存地址的类型。

---

## 丢弃顺序 (RFC 1857)

- **局部变量**：按声明的 **逆序** 丢弃。
- **结构体字段**：按声明的 **顺序**（从上到下）丢弃。

***
