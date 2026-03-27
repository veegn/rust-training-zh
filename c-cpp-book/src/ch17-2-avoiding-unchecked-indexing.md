## Avoiding unchecked indexing / 避免未检查的索引
 
 > **What you'll learn / 你将学到：** Why `vec[i]` is dangerous in Rust (panics on out-of-bounds), and safe alternatives like `.get()`, iterators, and `entry()` API for `HashMap`. Replaces C++'s undefined behavior with explicit handling.
 >
 > 为什么 `vec[i]` 在 Rust 中是危险的（越界时会发生 panic），以及像 `.get()`、迭代器和 `HashMap` 的 `entry()` API 这样的安全替代方案。我们将用显式处理来替代 C++ 中的未定义行为。
 
- - In C++, `vec[i]` and `map[key]` have undefined behavior / auto-insert on missing keys. Rust's `[]` panics on out-of-bounds.
+ - 在 C++ 中，`vec[i]` 和 `map[key]` 在索引越界或键缺失时会产生未定义行为或自动插入。而 Rust 的 `[]` 在越界时会直接 panic。
- - **Rule**: Use `.get()` instead of `[]` unless you can *prove* the index is valid.
+ - **规则**：除非你能**证明**索引是有效的，否则请使用 `.get()` 代替 `[]`。
 
- ### C++ → Rust comparison
+ ### C++ → Rust comparison / C++ vs Rust 对比
 ```cpp
 // C++ — silent UB or insertion
+// C++ — 静默的未定义行为或插入
 std::vector<int> v = {1, 2, 3};
- int x = v[10];        // UB! No bounds check with operator[]
+ int x = v[10];        // UB! No bounds check / 未定义行为！operator[] 不执行边界检查
 
 std::map<std::string, int> m;
- int y = m["missing"]; // Silently inserts key with value 0!
+ int y = m["missing"]; // Auto-insert / 静默插入键并赋值为 0！
 ```
 
 ```rust
 // Rust — safe alternatives
+// Rust — 安全替代方案
 let v = vec![1, 2, 3];
 
- // Bad: panics if index out of bounds
+ // Bad / 坏习惯：如果索引越界会发生 panic
 // let x = v[10];
 
- // Good: returns Option<&i32>
+ // Good / 好习惯：返回 Option<&i32>
- let x = v.get(10);              // None — no panic
+ let x = v.get(10);              // None / 返回 None —— 不会 panic
- let x = v.get(1).copied().unwrap_or(0);  // 2, or 0 if missing
+ let x = v.get(1).copied().unwrap_or(0);  // 结果为 2，如果缺失则为 0
 ```
 
- ### Real example: safe byte parsing from production Rust code
+ ### Real example: safe byte parsing from production Rust code / 真实示例：生产代码中的安全字节解析
 ```rust
 // Example: diagnostics.rs
 // Parsing a binary SEL record — buffer might be shorter than expected
+// 解析二进制 SEL 记录 —— 缓冲区可能比预期的短
 let sensor_num = bytes.get(7).copied().unwrap_or(0);
 let ppin = cpu_ppin.get(i).map(|s| s.as_str()).unwrap_or("");
 ```
 
- ### Real example: chained safe lookups with `.and_then()`
+ ### Real example: chained safe lookups with `.and_then()` / 真实示例：使用 `.and_then()` 进行链式安全查找
 ```rust
 // Example: profile.rs — double lookup: HashMap → Vec
+// 示例：双重查找：从 HashMap 到 Vec
 pub fn get_processor(&self, location: &str) -> Option<&Processor> {
     self.processor_by_location
         .get(location)                              // HashMap → Option<&usize>
         .and_then(|&idx| self.processors.get(idx))   // Vec → Option<&Processor>
 }
- // Both lookups return Option — no panics, no UB
+ // Both return Option / 两次查找均返回 Option —— 无 panic，无未定义行为
 ```
 
- ### Real example: safe JSON navigation
+ ### Real example: safe JSON navigation / 真实示例：安全的 JSON 导航
 ```rust
 // Example: framework.rs — every JSON key returns Option
 pub let manufacturer = product_fru
-     .get("Manufacturer")            // Option<&Value>
+     .get("Manufacturer")            // Option<&Value> / 获取“Manufacturer”键
-     .and_then(|v| v.as_str())       // Option<&str>
+     .and_then(|v| v.as_str())       // Option<&str> / 尝试转为字符串切片
-     .unwrap_or(UNKNOWN_VALUE)       // &str (safe fallback)
+     .unwrap_or(UNKNOWN_VALUE)       // Safe fallback / 安全的备选值
      .to_string();
 ```
