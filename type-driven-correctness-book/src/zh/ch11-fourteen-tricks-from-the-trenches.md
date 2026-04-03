[English Original](../en/ch11-fourteen-tricks-from-the-trenches.md)

# 来自实战的 14 个小技巧 🟡

> **你将学到：**
> - 14 种具体的“正确构建”小技巧 —— 从消除哨兵值、密封特性到会话类型、`Pin`、RAII 以及 `#[must_use]`。
> - 每一项技巧都能以近乎零成本的方式消除一类特定的 Bug。

> **参考：** [第 2 章](ch02-typed-command-interfaces-request-determi.md)（密封特性扩展了第 2 章的内容）、[第 5 章](ch05-protocol-state-machines-type-state-for-r.md)（类型状态构建器扩展了第 5 章的内容）、[第 7 章](ch07-validated-boundaries-parse-dont-validate.md)（FromStr 扩展了第 7 章的内容）。

## 来自实战的 14 个小技巧

第 2 章至第 9 章介绍的八种核心模式涵盖了主要的“正确构建”技术。本章则收集了 14 个在生产级 Rust 代码中反复出现的 **虽然较小但价值很高的小技巧** —— 它们每一个都能以零或近乎零的成本消除一类特定的 Bug。

### 技巧 1 —— 在边界处将“哨兵值”映射为 `Option`

硬件协议中充斥着“哨兵值 (Sentinel Values)”：IPMI 使用 `0xFF` 表示“传感器不存在”，PCI 使用 `0xFFFF` 表示“没有设备”，SMBIOS 使用 `0x00` 表示“未知”。如果你在代码中一直将这些哨兵值作为普通整数传递，那么每个消费者都必须记住去检查那个魔数。即使只有一次比较忘记了检查，你也会得到一个 255°C 的幻影读数，或者一个伪造的厂商 ID 匹配。

**规则：** 在最外层的解析边界处就将哨兵值转换为 `Option`，只有在最终的序列化边界处才将其转换 *回* 哨兵值。

#### 反模式 (来自 `pcie_tree/src/lspci.rs`)

```rust,ignore
// 内部携带了哨兵值 —— 每次比较都必须记住它
let mut current_vendor_id: u16 = 0xFFFF;
let mut current_device_id: u16 = 0xFFFF;

// ... 稍后，解析在静默状态下失败了 ...
current_vendor_id = u16::from_str_radix(hex, 16)
    .unwrap_or(0xFFFF);  // 哨兵值隐藏了错误
```

每个接收 `current_vendor_id` 的函数都必须知道 `0xFFFF` 是特殊的。如果有人在没先检查 `0xFFFF` 的情况下写了 `if vendor_id == target_id`，那么当目标 ID 碰巧也因为错误的输入被解析为 `0xFFFF` 时，一个缺失的设备就会在静默状态下发生匹配。

#### 正确模式 (来自 `nic_sel/src/events.rs`)

```rust,ignore
pub struct ThermalEvent {
    pub record_id: u16,
    pub temperature: Option<u8>,  // 如果传感器报告 0xFF，则为 None
}

impl ThermalEvent {
    pub fn from_raw(record_id: u16, raw_temp: u8) -> Self {
        ThermalEvent {
            record_id,
            temperature: if raw_temp != 0xFF {
                Some(raw_temp)
            } else {
                None
            },
        }
    }
}
```

现在，每个消费者都 **必须** 处理 `None` 的情况 —— 这是编译器强制执行的：

```rust,ignore
// 安全 —— 编译器确保我们处理了温度缺失的情况
fn is_overtemp(temp: Option<u8>, threshold: u8) -> bool {
    temp.map_or(false, |t| t > threshold)
}

// 忘记处理 None 会导致编译错误：
// fn bad_check(temp: Option<u8>, threshold: u8) -> bool {
//     temp > threshold  // 错误：无法将 Option<u8> 与 u8 进行比较
// }
```

#### 现实世界的影响

`inventory/src/events.rs` 在 GPU 温度告警中使用了同样的模式：
```rust,ignore
temperature: if data[1] != 0xFF {
    Some(data[1] as i8)
} else {
    None
},
```

对 `pcie_tree/src/lspci.rs` 的重构非常简单：将 `current_vendor_id: u16` 改为 `current_vendor_id: Option<u16>`，用 `None` 替换 `0xFFFF`，然后让编译器找到每一个需要更新的地方。

| 重构前 | 重构后 |
|--------|-------|
| `let mut vendor_id: u16 = 0xFFFF` | `let mut vendor_id: Option<u16> = None` |
| `.unwrap_or(0xFFFF)` | `.ok()` (本身就返回 `Option`) |
| `if vendor_id != 0xFFFF { ... }` | `if let Some(vid) = vendor_id { ... }` |
| 序列化：`vendor_id` | `vendor_id.unwrap_or(0xFFFF)` |

***

### 技巧 2 —— 密封特性 (Sealed Traits)

第 2 章介绍了 `IpmiCmd`，它带有一个关联类型，将每个命令与其响应绑定。但这里有一个漏洞：如果 *任何* 代码都能实现 `IpmiCmd`，那么有人就可能写出一个 `MaliciousCmd`，其 `parse_response` 返回错误的类型或者直接发生 panic。整个系统的类型安全性都建立在每一个实现都是正确的基础之上。

**密封特性 (Sealed Trait)** 关闭了这个漏洞。其背后的思想很简单：让特性要求一个 *私有* 的父特性 (Supertrait)，而该父特性只有在你自己的 crate 中才能被实现。

```rust,ignore
// — 私有模块：不从 crate 中导出 —
mod private {
    pub trait Sealed {}
}

// — 公有特性：要求实现 Sealed，而外部无法实现该私有特性 —
pub trait IpmiCmd: private::Sealed {
    type Response;
    fn net_fn(&self) -> u8;
    fn cmd_byte(&self) -> u8;
    fn payload(&self) -> Vec<u8>;
    fn parse_response(&self, raw: &[u8]) -> io::Result<Self::Response>;
}
```

