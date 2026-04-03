[English Original](../en/ch14-1-testing.md)

## Rust 中的测试与 C# 对比

> **你将学到：** 内置的 `#[test]` 与 xUnit 的对比；使用 `rstest` 实现的参数化测试（类似于 `[Theory]`）；使用 `proptest` 进行属性测试；使用 `mockall` 进行 Mock；以及异步测试模式。
>
> **难度：** 🟡 中级

### 单元测试
```csharp
// C# — xUnit
using Xunit;

public class CalculatorTests
{
    [Fact]
    public void Add_ReturnsSum()
    {
        var calc = new Calculator();
        Assert.Equal(5, calc.Add(2, 3));
    }

    [Theory]
    [InlineData(1, 2, 3)]
    [InlineData(0, 0, 0)]
    [InlineData(-1, 1, 0)]
    public void Add_Theory(int a, int b, int expected)
    {
        Assert.Equal(expected, new Calculator().Add(a, b));
    }
}
```

```rust
// Rust — 内置测试支持，无需外部框架
pub fn add(a: i32, b: i32) -> i32 { a + b }

#[cfg(test)]  // 仅在执行 `cargo test` 时编译
mod tests {
    use super::*;  // 从父模块导入

    #[test]
    fn add_returns_sum() {
        assert_eq!(add(2, 3), 5);
    }

    #[test]
    fn add_negative_numbers() {
        assert_eq!(add(-1, 1), 0);
    }

    #[test]
    #[should_panic(expected = "overflow")]
    fn add_overflow_panics() {
        let _ = add(i32::MAX, 1); // 在 debug 模式下会发生 panic
    }
}
```

### 参数化测试 (类似于 `[Theory]`)
```rust
// 使用 `rstest` crate 进行参数化测试
use rstest::rstest;

#[rstest]
#[case(1, 2, 3)]
#[case(0, 0, 0)]
#[case(-1, 1, 0)]
fn test_add(#[case] a: i32, #[case] b: i32, #[case] expected: i32) {
    assert_eq!(add(a, b), expected);
}

// Fixtures —— 类似于测试设置 (Setup) 方法
#[rstest]
fn test_with_fixture(#[values(1, 2, 3)] x: i32) {
    assert!(x > 0);
}
```

### 断言对比 (Assertions Comparison)

| C# (xUnit) | Rust | 备注 |
|-------------|------|-------|
| `Assert.Equal(expected, actual)` | `assert_eq!(expected, actual)` | 失败时打印差异 (Diff) |
| `Assert.NotEqual(a, b)` | `assert_ne!(a, b)` | |
| `Assert.True(condition)` | `assert!(condition)` | |
| `Assert.Contains("sub", str)` | `assert!(str.contains("sub"))` | |
| `Assert.Throws<T>(() => ...)` | `#[should_panic]` | 也可以使用 `std::panic::catch_unwind` |
| `Assert.Null(obj)` | `assert!(option.is_none())` | 无 null —— 使用 `Option` |

### 测试组织结构

```text
my_crate/
├── src/
│   ├── lib.rs          # 单元测试写在 #[cfg(test)] mod tests { } 中
│   └── parser.rs       # 每个模块都可以有自己的测试子模块
├── tests/              # 集成测试 (每个文件被视为一个独立的 Crate)
│   ├── parser_test.rs  # 作为外部消费者测试公有 API
│   └── api_test.rs
└── benches/            # 基准测试 (使用 criterion crate)
    └── my_benchmark.rs
```

```rust
// tests/parser_test.rs —— 集成测试
// 仅能访问公有 (PUBLIC) API (类似于从程序集外部进行测试)
use my_crate::parser;

#[test]
fn test_parse_valid_input() {
    let result = parser::parse("有效的输入数据");
    assert!(result.is_ok());
}
```

### 异步测试
```csharp
// C# — 使用 xUnit 进行异步测试
[Fact]
public async Task GetUser_ReturnsUser()
{
    var service = new UserService();
    var user = await service.GetUserAsync(1);
    Assert.Equal("Alice", user.Name);
}
```

```rust
// Rust — 使用 tokio 进行异步测试
#[tokio::test]
async fn get_user_returns_user() {
    let service = UserService::new();
    let user = service.get_user(1).await.unwrap();
    assert_eq!(user.name, "Alice");
}
```

### 使用 mockall 进行 Mock
```rust
use mockall::automock;

#[automock]                         // 自动生成 MockUserRepo 结构体
trait UserRepo {
    fn find_by_id(&self, id: u32) -> Option<User>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn service_returns_user_from_repo() {
        let mut mock = MockUserRepo::new();
        mock.expect_find_by_id()
            .with(mockall::predicate::eq(1))
            .returning(|_| Some(User { name: "Alice".into() }));

        let service = UserService::new(mock);
        let user = service.get_user(1).unwrap();
        assert_eq!(user.name, "Alice");
    }
}
```