- Compare to the C++ pattern: `json["SystemInfo"]["ProductFru"]["Manufacturer"]` — any missing key throws `nlohmann::json::out_of_range`.
+ 对比 C++ 模式：`json["SystemInfo"]["ProductFru"]["Manufacturer"]` —— 任何缺失的键都会抛出 `nlohmann::json::out_of_range` 异常。
 
- ### When `[]` is acceptable
+ ### When `[]` is acceptable / 何时使用 `[]` 是可以接受的
- - **After a bounds check**: `if i < v.len() { v[i] }`
+ - **在边界检查之后**：`if i < v.len() { v[i] }`。
- - **In tests**: Where panicking is the desired behavior
+ - **在测试中**：此时发生 panic 正是预期的行为。
- - **With constants**: `let first = v[0];` right after `assert!(!v.is_empty());`
+ - **配合常量使用**：例如在 `assert!(!v.is_empty());` 之后立即使用 `let first = v[0];`。
 
 ----
 
- ## Safe value extraction with unwrap_or
+ ## Safe value extraction with unwrap_or / 使用 unwrap_or 进行安全值提取
 
- - `unwrap()` panics on `None` / `Err`. In production code, prefer the safe alternatives.
+ - `unwrap()` 在遇到 `None` 或 `Err` 时会发生 panic。在生产代码中，请优先选择安全的替代方案。
 
- ### The unwrap family
+ ### The unwrap family / unwrap 家族
-| **Method** | **Behavior on None/Err** | **Use When** |
+| **Method / 方法** | **Behavior on None/Err / 失败时的行为** | **Use When / 适用场景** |
 |-----------|------------------------|-------------|
-| `.unwrap()` | **Panics** | Tests only, or provably infallible |
+| `.unwrap()` | **Panics** / 发生 Panic | Tests / 仅限测试，或被证明绝不会失败 |
-| `.expect("msg")` | Panics with message | When panic is justified, explain why |
+| `.expect("msg")` | Panic with msg / 带消息 Panic | 理由充分时，并解释原因 |
-| `.unwrap_or(default)` | Returns `default` | You have a cheap constant fallback |
+| `.unwrap_or(default)` | Returns default / 返回默认值 | 拥有廉价的常量备选值时 |
-| `.unwrap_or_else(\|\| expr)` | Calls closure | Fallback is expensive to compute |
+| `.unwrap_or_else(...)` | Calls closure / 调用闭包 | 备选值的计算开销较大时 |
-| `.unwrap_or_default()` | Returns `Default::default()` | Type implements `Default` |
+| `.unwrap_or_default()` | Returns Default / 返回默认值 | 类型实现了 `Default` 时 |
 
- ### Real example: parsing with safe defaults
+ ### Real example: parsing with safe defaults / 真实示例：带有安全默认值的解析
 ```rust
 // Example: peripherals.rs
 // Regex capture groups might not match — provide safe fallbacks
+// 正则捕获组可能不匹配 —— 提供安全的备选方案
 let bus_hex = caps.get(1).map(|m| m.as_str()).unwrap_or("00");
 let fw_status = caps.get(5).map(|m| m.as_str()).unwrap_or("0x0");
 let bus = u8::from_str_radix(bus_hex, 16).unwrap_or(0);
 ```
 
- ### Real example: `unwrap_or_else` with fallback struct
+ ### Real example: `unwrap_or_else` with fallback struct / 真实示例：配合备选结构体使用 `unwrap_or_else`
 ```rust
 // Example: framework.rs
 // Full function wraps logic in an Option-returning closure;
 // if anything fails, return a default struct:
+// 将逻辑封装在返回 Option 的闭包中；如果任何一步失败，则返回默认结构体：
 (|| -> Option<BaseboardFru> {
     let content = std::fs::read_to_string(path).ok()?;
     let json: serde_json::Value = serde_json::from_str(&content).ok()?;
-    // ... extract fields with .get()? chains
+    // ... extract / 使用 .get()? 链式提取字段
     Some(baseboard_fru)
 })()
 .unwrap_or_else(|| BaseboardFru {
     manufacturer: String::new(),
     model: String::new(),
     product_part_number: String::new(),
     serial_number: String::new(),
     asset_tag: String::new(),
 })
 ```
 
