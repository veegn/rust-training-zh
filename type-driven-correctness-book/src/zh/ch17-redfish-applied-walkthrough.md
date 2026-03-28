[English Original](../en/ch17-redfish-applied-walkthrough.md)

# Redfish 客户端实战讲解 🟡

> **你将学到什么？** 如何将类型驱动的理念应用到真实世界中的 REST API（如 Redfish、IPMI 或系统监控接口）；如何利用 Rust 的 `serde` 特征实现类型安全的数据解析；如何利用 Newtype 处理物理量；以及为什么这相比于通用的 JSON 全局解析，能彻底防止在 Redfish 交互中的逻辑错误。

## 引言：通用的、脆弱的 REST 交互

在处理复杂的 REST API 时，一个常见的问题是：为了一致性，通常使用通用的 JSON 对象来解析和处理返回。

考虑一个典型的、脆弱的通用 REST 模型：

```rust
struct RedfishEvent {
    id: String,
    data: serde_json::Value,
}

fn handle_event(event: RedfishEvent) {
    if event.id != "Thermal" {
        panic!("不支持此事件！");
    }
    // 处理动态 Data
}
```

这种方案的问题在于：你的 JSON 全局解析在所有事件中都是一样的，你依然依赖程序运行到那一行逻辑才报错。

## Rust 方案：具有语义的结构化解析

我们可以利用 `serde` 声明每一个特定 Redfish 事件的具体字段。

### 1. 为特定事件定义结构体

```rust
#[derive(Deserialize)]
pub struct ThermalEvent {
    pub temperature_c: f64,
}
```

### 2. 通过 Newtype 处理物理量

```rust
pub struct Celsius(pub f64);
```

### 3. 类型化的反序列化

```rust
fn handle_thermal(data: ThermalEvent) {
    // 逻辑
}
```

通过指定结构体而不是 `serde_json::Value`：
- 反序列化失败（如数据缺失或格式错误）直接在边界处以 `Err` 抛出；
- 后继逻辑可以假定所有的字段都是合法的。

## 现实应用：Redfish 监控与事件分发

在大型监控系统中使用 `enum` 和 `serde` 支持多种事件类型：
- 当你尝试把一个“机架开启”事件当做“电源过流”事件处理时，编译器会指出你的选择是错误的。

## 为什么这种模式至关重要

1.  **排除无效逻辑 (No Invalid Logic)**：解析失败在边界处就被排除了，核心逻辑始终可以操作“合法”的数据结构。
2.  **强制遵守 Redfish 模式 (Enforced Specs)**：通过定义具体结构体表达物理规格和字段依赖关系。
3.  **高性能 (High Performance)**：复杂的 JSON 校验只在解析时发生一次；生成的汇编代码不再包含任何额外的字段存在性校验逻辑。

在为具有多种访问级别或不同规格的复杂分布式系统编写统一维护框架时，这种模式可以有效防御缓冲区溢出和逻辑漏洞。

***
