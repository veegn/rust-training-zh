[English Original](../en/ch09-1-error-handling-best-practices.md)

# Rust Option 和 Result 核心要点

> **你将学到：** 惯用 (Idiomatic) 的错误处理模式 —— `unwrap()` 的安全替代方案、用于传播错误的 `?` 操作符、自定义错误类型，以及在生产环境代码中何时使用 `anyhow` vs `thiserror`。

- `Option` 和 `Result` 是编写惯用 Rust 代码不可或缺的一部分。
- **`unwrap()` 的安全替代方案**：
```rust
// Option<T> 的安全替代
let value = opt.unwrap_or(default);              // 提供备选值 (Fallback)
let value = opt.unwrap_or_else(|| compute());    // 惰性计算备选值
let value = opt.unwrap_or_default();             // 使用 Default Trait 的实现
let value = opt.expect("描述性错误信息");          // 仅在可以接受崩溃的情况下使用

// Result<T, E> 的安全替代  
let value = result.unwrap_or(fallback);          // 忽略错误，使用备选值
let value = result.unwrap_or_else(|e| handle(e)); // 处理错误并返回备选值
let value = result.unwrap_or_default();          // 使用 Default Trait
```
- **用于显式控制的模式匹配**：
```rust
match some_option {
    Some(value) => println!("获取到: {}", value),
    None => println!("未找到任何值"),
}

match some_result {
    Ok(value) => process(value),
    Err(error) => log_error(error),
}
```
- **使用 `?` 操作符进行错误传播**：实现错误触发时的“短路”行为并将其向上层抛出：
```rust
fn process_file(path: &str) -> Result<String, std::io::Error> {
    let content = std::fs::read_to_string(path)?; // 发生错误时自动返回 Err
    Ok(content.to_uppercase())
}
```
- **变换方法**：
    - `map()`：变换成功的变元，即 `Ok(T)` -> `Ok(U)` 或 `Some(T)` -> `Some(U)`。
    - `map_err()`：变换错误类型，即 `Err(E)` -> `Err(F)`。
    - `and_then()`：链式调用可能失败的操作。