- ### Real example: `unwrap_or_default` on config deserialization
+ ### Real example: `unwrap_or_default` on config deserialization / 真实示例：配置反序列化时的 `unwrap_or_default`
 ```rust
 // Example: framework.rs
 // If JSON config parsing fails, fall back to Default — no crash
+// 如果 JSON 配置解析失败，则回退到默认值 —— 不会崩溃
 Ok(json) => serde_json::from_str(&json).unwrap_or_default(),
 ```
- The C++ equivalent would be a `try/catch` around `nlohmann::json::parse()` with manual default construction in the catch block.
+ C++ 的对应做法是围绕 `nlohmann::json::parse()` 使用 `try/catch`，并在 catch 块中手动构造默认值。
 
 ----
 
- ## Functional transforms: map, map_err, find_map
+ ## Functional transforms: map, map_err, find_map / 函数式转换：map、map_err、find_map
 
- - These methods on `Option` and `Result` let you transform the contained value without unwrapping, replacing nested `if/else` with linear chains.
+ - `Option` 和 `Result` 上的这些方法允许你在不解包的情况下转换其中的值，从而将嵌套的 `if/else` 替换为线性链式调用。
 
- ### Quick reference
+ ### Quick reference / 快速参考
-| **Method** | **On** | **Does** | **C++ Equivalent** |
+| **Method / 方法** | **On / 作用于** | **Does / 功能** | **C++ Equivalent / C++ 等价物** |
 |-----------|-------|---------|-------------------|
-| `.map(\|v\| ...)` | `Option` / `Result` | Transform the `Some`/`Ok` value | `if (opt) { *opt = transform(*opt); }` |
+| `.map(...)` | `Option` / `Result` | Transform / 转换内部值 | `if (opt) { *opt = ...; }` |
-| `.map_err(\|e\| ...)` | `Result` | Transform the `Err` value | Adding context to catch block |
+| `.map_err(...)` | `Result` | Transform Err / 转换错误值 | 为 catch 块添加上下文 |
-| `.and_then(\|v\| ...)` | `Option` / `Result` | Chain operations that return `Option`/`Result` | Nested if-checks |
+| `.and_then(...)` | `Option` / `Result` | Chain fallible ops / 链式调用可能失败的操作 | 嵌套的 if 检查 |
-| `.find_map(\|v\| ...)` | Iterator | `find` + `map` in one pass | Loop with `if + break` |
+| `.find_map(...)` | Iterator | find + map / 一次完成查找与转换 | 带 if + break 的循环 |
-| `.filter(\|v\| ...)` | `Option` / Iterator | Keep only values matching predicate | `if (!predicate) return nullopt;` |
+| `.filter(...)` | `Option` / Iterator | Keep matching / 仅保留符合条件的项 | 谓词判断 |
-| `.ok()?` | `Result` | Convert `Result → Option` and propagate `None` | `if (result.has_error()) return nullopt;` |
+| `.ok()?` | `Result` | Result → Option / 错误转为 None | 将错误处理转换为 Option 处理 |
 
- ### Real example: `.and_then()` chain for JSON field extraction
+ ### Real example: `.and_then()` chain for JSON field extraction / 真实示例：用于 JSON 字段提取的 `.and_then()` 链
 ```rust
 // Example: framework.rs — finding serial number with fallbacks
+// 示例：通过备选方案查找序列号
 let sys_info = json.get("SystemInfo")?;
 
 // Try BaseboardFru.BoardSerialNumber first
+// 首先尝试 BaseboardFru.BoardSerialNumber
 if let Some(serial) = sys_info
     .get("BaseboardFru")
     .and_then(|b| b.get("BoardSerialNumber"))
     .and_then(|v| v.as_str())
-    .filter(valid_serial)     // Only accept non-empty, valid serials
+    .filter(valid_serial)     // Only valid / 仅接收非空且有效的序列号
 {
     return Some(serial.to_string());
 }
 
 // Fallback to BoardFru.SerialNumber
+// 回退到 BoardFru.SerialNumber
 sys_info
     .get("BoardFru")
     .and_then(|b| b.get("SerialNumber"))
     .and_then(|v| v.as_str())
     .filter(valid_serial)
-    .map(|s| s.to_string())   // Convert &str → String only if Some
+    .map(|s| s.to_string())   // Only if Some / 仅在为 Some 时将 &str 转为 String
 ```
