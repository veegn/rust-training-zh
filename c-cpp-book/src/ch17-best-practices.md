# Rust Best Practices Summary / Rust 最佳实践总结

> **What you'll learn / 你将学到：** Practical guidelines for writing idiomatic Rust — code organization, naming conventions, error handling patterns, and documentation. A quick-reference chapter you'll return to often.
>
> 编写地道（idiomatic）Rust 代码的实用指南 —— 包括代码组织、命名规范、错误处理模式及文档编写。这是一个你会经常查阅的快速参考章节。

---

## Code Organization / 代码组织

- **Prefer small functions**: Easy to test and reason about. / **优先使用小型函数**：易于测试和推理。
- **Use descriptive names**: `calculate_total_price()` vs `calc()`. / **使用描述性名称**：例如 `calculate_total_price()` 优于 `calc()`。
- **Group related functionality**: Use modules and separate files. / **对相关功能进行分组**：使用模块（module）和独立的文件。
- **Write documentation**: Use `///` for public APIs. / **编写文档**：为公开 API 使用 `///` 注释。

---

## Error Handling / 错误处理

- **Avoid `unwrap()` unless infallible**: Only use when you're 100% certain it won't panic. / **除非确定不会出错，否则避免使用 `unwrap()`**：仅当你 100% 确定不会发生 panic 时才使用它。

```rust
// Bad: Can panic / 坏习惯：可能导致 panic
let value = some_option.unwrap();

// Good: Handle None / 好习惯：处理 None 的情况
let value = some_option.unwrap_or(default_value);
let value = some_option.unwrap_or_else(|| expensive_computation());
let value = some_option.unwrap_or_default(); // Uses Default trait / 使用 Default trait

// For Result<T, E> / 针对 Result<T, E>
let value = some_result.unwrap_or(fallback_value);
let value = some_result.unwrap_or_else(|err| {
    eprintln!("Error occurred: {err}");
    default_value
});
```

- **Use `expect()` with descriptive messages**: When unwrap is justified, explain why. / **使用带有描述性消息的 `expect()`**：当使用 unwrap 是合理的时候，请解释理由。

```rust
let config = std::env::var("CONFIG_PATH")
    .expect("CONFIG_PATH environment variable must be set");
```

