[English Original](../en/ch09-1-crate-level-error-types-and-result-alias.md)

## Crate 级错误类型与 Result 别名

> **你将学到：** 使用 `thiserror` 定义每个 Crate 独有的错误枚举的生产级模式；创建 `Result<T>` 类型别名；以及如何选择 `thiserror`（用于库）与 `anyhow`（用于应用程序）。
>
> **难度：** 🟡 中级

这是编写生产级 Rust 代码的一个关键模式：为每个 Crate 定义一个错误枚举以及一个 `Result` 类型别名，以消除样板代码。

### 模式范式
```rust
// src/error.rs
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("数据库错误: {0}")]
    Database(#[from] sqlx::Error),

    #[error("HTTP 错误: {0}")]
    Http(#[from] reqwest::Error),

    #[error("序列化错误: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("验证错误: {message}")]
    Validation { message: String },

    #[error("未找到：ID 为 {id} 的 {entity}")]
    NotFound { entity: String, id: String },
}

/// Crate 范围内的 Result 别名 —— 每个函数都返回此类型
pub type Result<T> = std::result::Result<T, AppError>;
```

### 在 Crate 中使用
```rust
use crate::error::{AppError, Result};

// 假设数据库连接池可用，例如：
// async fn get_user(pool: &PgPool, id: Uuid) -> Result<User>
// 在此我们展示使用 `pool` 的简略模式。
pub async fn get_user(id: Uuid) -> Result<User> {
    let user = sqlx::query_as!(User, "SELECT * FROM users WHERE id = $1", id)
        .fetch_optional(&pool)
        .await?;  // 通过 #[from] 将 sqlx::Error 自动转换为 AppError::Database

    user.ok_or_else(|| AppError::NotFound {
        entity: "User".into(),
        id: id.to_string(),
    })
}

pub async fn create_user(req: CreateUserRequest) -> Result<User> {
    if req.name.trim().is_empty() {
        return Err(AppError::Validation {
            message: "名称不能为空".into(),
        });
    }
    // ...
}
```

### C# 对比
```csharp
// C# 中的等效模式
public class AppException : Exception
{
    public string ErrorCode { get; }
    public AppException(string code, string message) : base(message)
    {
        ErrorCode = code;
    }
}

// 但在 C# 中，调用者并不知道会抛出哪些异常！
// 而在 Rust 中，错误类型直接体现在函数签名里。
```

### 为什么这很重要
- **`thiserror`** 自动生成 `Display` 和 `Error` trait 的实现。
- **`#[from]`** 让 `?` 运算符能够自动转换库错误。
- `Result<T>` 别名意味着每个函数的签名都非常整洁：`fn foo() -> Result<Bar>`。
- **与 C# 异常不同**，调用者可以在类型定义中看到所有可能的错误变体。

### thiserror vs anyhow：如何选择

在 Rust 的错误处理领域，有两个 Crate 占据主导地位。在它们之间做出选择是你首先要做的决定：

| | `thiserror` | `anyhow` |
|---|---|---|
| **用途** | 为**库 (Libraries)** 定义结构化的错误类型 | 为**应用程序 (Applications)** 提供快速的错误处理 |
| **产出** | 由你控制的自定义枚举 (Enum) | 不透明的 `anyhow::Error` 封装 |
| **调用者可见度** | 类型中包含所有错误变体 | 仅能看到 `anyhow::Error` —— 是不透明的 |
| **最适用于** | 库 Crate、API、任何有下游使用者的代码 | 二进制程序、脚本、原型项目、命令行工具 |
| **向下转换 (Downcasting)** | 直接通过 `match` 匹配各个变体 | 使用 `error.downcast_ref::<MyError>()` |

```rust
// thiserror —— 适用于库 (调用者需要对错误变体进行 match)
use thiserror::Error;

#[derive(Error, Debug)]
pub enum StorageError {
    #[error("未找到文件：{path}")]
    NotFound { path: String },

    #[error("权限被拒绝：{0}")]
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
// anyhow —— 适用于应用程序 (只需传播错误，无需定义类型)
use anyhow::{Context, Result};

fn main() -> Result<()> {
    let config = std::fs::read_to_string("config.toml")
        .context("读取配置文件失败")?;

    let port: u16 = config.parse()
        .context("解析端口号失败")?;

    println!("正在监听端口 {port}");
    Ok(())
}
// anyhow::Result<T> 等价于 Result<T, anyhow::Error>
// .context() 为任何错误添加易读的上下文信息
```