在你自己的 crate 内部，你为每一个经过批准的命令类型实现 `Sealed`：

```rust,ignore
pub struct ReadTemp { pub sensor_id: u8 }
impl private::Sealed for ReadTemp {}

impl IpmiCmd for ReadTemp {
    type Response = Celsius;
    fn net_fn(&self) -> u8 { 0x04 }
    fn cmd_byte(&self) -> u8 { 0x2D }
    fn payload(&self) -> Vec<u8> { vec![self.sensor_id] }
    fn parse_response(&self, raw: &[u8]) -> io::Result<Celsius> {
        if raw.is_empty() { return Err(io::Error::new(io::ErrorKind::InvalidData, "empty")); }
        Ok(Celsius(raw[0] as f64))
    }
}
```

外部代码可以看到 `IpmiCmd` 并能调用 `execute()`，但无法实现它：

```rust,ignore
// 在另一个 crate 中：
struct EvilCmd;
// impl private::Sealed for EvilCmd {}  // 错误：模块 `private` 是私有的
// impl IpmiCmd for EvilCmd { ... }     // 错误：不满足 `Sealed` 约束
```

#### 何时进行密封

| 在以下情况密封…… | 在以下情况不要密封…… |
|-----------|-----------------|
| 安全性依赖于正确的实现 (IpmiCmd, DiagModule) | 用户应当能扩展系统 (自定义报告格式化器) |
| 关联类型必须满足某些不变式 | 特性只是一个简单的能力标记 (HasIpmi) |
| 你拥有规范的实现集合 | 第三方插件是一个设计目标 |

#### 现实世界中的候选对象

- `IpmiCmd` —— 错误的解析可能会破坏类型化响应。
- `DiagModule` —— 框架假设 `run()` 会返回有效的 DER 记录。
- `SelEventFilter` —— 损坏的过滤器可能会漏掉关键的 SEL 事件。

***

### 技巧 3 —— 使用 `#[non_exhaustive]` 处理不断演进的枚举

`inventory/src/types.rs` 中的 `SkuVariant` 目前有五个变体：

```rust,ignore
pub enum SkuVariant {
    S1001, S2001, S2002, S2003, S3001,
}
```

当下一代产品发布并增加 `S4001` 时，任何外部代码如果对 `SkuVariant` 进行匹配且没有通配符分支 (wildcard arm)，都会 **由于无法编译而静默失败** —— 这正是重点所在。但内部代码呢？如果没有 `#[non_exhaustive]`，你在 *同一个 crate* 中的 `match` 可以在没写通配符的情况下通过编译，而增加新变体就会导致你自己的构建失败。

将枚举标记为 `#[non_exhaustive]` 会强制 **外部 crate** 在对其进行匹配时必须包含通配符分支。而在定义该枚举的 crate 内部，`#[non_exhaustive]` 不起作用 —— 你依然可以编写穷尽匹配 (exhaustive matches)。

**为什么这很有用：** 当你从一个库 crate（或工作区中的共享子 crate）导出 `SkuVariant` 时，下游代码会被迫处理未来未知的变体。当下个世代增加 `S4001` 时，下游代码依然可以通过编译 —— 因为它们已经有了通配符分支。

```rust,ignore
// 在 gpu_sel crate 中 (定义所在的 crate)：
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SkuVariant {
    S1001,
    S2001,
    S2002,
    S2003,
    S3001,
    // 当下一个 SKU 发布时，在此处添加。
    // 外部消费者已经有了通配符分支 —— 对他们来说零破坏。
}

// 在 gpu_sel 内部 —— 允许穷尽匹配 (无需通配符)：
fn diag_path_internal(sku: SkuVariant) -> &'static str {
    match sku {
        SkuVariant::S1001 => "legacy_gen1",
        SkuVariant::S2001 => "gen2_accel_diag",
        SkuVariant::S2002 => "gen2_alt_diag",
        SkuVariant::S2003 => "gen2_alt_hf_diag",
        SkuVariant::S3001 => "gen3_accel_diag",
        // 在定义该枚举的 crate 内部无需通配符。
        // 在此处增加 S4001 会导致此 match 出现编译错误，
        // 这正是你想要的 —— 它会迫使你更新后续逻辑。
    }
}
```

```rust,ignore
// 在二进制 crate 中 (依赖 inventory 的下游 crate)：
fn diag_path_external(sku: inventory::SkuVariant) -> &'static str {
    match sku {
        inventory::SkuVariant::S1001 => "legacy_gen1",
        inventory::SkuVariant::S2001 => "gen2_accel_diag",
        inventory::SkuVariant::S2002 => "gen2_alt_diag",
        inventory::SkuVariant::S2003 => "gen2_alt_hf_diag",
        inventory::SkuVariant::S3001 => "gen3_accel_diag",
        _ => "generic_diag",  // 对于外部 crate，#[non_exhaustive] 要求必须有此分支
    }
}
```

> **工作区 (Workspace) 提示：** 如果你所有的代码都在同一个 crate 中，`#[non_exhaustive]` 就起不到作用 —— 它只影响跨 crate 边界的情况。对于本项目的大型工作区，请将不断演进的枚举放在共享 crate（如 `core_lib` 或 `inventory`）中，这样该属性就能保护其他工作区 crate 中的消费者。

#### 候选对象

| 枚举 | 模块 | 原因 |
|------|--------|-----|
| `SkuVariant` | `inventory`, `net_inventory` | 每一代都会有新的 SKU |
| `SensorType` | `protocol_lib` | IPMI 规范将 0xC0–0xFF 留给 OEM 扩展 |
| `CompletionCode` | `protocol_lib` | 自定义 BMC 厂商会增加特有的返回码 |
| `Component` | `event_handler` | 新的硬件类别 (最近刚增加了 NewSoC) |

***

### 技巧 4 —— 类型状态构建器 (Typestate Builder)

