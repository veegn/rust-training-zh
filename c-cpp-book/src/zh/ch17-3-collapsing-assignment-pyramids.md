[English Original](../en/ch17-3-collapsing-assignment-pyramids.md)

## 利用闭包折叠赋值“金字塔”

> **你将学到：** Rust 基于表达式的语法和闭包如何将 C++ 中深层嵌套的 `if/else` 校验链，扁平化为整洁、线性的代码。

- 在 C++ 中，为了给变量赋值，往往需要编写多块 `if/else` 链，特别是在涉及校验或回退 (fallback) 逻辑时。Rust 基于表达式的语法和闭包可以将这些逻辑折叠为扁平的线性代码。

### 模式 1：利用 `if` 表达式进行元组赋值
```cpp
// C++ —— 通过多块 if/else 链设置三个变量
uint32_t fault_code;
const char* der_marker;
const char* action;
if (is_c44ad) {
    fault_code = 32709; der_marker = "CSI_WARN"; action = "No action";
} else if (error.is_hardware_error()) {
    fault_code = 67956; der_marker = "CSI_ERR"; action = "Replace GPU";
} else {
    fault_code = 32709; der_marker = "CSI_WARN"; action = "No action";
}
```

```rust
// Rust 等效写法：accel_fieldiag.rs
// 通过单一表达式同时为三个变量赋值：
let (fault_code, der_marker, recommended_action) = if is_c44ad {
    (32709u32, "CSI_WARN", "无操作")
} else if error.is_hardware_error() {
    (67956u32, "CSI_ERR", "更换 GPU")
} else {
    (32709u32, "CSI_WARN", "无操作")
---

### 模式 2：用于易错链式调用的 IIFE (立即调用函数表达式)
```cpp
// C++ —— JSON 导航中的“死亡金字塔”
std::string get_part_number(const nlohmann::json& root) {
    if (root.contains("SystemInfo")) {
        auto& sys = root["SystemInfo"];
        if (sys.contains("BaseboardFru")) {
            auto& bb = sys["BaseboardFru"];
            if (bb.contains("ProductPartNumber")) {
                return bb["ProductPartNumber"].get<std::string>();
            }
        }
    }
    return "UNKNOWN";
}
```

```rust
// Rust 等效写法：framework.rs
// 闭包 + ? 运算符将金字塔结构折叠为线性代码：
let part_number = (|| -> Option<String> {
    let path = self.args.sysinfo.as_ref()?;
    let content = std::fs::read_to_string(path).ok()?;
    let json: serde_json::Value = serde_json::from_str(&content).ok()?;
    let ppn = json
        .get("SystemInfo")?
        .get("BaseboardFru")?
        .get("ProductPartNumber")?
        .as_str()?;
    Some(ppn.to_string())
})()
.unwrap_or_else(|| "UNKNOWN".to_string());
```
该闭包创建了一个 `Option<String>` 作用域，在此作用域内，`?` 可以在任何步骤提前退出。随后，`.unwrap_or_else()` 在代码最后仅提供一次回退值即可。

---

### 模式 3：用迭代器链式调用 替代 手动循环 + `push_back`
```cpp
// C++ —— 使用中间变量的手动循环
std::vector<std::tuple<std::vector<std::string>, std::string, std::string>> gpu_info;
for (const auto& [key, info] : gpu_pcie_map) {
    std::vector<std::string> bdfs;
    // ... 将 bdf_path 解析为 bdfs
    std::string serial = info.serial_number.value_or("UNKNOWN");
    std::string model = info.model_number.value_or(model_name);
    gpu_info.push_back({bdfs, serial, model});
}
```

```rust
// Rust 等效写法：peripherals.rs
// 单一链式调用：values() -> map -> collect
let gpu_info: Vec<(Vec<String>, String, String, String)> = self
    .gpu_pcie_map
    .values()
    .map(|info| {
        let bdfs: Vec<String> = info.bdf_path
            .split(')')
            .filter(|s| !s.is_empty())
            .map(|s| s.trim_start_matches('(').to_string())
            .collect();
        let serial = info.serial_number.clone()
            .unwrap_or_else(|| "UNKNOWN".to_string());
        let model = info.model_number.clone()
            .unwrap_or_else(|| model_name.to_string());
        let gpu_bdf = format!("{}:{}:{}.{}",
            info.bdf.segment, info.bdf.bus, info.bdf.device, info.bdf.function);
        (bdfs, serial, model, gpu_bdf)
    })
    .collect();
