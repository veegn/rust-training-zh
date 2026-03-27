## Crate-Level Error Types and Result Aliases | Crate 级错误类型与 Result 别名

> **What you'll learn:** The production pattern of defining a per-crate error enum with `thiserror`,
> creating a `Result<T>` type alias, and when to choose `thiserror` (libraries) vs `anyhow` (applications).
>
> **你将学到什么：** 如何在生产代码中为每个 crate 定义基于 `thiserror` 的错误枚举，
> 如何创建 `Result<T>` 类型别名，以及什么时候该用 `thiserror`（库）还是 `anyhow`（应用）。
>
> **Difficulty:** Intermediate
>
> **难度：** 中级

A critical pattern for production Rust: define a per-crate error enum and a `Result` type alias to eliminate boilerplate.

在生产级 Rust 项目里，一个非常关键的模式是：为整个 crate 定义统一错误枚举，并配一个 `Result` 类型别名，以减少样板代码。

### The Pattern | 这种模式的基本写法
```rust
// src/error.rs
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Validation error: {message}")]
    Validation { message: String },

    #[error("Not found: {entity} with id {id}")]
    NotFound { entity: String, id: String },
}

/// Crate-wide Result alias - every function returns this
pub type Result<T> = std::result::Result<T, AppError>;
```

### Usage Throughout Your Crate | 在整个 Crate 中使用
```rust
use crate::error::{AppError, Result};

pub async fn get_user(id: Uuid) -> Result<User> {
    let user = sqlx::query_as!(User, "SELECT * FROM users WHERE id = $1", id)
        .fetch_optional(&pool)
        .await?;  // sqlx::Error -> AppError::Database via #[from]

    user.ok_or_else(|| AppError::NotFound {
        entity: "User".into(),
        id: id.to_string(),
    })
}

pub async fn create_user(req: CreateUserRequest) -> Result<User> {
    if req.name.trim().is_empty() {
        return Err(AppError::Validation {
            message: "Name cannot be empty".into(),
        });
    }
    // ...
}
```

### C# Comparison | 与 C# 的对比
```csharp
// C# equivalent pattern
public class AppException : Exception
{
    public string ErrorCode { get; }
    public AppException(string code, string message) : base(message)
    {
        ErrorCode = code;
    }
}

// But in C#, callers don't know what exceptions to expect!
// In Rust, the error type is in the function signature.
```

### Why This Matters | 为什么这个模式重要
- **`thiserror`** generates `Display` and `Error` impls automatically
- **`thiserror`** 能自动生成 `Display` 和 `Error` 实现
- **`#[from]`** enables the `?` operator to convert library errors automatically
- **`#[from]`** 让 `?` 操作符能够自动把底层库错误转换成你的统一错误类型
- The `Result<T>` alias means every function signature is clean: `fn foo() -> Result<Bar>`
- `Result<T>` 别名让函数签名更干净，比如 `fn foo() -> Result<Bar>`
- **Unlike C# exceptions**, callers see all possible error variants in the type
- **不同于 C# 异常**，调用方可以从类型上直接看到可能出现的错误类别

### thiserror vs anyhow: When to Use Which | `thiserror` 与 `anyhow`：什么时候用哪个

Two crates dominate Rust error handling. Choosing between them is the first decision you'll make:

Rust 错误处理里最常见的两个 crate 是 `thiserror` 和 `anyhow`。在很多项目里，第一步就是决定该选谁：

| | `thiserror` | `anyhow` |
|---|---|---|
| **Purpose** | Define structured error types for **libraries** | Quick error handling for **applications** |
| **用途** | 为**库**定义结构化错误类型 | 为**应用程序**快速处理错误 |
| **Output** | Custom enum you control | Opaque `anyhow::Error` wrapper |
| **输出类型** | 你自己控制的自定义枚举 | 不透明的 `anyhow::Error` 包装器 |
| **Caller sees** | All error variants in the type | Just `anyhow::Error` - opaque |
| **调用方看到的内容** | 类型中列出的全部错误变体 | 只看到 `anyhow::Error`，不透明 |
| **Best for** | Library crates, APIs, any code with consumers | Binaries, scripts, prototypes, CLI tools |
| **适用场景** | 库 crate、API、会被别人调用的代码 | 可执行程序、脚本、原型、CLI 工具 |
| **Downcasting** | `match` on variants directly | `error.downcast_ref::<MyError>()` |
| **向下转型** | 可直接 `match` 错误变体 | 通过 `error.downcast_ref::<MyError>()` |

```rust
// thiserror - for LIBRARIES (callers need to match on error variants)
use thiserror::Error;

#[derive(Error, Debug)]
pub enum StorageError {
    #[error("File not found: {path}")]
    NotFound { path: String },

    #[error("Permission denied: {0}")]
    PermissionDenied(String),

    #[error(transparent)]
    Io(#[from] std::io::Error),
}

pub fn read_config(path: &str) -> Result<String, StorageError> {
    std::fs::read_to_string(path).map_err(|e| match e.kind() {
        std::io::ErrorKind::NotFound => StorageError::NotFound { path: path.into() },
        std::io::ErrorKind::PermissionDenied => StorageError::PermissionDenied(path.into()),
        _ => StorageError::Io(e),
    })
}
```

```rust
// anyhow - for APPLICATIONS (just propagate errors, don't define types)
use anyhow::{Context, Result};

fn main() -> Result<()> {
    let config = std::fs::read_to_string("config.toml")
        .context("Failed to read config file")?;

    let port: u16 = config.parse()
        .context("Failed to parse port number")?;

    println!("Listening on port {port}");
    Ok(())
}
// anyhow::Result<T> = Result<T, anyhow::Error>
// .context() adds human-readable context to any error
```

