# Rust Best Practices Summary / Rust 最佳实践总结
 
 > **What you'll learn / 你将学到：** Practical guidelines for writing idiomatic Rust — code organization, naming conventions, error handling patterns, and documentation. A quick-reference chapter you'll return to often.
 >
 > 编写地道（idiomatic）Rust 代码的实用指南 —— 包括代码组织、命名规范、错误处理模式及文档编写。这是一个你会经常查阅的快速参考章节。
 
- ## Code Organization
+ ## Code Organization / 代码组织
- - **Prefer small functions**: Easy to test and reason about
+ - **优先使用小型函数**：易于测试和推理。
- - **Use descriptive names**: `calculate_total_price()` vs `calc()`
+ - **使用描述性名称**：例如 `calculate_total_price()` 优于 `calc()`。
- - **Group related functionality**: Use modules and separate files
+ - **对相关功能进行分组**：使用模块（module）和独立的文件。
- - **Write documentation**: Use `///` for public APIs
+ - **编写文档**：为公开 API 使用 `///` 注释。
 
- ## Error Handling
+ ## Error Handling / 错误处理
- - **Avoid `unwrap()` unless infallible**: Only use when you're 100% certain it won't panic
+ - **除非确定不会出错，否则避免使用 `unwrap()`**：仅当你 100% 确定不会发生 panic 时才使用它。
 ```rust
- // Bad: Can panic
+ // Bad: Can panic / 坏习惯：可能导致 panic
 let value = some_option.unwrap();
 
- // Good: Handle the None case
+ // Good: Handle None / 好习惯：处理 None 的情况
 let value = some_option.unwrap_or(default_value);
 let value = some_option.unwrap_or_else(|| expensive_computation());
- let value = some_option.unwrap_or_default(); // Uses Default trait
+ let value = some_option.unwrap_or_default(); // 使用 Default trait
 
- // For Result<T, E>
+ // For Result<T, E> / 针对 Result<T, E>
 let value = some_result.unwrap_or(fallback_value);
 let value = some_result.unwrap_or_else(|err| {
     eprintln!("Error occurred: {err}");
     default_value
 });
 ```
- - **Use `expect()` with descriptive messages**: When unwrap is justified, explain why
+ - **使用带有描述性消息的 `expect()`**：当使用 unwrap 是合理的时候，请解释理由。
 ```rust
 let config = std::env::var("CONFIG_PATH")
     .expect("CONFIG_PATH environment variable must be set");
 ```
- - **Return `Result<T, E>` for fallible operations**: Let callers decide how to handle errors
+ - **对可能失败的操作返回 `Result<T, E>`**：让调用者决定如何处理错误。
- - **Use `thiserror` for custom error types**: More ergonomic than manual implementations
+ - **为自定义错误类型使用 `thiserror`**：比手动实现更符合人体工程学。
 ```rust
 use thiserror::Error;
 
 #[derive(Error, Debug)]
 pub enum MyError {
-    #[error("IO error: {0}")]
+    #[error("IO error: {0}")] // IO 错误
     Io(#[from] std::io::Error),
     
-    #[error("Parse error: {message}")]
+    #[error("Parse error: {message}")] // 解析错误
     Parse { message: String },
     
-    #[error("Value {value} is out of range")]
+    #[error("Value {value} is out of range")] // 数值越界
     OutOfRange { value: i32 },
 }
 ```
- - **Chain errors with `?` operator**: Propagate errors up the call stack
+ - **使用 `?` 运算符链接错误**：将错误沿调用栈向上传播。
- - **Prefer `thiserror` over `anyhow`**: Our team convention is to define explicit error
-   enums with `#[derive(thiserror::Error)]` so callers can match on specific variants.
-   `anyhow::Error` is convenient for quick prototyping but erases the error type, making
-   it harder for callers to handle specific failures. Use `thiserror` for library and
-   production code; reserve `anyhow` for throwaway scripts or top-level binaries where
-   you only need to print the error.
+ - **优先使用 `thiserror` 而非 `anyhow`**：我们团队的规范是使用 `#[derive(thiserror::Error)]` 定义明确的错误枚举，以便调用者可以对特定变体进行匹配。`anyhow::Error` 虽然在快速原型设计时很方便，但它会抹消错误类型，导致调用者难以处理特定的失败情况。在库和生产代码中使用 `thiserror`；将 `anyhow` 留给一次性脚本或只需要打印错误信息的顶级二进制程序。
- - **When `unwrap()` is acceptable**:
+ - **何时 `unwrap()` 是可以接受的**：
-   - **Unit tests**: `assert_eq!(result.unwrap(), expected)`
+   - **单元测试**：`assert_eq!(result.unwrap(), expected)`
-   - **Prototyping**: Quick and dirty code that you'll replace
+   - **原型设计**：那些之后会被替换掉的临时代码
-   - **Infallible operations**: When you can prove it won't fail
+   - **必然成功的操作**：当你能证明它绝不会失败时
 ```rust
 let numbers = vec![1, 2, 3];
- let first = numbers.get(0).unwrap(); // Safe: we just created the vec with elements
+ let first = numbers.get(0).unwrap(); // Safe / 安全：我们刚刚创建了带元素的 vec
 
- // Better: Use expect() with explanation
+ // Better / 更好：使用带有解释的 expect()
 let first = numbers.get(0).expect("numbers vec is non-empty by construction");
 ```
