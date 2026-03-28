[English Original](../en/ch04-control-flow.md)

# 控制流与函数

> **你将学到什么：** Rust 与 C# 中函数和方法的差异、表达式与语句之间的关键区别、`if`/`match`/`loop`/`while`/`for` 的写法，以及 Rust 的表达式导向设计如何让三元运算符变得不再必要。
>
> **难度：** 🟢 初级

## 函数与方法

### C# 函数声明
在 C# 中，方法必须位于类或结构体中。
```csharp
public class Calculator {
    public int Add(int a, int b) {
        return a + b;
    }
}
```

### Rust 函数声明
Rust 支持独立的顶层函数。
```rust
fn add(a: i32, b: i32) -> i32 {
    a + b // 无需 'return' 关键字来返回最后一个表达式的值
}

fn main() {
    let result = add(5, 3);
    println!("5 + 3 = {result}");
}
```

---

## 表达式与语句 (非常重要！)
这是 Rust 控制流中最核心的思想转变。

*   **语句 (Statements)**：执行一个动作但不返回值。它们通常以分号 `;` 结尾。
*   **表达式 (Expressions)**：求得一个值。它们**不使用**分号。

```rust
fn get_value(condition: bool) -> i32 {
    if condition {
        42 // 表达式（无分号）
    } else {
        0  // 表达式（无分号）
    }
}
```
在 Rust 中，整个 `if` 块及其后续分支其实是一个返回值的表达式。

---

## 控制流基础

### If 表达式
由于 `if` 本身是一个表达式，它完美替代了 C# 中的三元运算符 (`? :`)。
```rust
let x = 5;
let message = if x > 10 { "大" } else { "小" };
```

### 循环 (Loops)
Rust 提供三种主要循环：
1.  **`loop`**：真正的无限循环。
2.  **`while`**：条件为真时执行。
3.  **`for`**：遍历范围或集合。

```rust
// 指定范围的 for 循环
for i in 0..5 { // 0 到 4
    println!("{i}");
}

// 带返回值的 loop
let result = loop {
    counter += 1;
    if counter == 10 {
        break counter * 2; // 从循环中直接返回值
    }
};
```

---

## 练习：温度转换器
**挑战：** 将一段 C# 风格的 `switch` 温度转换逻辑翻译为 Rust 惯用的枚举和表达式结合的方式。

```rust
enum TempUnit { Celsius, Fahrenheit }

fn convert(value: f64, from: TempUnit, to: TempUnit) -> f64 {
    let celsius = match from {
        TempUnit::Fahrenheit => (value - 32.0) * 5.0 / 9.0,
        TempUnit::Celsius => value,
    };
    match to {
        TempUnit::Fahrenheit => celsius * 9.0 / 5.0 + 32.0,
        TempUnit::Celsius => celsius,
    }
}
```
**关键点：** 将 `match` 作为表达式使用，可以使代码更加简洁紧凑，并有效避免遗漏某些返回情况。
