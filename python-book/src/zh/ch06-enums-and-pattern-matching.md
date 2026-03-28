[English Original](../en/ch06-enums-and-pattern-matching.md)

# 6. 枚举与模式匹配 🟡

> **你将学到：**
> - 携带数据的 Rust 枚举 (Enums) vs Python 的 `Union` 联合类型
> - 穷尽模式匹配 (Exhaustive `match`) vs Python 的 `match/case` 语法
> - `Option<T>`：替代 `None` 的类型安全方案
> - 枚举在 Rust 中如何替代多种 Python 设计模式 (标签联合、状态常量等)

## 代数数据类型 vs 联合类型

Python 3.10 引入了 `match` 和类型联合 (`Union[Circle, Rectangle]`)。Rust 的枚举则更进一步：每个成员都可以携带**完全不同结构**的数据，并且编译器会**强制**你必须处理每一种可能性。

### Rust 枚举 — 携带数据的变体
```rust
enum Shape {
    Circle(f64),                      // 圆形：只存半径
    Rectangle(f64, f64),              // 矩形：存宽、高
    Triangle { base: f64, height: f64 }, // 三角形：存底、高 (命名参数风格)
}

fn area(shape: &Shape) -> f64 {
    match shape {
        Shape::Circle(r) => std::f64::consts::PI * r * r,
        Shape::Rectangle(w, h) => w * h,
        Shape::Triangle { base, height } => 0.5 * base * height,
        // ❌ 如果少处理了任何一个成员，代码将无法编译通过。
    }
}
```

---

## 穷尽模式匹配 (Exhaustive match)

在 Python 中，如果在 `match` 里漏掉了一个 `case`，它会默默返回 `None`。而在 Rust 中，**如果没处理完各种 case，程序根本没法跑**。

```rust
enum Status { Pending, Active, Closed }

fn describe(s: Status) -> &'static str {
    match s {
        Status::Pending => "等待中...",
        Status::Active => "进行中！",
        Status::Closed => "已结束",
        // 这里不需要通配符 (_)，因为编译器知道总共就这三种状态，且都处理了。
    }
}
```

### 模式匹配特性：
- **区间匹配**：`1..=10 => println!("小数字")`
- **多值匹配**：`1 | 2 | 3 => println!("低级别")`
- **守护模式 (Guards)**：`t if t > 100 => println!("沸腾")`

---

## 用于防止 None 的 Option

`Option<T>` 大概是给 Python 开发者的最重要的知识点。它把 `None` 变成了一个明确的枚举变体，并**迫使**你在进行任何操作前，都必须判断该值到底存不存在。

```rust
fn find_user(id: i32) -> Option<String> {
    if id == 1 { Some("张三".to_string()) } else { None }
}

let user = find_user(1);
// user.to_uppercase(); // ❌ 报错：user 是 Option<String> 而不是 String！

// 你【必须】处理它：
match user {
    Some(name) => println!("你好, {name}"),
    None => println!("找不到用户"),
}

// 或者是简便写法 if let:
if let Some(name) = find_user(1) {
    println!("{name}");
}
```

---

## 练习

<details>
<summary><strong>🏋️ 练习：消息处理器</strong> (点击展开)</summary>

**挑战**：定义一个枚举 `Message`，包含三种变体：`Quit` (无数据)、`Move { x: i32, y: i32 }` (坐标点) 以及 `Write(String)` (文字内容)。实现一个函数 `process(m: Message)` 根据消息类型打印出不同的操作逻辑。

<details>
<summary>参考答案</summary>

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
}

fn process(m: Message) {
    match m {
        Message::Quit => println!("正在停机..."),
        Message::Move { x, y } => println!("向 ({x}, {y}) 移动"),
        Message::Write(text) => println!("收到文本: {text}"),
    }
}

fn main() {
    process(Message::Move { x: 10, y: 20 });
    process(Message::Write("你好, Rust".to_string()));
    process(Message::Quit);
}
```

</details>
</details>

***
