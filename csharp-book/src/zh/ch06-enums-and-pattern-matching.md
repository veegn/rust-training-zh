[English Original](../en/ch06-enums-and-pattern-matching.md)

# 枚举与模式匹配

> **你将学到什么：** Rust 的代数数据类型（可携带数据的 enum）与 C# 中受限的判别联合的区别，带穷尽检查的 `match` 表达式、守卫条件，以及嵌套模式解构。
>
> **难度：** 🟡 中级

## 代数数据类型 (Enums)
Rust 的枚举比 C# 中的枚举要强大得多。它们是**代数数据类型 (ADTs)**，意味着每一个变体（variant）都可以携带自己特有的数据。

### C# vs Rust 枚举
在 C# 中，一个枚举只是一组具名的整数常量。要想在对应的枚举分支上关联数据，通常需要一套类或 Record 的继承体系。

```csharp
// 在 C# 中需要通过继承来保存不同分支的数据
public abstract record Shape;
public record Circle(double Radius) : Shape;
public record Rectangle(double Width, double Height) : Shape;
```

而在 Rust 中，这只是一个单一且紧凑的类型：
```rust
pub enum Shape {
    Circle { radius: f64 },
    Rectangle { width: f64, height: f64 },
    Triangle { base: f64, height: f64 },
}
```

---

## 使用 `match` 进行模式匹配
`match` 表达式是消费枚举变体的主要方式。与 C# 的 `switch` 不同，Rust 的 `match` 是**强制穷尽的** —— 如果你漏掉了一个变体，编译器将拒绝通过你的代码。

```rust
impl Shape {
    pub fn area(&self) -> f64 {
        match self {
            Shape::Circle { radius } => 3.14 * radius * radius,
            Shape::Rectangle { width, height } => width * height,
            Shape::Triangle { base, height } => 0.5 * base * height,
        }
    }
}
```

### 高级模式匹配
Rust 的模式匹配功能远不止于简单的等值判断：
*   **守卫语句 (Match Guards)**：`match x { n if n < 0 => ... }`
*   **范围匹配**：`match age { 0..=12 => "儿童", ... }`
*   **解构**：直接从嵌套、深层的结构中提取变量值。

---

## 对比总结

| **特性** | **C# 枚举 / Record** | **Rust 枚举** |
| :--- | :--- | :--- |
| **数据关联** | 需要继承/多态来变通 | 变体内建数据支持 |
| **穷尽性检查** | 可选（存在运行时风险） | **强制**（编译期安全保证） |
| **内存布局** | 堆分配（针对 Class 类型） | 栈分配（极度优化） |
| **模式匹配** | `switch` (C# 8+ 起大幅改善) | `match` (核心语言语法) |

---

## 练习：命令解析器
**挑战：** 建立一个小型的 CLI 命令系统。要求 `Quit`、`Echo(String)` 和 `Move { x, y }` 为不同的枚举变体。

```rust
enum Command {
    Quit,
    Echo(String),
    Move { x: i32, y: i32 },
}

fn execute(cmd: Command) {
    match cmd {
        Command::Quit => println!("再见！"),
        Command::Echo(s) => println!("{s}"),
        Command::Move { x, y } => println!("正在移动到 {x}, {y}"),
    }
}
```
**关键点：** 枚举允许你将相关的、但在结构上各异的数据集中处理。而使用 `match` 可以确保你完美覆盖并处理了所有的具体情况。