- In C++ this would be a pyramid of `if (json.contains("BaseboardFru")) { if (json["BaseboardFru"].contains("BoardSerialNumber")) { ... } }`.
+ 在 C++ 中，这会变成一座由 `if (json.contains("BaseboardFru")) { if (json["BaseboardFru"].contains("BoardSerialNumber")) { ... } }` 构成的“代码金字塔”。
 
- ### Real example: `find_map` — search + transform in one pass
+ ### Real example: `find_map` — search + transform in one pass / 真实示例：`find_map` —— 一次完成搜索与转换
 ```rust
 // Example: context.rs — find SDR record matching sensor + owner
+// 示例：查找与传感器及所有者匹配的 SDR 记录
 pub fn find_for_event(&self, sensor_number: u8, owner_id: u8) -> Option<&SdrRecord> {
     self.by_sensor.get(&sensor_number).and_then(|indices| {
         indices.iter().find_map(|&i| {
             let record = &self.records[i];
             if record.sensor_owner_id() == Some(owner_id) {
                 Some(record)
             } else {
                 None
             }
         })
     })
 }
 ```
- `find_map` is `find` + `map` fused: it stops at the first match and transforms it. The C++ equivalent is a `for` loop with an `if` + `break`.
+ `find_map` 是将 `find` 与 `map` 融合在一起：它在第一个匹配项处停止并对其进行转换。C++ 的等效做法是带 `if` + `break` 的 `for` 循环。
 
- ### Real example: `map_err` for error context
+ ### Real example: `map_err` for error context / 真实示例：使用 `map_err` 添加错误上下文
 ```rust
 // Example: main.rs — add context to errors before propagating
+// 示例：在传播错误之前为其添加上下文
 let json_str = serde_json::to_string_pretty(&config)
     .map_err(|e| format!("Failed to serialize config: {}", e))?;
 ```
- Transforms a `serde_json::Error` into a descriptive `String` error that includes context about *what* failed.
+ 将 `serde_json::Error` 转换为包含“哪里失败了”这一上下文的描述性 `String` 错误。
 
 ----
 
- ## JSON handling: nlohmann::json → serde
+ ## JSON handling: nlohmann::json → serde / JSON 处理：nlohmann::json 至 serde 的演进
 
- - C++ teams typically use `nlohmann::json` for JSON parsing. Rust uses **serde** + **serde_json** — which is more powerful because the JSON schema is encoded *in the type system*.
+ - C++ 团队通常使用 `nlohmann::json` 进行 JSON 解析。Rust 则使用 **serde** + **serde_json** —— 这更强大，因为 JSON 模式（Schema）是编码在**类型系统**中的。
 
- ### C++ (nlohmann) vs Rust (serde) comparison
+ ### C++ (nlohmann) vs Rust (serde) comparison / C++ vs Rust 对比
 
 ```cpp
 // C++ with nlohmann::json — runtime field access
+// C++ 使用 nlohmann::json — 运行时字段访问
 #include <nlohmann/json.hpp>
 using json = nlohmann::json;
 
 struct Fan {
     std::string logical_id;
     std::vector<std::string> sensor_ids;
 };
 
 Fan parse_fan(const json& j) {
     Fan f;
-    f.logical_id = j.at("LogicalID").get<std::string>();    // throws if missing
+    f.logical_id = j.at("LogicalID").get<std::string>();    // Throws / 若缺失则抛异常
-    if (j.contains("SDRSensorIdHexes")) {                   // manual default handling
+    if (j.contains("SDRSensorIdHexes")) {                   // Manual / 手动默认值处理
         f.sensor_ids = j["SDRSensorIdHexes"].get<std::vector<std::string>>();
     }
     return f;
 }
 ```
 
 ```rust
 // Rust with serde — compile-time schema, automatic field mapping
+// Rust 使用 serde — 编译时模式，自动字段映射
 use serde::{Serialize, Deserialize};
 
 #[derive(Debug, Clone, Serialize, Deserialize)]
 pub struct Fan {
     pub logical_id: String,
-    #[serde(rename = "SDRSensorIdHexes", default)]  // JSON key → Rust field
+    #[serde(rename = "SDRSensorIdHexes", default)]  // JSON key -> field / 映射键名并提供默认值
-    pub sensor_ids: Vec<String>,                     // Missing → empty Vec
+    pub sensor_ids: Vec<String>,                     // Missing -> empty / 缺失则为空 Vec
     #[serde(default)]
-    pub sensor_names: Vec<String>,                   // Missing → empty Vec
+    pub sensor_names: Vec<String>,                   // Missing -> empty / 缺失则为空 Vec
 }
 
- // One line replaces the entire parse function:
+ // One line replaces parse / 一行代码即可替代整个解析函数：
 let fan: Fan = serde_json::from_str(json_str)?;
 ```
 
