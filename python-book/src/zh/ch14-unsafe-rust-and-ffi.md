[English Original](../en/ch14-unsafe-rust-and-ffi.md)

# 14. 非安全 Rust 与 FFI 🔴

> **你将学到：**
> - 何时使用 `unsafe` (以及为什么你可能永远用不上它)
> - **PyO3**：让 Rust 编写 Python 扩展从未如此简单
> - 内建单元测试 vs `pytest`
> - 模拟 (Mocking) 与 性能基准测试

## 什么是 Unsafe？

在 Rust 中，`unsafe` 关键字是在告诉编译器：“我知道自己在做什么，这里请允许我跳过那些内存安全检查。”它主要用于编译器无法验证的行为，如：
- 解引用原始指针。
- 调用其他语言（如 C 或 Python）编写的函数。

**黄金法则：** 99% 的代码都应该是 Safe Rust。只有在需要极致性能或与外部语言交互时，才在小范围内谨慎使用 `unsafe`。

---

## PyO3：用 Rust 写 Python 扩展 🐍

PyO3 是给 Python 开发者的“终极神器”。它允许你编写 Rust 函数和类，并直接将其包成 Python 模块，在 Python 脚本里直接 `import`。

### 写一个被 Python 调用的 Rust 函数
```rust
use pyo3::prelude::*;

#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string()) // 计算结果作为字符串返回给 Python
}

#[pymodule]
fn my_rust_lib(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    Ok(())
}
```

### 为什么要用 PyO3？
1. **性能飞跃**：用 Rust 替换掉 Python 代码中那些慢得离谱的循环，性能提升几十倍。
2. **内存安全**：传统的 C 扩展极易导致内存错误，而 Rust 从根源上保证了扩展模块的安全性。
3. **生态融合**：你可以在 Python 项目里无缝使用 Rust 庞大的生态系统（比如 `serde` 或 `tokio`）。

---

## 测试：Rust vs Pytest

Rust 拥有极其强大的内建测试跑数器。通常情况下，测试代码就写在被测试文件的最下方的一个 `tests` 子模块里。

| 核心功能 | Pytest | Rust |
|---------|--------|------|
| 运行所有测试 | `pytest` | `cargo test` |
| 等值断言 | `assert x == y` | `assert_eq!(x, y)` |
| 期望报错 | `pytest.raises(Err)`| `#[should_panic]` |
| 标记慢速测试 | `pytest.mark.slow` | `#[ignore]` |

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(2 + 2, 4);
    }
}
```

---

## 练习

<details>
<summary><strong>🏋️ 练习：第一个 PyO3 模块</strong> (点击展开)</summary>

**挑战**：调研 `maturin` 工具。在命令行中，如何快速初始化一个 Rust-Python 混合项目？

<details>
<summary>参考答案</summary>

直接运行 `maturin init`。它会自动创建包含 `Cargo.toml` (带有 `pyo3`) 和 `pyproject.toml` 的项目骨架，以便直接进行构建和打包。

</details>
</details>

***