- **Return `Result<T, E>` for fallible operations**: Let callers decide how to handle errors. / **对可能失败的操作返回 `Result<T, E>`**：让调用者决定如何处理错误。
- **Use `thiserror` for custom error types**: More ergonomic than manual implementations. / **为自定义错误类型使用 `thiserror`**：比手动实现更符合人体工程学。

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MyError {
    #[error("IO error: {0}")] // IO 错误
    Io(#[from] std::io::Error),
    
    #[error("Parse error: {message}")] // 解析错误
    Parse { message: String },
    
    #[error("Value {value} is out of range")] // 数值越界
    OutOfRange { value: i32 },
}
```

- **Chain errors with `?` operator**: Propagate errors up the call stack. / **使用 `?` 运算符链接错误**：将错误沿调用栈向上传播。
- **Prefer `thiserror` over `anyhow`**: Our team convention is to define explicit error enums with `#[derive(thiserror::Error)]` so callers can match on specific variants. `anyhow::Error` is convenient for quick prototyping but erases the error type. / **优先使用 `thiserror` 而非 `anyhow`**：我们团队的规范是使用 `#[derive(thiserror::Error)]` 定义明确的错误枚举，以便调用者可以对特定变体进行匹配。

**When `unwrap()` is acceptable / 何时 `unwrap()` 是可以接受的：**
- **Unit tests**: `assert_eq!(result.unwrap(), expected)` / **单元测试**
- **Prototyping**: Quick and dirty code / **原型设计**
- **Infallible operations**: When you can prove it won't fail / **必然成功的操作**

```rust
let numbers = vec![1, 2, 3];
let first = numbers.get(0).expect("numbers vec is non-empty by construction");
```

- **Fail fast**: Check preconditions early and return errors immediately. / **及早失效（Fail fast）**：尽早检查先决条件并立即返回错误。

---

## Memory Management / 内存管理

- **Prefer borrowing over cloning**: Use `&T` instead of cloning when possible. / **借用优于克隆**：尽可能使用 `&T` 而不是克隆。
- **Use `Rc<T>` sparingly**: Only when you need shared ownership. / **审慎使用 `Rc<T>`**：仅在确实需要共享所有权时使用。
- **Limit lifetimes**: Use scopes `{}` to control when values are dropped. / **限制生命周期**：使用作用域 `{}` 来控制值的释放时机。
- **Avoid `RefCell<T>` in public APIs**: Keep interior mutability internal. / **避免在公开 API 中使用 `RefCell<T>`**：将内部可变性保持在内部。

---

## Performance / 性能

- **Profile before optimizing**: Use `cargo bench` and profiling tools. / **先分析（Profile）再优化**：使用 `cargo bench` 和性能分析工具。
- **Prefer iterators over loops**: More readable and often faster. / **迭代器优于循环**：更具可读性，通常也更快。
- **Use `&str` over `String`**: When you don't need ownership. / **使用 `&str` 而非 `String`**：当你不需要所有权时。
- **Consider `Box<T>` for large stack objects**: Move them to heap if needed. / **对于大型栈对象考虑使用 `Box<T>`**：如果需要，将它们移动到堆上。

---

## Essential Traits to Implement / 必装实现的 Trait

### Core Traits Every Type Should Consider / 每个类型都应考虑的核心 Trait

#### **Debug and Display** / Debug 与 Display
```rust
use std::fmt;

#[derive(Debug)]  // Automatic implementation / 为调试自动实现
struct Person {
    name: String,
    age: u32,
}

impl fmt::Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} (age {})", self.name, self.age)
    }
}
```

#### **Clone and Copy** / Clone 与 Copy
```rust
// Copy: Implicit duplication / 为小型简单类型提供隐式复制
#[derive(Debug, Clone, Copy)]
struct Point { x: i32, y: i32 }

// Clone: Explicit duplication / 为复杂类型提供显式复制
#[derive(Debug, Clone)]
struct Person {
    name: String,  // String doesn't implement Copy / String 没有实现 Copy
    age: u32,
}
```

#### **PartialEq and Eq** / PartialEq 与 Eq
```rust
#[derive(Debug, PartialEq, Eq)]
struct UserId(u64);

#[derive(Debug, PartialEq)]
struct Temperature {
    celsius: f64,  // f64 doesn't implement Eq (due to NaN) / f64 没有实现 Eq
}
```

#### **PartialOrd and Ord** / PartialOrd 与 Ord
```rust
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Priority(u8);
```

#### **Default** / Default
```rust
#[derive(Debug, Default)]
struct Config {
    debug: bool,
    max_connections: u32,
}
```

#### **From and Into** / From 与 Into
```rust
impl From<u64> for UserId {
    fn from(id: u64) -> Self { UserId(id) }
}
// Usage: let user_id: UserId = 123u64.into();
```

---

### Trait Implementation Checklist / Trait 实现检查表

```rust
#[derive(
    Debug,          // [推荐] 始终为调试实现
    Clone,          // [依据情况] 如果类型应该能够复制
    PartialEq,      // [依据情况] 如果类型应该能够比较
    Eq,             // [依据情况] 如果比较满足自反性/传递性
    PartialOrd,     // [依据情况] 如果类型具有顺序
    Ord,            // [依据情况] 如果顺序是全序（total）
    Hash,           // [依据情况] 如果类型将用作 HashMap 的键
    Default,        // [依据情况] 如果存在合理的默认值
)]
struct MyType {
    // fields... / 字段...
}
```

---

### Summary: Trait Benefits / 总结：Trait 的优势

| **Trait** | **Benefit / 优势** | **When to Use / 何时使用** |
|-------|---------|-------------|
| `Debug` | `{:?}` padding / 调试打印 | Always / 始终 (极少数情况除外) |
| `Display` | `{}` user output / 面向用户的输出 | User-facing types / 用户可见类型 |
| `Clone` | Explicit duplication / 显式复制 | When sensible / 当显式复制合理时 |
| `Copy` | Implicit duplication / 隐式复制 | Simple types / 简单类型 |
| `PartialEq` | `==` and `!=` / 相等性判别 | Most types / 大多数类型 |
| `Eq` | Total equality / 全等性 | When logical / 逻辑上合理时 |
| `PartialOrd` | Comparisons / 大小比较 | Natural order / 具有自然顺序时 |
| `Ord` | Sorting / 排序支持 | Total order / 全序关系 |
| `Hash` | `HashMap` keys / 字典键 | As map keys / 作为 key 时 |
| `Default` | `default()` / 默认构造 | Obvious defaults / 有明显默认值时 |
| `From/Into` | Conversions / 转换 | Type mapping / 类型转换 |
| `TryFrom/TryInto` | Fallible / 易错转换 | Can fail / 可能失败的转换 |