第 5 章展示了针对 *协议* 的类型状态（会话生命周期、链路训练）。同样的想法也适用于 *构建器 (Builders)* —— 这些结构体的 `build()` / `finish()` 方法只有在所有必填字段都已设置时才能被调用。

#### 链式构建器的问题

`diag_framework/src/der.rs` 中的 `DerBuilder` 目前的代码如下（简化版）：

```rust,ignore
// 当前的链式构建器 —— finish() 总是可用
pub struct DerBuilder {
    der: Der,
}

impl DerBuilder {
    pub fn new(marker: &str, fault_code: u32) -> Self { ... }
    pub fn mnemonic(mut self, m: &str) -> Self { ... }
    pub fn fault_class(mut self, fc: &str) -> Self { ... }
    pub fn finish(self) -> Der { self.der }  // ← 总是可以调用！
}
```

这可以编译通过，但会产生一个不完整的 DER 记录：

```rust,ignore
let bad = DerBuilder::new("CSI_ERR", 62691)
    .finish();  // 漏掉了 mnemonic 和 fault_class
```

#### 类型状态构建器：`finish()` 要求两个字段都必须已设置

```rust,ignore
pub struct Missing;
pub struct Set<T>(T);

pub struct DerBuilder<Mnemonic, FaultClass> {
    marker: String,
    fault_code: u32,
    mnemonic: Mnemonic,
    fault_class: FaultClass,
    description: Option<String>,
}

// 构造函数：启动时两个必填字段均为 Missing
impl DerBuilder<Missing, Missing> {
    pub fn new(marker: &str, fault_code: u32) -> Self {
        DerBuilder {
            marker: marker.to_string(),
            fault_code,
            mnemonic: Missing,
            fault_class: Missing,
            description: None,
        }
    }
}

// 设置助记符 mnemonic (无论 fault_class 的状态如何都可用)
impl<FC> DerBuilder<Missing, FC> {
    pub fn mnemonic(self, m: &str) -> DerBuilder<Set<String>, FC> {
        DerBuilder {
            marker: self.marker, fault_code: self.fault_code,
            mnemonic: Set(m.to_string()),
            fault_class: self.fault_class,
            description: self.description,
        }
    }
}

// 设置错误类别 fault_class (无论 mnemonic 的状态如何都可用)
impl<MN> DerBuilder<MN, Missing> {
    pub fn fault_class(self, fc: &str) -> DerBuilder<MN, Set<String>> {
        DerBuilder {
            marker: self.marker, fault_code: self.fault_code,
            mnemonic: self.mnemonic,
            fault_class: Set(fc.to_string()),
            description: self.description,
        }
    }
}

// 可选字段 —— 在任何状态下均可用
impl<MN, FC> DerBuilder<MN, FC> {
    pub fn description(mut self, desc: &str) -> Self {
        self.description = Some(desc.to_string());
        self
    }
}

/// 已完全构建好的 DER 记录。
pub struct Der {
    pub marker: String,
    pub fault_code: u32,
    pub mnemonic: String,
    pub fault_class: String,
    pub description: Option<String>,
}

// finish() 只有在两个必填字段都标记为 Set 时才可用
impl DerBuilder<Set<String>, Set<String>> {
    pub fn finish(self) -> Der {
        Der {
            marker: self.marker,
            fault_code: self.fault_code,
            mnemonic: self.mnemonic.0,
            fault_class: self.fault_class.0,
            description: self.description,
        }
    }
}
```

现在，那个存在 Bug 的调用会导致编译错误：

```rust,ignore
// ✅ 可编译 —— 所有的两个必填字段都已设置 (顺序无关)
let der = DerBuilder::new("CSI_ERR", 62691)
    .fault_class("GPU 模块")   // 顺序不影响
    .mnemonic("ACCEL_CARD_ER691")
    .description("热过载触发降频")
    .finish();

// ❌ 编译错误 —— DerBuilder<Set<String>, Missing> 上没有 `finish()` 方法
let bad = DerBuilder::new("CSI_ERR", 62691)
    .mnemonic("ACCEL_CARD_ER691")
    .finish();  // 错误：未找到 `finish` 方法
```

#### 何时使用类型状态构建器

| 在以下情况使用…… | 在以下情况不用费事…… |
|-----------|-------------------|
| 遗漏一个字段会导致静默 Bug (如 DER 缺失助记符) | 所有的字段都有合理的默认值 |
| 该构建器是公有 API 的一部分 | 该构建器仅仅是测试用的临时脚手架 |
| 有 2 到 3 个以上的必填字段 | 只有一个必填字段 (直接在 `new()` 中传入即可) |

***

### 技巧 5 —— 将 `FromStr` 作为验证边界

第 7 章介绍了针对二进制数据（FRU 记录、SEL 条目）的 `TryFrom<&[u8]>`。对于 **字符串** 输入 —— 配置文件、CLI 参数、JSON 字段 —— 类似的边界是 `FromStr`。

#### 问题

```rust,ignore
// C++ / 未经验证的 Rust：如果在分支匹配外，就静默地进入默认分支
fn route_diag(level: &str) -> DiagMode {
    if level == "quick" { ... }
    else if level == "standard" { ... }
    else { QuickMode }  // 配置文件里写错了？ ¯\_(ツ)_/¯
}
```

如果配置文件中将 `"diag_level"` 写成了 `"extendedd"` (笔误)，它会自动静默地进入 `QuickMode`。

#### 正确模式 (来自 `config_loader/src/diag.rs`)

```rust,ignore
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiagLevel {
    Quick,
    Standard,
    Extended,
    Stress,
}

impl FromStr for DiagLevel {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "quick"    | "1" => Ok(DiagLevel::Quick),
            "standard" | "2" => Ok(DiagLevel::Standard),
            "extended" | "3" => Ok(DiagLevel::Extended),
            "stress"   | "4" => Ok(DiagLevel::Stress),
            other => Err(format!("未知的诊断级别：'{other}'")),
        }
    }
}
```

现在，笔误会立即被发现：

