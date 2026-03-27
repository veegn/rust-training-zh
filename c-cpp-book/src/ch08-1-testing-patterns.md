## Testing Patterns for C++ Programmers / C++ 程序员的测试模式
 
 > **What you'll learn / 你将学到：** Rust's built-in test framework — `#[test]`, `#[should_panic]`, `Result`-returning tests, builder patterns for test data, trait-based mocking, property testing with `proptest`, snapshot testing with `insta`, and integration test organization. Zero-config testing that replaces Google Test + CMake.
 >
 > Rust 内置的测试框架 —— `#[test]`、`#[should_panic]`、返回 `Result` 的测试、测试数据的构建器模式、基于 trait 的模拟（mocking）、使用 `proptest` 进行属性测试、使用 `insta` 进行快照测试，以及集成测试的组织。这种零配置测试将取代 Google Test + CMake。
 
- C++ testing typically relies on external frameworks (Google Test, Catch2, Boost.Test)
+ C++ 测试通常依赖于外部框架（Google Test、Catch2、Boost.Test）
- with complex build integration. Rust's test framework is **built into the language
+ 并伴随着复杂的构建集成。Rust 的测试框架是**内置于语言和工具链中**的 —— 无需依赖，无需 CMake 集成，也无需配置测试运行器（test runner）。
- and toolchain** — no dependencies, no CMake integration, no test runner configuration.
 
- ### Test attributes beyond `#[test]`
+ ### Test attributes beyond `#[test]` / 除 `#[test]` 之外的测试属性
 
 ```rust
 #[cfg(test)]
 mod tests {
     use super::*;
 
     #[test]
     fn basic_pass() {
         assert_eq!(2 + 2, 4);
     }
 
-    // Expect a panic — equivalent to GTest's EXPECT_DEATH
+    // Expect a panic — equivalent to GTest's EXPECT_DEATH / 预期发生 panic —— 相当于 GTest 的 EXPECT_DEATH
     #[test]
     #[should_panic]
     fn out_of_bounds_panics() {
         let v = vec![1, 2, 3];
-        let _ = v[10]; // Panics — test passes
+        let _ = v[10]; // Panics — test passes / 发生 panic —— 测试通过
     }
 
-    // Expect a panic with a specific message substring
+    // Expect a panic with a specific message substring / 预期带有特定消息子串的 panic
     #[test]
     #[should_panic(expected = "index out of bounds")]
     fn specific_panic_message() {
         let v = vec![1, 2, 3];
         let _ = v[10];
     }
 
-    // Tests that return Result<(), E> — use ? instead of unwrap()
+    // Tests that return Result<(), E> — use ? instead of unwrap() / 返回 Result<(), E> 的测试 —— 使用 ? 代替 unwrap()
     #[test]
     fn test_with_result() -> Result<(), String> {
         let value: u32 = "42".parse().map_err(|e| format!("{e}"))?;
         assert_eq!(value, 42);
         Ok(())
     }
 
-    // Ignore slow tests by default — run with `cargo test -- --ignored`
+    // Ignore slow tests by default — run with `cargo test -- --ignored` / 默认忽略慢速测试 —— 使用 `cargo test -- --ignored` 运行
     #[test]
     #[ignore]
     fn slow_integration_test() {
         std::thread::sleep(std::time::Duration::from_secs(10));
     }
 }
 ```
 
 ```bash
- cargo test                          # Run all non-ignored tests
+ cargo test                          # Run all non-ignored tests / 运行所有未被忽略的测试
- cargo test -- --ignored             # Run only ignored tests
+ cargo test -- --ignored             # Run only ignored tests / 仅运行被忽略的测试
- cargo test -- --include-ignored     # Run ALL tests including ignored
+ cargo test -- --include-ignored     # Run ALL tests including ignored / 运行包括忽略测试在内的所有测试
- cargo test test_name                # Run tests matching a name pattern
+ cargo test test_name                # Run tests matching a name pattern / 运行匹配名称模式的测试
- cargo test -- --nocapture           # Show println! output during tests
+ cargo test -- --nocapture           # Show println! output during tests / 测试期间显示 println! 输出
- cargo test -- --test-threads=1      # Run tests serially (for shared state)
+ cargo test -- --test-threads=1      # Run tests serially (for shared state) / 串行运行测试（用于共享状态）
 ```
 
