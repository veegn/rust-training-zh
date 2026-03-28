[English Original](../en/ch02-1-essential-keywords-reference.md)

# 面向 C# 开发者的 Rust 核心关键字速查

> **你将学到什么：** 通过一份速查表理解 Rust 关键字与 C# 对应概念之间的映射，包括可见性修饰符、所有权相关关键字、控制流、类型定义和模式匹配语法。
>
> **难度：** 🟢 初级

理解 Rust 的关键字及其用途，可以帮助 C# 开发者更快建立语言直觉，并在阅读代码时迅速定位语义。

---

## 可见性与访问控制

### C# 访问修饰符
```csharp
public int PublicField;     // 公开
private int privateField;   // 私有（仅本类）
internal int internalField; // 当前程序集
```

### Rust 可见性关键字
```rust
pub struct PublicStruct {
    pub public_field: i32,     // 公开
    private_field: i32,        // 默认私有（无需关键字）
}

pub(crate) fn internal_fn() {} // 当前 crate 的公开（相当于 internal）
```

---

## 内存与所有权

### C# 内存相关关键字
```csharp
public void Method(ref int val); // 引用传递
public void Method(in int val);  // 只读引用 (C# 7.2+)
```

### Rust 所有权关键字
```rust
fn read_only(data: &Vec<i32>);   // 不可变引用 (&)
fn modify(data: &mut Vec<i32>); // 可变引用 (&mut)

let closure = move || { ... };   // 强制在闭包中移动所有权 (move capture)
let boxed = Box::new(42);        // 堆分配 (类似 C# 引用类型的 new)
```

---

## 控制流

### C# 与 Rust 对比
*   **`return`**: 在 Rust 中，如果最后一行是表达式，它是隐式返回的。
*   **`loop`**: 真正的无限循环，相当于 `while(true)`。
*   **`break` / `continue`**: 标准用法，但 `break` 在 `loop` 中可以向外返回一个值。

---

## 类型定义

| **C#** | **Rust** | **备注** |
| :--- | :--- | :--- |
| `class` / `struct` | `struct` | 数据结构 |
| `interface` | `trait` | 共享行为 |
| `enum` | `enum` | 代数数据类型 (ADT)，比 C# 枚举更强大 |
| `using alias` | `type` | 类型别名 |

---

## 关键字总结速查表

| **用途** | **C#** | **Rust** | **关键差异** |
| :--- | :--- | :--- | :--- |
| **可见性** | `public`, `private` | `pub`, 默认私有 | 更细粒度的 `pub(crate)` 控制 |
| **变量** | `var`, `readonly` | `let`, `let mut` | 默认不可变 |
| **函数** | `method()` | `fn` | 独立的顶层函数 |
| **模式** | `switch`, `is` | `match`, `if let` | 必须满足穷尽性匹配 |
| **引用** | `ref`, `in` | `&mut`, `&` | 编译期的借用检查 |