```rust,ignore
let level: DiagLevel = "extendedd".parse()?;
// 错误：("未知的诊断级别：'extendedd'")
```

#### 三大优势

1. **快速失败：** 错误的输入在解析边界处就被捕获，而不是在后续三层深的代码逻辑中触发异常。
2. **别名显式化：** `"MEM"`、`"DIMM"` 和 `"MEMORY"` 都能映射到 `Component::Memory` —— match 分支记录了这些映射关系。
3. **`.parse()` 非常符合人体工程学：** 由于 `FromStr` 集成了 `str::parse()`，你可以实现清爽的代码：`let level: DiagLevel = config["level"].parse()?;`

#### 在实际代码库中的运用

本项目已经有了 8 个 `FromStr` 实现：

| 类型 | 模块 | 值得注意的别名 |
|------|--------|----------------|
| `DiagLevel` | `config_loader` | `"1"` = Quick, `"4"` = Stress |
| `Component` | `event_handler` | `"MEM"` / `"DIMM"` = Memory, `"SSD"` / `"NVME"` = Disk |
| `SkuVariant` | `net_inventory` | `"Accel-X1"` = S2001, `"Accel-M1"` = S2002, `"Accel-Z1"` = S3001 |
| `SkuVariant` | `inventory` | 同样的别名 (不同的模块，同样的模式) |
| `FaultStatus` | `config_loader` | 故障生命周期状态 |
| `DiagAction` | `config_loader` | 补救动作类型 |
| `ActionType` | `config_loader` | 动作类别 |
| `DiagMode` | `cluster_diag` | 多节点测试模式 |

与 `TryFrom` 的对比：

| | `TryFrom<&[u8]>` | `FromStr` |
|---|---|---|
| 输入 | 原始字节 (二进制协议) | 字符串 (配置、CLI、JSON) |
| 典型来源 | IPMI, PCIe 配置空间, FRU | JSON 字段, 环境变量, 用户输入 |
| 对应章节 | 第 7 章 | 第 11 章 |
| 共同点 | 使用 `Result` —— 迫使调用者处理无效输入 |

***

### 技巧 6 —— 使用常量泛型进行编译时大小验证

当硬件缓冲区、寄存器组或协议帧具有固定大小时，常量泛型 (Const Generics) 能让编译器强制执行这些约束：

```rust,ignore
/// 固定大小的寄存器组。其大小是该类型的一部分。
/// `RegisterBank<256>` 和 `RegisterBank<4096>` 是不同的类型。
pub struct RegisterBank<const N: usize> {
    data: [u8; N],
}

impl<const N: usize> RegisterBank<N> {
    /// 在指定的偏移量读取一个寄存器。
    /// 编译时：N 是已知的，因此数组大小是固定的。
    /// 运行时：仅对偏移量进行检查。
    pub fn read(&self, offset: usize) -> Option<u8> {
        self.data.get(offset).copied()
    }
}

// PCIe 常规配置空间：256 字节
type PciConfigSpace = RegisterBank<256>;

// PCIe 扩展配置空间：4096 字节
type PcieExtConfigSpace = RegisterBank<4096>;

// 它们是不同的类型 —— 你无法不小心把其中一个当成另一个传入：
fn read_extended_cap(config: &PcieExtConfigSpace, offset: usize) -> Option<u8> {
    config.read(offset)
}
// read_extended_cap(&pci_config, 0x100);
//                   ^^^^^^^^^^^ 预期得到 RegisterBank<4096>，实际发现是 RegisterBank<256> ❌
```

**使用常量泛型实现编译时断言：**

```rust,ignore
/// NVMe 管理命令使用 4096 字节的缓冲区。在编译时强制执行。
pub struct NvmeBuffer<const N: usize> {
    data: Box<[u8; N]>,
}

impl<const N: usize> NvmeBuffer<N> {
    pub fn new() -> Self {
        // 运行时断言：仅允许 512 或 4096
        assert!(N == 4096 || N == 512, "NVMe 缓冲区必须是 512 或 4096 字节");
        NvmeBuffer { data: Box::new([0u8; N]) }
    }
}
// NvmeBuffer::<1024>::new();  // 这种形式会在运行时发生 panic
// 想要实现真正的编译时强制执行，请参见技巧 9 (常量断言)。
```

> **何时使用：** 固定大小的协议缓冲区 (NVMe, PCIe 配置空间)、DMA 描述符、硬件 FIFO 深度。任何在硬件层面上被定义为常量、且在运行时永远不应当变动的大小。

***

### 技巧 7 —— 对 `unsafe` 进行安全封装

本项目目前几乎没有 `unsafe` 语句块。但当你需要增加 MMIO 寄存器访问、DMA 或 FFI 时，你就会用到 `unsafe`。正确构建的方法是：**将每一个 `unsafe` 语句块都封装在一个安全的抽象中**，从而使非安全性受到限制且可审计。

```rust,ignore
/// MMIO 映射后的寄存。只要映射有效，该指针就是有效的。
/// 所有的 unsafe 都限制在这个模块内 —— 调用者使用的是安全的方法。
pub struct MmioRegion {
    base: *mut u8,
    len: usize,
}

impl MmioRegion {
    /// # 安全性 (Safety)
    /// - `base` 必须是一个指向 MMIO 映射区域的有效指针。
    /// - 该区域在整个结构体的生命周期内必须保持映射状态。
    /// - 其他任何代码都不能为由于该区域设置别名。
    pub unsafe fn new(base: *mut u8, len: usize) -> Self {
        MmioRegion { base, len }
    }

    /// 安全读取 —— 边界检查能防止越界的 MMIO 访问。
    pub fn read_u32(&self, offset: usize) -> Option<u32> {
        if offset + 4 > self.len { return None; }
        // 安全性 (SAFETY)：offset 已在上方进行了边界检查，base 已由 new() 的契约保证有效
        Some(unsafe {
            core::ptr::read_volatile(self.base.add(offset) as *const u32)
        })
    }

    /// 安全写入 —— 边界检查能防止越界的 MMIO 访问。
    pub fn write_u32(&self, offset: usize, value: u32) -> bool {
        if offset + 4 > self.len { return false; }
        // 安全性 (SAFETY)：offset 已在上方进行了边界检查，base 已由 new() 的契约保证有效
        unsafe {
            core::ptr::write_volatile(self.base.add(offset) as *mut u32, value);
        }
        true
    }
}
```

