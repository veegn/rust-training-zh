# 生产级错误处理模式

> **你将学到什么：** 如何在生产级 Rust 代码中通过 `thiserror` 为每一个 crate 定义统一的错误枚举，如何创建 `Result<T>` 类型别名，以及什么时候该用 `thiserror`（库）还是 `anyhow`（应用程序）。
>
> **难度：** 中级

在生产环境中的 Rust 项目中，我们尽量避免使用 `String` 或 `Box<dyn Error>` 来表示错误。取而代之的是，我们使用结构化的枚举来代表我们的代码可能遇到的每一种失败模式。

---

## Crate 级别的错误处理模式
一个常见的实践是：为你的 crate 定义一个统一的 `Error` 枚举以及对应的 `Result` 别名。

```rust
// error.rs
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("I/O 失败：{0}")]
    Io(#[from] std::io::Error), // 将 std::io::Error 自动转换为 AppError

    #[error("数据库错误：{0}")]
    Sql(String),

    #[error("校验失败：{message}")]
    Validation { message: String },
}

pub type Result<T> = std::result::Result<T, AppError>;
```

### 核心优势
1.  **更干净的函数签名**：你的函数只需返回 `Result<User>`，而非冗长的 `Result<User, AppError>`。
2.  **自动传播**：使用 `#[from]` 可以让 `?` 操作符自动将底层错误（如 `io::Error`）转换为你的高层业务错误 `AppError`。

---

## `thiserror` vs `anyhow`
在任何 Rust 项目开始时，决定使用这两个 crate 中的哪一个通常是你的第一个决策。

| **Crate** | **最适合** | **主要哲学** |
| :--- | :--- | :--- |
| **`thiserror`** | **库 (Libraries)** | 适用于会被他人调用的代码，因为调用者需要明确处理特定的错误变体 (`match`)。 |
| **`anyhow`** | **应用程序 (Applications)** | 适用于最终生成的二进制可执行程序。它提供了一个不透明且极易添加上下文的 `Error` 类型。 |

### 在应用程序中使用 `anyhow`
```rust
use anyhow::{Context, Result};

fn main() -> Result<()> {
    // .context() 会为错误添加一个易于阅读的人类语言说明
    let config = std::fs::read_to_string("config.toml")
        .context("缺少 config.toml 配置文件")?; 
    Ok(())
}
```

---

## C# 开发者总结表
*   **`thiserror`** 类似于在你的 C# 库中定义了各种自定义的 `Exception` 类。
*   **`anyhow`** 类似于在你的 `Main` 方法中 catch 了所有 `Exception` 并为其包装一个描述性的文案：`throw new Exception("...", innerException)`。

---

## 练习：设计一个 Crate 的 Error 枚举
**挑战：** 使用 `thiserror` 创建一个 `RegistrationError` 枚举，并支持 `DuplicateEmail`（包含 email 字符串）和 `DatabaseError`（包含 sqlx 错误）。

```rust
#[derive(Error, Debug)]
pub enum RegistrationError {
    #[error("邮箱 {0} 已被占用")]
    DuplicateEmail(String),
    #[error("数据库连接失败")]
    Database(#[from] sqlx::Error),
}
```
**关键理解：** 一个设计良好的错误枚举本身即是你的 API 文档的一部分。它告诉了调用方到底什么地方会出错，并赋予了他们优雅处理这些错误的能力。
