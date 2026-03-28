# 17.3 精简层层嵌套的赋值结构 🟢

在 C++ 等语言中，你经常会发现自己编写了深层嵌套的 `if-else` 或 `switch` 语句，以便根据多种条件为变量赋值。这种结构有时被称为“赋值金字塔 (Assignment Pyramid)”。Rust 基于表达式的语法允许你将这些金字塔精简为更加整洁且易读的代码。

### 1. 将表达式的结果赋值给变量
在 Rust 中，几乎一切皆为表达式，都会返回一个值。这意味着你可以直接使用 `if`、`match` 甚至代码块 (`{}`) 为变量赋值。

```rust
fn main() {
    let score = 85;

    // 良好实践：使用 if 表达式进行赋值
    let grade = if score >= 90 {
        "A"
    } else if score >= 80 {
        "B"
    } else if score >= 70 {
        "C"
    } else {
        "F"
    };

    println!("成绩：{}", grade);
}
```

---

### 2. 使用 `match` 进行精简
`match` 表达式在大规模精简复杂条件逻辑（尤其是处理枚举或多个变量时）方面更加强大。

```rust
enum Status {
    Success,
    Warning(u32),
    Error(String),
}

fn main() {
    let status = Status::Warning(404);

    // 良好实践：使用 match 提取值并赋值
    let message = match status {
        Status::Success => String::from("操作成功"),
        Status::Warning(code) => format!("警告代码：{}", code),
        Status::Error(err) => format!("发生错误：{}", err),
    };

    println!("{}", message);
}
```

---

### 3. 使用闭包和 `?` 处理可选链式调用
如果你有一系列可能失败的操作（返回 `Option` 或 `Result`），可以使用闭包和 `?` 运算符来避免嵌套的 `if let` 或 `match` 语句。

```rust
fn get_user_id() -> Option<u32> { Some(123) }
fn get_user_name(id: u32) -> Option<String> { Some(String::from("Alice")) }

fn main() {
    // 良好实践：使用 and_then 串联链式调用
    let name = get_user_id()
        .and_then(|id| get_user_name(id))
        .unwrap_or_else(|| String::from("未知"));

    println!("用户名：{}", name);
}
```

---

### 4. 表达式块 (Expression Blocks)
你可以将一个代码块 `{}` 用作表达式。这对于一些需要临时变量的复杂初始化非常有用。

```rust
fn main() {
    let config_val = {
        let temp = 10 * 2;
        let offset = 5;
        temp + offset // 代码块中的最后一个表达式即为返回值
    };

    println!("配置值：{}", config_val); // 25
}
```

---

### 对于 C/C++ 开发者的总结
- **在 C++ 中**：你可能会对简单的赋值使用三元运算符 (`condition ? a : b`)，但对于更复杂的逻辑，你只能使用多个 `if-else` 代码块，并且可能会面临变量未初始化的风险。
- **在 Rust 中**：基于表达式的语法使你的代码更具声明性 (Declarative)。它还可以帮助编译器确保变量始终已初始化，从而排除了在 C/C++ 中常见的一类 Bug。

***
