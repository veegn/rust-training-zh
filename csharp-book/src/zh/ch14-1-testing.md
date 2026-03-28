[English Original](../en/ch14-1-testing.md)

# 测试：内建的支持与进阶工具

> **你将学到什么：** Rust 内置 `#[test]` 与 xUnit 的对比，如何用 `rstest` 编写参数化测试，以及用 `mockall` 做 mock。
>
> **难度：** 中级

在 C# 中，测试往往需要依赖 xUnit、NUnit 或 MSTest 等外部框架。然而，Rust 直接在语言层面和 `cargo` 工具中提供了非常强大的内建测试框架。

---

## 单元测试 (Unit Testing)
Rust 的单元测试通常直接和它们要测试的代码写在同一个文件里，并被包裹在一个专门的子模块中。

### C# xUnit
```csharp
[Fact]
public void Add_ReturnsSum() {
    Assert.Equal(5, Calculator.Add(2, 3));
}
```

### Rust 内建测试
```rust
pub fn add(a: i32, b: i32) -> i32 { a + b }

#[cfg(test)] // 仅在运行 'cargo test' 时才编译
mod tests {
    use super::*; // 从父模块中导入 'add' 函数

    #[test]
    fn add_returns_sum() {
        assert_eq!(add(2, 3), 5);
    }
}
```

---

## 断言 (Assertions)
| **xUnit** | **Rust** | **说明** |
| :--- | :--- | :--- |
| `Assert.Equal(e, a)` | `assert_eq!(e, a)` | 最常用，判断相等 |
| `Assert.True(c)` | `assert!(c)` | 基本的布尔检查 |
| `Assert.Throws<T>(...)` | `#[should_panic]` | 标记在测试函数上的属性 |

---

## 参数化测试 (Parameterized Tests - `[Theory]`)
为了实现像 C# `[Theory]` 这样的功能，Rust 社区通常会使用 **`rstest`** 这个 Crate。

```rust
use rstest::rstest;

#[rstest]
#[case(1, 1, 2)]
#[case(2, 3, 5)]
fn test_add(#[case] a: i32, #[case] b: i32, #[case] expected: i32) {
    assert_eq!(add(a, b), expected);
}
```

---

## 使用 `mockall` 进行 Mock 
在 Rust 里，你通常使用 **`mockall`** 来生成 Trait 的 Mock 对象。这相当于 Moq 或 NSubstitute 在 C# 中的作用。

```rust
use mockall::automock;

#[automock]
trait Database {
    fn get_user_name(&self, id: i32) -> String;
}

#[test]
fn test_user_service() {
    let mut mock = MockDatabase::new();
    mock.expect_get_user_name()
        .with(mockall::predicate::eq(1))
        .returning(|_| "Alice".to_string());
    
    // 接下来开始使用这个 Mock...
}
```

---

## C# 开发者总结表
| **特性** | **C# / xUnit** | **Rust** |
| :--- | :--- | :--- |
| **测试执行器** | VS Test Run / `dotnet test` | `cargo test` |
| **测试位置** | 独立的 Test 项目 | 通常在同一个代码文件里 |
| **集成测试** | 独立的 Test 项目 | 特定的 `tests/` 目录 |
| **异步测试** | `async Task` 测试 | `#[tokio::test]` |
| **文档测试** | XML 注释 | `///` 注释中直接贴可运行代码！ |

---

## 练习：编写一个测试
**挑战：** 编写一个名为 `divide` 的函数，要求其返回 `Result<i32, String>`。并编写两个测试：一个用于验证正常除法，另一个用于验证除以零时返回了 `Err`。

```rust
fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 { Err("除以零".into()) }
    else { Ok(a / b) }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_ok() { assert!(divide(10, 2).is_ok()); }
    #[test]
    fn test_err() { assert!(divide(10, 0).is_err()); }
}
```
**关键理解：** 将测试作为语言的“头等公民”意味着每个 Rust 项目都有着非常一致的测试运行方式。所有你只需要记住的命令就是 `cargo test`。
