## Best Practices for C# Developers | 面向 C# 开发者的最佳实践

> **What you'll learn:** Five critical mindset shifts (GC->ownership, exceptions->results, inheritance->composition),
> idiomatic project organization, error handling strategy, testing patterns, and the most common
> mistakes C# developers make in Rust.
>
> **你将学到什么：** 五个关键的思维转变（GC->所有权、异常->Result、继承->组合），
> 惯用的项目组织方式、错误处理策略、测试模式，以及 C# 开发者在 Rust 中最常犯的错误。
>
> **Difficulty:** Intermediate
>
> **难度：** 中级

### 1. **Mindset Shifts** | 1. 思维方式转变
- **From GC to Ownership**: Think about who owns data and when it's freed
- **从 GC 到所有权**：始终思考“数据归谁拥有，以及何时释放”
- **From Exceptions to Results**: Make error handling explicit and visible
- **从异常到 Result**：让错误处理显式化、可见化
- **From Inheritance to Composition**: Use traits to compose behavior
- **从继承到组合**：用 trait 组合行为，而不是堆叠继承层级
- **From Null to Option**: Make absence of values explicit in the type system
- **从 Null 到 Option**：把“值不存在”显式编码进类型系统

### 2. **Code Organization** | 2. 代码组织
```rust
// Structure projects like C# solutions
src/
|- main.rs          // Program.cs equivalent
|- lib.rs           // Library entry point
|- models/          // Like Models/ folder in C#
|  |- mod.rs
|  |- user.rs
|  |- product.rs
|- services/        // Like Services/ folder
|  |- mod.rs
|  |- user_service.rs
|  |- product_service.rs
|- controllers/     // Like Controllers/ (for web apps)
|- repositories/    // Like Repositories/
|- utils/           // Like Utilities/
```

### 3. **Error Handling Strategy** | 3. 错误处理策略
```rust
// Create a common Result type for your application
pub type AppResult<T> = Result<T, AppError>;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),
    
    #[error("Validation error: {message}")]
    Validation { message: String },
    
    #[error("Business logic error: {message}")]
    Business { message: String },
}

// Use throughout your application
pub async fn create_user(data: CreateUserRequest) -> AppResult<User> {
    validate_user_data(&data)?;  // Returns AppError::Validation
    let user = repository.create_user(data).await?;  // Returns AppError::Database
    Ok(user)
}
```

### 4. **Testing Patterns** | 4. 测试模式
```rust
// Structure tests like C# unit tests
#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;  // For parameterized tests like C# [Theory]
    
    #[test]
    fn test_basic_functionality() {
        // Arrange
        let input = "test data";
        
        // Act
        let result = process_data(input);
        
        // Assert
        assert_eq!(result, "expected output");
    }
    
    #[rstest]
    #[case(1, 2, 3)]
    #[case(5, 5, 10)]
    #[case(0, 0, 0)]
    fn test_addition(#[case] a: i32, #[case] b: i32, #[case] expected: i32) {
        assert_eq!(add(a, b), expected);
    }
    
    #[tokio::test]  // For async tests
    async fn test_async_functionality() {
        let result = async_function().await;
        assert!(result.is_ok());
    }
}
```

### 5. **Common Mistakes to Avoid** | 5. 需要避免的常见错误
```rust
// [ERROR] Don't try to implement inheritance
// Instead of:
// struct Manager : Employee  // This doesn't exist in Rust

// [OK] Use composition with traits
trait Employee {
    fn get_salary(&self) -> u32;
}

trait Manager: Employee {
    fn get_team_size(&self) -> usize;
}

// [ERROR] Don't use unwrap() everywhere (like ignoring exceptions)
let value = might_fail().unwrap();  // Can panic!

// [OK] Handle errors properly
let value = match might_fail() {
    Ok(v) => v,
    Err(e) => {
        log::error!("Operation failed: {}", e);
        return Err(e.into());
    }
};

// [ERROR] Don't clone everything (like copying objects unnecessarily)
let data = expensive_data.clone();  // Expensive!

// [OK] Use borrowing when possible
let data = &expensive_data;  // Just a reference

// [ERROR] Don't use RefCell everywhere (like making everything mutable)
struct Data {
    value: RefCell<i32>,  // Interior mutability - use sparingly
}

// [OK] Prefer owned or borrowed data
struct Data {
    value: i32,  // Simple and clear
}
```