- ### Test helpers: builder pattern for test data
+ ### Test helpers: builder pattern for test data / 测试辅助工具：测试数据的构建器模式
 
- In C++ you'd use Google Test fixtures (`class MyTest : public ::testing::Test`).
+ 在 C++ 中，你会使用 Google Test 固件（Google Test fixtures，`class MyTest : public ::testing::Test`）。
- In Rust, use builder functions or the `Default` trait:
+ 在 Rust 中，使用构建器函数或 `Default` trait：
 
 ```rust
 #[cfg(test)]
 mod tests {
     use super::*;
 
-    // Builder function — creates test data with sensible defaults
+    // Builder function — creates test data with sensible defaults / 构建器函数 —— 使用合理的默认值创建测试数据
     fn make_gpu_event(severity: Severity, fault_code: u32) -> DiagEvent {
         DiagEvent {
             source: "accel_diag".to_string(),
             severity,
             message: format!("Test event FC:{fault_code}"),
             fault_code,
         }
     }
 
-    // Reusable test fixture — a set of pre-built events
+    // Reusable test fixture — a set of pre-built events / 可重用的测试固件 —— 一组预构建的事件
     fn sample_events() -> Vec<DiagEvent> {
         vec![
             make_gpu_event(Severity::Critical, 67956),
             make_gpu_event(Severity::Warning, 32709),
             make_gpu_event(Severity::Info, 10001),
         ]
     }
 
     #[test]
     fn filter_critical_events() {
         let events = sample_events();
         let critical: Vec<_> = events.iter()
             .filter(|e| e.severity == Severity::Critical)
             .collect();
         assert_eq!(critical.len(), 1);
         assert_eq!(critical[0].fault_code, 67956);
     }
 }
 ```
 
- ### Mocking with traits
+ ### Mocking with traits / 使用 Trait 进行模拟（Mocking）
 
- In C++, mocking requires frameworks like Google Mock or manual virtual overrides.
+ 在 C++ 中，模拟（mocking）需要像 Google Mock 这样的框架或手动进行虚函数覆盖。
- In Rust, define a trait for the dependency and swap implementations in tests:
+ 在 Rust 中，为依赖项定义一个 trait 并在测试中更换实现：
 
 ```rust
- // Production trait
+ // Production trait / 生产环境使用的 trait
 trait SensorReader {
     fn read_temperature(&self, sensor_id: u32) -> Result<f64, String>;
 }
 
- // Production implementation
+ // Production implementation / 生产环境实现
 struct HwSensorReader;
 impl SensorReader for HwSensorReader {
     fn read_temperature(&self, sensor_id: u32) -> Result<f64, String> {
-        // Real hardware call...
+        // Real hardware call... / 真实的硬件调用...
         Ok(72.5)
     }
 }
 
- // Test mock — returns predictable values
+ // Test mock — returns predictable values / 测试模拟 —— 返回可预测的值
 #[cfg(test)]
 struct MockSensorReader {
     temperatures: std::collections::HashMap<u32, f64>,
 }
 
 #[cfg(test)]
 impl SensorReader for MockSensorReader {
     fn read_temperature(&self, sensor_id: u32) -> Result<f64, String> {
         self.temperatures.get(&sensor_id)
             .copied()
             .ok_or_else(|| format!("Unknown sensor {sensor_id}"))
     }
 }
 
- // Function under test — generic over the reader
+ // Function under test — generic over the reader / 待测函数 —— 对 reader 进行泛型处理
 fn check_overtemp(reader: &impl SensorReader, ids: &[u32], threshold: f64) -> Vec<u32> {
     ids.iter()
         .filter(|&&id| reader.read_temperature(id).unwrap_or(0.0) > threshold)
         .copied()
         .collect()
 }
 
 #[cfg(test)]
 mod tests {
     use super::*;
 
     #[test]
     fn detect_overtemp_sensors() {
         let mut mock = MockSensorReader { temperatures: Default::default() };
-        mock.temperatures.insert(0, 72.5);
+        mock.temperatures.insert(0, 72.5); // 存入模拟值
-        mock.temperatures.insert(1, 91.0);  // Over threshold
+        mock.temperatures.insert(1, 91.0);  // Over threshold / 超过阈值
         mock.temperatures.insert(2, 65.0);
 
         let hot = check_overtemp(&mock, &[0, 1, 2], 80.0);
         assert_eq!(hot, vec![1]);
     }
 }
 ```
 
