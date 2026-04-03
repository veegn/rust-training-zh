[English Original](../en/ch17-2-avoiding-unchecked-indexing.md)

## 避免未检查的索引访问

> **你将学到：** 为什么在 Rust 中使用 `vec[i]` 是危险的（越界会引发 panic）、以及安全的替代方案如 `.get()`、迭代器和 `HashMap` 的 `entry()` API。用显式的处理方式取代 C++ 中的未定义行为。

- 在 C++ 中，`vec[i]` 和 `map[key]` （在键缺失时自动插入）往往伴随着未定义行为。而 Rust 的 `[]` 在发生越界时会直接引发 panic。
- **准则**：除非你能**证明**索引始终有效，否则请优先使用 `.get()` 而非 `[]`。

### C++ 与 Rust 的对比
```cpp
// C++ —— 默默发生的未定义行为 (UB) 或自动插入
std::vector<int> v = {1, 2, 3};
int x = v[10];        // UB！使用 operator[] 不进行边界检查

std::map<std::string, int> m;
int y = m["missing"]; // 默默地插入键，并赋予默认值 0！
```

```rust
// Rust —— 安全替代方案
let v = vec![1, 2, 3];

// 错误做法：如果索引越界会引发 panic
// let x = v[10];

// 正确做法：返回 Option<&i32>
let x = v.get(10);              // 返回 None —— 不会引发 panic
---

### 真实示例：生产代码中的安全字节解析
```rust
// 示例：diagnostics.rs
// 解析二进制 SEL 记录 —— 缓冲区长度可能短于预期
let sensor_num = bytes.get(7).copied().unwrap_or(0);
let ppin = cpu_ppin.get(i).map(|s| s.as_str()).unwrap_or("");
```

### 真实示例：使用 `.and_then()` 进行链式安全查考
```rust
// 示例：profile.rs —— 双重查找：HashMap → Vec
pub fn get_processor(&self, location: &str) -> Option<&Processor> {
    self.processor_by_location
        .get(location)                              // HashMap → Option<&usize>
        .and_then(|&idx| self.processors.get(idx))   // Vec → Option<&Processor>
}
// 两次查考均返回 Option —— 无 panic，无未定义行为
```

### 真实示例：安全的 JSON 导航
```rust
// 示例：framework.rs —— 每一个 JSON 键均返回 Option
let manufacturer = product_fru
    .get("Manufacturer")            // Option<&Value>
    .and_then(|v| v.as_str())       // Option<&str>
    .unwrap_or(UNKNOWN_VALUE)       // &str (安全回退值)
    .to_string();
