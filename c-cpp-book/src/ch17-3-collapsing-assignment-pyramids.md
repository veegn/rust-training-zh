## Collapsing assignment pyramids with closures / 精简层层嵌套的赋值结构

> **What you'll learn / 你将学到：** How Rust's expression-based syntax and closures flatten deeply-nested C++ `if/else` validation chains into clean, linear code.
>
> Rust 基于表达式的语法和闭包如何将 C++ 中深层嵌套的 `if/else` 验证链简化为整洁、线性的代码。

*C++ often requires multi-block `if/else` chains to assign variables, especially when validation or fallback logic is involved. Rust's expression-based syntax and closures collapse these into flat, linear code.*

在 C++ 中，为了给变量赋值（特别是涉及验证或备选逻辑时），通常需要编写多个 `if/else` 代码块。Rust 基于表达式的语法和闭包将这些结构精简为扁平的线性代码。

---

### Pattern 1: Tuple assignment with `if` expression / 模式 1：使用 `if` 表达式进行元组赋值

```cpp
// C++ — three variables set across a multi-block if/else chain
// C++ — 在多个 if/else 块中设置三个变量
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
// Rust equivalent: accel_fieldiag.rs — single expression assigns all three
// 单个表达式同时为三个变量赋值：
let (fault_code, der_marker, recommended_action) = if is_c44ad {
    (32709u32, "CSI_WARN", "No action")
} else if error.is_hardware_error() {
    (67956u32, "CSI_ERR", "Replace GPU")
} else {
    (32709u32, "CSI_WARN", "No action")
};
```

---

### Pattern 2: IIFE for fallible chains / 模式 2：用于易错链式调用的 IIFE（立即调用函数表达式）

```cpp
// C++ — pyramid of doom for JSON navigation
// C++ — JSON 导航中的“末日金字塔”
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
// Rust equivalent: framework.rs — closure + ? operator collapses the pyramid
// 闭包 + ? 运算符将金字塔结构精简为线性代码：
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

---

### Pattern 3: Iterator chain for collection building / 模式 3：用于构建集合的迭代器链

```cpp
// C++ — manual loop with intermediate variables
// C++ — 带有中间变量的手动循环
std::vector<std::tuple<std::vector<std::string>, std::string, std::string>> gpu_info;
for (const auto& [key, info] : gpu_pcie_map) {
    std::vector<std::string> bdfs;
    // ... parse / 将 bdf_path 解析为 bdfs
    std::string serial = info.serial_number.value_or("UNKNOWN");
    std::string model = info.model_number.value_or(model_name);
    gpu_info.push_back({bdfs, serial, model});
}
```

```rust
// Rust equivalent: peripherals.rs — single chain: values() -> map -> collect
// 单个链式调用：values() -> map -> collect
let gpu_info: Vec<(Vec<String>, String, String, String)> = self
    .gpu_pcie_map
    .values()
    .map(|info| {
        let bdfs: Vec<String> = info.bdf_path
            .split(')')
            .filter(|s| !s.is_empty())
            .map(|s| s.trim_start_matches('(').to_string())
            .collect();
        let serial = info.serial_number.as_deref().unwrap_or("UNKNOWN").to_string();
        let model = info.model_number.as_deref().unwrap_or(model_name).to_string();
        let gpu_bdf = format!("{}:{}:{}.{}",
            info.bdf.segment, info.bdf.bus, info.bdf.device, info.bdf.function);
        (bdfs, serial, model, gpu_bdf)
    })
    .collect();
```

---

### Summary: Pattern mapping / 总结：模式映射

| **C++ Pattern / C++ 模式** | **Rust Replacement / Rust 替代方案** | **Key Benefit / 核心优势** |
|----------------|---------------------|-----------------|
| Multi-block assignment / 多块赋值 | `let (a, b) = if ... { } else { };` | Atomic / 变量原子绑定 |
| Nested if pyramid / 嵌套 if 金字塔 | IIFE closure with `?` / IIFE 闭包 + `?` | Flat, early-exit / 扁平、提前退出 |
| `for` + `push_back` | `.iter().map().collect()` | No intermediate / 无需中间的可变 Vec |
| `for` + `if continue` | `.iter().filter().collect()` | Declarative / 声明性意图 |
| `for` + `if + break` (查找首个) | `.iter().find_map()` | One pass / 一次完成搜索与转换 |

---

# Capstone Exercise: Diagnostic Event Pipeline / 综合练习：诊断事件流水线

🔴 **Challenge / 挑战** — 结合了枚举、Trait、迭代器、错误处理和泛型的综合练习。你将构建一个简化的诊断事件处理流水线，类似于生产环境 Rust 代码中使用的模式。

**Requirements / 需求：**
1. 定义 `enum Severity { Info, Warning, Critical }` 和 `struct DiagEvent { source: String, severity: Severity, message: String, fault_code: u32 }`。
2. 定义 `trait EventFilter` 包含 `fn should_include(&self, event: &DiagEvent) -> bool`。
3. 实现 `SeverityFilter` 和 `SourceFilter`。
4. 编写 `process_events(events: &[DiagEvent], filters: &[&dyn EventFilter]) -> Vec<String>`。
5. 编写 `parse_event(line: &str) -> Result<DiagEvent, String>` 解析 `"source:severity:fault_code:message"`。

<details><summary>Solution / 解决方案（点击展开）</summary>

```rust
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Severity { Info, Warning, Critical }

impl fmt::Display for Severity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", format!("{:?}", self).to_uppercase())
    }
}

// ... struct DiagEvent and filters implementations ...

fn process_events(events: &[DiagEvent], filters: &[&dyn EventFilter]) -> Vec<String> {
    events.iter()
        .filter(|e| filters.iter().all(|f| f.should_include(e)))
        .map(|e| format!("[{}] {} (FC:{}): {}", e.severity, e.source, e.fault_code, e.message))
        .collect()
}

fn parse_event(line: &str) -> Result<DiagEvent, String> {
    let parts: Vec<&str> = line.split(':').collect();
    if parts.len() != 4 { return Err("Invalid format".to_string()); }
    // ... parsing logic ...
    Ok(DiagEvent { 
        source: parts[0].into(), 
        severity: Severity::Critical, // Simplified
        message: parts[3].into(),
        fault_code: parts[2].parse().map_err(|e| e.to_string())?
    })
}
```
</details>