- ### Temporary files and directories in tests
+ ### Temporary files and directories in tests / 测试中的临时文件与目录
 
- C++ tests often use platform-specific temp directories. Rust has `tempfile`:
+ C++ 测试通常使用特定于平台的临时目录。Rust 有 `tempfile`：
 
 ```rust
 // Cargo.toml: [dev-dependencies]
 // tempfile = "3"
 
 #[cfg(test)]
 mod tests {
     use super::*;
     use tempfile::NamedTempFile;
     use std::io::Write;
 
     #[test]
     fn parse_config_from_file() -> Result<(), Box<dyn std::error::Error>> {
-        // Create a temp file that's auto-deleted when dropped
+        // Create a temp file that's auto-deleted when dropped / 创建一个在释放（dropped）时自动删除的临时文件
         let mut file = NamedTempFile::new()?;
         writeln!(file, r#"{{"sku": "ServerNode", "level": "Quick"}}"#)?;
 
         let config = load_config(file.path().to_str().unwrap())?;
         assert_eq!(config.sku, "ServerNode");
         Ok(())
-        // file is deleted here — no cleanup code needed
+        // file is deleted here — no cleanup code needed / 文件在这里被删除 —— 无需清理代码
     }
 }
 ```
 
- ### Property-based testing with `proptest`
+ ### Property-based testing with `proptest` / 使用 `proptest` 进行基于属性的测试
 
- Instead of writing specific test cases, describe **properties** that should hold
+ 与其编写特定的测试用例，不如描述对于所有输入都应成立的**属性（properties）**。
- for all inputs. `proptest` generates random inputs and finds minimal failing cases:
+ `proptest` 会生成随机输入并找到最小的失败用例：
 
 ```rust
 // Cargo.toml: [dev-dependencies]
 // proptest = "1"
 
 #[cfg(test)]
 mod tests {
     use proptest::prelude::*;
 
     fn parse_and_format(n: u32) -> String {
         format!("{n}")
     }
 
     proptest! {
         #[test]
-        fn roundtrip_u32(n: u32) {
+        fn roundtrip_u32(n: u32) { // 测试往返转换
             let formatted = parse_and_format(n);
             let parsed: u32 = formatted.parse().unwrap();
             prop_assert_eq!(n, parsed);
         }
 
         #[test]
-        fn string_contains_no_null(s in "[a-zA-Z0-9 ]{0,100}") {
+        fn string_contains_no_null(s in "[a-zA-Z0-9 ]{0,100}") { // 字符串不包含 null
             prop_assert!(!s.contains('\0'));
         }
     }
 }
 ```
 
- ### Snapshot testing with `insta`
+ ### Snapshot testing with `insta` / 使用 `insta` 进行快照测试
 
