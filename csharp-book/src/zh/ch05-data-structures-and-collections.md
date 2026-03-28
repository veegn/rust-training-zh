[English Original](../en/ch05-data-structures-and-collections.md)

# 元组与数据结构

> **你将学到什么：** Rust 元组与 C# `ValueTuple` 的区别、数组与切片、结构体与类、如何用 newtype 模式为领域建模提供零成本类型安全，以及解构语法。
>
> **难度：** 🟢 初级

## 元组与解构
C# 从 C# 7 起提供了 `ValueTuple`。Rust 元组在概念上非常类似，但在语言中的集成程度更深。

### Rust 元组 (Tuples)
```rust
// Rust 元组默认是不可变的，且不支持命名元素。
let point = (10, 20); // (i32, i32)
let (x, y) = point;    // 解构 (Destructuring)

// 通过索引访问
println!("x={}, y={}", point.0, point.1);

// 元组作为返回值
fn divide(a: i32, b: i32) -> (i32, i32) {
    (a / b, a % b)
}
```

### 元组结构体 (Newtypes)
当普通元组的语义不够清晰，或者你想要在编译期防止单位混用时，可以使用元组结构体：
```rust
struct Meters(f64);
struct Celsius(f64);

let d = Meters(100.0);
let t = Celsius(36.6);
// d == t; // ❌ 编译错误：类型不匹配！
```

---

## Newtype 模式：零成本领域建模
通过 **Newtype 模式**，Rust 开发人员可以把业务规则直接编码进类型系统中。

### C# 基准案例（运行时校验）
在 C# 中，我们通常对很多概念使用 string 或 int，并依赖运行时的“守卫逻辑”。
```csharp
public void SendEmail(string email) {
    if (!email.Contains('@')) throw new ArgumentException("无效邮箱");
    // ...
}
```

### Rust Newtype 方式（编译期证明）
在 Rust 中，类型本身就是合法性的证明。
```rust
pub struct Email(String);

impl Email {
    pub fn new(raw: &str) -> Result<Self, &'static str> {
        if raw.contains('@') {
            Ok(Email(raw.to_string()))
        } else {
            Err("无效邮箱格式")
        }
    }
}

// 任何接收 Email 类型的函数现在都获得了“该邮箱一定合法”的保证。
fn send_email(to: Email) { ... }
```
**零成本 (Zero-cost)：** Newtype 最终会被编译为与内部原始类型完全相同的机器码。

---

## 数组与切片

### 1. 数组 (Arrays)
固定大小，分配在栈上。
```rust
let numbers: [i32; 5] = [1, 2, 3, 4, 5];
```

### 2. 切片 (Slices)
指向内存中一段连续序列的引用。切片就像是一个内存“视图”。
```rust
let slice: &[i32] = &numbers[1..4]; // 元素 1, 2, 3
```

### 3. 以切片作为参数
在概念上非常类似于 C# 的 `ReadOnlySpan<T>`。
```rust
fn process(data: &[i32]) { ... }

// 这种写法同时支持数组和 Vec！
process(&array);
process(&vec);
```

---

## 结构体 vs 类

| **特性** | **C# 类** | **Rust 结构体** |
| :--- | :--- | :--- |
| **内存位置** | 总是分配在堆上 | 默认分配在栈上 |
| **方法定义** | 定义在类内部 | 定义在外部的 `impl` 块中 |
| **隐私修饰** | 逐个成员指定关键字 | 默认全部私有 |

### Rust 结构体示例
```rust
pub struct Person {
    pub name: String,
    age: u32, // 私有 (Private)
}

impl Person {
    pub fn new(name: String, age: u32) -> Self {
        Self { name, age }
    }
    
    pub fn get_info(&self) -> String {
        format!("{} 之龄：{}", self.name, self.age)
    }
}
```
**核心洞见：** C# 的对象总是包含引用和堆分配。Rust 的结构体则尽可能留在栈上，除非你显式使用 `Box` 等将其移动到堆上。