This guide provides C# developers with a comprehensive understanding of how their existing knowledge translates to Rust, highlighting both the similarities and the fundamental differences in approach. The key is understanding that Rust's constraints (like ownership) are designed to prevent entire classes of bugs that are possible in C#, at the cost of some initial complexity.

这份指南的重点，是帮助 C# 开发者建立“已有知识如何迁移到 Rust”的整体认知，既看到两者的相通之处，也看清方法论上的本质差异。关键在于理解：Rust 的各种约束（比如所有权）虽然会带来早期学习成本，但它们的目的正是为了消灭一整类在 C# 中可能出现的 bug。

---

### 6. **Avoiding Excessive `clone()`** | 6. 避免过度使用 `clone()`

C# developers instinctively clone data because the GC handles the cost. In Rust, every `.clone()` is an explicit allocation. Most can be eliminated with borrowing.

C# 开发者常常会本能地复制数据，因为 GC 会把很多成本“藏起来”。但在 Rust 中，每一个 `.clone()` 都是显式成本，很多时候都可以通过借用消掉。

```rust
// [ERROR] C# habit: cloning strings to pass around
fn greet(name: String) {
    println!("Hello, {name}");
}

let user_name = String::from("Alice");
greet(user_name.clone());  // unnecessary allocation
greet(user_name.clone());  // and again

// [OK] Borrow instead - zero allocation
fn greet(name: &str) {
    println!("Hello, {name}");
}

let user_name = String::from("Alice");
greet(&user_name);  // borrows
greet(&user_name);  // borrows again - no cost
```

**When clone is appropriate:**
- Moving data into a thread or `'static` closure (`Arc::clone` is cheap - it bumps a counter)
- Caching: you genuinely need an independent copy
- Prototyping: get it working, then remove clones later

**什么时候 clone 是合理的：**
- 需要把数据 move 进线程或 `'static` 闭包时（`Arc::clone` 很便宜，只是增加引用计数）
- 做缓存时，确实需要一个独立副本
- 原型阶段先跑通逻辑，之后再回头消除 clone

**Decision checklist:**
1. Can you pass `&T` or `&str` instead? -> Do that
2. Does the callee need ownership? -> Pass by move, not clone
3. Is it shared across threads? -> Use `Arc<T>` (clone is just a reference count bump)
4. None of the above? -> `clone()` is justified

**决策清单：**
1. 能不能改成传 `&T` 或 `&str`？可以就这么做
2. 调用方真的需要所有权吗？需要就 move，不要 clone
3. 是不是跨线程共享？那就用 `Arc<T>`（clone 只是加引用计数）
4. 如果以上都不满足，那 `clone()` 才算合理

---

### 7. **Avoiding `unwrap()` in Production Code** | 7. 避免在生产代码中滥用 `unwrap()`

C# developers who ignore exceptions write `.unwrap()` everywhere in Rust. Both are equally dangerous.

如果一个 C# 开发者习惯无视异常，那么到了 Rust 里，往往就会到处写 `.unwrap()`。这两种做法一样危险。

```rust
// [ERROR] The "I'll fix this later" trap
let config = std::fs::read_to_string("config.toml").unwrap();
let port: u16 = config_value.parse().unwrap();
let conn = db_pool.get().await.unwrap();

// [OK] Propagate with ? in application code
let config = std::fs::read_to_string("config.toml")?;
let port: u16 = config_value.parse()?;
let conn = db_pool.get().await?;

// [OK] Use expect() only when failure is truly a bug
let home = std::env::var("HOME")
    .expect("HOME environment variable must be set");  // documents the invariant
```

**Rule of thumb:**

| Method | When to use |
|--------|------------|
| `?` | Application/library code - propagate to caller |
| `?` | 应用或库代码中 - 继续向上传播 |
| `expect("reason")` | Startup assertions, invariants that *must* hold |
| `expect("reason")` | 启动阶段断言、必须成立的不变量 |
| `unwrap()` | Tests only, or after an `is_some()`/`is_ok()` check |
| `unwrap()` | 基本只在测试里用，或在已做 `is_some()`/`is_ok()` 检查后使用 |
| `unwrap_or(default)` | When you have a sensible fallback |
| `unwrap_or(default)` | 当你有合理默认值时 |
| `unwrap_or_else(|| ...)` | When the fallback is expensive to compute |
| `unwrap_or_else(|| ...)` | 当默认值计算成本较高时 |