**结合幽灵类型 (第 9 章) 实现类型化的 MMIO：**

```rust,ignore
use std::marker::PhantomData;

pub struct ReadOnly;
pub struct ReadWrite;

pub struct TypedMmio<Perm> {
    region: MmioRegion,
    _perm: PhantomData<Perm>,
}

impl TypedMmio<ReadOnly> {
    pub fn read_u32(&self, offset: usize) -> Option<u32> {
        self.region.read_u32(offset)
    }
    // 没有 write 方法 —— 如果你尝试写入 ReadOnly 区域，会产生编译错误
}

impl TypedMmio<ReadWrite> {
    pub fn read_u32(&self, offset: usize) -> Option<u32> {
        self.region.read_u32(offset)
    }
    pub fn write_u32(&self, offset: usize, value: u32) -> bool {
        self.region.write_u32(offset, value)
    }
}
```

> **`unsafe` 封装指南：**
>
> | 规则 | 原因 |
> |------|-----|
> | 仅提供一个带有 `# 安全性 (# Safety)` 文档注释的 `unsafe fn new()` | 调用者只需在入口处负责一次 |
> | 其他所有方法均为安全的 (safe) | 调用者无法触发未定义行为 (UB) |
> | 每一个 `unsafe` 块上方都有 `# 安全性 (# SAFETY:)` 注释 | 审计者可以进行局部验证 |
> | 封装在一个模块内并加上 `#[deny(unsafe_op_in_unsafe_fn)]` | 即使在 `unsafe fn` 内部，单独的操作也要加上 `unsafe` 标记 |
> | 在封装模块上运行 `cargo +nightly miri test` | 验证其是否符合内存模型 |

---

### ✅ 检查点：技巧 1–7

你已经掌握了 7 个日常开发中的小技巧。下面是一个快速记分卡：

| 技巧 | 被消除的 Bug 类型 | 采用成本 |
|:-----:|----------------------|:---------------:|
| 1 | 哨兵值混淆 (0xFF) | 低 —— 仅需在边界处进行一次 `match` |
| 2 | 未经授权的特性实现 | 低 —— 增加 `Sealed` 父特性约束 |
| 3 | 枚举扩充导致的消费者崩溃 | 低 —— 只需要增加一行属性标记 |
| 4 | 遗漏了构建器字段 | 中 —— 增加了额外的类型参数 |
| 5 | 字符串类型的配置项写错 | 低 —— 实现 `FromStr` 特性 |
| 6 | 错误的缓冲区大小 | 低 —— 增加一个常量泛型参数 |
| 7 | `unsafe` 代码散落在各处 | 中 —— 编写封装模块 |

技巧 8-14 属于 **进阶** 内容 —— 它们涉及 async、常量求值、会话类型、`Pin` 以及 `Drop`。如果你觉得累了，可以在这里稍作休息；上面这几项技巧对于明天的开发工作来说已经是非常高价值、且低投入的即战力了。

***

### 技巧 8 —— 异步类型状态机 (Async Type-State Machines)

当硬件驱动使用 `async` 时（例如：异步 BMC 通信、异步 NVMe I/O），类型状态依然有效 —— 但需要注意所有权在 `.await` 点之间的转移：

```rust,ignore
use std::marker::PhantomData;

pub struct Idle;
pub struct Authenticating;
pub struct Active;

pub struct AsyncSession<S> {
    host: String,
    _state: PhantomData<S>,
}

impl AsyncSession<Idle> {
    pub fn new(host: &str) -> Self {
        AsyncSession { host: host.to_string(), _state: PhantomData }
    }

    /// 执行 Idle → Authenticating → Active 的状态转换。
    /// Session 在跨越 .await 时被消耗（移动到了 future 中）。
    pub async fn authenticate(self, user: &str, pass: &str)
        -> Result<AsyncSession<Active>, String>
    {
        // 第一阶段：发送凭据 (消耗掉 Idle session)
        let pending: AsyncSession<Authenticating> = AsyncSession {
            host: self.host,
            _state: PhantomData,
        };

        // 模拟异步 BMC 身份验证
        // tokio::time::sleep(Duration::from_secs(1)).await;

        // 第二阶段：返回 Active session
        Ok(AsyncSession {
            host: pending.host,
            _state: PhantomData,
        })
    }
}

impl AsyncSession<Active> {
    pub async fn send_command(&mut self, cmd: &[u8]) -> Vec<u8> {
        // 在此处进行异步 I/O...
        vec![0x00]
    }
}

// 用法示例：
// let session = AsyncSession::new("192.168.1.100");
// let mut session = session.authenticate("admin", "pass").await?;
// let resp = session.send_command(&[0x04, 0x2D]).await;
```

**异步类型状态的关键规则：**

| 规则 | 原因 |
|------|-----|
| 转换方法采用 `self` (按值传递)，而非 `&mut self` | 这样所有权转移在跨越 `.await` 时才能生效 |
| 对于可恢复的错误，返回 `Result<NextState, (Error, PrevState)>` | 这样调用者可以从之前的状态重试 |
| 不要将状态拆分到多个不同的 Future 中 | 单个 Future 应当拥有对应的单个 Session |
| 配合 `tokio::spawn` 时使用 `Send + 'static` 约束 | 该 Session 必须可跨线程移动 |

> **注意：** 如果你需要在出错时找回 *之前* 的状态（以便重试），请返回 `Result<AsyncSession<Active>, (Error, AsyncSession<Idle>)>`，这样调用者才能拿回所有权。否则，一个失败的 `.await` 会永久性地“丢弃”该 session。

***