- ### Key serde attributes (real examples from production Rust code)
+ ### Key serde attributes / 关键 serde 属性（生产代码示例）
 
-| **Attribute** | **Purpose** | **C++ Equivalent** |
+| **Attribute / 属性** | **Purpose / 用途** | **C++ Equivalent / C++ 等价物** |
 |--------------|------------|--------------------|
-| `#[serde(default)]` | Use `Default::default()` for missing fields | `if (j.contains(key)) { ... } else { default; }` |
-| `#[serde(default)]` | 缺失时使用 `Default::default()` | 包含性检查与手动赋值 |
-| `#[serde(rename = "Key")]` | Map JSON key name to Rust field name | Manual `j.at("Key")` access |
-| `#[serde(rename = "Key")]` | 重命名 JSON 键名 | 手动的 `j.at("Key")` 访问 |
-| `#[serde(flatten)]` | Absorb unknown keys into `HashMap` | `for (auto& [k,v] : j.items()) { ... }` |
-| `#[serde(flatten)]` | 吸收未知键到 `HashMap` 中 | 手动遍历并吸收 |
-| `#[serde(skip)]` | Don't serialize/deserialize this field | Not storing in JSON |
-| `#[serde(skip)]` | 不进行序列化/反序列化 | 跳过存储 |
-| `#[serde(tag = "type")]` | Internally tagged enum (discriminator field) | `if (j["type"] == "gpu") { ... }` |
+| `#[serde(tag = "type")]` | Enum tag / 内部标签枚举 | 基于字段值的 `if/else` |
 
- ### Real example: full config struct
+ ### Real example: full config struct / 真实示例：完整配置结构体
 ```rust
 // Example: diag.rs
 #[derive(Debug, Clone, Serialize, Deserialize)]
 pub struct DiagConfig {
     pub sku: SkuConfig,
     #[serde(default)]
-    pub level: DiagLevel,            // Missing → DiagLevel::default()
+    pub level: DiagLevel,            // Missing -> default / 缺失则使用默认级别
     #[serde(default)]
-    pub modules: ModuleConfig,       // Missing → ModuleConfig::default()
+    pub modules: ModuleConfig,       // Missing -> default / 缺失则使用默认配置
     #[serde(default)]
-    pub output_dir: String,          // Missing → ""
+    pub output_dir: String,          // Missing -> empty / 缺失则为空内容
     #[serde(default, flatten)]
-    pub options: HashMap<String, serde_json::Value>,  // Absorbs unknown keys
+    pub options: HashMap<String, serde_json::Value>,  // Absorb / 吸收未知键
 }
 
- // Loading is 3 lines (vs ~20+ in C++ with nlohmann):
+ // Loading / 加载仅需 3 行（对比 C++ nlohmann 约需 20+ 行）：
 let content = std::fs::read_to_string(path)?;
 let config: DiagConfig = serde_json::from_str(&content)?;
 Ok(config)
 ```
 
- ### Enum deserialization with `#[serde(tag = "type")]`
+ ### Enum deserialization with `#[serde(tag = "type")]` / 使用 `#[serde(tag = "type")]` 进行枚举反序列化
 ```rust
 // Example: components.rs
 #[derive(Debug, Clone, Serialize, Deserialize)]
- #[serde(tag = "type")]                   // JSON: {"type": "Gpu", "product": ...}
+ #[serde(tag = "type")]                   // JSON 格式示例：{"type": "Gpu", "product": ...}
 pub enum PcieDeviceKind {
     Gpu { product: GpuProduct, manufacturer: GpuManufacturer },
     Nic { product: NicProduct, manufacturer: NicManufacturer },
     NvmeDrive { drive_type: StorageDriveType, capacity_gb: u32 },
-    // ... 9 more variants
+    // ... 9 more / 还有 9 种变体
 }
- // serde automatically dispatches on the "type" field — no manual if/else chain
+ // Dispatch / serde 会根据 "type" 字段自动分发 —— 无需手动 if/else 链
 ```