---

### 8. **Fighting the Borrow Checker (and How to Stop)** | 8. 和借用检查器“对抗”时，如何停下来重构

Every C# developer hits a phase where the borrow checker rejects valid-seeming code. The fix is usually a structural change, not a workaround.

几乎每个 C# 开发者都会经历一个阶段：明明觉得代码“看起来没问题”，借用检查器却不放行。真正的解决方法通常不是绕过去，而是调整代码结构。

```rust
// [ERROR] Trying to mutate while iterating (C# foreach + modify pattern)
let mut items = vec![1, 2, 3, 4, 5];
for item in &items {
    if *item > 3 {
        items.push(*item * 2);  // ERROR: can't borrow items as mutable
    }
}

// [OK] Collect first, then mutate
let extras: Vec<i32> = items.iter()
    .filter(|&&x| x > 3)
    .map(|&x| x * 2)
    .collect();
items.extend(extras);
```

```rust
// [ERROR] Returning a reference to a local (C# returns references freely via GC)
fn get_greeting() -> &str {
    let s = String::from("hello");
    &s  // ERROR: s is dropped at end of function
}

// [OK] Return owned data
fn get_greeting() -> String {
    String::from("hello")  // caller owns it
}
```

**Common patterns that resolve borrow checker conflicts:**

| C# habit | Rust solution |
|----------|--------------|
| Store references in structs | Use owned data, or add lifetime parameters |
| 在结构体里随手存引用 | 改为拥有所有权的数据，或显式加生命周期参数 |
| Mutate shared state freely | Use `Arc<Mutex<T>>` or restructure to avoid sharing |
| 自由修改共享状态 | 用 `Arc<Mutex<T>>`，或直接重构避免共享 |
| Return references to locals | Return owned values |
| 返回局部变量的引用 | 改为返回拥有所有权的值 |
| Modify collection while iterating | Collect changes, then apply |
| 一边迭代一边修改集合 | 先收集变更，再统一应用 |
| Multiple mutable references | Split struct into independent parts |
| 同时拿多个可变引用 | 把结构体拆成独立部分 |

---

### 9. **Collapsing Assignment Pyramids** | 9. 把层层嵌套的判断压平

C# developers write chains of `if (x != null) { if (x.Value > 0) { ... } }`. Rust's `match`, `if let`, and `?` flatten these.

C# 开发者很容易写出 `if (x != null) { if (x.Value > 0) { ... } }` 这种层层嵌套。Rust 的 `match`、`if let` 和 `?` 更鼓励把逻辑压平。

```rust
// [ERROR] Nested null-checking style from C#
fn process(input: Option<String>) -> Option<usize> {
    match input {
        Some(s) => {
            if !s.is_empty() {
                match s.parse::<usize>() {
                    Ok(n) => {
                        if n > 0 {
                            Some(n * 2)
                        } else {
                            None
                        }
                    }
                    Err(_) => None,
                }
            } else {
                None
            }
        }
        None => None,
    }
}

// [OK] Flatten with combinators
fn process(input: Option<String>) -> Option<usize> {
    input
        .filter(|s| !s.is_empty())
        .and_then(|s| s.parse::<usize>().ok())
        .filter(|&n| n > 0)
        .map(|n| n * 2)
}
```

**Key combinators every C# developer should know:**

| Combinator | What it does | C# equivalent |
|-----------|-------------|---------------|
| `map` | Transform the inner value | `Select` / null-conditional `?.` |
| `map` | 转换内部值 | `Select` / 空条件访问 `?.` |
| `and_then` | Chain operations that return Option/Result | `SelectMany` / `?.Method()` |
| `and_then` | 串联返回 Option/Result 的操作 | `SelectMany` / `?.Method()` |
| `filter` | Keep value only if predicate passes | `Where` |
| `filter` | 只有满足条件时才保留值 | `Where` |
| `unwrap_or` | Provide default | `?? defaultValue` |
| `unwrap_or` | 提供默认值 | `?? defaultValue` |
| `ok()` | Convert `Result` to `Option` (discard error) | - |
| `ok()` | 把 `Result` 转成 `Option`（丢弃错误） | - |
| `transpose` | Flip `Option<Result>` to `Result<Option>` | - |
| `transpose` | 把 `Option<Result>` 翻转成 `Result<Option>` | - |
