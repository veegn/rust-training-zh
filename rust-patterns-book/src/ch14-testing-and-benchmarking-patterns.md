# 14. Testing and Benchmarking Patterns / 14. 测试与基准模式 🟢

> **What you'll learn / 你将学到：**
> - Rust's three test tiers: unit, integration, and doc tests / Rust 的三级测试体系：单元测试、集成测试和文档测试
> - Property-based testing with proptest for discovering edge cases / 使用 proptest 进行基于属性的测试以发现边界情况
> - Benchmarking with criterion for reliable performance measurement / 使用 criterion 进行基准测试以实现可靠的性能衡量
> - Mocking strategies without heavyweight frameworks / 不依赖重型框架的 Mock 策略

## Unit Tests, Integration Tests, Doc Tests / 单元测试、集成测试、文档测试

Rust has three testing tiers built into the language:

Rust 语言内置了三个层级的测试体系：

```rust
// --- Unit tests: in the same file as the code ---
// --- 单元测试：与代码位于同一文件中 ---
pub fn factorial(n: u64) -> u64 {
    (1..=n).product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_factorial_zero() {
        // (1..=0).product() returns 1 — the multiplication identity for empty ranges
        // (1..=0).product() 返回 1 —— 这是空范围的乘法单位元
        assert_eq!(factorial(0), 1);
    }

    #[test]
    fn test_factorial_five() {
        assert_eq!(factorial(5), 120);
    }

    #[test]
    #[cfg(debug_assertions)] // overflow checks are only enabled in debug mode
                             // 溢出检查仅在调试模式下启用
    #[should_panic(expected = "overflow")]
    fn test_factorial_overflow() {
        // ⚠️ This test only passes in debug mode (overflow checks enabled).
        // In release mode (`cargo test --release`), u64 arithmetic wraps
        // silently and no panic occurs. Use `checked_mul` or the
        // `overflow-checks = true` profile setting for release-mode safety.
        // ⚠️ 此测试仅在调试模式下通过（启用了溢出检查）。
        // 在发布模式下（`cargo test --release`），u64 算术会静默回绕而不发生 panic。
        // 为了发布模式下的安全性，请使用 `checked_mul` 或 `overflow-checks = true` 配置。
        factorial(100); // Should panic on overflow
                        // 溢出时应当 panic
    }

    #[test]
    fn test_with_result() -> Result<(), Box<dyn std::error::Error>> {
        // Tests can return Result — ? works inside!
        // 测试可以返回 Result —— 内部可以使用 ? 操作符！
        let value: u64 = "42".parse()?;
        assert_eq!(value, 42);
        Ok(())
    }
}
```

```rust
// --- Integration tests: in tests/ directory ---
// --- 集成测试：位于 tests/ 目录中 ---
// tests/integration_test.rs
// These test your crate's PUBLIC API only
// 这些测试仅针对你的 crate 的 公共 API

use my_crate::factorial;

#[test]
fn test_factorial_from_outside() {
    assert_eq!(factorial(10), 3_628_800);
}
```

```rust
// --- Doc tests: in documentation comments ---
// --- 文档测试：位于文档注释中 ---
/// Computes the factorial of `n`.
/// 计算 `n` 的阶乘。
///
/// # Examples
///
/// ```
/// use my_crate::factorial;
/// assert_eq!(factorial(5), 120);
/// ```
///
/// # Panics
///
/// Panics if the result overflows `u64`.
///
/// ```should_panic
/// my_crate::factorial(100);
/// ```
pub fn factorial(n: u64) -> u64 {
    (1..=n).product()
}
// Doc tests are compiled and run by `cargo test` — they keep examples honest.
// 文档测试由 `cargo test` 编译并运行 —— 它们确保示例代码的准确性。
```
```

### Test Fixtures and Setup / 测试固件与初始化

```rust
#[cfg(test)]
mod tests {
    use super::*;

    // Shared setup — create a helper function
    // 共享初始化 —— 创建一个辅助函数
    fn setup_database() -> TestDb {
        let db = TestDb::new_in_memory();
        db.run_migrations();
        db.seed_test_data();
        db
    }

    #[test]
    fn test_user_creation() {
        let db = setup_database();
        let user = db.create_user("Alice", "alice@test.com").unwrap();
        assert_eq!(user.name, "Alice");
    }

    #[test]
    fn test_user_deletion() {
        let db = setup_database();
        db.create_user("Bob", "bob@test.com").unwrap();
        assert!(db.delete_user("Bob").is_ok());
        assert!(db.get_user("Bob").is_none());
    }

    // Cleanup with Drop (RAII):
    // 使用 Drop 进行清理 (RAII):
    struct TempDir {
        path: std::path::PathBuf,
    }

    impl TempDir {
        fn new() -> Self {
            // Cargo.toml: rand = "0.8"
            let path = std::env::temp_dir().join(format!("test_{}", rand::random::<u32>()));
            std::fs::create_dir_all(&path).unwrap();
            TempDir { path }
        }
    }

    impl Drop for TempDir {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.path);
        }
    }

    #[test]
    fn test_file_operations() {
        let dir = TempDir::new(); // Created / 已创建
        std::fs::write(dir.path.join("test.txt"), "hello").unwrap();
        assert!(dir.path.join("test.txt").exists());
    } // dir dropped here → temp directory cleaned up
      // dir 在此处被释放 → 临时目录被清理
}
```