```

---

### 模式 4：`.filter().collect()` 替代 循环 + `if (condition) continue`
```cpp
// C++
std::vector<TestResult*> failures;
for (auto& t : test_results) {
    if (!t.is_pass()) {
        failures.push_back(&t);
    }
}
```

```rust
// Rust —— 摘自 accel_diag/src/healthcheck.rs
pub fn failed_tests(&self) -> Vec<&TestResult> {
    self.test_results.iter().filter(|t| !t.is_pass()).collect()
}
```

---

### 总结：何时使用何种模式

| **C++ 模式** | **Rust 替代方案** | **核心优势** |
|----------------|---------------------|-----------------|
| 多块变量赋值 | `let (a, b) = if ... { } else { };` | 所有变量以原子化方式进行绑定 |
| 嵌套的 `if (contains)` 金字塔 | 带有 `?` 运算符的 IIFE 闭包 | 线性、扁平、提前退出 |
| `for` 循环 + `push_back` | `.iter().map(||).collect()` | 无需中间状态的可变 Vec |
| `for` + `if (cond) continue` | `.iter().filter(||).collect()` | 声明式意图 |
| `for` + `if + break` (查找第一个) | `.iter().find_map(||)` | 单次遍历完成 查找 + 转换 |

---

# 终极练习：诊断事件流水线

🔴 **挑战** —— 该练习综合了枚举 (Enums)、Trait、迭代器、错误处理以及泛型。

你将构建一个简化版的诊断事件处理流水线，这与生产环境 Rust 代码中使用的模式非常相似。

**要求：**
1. 定义一个 `enum Severity { Info, Warning, Critical }`，实现 `Display` Trait；并定义一个 `struct DiagEvent`，其中包含 `source: String`、`severity: Severity`、`message: String` 以及 `fault_code: u32` 字段。
2. 定义一个 `trait EventFilter`，其中包含一个 `fn should_include(&self, event: &DiagEvent) -> bool` 方法。
3. 实现两个过滤器：`SeverityFilter`（仅包含严重程度大于或等于给定值的事件）和 `SourceFilter`（仅包含来自特定源字符串的事件）。
4. 编写一个 `fn process_events(events: &[DiagEvent], filters: &[&dyn EventFilter]) -> Vec<String>` 函数，该函数对所有通过了**全部**过滤器的事件返回格式化的报告行。
5. 编写一个 `fn parse_event(line: &str) -> Result<DiagEvent, String>` 函数，该函数能解析格式如 `"source:severity:fault_code:message"` 的字符串行（对于错误的输入返回 `Err`）。

---

**起始代码：**
```rust
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Severity {
    Info,
    Warning,
    Critical,
}

impl fmt::Display for Severity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

#[derive(Debug, Clone)]
struct DiagEvent {
    source: String,
    severity: Severity,
    message: String,
    fault_code: u32,
}

trait EventFilter {
    fn should_include(&self, event: &DiagEvent) -> bool;
}

struct SeverityFilter {
    min_severity: Severity,
}
// TODO: 为 SeverityFilter 实现 EventFilter

struct SourceFilter {
    source: String,
}
// TODO: 为 SourceFilter 实现 EventFilter

fn process_events(events: &[DiagEvent], filters: &[&dyn EventFilter]) -> Vec<String> {
    // TODO: 过滤通过了全部过滤器的事件，并格式化为：
    // "[SEVERITY] source (FC:fault_code): message"
    todo!()
}