- - **Fail fast**: Check preconditions early and return errors immediately
+ - **及早失效（Fail fast）**：尽早检查先决条件并立即返回错误。
 
- ## Memory Management
+ ## Memory Management / 内存管理
- - **Prefer borrowing over cloning**: Use `&T` instead of cloning when possible
+ - **借用优于克隆**：尽可能使用 `&T` 而不是克隆。
- - **Use `Rc<T>` sparingly**: Only when you need shared ownership
+ - **审慎使用 `Rc<T>`**：仅在确实需要共享所有权时使用。
- - **Limit lifetimes**: Use scopes `{}` to control when values are dropped
+ - **限制生命周期**：使用作用域 `{}` 来控制值的释放时机。
- - **Avoid `RefCell<T>` in public APIs**: Keep interior mutability internal
+ - **避免在公开 API 中使用 `RefCell<T>`**：将内部可变性保持在内部。
 
- ## Performance
+ ## Performance / 性能
- - **Profile before optimizing**: Use `cargo bench` and profiling tools
+ - **先分析（Profile）再优化**：使用 `cargo bench` 和性能分析工具。
- - **Prefer iterators over loops**: More readable and often faster
+ - **迭代器优于循环**：更具可读性，通常也更快。
- - **Use `&str` over `String`**: When you don't need ownership
+ - **使用 `&str` 而非 `String`**：当你不需要所有权时。
- - **Consider `Box<T>` for large stack objects**: Move them to heap if needed
+ - **对于大型栈对象考虑使用 `Box<T>`**：如果需要，将它们移动到堆上。
 
- ## Essential Traits to Implement
+ ## Essential Traits to Implement / 必装实现的 Trait
 
- ### Core Traits Every Type Should Consider
+ ### Core Traits Every Type Should Consider / 每个类型都应考虑的核心 Trait
 
- When creating custom types, consider implementing these fundamental traits to make your types feel native to Rust:
+ 在创建自定义类型时，考虑实现这些基础 trait，让你的类型感觉像是 Rust 原生的：
 
- #### **Debug and Display**
+ #### **Debug 和 Display**
 ```rust
 use std::fmt;
 
- #[derive(Debug)]  // Automatic implementation for debugging
+ #[derive(Debug)]  // 为调试自动实现
 struct Person {
     name: String,
     age: u32,
 }
 
- // Manual Display implementation for user-facing output
+ // 为面向用户的输出手动实现 Display
 impl fmt::Display for Person {
     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
         write!(f, "{} (age {})", self.name, self.age)
     }
 }
 
- // Usage:
+ // Usage / 用法：
 let person = Person { name: "Alice".to_string(), age: 30 };
- println!("{:?}", person);  // Debug: Person { name: "Alice", age: 30 }
+ println!("{:?}", person);  // Debug 输出: Person { name: "Alice", age: 30 }
- println!("{}", person);    // Display: Alice (age 30)
+ println!("{}", person);    // Display 输出: Alice (age 30)
 ```
 
- #### **Clone and Copy**
+ #### **Clone 和 Copy**
 ```rust
- // Copy: Implicit duplication for small, simple types
+ // Copy: 为小型简单类型提供隐式复制
 #[derive(Debug, Clone, Copy)]
 struct Point {
     x: i32,
     y: i32,
 }
 
- // Clone: Explicit duplication for complex types
+ // Clone: 为复杂类型提供显式复制
 #[derive(Debug, Clone)]
 struct Person {
-    name: String,  // String doesn't implement Copy
+    name: String,  // String 没有实现 Copy
     age: u32,
 }
 
 let p1 = Point { x: 1, y: 2 };
- let p2 = p1;  // Copy (implicit)
+ let p2 = p1;  // Copy (隐式)
 
 let person1 = Person { name: "Bob".to_string(), age: 25 };
- let person2 = person1.clone();  // Clone (explicit)
+ let person2 = person1.clone();  // Clone (显式)
 ```
 
