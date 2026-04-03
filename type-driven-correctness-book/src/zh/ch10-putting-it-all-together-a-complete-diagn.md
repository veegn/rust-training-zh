[English Original](../en/ch10-putting-it-all-together-a-complete-diagn.md)

# 综合运用 —— 一个完整的诊断平台 🟡

> **你将学到：**
> - 如何将所有七种核心模式（第 2 章至第 9 章）组合成一个单一的诊断工作流。
> - 涵盖身份验证、会话管理、类型化命令、审计令牌、量纲结果、验证过的数据以及幽灵类型寄存器。
> - 所有这些保证的运行时总开销均为零。

> **参考：** 所有核心模式章节（第 2 章至第 9 章）、[第 14 章](ch14-testing-type-level-guarantees.md)（测试这些保证）。

## 目标

本章将第 2 章至第 9 章中的 **七种模式** 结合到一个真实、完整的诊断工作流中。我们将构建一个服务器健康检查程序，它能够：

1. **进行身份验证**（能力令牌 —— 第 4 章）
2. **开启 IPMI 会话**（类型状态 —— 第 5 章）
3. **发送类型化命令**（类型化命令 —— 第 2 章）
4. **使用一次性令牌** 进行审计日志记录（一次性类型 —— 第 3 章）
5. **返回量纲结果**（量纲分析 —— 第 6 章）
6. **验证 FRU 数据**（已验证边界 —— 第 7 章）
7. **读取类型化寄存器**（幽灵类型 —— 第 9 章）

```rust,ignore
use std::marker::PhantomData;
use std::io;

// ──── 模式 1：量纲类型 (第 6 章) ────

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Celsius(pub f64);

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Rpm(pub f64);

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Volts(pub f64);

// ──── 模式 2：类型化命令 (第 2 章) ────

/// 与第 2 章相同的 trait 结构，为了保持一致性使用方法（而非关联常量）。
/// 当值在每个类型中确实固定时，关联常量 (`const NETFN: u8`) 也是一种同样有效的替代方案。
pub trait IpmiCmd {
    type Response;
    fn net_fn(&self) -> u8;
    fn cmd_byte(&self) -> u8;
    fn payload(&self) -> Vec<u8>;
    fn parse_response(&self, raw: &[u8]) -> io::Result<Self::Response>;
}

pub struct ReadTemp { pub sensor_id: u8 }
impl IpmiCmd for ReadTemp {
    type Response = Celsius;   // ← 量纲类型！
    fn net_fn(&self) -> u8 { 0x04 }
    fn cmd_byte(&self) -> u8 { 0x2D }
    fn payload(&self) -> Vec<u8> { vec![self.sensor_id] }
    fn parse_response(&self, raw: &[u8]) -> io::Result<Celsius> {
        if raw.is_empty() {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "empty"));
        }
        Ok(Celsius(raw[0] as f64))
    }
}

pub struct ReadFanSpeed { pub fan_id: u8 }
impl IpmiCmd for ReadFanSpeed {
    type Response = Rpm;
    fn net_fn(&self) -> u8 { 0x04 }
    fn cmd_byte(&self) -> u8 { 0x2D }
    fn payload(&self) -> Vec<u8> { vec![self.fan_id] }
    fn parse_response(&self, raw: &[u8]) -> io::Result<Rpm> {
        if raw.len() < 2 {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "need 2 bytes"));
        }
        Ok(Rpm(u16::from_le_bytes([raw[0], raw[1]]) as f64))
    }
}

// ──── 模式 3：能力令牌 (第 4 章) ────

}

// ──── 模式 4：类型状态会话 (第 5 章) ────

pub struct Idle;
pub struct Active;

pub struct Session<State> {
    host: String,
    _state: PhantomData<State>,
}

impl Session<Idle> {
    pub fn connect(host: &str) -> Self {
        Session { host: host.to_string(), _state: PhantomData }
    }

    pub fn activate(
        self,
        _admin: &AdminToken,  // ← 需要能力令牌
    ) -> Result<Session<Active>, String> {
        println!("会话已在 {} 上激活", self.host);
        Ok(Session { host: self.host, _state: PhantomData })
    }
}

impl Session<Active> {
    /// 执行类型化命令 —— 仅在 Active 会话上可用。
    /// 返回 io::Result 以传播传输错误（与第 2 章一致）。
    pub fn execute<C: IpmiCmd>(&mut self, cmd: &C) -> io::Result<C::Response> {
        let raw_response = self.raw_send(cmd.net_fn(), cmd.cmd_byte(), &cmd.payload())?;
        cmd.parse_response(&raw_response)
    }

    fn raw_send(&self, _nf: u8, _cmd: u8, _data: &[u8]) -> io::Result<Vec<u8>> {
        Ok(vec![42, 0x1E]) // 存根：原始 IPMI 响应
    }

    pub fn close(self) { println!("会话已关闭"); }
}

// ──── 模式 5：一次性审计令牌 (第 3 章) ────

/// 每一个诊断运行都会获得一个独一无二的审计令牌。
/// 不可 Clone，也不可 Copy —— 保证每一条审计条目都是唯一的。
pub struct AuditToken {
    run_id: u64,
}

    }
}

// ──── 模式 6：已验证边界 (第 7 章) ────
// 这里简化了第 7 章中完整的 ValidFru —— 仅保留本复合示例所需的字段。
// 完整的 TryFrom<RawFruData> 版本请参见第 7 章。

pub struct ValidFru {
    pub board_serial: String,
    pub product_name: String,
}

impl ValidFru {
    pub fn parse(raw: &[u8]) -> Result<Self, &'static str> {
        if raw.len() < 8 { return Err("FRU 太短"); }
        if raw[0] != 0x01 { return Err("FRU 版本错误"); }
        Ok(ValidFru {
            board_serial: "SN12345".to_string(),  // 存根
            product_name: "ServerX".to_string(),
        })
    }
}

// ──── 模式 7：幽灵类型寄存器 (第 9 章) ────

pub struct Width16;
pub struct Reg<W> { offset: u16, _w: PhantomData<W> }

impl Reg<Width16> {
    pub fn read(&self) -> u16 { 0x8086 } // 存根
}

pub struct PcieDev {
    pub vendor_id: Reg<Width16>,
    pub device_id: Reg<Width16>,
}

    }
}

// ──── 复合工作流 (Composite Workflow) ────

fn full_diagnostic() -> Result<(), String> {
    // 1. 验证身份 → 获取能力令牌
    let admin = authenticate("admin", "secret")
        .map_err(|e| e.to_string())?;

    // 2. 连接并开启会话 (类型状态：Idle → Active)
    let session = Session::connect("192.168.1.100");
    let mut session = session.activate(&admin)?;  // 需要 AdminToken

    // 3. 发送类型化命令 (响应类型与命令相匹配)
    let temp: Celsius = session.execute(&ReadTemp { sensor_id: 0 })
        .map_err(|e| e.to_string())?;
    let fan: Rpm = session.execute(&ReadFanSpeed { fan_id: 1 })
        .map_err(|e| e.to_string())?;

    // 类型不匹配会被捕获：
    // let wrong: Volts = session.execute(&ReadTemp { sensor_id: 0 })?;
    //  ❌ 错误：预期得到 Celsius，实际发现是 Volts

    // 4. 读取幽灵类型化的 PCIe 寄存器
    let pcie = PcieDev::new();
    let vid: u16 = pcie.vendor_id.read();  // 保证获得 u16

    // 5. 在边界处验证 FRU 数据
    let raw_fru = vec![0x01, 0x00, 0x00, 0x01, 0x01, 0x00, 0x00, 0xFD];
    let fru = ValidFru::parse(&raw_fru)
        .map_err(|e| e.to_string())?;

    // 6. 签发一次性审计令牌
    let audit = AuditToken::issue(1001);

    // 7. 生成报告 (所有数据均已类型化并经验证)
    let report = format!(
        "服务器: {} (SN: {}), VID: 0x{:04X}, CPU: {:?}, 风扇: {:?}",
        fru.product_name, fru.board_serial, vid, temp, fan,
    );

    // 8. 消耗审计令牌 —— 无法记录两次
    audit.log(&report);
    // audit.log("oops");  // ❌ 错误：尝试使用已被移动的值 (use of moved value)

    // 9. 关闭会话 (类型状态：Active → 被消费/丢弃)
    session.close();

    Ok(())
}
```