- For tests that produce complex output (JSON, formatted strings), `insta` auto-generates
+ 对于产生复杂输出（JSON、格式化字符串）的测试，`insta` 会自动生成并管理参考快照：
- and manages reference snapshots:
 
 ```rust
 // Cargo.toml: [dev-dependencies]
 // insta = { version = "1", features = ["json"] }
 
 #[cfg(test)]
 mod tests {
     use insta::assert_json_snapshot;
 
     #[test]
     fn der_entry_format() {
         let entry = DerEntry {
             fault_code: 67956,
             component: "GPU".to_string(),
             message: "ECC error detected".to_string(),
         };
-        // First run: creates a snapshot file in tests/snapshots/
+        // First run: creates a snapshot file in tests/snapshots/ / 第一次运行：在 tests/snapshots/ 中创建一个快照文件
-        // Subsequent runs: compares against the saved snapshot
+        // Subsequent runs: compares against the saved snapshot / 后续运行：与保存的快照进行比较
         assert_json_snapshot!(entry);
     }
 }
 ```
 
 ```bash
- cargo insta test              # Run tests and review new/changed snapshots
+ cargo insta test              # Run tests and review snapshots / 运行测试并查看新快照或变更的快照
- cargo insta review            # Interactive review of snapshot changes
+ cargo insta review            # Interactive review of snapshot changes / 以交互方式查看快照变更
 ```
 
- ### C++ vs Rust testing comparison
+ ### C++ vs Rust testing comparison / C++ vs Rust 测试对比
 
-| **C++ (Google Test)** | **Rust** | **Notes** |
+| **C++ (Google Test)** | **Rust** | **Notes / 说明** |
 |----------------------|---------|----------|
-| `TEST(Suite, Name) { }` | `#[test] fn name() { }` | No suite/class hierarchy needed |
+| `TEST(Suite, Name) { }` | `#[test] fn name() { }` | No suite/class hierarchy needed / 无需测试套件或类层次结构 |
-| `ASSERT_EQ(a, b)` | `assert_eq!(a, b)` | Built-in macro, no framework needed |
+| `ASSERT_EQ(a, b)` | `assert_eq!(a, b)` | Built-in macro, no framework needed / 内置宏，无需框架 |
-| `ASSERT_NEAR(a, b, eps)` | `assert!((a - b).abs() < eps)` | Or use `approx` crate |
+| `ASSERT_NEAR(a, b, eps)` | `assert!((a - b).abs() < eps)` | Or use `approx` crate / 或者使用 `approx` crate |
-| `EXPECT_THROW(expr, type)` | `#[should_panic(expected = "...")]` | Or `catch_unwind` for fine control |
+| `EXPECT_THROW(expr, type)` | `#[should_panic(expected = "...")]` | Or `catch_unwind` / 或使用 `catch_unwind` 进行精细控制 |
-| `EXPECT_DEATH(expr, "msg")` | `#[should_panic(expected = "msg")]` | |
-| `EXPECT_DEATH(expr, "msg")` | `#[should_panic(expected = "msg")]` | |
-| `class Fixture : public ::testing::Test` | Builder functions + `Default` | No inheritance needed |
+| `class Fixture : public ::testing::Test` | Builder functions + `Default` | No inheritance needed / 无需继承 |
-| Google Mock `MOCK_METHOD` | Trait + test impl | More explicit, no macro magic |
+| Google Mock `MOCK_METHOD` | Trait + test impl | More explicit, no macro magic / 更显式，无宏魔法 |
-| `INSTANTIATE_TEST_SUITE_P` (parameterized) | `proptest!` or macro-generated tests | |
-| `INSTANTIATE_TEST_SUITE_P` (parameterized) | `proptest!` or macro-generated tests | |
-| `SetUp()` / `TearDown()` | RAII via `Drop` — cleanup is automatic | Variables dropped at end of test |
+| `SetUp()` / `TearDown()` | RAII via `Drop` — cleanup is automatic | Variables dropped at end of test / 变量在测试结束时被释放 |
-| Separate test binary + CMake | `cargo test` — zero config | |
-| `Separate test binary + CMake` | `cargo test` — zero config / 零配置 | |
-| `ctest --output-on-failure` | `cargo test -- --nocapture` | |
-| `ctest --output-on-failure` | `cargo test -- --nocapture` | |
 
 ----
 
- ### Integration tests: the `tests/` directory
+ ### Integration tests: the `tests/` directory / 集成测试：`tests/` 目录
 
