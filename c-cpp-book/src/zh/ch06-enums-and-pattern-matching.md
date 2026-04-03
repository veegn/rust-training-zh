[English Original](../en/ch06-enums-and-pattern-matching.md)

# Rust 枚举类型 (Enums)

> **你将学到：** 作为可辨识联合 (Discriminated Unions) 的 Rust 枚举（真正好用的标签联合）、用于穷尽性模式匹配的 `match`，以及枚举如何通过编译器强制的安全机制取代 C++ 类层次结构和 C 的标签联合。

- 枚举类型是可辨识联合，即它们是多种可能类型的“求和类型 (Sum Type)”，带有一个用于标识具体变体 (Variant) 的标签。
    - 对于 C 开发者：Rust 中的枚举可以携带数据（真正好用的标签联合 —— 编译器会追踪哪个变体是活跃的）。
    - 对于 C++ 开发者：Rust 的枚举类似于 `std::variant`，但支持穷尽性模式匹配，没有 `std::get` 异常，也没有 `std::visit` 那样繁琐的样板代码。
    - `enum` 的大小由其可能的最大变体决定。各个变体之间没有关联，并且可以拥有完全不同的类型。
    - `enum` 类型是 Rust 最强大的特性之一 —— 它们可以取代 C++ 中复杂的类继承体系（详见后续案例研究章节）。
```rust
fn main() {
    enum Numbers {
        Zero,
        SmallNumber(u8),
        BiggerNumber(u32),
        EvenBiggerNumber(u64),
    }
    let a = Numbers::Zero;
    let b = Numbers::SmallNumber(42);
    let c : Numbers = a; // OK -- a 的类型是 Numbers
    let d : Numbers = b; // OK -- b 的类型是 Numbers
}
```

---

# Rust match 语句
- Rust 的 `match` 相当于“加强版”的 C 语言 `switch` 语句：
    - `match` 可用于对简单数据类型、`struct`、`enum` 进行模式匹配。
    - `match` 语句必须是**穷尽的 (Exhaustive)**，即它们必须复盖给定类型的所有可能情况。`_` 可以用作捕获“所有其他情况”的通配符。
    - `match` 可以产生一个值，但所有分支 (`=>`) 必须返回相同类型的值。

```rust
fn main() {
    let x = 42;
    // 在这里，_ 复盖了除明确列出以外的所有数字
    let is_secret_of_life = match x {
        42 => true, // 返回布尔值
        _ => false, // 返回布尔值
        // 下行无法编译，因为返回类型不是布尔值
        // _ => 0  
    };
    println!("{is_secret_of_life}");
}
```

# Rust match 语句的应用
- `match` 支持范围匹配、布尔过滤器以及 `if` 守卫 (Guard) 语句。
```rust
fn main() {
    let x = 42;
    match x {
        // 注意：..=41 表示包含 41 的闭区间范围
        0..=41 => println!("小于生命之秘"),
        42 => println!("生命之秘"),
        _ => println!("大于生命之秘"),
    }
    let y = 100;
    match y {
        100 if x == 43 => println!("y 百分之百不是生命之秘"),
        100 if x == 42 => println!("y 百分之百是生命之秘"),
        _ => (),    // 什么都不做
    }
}
```

---

# Rust match 语句与枚举
- `match` 和 `enum` 经常配合使用：
    - `match` 语句可以将其包含的值“绑定”到一个变量上。如果不对值感兴趣，请使用 `_`。
    - `matches!` 宏可用于测试是否匹配特定的变体。
```rust
fn main() {
    enum Numbers {
        Zero,
        SmallNumber(u8),
        BiggerNumber(u32),
        EvenBiggerNumber(u64),
    }
    let b = Numbers::SmallNumber(42);
    match b {
        Numbers::Zero => println!("零"),
        Numbers::SmallNumber(value) => println!("小数字 {value}"),
        Numbers::BiggerNumber(_) | Numbers::EvenBiggerNumber(_) => println!("较大的数字或更大的数字"),
    }
    
    // 针对特定变体进行布尔判断
    if matches!(b, Numbers::Zero | Numbers::SmallNumber(_)) {
        println!("匹配到了 零 或者 小数字");
    }
}
```

# Rust match 语句与解构
- `match` 还可以针对解构 (Destructuring) 和切片 (Slices) 执行匹配：
```rust
fn main() {
    struct Foo {
        x: (u32, bool),
        y: u32
    }
    let f = Foo {x: (42, true), y: 100};
    match f {
        // 将 x 的值捕获到一个叫做 tuple 的变量中
        Foo{y: 100, x : tuple} => println!("匹配到了 x: {tuple:?}"),
        _ => ()
    }
    let a = [40, 41, 42];
    match a {
        // 切片的最后一个元素必须是 42。使用 @ 进行匹配绑定
        [rest @ .., 42] => println!("剩余元素: {rest:?}"),
        // 切片的第一个元素必须是 42。使用 @ 进行匹配绑定
        [42, rest @ ..] => println!("剩余元素: {rest:?}"),
        _ => (),
    }
}
```