### 编译器证明了什么

| Bug 类型 | 它是如何被防止的 | 模式 |
|-----------|-------------------|---------|
| 未经授权的访问 | `activate()` 需要 `&AdminToken` | 能力令牌 |
| 在错误的会话状态下发送命令 | `execute()` 仅存在于 `Session<Active>` 上 | 类型状态 |
| 错误的响应类型 | `ReadTemp::Response = Celsius`，通过 trait 固化 | 类型化命令 |
| 单位混淆 (°C vs RPM) | `Celsius` ≠ `Rpm` ≠ `Volts` | 量纲类型 |
| 寄存器宽度不匹配 | `Reg<Width16>` 返回 `u16` | 幽灵类型 |
| 处理未经验证的数据 | 必须首先调用 `ValidFru::parse()` | 已验证边界 |
| 重复的审计条目 | `AuditToken` 在记录日志时被消耗 | 一次性类型 |
| 上电时序错误 (乱序) | 每个步骤都需要前一步生成的令牌 | 能力令牌 (第 4 章) |

**实现所有这些保证所带来的运行时总开销：零。**

每一项检查都发生在编译时。生成的汇编代码与那些没有任何检查的手写 C 代码完全相同 —— 区别在于 **C 语言可能存在 Bug，而这里不会**。

## 关键要点

1. **七种模式可以无缝组合** —— 能力令牌、类型状态、类型化命令、一次性类型、量纲类型、已验证边界以及幽灵类型都能完美配合工作。
2. **编译器证明了八类 Bug 是不可能存在的** —— 参见上面的“编译器证明了什么”表格。
3. **零运行时总开销** —— 生成的汇编代码与未受检的 C 代码完全一致。
4. **每种模式都可以独立发挥作用** —— 你不需要一次性使用全部七种；可以根据需要逐步采用。
5. **集成章是一个设计模板** —— 可以将其作为你构建自己的类型化诊断工作流的起点。
6. **从 IPMI 扩展到大规模 Redfish** —— 第 17 章和第 18 章将这些相同的七种模式（加上第 8 章的能力混入）应用于完整的 Redfish 客户端和服务器。这里的 IPMI 工作流是基础；Redfish 演练则展示了这些组合如何扩展到具有多个数据源和模式版本约束的生产系统。

---