fn parse_event(line: &str) -> Result<DiagEvent, String> {
    // 解析 "source:severity:fault_code:message"
    // 出现非法输入时返回 Err
    todo!()
}
```

---

```rust
fn main() {
    let raw_lines = vec![
        "accel_diag:Critical:67956:检测到 ECC 不可纠正错误",
        "nic_diag:Warning:32709:链路速度下降",
        "accel_diag:Info:10001:自检通过",
        "cpu_diag:Critical:55012:热节流激活",
        "accel_diag:Warning:32710:PCIe 链路宽度降低",
    ];

    // 解析所有行，收集成功的记录并报告解析错误
    let events: Vec<DiagEvent> = raw_lines.iter()
        .filter_map(|line| match parse_event(line) {
            Ok(e) => Some(e),
            Err(e) => { eprintln!("解析错误: {e}"); None }
        })
        .collect();

    // 应用过滤器：仅提取来自 accel_diag 且严重程度为 Warning 或更高级别的事件
    let sev_filter = SeverityFilter { min_severity: Severity::Warning };
    let src_filter = SourceFilter { source: "accel_diag".to_string() };
    let filters: Vec<&dyn EventFilter> = vec![&sev_filter, &src_filter];

    let report = process_events(&events, &filters);
    for line in &report {
        println!("{line}");
    }
    println!("--- 匹配到 {} 条事件 ---", report.len());
}
```

---

<details><summary>答案 (点击展开)</summary>

```rust
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Severity {
    Info,
    Warning,
    Critical,
}

impl fmt::Display for Severity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Severity::Info => write!(f, "INFO"),
            Severity::Warning => write!(f, "WARNING"),
            Severity::Critical => write!(f, "CRITICAL"),
        }
    }
}

impl Severity {
    fn from_str(s: &str) -> Result<Self, String> {
        match s {
            "Info" => Ok(Severity::Info),
            "Warning" => Ok(Severity::Warning),
            "Critical" => Ok(Severity::Critical),
            other => Err(format!("未知严重程度: {other}")),
        }
    }
}

#[derive(Debug, Clone)]
struct DiagEvent {
    source: String,
    severity: Severity,
    message: String,
    fault_code: u32,
}

trait EventFilter {
    fn should_include(&self, event: &DiagEvent) -> bool;
}

struct SeverityFilter {
    min_severity: Severity,
}

impl EventFilter for SeverityFilter {
    fn should_include(&self, event: &DiagEvent) -> bool {
        event.severity >= self.min_severity
    }
}

struct SourceFilter {
    source: String,
}

impl EventFilter for SourceFilter {
    fn should_include(&self, event: &DiagEvent) -> bool {
        event.source == self.source
    }
}

fn process_events(events: &[DiagEvent], filters: &[&dyn EventFilter]) -> Vec<String> {
    events.iter()
        .filter(|e| filters.iter().all(|f| f.should_include(e)))
        .map(|e| format!("[{}] {} (FC:{}): {}", e.severity, e.source, e.fault_code, e.message))
        .collect()
}
```
---

```rust
fn parse_event(line: &str) -> Result<DiagEvent, String> {
    let parts: Vec<&str> = line.splitn(4, ':').collect();
    if parts.len() != 4 {
        return Err(format!("应输入由冒号分隔的 4 个字段，实际获取到 {} 个", parts.len()));
    }
    let fault_code = parts[2].parse::<u32>()
        .map_err(|e| format!("非法的故障代码 '{}': {e}", parts[2]))?;
    Ok(DiagEvent {
        source: parts[0].to_string(),
        severity: Severity::from_str(parts[1])?,
        fault_code,
        message: parts[3].to_string(),
    })
}

fn main() {
    let raw_lines = vec![
        "accel_diag:Critical:67956:检测到 ECC 不可纠正错误",
        "nic_diag:Warning:32709:链路速度下降",
        "accel_diag:Info:10001:自检通过",
        "cpu_diag:Critical:55012:热节流激活",
        "accel_diag:Warning:32710:PCIe 链路宽度降低",
    ];

    let events: Vec<DiagEvent> = raw_lines.iter()
        .filter_map(|line| match parse_event(line) {
            Ok(e) => Some(e),
            Err(e) => { eprintln!("解析错误: {e}"); None }
        })
        .collect();

    let sev_filter = SeverityFilter { min_severity: Severity::Warning };
    let src_filter = SourceFilter { source: "accel_diag".to_string() };
    let filters: Vec<&dyn EventFilter> = vec![&sev_filter, &src_filter];

    let report = process_events(&events, &filters);
    for line in &report {
        println!("{line}");
    }
    println!("--- 匹配到 {} 条事件 ---", report.len());
}
// 输出示例：
// [CRITICAL] accel_diag (FC:67956): 检测到 ECC 不可纠正错误
// [WARNING] accel_diag (FC:32710): PCIe 链路宽度降低
// --- 匹配到 2 条事件 ---
```

</details>

---
