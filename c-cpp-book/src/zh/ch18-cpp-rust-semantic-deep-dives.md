# 18. C++ 到 Rust 语义深度对比 🟢

从 C++ 转向 Rust 不仅仅是学习新语法，还需要在思考内存、类型和程序结构的方式上发生转变。本章将深入探讨 C++ 与 Rust 之间的语义差异和映射。

### 1. RAII (资源获取即初始化)
C++ 和 Rust 都使用 RAII 来管理资源。在 C++ 中，这与构造函数和析构函数相关联。在 Rust 中，它与所有权和 `Drop` Trait 相关。

```rust
struct MyResource {
    name: String,
}

impl Drop for MyResource {
    fn drop(&mut self) {
        println!("正在释放资源：{}", self.name);
        // 资源清理在此处自动发生
    }
}

fn main() {
    {
        let _res = MyResource { name: String::from("文件句柄") };
    } // `_res` 超出作用域，将在此处被丢弃 (Dropped)
}
```

---

### 2. 移动语义 (Move Semantics)
在 C++ 中，移动语义是显式的（使用 `std::move`），且对象被留在一种“有效但未指定”的状态。在 Rust 中，**移动是默认行为**，并且编译器会阻止你在对象被移动后使用它。

```rust
fn main() {
    let s1 = String::from("你好");
    let s2 = s1; // s1 被 MOVED 到 s2

    // println!("{}", s1); // 错误：s1 已被移动
    println!("{}", s2); // 正确
}
```

---

### 3. 查看智能指针 (Smart Pointers)
Rust 的智能指针与 C++ 的智能指针映射关系非常紧密，但具有更严格的安全规则。

| C++ | Rust | 描述 |
|-----|------|-------------|
| `std::unique_ptr<T>` | `Box<T>` | 堆上的单一所有权。 |
| `std::shared_ptr<T>` | `Arc<T>` | 线程安全的引用计数。 |
| `std::weak_ptr<T>` | `Weak<T>` | 指向 `Arc` 的非拥有性引用。 |
| `T*` (裸指针) | `*const T`, `*mut T` | 谨慎使用的 unsafe 指针。 |

---

### 4. 零成本抽象 (Zero-Cost Abstractions)
两门语言都以“不为你未使用的东西付费”而自豪。Rust 的 Trait 和泛型会被编译成高效的机器码，通常使用单态化 (Monomorphization) —— 类似于 C++ 模板。

```rust
fn print_it<T: std::fmt::Display>(item: T) {
    println!("{}", item);
}

fn main() {
    print_it(42);      // 为 i32 编译
    print_it("你好");   // 为 &str 编译
}
```

---

### 对于 C/C++ 开发者的总结
- **在 C++ 中**：你拥有巨大的权力，但也担负着巨大的责任。许多“最佳实践”只是准则，编译器并不会严格强制执行。
- **在 Rust 中**：编译器默认强制执行内存安全和线程安全。虽然一开始这可能会让你感到受限，但它允许你以在 C++ 中难以实现的信心水平来构建复杂的系统。理解这些语义映射是成为熟练 Rust 开发者的关键。

***