```csharp
// C# comparison:
// thiserror ~= defining custom exception classes with specific properties
// anyhow ~= catching Exception and wrapping with message:
//   throw new InvalidOperationException("Failed to read config", ex);
```

**Guideline**: If your code is a **library** (other code calls it), use `thiserror`. If your code is an **application** (the final binary), use `anyhow`. Many projects use both - `thiserror` for the library crate's public API, `anyhow` in the `main()` binary.

**经验建议：** 如果你的代码是**库**（会被别人调用），优先用 `thiserror`。如果你的代码是**应用程序**（最终产出的二进制程序），优先用 `anyhow`。很多项目会同时使用两者：对外的库 API 使用 `thiserror`，而 `main()` 或 CLI 层使用 `anyhow`。

### Error Recovery Patterns | 错误恢复模式

C# developers are used to `try/catch` blocks that recover from specific exceptions. Rust uses combinators on `Result` for the same purpose:

C# 开发者习惯用 `try/catch` 针对特定异常做恢复。Rust 则通常通过 `Result` 上的组合器和 `match` 来表达同样的逻辑：

```rust
use std::fs;

// Pattern 1: Recover with a fallback value
let config = fs::read_to_string("config.toml")
    .unwrap_or_else(|_| String::from("port = 8080"));  // default if missing

// Pattern 2: Recover from specific errors, propagate others
fn read_or_create(path: &str) -> Result<String, std::io::Error> {
    match fs::read_to_string(path) {
        Ok(content) => Ok(content),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            let default = String::from("# new file");
            fs::write(path, &default)?;
            Ok(default)
        }
        Err(e) => Err(e),  // propagate permission errors, etc.
    }
}

// Pattern 3: Add context before propagating
use anyhow::Context;

fn load_config() -> anyhow::Result<Config> {
    let text = fs::read_to_string("config.toml")
        .context("Failed to read config.toml")?;
    let config: Config = toml::from_str(&text)
        .context("Failed to parse config.toml")?;
    Ok(config)
}

// Pattern 4: Map errors to your domain type
fn parse_port(s: &str) -> Result<u16, AppError> {
    s.parse::<u16>()
        .map_err(|_| AppError::Validation {
            message: format!("Invalid port: {s}"),
        })
}
```

```csharp
// C# equivalents:
try { config = File.ReadAllText("config.toml"); }
catch (FileNotFoundException) { config = "port = 8080"; }  // Pattern 1

try { /* ... */ }
catch (FileNotFoundException) { /* create file */ }        // Pattern 2
catch { throw; }                                            // re-throw others
```

**When to recover vs propagate:**
- **Recover** when the error has a sensible default or retry strategy
- **在有合理默认值或重试策略时恢复**
- **Propagate with `?`** when the *caller* should decide what to do
- **当应该由调用方决定怎么处理时，用 `?` 继续向上传播**
- **Add context** (`.context()`) at module boundaries to build an error trail
- **在模块边界补充上下文**（`.context()`），把错误链条补完整

---

## Exercises | 练习

<details>
<summary><strong>Exercise: Design a Crate Error Type | 练习：设计一个 Crate 错误类型</strong> (click to expand / 点击展开)</summary>

You're building a user registration service. Design the error type using `thiserror`:

你正在实现一个用户注册服务。请使用 `thiserror` 设计错误类型：

1. Define `RegistrationError` with variants: `DuplicateEmail(String)`, `WeakPassword(String)`, `DatabaseError(#[from] sqlx::Error)`, `RateLimited { retry_after_secs: u64 }`
1. 定义 `RegistrationError`，包含这些变体：`DuplicateEmail(String)`、`WeakPassword(String)`、`DatabaseError(#[from] sqlx::Error)`、`RateLimited { retry_after_secs: u64 }`
2. Create a `type Result<T> = std::result::Result<T, RegistrationError>;` alias
2. 创建 `type Result<T> = std::result::Result<T, RegistrationError>;` 别名
3. Write a `register_user(email: &str, password: &str) -> Result<()>` that demonstrates `?` propagation and explicit error construction
3. 编写 `register_user(email: &str, password: &str) -> Result<()>`，演示 `?` 错误传播和显式构造领域错误

<details>
<summary>Solution | 参考答案</summary>

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RegistrationError {
    #[error("Email already registered: {0}")]
    DuplicateEmail(String),

    #[error("Password too weak: {0}")]
    WeakPassword(String),

    #[error("Database error")]
    Database(#[from] sqlx::Error),

    #[error("Rate limited - retry after {retry_after_secs}s")]
    RateLimited { retry_after_secs: u64 },
}

pub type Result<T> = std::result::Result<T, RegistrationError>;

pub fn register_user(email: &str, password: &str) -> Result<()> {
    if password.len() < 8 {
        return Err(RegistrationError::WeakPassword(
            "must be at least 8 characters".into(),
        ));
    }

    // This ? converts sqlx::Error -> RegistrationError::Database automatically
    // db.check_email_unique(email).await?;

    // This is explicit construction for domain logic
    if email.contains("+spam") {
        return Err(RegistrationError::DuplicateEmail(email.to_string()));
    }

    Ok(())
}
```

**Key pattern**: `#[from]` enables `?` for library errors; explicit `Err(...)` for domain logic. The Result alias keeps every signature clean.

**关键模式：** `#[from]` 负责让库错误支持 `?` 自动转换；而领域逻辑错误则通过显式 `Err(...)` 构造。`Result` 别名可以让整个 crate 的函数签名保持整洁。

</details>
</details>

***
