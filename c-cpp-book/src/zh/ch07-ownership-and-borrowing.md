# 7. 所有权与借用 🟢

所有权 (Ownership) 是 Rust 最独特的特性，它使得 Rust 在不需要垃圾回收器的情况下就能提供内存安全保证。

### 1. 所有权的三项规则
1. Rust 中的每个值都有一个被称为其 **所有者 (Owner)** 的变量。
2. 任一时刻只能有 **一个所有者**。
3. 当所有者离开作用域，该值将被 **释放 (Dropped)**（内存被回收）。

---

### 2. 移动语义 (Move Semantics)
当你将一个具有所有权的值赋值给另一个变量时，其所有权会发生 **移动 (Move)**。原始变量将变得失效。

```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1; // s1 的所有权移动到了 s2

    // println!("{s1}"); // ❌ 编译错误：使用了已移动的值
}
```

这与 C++ 不同，在 C++ 中 `std::move` 会使原始对象处于“有效但未指定 (valid but unspecified)”的状态。在 Rust 中，编译器严格禁止访问已移动出的变量。

---

### 3. 借用 (Borrowing / 引用)
与其转移所有权，你可以通过创建引用 (`&`) 来 **借用 (Borrow)** 一个值。

#### 借用规则
在任何给定时刻，你只能拥有以下二者之一：
- **任意数量的不可变引用** (`&T`)。
- **恰好一个可变引用** (`&mut T`)。

并且引用必须始终有效（它们不能活得比所有者还久）。

```rust
fn main() {
    let mut s = String::from("hello");

    let r1 = &s; // 不可变借用
    let r2 = &s; // 另一个不可变借用
    println!("{r1} 和 {r2}"); // ✅ 正确

    // let r3 = &mut s; // ❌ 错误：无法作为可变借用，因为已经有了不可变借用
    
    {
        let r4 = &mut s; // ✅ 正确：在此之前 r1 和 r2 已不再使用
        r4.push_str(", world");
    }
}
```

---

### 4. `Copy` Trait
完全存储在栈上的类型（如 `i32`、`bool`、`f64`）不具备移动语义。它们会被 **拷贝 (Copied)**。

```rust
fn main() {
    let x = 5;
    let y = x; // x 被拷贝到 y；两者均有效
    println!("x={x}, y={y}"); // ✅ 正确
}
```

---

### 5. `Drop` Trait (RAII)
`Drop` Trait 允许你自定义当一个值离开作用域时发生的事情。这相当于 C++ 中的 **析构函数 (Destructor)**。

```rust
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("正在释放 CustomSmartPointer，其数据为 `{}`！", self.data);
    }
}

fn main() {
    let c = CustomSmartPointer {
        data: String::from("我的东西"),
    };
    println!("CustomSmartPointer 已创建。");
} // `c` 在这里离开作用域，`drop` 被自动调用。
```

***
