[English Original](../en/ch09-error-handling.md)

# 9. 错误处理 🟡

> **你将学到：**
> - `Result<T, E>` 核心机制及其与 Python 异常的区别
> - 使用 `?` 操作符进行清晰、可见的错误传递
> - 利用 `thiserror` 定义应用自定义错误枚举
> - 为什么显式的错误处理能避免大型应用中的静默生产 Bug

## 异常 vs Result

在 Python 中，错误是**抛出 (Thrown)** 的，可以在任何地方被捕获（或者根本不捕获导致奔溃）。而在 Rust 中，错误是函数的**返回值 (Value)**。这种方式让错误在函数签名中变得一目了然，且编译器**必须**强制你处理它们。

### Python: 隐式的异常抛出
```python
def load_config(path):
    with open(path) as f:
        return json.load(f) 
# 该函数可能会抛出 FileNotFoundError 或 JSONDecodeError。
# 仅仅看函数签名，你是完全猜不到的！
```

### Rust: 显式的 Result 返回
```rust
fn load_config(path: &str) -> Result<Config, ConfigError> {
    let s = std::fs::read_to_string(path)?; // 如果失败，立刻作为 Err 返回
    let config: Config = serde_json::from_str(&s)?; // 如果解析出错，立刻作为 Err 返回
    Ok(config)
}
```

---

## `?` 操作符：可见的传递

`?` 操作符是 Rust 对 Python 异常传递机制的一种更清晰的替代方案。它的语义是：“如果成功，取出其中的值；如果失败，立刻跳出当前函数，并把错误返回给上层调用者。”

相比 Python 异常，它的优点是：
1. **显眼**：通过代码中的 `?` 就能一眼看出哪些行可能会出错。
2. **类型安全**：返回的错误类型必须与当前函数的返回类型匹配。

```rust
fn process() -> Result<(), io::Error> {
    step_one()?; // 如果此处出错，process() 函数停止执行，向上返回此错误
    step_two()?; 
    Ok(())
}
```

---

## 使用 `thiserror` 自定义错误类型

与其使用通用的字符串，Rust 通常建议使用枚举来分类错误。`thiserror` Crate 让这类声明变得优雅。

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("用户 {0} 找不到了")]
    NotFound(i32),

    #[error("发生了网络错误: {0}")]
    Network(#[from] std::io::Error), // 自动将底层 IO 错误转换为本枚举变体！
}

fn fetch_user(id: i32) -> Result<User, AppError> {
    // ...
    Err(AppError::NotFound(id))
}
```

### 快速概念映射：
- `try / except` → `match result { Ok(v) => ..., Err(e) => ... }`
- `raise Exception("...")` → `return Err(AppError::...)`
- `finally` → `Drop` Trait (当变量离开作用域时由系统自动触发，不依赖 try 块)

---

## 练习

<details>
<summary><strong>🏋️ 练习：安全除法</strong> (点击展开)</summary>

**挑战**：写一个函数 `divide(a: f64, b: f64) -> Result<f64, String>`。如果除数为零，返回一个包含 `"除数不能为零"` 错误信息的 Err。使用 `match` 调用该函数并打印结果。

<details>
<summary>参考答案</summary>

```rust
fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        return Err("除数不能为零".to_string());
    }
    Ok(a / b)
}

fn main() {
    match divide(10.0, 0.0) {
        Ok(val) => println!("结果是: {val}"),
        Err(e) => println!("出错啦: {e}"),
    }
}
```
</details>
</details>

***
