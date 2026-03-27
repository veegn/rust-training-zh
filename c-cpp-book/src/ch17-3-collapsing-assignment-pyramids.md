## Collapsing assignment pyramids with closures / 精简层层嵌套的赋值结构
 
 > **What you'll learn / 你将学到：** How Rust's expression-based syntax and closures flatten deeply-nested C++ `if/else` validation chains into clean, linear code.
 >
 > Rust 基于表达式的语法和闭包如何将 C++ 中深层嵌套的 `if/else` 验证链简化为整洁、线性的代码。
 
- - C++ often requires multi-block `if/else` chains to assign variables, especially when validation or fallback logic is involved. Rust's expression-based syntax and closures collapse these into flat, linear code.
+ - 在 C++ 中，为了给变量赋值（特别是涉及验证或备选逻辑时），通常需要编写多个 `if/else` 代码块。Rust 基于表达式的语法和闭包将这些结构精简为扁平的线性代码。
 
- ### Pattern 1: Tuple assignment with `if` expression
+ ### Pattern 1: Tuple assignment with `if` expression / 模式 1：使用 `if` 表达式进行元组赋值
 ```cpp
 // C++ — three variables set across a multi-block if/else chain
+// C++ — 在多个 if/else 块中设置三个变量
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
 // Rust equivalent:accel_fieldiag.rs
 // Single expression assigns all three at once:
+// 单个表达式同时为三个变量赋值：
 let (fault_code, der_marker, recommended_action) = if is_c44ad {
     (32709u32, "CSI_WARN", "No action")
 } else if error.is_hardware_error() {
     (67956u32, "CSI_ERR", "Replace GPU")
 } else {
     (32709u32, "CSI_WARN", "No action")
 };
 ```
 
- ### Pattern 2: IIFE (Immediately Invoked Function Expression) for fallible chains
+ ### Pattern 2: IIFE (Immediately Invoked Function Expression) for fallible chains / 模式 2：用于易错链式调用的 IIFE（立即调用函数表达式）
 ```cpp
 // C++ — pyramid of doom for JSON navigation
+// C++ — JSON 导航中的“末日金字塔”
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
 // Rust equivalent:framework.rs
 // Closure + ? operator collapses the pyramid into linear code:
+// 闭包 + ? 运算符将金字塔结构精简为线性代码：
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
- The closure creates an `Option<String>` scope where `?` bails early at any step. The `.unwrap_or_else()` provides the fallback once, at the end.
+ 闭包创建了一个 `Option<String>` 作用域，其中 `?` 可以在任何步骤提前返回。`.unwrap_or_else()` 则在最后统一提供备选方案。
 
- ### Pattern 3: Iterator chain replacing manual loop + push_back
+ ### Pattern 3: Iterator chain replacing manual loop + push_back / 模式 3：替代手动循环和 push_back 的迭代器链
 ```cpp
 // C++ — manual loop with intermediate variables
+// C++ — 带有中间变量的手动循环
 std::vector<std::tuple<std::vector<std::string>, std::string, std::string>> gpu_info;
 for (const auto& [key, info] : gpu_pcie_map) {
     std::vector<std::string> bdfs;
-    // ... parse bdf_path into bdfs
+    // ... parse / 将 bdf_path 解析为 bdfs
     std::string serial = info.serial_number.value_or("UNKNOWN");
     std::string model = info.model_number.value_or(model_name);
     gpu_info.push_back({bdfs, serial, model});
 }
 ```
 
 ```rust
 // Rust equivalent:peripherals.rs
 // Single chain: values() → map → collect
+// 单个链式调用：values() -> map -> collect
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
 
- ### Pattern 4: `.filter().collect()` replacing loop + `if (condition) continue`
+ ### Pattern 4: `.filter().collect()` replacing loop + `if (condition) continue` / 模式 4：替代循环和继续条件的 `.filter().collect()`
 ```cpp
 // C++
+// C++ 示例
 std::vector<TestResult*> failures;
 for (auto& t : test_results) {
     if (!t.is_pass()) {
         failures.push_back(&t);
     }
 }
 ```
 
 ```rust
 // Rust — from accel_diag/src/healthcheck.rs
+// Rust 示例
 pub fn failed_tests(&self) -> Vec<&TestResult> {
     self.test_results.iter().filter(|t| !t.is_pass()).collect()
 }
 ```
 
- ### Summary: When to use each pattern
+ ### Summary: When to use each pattern / 总结：何时使用各模式
-| **C++ Pattern** | **Rust Replacement** | **Key Benefit** |
+| **C++ Pattern / C++ 模式** | **Rust Replacement / Rust 替代方案** | **Key Benefit / 核心优势** |
 |----------------|---------------------|-----------------|
-| Multi-block variable assignment | `let (a, b) = if ... { } else { };` | All variables bound atomically |
+| Multi-block assignment / 多块赋值 | `let (a, b) = if ... { } else { };` | Atomic / 变量原子绑定 |
-| Nested `if (contains)` pyramid | IIFE closure with `?` operator | Linear, flat, early-exit |
+| Nested if pyramid / 嵌套 if 金字塔 | IIFE closure with `?` / IIFE 闭包 + `?` | Flat, early-exit / 扁平、提前退出 |
-| `for` loop + `push_back` | `.iter().map(\|\|).collect()` | No intermediate mut Vec |
+| `for` + `push_back` | `.iter().map().collect()` | No intermediate / 无需中间的可变 Vec |
-| `for` + `if (cond) continue` | `.iter().filter(\|\|).collect()` | Declarative intent |
+| `for` + `if continue` | `.iter().filter().collect()` | Declarative / 声明性意图 |
-| `for` + `if + break` (find first) | `.iter().find_map(\|\|)` | Search + transform in one pass |
-| `for` + `if + break` (查找首个) | `.iter().find_map()` | One pass / 一次完成搜索与转换 |
 
 ----
 