- **在自定义 API 中使用**：优先选择 `Result<T, E>`，而非异常或错误码。
- **参考资料**：[Option 文档](https://doc.rust-lang.org/std/option/enum.Option.html) | [Result 文档](https://doc.rust-lang.org/std/result/enum.Result.html)

---

# Rust 常见坑点与调试技巧

- **借用 (Borrowing) 问题**：最常见的初学者错误。
    - "cannot borrow as mutable"：一次仅允许存在一个可变引用。
    - "borrowed value does not live long enough"：引用超出了它所指向数据的生命周期。
    - **解决方法**：使用作用域 `{}` 来限制引用的生命周期，或在需要时克隆 (Clone) 数据。
- **缺失 Trait 实现**："method not found" 错误。
    - **解决方法**：为常用类型添加 `#[derive(Debug, Clone, PartialEq)]`。
    - 使用 `cargo check` 而非 `cargo run` 来获取更详细的错误消息。
- **调试模式下的整数溢出**：Rust 在溢出时会崩溃 (Panic)。
    - **解决方法**：使用 `wrapping_add()`、`saturating_add()` 或 `checked_add()` 以获得明确的行为。
- **String 与 &str 的混淆**：它们是针对不同用例的不同类型。
    - `&str` 用于字符串切片（借用），`String` 用于拥有所有权的字符串。
    - **解决方法**：使用 `.to_string()` 或 `String::from()` 将 `&str` 转换为 `String`。
- **对抗借用检查器 (Borrow Checker)**：不要试图超越它。
    - **解决方法**：调整代码结构以适应所有权规则，而不是试图绕过它。
    - 在需要复杂共享的场景下（谨慎）使用 `Rc<RefCell<T>>`。

---

## 错误处理示例：好与坏

```rust
// [错误] BAD: 可能会在没有任何预警的情况下触发崩溃
fn bad_config_reader() -> String {
    let config = std::env::var("CONFIG_FILE").unwrap(); // 如果未设置该环境变量，程序会崩溃！
    std::fs::read_to_string(config).unwrap()           // 如果文件不存在，程序会崩溃！
}

// [好] GOOD: 优雅地处理错误
fn good_config_reader() -> Result<String, ConfigError> {
    let config_path = std::env::var("CONFIG_FILE")
        .unwrap_or_else(|_| "default.conf".to_string()); // 默认退回 default.conf
    
    let content = std::fs::read_to_string(config_path)
        .map_err(ConfigError::FileRead)?;                // 转换并传播错误
    
    Ok(content)
}

// [更好] EVEN BETTER: 使用专门的错误类型
use thiserror::Error;

#[derive(Error, Debug)]
enum ConfigError {
    #[error("无法读取配置文件: {0}")]
    FileRead(#[from] std::io::Error),
    
    #[error("配置信息无效: {message}")]
    Invalid { message: String },
}
```

---

让我们分析一下这里的逻辑。`ConfigError` 仅包含**两个变元 (Variants)** —— 一个用于 I/O 错误，另一个用于验证错误。对于大多数模块来说，这都是一个非常好的起点：

| `ConfigError` 变元 | 持有的内容 | 创建方式 |
|----------------------|-------|-----------|
| `FileRead(io::Error)` | 原始 I/O 错误 | `#[from]` 会实现通过 `?` 自动转换 |
| `Invalid { message }` | 人类可读的解释说明 | 你的验证代码 |

现在，你可以编写返回 `Result<T, ConfigError>` 的函数了：

```rust
fn read_config(path: &str) -> Result<String, ConfigError> {
    let content = std::fs::read_to_string(path)?;  // io::Error → ConfigError::FileRead
    if content.is_empty() {
        return Err(ConfigError::Invalid {
            message: "配置文件为空".to_string(),
        });
    }
    Ok(content)
}
```

> **🟢 自学检查点：** 在继续之前，请确保你能回答以下两个问题：
> 1. 为什么在 `read_to_string` 调用处使用 `?` 是有效的？（因为 `#[from]` 生成了 `impl From<io::Error> for ConfigError`）
> 2. 如果你增加第三个变元 `MissingKey(String)` —— 哪些代码需要修改？（只需增加变元即可；现有代码依然可以正常编译）

---

## 单元包级别的错误类型与 Result 别名

随着项目规模超出单个文件，你会将多个模块级别的错误组合成一个**单元包级别的错误类型**。这是 Rust 生产环境代码中的标准模式。让我们在上面的 `ConfigError` 基础上继续构建。

在真实的 Rust 项目中，每个单元包 (Crate)（或规模较大的模块）都会定义自己的 `Error` 枚举和 `Result` 类型别名。这是一种惯用模式 —— 类似于在 C++ 中为每个库定义异常层次结构以及 `using Result = std::expected<T, Error>`。

### 模式实现

```rust
// src/error.rs (或 lib.rs 的顶部)
use thiserror::Error;

/// 本单元包可能产生的所有错误。
#[derive(Error, Debug)]
pub enum Error {
    #[error("I/O 错误: {0}")]
    Io(#[from] std::io::Error),          // 通过 From 自动转换

    #[error("JSON 解析错误: {0}")]
    Json(#[from] serde_json::Error),     // 通过 From 自动转换

    #[error("传感器 ID 无效: {0}")]
    InvalidSensor(u32),                  // 业务领域相关的变元

    #[error("在 {ms} 毫秒后超时")]
    Timeout { ms: u64 },
}

/// 单元包通用的 Result 别名 —— 减少全包范围内的重复输入。
pub type Result<T> = core::result::Result<T, Error>;
```

---

### 该模式如何简化每一个函数

如果不使用别名，你需要这样写：

```rust
// 冗长 —— 错误类型在各处不断重复
fn read_sensor(id: u32) -> Result<f64, crate::Error> { ... }
fn parse_config(path: &str) -> Result<Config, crate::Error> { ... }
```

使用别名后：

```rust
// 简洁 —— 仅需 `Result<T>`
use crate::{Error, Result};

fn read_sensor(id: u32) -> Result<f64> {
    if id > 128 {
        return Err(Error::InvalidSensor(id));
    }
    // io::Error → Error::Io (通过 ? 自动转换)
    let raw = std::fs::read_to_string(format!("/dev/sensor/{id}"))?; 
    let value: f64 = raw.trim().parse()
        .map_err(|_| Error::InvalidSensor(id))?;
    Ok(value)
}
```

---

`thiserror` 的 `#[from]` 属性会为你免费生成如下 `impl`：

```rust
// 由 thiserror 的 #[from] 自动生成
impl From<std::io::Error> for Error {
    fn from(source: std::io::Error) -> Self {
        Error::Io(source)
    }
}
```

这就是 `?` 能够工作的原因：当某个函数正在抛出 `std::io::Error`，而你的函数返回的是你的别名 `Result<T>` 时，编译器会自动调用 `From::from()` 进行转换。

---

### 组合模块级别的错误

较大规模的单元包会在每个模块中分别定义错误，然后在单元包根节点进行组合：

```rust
// src/config/error.rs
#[derive(thiserror::Error, Debug)]
pub enum ConfigError {
    #[error("缺失键: {0}")]
    MissingKey(String),
    #[error("'{key}' 的值无效: {reason}")]
    InvalidValue { key: String, reason: String },
}

// src/error.rs (单元包级别)
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]               // 将 Display 委托给内部错误
    Config(#[from] crate::config::ConfigError),

    #[error("I/O 错误: {0}")]
    Io(#[from] std::io::Error),
}
pub type Result<T> = core::result::Result<T, Error>;
```

调用者仍然可以匹配特定的配置错误：

```rust
match result {
    Err(Error::Config(ConfigError::MissingKey(k))) => eprintln!("请在配置中添加 '{k}'"),
    Err(e) => eprintln!("其他错误: {e}"),
    Ok(v) => use_value(v),
}
```

---

### C++ 对比

| 概念 | C++ | Rust |
|---------|-----|------|
| 错误层次结构 | `class AppError : public std::runtime_error` | `#[derive(thiserror::Error)] enum Error { ... }` |
| 返回错误 | `std::expected<T, Error>` 或 `throw` | `fn foo() -> Result<T>` |
| 转换错误 | 手动的 `try/catch` + 重新抛出 (Rethrow) | `#[from]` + `?` —— 零样板代码 |
| Result 别名 | `template<class T> using Result = std::expected<T, Error>;` | `pub type Result<T> = core::result::Result<T, Error>;` |
| 错误消息 | 重写 (Override) `what()` | `#[error("...")]` —— 编译为 `Display` 实现 |

---
