[English Original](../en/ch08-1-testing-patterns.md)

# 针对 C++ 程序员的测试模式

> **你将学到：** Rust 内置的测试框架 —— `#[test]`、`#[should_panic]`、返回 `Result` 的测试、测试数据的 Builder 模式、基于 Trait 的 Mock、利用 `proptest` 进行属性测试、利用 `insta` 进行快照测试以及集成测试的组织。这些“零配置”测试将取代 Google Test 和 CMake。

C++ 测试通常依赖于外部框架（如 Google Test、Catch2、Boost.Test），并涉及复杂的构建集成。而 Rust 的测试框架是**内置于语言和工具链中**的 —— 无需额外依赖，无需配置 CMake，也无需单独配置测试运行器 (Test runner)。

### 除了 `#[test]` 之外的测试属性

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_pass() {
        assert_eq!(2 + 2, 4);
    }

    // 预期触发 Panic —— 等同于 GTest 中的 EXPECT_DEATH
    #[test]
    #[should_panic]
    fn out_of_bounds_panics() {
        let v = vec![1, 2, 3];
        let _ = v[10]; // 触发 Panic ——> 测试通过
    }

    // 预期触发包含特定信息的 Panic
    #[test]
    #[should_panic(expected = "index out of bounds")]
    fn specific_panic_message() {
        let v = vec![1, 2, 3];
        let _ = v[10];
    }

    // 返回 Result<(), E> 的测试 —— 使用 ? 而不是 unwrap()
    #[test]
    fn test_with_result() -> Result<(), String> {
        let value: u32 = "42".parse().map_err(|e| format!("{e}"))?;
        assert_eq!(value, 42);
        Ok(())
    }

    // 默认忽略耗时较长的测试 —— 使用 `cargo test -- --ignored` 来运行
    #[test]
    #[ignore]
    fn slow_integration_test() {
        std::thread::sleep(std::time::Duration::from_secs(10));
    }
}
```

```bash
cargo test                          # 运行所有非忽略的测试
cargo test -- --ignored             # 仅运行被忽略的测试
cargo test -- --include-ignored     # 运行所有测试（包括被忽略的）
cargo test test_name                # 运行名称匹配特定模式的测试
cargo test -- --nocapture           # 在测试期间打印 println! 的输出
cargo test -- --test-threads=1      # 串行运行测试（适用于共享状态的情况）
```

---

### 测试辅助函数：测试数据的 Builder 模式

在 C++ 中，你会使用 Google Test 的 Fixture (`class MyTest : public ::testing::Test`)。而在 Rust 中，使用 Builder 函数或 `Default` Trait 即可。

```rust
#[cfg(test)]
mod tests {
    use super::*;

    // Builder 函数 —— 创建具有合理默认值的测试数据
    fn make_gpu_event(severity: Severity, fault_code: u32) -> DiagEvent {
        DiagEvent {
            source: "accel_diag".to_string(),
            severity,
            message: format!("测试事件 FC:{fault_code}"),
            fault_code,
        }
    }

    // 可复用的测试脚手架 —— 一组预构建的事件
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

---

### 利用 Trait 进行 Mock

在 C++ 中，模拟 (Mocking) 需要 Google Mock 之类的框架或者手动重载虚函数。而在 Rust 中，只需为依赖项定义 Trait，然后在测试中替换对应的实现即可：

```rust
// 生产环境 Trait
trait SensorReader {
    fn read_temperature(&self, sensor_id: u32) -> Result<f64, String>;
}

// 生产实现
struct HwSensorReader;
impl SensorReader for HwSensorReader {
    fn read_temperature(&self, sensor_id: u32) -> Result<f64, String> {
        // 真实的硬件调用代码...
        Ok(72.5)
    }
}

// 测试环境 Mock —— 返回可预测的值
#[cfg(test)]
struct MockSensorReader {
    temperatures: std::collections::HashMap<u32, f64>,
}

#[cfg(test)]
impl SensorReader for MockSensorReader {
    fn read_temperature(&self, sensor_id: u32) -> Result<f64, String> {
        self.temperatures.get(&sensor_id)
            .copied()
            .ok_or_else(|| format!("未知传感器 ID {sensor_id}"))
    }
}

// 待测试函数 —— 对读取器应用泛型
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
        mock.temperatures.insert(0, 72.5);
        mock.temperatures.insert(1, 91.0);  // 超过阈值
        mock.temperatures.insert(2, 65.0);

        let hot = check_overtemp(&mock, &[0, 1, 2], 80.0);
        assert_eq!(hot, vec![1]);
    }
}
```

---

### 测试中的临时文件与目录

C++ 测试通常使用平台特定的临时目录。而 Rust 拥有 `tempfile` 库：

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
        // 创建一个在释放 (Drop) 时会自动删除的临时文件
        let mut file = NamedTempFile::new()?;
        writeln!(file, r#"{{"sku": "ServerNode", "level": "Quick"}}"#)?;

        let config = load_config(file.path().to_str().unwrap())?;
        assert_eq!(config.sku, "ServerNode");
        Ok(())
        // 文件在此处被删除 —— 无需额外的清理代码
    }
}
```

---

### 利用 `proptest` 进行基于属性的测试

无需编写特定的测试用例，只需描述对所有输入都成立的**属性 (Properties)**。`proptest` 将生成随机输入并自动探测能使程序出错的最小失败用例：

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
        fn roundtrip_u32(n: u32) {
            let formatted = parse_and_format(n);
            let parsed: u32 = formatted.parse().unwrap();
            prop_assert_eq!(n, parsed);
        }

        #[test]
        fn string_contains_no_null(s in "[a-zA-Z0-9 ]{0,100}") {
            prop_assert!(!s.contains('\0'));
        }
    }
}
```