### Property-Based Testing (proptest) / 基于属性的测试 (proptest)

Instead of testing specific values, test *properties* that should always hold:

不要只测试特定值，而应测试那些始终应当成立的 *属性（properties）*：

```rust
// Cargo.toml: proptest = "1"
use proptest::prelude::*;

fn reverse(v: &[i32]) -> Vec<i32> {
    v.iter().rev().cloned().collect()
}

proptest! {
    #[test]
    fn test_reverse_twice_is_identity(v in prop::collection::vec(any::<i32>(), 0..100)) {
        // Property: reversing twice gives back the original
        // 属性：反转两次会得到原始输入
        assert_eq!(reverse(&reverse(&v)), v);
    }

    #[test]
    fn test_reverse_preserves_length(v in prop::collection::vec(any::<i32>(), 0..100)) {
        assert_eq!(reverse(&v).len(), v.len());
    }

    #[test]
    fn test_sort_is_idempotent(mut v in prop::collection::vec(any::<i32>(), 0..100)) {
        v.sort();
        let sorted_once = v.clone();
        v.sort();
        assert_eq!(v, sorted_once); // Sorting twice = sorting once
                                    // 排序两次 = 排序一次
    }

    #[test]
    fn test_parse_roundtrip(x in any::<f64>().prop_filter("finite", |x| x.is_finite())) {
        // Property: formatting then parsing gives back the same value
        // 属性：进行格式化后再解析会得到相同的值
        let s = format!("{x}");
        let parsed: f64 = s.parse().unwrap();
        prop_assert!((x - parsed).abs() < f64::EPSILON);
    }
}
```

> **When to use proptest / 何时使用 proptest**：When you're testing a function with a large input space and want confidence it works for edge cases you didn't think of. proptest generates hundreds of random inputs and shrinks failures to the minimal reproducing case.
>
> 当你在测试一个输入空间巨大的函数，并希望确保它在你未曾想到的边界情况下也能正常工作时。proptest 会生成数百个随机输入，并会将失败案例缩减（shrink）为最小的可复现案例。

### Benchmarking with criterion / 使用 criterion 进行基准测试

```rust
// Cargo.toml:
// [dev-dependencies]
// criterion = { version = "0.5", features = ["html_reports"] }
//
// [[bench]]
// name = "my_benchmarks"
// harness = false

// benches/my_benchmarks.rs
use criterion::{criterion_group, criterion_main, Criterion, black_box};

fn fibonacci(n: u64) -> u64 {
    match n {
        0 | 1 => n,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

fn bench_fibonacci(c: &mut Criterion) {
    c.bench_function("fibonacci 20", |b| {
        // Use black_box to prevent the compiler from optimizing away the call
        // 使用 black_box 防止编译器将调用优化掉
        b.iter(|| fibonacci(black_box(20)))
    });

    // Compare different implementations:
    // 比较不同实现：
    let mut group = c.benchmark_group("fibonacci_compare");
    for size in [10, 15, 20, 25] {
        group.bench_with_input(
            criterion::BenchmarkId::from_parameter(size),
            &size,
            |b, &size| b.iter(|| fibonacci(black_box(size))),
        );
    }
    group.finish();
}

criterion_group!(benches, bench_fibonacci);
criterion_main!(benches);

// Run / 运行: cargo bench
// Produces HTML reports in target/criterion/
// 在 target/criterion/ 中生成 HTML 报告
```

### Mocking Strategies without Frameworks / 不依赖框架的 Mock 策略

Rust's trait system provides natural dependency injection — no mocking framework required:

Rust 的 trait 系统提供了天然的依赖注入机制 —— 无需任何 Mock 框架：

