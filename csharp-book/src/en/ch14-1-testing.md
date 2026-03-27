# Testing: Built-in and Beyond

> **What you'll learn:** Built-in `#[test]` vs xUnit, parameterized tests with `rstest`, and mocking with `mockall`.
>
> **Difficulty:** Intermediate

In C#, you depend on external frameworks like xUnit, NUnit, or MSTest. Rust, however, provides a powerful testing framework built directly into the language and `cargo`.

---

## Unit Testing
Unit tests in Rust usually live in the same file as the code they test, inside a special module.

### C# xUnit
```csharp
[Fact]
public void Add_ReturnsSum() {
    Assert.Equal(5, Calculator.Add(2, 3));
}
```

### Rust Built-in Test
```rust
pub fn add(a: i32, b: i32) -> i32 { a + b }

#[cfg(test)] // Only compiles when running 'cargo test'
mod tests {
    use super::*; // Import 'add' from the parent module

    #[test]
    fn add_returns_sum() {
        assert_eq!(add(2, 3), 5);
    }
}
```

---

## Assertions
| **xUnit** | **Rust** | **Notes** |
| :--- | :--- | :--- |
| `Assert.Equal(e, a)` | `assert_eq!(e, a)` | Most common |
| `Assert.True(c)` | `assert!(c)` | Basic boolean check |
| `Assert.Throws<T>(...)` | `#[should_panic]` | Attribute on the test function |

---

## Parameterized Tests (`[Theory]`)
To get functionality like C#'s `[Theory]`, the Rust community uses the **`rstest`** crate.

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

## Mocking with `mockall`
Instead of Moq or NSubstitute, Rust developers often use **`mockall`**. It generates mock objects for traits.

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
    
    // Use the mock...
}
```

---

## Summary for C# Developers
| **Feature** | **C# / xUnit** | **Rust** |
| :--- | :--- | :--- |
| **Test Runner** | Visual Studio / `dotnet test` | `cargo test` |
| **Test Location** | Separate Project | Same File (usually) |
| **Integration Tests** | Separate Project | `tests/` directory |
| **Async Test** | `async Task` test | `#[tokio::test]` |
| **Documentation** | XML Comments | `///` with runnable code! |

---

## Exercise: Write a Test
**Challenge:** Add a `divide` function that returns `Result<i32, String>`. Write two tests: one for a successful division and one that checks if it returns an `Err` when dividing by zero.

```rust
fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 { Err("Devide by zero".into()) }
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
**Takeaway:** Having testing as a first-class citizen means every Rust crate has a consistent way to run tests. `cargo test` is all you need to remember.
