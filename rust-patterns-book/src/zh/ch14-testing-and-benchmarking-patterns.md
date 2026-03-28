[English Original](../en/ch14-testing-and-benchmarking-patterns.md)

# 14. 测试与基准模式 🟢

> **你将学到：**
> - 单元测试、集成测试和文档测试。
> - 使用 `proptest` 进行基于属性的测试。
> - 使用 `criterion` 进行基准测试。
> - 使用 trait 进行 Mock 的策略。

## 三级测试体系

1. **单元测试 (Unit Tests)**：代码位于同一文件中，使用 `#[cfg(test)]`。
2. **集成测试 (Integration Tests)**：位于 `tests/` 目录中，仅测试公共 API。
3. **文档测试 (Doc Tests)**：位于三斜杠文档注释（`///`）中，不仅是示例，也会被 `cargo test` 编译并运行。

```rust
/// 加法运算。
/// ```
/// assert_eq!(my_crate::add(2, 2), 4);
/// ```
pub fn add(a: i32, b: i32) -> i32 { a + b }
```

---

## 基于属性的测试 (proptest)

与其提供特定的测试值，不如测试那些对任何输入都成立的 **属性（Properties）**。

```rust
proptest! {
    #[test]
    fn test_reverse_twice_is_identity(v in prop::collection::vec(any::<i32>(), 0..100)) {
        let original = v.clone();
        let reversed = reverse(&reverse(&v));
        assert_eq!(original, reversed);
    }
}
```

---

## 基准测试 (criterion)

使用 `criterion` 进行具有统计学严谨性的性能评估。它会自动生成 HTML 报告并处理热身期（warmup）和冷却期数据。

```rust
fn bench_method(c: &mut Criterion) {
    c.bench_function("my_func", |b| b.iter(|| my_func(black_box(20))));
}
```

---

## 使用 Trait 进行 Mock

避免使用重型的 Mock 框架。使用 trait 来注入依赖，这能使你的代码自然地具备可测试性。

```rust
trait Database {
    fn get_user(&self, id: u32) -> User;
}

struct MyService<D: Database> {
    db: D,
}

#[test]
fn test_service() {
    let mock_db = MockDatabase { ... };
    let service = MyService { db: mock_db };
    // ...
}
```

***