---

### 利用 `insta` 进行快照测试 (Snapshot Testing)

对于产生复杂输出（如 JSON、格式化字符串）的测试，`insta` 可以自动生成并管理参考快照 (Reference snapshots)：

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
            message: "检测到 ECC 错误".to_string(),
        };
        // 首次运行：在 tests/snapshots/ 目录下创建一个快照文件
        // 后续运行：与已保存的快照进行比对
        assert_json_snapshot!(entry);
    }
}
```

```bash
cargo insta test              # 运行测试并检查新增或变化的快照
cargo insta review            # 交互式检查快照的变化情况
```

---

### C++ vs Rust 测试对比

| **C++ (Google Test)** | **Rust** | **备注** |
|----------------------|---------|----------|
| `TEST(Suite, Name) { }` | `#[test] fn name() { }` | 无需套件 (Suite)/类层次结构 |
| `ASSERT_EQ(a, b)` | `assert_eq!(a, b)` | 内置宏，无需框架 |
| `ASSERT_NEAR(a, b, eps)` | `assert!((a - b).abs() < eps)` | 或者使用 `approx` 库 |
| `EXPECT_THROW(expr, type)` | `#[should_panic(expected = "...")]` | 或者使用 `catch_unwind` 进行精细控制 |
| `EXPECT_DEATH(expr, "msg")` | `#[should_panic(expected = "msg")]` | |
| `class Fixture : public ::testing::Test` | Builder 函数 + `Default` | 无需继承 |
| Google Mock `MOCK_METHOD` | Trait + 测试环境实现 | 更显式，无宏魔法 |
| `INSTANTIATE_TEST_SUITE_P` (参数化测试) | `proptest!` 或由宏生成的测试 | |
| `SetUp()` / `TearDown()` | 通过 `Drop` 实现 RAII —— 清理是自动的 | 变量在测试结束时自动释放 |
| 独立的测试二进制 + CMake | `cargo test` —— 零配置 | |
| `ctest --output-on-failure` | `cargo test -- --nocapture` | |

---

### 集成测试：`tests/` 目录

单元测试与你的代码并排处于 `#[cfg(test)]` 模块中。而**集成测试 (Integration tests)** 则位于单元包根目录下的独立 `tests/` 目录中，它们会像外部消费者使用你的库那样，仅对库的公开 API 进行测试：

```text
my_crate/
├── src/
│   └── lib.rs          # 库代码
├── tests/
│   ├── smoke.rs        # 每个 .rs 文件都是一个独立的测试二进制
│   ├── regression.rs
│   └── common/
│       └── mod.rs      # 共享的测试辅助函数 (其本身并非测试)
└── Cargo.toml
```

```rust
// tests/smoke.rs —— 像外部用户那样测试你的库
use my_crate::DiagEngine;  // 只能访问公开 (pub) API

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
// tests/common/mod.rs —— 共享 Helper 函数，不会被编译为测试二进制
pub fn setup_test_environment() -> tempfile::TempDir {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(dir.path().join("config.json"), r#"{"log_level": "debug"}"#).unwrap();
    dir
}
```

```rust
// tests/regression.rs —— 可以使用共享 Helper 
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

**运行集成测试：**
```bash
cargo test                          # 运行单元测试 AND 集成测试
cargo test --test smoke             # 仅运行 tests/smoke.rs
cargo test --test regression        # 仅运行 tests/regression.rs
cargo test --lib                    # 仅运行单元测试 (跳过集成测试)
```

> **与单元测试的关键区别**：集成测试无法访问私有函数或 `pub(crate)` 项。这会迫使你验证公开 API 设计是否完备 —— 这是一个极其有价值的设计信号。用 C++ 的话来说，这就像只根据公开头文件进行测试，且没有任何 `friend` 访问权限。

---