- Unit tests live inside `#[cfg(test)]` modules alongside your code. **Integration tests** live in a separate `tests/` directory at the crate root and test your library's public API as an external consumer would:
+ 单元测试位于代码旁边的 `#[cfg(test)]` 模块中。**集成测试**位于 crate 根目录下的独立 `tests/` 目录中，并像外部消费者一样测试你的库的公有 API：
 
 ```
- my_crate/
+ my_crate/ # 你的 crate
 ├── src/
- │   └── lib.rs          # Your library code
+ │   └── lib.rs          # Your library code / 你的库代码
 ├── tests/
- │   ├── smoke.rs        # Each .rs file is a separate test binary
+ │   ├── smoke.rs        # Each .rs file is a separate test binary / 每个 .rs 文件都是一个独立的测试二进制文件
- │   ├── regression.rs
- │   └── common/
- │       └── mod.rs      # Shared test helpers (NOT a test itself)
+ │   ├── regression.rs   # 回归测试
+ │   └── common/ # 测试辅助工具
+ │       └── mod.rs      # Shared test helpers (NOT a test itself) / 共享测试辅助工具（其本身不是测试）
 └── Cargo.toml
 ```
 
 ```rust
- // tests/smoke.rs — tests your crate as an external user would
+ // tests/smoke.rs — tests your crate as an external user would / 像外部用户一样测试你的 crate
- use my_crate::DiagEngine;  // Only public API is accessible
+ use my_crate::DiagEngine;  // Only public API is accessible / 仅可访问公有 API
 
 #[test]
 fn engine_starts_successfully() {
     let engine = DiagEngine::new("test_config.json");
     assert!(engine.is_ok());
 }
 
 #[test]
 fn engine_rejects_invalid_config() {
     let engine = DiagEngine::new("nonexistent.json");
     assert!(engine.is_err());
 }
 ```
 
 ```rust
- // tests/common/mod.rs — shared helpers, NOT compiled as a test binary
+ // tests/common/mod.rs — shared helpers / 共享辅助工具，不会被编译为测试二进制文件
 pub fn setup_test_environment() -> tempfile::TempDir {
     let dir = tempfile::tempdir().unwrap();
     std::fs::write(dir.path().join("config.json"), r#"{"log_level": "debug"}"#).unwrap();
     dir
 }
 ```
 
 ```rust
- // tests/regression.rs — can use shared helpers
+ // tests/regression.rs — can use shared helpers / 可以使用共享辅助工具
 mod common;
 
 #[test]
 fn regression_issue_42() {
     let env = common::setup_test_environment();
     let engine = my_crate::DiagEngine::new(
         env.path().join("config.json").to_str().unwrap()
     );
     assert!(engine.is_ok());
 }
 ```
 
- **Running integration tests:**
+ **Running integration tests / 运行集成测试：**
 ```bash
- cargo test                          # Runs unit AND integration tests
+ cargo test                          # Runs unit AND integration tests / 运行单元测试和集成测试
- cargo test --test smoke             # Run only tests/smoke.rs
+ cargo test --test smoke             # Run only tests/smoke.rs / 仅运行 tests/smoke.rs
- cargo test --test regression        # Run only tests/regression.rs
+ cargo test --test regression        # Run only tests/regression.rs / 仅运行 tests/regression.rs
- cargo test --lib                    # Run ONLY unit tests (skip integration)
+ cargo test --lib                    # Run ONLY unit tests (skip integration) / 仅运行单元测试（跳过集成测试）
 ```
 
- > **Key difference from unit tests**: Integration tests cannot access private functions or `pub(crate)` items. This forces you to verify that your public API is sufficient — a valuable design signal. In C++ terms, it's like testing against only the public header with no `friend` access.
+ > **与单元测试的关键区别**：集成测试无法访问私有函数或 `pub(crate)` 项。这会强制你验证公有 API 是否足够 —— 这是一个宝贵的辅助设计的信号。用 C++ 的话来说，这就像是在没有任何 `friend` 访问权限的情况下，仅针对公有头文件进行测试。