- # Capstone Exercise: Diagnostic Event Pipeline
+ # Capstone Exercise: Diagnostic Event Pipeline / 综合练习：诊断事件流水线
 
- 🔴 **Challenge** — integrative exercise combining enums, traits, iterators, error handling, and generics
+ 🔴 **挑战** —— 结合了枚举、Trait、迭代器、错误处理和泛型的综合练习
 
- This integrative exercise brings together enums, traits, iterators, error handling, and generics. You'll build a simplified diagnostic event processing pipeline similar to patterns used in production Rust code.
+ 这个综合练习将枚举、Trait、迭代器、错误处理和泛型结合在一起。你将构建一个简化的诊断事件处理流水线，类似于生产环境 Rust 代码中使用的模式。
 
- **Requirements:**
+ **需求：**
- 1. Define an `enum Severity { Info, Warning, Critical }` with `Display`, and a `struct DiagEvent` containing `source: String`, `severity: Severity`, `message: String`, and `fault_code: u32`
+ 1. 定义一个带有 `Display` 实现的 `enum Severity { Info, Warning, Critical }`，以及一个包含 `source: String`、`severity: Severity`、`message: String` 和 `fault_code: u32` 的 `struct DiagEvent`。
- 2. Define a `trait EventFilter` with a method `fn should_include(&self, event: &DiagEvent) -> bool`
+ 2. 定义一个 `trait EventFilter`，其中包含一个方法 `fn should_include(&self, event: &DiagEvent) -> bool`。
- 3. Implement two filters: `SeverityFilter` (only events >= a given severity) and `SourceFilter` (only events from a specific source string)
+ 3. 实现两个过滤器：`SeverityFilter`（仅包含大于或等于指定严重性的事件）和 `SourceFilter`（仅包含来自特定源字符串的事件）。
- 4. Write a function `fn process_events(events: &[DiagEvent], filters: &[&dyn EventFilter]) -> Vec<String>` that returns formatted report lines for events that pass **all** filters
+ 4. 编写一个函数 `fn process_events(events: &[DiagEvent], filters: &[&dyn EventFilter]) -> Vec<String>`，返回通过**所有**过滤器的事件的格式化报告行。
- 5. Write a `fn parse_event(line: &str) -> Result<DiagEvent, String>` that parses lines of the form `"source:severity:fault_code:message"` (return `Err` for bad input)
+ 5. 编写一个 `fn parse_event(line: &str) -> Result<DiagEvent, String>`，解析格式为 `"source:severity:fault_code:message"` 的行（输入错误时返回 `Err`）。
 
- **Starter code:**
+ **入门代码：**
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
-        todo!()
+        todo!() // TODO: 实现此处
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
- // TODO: impl EventFilter for SeverityFilter
+ // TODO: 为 SeverityFilter 实现 EventFilter
 
 struct SourceFilter {
     source: String,
 }
- // TODO: impl EventFilter for SourceFilter
+ // TODO: 为 SourceFilter 实现 EventFilter
 
 fn process_events(events: &[DiagEvent], filters: &[&dyn EventFilter]) -> Vec<String> {
-    // TODO: Filter events that pass ALL filters, format as
+    // TODO: 过滤通过所有过滤器的事件，格式化为：
     // "[SEVERITY] source (FC:fault_code): message"
-    todo!()
+    todo!() // TODO: 实现此处
 }
 
 fn parse_event(line: &str) -> Result<DiagEvent, String> {
-    // Parse "source:severity:fault_code:message"
-    // Return Err for invalid input
-    todo!()
+    // 解析 "source:severity:fault_code:message"
+    // 错误输入返回 Err
+    todo!() // TODO: 实现此处
 }
 
 fn main() {
     let raw_lines = vec![
         "accel_diag:Critical:67956:ECC uncorrectable error detected",
         "nic_diag:Warning:32709:Link speed degraded",
         "accel_diag:Info:10001:Self-test passed",
         "cpu_diag:Critical:55012:Thermal throttling active",
         "accel_diag:Warning:32710:PCIe link width reduced",
     ];
 
-    // Parse all lines, collect successes and report errors
+    // 解析所有行，收集成功结果并报告错误
     let events: Vec<DiagEvent> = raw_lines.iter()
         .filter_map(|line| match parse_event(line) {
             Ok(e) => Some(e),
             Err(e) => { eprintln!("Parse error: {e}"); None }
         })
         .collect();
 
-    // Apply filters: only Critical+Warning events from accel_diag
+    // 应用过滤器：仅包含 accel_diag 且严重性在 Warning 及以上的事件
     let sev_filter = SeverityFilter { min_severity: Severity::Warning };
     let src_filter = SourceFilter { source: "accel_diag".to_string() };
     let filters: Vec<&dyn EventFilter> = vec![&sev_filter, &src_filter];
 
     let report = process_events(&events, &filters);
     for line in &report {
         println!("{line}");
     }
     println!("--- {} event(s) matched ---", report.len());
 }
 ```
 
- <details><summary>Solution (click to expand)</summary>
+ <details><summary>Solution / 解决方案（点击展开）</summary>
 
 ```rust
 use std::fmt;
@@ -337,11 +333,11 @@
     for line in &report {
         println!("{line}");
     }
     println!("--- {} event(s) matched ---", report.len());
 }
- // Output:
+ // Output / 输出：
 // [CRITICAL] accel_diag (FC:67956): ECC uncorrectable error detected
 // [WARNING] accel_diag (FC:32710): PCIe link width reduced
 // --- 2 event(s) matched ---
 ```
 
 </details>