- The C++ equivalent would be: `if (j["type"] == "Gpu") { parse_gpu(j); } else if (j["type"] == "Nic") { parse_nic(j); } ...`
+ C++ 的对应做法是：`if (j["type"] == "Gpu") { parse_gpu(j); } else if (j["type"] == "Nic") { parse_nic(j); } ...`
 
- # Exercise: JSON deserialization with serde
+ # Exercise: JSON deserialization with serde / 练习：使用 serde 进行 JSON 反序列化
 
- - Define a `ServerConfig` struct that can be deserialized from the following JSON:
+ - 定义一个 `ServerConfig` 结构体，使其能够从以下 JSON 中反序列化：
 ```json
 {
     "hostname": "diag-node-01",
     "port": 8080,
     "debug": true,
     "modules": ["accel_diag", "nic_diag", "cpu_diag"]
 }
 ```
- - Use `#[derive(Deserialize)]` and `serde_json::from_str()` to parse it
+ - 使用 `#[derive(Deserialize)]` 和 `serde_json::from_str()` 进行解析。
- - Add `#[serde(default)]` to `debug` so it defaults to `false` if missing
+ - 为 `debug` 添加 `#[serde(default)]`，使其在缺失时默认为 `false`。
- - **Bonus**: Add an `enum DiagLevel { Quick, Full, Extended }` field with `#[serde(default)]` that defaults to `Quick`
+ - **加分项**：添加一个 `enum DiagLevel { Quick, Full, Extended }` 字段，并带有 `#[serde(default)]` 且默认为 `Quick`。
 
- **Starter code** (requires `cargo add serde --features derive` and `cargo add serde_json`):
+ **入门代码**（需要运行 `cargo add serde --features derive` 和 `cargo add serde_json`）：
 ```rust
 use serde::Deserialize;
 
- // TODO: Define DiagLevel enum with Default impl
+ // TODO: 定义具有 Default 实现的 DiagLevel 枚举
 
- // TODO: Define ServerConfig struct with serde attributes
+ // TODO: 定义带有 serde 属性的 ServerConfig 结构体
 
 fn main() {
     let json_input = r#"{
         "hostname": "diag-node-01",
         "port": 8080,
         "debug": true,
         "modules": ["accel_diag", "nic_diag", "cpu_diag"]
     }"#;
 
-    // TODO: Deserialize and print the config
+    // TODO: 反序列化并打印配置
-    // TODO: Try parsing JSON with "debug" field missing — verify it defaults to false
+    // TODO: 尝试解析缺失 "debug" 字段的 JSON —— 验证其默认是否为 false
 }
 ```
 
- <details><summary>Solution (click to expand)</summary>
+ <details><summary>Solution / 解决方案（点击展开）</summary>
 
 ```rust
 use serde::Deserialize;
 
 #[derive(Debug, Deserialize, Default)]
 enum DiagLevel {
     #[default]
     Quick,
     Full,
     Extended,
 }
 
 #[derive(Debug, Deserialize)]
 struct ServerConfig {
     hostname: String,
     port: u16,
-    #[serde(default)]       // defaults to false if missing
+    #[serde(default)]       // Missing -> false / 缺失则默认为 false
     debug: bool,
     modules: Vec<String>,
-    #[serde(default)]       // defaults to DiagLevel::Quick if missing
+    #[serde(default)]       // Missing -> Quick / 缺失则默认为 Quick
     level: DiagLevel,
 }
 
 fn main() {
     let json_input = r#"{
         "hostname": "diag-node-01",
         "port": 8080,
         "debug": true,
         "modules": ["accel_diag", "nic_diag", "cpu_diag"]
     }"#;
 
     let config: ServerConfig = serde_json::from_str(json_input)
         .expect("Failed to parse JSON");
     println!("{config:#?}");
 
-    // Test with missing optional fields
+    // Test / 测试缺失可选字段的情况
     let minimal = r#"{
         "hostname": "node-02",
         "port": 9090,
         "modules": []
     }"#;
     let config2: ServerConfig = serde_json::from_str(minimal)
         .expect("Failed to parse minimal JSON");
-    println!("debug (default): {}", config2.debug);    // false
+    println!("debug (default): {}", config2.debug);    // 结果为 false
-    println!("level (default): {:?}", config2.level);  // Quick
+    println!("level (default): {:?}", config2.level);  // 结果为 Quick
 }
 ```
 
 </details>