```csharp
// C# 对比：
// thiserror ≈ 使用特定属性定义自定义异常类
// anyhow ≈ 捕获 Exception 并包装信息：
//   throw new InvalidOperationException("读取配置失败", ex);
```

**指导方针**：如果你的代码是一个**库 (library)**（供其他代码调用），请使用 `thiserror`。如果你的代码是一个**应用程序 (application)**（最终生成的二进制程序），请使用 `anyhow`。许多项目会两者结合使用 —— 库 Crate 的公开 API 使用 `thiserror`，而在 `main()` 二进制程序中使用 `anyhow`。

### 错误恢复模式

C# 开发者习惯于使用 `try/catch` 逻辑块来从特定的异常中恢复。Rust 则在 `Result` 上使用组合子 (Combinators) 来达到同样的目的：

```rust
use std::fs;

// 模式 1：使用默认值进行恢复
let config = fs::read_to_string("config.toml")
    .unwrap_or_else(|_| String::from("port = 8080"));  // 如果缺失则使用默认值

// 模式 2：从特定错误中恢复，传播其他错误
fn read_or_create(path: &str) -> Result<String, std::io::Error> {
    match fs::read_to_string(path) {
        Ok(content) => Ok(content),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            let default = String::from("# 新文件");
            fs::write(path, &default)?;
            Ok(default)
        }
        Err(e) => Err(e),  // 传播权限错误等其他错误
    }
}

// 模式 3：在传播前添加上下文
use anyhow::Context;

fn load_config() -> anyhow::Result<Config> {
    let text = fs::read_to_string("config.toml")
        .context("无法读取 config.toml")?;
    let config: Config = toml::from_str(&text)
        .context("无法解析 config.toml")?;
    Ok(config)
}

// 模式 4：将错误映射到你的领域类型
fn parse_port(s: &str) -> Result<u16, AppError> {
    s.parse::<u16>()
        .map_err(|_| AppError::Validation {
            message: format!("无效的端口：{s}"),
        })
}
```

```csharp
// C# 等效写法：
try { config = File.ReadAllText("config.toml"); }
catch (FileNotFoundException) { config = "port = 8080"; }  // 模式 1

try { /* ... */ }
catch (FileNotFoundException) { /* 创建文件 */ }           // 模式 2
catch { throw; }                                            // 重新抛出其他异常
```

**何时恢复 vs 何时传播：**
- 当错误有合理的默认值或重试策略时，选择**恢复 (Recover)**。
- 当应该由**调用者**决定如何处理时，使用 **`?` 进行传播 (Propagate)**。
- 在模块边界处**添加上下文** (`.context()`) 以构建错误追踪链。

---

## 练习

<details>
<summary><strong>🏋️ 练习：设计 Crate 错误类型</strong> (点击展开)</summary>

你正在构建一个用户注册服务。请使用 `thiserror` 设计其错误类型：

1. 定义 `RegistrationError` 枚举，包含以下变体：`DuplicateEmail(String)`、`WeakPassword(String)`、`DatabaseError(#[from] sqlx::Error)`、`RateLimited { retry_after_secs: u64 }`。
2. 创建 `type Result<T> = std::result::Result<T, RegistrationError>;` 别名。
3. 编写一个 `register_user(email: &str, password: &str) -> Result<()>` 函数，演示 `?` 错误的传播以及显式的错误构造逻辑。

<details>
<summary>🔑 参考答案</summary>

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RegistrationError {
    #[error("邮箱已被注册：{0}")]
    DuplicateEmail(String),

    #[error("密码过弱：{0}")]
    WeakPassword(String),

    #[error("数据库错误")]
    Database(#[from] sqlx::Error),

    #[error("速率限制 —— 请在 {retry_after_secs} 秒后重试")]
    RateLimited { retry_after_secs: u64 },
}

pub type Result<T> = std::result::Result<T, RegistrationError>;

pub fn register_user(email: &str, password: &str) -> Result<()> {
    if password.len() < 8 {
        return Err(RegistrationError::WeakPassword(
            "长度必须至少为 8 个字符".into(),
        ));
    }

    // 此处的 ? 会将 sqlx::Error 自动转换为 RegistrationError::Database
    // db.check_email_unique(email).await?;

    // 这是针对领域逻辑的显式构造
    if email.contains("+spam") {
        return Err(RegistrationError::DuplicateEmail(email.to_string()));
    }

    Ok(())
}
```

**关键模式**：针对第三方库错误使用 `#[from]` 开启 `?` 转换；针对领域逻辑使用显式的 `Err(...)`。Result 别名让每一个签名都保持整洁。

</details>
</details>
