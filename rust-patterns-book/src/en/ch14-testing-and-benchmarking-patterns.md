# 14. Testing and Benchmarking Patterns 🟢

> **What you'll learn:**
> - Unit, integration, and doc tests.
> - Property-based testing with `proptest`.
> - Benchmarking with `criterion`.
> - Mocking strategies using traits.

## The Three Tiers of Testing

1. **Unit Tests**: Located in the same file as the code. Use `#[cfg(test)]`.
2. **Integration Tests**: In the `tests/` directory. They only test the public API.
3. **Doc Tests**: Inside triple-slash documentation comments (`///`). They are compiled and run by `cargo test`.

```rust
/// Adds two numbers.
/// ```
/// assert_eq!(my_crate::add(2, 2), 4);
/// ```
pub fn add(a: i32, b: i32) -> i32 { a + b }
```

---

## Property-Based Testing (proptest)

Instead of testing specific values, test **properties** that should always hold true for any input.

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

## Benchmarking (criterion)

Use `criterion` for statistically significant performance measurements. It generates HTML reports and handles warmup/cooldown.

```rust
fn bench_method(c: &mut Criterion) {
    c.bench_function("my_func", |b| b.iter(|| my_func(black_box(20))));
}
```

---

## Mocking with Traits

Avoid heavy mocking frameworks. Use traits to inject dependencies, making your code naturally testable.

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