### 技巧 9 —— 通过常量断言实现细化类型 (Refinement Types)

当数值约束是编译时不变式（而非运行时数据）时，使用 `const` 求值来强制执行它。这与技巧 6 不同（技巧 6 提供的是类型层面的大小区分） —— 这里我们是在编译时 *拒绝无效的数值*：

```rust,ignore
/// 必须处于 IPMI SDR 范围 (0x01..=0xFE) 内的传感器 ID。
/// 当 `N` 为常量时，约束会在编译时被检查。
pub struct SdrSensorId<const N: u8>;

impl<const N: u8> SdrSensorId<N> {
    /// 编译时验证：如果 N 超出范围，会在编译期间产生 panic。
    pub const fn validate() {
        assert!(N >= 0x01, "传感器 ID 必须 >= 0x01");
        assert!(N <= 0xFE, "传感器 ID 必须 <= 0xFE (0xFF 已保留)");
    }

    pub const VALIDATED: () = Self::validate();

    pub const fn value() -> u8 { N }
}

// 用法示例：
fn read_sensor_const<const N: u8>() -> f64 {
    let _ = SdrSensorId::<N>::VALIDATED;  // 编译时检查
    // 读取传感器 N...
    42.0
}

// read_sensor_const::<0x20>();   // ✅ 可编译 —— 0x20 是有效的
// read_sensor_const::<0x00>();   // ❌ 编译错误 —— "传感器 ID 必须 >= 0x01"
// read_sensor_const::<0xFF>();   // ❌ 编译错误 —— 0xFF 已保留
```

**更简单的形式 —— 有界风扇 ID：**

```rust,ignore
pub struct BoundedFanId<const N: u8>;

impl<const N: u8> BoundedFanId<N> {
    pub const VALIDATED: () = assert!(N < 8, "服务器最多有 8 个风扇 (0..7)");

    pub const fn id() -> u8 {
        let _ = Self::VALIDATED;
        N
    }
}

// BoundedFanId::<3>::id();   // ✅
// BoundedFanId::<10>::id();  // ❌ 编译错误
```

> **何时使用：** 在编译时已知的硬件定义固定 ID（传感器 ID、风扇插槽、PCIe 插槽编号）。如果数值来自运行时数据（配置文件、用户输入），请改用 `TryFrom` / `FromStr` (第 7 章，技巧 5)。

***

### 技巧 10 —— 用于信道通信的会话类型 (Session Types)

当两个组件通过信道 (Channel) 通信时（例如：诊断编排器 ↔ 工作线程），**会话类型 (Session Types)** 可以在类型系统中对协议进行编码：

```rust,ignore
use std::marker::PhantomData;

// 协议定义：客户端发送请求 (Request)，服务端发送响应 (Response)，然后结束。
pub struct SendRequest;
pub struct RecvResponse;
pub struct Done;

/// 一个类型化的信道端点。`S` 是当前协议状态。
pub struct Chan<S> {
    // 实际代码中：内部封装了一对 mpsc::Sender/Receiver
    _state: PhantomData<S>,
}

impl Chan<SendRequest> {
    pub fn send(self, req: String) -> Chan<RecvResponse> {
        println!("发送请求中：{req}");
        Chan { _state: PhantomData }
    }
}

impl Chan<RecvResponse> {
    pub fn recv(self) -> (String, Chan<Done>) {
        ( "响应数据".to_string(), Chan { _state: PhantomData } )
    }
}

// 用法示例：
fn protocol_demo(c: Chan<SendRequest>) {
    let c = c.send("开始诊断".to_string());
    let (resp, c) = c.recv();
    println!("收到响应：{resp}");
    // c 现在处于 Done 状态 —— 无法再发送或接收
}
```

**为什么这很有用：**
- **防止违反协议：** 你无法在 `Done` 状态下发送请求。
- **强制实现全对等：** 编译器确保你不仅发送了请求，还 *不得不* 接收响应（否则你无法获得 `Chan<Done>`）。
- **零成本：** 所有状态转换在跨越边界时都会被内联消除。