```
对比 C++ 模式：`json["SystemInfo"]["ProductFru"]["Manufacturer"]` —— 任何键的缺失都会抛出 `nlohmann::json::out_of_range` 异常。

### 何时使用 `[]` 是可以接受的
- **在边界检查之后**：`if i < v.len() { v[i] }`
- **在单元测试中**：此时 panic 是预期的行为
- **使用常量时**：例如在 `assert!(!v.is_empty());` 之后紧接着使用 `let first = v[0];`

---

## 使用 unwrap_or 进行安全提取

- `unwrap()` 在遇到 `None` / `Err` 时会引发 panic。在生产环境中，应优先选用安全的替代方案。

### unwrap 家族

| **方法** | **遇到 None/Err 时的行为** | **适用场景** |
|-----------|------------------------|-------------|
| `.unwrap()` | **引发 Panic** | 仅限于测试，或可证明绝对不会失败时 |
| `.expect("msg")` | 引发带信息的 Panic | 当 panic 合理时，对其原因提供解释 |
| `.unwrap_or(default)` | 返回 `default` | 提供零开销的常量回退值时 |
| `.unwrap_or_else(|| expr)` | 调用闭包 | 当回退值计算开销巨大时 |
| `.unwrap_or_default()` | 返回 `Default::default()` | 当类型实现了 `Default` Trait 时 |

---

### 真实示例：使用安全默认值进行解析
```rust
// 示例：peripherals.rs
// 正则捕获组可能未匹配 —— 提供安全的回退值
let bus_hex = caps.get(1).map(|m| m.as_str()).unwrap_or("00");
let fw_status = caps.get(5).map(|m| m.as_str()).unwrap_or("0x0");
let bus = u8::from_str_radix(bus_hex, 16).unwrap_or(0);
```

### 真实示例：带有回退结构体的 `unwrap_or_else` 
```rust
// 示例：framework.rs
// 该函数在返回 Option 的闭包中封装了逻辑；
// 如果任何环节失败，则返回一个默认的结构体：
(|| -> Option<BaseboardFru> {
    let content = std::fs::read_to_string(path).ok()?;
    let json: serde_json::Value = serde_json::from_str(&content).ok()?;
    // ... 之后是利用 .get()? 链提取字段
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

### 真实示例：在配置反序列化时使用 `unwrap_or_default`
```rust
// 示例：framework.rs
// 如果 JSON 配置解析失败，则回退到 Default 状态 —— 避免程序崩溃
Ok(json) => serde_json::from_str(&json).unwrap_or_default(),
```
其 C++ 等效做法通常是在 `nlohmann::json::parse()` 周围使用 `try/catch` 块，并在 catch 块中手动构造默认对象。

---

## 函数式转换：map、map_err 与 find_map

- `Option` 和 `Result` 的这些方法允许你在不解包 (unwrapping) 的情况下转换其中包含的值，将嵌套的 `if/else` 逻辑替换为线性的链式调用。

### 快速参考

| **方法** | **作用于** | **功能** | **C++ 等效做法** |
|-----------|-------|---------|-------------------|
| `.map(|v| ...)` | `Option` / `Result` | 转换 `Some`/`Ok` 中的值 | `if (opt) { *opt = transform(*opt); }` |
| `.map_err(|e| ...)` | `Result` | 转换 `Err` 中的值 | 在 catch 块中添加上下文 |
| `.and_then(|v| ...)` | `Option` / `Result` | 链接多个返回 `Option`/`Result` 的操作 | 嵌套的 if 检查 |
| `.find_map(|v| ...)` | 迭代器 | 一次性完成 `find` + `map` 操作 | 带有 `if` + `break` 的循环 |
| `.filter(|v| ...)` | `Option` / 迭代器 | 仅保留符合谓词条件的值 | `if (!predicate) return nullopt;` |
| `.ok()?` | `Result` | 将 `Result` 转为 `Option` 并向上传递 `None` | `if (result.has_error()) return nullopt;` |

---

### 真实示例：用于提取 JSON 字段的 `.and_then()` 链
```rust
// 示例：framework.rs —— 带有回退机制的序列号查找
let sys_info = json.get("SystemInfo")?;

// 首先尝试从 BaseboardFru.BoardSerialNumber 中获取
if let Some(serial) = sys_info
    .get("BaseboardFru")
    .and_then(|b| b.get("BoardSerialNumber"))
    .and_then(|v| v.as_str())
    .filter(valid_serial)     // 仅接受非空且有效的序列号
{
    return Some(serial.to_string());
}

// 如果获取不到，则回退到 BoardFru.SerialNumber
sys_info
    .get("BoardFru")
    .and_then(|b| b.get("SerialNumber"))
    .and_then(|v| v.as_str())
    .filter(valid_serial)
    .map(|s| s.to_string())   // 仅在为 Some 时才进行 &str -> String 转换
```
在 C++ 中，这会导致形成一个缩进极深的结构：`if (json.contains("BaseboardFru")) { if (json["BaseboardFru"].contains("BoardSerialNumber")) { ... } }`。

---

### 真实示例：`find_map` —— 在单次遍历中完成 查找+转换
```rust
// 示例：context.rs —— 查找匹配特定传感器编号及所有者 ID 的 SDR 记录
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
`find_map` 是 `find` 与 `map` 的融合体：它会在发现第一个匹配项时停止遍历并执行转换。其 C++ 的等效做法通常是一个带有 `if` 判断以及 `break` 跳出的 `for` 循环。

### 真实示例：为错误添加上下文的 `map_err` 
```rust
// 示例：main.rs —— 在传播错误之前，为错误添加详细的上下文信息
let json_str = serde_json::to_string_pretty(&config)
    .map_err(|e| format!("序列化配置信息失败: {}", e))?;
```
该操作将 `serde_json::Error` 转为了一段描述性的 `String` 错误消息，其中涵盖了导致故障的具体上下文。

---

## JSON 处理：从 nlohmann::json 到 serde

- C++ 团队通常使用 `nlohmann::json` 进行 JSON 解析。而 Rust 使用 **serde** + **serde_json** —— 其功能更加强大，因为 JSON 架构 (Schema) 是直接编码在类型系统中的。

### C++ (nlohmann) 与 Rust (serde) 的对比

```cpp
// C++ 使用 nlohmann::json —— 运行时字段访问
#include <nlohmann/json.hpp>
using json = nlohmann::json;

struct Fan {
    std::string logical_id;
    std::vector<std::string> sensor_ids;
};

Fan parse_fan(const json& j) {
    Fan f;
    f.logical_id = j.at("LogicalID").get<std::string>();    // 如果缺失则抛出异常
    if (j.contains("SDRSensorIdHexes")) {                   // 手动处理默认值
        f.sensor_ids = j["SDRSensorIdHexes"].get<std::vector<std::string>>();
    }
    return f;
}
```

```rust
// Rust 使用 serde —— 编译期架构，自动字段映射
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Fan {
    pub logical_id: String,
    #[serde(rename = "SDRSensorIdHexes", default)]  // 将 JSON 键映射至 Rust 字段
    pub sensor_ids: Vec<String>,                     // 如果缺失则默认为空 Vec
    #[serde(default)]
    pub sensor_names: Vec<String>,                   // 如果缺失则默认为空 Vec
}

// 仅需一行即可替代整个解析函数：
let fan: Fan = serde_json::from_str(json_str)?;
```

---

### 常见的 serde 属性（摘自生产环境 Rust 代码示例）

| **属性** | **用途** | **C++ 等效做法** |
|--------------|------------|--------------------|
| `#[serde(default)]` | 字段缺失时使用 `Default::default()` | `if (j.contains(key)) { ... } else { default; }` |
| `#[serde(rename = "Key")]` | 将 JSON 键名映射至 Rust 字段名 | 手动访问 `j.at("Key")` |
| `#[serde(flatten)]` | 将未知的键吸收进 `HashMap` 中 | `for (auto& [k,v] : j.items()) { ... }` |
| `#[serde(skip)]` | 不参与序列化/反序列化该字段 | 不将其存储在 JSON 中 |
| `#[serde(tag = "type")]` | 内部标记型枚举（判别式字段） | `if (j["type"] == "gpu") { ... }` |

---

### 真实示例：完整的配置结构体
```rust
// 示例：diag.rs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiagConfig {
    pub sku: SkuConfig,
    #[serde(default)]
    pub level: DiagLevel,            // 字段缺失 → 使用 DiagLevel::default()
    #[serde(default)]
    pub modules: ModuleConfig,       // 字段缺失 → 使用 ModuleConfig::default()
    #[serde(default)]
    pub output_dir: String,          // 字段缺失 → 使用 ""
    #[serde(default, flatten)]
    pub options: HashMap<String, serde_json::Value>,  // 吸收所有未知的键
}

// 加载逻辑仅需 3 行（对比 C++ 下使用 nlohmann 约需 20 多行）：
let content = std::fs::read_to_string(path)?;
let config: DiagConfig = serde_json::from_str(&content)?;
Ok(config)
```

### 使用 `#[serde(tag = "type")]` 反序列化枚举
```rust
// 示例：components.rs
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]                   // JSON 格式示例：{"type": "Gpu", "product": ...}
pub enum PcieDeviceKind {
    Gpu { product: GpuProduct, manufacturer: GpuManufacturer },
    Nic { product: NicProduct, manufacturer: NicManufacturer },
    NvmeDrive { drive_type: StorageDriveType, capacity_gb: u32 },
    // ... 还有 9 个以上的变体
}
// serde 会自动根据 "type" 字段的值分派反序列化逻辑 —— 无需编写手动 if/else 链
```
其 C++ 等效做法通常是：`if (j["type"] == "Gpu") { parse_gpu(j); } else if (j["type"] == "Nic") { parse_nic(j); } ...`

---

# 练习：使用 serde 进行 JSON 反序列化

- 定义一个 `ServerConfig` 结构体，使其能够从如下 JSON 中反序列化：
```json
{
    "hostname": "diag-node-01",
    "port": 8080,
    "debug": true,
    "modules": ["accel_diag", "nic_diag", "cpu_diag"]
}
```
- 使用 `#[derive(Deserialize)]` 和 `serde_json::from_str()` 对其进行解析。
- 为 `debug` 字段添加 `#[serde(default)]` 属性，使其在字段缺失时默认为 `false`。
- **加分项**：添加一个具有 `#[serde(default)]` 属性的 `enum DiagLevel { Quick, Full, Extended }` 字段，使其默认值为 `Quick`。

**起始代码**（需要先执行 `cargo add serde --features derive` 和 `cargo add serde_json`）：
```rust
use serde::Deserialize;

// TODO: 定义 DiagLevel 枚举并实现 Default Trait

// TODO: 定义带有 serde 属性的 ServerConfig 结构体

fn main() {
    let json_input = r#"{
        "hostname": "diag-node-01",
        "port": 8080,
        "debug": true,
        "modules": ["accel_diag", "nic_diag", "cpu_diag"]
    }"#;

    // TODO: 执行反序列化并打印配置信息
    // TODO: 尝试解析缺失 "debug" 字段的 JSON —— 验证其默认值是否为 false
}
```

---

<details><summary>答案 (点击展开)</summary>

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
    #[serde(default)]       // 若字段缺失，则默认为 false
    debug: bool,
    modules: Vec<String>,
    #[serde(default)]       // 若字段缺失，则默认为 DiagLevel::Quick
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
        .expect("解析 JSON 失败");
    println!("{config:#?}");

    // 测试缺失可选字段的情况
    let minimal = r#"{
        "hostname": "node-02",
        "port": 9090,
        "modules": []
    }"#;
    let config2: ServerConfig = serde_json::from_str(minimal)
        .expect("解析最简 JSON 失败");
    println!("debug (默认值): {}", config2.debug);    // false
    println!("level (默认值): {:?}", config2.level);  // Quick
}
// 输出示例：
// ServerConfig {
//     hostname: "diag-node-01",
//     port: 8080,
//     debug: true,
//     modules: ["accel_diag", "nic_diag", "cpu_diag"],
//     level: Quick,
// }
// debug (默认值): false
// level (默认值): Quick
```

</details>

---
