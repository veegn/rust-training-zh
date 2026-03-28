# 已验证边界 — 解析，而非验证 🟡

> **你将学到：** 如何在系统边界处精确验证一次数据，将验证证明携带在专用类型中，并永不重复检查 —— 应用于 IPMI FRU 记录、Redfish JSON 以及包含嵌套派发的 IPMI SEL 记录。
>
> **相关章节：** [ch02](ch02-typed-command-interfaces-request-determi.md)（类型化命令）、[ch06](ch06-dimensional-analysis-making-the-compiler.md)（量纲类型）、[ch11](ch11-fourteen-tricks-from-the-trenches.md)（技巧 2, 3, 5）、[ch14](ch14-testing-type-level-guarantees.md) (proptest)

## 问题：分散验证 (Shotgun Validation)

在许多代码库中，验证逻辑散布在接收数据的每个函数中。这会导致代码冗余，并且如果某个函数忘记检查，就会出现逻辑漏洞或崩溃。

## 解析，而非验证 (Parse, Don't Validate)

“构造即正确”的方法是：**在边界处进行一次性验证，然后将验证证明携带在类型中。**

### IPMI FRU 数据案例

```rust
pub struct ValidFru {
    format_version: u8,
    internal_area_offset: u8,
    chassis_area_offset: u8,
    board_area_offset: u8,
    product_area_offset: u8,
    data: Vec<u8>,
}

impl TryFrom<RawFruData> for ValidFru {
    type Error = FruError;

    fn try_from(raw: RawFruData) -> Result<Self, FruError> {
        let data = raw.0;
        // 1. 长度检查
        // 2. 格式版本检查
        // 3. 校验和验证
        // 4. 不同偏移量是否越界
        // ...
        Ok(ValidFru { ... })
    }
}
```

一旦你持有 `ValidFru`，所有下游函数都可以确定数据是格式正确的，无需再次检查。

## 已验证的 Redfish JSON

解析 Redfish 响应为类型化结构，可以确保所有必需字段均存在且在合法范围内。

```rust
pub struct ValidThermalResponse {
    pub temperatures: Vec<ValidTemperatureReading>,
    pub fans: Vec<ValidFanReading>,
}
```

## 多态验证：IPMI SEL 记录

IPMI 系统事件日志 (SEL) 记录是固定 16 字节但具备多态性的。其含义会根据“记录类型”、“事件类型”和“传感器类型”而发生变化。

我们通过嵌套枚举 (nested enums) 来映射 spec 中的派发层级：

```rust
pub enum ValidSelRecord {
    SystemEvent(SystemEventRecord),
    OemTimestamped(OemTimestampedRecord),
    OemNonTimestamped(OemNonTimestampedRecord),
}

pub enum TypedEvent {
    Threshold(ThresholdEvent),
    SensorSpecific(SensorSpecificEvent),
    Discrete { offset: u8, event_data: [u8; 3] },
}
```

## 边界验证的优势

1. **消除冗余** —— 检查仅发生一次。
2. **穷尽处理** —— 枚举迫使你处理所有可能的情况（例如全部 42 种类型的 IPMI 传感器类型）。
3. **内部安全** —— 接收已验证类型的函数可以安全地使用 `unwrap()` 或直接进行索引。

## 关键收获

1. **边界即过滤器** —— 数据入口是唯一的验证点。
2. **类型即证明** —— 看到 `ValidFru` 就是对校验和以及边界已检查过的证明。
3. **枚举处理多态性** —— 在类型系统中映射 spec 的层级结构，避免由于分支缺失导致的静默错误。
4. **代码更整洁** —— 不再需要在各处编写防御性的 `if` 语句。

***