- #### **PartialEq and Eq**
+ #### **PartialEq 和 Eq**
 ```rust
 #[derive(Debug, PartialEq, Eq)]
 struct UserId(u64);
 
 #[derive(Debug, PartialEq)]
 struct Temperature {
-    celsius: f64,  // f64 doesn't implement Eq (due to NaN)
+    celsius: f64,  // f64 没有实现 Eq (由于 NaN 的存在)
 }
 
 let id1 = UserId(123);
 let id2 = UserId(123);
- assert_eq!(id1, id2);  // Works because of PartialEq
+ assert_eq!(id1, id2);  // 由于实现了 PartialEq，此处成立
 
 let temp1 = Temperature { celsius: 20.0 };
 let temp2 = Temperature { celsius: 20.0 };
- assert_eq!(temp1, temp2);  // Works with PartialEq
+ assert_eq!(temp1, temp2);  // 由于实现了 PartialEq，此处成立
 ```
 
- #### **PartialOrd and Ord**
+ #### **PartialOrd 和 Ord**
 ```rust
 #[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
 struct Priority(u8);
 
 let high = Priority(1);
 let low = Priority(10);
- assert!(high < low);  // Lower numbers = higher priority
+ assert!(high < low);  // 数字越小 = 优先级越高
 
- // Use in collections
+ // 在集合中使用
 let mut priorities = vec![Priority(5), Priority(1), Priority(8)];
- priorities.sort();  // Works because Priority implements Ord
+ priorities.sort();  // 由于 Priority 实现了 Ord，此处可以排序
 ```
 
- #### **Default**
+ #### **Default**
 ```rust
 #[derive(Debug, Default)]
 struct Config {
-    debug: bool,           // false (default)
+    debug: bool,           // 默认 false
-    max_connections: u32,  // 0 (default)
+    max_connections: u32,  // 默认 0
-    timeout: Option<u64>,  // None (default)
+    timeout: Option<u64>,  // 默认 None
 }
 
- // Custom Default implementation
+ // 手动实现 Default
 impl Default for Config {
     fn default() -> Self {
         Config {
             debug: false,
-            max_connections: 100,  // Custom default
+            max_connections: 100,  // 自定义默认值
-            timeout: Some(30),     // Custom default
+            timeout: Some(30),     // 自定义默认值
         }
     }
 }
 
 let config = Config::default();
- let config = Config { debug: true, ..Default::default() };  // Partial override
+ let config = Config { debug: true, ..Default::default() };  // 部分覆盖
 ```
 
- #### **From and Into**
+ #### **From 和 Into**
 ```rust
 struct UserId(u64);
 struct UserName(String);
 
- // Implement From, and Into comes for free
+ // 实现 From 之后，Into 也就自动获得了
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
 
- // Usage:
+ // Usage / 用法：
- let user_id: UserId = 123u64.into();         // Using Into
+ let user_id: UserId = 123u64.into();         // 使用 Into
- let user_id = UserId::from(123u64);          // Using From
+ let user_id = UserId::from(123u64);          // 使用 From
- let username = UserName::from("alice");      // &str -> UserName
+ let username = UserName::from("alice");      // &str 转为 UserName
- let username: UserName = "bob".into();       // Using Into
+ let username: UserName = "bob".into();       // 使用 Into
 ```
 
- #### **TryFrom and TryInto**
+ #### **TryFrom 和 TryInto**
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
 
- // Usage:
- let positive = PositiveNumber::try_from(42)?;     // Ok(PositiveNumber(42))
- let error = PositiveNumber::try_from(-5);         // Err(NegativeNumberError)
+ // Usage / 用法：
+ let positive = PositiveNumber::try_from(42)?;     // 成功转换
+ let error = PositiveNumber::try_from(-5);         // 转换失败
 ```
 
- #### **Serde (for serialization)**
+ #### **Serde（用于序列化）**
 ```rust
 use serde::{Deserialize, Serialize};
 
 #[derive(Debug, Serialize, Deserialize)]
 struct User {
     id: u64,
     name: String,
     email: String,
 }
 
- // Automatic JSON serialization/deserialization
+ // 自动 JSON 序列化/反序列化
 let user = User {
     id: 1,
     name: "Alice".to_string(),
     email: "alice@example.com".to_string(),
 };
 
 let json = serde_json::to_string(&user)?;
 let deserialized: User = serde_json::from_str(&json)?;
 ```
 
- ### Trait Implementation Checklist
+ ### Trait Implementation Checklist / Trait 实现检查表
 
- For any new type, consider this checklist:
+ 对于任何新类型，请考虑以下检查表：
 
 ```rust
 #[derive(
