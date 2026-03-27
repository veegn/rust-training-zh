# 空值安全与穷尽匹配

> **你将学到什么：** 为什么 C# 的 `switch` 表达式可能悄悄漏掉分支，而 Rust 的 `match` 会在编译期抓出所有遗漏的分支；`Option<T>` 与 `Nullable<T>` 在空值安全上的区别；以及如何用 `Result<T, E>` 表达自定义错误。
>
> **难度：** 🟡 中级

## 缺陷：不完整的 Switch
在 C# 中，`switch` 表达式看起来是穷尽的，但实际上在编译期并不能得到绝对的保障。

### C# Switch (仅提供警告)
```csharp
public enum Status { Ok, NotFound, Error }

public string Handle(Status s) => s switch {
    Status.Ok => "Success",
    Status.NotFound => "Not Found",
    // 漏掉了 'Error' 分支！
    // 尽管会产生 CS8524 编译警告，但程序依然能成功编译。
    // 在运行时，如果传入了 Error，则会抛出 SwitchExpressionException。
};
```

### Rust Match (编译期报错)
在 Rust 中，如果你漏掉了一个变体，你的程序**根本无法编译**。在进行大规模重构时，编译器就是你最稳固的安全防线。
```rust
enum Status { Ok, NotFound, Error }

fn handle(s: Status) -> &'static str {
    match s {
        Status::Ok => "Success",
        Status::NotFound => "Not Found",
        // 报错 (ERROR): 模式未穷尽：`Error` 未被覆盖
    }
}
```

---

## 空值安全：`Option<T>`
Rust 没有 `null` 关键字。转而使用 `Option<T>` 枚举来明确地表示“某个值可能不存在”。

| **特性** | **C# 可空类型 (Nullable)** | **Rust `Option<T>`** |
| :--- | :--- | :--- |
| **机制** | `T?` 或 `Nullable<T>` | `Option<T>` 枚举 |
| **安全性** | 静态警告（取决于配置） | **编译器强制检查** |
| **访问** | `obj?.Prop` 或 `.Value` | `match`、`if let` 或组合器 (Combinators) |

### 组合器 (Rust 版的 `?.` 运算符)
在 C# 中，你习惯使用 `?.` 来链式检查 null。而在 Rust 中，你需要使用 `and_then` 和 `map`。

```rust
// C# 写法
string? name = user?.Address?.City?.ToUpper();

// Rust 写法 (对应的组合器链)
let name = user.and_then(|u| u.address.as_ref())
               .and_then(|a| a.city.as_ref())
               .map(|c| c.to_uppercase());
```

---

## 错误处理：`Result<T, E>`
Rust 并不使用异常 (Exception)，而是通过 `Result` 枚举来处理错误数据。

```rust
fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err("除数为零".to_string())
    } else {
        Ok(a / b)
    }
}
```

### `?` 运算符
`?` 运算符是一种非常简洁的“错误传播”方式，类似于一个自动触发且能精准捕捉类型的 `try-catch`。
```rust
fn total_score() -> Result<i32, String> {
    let s1 = get_score("Math")?;    // 如果出错，就会提前返回 (Early return)
    let s2 = get_score("Science")?; // 否则继续
    Ok(s1 + s2)
}
```

---

## 练习：Option 组合器
**挑战：** 尝试将一段层层嵌套的 C# null 判断重写为单条 Rust `Option` 链。

```rust
fn get_city_name(user: Option<&User>) -> String {
    user.and_then(|u| u.address.as_ref())
        .and_then(|a| a.city.as_ref())
        .map(|c| c.to_uppercase())
        .unwrap_or_else(|| "未知城市".to_string())
}
```
**关键点：** `and_then` 允许你在处理层层嵌套的可选数据时，无需缩进，也不需要多次手写 `match`，代码逻辑极其清晰。