---

# 练习：利用 match 和 enum 实现加减法计算

🟢 **入门级**

- 编写一个函数，对 64 位无符号整数执行算术运算。
- **第一步**：定义一个表示操作的枚举：
```rust
enum Operation {
    Add(u64, u64),
    Subtract(u64, u64),
}
```
- **第二步**：定义一个表示结果的枚举：
```rust
enum CalcResult {
    Ok(u64),                    // 成功结果
    Invalid(String),            // 无效操作的错误信息
}
```
- **第三步**：实现 `calculate(op: Operation) -> CalcResult` 函数
    - 对于 `Add`：返回 `Ok(sum)`。
    - 对于 `Subtract`：如果第一个数 >= 第二个数，返回 `Ok(difference)`，否则返回 `Invalid("Underflow")`。
- **提示**：在函数中使用模式匹配：
```rust
match op {
    Operation::Add(a, b) => { /* 你的代码 */ },
    Operation::Subtract(a, b) => { /* 你的代码 */ },
}
```

<details><summary>参考答案 (点击展开)</summary>

```rust
enum Operation {
    Add(u64, u64),
    Subtract(u64, u64),
}

enum CalcResult {
    Ok(u64),
    Invalid(String),
}

fn calculate(op: Operation) -> CalcResult {
    match op {
        Operation::Add(a, b) => CalcResult::Ok(a + b),
        Operation::Subtract(a, b) => {
            if a >= b {
                CalcResult::Ok(a - b)
            } else {
                CalcResult::Invalid("Underflow".to_string())
            }
        }
    }
}

fn main() {
    match calculate(Operation::Add(10, 20)) {
        CalcResult::Ok(result) => println!("10 + 20 = {result}"),
        CalcResult::Invalid(msg) => println!("错误: {msg}"),
    }
    match calculate(Operation::Subtract(5, 10)) {
        CalcResult::Ok(result) => println!("5 - 10 = {result}"),
        CalcResult::Invalid(msg) => println!("错误: {msg}"),
    }
}
// 输出示例:
// 10 + 20 = 30
// 错误: Underflow
```

</details>

---

# Rust 关联方法 (Associated Methods)
- `impl` 块可以为 `struct`、`enum` 等类型定义关联方法。
    - 方法可以可选地接收 `self` 作为参数。`self` 在概念上类似于 C 语言中作为第一个参数传递的结构体指针，或者是 C++ 中的 `this`。
    - 对 `self` 的引用可以是不可变的（默认：`&self`）、可变的（`&mut self`），或者获取所有权的（`self`）。
    - `Self` 关键字可以作为类型名的缩写。
```rust
struct Point {x: u32, y: u32}
impl Point {
    fn new(x: u32, y: u32) -> Self {
        Point {x, y}
    }
    fn increment_x(&mut self) {
        self.x += 1;
    }
}
fn main() {
    let mut p = Point::new(10, 20);
    p.increment_x();
}
```

# 练习：Point 的相加与转换

🟡 **中级** —— 此练习旨在加深对方法签名中“移动 (Move)”与“借用 (Borrow)”区别的理解。
- 为 `Point` 结构体实现以下关联方法：
    - `add()`：接收另一个 `Point`，并原地增加当前点的 x 和 y 值（提示：使用 `&mut self`）。
    - `transform()`：消耗现有的 `Point`（提示：使用 `self`），并返回一个新的 `Point`，其 x 和 y 值为原值的平方。

<details><summary>参考答案 (点击展开)</summary>

```rust
struct Point { x: u32, y: u32 }

impl Point {
    fn new(x: u32, y: u32) -> Self {
        Point { x, y }
    }
    fn add(&mut self, other: &Point) {
        self.x += other.x;
        self.y += other.y;
    }
    fn transform(self) -> Point {
        Point { x: self.x * self.x, y: self.y * self.y }
    }
}

fn main() {
    let mut p1 = Point::new(2, 3);
    let p2 = Point::new(10, 20);
    p1.add(&p2);
    println!("相加之后: x={}, y={}", p1.x, p1.y);           // x=12, y=23
    let p3 = p1.transform();
    println!("转换之后: x={}, y={}", p3.x, p3.y);           // x=144, y=529
    // p1 现在无法再被访问了 —— transform() 已经消耗了它的所有权
}
```

</details>

---