```csharp
// C# — Moq 等效写法
var mock = new Mock<IUserRepo>();
mock.Setup(r => r.FindById(1)).Returns(new User { Name = "Alice" });
var service = new UserService(mock.Object);
Assert.Equal("Alice", service.GetUser(1).Name);
```

---

<details>
<summary><strong>🏋️ 练习：编写全面的测试用例</strong> (点击展开)</summary>

**挑战**：针对以下函数，编写涵盖以下情况的测试：正常路径 (Happy path)、空输入、数字字符串以及 Unicode 字符。

```rust
pub fn title_case(input: &str) -> String {
    input.split_whitespace()
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                Some(c) => format!("{}{}", c.to_uppercase(), chars.as_str().to_lowercase()),
                None => String::new(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}
```

<details>
<summary>🔑 参考答案</summary>

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn happy_path() {
        assert_eq!(title_case("hello world"), "Hello World");
    }

    #[test]
    fn empty_input() {
        assert_eq!(title_case(""), "");
    }

    #[test]
    fn single_word() {
        assert_eq!(title_case("rust"), "Rust");
    }

    #[test]
    fn already_title_case() {
        assert_eq!(title_case("Hello World"), "Hello World");
    }

    #[test]
    fn all_caps() {
        assert_eq!(title_case("HELLO WORLD"), "Hello World");
    }

    #[test]
    fn extra_whitespace() {
        // split_whitespace 可以处理多个空格
        assert_eq!(title_case("  hello   world  "), "Hello World");
    }

    #[test]
    fn unicode() {
        assert_eq!(title_case("café résumé"), "Café Résumé");
    }

    #[test]
    fn numeric_words() {
        assert_eq!(title_case("hello 42 world"), "Hello 42 World");
    }
}
```

**关键收获**：Rust 内置的测试框架满足了大多数单元测试的需求。对于参数化测试可以使用 `rstest`，对于 Mock 可以使用 `mockall` —— 无需像 xUnit 这样的大型测试框架。

</details>
</details>

## 属性测试 (Property Testing)：在大规模下证明正确性

熟悉 **FsCheck** 的 C# 开发者会认出属性测试：你不再编写单个测试用例，而是描述必须对**所有可能输入**都成立的“属性”，框架会生成数以千计的随机输入来尝试破坏这些属性。

### 为什么属性测试很重要
```csharp
// C# — 手写的单元测试检查特定案例
[Fact]
public void Reverse_Twice_Returns_Original()
{
    var list = new List<int> { 1, 2, 3 };
    list.Reverse();
    list.Reverse();
    Assert.Equal(new[] { 1, 2, 3 }, list);
}
// 但空列表呢？单个元素呢？10,000 个元素呢？负数呢？
// 你需要手写几十个案例。
```

```rust
// Rust — proptest 自动生成数千个输入
use proptest::prelude::*;

fn reverse<T: Clone>(v: &[T]) -> Vec<T> {
    v.iter().rev().cloned().collect()
}

proptest! {
    #[test]
    fn reverse_twice_is_identity(ref v in prop::collection::vec(any::<i32>(), 0..1000)) {
        let reversed_twice = reverse(&reverse(v));
        prop_assert_eq!(v, &reversed_twice);
    }
    // proptest 使用数百个随机 Vec<i32> 值运行此测试：
    // [], [0], [i32::MIN, i32::MAX], [42; 999], 随机序列等...
    // 如果失败了，它会“收缩 (Shrink)”到导致失败的最小输入！
}
```

### 开始使用 proptest
```toml
# Cargo.toml
[dev-dependencies]
proptest = "1.4"
```

### C# 开发者的常用模式

```rust
use proptest::prelude::*;

// 1. 来回转换属性：序列化 → 反序列化 = 原始值
// (类似于测试 JsonSerializer.Serialize → Deserialize)
proptest! {
    #[test]
    fn json_roundtrip(name in "[a-zA-Z]{1,50}", age in 0u32..150) {
        let user = User { name: name.clone(), age };
        let json = serde_json::to_string(&user).unwrap();
        let parsed: User = serde_json::from_str(&json).unwrap();
        prop_assert_eq!(user, parsed);
    }
}

