[English Original](../en/ch05-data-structures.md)

# 5. 数据结构 🟢

本章涵盖了 Rust 的基础数据结构。我们将探索 Rust 如何管理集合和复合类型，这些类型中的许多都能直接映射到 C/C++ 概念，但增加了额外的安全性保证。

---

### 1. 数组与元组

#### 数组 (Arrays)
数组包含固定数量的同类型元素，在编译时确定。它们存储在栈 (Stack) 上。
```rust
fn main() {
    let a: [i32; 3] = [1, 2, 3];
    let b = [0; 5]; // [0, 0, 0, 0, 0]
    
    println!("第一个元素：{}", a[0]);
    // a[10]; // ❌ 运行时崩溃 (边界检查)
}
```

#### 元组 (Tuples)
元组具有固定大小，可以组合不同的类型。
```rust
fn main() {
    let t: (i32, f64, &str) = (500, 6.4, "hello");
    let (x, y, z) = t; // 解构
    
    println!("y 的值：{}", t.1); // 通过点符号访问
}
```

---

### 2. 切片 (Slices)
切片是集合中连续元素序列的视图。它们是包含指向数据的指针和长度的“胖指针”。
```rust
fn main() {
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..4]; // [2, 3, 4]
    
    println!("长度：{}", slice.len());
}
```

---

### 3. 字符串：`String` vs `&str`
Rust 有两种主要的字符串类型。

| 维度 | `String` | `&str` (字符串切片) |
|--------|----------|-----------------------|
| **内存** | 堆分配，可增长 | 借用，大小固定 |
| **所有权** | 拥有所有权 | 借用 (引用) |
| **C++ 等价物** | `std::string` | `std::string_view` |

```rust
fn main() {
    let s_slice: &str = "Hello"; // 字符串字面量（存储在二进制中）
    let mut s_owned = String::from("Hello"); // 堆分配
    s_owned.push_str(", world!");
    
    let borrow: &str = &s_owned; // 将 String 借用为 &str
}
```

---

### 4. 结构体 (Structs)

#### 命名字段结构体
```rust
struct User {
    username: String,
    active: bool,
}

fn main() {
    let user1 = User {
        username: String::from("alice"),
        active: true,
    };
}
```

#### 元组结构体
对于从原始类型创建不同类型非常有用（即“Newtype”模式）。
```rust
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

let black = Color(0, 0, 0);
let origin = Point(0, 0, 0);
```

---

### 5. 集合：`Vec` 与 `HashMap`

#### `Vec<T>` (动态数组)
可增长、在堆上分配的数组。相当于 C++ 中的 `std::vector`。
```rust
fn main() {
    let mut v = vec![1, 2, 3];
    v.push(4);
    
    // 危险的索引
    let third = &v[2]; // 可能导致崩溃
    let safe_third = v.get(2); // 返回 Option<&T>
}
```

#### `HashMap<K, V>`
键值对。相当于 C++ 中的 `std::unordered_map`。
```rust
use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
}
```

---

### 深入探讨：C++ vs Rust 引用
对 C++ 开发者而言，重要的是注意：
1. **没有右值/万能引用**：Rust 默认使用所有权和移动，而非 `&&`。
2. **按位移动**：Rust 中的移动始终是浅层的 `memcpy`。没有移动构造函数。
3. **自动解引用 (Auto-Deref)**：编译器可以通过 `Deref` Trait 自动对多层指针（例如从 `Box<String>` 到 `&str`）进行解引用。

***