```rust
// Define behavior as a trait
// 将行为定义为 trait
trait Clock {
    fn now(&self) -> std::time::Instant;
}

trait HttpClient {
    fn get(&self, url: &str) -> Result<String, String>;
}

// Production implementations
// 生产环境下的实现
struct RealClock;
impl Clock for RealClock {
    fn now(&self) -> std::time::Instant { std::time::Instant::now() }
}

// Service depends on abstractions
// 服务依赖于抽象
struct CacheService<C: Clock, H: HttpClient> {
    clock: C,
    client: H,
    ttl: std::time::Duration,
}

impl<C: Clock, H: HttpClient> CacheService<C, H> {
    fn fetch(&self, url: &str) -> Result<String, String> {
        // Uses self.clock and self.client — injectable
        // 使用 self.clock 和 self.client —— 可被注入
        self.client.get(url)
    }
}

// Test with mock implementations — no framework needed!
// 使用 Mock 实现进行测试 —— 无需框架！
#[cfg(test)]
mod tests {
    use super::*;

    struct MockClock {
        fixed_time: std::time::Instant,
    }
    impl Clock for MockClock {
        fn now(&self) -> std::time::Instant { self.fixed_time }
    }

    struct MockHttpClient {
        response: String,
    }
    impl HttpClient for MockHttpClient {
        fn get(&self, _url: &str) -> Result<String, String> {
            Ok(self.response.clone())
        }
    }

    #[test]
    fn test_cache_service() {
        let service = CacheService {
            clock: MockClock { fixed_time: std::time::Instant::now() },
            client: MockHttpClient { response: "cached data".into() },
            ttl: std::time::Duration::from_secs(300),
        };

        assert_eq!(service.fetch("http://example.com").unwrap(), "cached data");
    }
}
```

> **Test philosophy / 测试理念**：Prefer real dependencies in integration tests, trait-based mocks in unit tests. Avoid mocking frameworks unless your dependency graph is complex — Rust's trait generics handle most cases naturally.
>
> 在集成测试中优先使用真实的依赖，在单元测试中优先使用基于 trait 的 Mock。除非你的依赖图非常复杂，否则请尽量避免使用 Mock 框架 —— Rust 的 trait 泛型能够自然地处理大多数情况。

> **Key Takeaways — Testing / 关键要点：测试**
> - Doc tests (`///`) double as documentation and regression tests — they're compiled and run / 文档测试（`///`）既是文档也是回归测试 —— 它们会被编译并运行
> - `proptest` generates random inputs to find edge cases you'd never write manually / `proptest` 会生成随机输入以发现你永远不会手动编写的边界情况
> - `criterion` provides statistically rigorous benchmarks with HTML reports / `criterion` 提供了具有统计学严谨性的基准测试及 HTML 报告
> - Mock via trait generics + test doubles, not mock frameworks / 通过 trait 泛型 + 测试桩（test doubles）进行 Mock，而非通过 Mock 框架

> **See also / 延伸阅读**：[Ch 13 — Macros](ch13-macros-code-that-writes-code.md) 了解如何测试由宏生成的代码。[Ch 15 — API Design](ch15-crate-architecture-and-api-design.md) 了解模块布局如何影响测试的组织形式。

---

### Exercise: Property-Based Testing with proptest ★★ (~25 min) / 练习：使用 proptest 进行基于属性的测试

Write a `SortedVec<T: Ord>` wrapper that maintains a sorted invariant. Use `proptest` to verify that:
1. After any sequence of insertions, the internal vec is always sorted
2. `contains()` agrees with the stdlib `Vec::contains()`
3. The length equals the number of insertions

编写一个 `SortedVec<T: Ord>` 封装，以维持排序不变量。使用 `proptest` 来验证以下内容：
1. 在任何插入序列之后，内部的 vector 始终是已排序的。
2. `contains()` 与标准库的 `Vec::contains()` 结果一致。
3. 长度等于插入的次数。

<details>
<summary>🔑 Solution / 参考答案</summary>

```rust,ignore
#[derive(Debug)]
struct SortedVec<T: Ord> {
    inner: Vec<T>,
}

impl<T: Ord> SortedVec<T> {
    fn new() -> Self { SortedVec { inner: Vec::new() } }

    fn insert(&mut self, value: T) {
        let pos = self.inner.binary_search(&value).unwrap_or_else(|p| p);
        self.inner.insert(pos, value);
    }

    fn contains(&self, value: &T) -> bool {
        self.inner.binary_search(value).is_ok()
    }

    fn len(&self) -> usize { self.inner.len() }
    fn as_slice(&self) -> &[T] { &self.inner }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn always_sorted(values in proptest::collection::vec(-1000i32..1000, 0..100)) {
            let mut sv = SortedVec::new();
            for v in &values {
                sv.insert(*v);
            }
            for w in sv.as_slice().windows(2) {
                prop_assert!(w[0] <= w[1]);
            }
            prop_assert_eq!(sv.len(), values.len());
        }

        #[test]
        fn contains_matches_stdlib(values in proptest::collection::vec(0i32..50, 1..30)) {
            let mut sv = SortedVec::new();
            for v in &values {
                sv.insert(*v);
            }
            for v in &values {
                prop_assert!(sv.contains(v));
            }
            prop_assert!(!sv.contains(&9999));
        }
    }
}
```

</details>

***