// 2. 恒定属性：输出始终满足某个条件
proptest! {
    #[test]
    fn sort_output_is_sorted(ref v in prop::collection::vec(any::<i32>(), 0..500)) {
        let mut sorted = v.clone();
        sorted.sort();
        // 每一组相邻对必须是有序的
        for window in sorted.windows(2) {
            prop_assert!(window[0] <= window[1]);
        }
    }
}

// 3. 先知 (Oracle) 属性：比较两个实现版本
proptest! {
    #[test]
    fn fast_path_matches_slow_path(input in "[0-9a-f]{1,100}") {
        let result_fast = parse_hex_fast(&input);
        let result_slow = parse_hex_slow(&input);
        prop_assert_eq!(result_fast, result_slow);
    }
}

// 4. 自定义策略：生成领域特定的测试数据
fn valid_email() -> impl Strategy<Value = String> {
    ("[a-z]{1,20}", "[a-z]{1,10}", prop::sample::select(vec!["com", "org", "io"]))
        .prop_map(|(user, domain, tld)| format!("{}@{}.{}", user, domain, tld))
}

proptest! {
    #[test]
    fn email_parsing_accepts_valid_emails(email in valid_email()) {
        let result = Email::new(&email);
        prop_assert!(result.is_ok(), "解析失败：{}", email);
    }
}
```

### proptest 与 FsCheck 对比

| 特性 | C# FsCheck | Rust proptest |
|---------|-----------|---------------|
| 随机输入生成 | `Arb.Generate<T>()` | `any::<T>()` |
| 自定义生成器 | `Arb.Register<T>()` | `impl Strategy<Value = T>` |
| 失败时自动收缩 | 自动 | 自动 |
| 字符串模式 | 手动 | `"[正则表达式]"` 策略 |
| 集合生成 | `Gen.ListOf` | `prop::collection::vec(策略, 范围)` |
| 组合生成器 | `Gen.Select` | `.prop_map()`, `.prop_flat_map()` |
| 配置项（案例数） | `Config.MaxTest` | 在 `proptest!` 块内使用配置属性 |

### 何时使用属性测试 vs 单元测试

| 使用**单元测试**场景 | 使用 **proptest** 场景 |
|------------------------|----------------------|
| 测试特定的边界情况 | 验证在所有输入下都成立的恒定性 |
| 测试错误消息或错误码 | 来回转换属性 (解析 ↔ 格式化) |
| 集成测试 / Mock 测试 | 比较两个不同算法的实现 |
| 行为取决于精确的特定值 | “对于所有的 X，属性 P 均成立” |

---

## 集成测试：`tests/` 目录

单元测试通过 `#[cfg(test)]` 存在于 `src/` 中。集成测试存在于独立的 `tests/` 目录中，并测试 Crate 的**公有 API** —— 就像 C# 的集成测试将项目作为外部程序集引用一样。

```
my_crate/
├── src/
│   ├── lib.rs          // 公有 API
│   └── internal.rs     // 私有实现
├── tests/
│   ├── smoke.rs        // 每个文件是一个单独的测试二进制文件
│   ├── api_tests.rs
│   └── common/
│       └── mod.rs      // 共享的测试辅助代码
└── Cargo.toml
```

### 编写集成测试

`tests/` 下的每个文件都被编译为依赖于 your library 的独立 Crate：

```rust
// tests/smoke.rs —— 仅能访问 my_crate 的 pub 项
use my_crate::{process_order, Order, OrderResult};

#[test]
fn process_valid_order_returns_confirmation() {
    let order = Order::new("SKU-001", 3);
    let result = process_order(order);
    assert!(matches!(result, OrderResult::Confirmed { .. }));
}
```

### 共享测试辅助工具

将共享的设置 (Setup) 代码放在 `tests/common/mod.rs` 中（不要叫 `tests/common.rs`，否则它会被当成一个独立的测试文件）：

```rust
// tests/common/mod.rs
use my_crate::Config;

pub fn test_config() -> Config {
    Config::builder()
        .database_url("sqlite::memory:")
        .build()
        .expect("测试配置必须有效")
}
```

```rust
// tests/api_tests.rs
mod common;

use my_crate::App;

#[test]
fn app_starts_with_test_config() {
    let config = common::test_config();
    let app = App::new(config);
    assert!(app.is_healthy());
}
```

### 运行特定类型的测试

```bash
cargo test                  # 运行所有测试 (单元 + 集成)
cargo test --lib            # 仅运行单元测试
cargo test --test smoke     # 仅运行 tests/smoke.rs
cargo test --test api_tests # 仅运行 tests/api_tests.rs
```

**与 C# 的关键区别：** 集成测试文件只能访问你 Crate 的 `pub` API。私有函数是不可见的 —— 这迫使你通过公共接口进行测试，这通常是更好的测试设计实践。