-     Debug,          // [OK] Always implement for debugging
+     Debug,          // [推荐] 始终为调试实现
-     Clone,          // [OK] If the type should be duplicatable
+     Clone,          // [依据情况] 如果类型应该能够复制
-     PartialEq,      // [OK] If the type should be comparable
+     PartialEq,      // [依据情况] 如果类型应该能够比较
-     Eq,             // [OK] If comparison is reflexive/transitive
+     Eq,             // [依据情况] 如果比较满足自反性/传递性
-     PartialOrd,     // [OK] If the type has ordering
+     PartialOrd,     // [依据情况] 如果类型具有顺序
-     Ord,            // [OK] If ordering is total
+     Ord,            // [依据情况] 如果顺序是全序（total）
-     Hash,           // [OK] If type will be used as HashMap key
+     Hash,           // [依据情况] 如果类型将用作 HashMap 的键
-     Default,        // [OK] If there's a sensible default value
+     Default,        // [依据情况] 如果存在合理的默认值
 )]
 struct MyType {
-    // fields...
+    // fields... / 字段...
 }
 
- // Manual implementations to consider:
+ // 值得考虑的手动实现：
- impl Display for MyType { /* user-facing representation */ }
+ impl Display for MyType { /* 面向用户的表示 */ }
- impl From<OtherType> for MyType { /* convenient conversion */ }
+ impl From<OtherType> for MyType { /* 便捷转换 */ }
- impl TryFrom<FallibleType> for MyType { /* fallible conversion */ }
+ impl TryFrom<FallibleType> for MyType { /* 可能失败的转换 */ }
 ```
 
- ### When NOT to Implement Traits
+ ### When NOT to Implement Traits / 何时不应实现 Trait
 
- - **Don't implement Copy for types with heap data**: `String`, `Vec`, `HashMap` etc.
+ - **不要为带有堆数据的类型实现 Copy**：如 `String`、`Vec`、`HashMap` 等。
- - **Don't implement Eq if values can be NaN**: Types containing `f32`/`f64`
+ - **如果值可能为 NaN，不要实现 Eq**：如包含 `f32`/`f64` 的类型。
- - **Don't implement Default if there's no sensible default**: File handles, network connections
+ - **如果没有合理的默认值，不要实现 Default**：如文件句柄、网络连接等。
- - **Don't implement Clone if cloning is expensive**: Large data structures (consider `Rc<T>` instead)
+ - **如果克隆代价高昂，不要实现 Clone**：如大型数据结构（考虑使用 `Rc<T>` 代替）。
 
- ### Summary: Trait Benefits
+ ### Summary: Trait Benefits / 总结：Trait 的优势
 
-| Trait | Benefit | When to Use |
+| **Trait** | **Benefit / 优势** | **When to Use / 何时使用** |
 |-------|---------|-------------|
-| `Debug` | `println!("{:?}", value)` | Always (except rare cases) |
+| `Debug` | `{:?}` padding / 调试打印 | Always / 始终 (极少数情况除外) |
-| `Display` | `println!("{}", value)` | User-facing types |
+| `Display` | `{}` user output / 面向用户的输出 | User-facing types / 用户可见类型 |
-| `Clone` | `value.clone()` | When explicit duplication makes sense |
+| `Clone` | Explicit duplication / 显式复制 | When sensible / 当显式复制合理时 |
-| `Copy` | Implicit duplication | Small, simple types |
+| `Copy` | Implicit duplication / 隐式复制 | Simple types / 简单类型 |
-| `PartialEq` | `==` and `!=` operators | Most types |
+| `PartialEq` | `==` and `!=` / 相等性判别 | Most types / 大多数类型 |
-| `Eq` | Reflexive equality | When equality is mathematically sound |
+| `Eq` | Total equality / 全等性 | When logical / 逻辑上合理时 |
-| `PartialOrd` | `<`, `>`, `<=`, `>=` | Types with natural ordering |
+| `PartialOrd` | Comparisons / 大小比较 | Natural order / 具有自然顺序时 |
-| `Ord` | `sort()`, `BinaryHeap` | When ordering is total |
+| `Ord` | Sorting / 排序支持 | Total order / 全序关系 |
-| `Hash` | `HashMap` keys | Types used as map keys |
+| `Hash` | `HashMap` keys / 字典键 | As map keys / 作为 key 时 |
-| `Default` | `Default::default()` | Types with obvious defaults |
+| `Default` | `default()` / 默认构造 | Obvious defaults / 有明显默认值时 |
-| `From/Into` | Convenient conversions | Common type conversions |
+| `From/Into` | Conversions / 转换 | Type mapping / 类型转换 |
-| `TryFrom/TryInto` | Fallible conversions | Conversions that can fail |
+| `TryFrom/TryInto` | Fallible / 易错转换 | Can fail / 可能失败的转换 |
