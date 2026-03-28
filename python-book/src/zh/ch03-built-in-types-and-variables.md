[English Original](../en/ch03-built-in-types-and-variables.md)

# 3. 内建类型与变量 🟢

> **你将学到：**
> - 变量默认不可变特性与显式 `mut` 关键字
> - 原生数值类型对比 Python 的大整数 `int`
> - `String` 与 `&str`（“拥有”与“借用”的区别）
> - 字符串格式化与 Debug 打印
> - Rust 强制性的、由编译器检查的类型注解

## 变量与可变性

### Python 变量声明
```python
# Python — 一切皆可变，动态类型
count = 0          # 可变，类型推导为 int
count = 10         # ✅ 没问题
count = "hello"    # ✅ 类型可以随意改变
```

### Rust 变量声明
```rust
// Rust — 默认不可变，静态类型
let count = 0;           // 不可变，类型推导为 i32
// count = 10;           // ❌ 编译错误：不能对不可变变量进行二次赋值

let mut count = 0;       // 加上 mut 变为可变
count = 10;              // ✅ 没问题
// count = "hello";      // ❌ 类型依然不能改变

const MAX_SIZE: usize = 1024; // 编译器强制执行的常量
```

### 心智模型转变

在 Python 中，变量是贴在堆对象上的**标签**。在 Rust 中，变量是拥有其值的**存储位置/容器**，并且具有**所有权**。

**变量遮蔽 (Shadowing)** 是 Rust 的一大特色：
```rust
let input = "42";              // 类型为 &str
let input = input.parse::<i32>().unwrap(); // 变为 i32：这是个新变量，只是名字相同
let input = input * 2;         // 结果 84
```

---

## 常用基础类型对比

### 数值类型

| Python | Rust | 说明 |
|--------|------|-------|
| `int` | `i8`..`i128`, `isize` | Rust 整数有固定的大小 |
| `int` (无符号) | `u8`..`u128`, `usize` | 显式的无符号类型 |
| `float` | `f32`, `f64` | Python 只有 64 位浮点数 |
| `bool` | `bool` | 概念一致 |

### 索引类型 (usize)

```rust
// usize 用于索引：其大小取决于操作系统架构（32位或64位）
let length: usize = vec![1, 2, 3].len();
let index: usize = 0; // 数组索引必须是 usize 

// 混合 i32 和 usize 时需要显式转换：
let i: i32 = 5;
let item = vec[i as usize]; // ✅ 显式转换
```

---

## 两种字符串：String 与 &str

这是让 Python 开发者最困惑的地方。Python 只有一种字符串，而 Rust 有两种。

### Rust 字符串类型
```rust
// 1. &str (字符串切片) — 借用的、不可变的“视图”
let name: &str = "Alice"; // 指向程序内硬编码的数据

// 2. String (拥有所有权的字符串) — 堆分配、可增长
let mut greeting = String::from("你好, "); // 属于你，可以修改
greeting.push_str(name);
```

### 简单判断准则：
- **&str** = “我只是在看别人拥有的字符串”（只读视图）。
- **String** = “我拥有这串数据，我可以随便改它”。

---

## 打印与格式化

### 基础输出
```rust
println!("你好，世界！");
println!("姓名: {} 年龄: {}", name, age); // 位置占位符
println!("姓名: {name}, 年龄: {age}");   // 内联变量 (类似 f-string)
```

### Debug 打印
```rust
// 使用 {:?} 或 {:#?} (相当于 repr() 或 pprint)
println!("{:?}", vec![1, 2, 3]);

#[derive(Debug)] // 让你的结构体支持打印
struct Point { x: f64, y: f64 }
```

---

## 类型注解

在 Python 中，类型提示主要是给 IDE 看的。在 Rust 中，类型是**程序的核心约束** —— 编译器利用它们来消灭空指针错误和内存争用。

---

## 练习

<details>
<summary><strong>🏋️ 练习：温度转换器</strong> (点击展开)</summary>

**挑战**：实现一个函数 `celsius_to_fahrenheit(c: f64) -> f64` 以及一个分类函数 `classify(temp_f: f64) -> &'static str`。分别打印 0, 20, 35 摄氏度对应的华氏度及其体感（cold/mild/hot）。

<details>
<summary>参考答案</summary>

```rust
fn celsius_to_fahrenheit(c: f64) -> f64 {
    c * 1.8 + 32.0
}

fn classify(f: f64) -> &'static str {
    if f < 50.0 { "寒冷" }
    else if f < 77.0 { "温和" }
    else { "炎热" }
}

fn main() {
    for c in [0.0, 20.0, 35.0] {
        let f = celsius_to_fahrenheit(c);
        println!("{:.1}°C = {:.1}°F — {}", c, f, classify(f));
    }
}
```
</details>
</details>

***
