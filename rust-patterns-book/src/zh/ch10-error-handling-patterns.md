[English Original](../en/ch10-error-handling-patterns.md)

# 10. 错误处理模式 🟢

> **你将学到：**
> - 何时使用 `thiserror`（库）与 `anyhow`（应用程序）
> - 使用 `#[from]` 和 `.context()` 的错误转换链
> - `?` 运算符的反糖化工作原理
> - 何时触发 Panic 与何时返回错误

## thiserror 与 anyhow

| | `thiserror` | `anyhow` |
|---|---|---|
| **用于** | 库、共享 crate | 应用程序、二进制文件 |
| **错误类型** | 具体的枚举（可匹配） | `anyhow::Error`（不透明） |
| **开发工作量** | 需要定义错误枚举 | 直接使用 `anyhow::Result<T>` |

---

## 库级错误模式 (thiserror)

使用 `thiserror` 为你的库用户定义有意义且可匹配的错误类型。

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MyError {
    #[error("I/O 失败: {0}")]
    Io(#[from] std::io::Error),

    #[error("未找到: {id}")]
    NotFound { id: u64 },
}
```

---

## 应用级错误模式 (anyhow)

使用 `anyhow` 为顶层代码中传播的错误添加人类可读的上下文信息。

```rust
use anyhow::{Context, Result};

fn load_config() -> Result<Config> {
    let s = std::fs::read_to_string("cfg.json")
        .context("读取配置文件失败")?;
    
    serde_json::from_str(&s)
        .context("解析 JSON 失败")
}
```

---

## `?` 运算符

`?` 运算符是 `match` 块的语法糖，它会自动使用 `From` trait 进行错误类型转换并在遇到错误时提前返回。

```rust
// 这样：
let value = op()?;

// 反糖化后等效于：
let value = match op() {
    Ok(v) => v,
    Err(e) => return Err(From::from(e)),
};
```

> **注意**：在返回 `Option` 的函数中，`?` 也能用于 `Option`。

---

## 何时触发 Panic

- **使用 `Result`** 来处理 **预期内** 的错误（如文件未找到、网络超时）。
- **使用 `panic!`** 来处理 **Bug**（如索引越界、违反内部不变式）。
- **使用 `catch_unwind`** 仅在重大边界处（如 FFI 或线程池工作线程）。

***
