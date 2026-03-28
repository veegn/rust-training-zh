[English Original](../en/ch08-1-testing-patterns.md)

# 8.1 测试模式 🟢

Rust 内置了测试工具，使其无需第三方库即可轻松地编写和运行测试。

### 1. 单元测试 (Unit Tests)
单元测试通常写在被测试代码所在的文件中，位于一个被标记为 `#[cfg(test)]` 的 `tests` 模块内。

```rust
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*; // 将父级项带入作用域

    #[test]
    fn it_works() {
        assert_eq!(add(2, 2), 4);
    }
}
```

---

### 2. 集成测试 (Integration Tests)
集成测试对你的库而言是完全外部的。它们位于项目根目录下的 `tests` 目录中。

文件结构：
```text
├── src/
│   └── lib.rs
└── tests/
    └── integration_test.rs
```

在 `tests/integration_test.rs` 中：
```rust
use adder; // 导入你的库 Crate

#[test]
fn test_add() {
    assert_eq!(adder::add(3, 2), 5);
}
```

---

### 3. 常用的测试宏
- `assert!(condition)`：如果条件为假，则发生 Panic。
- `assert_eq!(left, right)`：如果 left != right，则发生 Panic。
- `assert_ne!(left, right)`：如果 left == right，则发生 Panic。

---

### 4. 处理预期的 Panic
你可以通过添加 `#[should_panic]` 属性来测试函数在应当发生 Panic 时是否确实发生了。

```rust
#[test]
#[should_panic(expected = "猜测值必须小于或等于 100")]
fn greater_than_100() {
    Guess::new(200);
}
```

---

### 5. 运行测试
使用 Cargo 运行项目中的所有测试：
```bash
cargo test
```

常用可选参数：
- `cargo test -- --nocapture`：显示已通过测试的输出（如 `println!`）。
- `cargo test test_name`：仅运行与指定名称匹配的测试。

---

### 对 C/C++ 开发者的总结
- **在 C/C++ 中**：你经常使用诸如 GTest 或 Catch2 之类的框架。你需要配置你的构建系统（CMake/Make）来编译并运行它们。
- **在 Rust 中**：测试是一等公民。`cargo test` 处理了一切，且语法是语言本身的一部分。

***
