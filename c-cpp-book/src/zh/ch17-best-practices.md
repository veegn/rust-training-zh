[English Original](../en/ch17-best-practices.md)

# Rust 最佳实践总结

> **你将学到：** 编写地道 Rust 代码的实用指南 —— 包括代码组织、命名规范、错误处理模式以及文档编写。这是一个你会经常回顾的快速参考章节。

## 代码组织
- **优先编写短小精悍的函数**：易于测试和理解。
- **使用描述性的名称**：例如使用 `calculate_total_price()` 而非 `calc()`。
- **对相关功能进行分组**：利用模块 (Modules) 和独立文件进行组织。
- **编写文档**：为公共 API 使用 `///` 编写文档注释。

## 错误处理
- **除非确定万无一失，否则避免使用 `unwrap()`**：仅当你 100% 确定不会发生 panic 时才使用它。
```rust
// 不良实践：可能会引发 panic
let value = some_option.unwrap();

// 良好实践：处理 None 的情况
let value = some_option.unwrap_or(default_value);
let value = some_option.unwrap_or_else(|| expensive_computation());
let value = some_option.unwrap_or_default(); // 使用 Default trait

// 对于 Result<T, E>
let value = some_result.unwrap_or(fallback_value);
let value = some_result.unwrap_or_else(|err| {
    eprintln!("发生错误: {err}");
    default_value
});
---

- **使用带有描述性信息的 `expect()`**：当使用 unwrap 是合理的时候，请解释其原因。
```rust
let config = std::env::var("CONFIG_PATH")
    .expect("必须设置 CONFIG_PATH 环境变量");
```
- **为可能失败的操作返回 `Result<T, E>`**：由调用者决定如何处理错误。
- **针对自定义错误类型使用 `thiserror`**：比手动实现更加符合工效学。
```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MyError {
    #[error("IO 错误: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("解析错误: {message}")]
    Parse { message: String },
    
    #[error("数值 {value} 超出范围")]
    OutOfRange { value: i32 },
}
```
- **利用 `?` 运算符链式处理错误**：将错误沿调用栈向上传递。
- **优先选择 `thiserror` 而非 `anyhow`**：我们团队的惯例是定义显式的带有 `#[derive(thiserror::Error)]` 的错误枚举，以便调用者可以对具体的变体进行模式匹配。`anyhow::Error` 虽然在快速原型设计时非常方便，但它会抹除错误的类型信息，导致调用者难以处理特定的故障。在库文件和生产环境代码中，请使用 `thiserror`；将 `anyhow` 留给临时的脚本或是仅需打印错误信息的顶层二进制文件。
- **何时使用 `unwrap()` 是可以接受的**：
  - **单元测试**：`assert_eq!(result.unwrap(), expected)`
  - **原型设计**：后期会被替换的临时代码。
  - **确定的无误操作**：当你能证明操作绝不会失败时。
```rust
let numbers = vec![1, 2, 3];
let first = numbers.get(0).unwrap(); // 安全：我们刚刚创建了带有元素的 vec

// 更好的方式：使用带有解释的 expect()
let first = numbers.get(0).expect("依构造可知 numbers vec 非空");
```
- **尽早失败 (Fail fast)**：尽早检查前置条件并立即返回错误。

---

## 内存管理
- **优先使用借用而非克隆**：只要可能，尽量使用 `&T` 而非克隆 (clone)。
- **谨慎使用 `Rc<T>`**：仅在你真正需要共享所有权时才使用它。
- **限制生命周期范围**：利用代码块 `{}` 显式控制值的销毁时机。
- **避免在公开 API 中使用 `RefCell<T>`**：将内部可变性保持在内部实现。

## 性能
- **优化前先进行基准测试**：使用 `cargo bench` 和性能剖析工具。
- **优先使用迭代器而非循环**：更具可读性且通常性能更佳。
- **优先使用 `&str` 而非 `String`**：当你不需要拥有所有权时。
- **为大型栈对象考虑使用 `Box<T>`**：如有必要，将其移动至堆空间。

## 必须实现的必备 Trait

### 每个类型都应考虑实现的核心 Trait

在创建自定义类型时，请考虑实现以下基础 Trait，使你的类型在 Rust 中显得更加“地道”：

#### **Debug 和 Display**
```rust
use std::fmt;

#[derive(Debug)]  // 自动生成以便于调试
struct Person {
    name: String,
    age: u32,
}

// 手动实现 Display 以获得面向用户的输出
impl fmt::Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} (年龄 {})", self.name, self.age)
    }
}

// 用法示例：
let person = Person { name: "Alice".to_string(), age: 30 };
println!("{:?}", person);  // 调试辅助: Person { name: "Alice", age: 30 }
println!("{}", person);    // 用户显示: Alice (年龄 30)
```

---

#### **Clone 和 Copy**
```rust
// Copy: 对小型简单类型进行隐式复制
#[derive(Debug, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

// Clone: 对复杂类型进行显式复制
#[derive(Debug, Clone)]
struct Person {
    name: String,  // String 未实现 Copy
    age: u32,
}

let p1 = Point { x: 1, y: 2 };
let p2 = p1;  // Copy (隐式)

let person1 = Person { name: "Bob".to_string(), age: 25 };
let person2 = person1.clone();  // Clone (显式)
```

