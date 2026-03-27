## Avoiding unchecked indexing / 避免未检查的索引

> **What you'll learn / 你将学到：** Why `vec[i]` is dangerous in Rust (panics on out-of-bounds), and safe alternatives like `.get()`, iterators, and `entry()` API for `HashMap`. Replaces C++'s undefined behavior with explicit handling.
>
> 为什么 `vec[i]` 在 Rust 中是危险的（越界时会发生 panic），以及像 `.get()`、迭代器和 `HashMap` 的 `entry()` API 这样的安全替代方案。我们将用显式处理来替代 C++ 中的未定义行为。

*In C++, `vec[i]` and `map[key]` have undefined behavior or auto-insert on missing keys. Rust's `[]` panics on out-of-bounds. **Rule**: Use `.get()` instead of `[]` unless you can *prove* the index is valid.*

在 C++ 中，`vec[i]` 和 `map[key]` 在索引越界或键缺失时会产生未定义行为或自动插入。而 Rust 的 `[]` 在越界时会直接 panic。**规则**：除非你能**证明**索引是有效的，否则请使用 `.get()` 代替 `[]`。

---

### C++ → Rust comparison / C++ vs Rust 对比

```cpp
// C++ — silent UB or insertion / 静默的未定义行为或插入
std::vector<int> v = {1, 2, 3};
int x = v[10];        // UB! No bounds check / 未定义行为！operator[] 不执行边界检查

std::map<std::string, int> m;
int y = m["missing"]; // Auto-insert / 静默插入键并赋值为 0！
```

```rust
// Rust — safe alternatives / 安全替代方案
let v = vec![1, 2, 3];

// Bad / 坏习惯：如果索引越界会发生 panic
// let x = v[10];

// Good / 好习惯：返回 Option<&i32>
let x = v.get(10);              // None / 返回 None —— 不会 panic
let x = v.get(1).copied().unwrap_or(0);  // 结果为 2，如果缺失则为 0
```

### Real example: safe byte parsing / 真实示例：安全字节解析
```rust
// Example: diagnostics.rs — parsing binary SEL record
// 解析二进制 SEL 记录 —— 缓冲区可能比预期的短
let sensor_num = bytes.get(7).copied().unwrap_or(0);
let ppin = cpu_ppin.get(i).map(|s| s.as_str()).unwrap_or("");
```

### Real example: chained safe lookups / 真实示例：链式安全查找
```rust
// Example: profile.rs — double lookup: HashMap → Vec
pub fn get_processor(&self, location: &str) -> Option<&Processor> {
    self.processor_by_location
        .get(location)                              // HashMap → Option<&usize>
        .and_then(|&idx| self.processors.get(idx))   // Vec → Option<&Processor>
}
// Both return Option / 两次查找均返回 Option —— 无 panic，无未定义行为
```

---

## Safe value extraction / 使用 unwrap_or 进行安全值提取

*`unwrap()` panics on `None` / `Err`. In production code, prefer the safe alternatives.*

`unwrap()` 在遇到 `None` 或 `Err` 时会发生 panic。在生产代码中，请优先选择安全的替代方案。

| **Method / 方法** | **Behavior on None/Err / 失败时的行为** | **Use When / 适用场景** |
|-----------|------------------------|-------------|
| `.unwrap()` | **Panics** / 发生 Panic | Tests / 仅限测试，或被证明绝不会失败 |
| `.expect("msg")` | Panic with msg / 带消息 Panic | 理由充分时，并解释原因 |
| `.unwrap_or(default)` | Returns default / 返回默认值 | 拥有廉价的常量备选值时 |
| `.unwrap_or_else(...)` | Calls closure / 调用闭包 | 备选值的计算开销较大时 |
| `.unwrap_or_default()` | Returns Default / 返回默认值 | 类型实现了 `Default` 时 |

---

## Functional transforms: map, and_then / 函数式转换：map、and_then

*These methods on `Option` and `Result` let you transform the contained value without unwrapping, replacing nested `if/else` with linear chains.*

`Option` 和 `Result` 上的这些方法允许你在不解包的情况下转换其中的值，从而将嵌套的 `if/else` 替换为线性链式调用。

| **Method / 方法** | **On / 作用于** | **Does / 功能** | **C++ Equivalent / C++ 等效物** |
|-----------|-------|---------|-------------------|
| `.map(...)` | `Option` / `Result` | Transform / 转换内部值 | `if (opt) { *opt = ...; }` |
| `.map_err(...)` | `Result` | Transform Err / 转换错误值 | 为 catch 块添加上下文 |
| `.and_then(...)` | `Option` / `Result` | Chain fallible ops / 链式调用可能失败的操作 | 嵌套的 if 检查 |
| `.find_map(...)` | Iterator | find + map / 一次完成查找与转换 | 带 if + break 的循环 |
| `.ok()?` | `Result` | Result → Option / 错误转为 None | 将错误处理转换为 Option 处理 |

---

## JSON handling: nlohmann::json → serde / JSON 处理：nlohmann::json 至 serde 的演进

*C++ teams typically use `nlohmann::json`. Rust uses **serde** + **serde_json** — which is more powerful because the JSON schema is encoded *in the type system*.*

C++ 团队通常使用 `nlohmann::json` 进行 JSON 解析。Rust 则使用 **serde** + **serde_json** —— 这更强大，因为 JSON 模式（Schema）是编码在**类型系统**中的。

```rust
// Rust with serde — compile-time schema, automatic field mapping
// Rust 使用 serde — 编译时模式，自动字段映射
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Fan {
    pub logical_id: String,
    #[serde(rename = "SDRSensorIdHexes", default)]  // Mapping / 映射键名并提供默认值
    pub sensor_ids: Vec<String>,                     // Missing -> empty / 缺失则为空 Vec
    #[serde(default)]
    pub sensor_names: Vec<String>,                   // Missing -> empty / 缺失则为空 Vec
}

// One line replaces parse / 一行代码即可替代整个解析函数：
let fan: Fan = serde_json::from_str(json_str)?;
```

---

# Exercise: JSON deserialization with serde / 练习：使用 serde 进行 JSON 反序列化

Define a `ServerConfig` struct that can be deserialized from the following JSON:
```json
{
    "hostname": "diag-node-01",
    "port": 8080,
    "debug": true,
    "modules": ["accel_diag", "nic_diag", "cpu_diag"]
}
```
**Requirements / 需求：**
- Use `#[derive(Deserialize)]` and `serde_json::from_str()` to parse it.
- Add `#[serde(default)]` to `debug` so it defaults to `false` if missing.
- **Bonus**: Add an `enum DiagLevel { Quick, Full, Extended }` field with `#[serde(default)]`.

<details><summary>Solution / 解决方案（点击展开）</summary>

```rust
use serde::Deserialize;

#[derive(Debug, Deserialize, Default)]
enum DiagLevel {
    #[default] Quick, Full, Extended,
}

#[derive(Debug, Deserialize)]
struct ServerConfig {
    hostname: String,
    port: u16,
    #[serde(default)]       // Missing -> false / 缺失则默认为 false
    debug: bool,
    modules: Vec<String>,
    #[serde(default)]       // Missing -> Quick / 缺失则默认为 Quick
    level: DiagLevel,
}

fn main() {
    let json_input = r#"{"hostname": "diag-node-01", "port": 8080, "modules": []}"#;
    let config: ServerConfig = serde_json::from_str(json_input).unwrap();
    println!("Level: {:?}", config.level); // Quick
}
```
</details>
