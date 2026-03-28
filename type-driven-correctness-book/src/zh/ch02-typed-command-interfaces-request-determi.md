# 类型化命令接口 — 请求决定响应 🟡

> **你将学到：** 命令 trait 上的关联类型 (associated types) 如何在请求与响应之间创建编译期绑定，从而在 IPMI、Redfish 和 NVMe 协议中消除解析错误、单位混淆以及不合法的类型强制转换。
>
> **相关章节：** [ch01](ch01-the-philosophy-why-types-beat-tests.md)（理念）、[ch06](ch06-dimensional-analysis-making-the-compiler.md)（量纲类型）、[ch07](ch07-validated-boundaries-parse-dont-validate.md)（已验证边界）、[ch10](ch10-putting-it-all-together-a-complete-diagn.md)（集成）

## 非类型化的泥潭

大多数硬件管理栈开始时都是 `原始字节输入 → 原始字节输出`。这会导致解析错误、缩放问题以及单位混淆，且通常只有在生产环境中才能发现。

## 类型化命令模式

### 第 1 步 — 领域 Newtype

```rust
pub struct Celsius(pub f64);
pub struct Rpm(pub u32);
pub struct Volts(pub f64);
```

### 第 2 步 — 命令 Trait

关联类型 `Response` 是核心 —— 它将每个命令 struct 绑定到其特定的返回类型。

```rust
pub trait IpmiCmd {
    type Response;
    fn net_fn(&self) -> u8;
    fn cmd_byte(&self) -> u8;
    fn payload(&self) -> Vec<u8>;
    fn parse_response(&self, raw: &[u8]) -> io::Result<Self::Response>;
}
```

### 第 3 步 — 命令实现

每个命令 struct 自行定义其响应类型和解析逻辑。

```rust
pub struct ReadTemp { pub sensor_id: u8 }
impl IpmiCmd for ReadTemp {
    type Response = Celsius;
    fn parse_response(&self, raw: &[u8]) -> io::Result<Celsius> {
        Ok(Celsius(raw[0] as f64))
    }
    // ...
}
```

### 第 4 步 — 执行器

```rust
impl BmcConnection {
    pub fn execute<C: IpmiCmd>(&self, cmd: &C) -> io::Result<C::Response> {
        let raw = self.raw_send(cmd.net_fn(), cmd.cmd_byte(), &cmd.payload())?;
        cmd.parse_response(&raw)
    }
}
```

## 模式族

该模式适用于几乎所有的硬件协议：

| 协议 | 请求类型 | 响应类型 |
|----------|-------------|---------------|
| IPMI | `ReadTemp` | `Celsius` |
| Redfish | `GetThermal` | `ThermalResponse` |
| NVMe Admin | `Identify` | `IdentifyResponse` |
| PLDM | `GetFwParams` | `FwParamsResponse` |

## 关键收获

1. **关联类型即编译期契约** —— 将请求与响应牢牢锁定。
2. **封装化的解析** —— 解析逻辑与命令定义紧密结合。
3. **零成本派发** —— 泛型调用会被单态化。
4. **通用模式** —— 契合 IPMI、Redfish、NVMe 以及更多场景。

***
