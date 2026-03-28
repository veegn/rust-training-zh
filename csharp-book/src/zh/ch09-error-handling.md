[English Original](../en/ch09-error-handling.md)

# 错误处理：Result vs 异常

> **你将学到什么：** 为什么 Rust 用 `Result<T, E>` 替代异常，如何使用 `?` 操作符进行简洁的错误传播，以及显式错误处理如何消除隐藏的控制流。
>
> **难度：** 中级

在 C# 中，错误通过“异常（Exceptions）”来处理。而在 Rust 中，错误被视为类型系统的一部分。

---

## 核心设计哲学
*   **C#**：控制流是隐式的。方法可能在任意时刻抛出异常，而你必须时刻提防，并记着去 catch 它们。
*   **Rust**：控制流是显式的。如果一个函数可能失败，它**必须**返回一个 `Result<T, E>`。调用者被编译器强制要求必须同时处理成功和失败两种情况。

---

## Result 与 Option
Rust 使用两个主要的枚举来处理“非完美”的场景：
1.  **`Option<T>`**：用于表示某个值可能缺失（例如用 `None` 替代 `null`）。
2.  **`Result<T, E>`**：用于表示某个操作可能失败（例如用 `Err(e)` 替代 `throw`）。

```rust
fn get_user(id: i32) -> Result<User, String> {
    if id < 0 {
        Err("无效的用户 ID".to_string())
    } else {
        Ok(User { id })
    }
}
```

---

## `?` 操作符
`?` 操作符是 Rust 实现简洁错误处理的秘诀。它的语义是：“如果结果是 `Ok`，请把内部的值给我；如果是 `Err`，请立即带着这个错误从当前函数返回。”

```rust
fn process_user(id: i32) -> Result<(), String> {
    let user = get_user(id)?; // 如果失败，此处会直接提前返回
    println!("正在处理用户：{}", user.id);
    Ok(())
}
```
**C# 类比：** 这非常类似于调用一个会抛出异常的方法 —— 异常会自动向上传播（Bubble up）。区别在于，`?` 让这种传播变得极度清晰可见。

---

## 处理错误
你可以使用 `match`、`if let` 或者像 `unwrap_or` 这样的函数式组合器（Combinators）来处理错误。

```rust
let email = get_email(10).unwrap_or("default@example.com".to_string());

match get_user(1) {
    Ok(user) => println!("你好, {}", user.id),
    Err(e) => eprintln!("错误：{}", e),
}
```

---

## C# 开发者总结
| **概念** | **C# 方式** | **Rust 方式** |
| :--- | :--- | :--- |
| **错误类型** | `Exception` 类 | `Result<T, E>` 枚举 |
| **传播逻辑** | 自动（隐式） | `?` 操作符（显式） |
| **失败操作** | `throw new X()` | `return Err(X)` |
| **表示“空”** | `null` | `None` |

---

## 练习：传播一个错误
**挑战：** 编写一个函数，要求它读取一个文件，将其内容解析为整数，并返回结果。使用 `?` 操作符来同时传播 I/O 错误和解析错误。

```rust
fn read_id(path: &str) -> Result<i32, Box<dyn std::error::Error>> {
    let content = std::fs::read_to_string(path)?;
    let id = content.trim().parse::<i32>()?;
    Ok(id)
}
```
**关键理解：** `Result` 让错误处理成为了代码中的“一等公民”。你不能再无视错误，同时你也不需要为了处理预料之中的失败而去承受昂贵的异常解栈开销。
