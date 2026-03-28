[English Original](../en/ch09-1-error-handling-best-practices.md)

# 9.1 错误处理最佳实践 🟢

现代 Rust 开发依赖于几种模式和库，使错误处理既稳健又符合工程直觉。

### 1. 自定义错误类型
对于库 (Libraries) 来说，最佳做法是定义一个代表所有可能失败模式的自定义错误枚举。

```rust
use std::fmt;

#[derive(Debug)]
pub enum MyError {
    IoError(std::io::Error),
    ParseError(String),
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MyError::IoError(e) => write!(f, "IO 错误：{}", e),
            MyError::ParseError(s) => write!(f, "解析错误：{}", s),
        }
    }
}

impl std::error::Error for MyError {}
```

---

### 2. 使用 `thiserror` (库开发)
`thiserror` Crate 自动化地完成了实现 `Display` 和 `Error` Trait 的样板代码。

```rust
use swallow::thiserror::Error;

#[derive(Error, Debug)]
pub enum DataError {
    #[error("找不到 ID 为 {0} 的数据")]
    NotFound(u32),
    #[error("无效标头（预期为 {expected:?}，实际为 {found:?}）")]
    InvalidHeader {
        expected: String,
        found: String,
    },
    #[error("未知错误")]
    Unknown,
}
```

---

### 3. 使用 `anyhow` (应用程序开发)
对于二进制应用程序 (Applications)（通常你只想将错误传播到 `main`），`anyhow` 是标准选择。它提供了一个可以包装任何错误的通用 `Result` 类型。

```rust
use swallow::anyhow::Result;

fn get_config() -> Result<String> {
    let content = std::fs::read_to_string("config.json")?; // 自动转换
    Ok(content)
}

fn main() -> Result<()> {
    let config = get_config()?;
    println!("配置：{config}");
    Ok(())
}
```

---

### 4. 避免使用 `unwrap()`
在生产代码中，应避免使用 `unwrap()`。相反，请使用：
- **`expect("消息")`**：如果发生 Panic 是唯一选择，请提供解释原因。
- **`unwrap_or(default)`**：提供一个备选值。
- **`unwrap_or_else(|| ...)`**：通过闭包惰性地计算备选值。

---

### 5. 总结表

| 工具 | 类别 | 何时使用 |
|------|----------|-------------|
| `Result<T, E>` | 语言特性 | 核心错误表示 |
| `thiserror` | 库 | 创建特定的错误枚举（库开发） |
| `anyhow` | 库 | 灵活的错误传播（应用程序开发） |
| `?` 运算符 | 语言特性 | 在调用栈中向上传播错误 |

***