#### **PartialEq 和 Eq**
```rust
#[derive(Debug, PartialEq, Eq)]
struct UserId(u64);

#[derive(Debug, PartialEq)]
struct Temperature {
    celsius: f64,  // 由于 NaN 的存在，f64 未实现 Eq
}

let id1 = UserId(123);
let id2 = UserId(123);
assert_eq!(id1, id2);  // 由于实现 PartialEq，此处成立

let temp1 = Temperature { celsius: 20.0 };
let temp2 = Temperature { celsius: 20.0 };
assert_eq!(temp1, temp2);  // 由于实现 PartialEq，此处成立
```

#### **PartialOrd 和 Ord**
```rust
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Priority(u8);

let high = Priority(1);
let low = Priority(10);
assert!(high < low);  // 数值越小 = 优先级越高

// 用于集合
let mut priorities = vec![Priority(5), Priority(1), Priority(8)];
priorities.sort();  // 由于 Priority 实现了 Ord，此处成立
```

---

#### **Default**
```rust
#[derive(Debug, Default)]
struct Config {
    debug: bool,           // false (默认值)
    max_connections: u32,  // 0 (默认值)
    timeout: Option<u64>,  // None (默认值)
}

// 手动实现 Default
impl Default for Config {
    fn default() -> Self {
        Config {
            debug: false,
            max_connections: 100,  // 自定义默认值
            timeout: Some(30),     // 自定义默认值
        }
    }
}

let config = Config::default();
let config = Config { debug: true, ..Default::default() };  // 部分属性覆盖
```

#### **From 和 Into**
```rust
struct UserId(u64);
struct UserName(String);

// 实现 From 后，Into 将被自动实现
impl From<u64> for UserId {
    fn from(id: u64) -> Self {
        UserId(id)
    }
}

impl From<String> for UserName {
    fn from(name: String) -> Self {
        UserName(name)
    }
}

impl From<&str> for UserName {
    fn from(name: &str) -> Self {
        UserName(name.to_string())
    }
}

// 用法示例：
let user_id: UserId = 123u64.into();         // 使用 Into
let user_id = UserId::from(123u64);          // 使用 From
let username = UserName::from("alice");      // 从 &str 到 UserName
let username: UserName = "bob".into();       // 使用 Into
```

---

#### **TryFrom 和 TryInto**
```rust
use std::convert::TryFrom;

struct PositiveNumber(u32);

#[derive(Debug)]
struct NegativeNumberError;

impl TryFrom<i32> for PositiveNumber {
    type Error = NegativeNumberError;
    
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value >= 0 {
            Ok(PositiveNumber(value as u32))
        } else {
            Err(NegativeNumberError)
        }
    }
}

// 用法示例：
let positive = PositiveNumber::try_from(42)?;     // Ok(PositiveNumber(42))
let error = PositiveNumber::try_from(-5);         // Err(NegativeNumberError)
```

#### **Serde (用于序列化)**
```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct User {
    id: u64,
    name: String,
    email: String,
}

// 自动进行 JSON 序列化/反序列化
let user = User {
    id: 1,
    name: "Alice".to_string(),
    email: "alice@example.com".to_string(),
};

let json = serde_json::to_string(&user)?;
let deserialized: User = serde_json::from_str(&json)?;
```

---

### Trait 实现清单

对于任意新类型，请考虑以下清单：

```rust
#[derive(
    Debug,          // [推荐] 对调试始终实现
    Clone,          // [可选] 如果该类型应该是可复制的
    PartialEq,      // [可选] 如果该类型应该是可比较的
    Eq,             // [可选] 如果比较满足自反性/传递性
    PartialOrd,     // [可选] 如果该类型具有顺序关系
    Ord,            // [可选] 如果该类型具有全序关系
    Hash,           // [可选] 如果该类型将用作 HashMap 的键 (Key)
    Default,        // [可选] 如果该类型存在合理的默认值
)]
struct MyType {
    // 字段...
}

// 考虑进行手动实现的 Trait：
impl Display for MyType { /* 面向用户的表现形式 */ }
impl From<OtherType> for MyType { /* 便捷转换 */ }
impl TryFrom<FallibleType> for MyType { /* 可能失败的转换 */ }
```

### 何时不应实现某些 Trait

- **不要为持有堆数据的类型实现 Copy**：如 `String`、`Vec`、`HashMap` 等。
- **如果值可能为 NaN，则不要实现 Eq**：包含 `f32`/`f64` 的类型。
- **如果没有合理的默认值，则不要实现 Default**：如文件句柄、网络连接。
- **如果克隆开销巨大，则不要实现 Clone**：大型数据结构（此时考虑使用 `Rc<T>` 替代）。

---

### 总结：Trait 的收益

| Trait | 收益 | 何时使用 |
|-------|---------|-------------|
| `Debug` | `println!("{:?}", value)` | 几乎总是（极少数除外） |
| `Display` | `println!("{}", value)` | 面向用户的类型 |
| `Clone` | `value.clone()` | 显式复制有意义时 |
| `Copy` | 隐式复制 | 小型、简单类型 |
| `PartialEq` | `==` 和 `!=` 运算符 | 大多数类型 |
| `Eq` | 满足自反性的相等 | 相等具有数学健全性时 |
| `PartialOrd` | `<`, `>`, `<=`, `>=` | 具有自然顺序的类型 |
| `Ord` | `sort()`, `BinaryHeap` | 全序关系成立时 |
| `Hash` | `HashMap` 的键 | 类型需作为 Map 的键时 |
| `Default` | `Default::default()` | 有明显默认值的类型 |
| `From/Into` | 便捷转换 | 常用的类型转换 |
| `TryFrom/TryInto` | 可能失败的转换 | 转换逻辑可能失败时 |

---