> **何时使用：** 线程间的诊断协议、BMC 命令序列、任何对顺序有要求的“请求-响应”模式。对于复杂的多消息协议，可以考虑使用 [`session-types`](https://crates.io/crates/session-types) 或 [`rumpsteak`](https://crates.io/crates/rumpsteak) 等 crate。

***

### 技巧 11 —— 用于自引用状态机的 `Pin`

某些类型状态机需要持有对其自身数据的引用（例如：一个追踪其自有缓冲区内部位置的解析器）。Rust 通常禁止这样做，因为移动结构体（move）会导致内部指针失效。`Pin<T>` 通过保证该值 **不会被移动** 解决了这一问题：

```rust,ignore
use std::pin::Pin;
use std::marker::PhantomPinned;

/// 一个持有对其自身缓冲区引用的流式解析器。
/// 一旦被固定 (pinned)，它就不能再被移动 —— 从而保证内部引用始终有效。
pub struct StreamParser {
    buffer: Vec<u8>,
    /// 指向 `buffer` 内部。仅在被固定时有效。
    cursor: *const u8,
    _pin: PhantomPinned,  // 选择退出 Unpin —— 防止意外的取消固定 (unpinning)
}

impl StreamParser {
    pub fn new(data: Vec<u8>) -> Pin<Box<Self>> {
        let parser = StreamParser {
            buffer: data,
            cursor: std::ptr::null(),
            _pin: PhantomPinned,
        };
        let mut boxed = Box::pin(parser);

        // 设置游标指向已固定的缓冲区内部
        let cursor = boxed.buffer.as_ptr();
        // 安全性 (SAFETY)：我们拥有独占访问权，且解析器已被固定
        unsafe {
            let mut_ref = Pin::as_mut(&mut boxed);
            Pin::get_unchecked_mut(mut_ref).cursor = cursor;
        }

        boxed
    }

    /// 读取下一个字节 —— 仅能通过 Pin<&mut Self> 调用。
    pub fn next_byte(self: Pin<&mut Self>) -> Option<u8> {
        // 解析器无法被移动，因此游标 (cursor) 始终有效
        if self.cursor.is_null() { return None; }
        // ... 在缓冲区中推进游标 ...
        Some(42) // 存根示例
    }
}
```

**关键洞察：** 对于自引用结构体问题，`Pin` 是“正确构建”的解决方案。如果没有它，你需要使用 `unsafe` 并手动追踪生命周期。有了它之后，编译器会自动防止移动操作，从而维持内部指针的不变式。

| 在以下情况使用 `Pin`…… | 在以下情况不要用 `Pin`…… |
|-----------------|----------------------|
| 状态机持有结构体内部引用 | 所有字段的所有权都是独立的 |
| 需要跨 `.await` 借用的异步 Future | 不需要自引用 |
| 绝不能在内存中移位的 DMA 描述符 | 数据可以自由移动 |
| 带有内部游标的硬件环形缓冲区 | 简单的基于索引的迭代即可满足需求 |

***

### 技巧 12 —— 将 RAII / `Drop` 作为正确性保证

Rust 的 `Drop` 特性是一种“正确构建”机制：清理代码 **绝不会被遗忘**，因为编译器会自动插入它。对于必须被 **精确释放一次** 的硬件资源来说，这具有极高的价值。

```rust,ignore
use std::io;

/// 一个在完成后必须被关闭的 IPMI 会话。
/// `Drop` 实现保证了即使在发生 panic 或通过 `?` 提前返回时，清理逻辑也会执行。
pub struct IpmiSession {
    handle: u32,
}

impl IpmiSession {
    pub fn open(host: &str) -> io::Result<Self> {
        // ... 协商 IPMI 会话 ...
        Ok(IpmiSession { handle: 42 })
    }

    pub fn send_raw(&self, _data: &[u8]) -> io::Result<Vec<u8>> {
        Ok(vec![0x00])
    }
}

impl Drop for IpmiSession {
    fn drop(&mut self) {
        // 关闭会话命令：无论发生 panic 还是提前返回，都会运行。
        // 在 C 语言中，忘记调用 CloseSession() 会导致 BMC 会话槽位泄露。
        let _ = self.send_raw(&[0x06, 0x3C]);
        eprintln!("[RAII] 会话 {} 已关闭", self.handle);
    }
}

// 用法示例：
fn diagnose(host: &str) -> io::Result<()> {
    let session = IpmiSession::open(host)?;
    session.send_raw(&[0x04, 0x2D, 0x20])?;
    // 无需显式关闭 —— 此处会自动运行 Drop
    Ok(())
    // 即使 send_raw 返回了 Err(...)，会话依然会被关闭。
}
```

**RAII 消除的 C/C++ 故障模式：**

```text
C:     session = ipmi_open(host);
       ipmi_send(session, data);
       if (error) return -1;        // 🐛 泄露了会话 —— 忘记调用 close()
       ipmi_close(session);

Rust:  let session = IpmiSession::open(host)?;
       session.send_raw(data)?;     // ✅ ? 返回时会自动运行 Drop
       // Drop 总是会运行 —— 泄露是不可能的
```

**将 RAII 与类型状态 (第 5 章) 相组合以实现有序清理：**

你无法针对泛型参数特化 (specialize) `Drop`（Rust 错误 E0366）。取而代之的是，针对每种状态使用 **独立的包装类型**：

```rust,ignore
use std::marker::PhantomData;

pub struct Open;
pub struct Locked;

pub struct GpuContext<S> {
    device_id: u32,
    _state: PhantomData<S>,
}

impl GpuContext<Open> {
    pub fn lock_clocks(self) -> LockedGpu {
        // ... 锁定 GPU 时钟以实现稳定的基准测试 ...
        LockedGpu { device_id: self.device_id }
    }
}

/// 锁定状态下的独立类型 —— 拥有自己的 Drop 实现。
/// 我们无法实现 `impl Drop for GpuContext<Locked>` (E0366)，
/// 因此我们使用一个持有被锁定资源的独立包装器。
pub struct LockedGpu {
    device_id: u32,
}

impl LockedGpu {
    pub fn run_benchmark(&self) -> f64 {
        // ... 在锁定频率下运行基准测试 ...
        42.0
    }
}

impl Drop for LockedGpu {
    fn drop(&mut self) {
        // 在 drop 时解锁频率 —— 仅对锁定状态的包装器触发。
        eprintln!("[RAII] GPU {} 时钟已解锁", self.device_id);
    }
}

// GpuContext<Open> 没有特殊的 Drop —— 无需解锁时钟。
// LockedGpu 在 drop 时总是会解锁，哪怕发生了 panic 或提前返回。
```

> **为什么不能实现 `impl Drop for GpuContext<Locked>`？** Rust 要求 `Drop` 实现在泛型类型的所有实例化中都适用。想要实现特定状态的清理，请从以下方案中任选其一：
>
> | 方案 | 优点 | 缺点 |
> |----------|------|------|
> | 独立的包装类型 (如上) | 清晰、零成本 | 额外的类型名称 |
> | 泛型 `Drop` + 运行时的 `TypeId` 检查 | 单一类型 | 需要 `'static` 约束，有运行时成本 |
> | 带穷尽匹配的 `enum` 状态 | 单一泛型类型 | 运行时分发 (Dispatch)，类型安全性稍低 |

> **何时使用：** BMC 会话、GPU 频率锁、DMA 缓冲区映射、文件句柄、互斥锁守卫 (Mutex guards)、任何具有强制释放步骤的资源。如果你发现自己在写 `fn close(&mut self)` 或 `fn cleanup()`，那几乎可以肯定它应当是 `Drop`。

***

### 技巧 13 —— 将错误类型层级作为正确性

设计精良的错误类型可以防止静默地吞掉错误，并确保调用者能够恰当地处理每一种故障模式。使用 `thiserror` 处理结构化错误是一种“正确构建”模式：编译器会强制执行穷尽匹配。

```toml
# Cargo.toml
[dependencies]
thiserror = "1"
# 对于应用程序级的错误处理 (可选)：
# anyhow = "1"
```

```rust,ignore
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DiagError {
    #[error("IPMI 通信失败：{0}")]
    Ipmi(#[from] IpmiError),

    #[error("传感器 {sensor_id:#04x} 读数超出范围：{value}")]
    SensorRange { sensor_id: u8, value: f64 },

    #[error("GPU {gpu_id} 无响应")]
    GpuTimeout { gpu_id: u32 },

    #[error("配置无效：{0}")]
    Config(String),
}

#[derive(Debug, Error)]
pub enum IpmiError {
    #[error("会话身份验证失败")]
    AuthFailed,

    #[error("命令 {net_fn:#04x}/{cmd:#04x} 超时")]
    Timeout { net_fn: u8, cmd: u8 },

    #[error("完成码 (Completion code) {0:#04x}")]
    CompletionCode(u8),
}

// 调用者必须处理每一个变体 —— 而不是静默吞掉：
fn run_thermal_check() -> Result<(), DiagError> {
    // 如果此函数返回 IpmiError，它会通过 #[from] 属性自动转换为 DiagError::Ipmi。
    let temp = read_cpu_temp()?;
    if temp > 105.0 {
        return Err(DiagError::SensorRange {
            sensor_id: 0x20,
            value: temp,
        });
    }
    Ok(())
}
```

**为什么这是“正确构建”：**

| 没有结构化错误时 | 使用 `thiserror` 枚举时 |
|--------------------------|----------------------|
| `fn op() -> Result<T, String>` | `fn op() -> Result<T, DiagError>` |
| 调用者只能得到不透明的字符串 | 调用者可以对特定变体进行 match 匹配 |
| 无法区分身份验证失败与超时 | `DiagError::Ipmi(IpmiError::AuthFailed)` vs `Timeout` |
| 日志吞掉了错误 | `match` 迫使你处理每一种情况 |
| 增加了新的错误变体 → 没人注意到 | 增加新变体 → 编译器会警告 match 分支不穷尽 |

**`anyhow` 与 `thiserror` 的权衡决策：**

| 在以下情况使用 `thiserror`…… | 在以下情况使用 `anyhow`…… |
|-----------------------|-------------------|
| 编写库 (library) 或 crate 时 | 编写二进制文件 (binary) 或 CLI 时 |
| 调用者需要对错误变体进行 match 匹配时 | 调用者只需要记录日志并退出时 |
| 错误类型是公有 API 的一部分时 | 仅用于内部的错误传递逻辑时 |
| `protocol_lib`, `accel_diag`, `thermal_diag` | `diag_tool` 主二进制程序 |

> **何时使用：** 工作区中的每个 crate 都应当使用 `thiserror` 定义自己的错误枚举。顶层的二进制 crate 则可以使用 `anyhow` 进行汇总。这既能让库的调用者享受到编译时的错误处理保证，又能保持二进制程序的简洁性。

***

### 技巧 14 —— 使用 `#[must_use]` 强制消费

`#[must_use]` 属性会将“被忽略的返回值”变为编译器警告。这是一个轻量级的“正确构建”工具，可以与本书中的每一个模式完美配合：

```rust,ignore
/// 一个必须被使用的校准令牌 —— 悄悄 drop 掉它是一个 Bug。
#[must_use = "校准令牌必须传给 calibrate()，而非丢弃"]
pub struct CalibrationToken {
    _private: (),
}

/// 一个必须被检查的诊断结果 —— 忽略失败结果是一个 Bug。
#[must_use = "诊断结果必须被检查是否存在故障"]
pub struct DiagResult {
    pub passed: bool,
    pub details: String,
}

/// 返回重要值的函数也应当被如此标记：
#[must_use = "经身份验证的会话必须被使用或显式关闭"]
pub fn authenticate(user: &str, pass: &str) -> Result<Session, AuthError> {
    // ...
#   unimplemented!()
}
```

**编译器会告诉你什么：**

```text
warning: unused `CalibrationToken` that must be used
  --> src/main.rs:5:5
   |
5  |     CalibrationToken { _private: () };
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: 校准令牌必须传给 calibrate()，而非丢弃
```

**将 `#[must_use]` 应用于以下模式：**

| 模式 | 标注对象 | 原因 |
|---------|-----------------|-----|
| 一次性令牌 (第 3 章) | `CalibrationToken`, `FusePayload` | 未经使用就 drop 等同于逻辑 Bug |
| 能力令牌 (第 4 章) | `AdminToken` | 进行了身份验证但忽略了令牌 |
| 类型状态转换 | `authenticate()`、`activate()` 的返回值 | 会话已创建但从未被使用 |
| 结果 (Results) | `DiagResult`, `SensorReading` | 可能导致静默发生的失败吞没 |
| RAII 句柄 (技巧 12) | `IpmiSession`, `LockedGpu` | 打开了资源却不使用 |

> **经验法则：** 如果不使用直接丢弃某个值总是代表某种 Bug，那就加上 `#[must_use]`。如果有时是有意为之（例如 `Vec`），则不要加。下划线前缀（`let _ = foo()`）可以显式表示确认并消除警告 —— 这在确实有意 drop 时是没问题的。

## 关键要点

1. **边界处：哨兵值 → Option** —— 在解析时将魔数转换为 `Option`；编译器会强制调用者处理 `None` 的情况。
2. **密封特性关闭了实现漏洞** —— 私有父特性约束意味着只有你自己的 crate 才能实现该特性。
3. **`#[non_exhaustive]` + `#[must_use]` 是只需一行的价值巨大的标注** —— 将它们加在不断演进的枚举和被消费的令牌上。
4. **类型状态构建器强制执行必填字段要求** —— `finish()` 方法只有在所有必填的类型参数都被设为 `Set` 时才存在。
5. **每一个技巧都针对一类特定的 Bug** —— 逐步采用它们即可；没有任何一个技巧需要重写整个架构。

---
