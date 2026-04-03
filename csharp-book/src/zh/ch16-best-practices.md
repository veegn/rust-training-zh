[English Original](../en/ch16-best-practices.md)

## C# 开发者的最佳实践指南

> **你将学到：** 五项关键的思维转变（GC → 所有权、异常 → Result、继承 → 组合）；规范的项目组织结构；错误处理策略；测试模式；以及 C# 开发者在 Rust 中最常犯的错误。
>
> **难度：** 🟡 中级

### 1. 思思维转变 (Mindset Shifts)
- **从 GC 到所有权**：思考谁拥有数据，以及数据何时被释放。
- **从异常到 Result**：让错误处理变得显式且可见。
- **从继承到组合**：使用特性 (Trait) 来组合行为。
- **从 Null 到 Option**：利用类型系统显式化“空值”的可能性。

### 2. 代码组织结构
```rust
// 像组织 C# 解决方案 (Solution) 一样组织项目
src/
├── main.rs          // 等效于 Program.cs
├── lib.rs           // 库的入口
├── models/          // 类似于 C# 中的 Models/ 文件夹
│   ├── mod.rs
│   ├── user.rs
│   └── product.rs
├── services/        // 类似于 Services/ 文件夹
│   ├── mod.rs
│   ├── user_service.rs
│   └── product_service.rs
├── controllers/     // 类似于 Controllers/ (用于 Web 应用)
├── repositories/    // 类似于 Repositories/
└── utils/          // 类似于 Utilities/
```

### 3. 错误处理策略
```rust
// 给你的应用定义一个通用的 Result 类型
pub type AppResult<T> = Result<T, AppError>;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("数据库错误: {0}")]
    Database(#[from] sqlx::Error),
    
    #[error("HTTP 错误: {0}")]
    Http(#[from] reqwest::Error),
    
    #[error("验证错误: {message}")]
    Validation { message: String },
    
    #[error("业务逻辑错误: {message}")]
    Business { message: String },
}

// 在整个应用中使用它
pub async fn create_user(data: CreateUserRequest) -> AppResult<User> {
    validate_user_data(&data)?;  // 返回 AppError::Validation
    let user = repository.create_user(data).await?;  // 返回 AppError::Database
    Ok(user)
}
```

### 4. 测试模式
```rust
// 按照 C# 单元测试的思路来组织测试
#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;  // 用于像 C# 的 [Theory] 那样的参数化测试
    
    #[test]
    fn test_basic_functionality() {
        // Arrange (准备)
        let input = "测试数据";
        
        // Act (执行)
        let result = process_data(input);
        
        // Assert (断言)
        assert_eq!(result, "预期输出");
    }
    
    #[rstest]
    #[case(1, 2, 3)]
    #[case(5, 5, 10)]
    #[case(0, 0, 0)]
    fn test_addition(#[case] a: i32, #[case] b: i32, #[case] expected: i32) {
        assert_eq!(add(a, b), expected);
    }
    
    #[tokio::test]  // 用于异步测试
    async fn test_async_functionality() {
        let result = async_function().await;
        assert!(result.is_ok());
    }
}
```

### 5. 应避免的常见错误
```rust
// [错误] 不要尝试实现继承
// 不要写成：
// struct Manager : Employee  // 这在 Rust 中并不存在

// [正确] 使用带有特性的组合模式
trait Employee {
    fn get_salary(&self) -> u32;
}

trait Manager: Employee {
    fn get_team_size(&self) -> usize;
}

// [错误] 不要到处使用 unwrap() (这等同于忽略异常)
let value = might_fail().unwrap();  // 可能会发生 Panic！

// [正确] 妥善处理错误
let value = match might_fail() {
    Ok(v) => v,
    Err(e) => {
        log::error!("操作失败：{}", e);
        return Err(e.into());
    }
};

// [错误] 不要克隆 (clone) 所有东西 (这等同于在不必要时复制对象)
let data = expensive_data.clone();  // 开销很大！

// [正确] 尽可能使用借用 (Borrowing)
let data = &expensive_data;  // 只是一个引用

// [错误] 不要到处使用 RefCell (这等同于让所有东西都变成可变的)
struct Data {
    value: RefCell<i32>,  // 内部可变性 —— 请谨慎使用
}

// [正确] 优先使用拥有所有权或被借用的数据
struct Data {
    value: i32,  // 简洁明了
}
```

本指南旨在让 C# 开发者全面了解如何将现有知识转化为 Rust 技能，同时强调了两者的相似之处以及在处理方式上的根本区别。关键在于理解 Rust 的约束（如所有权）是为了在付出初期复杂性代价的前提下，杜绝 C# 中可能出现的各种逻辑 Bug。

---

### 6. 避免过度使用 `clone()` 🟡

C# 开发者习惯于克隆数据，因为 GC 会处理这些开销。但在 Rust 中，每一次 `.clone()` 都是一次显式的内存分配。通过借用，绝大多数克隆都可以被消除。

```rust
// [错误] C# 习惯：到处克隆字符串进行传递
fn greet(name: String) {
    println!("你好, {name}");
}

let user_name = String::from("Alice");
greet(user_name.clone());  // 不必要的内存分配
greet(user_name.clone());  // 再次分配

// [正确] 改用借用 —— 零成本分配
fn greet(name: &str) {
    println!("你好, {name}");
}

let user_name = String::from("Alice");
greet(&user_name);  // 借用
greet(&user_name);  // 再次借用 —— 无额外开销
```

**何时适合调用 clone ：**
- 将数据移动到另一个线程或 `'static` 闭包中（`Arc::clone` 开销很低 —— 它仅仅是增加一个计数器）。
- 缓存机制：你确实需要一个完全独立的副本。
- 原型设计：优先保证代码跑通，稍后再优化掉不必要的 clone。

**决策检查清单：**
1. 能否改传 `&T` 或 `&str`？ → 如果可以，就这样做。
2. 调用方是否需要所有权？ → 通过移动 (Move) 传递，而非克隆。
3. 是否需要在线程间共享？ → 使用 `Arc<T>` (克隆只是引用计数自增)。
4. 以上皆不适用？ → 此时使用 `clone()` 是合理的。

---

### 7. 不要在生产代码中使用 `unwrap()` 🟡

那些在 C# 中习惯忽略异常的开发者，往往会在 Rust 中到处写 `.unwrap()`。两者同样危险。

```rust
// [错误] “我稍后再改”的陷阱
let config = std::fs::read_to_string("config.toml").unwrap();
let port: u16 = config_value.parse().unwrap();
let conn = db_pool.get().await.unwrap();

// [正确] 在应用代码中通过 ? 向上游传播错误
let config = std::fs::read_to_string("config.toml")?;
let port: u16 = config_value.parse()?;
let conn = db_pool.get().await?;

// [正确] 仅在失败意味着程序逻辑确实存在 Bug 时才使用 expect()
let home = std::env::var("HOME")
    .expect("HOME 环境变量必须已设置");  // 这种写法也记录了不变性文档
```

**经验法则：**
| 方法 | 何时使用 |
|--------|------------|
| `?` | 在应用/库代码中 —— 传播给调用者 |
| `expect("原因")` | 启动时的断言、*必须* 成立的不变性 |
| `unwrap()` | 仅用于测试，或是在 `is_some()`/`is_ok()` 检查之后 |
| `unwrap_or(default)` | 当你有一个合理的备选方案 (Fallback) 时 |
| `unwrap_or_else(|| ...)` | 当备选方案的计算开销较大时 |

---

### 8. 停止与借用检查器“斗争” 🟡

每个 C# 开发者都会遇到借用检查器拒绝看似合理的代码的阶段。解决方法通常是调整架构方案，而非通过 Trick 绕过。

```rust
// [错误] 尝试在迭代期间修改集合 (C# 的 foreach + 修改模式)
let mut items = vec![1, 2, 3, 4, 5];
for item in &items {
    if *item > 3 {
        items.push(*item * 2);  // 错误：无法以可变方式借用 items
    }
}

// [正确] 先收集变动，再统一处理
let extras: Vec<i32> = items.iter()
    .filter(|&&x| x > 3)
    .map(|&x| x * 2)
    .collect();
items.extend(extras);
```

```rust
// [错误] 返回指向局部变量的引用 (C# 中通过 GC 自由返回引用)
fn get_greeting() -> &str {
    let s = String::from("你好");
    &s  // 错误：s 会在函数结束时被销毁
}

// [正确] 返回拥有所有权的数据
fn get_greeting() -> String {
    String::from("你好")  // 由调用者接手所有权
}
```

**解决借用冲突的常见模式：**

| C# 习惯 | Rust 解决方案 |
|----------|--------------|
| 将引用存储在结构体中 | 使用拥有所有权的数据，或添加生命周期参数 |
| 自由地修改共享状态 | 使用 `Arc<Mutex<T>>` 或通过重构避免共享 |
| 返回局部变量的引用 | 返回拥有所有权的值 |
| 迭代期间修改集合 | 先收集变更点，最后应用 |
| 需要多个可变引用 | 将结构体拆分为相互独立的各部分 |

---

### 9. 扁平化“嵌套金字塔” 🟢

C# 开发者常写 `if (x != null) { if (x.Value > 0) { ... } }` 这样的嵌套。而 Rust 的 `match`, `if let` 以及 `?` 可以将其扁平化。

```rust
// [错误] 延续自 C# 的嵌套判断风格
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

// [正确] 组合器扁平化风格
fn process(input: Option<String>) -> Option<usize> {
    input
        .filter(|s| !s.is_empty())
        .and_then(|s| s.parse::<usize>().ok())
        .filter(|&n| n > 0)
        .map(|n| n * 2)
}
```

**每个 C# 开发者都应掌握的关键组合器：**

| 组合器 | 作用 | C# 对应项 |
|-----------|-------------|---------------|
| `map` | 转换内部的数值 | `Select` / 空条件运算符 `?.` |
| `and_then` | 链式调用返回 Option/Result 的操作 | `SelectMany` / `?.Method()` |
| `filter` | 仅保留满足谓词的数值 | `Where` |
| `unwrap_or` | 提供默认值 | `?? 默认值` |
| `ok()` | 将 `Result` 转换为 `Option` (丢弃错误) | — |
| `transpose` | 交换嵌套层级 `Option<Result>` ↔ `Result<Option>` | — |
