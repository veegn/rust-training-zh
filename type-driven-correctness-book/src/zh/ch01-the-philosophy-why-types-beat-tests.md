[English Original](../en/ch01-the-philosophy-why-types-beat-tests.md)

# 核心理念 —— 为什么类型胜过测试 🟢

> **你将学到：**
> - 编译期正确性的三个层次（值、状态、协议）。
> - 泛型函数签名如何充当编译器检查的保证。
> - 何时“构造即正确 (correct-by-construction)”模式值得投入，以及何时不值得。

> **参考：** [第 2 章](ch02-typed-command-interfaces-request-determi.md)（类型化命令）、[第 5 章](ch05-protocol-state-machines-type-state-for-r.md)（类型状态）、[第 13 章](ch13-reference-card.md)（速查卡）。

## 运行时检查的成本

考虑诊断代码库中一个典型的运行时守卫 (Guard)：

```rust,ignore
fn read_sensor(sensor_type: &str, raw: &[u8]) -> f64 {
    match sensor_type {
        "temperature" => raw[0] as i8 as f64,          // 有符号字节
        "fan_speed"   => u16::from_le_bytes([raw[0], raw[1]]) as f64,
        "voltage"     => u16::from_le_bytes([raw[0], raw[1]]) as f64 / 1000.0,
        _             => panic!("未知传感器类型: {sensor_type}"),
    }
}
```

这个函数有 **四种失效模式** 是编译器无法捕捉到的：

1. **拼写错误**：`"temperture"` → 在运行时发生 panic。
2. **错误的 `raw` 长度**：`fan_speed` 仅带有 1 个字节 → 在运行时发生 panic。
3. **调用者误用**：调用者将返回的 `f64` 当作 RPM 使用，而它实际上是 °C → 逻辑 bug，且无声无息。
4. **扩展缺失**：添加了新的传感器类型但未更新此 `match` 语句 → 在运行时发生 panic。

每种失效模式都是在 **部署之后** 才被发现的。测试虽然有所帮助，但它们只能覆盖有人编写过的案例。而类型系统涵盖了 **所有** 情况，包括那些没人预料到的情况。

## 正确性的三个层次

### 层次 1 —— 值正确性 (Value Correctness)
**使无效值无法被表达。**

```rust,ignore
// ❌ 任何 u16 都可以是 "port" —— 0 是无效的但可以通过编译
fn connect(port: u16) { /* ... */ }

// ✅ 只有经过校验的端口才能存在
pub struct Port(u16);  // 私有字段

impl TryFrom<u16> for Port {
    type Error = &'static str;
    fn try_from(v: u16) -> Result<Self, Self::Error> {
        if v > 0 { Ok(Port(v)) } else { Err("端口必须 > 0") }
    }
}

fn connect(port: Port) { /* ... */ }
// Port(0) 永远无法被构造出来 —— 不变性在任何地方都成立
```

**硬件示例**：`SensorId(u8)` —— 包装一个原始传感器编号，并校验其在 SDR 范围内。

### 层次 2 —— 状态正确性 (State Correctness)
**使无效的状态转换无法被表达。**

```rust,ignore
use std::marker::PhantomData;

struct Disconnected;
struct Connected;

struct Socket<State> {
    fd: i32,
    _state: PhantomData<State>,
}

impl Socket<Disconnected> {
    fn connect(self, addr: &str) -> Socket<Connected> {
        // ... 连接逻辑 ...
        Socket { fd: self.fd, _state: PhantomData }
    }
}

impl Socket<Connected> {
    fn send(&mut self, data: &[u8]) { /* ... */ }
    fn disconnect(self) -> Socket<Disconnected> {
        Socket { fd: self.fd, _state: PhantomData }
    }
}

// Socket<Disconnected> 没有 send() 方法 —— 如果尝试调用将导致编译错误
```

**硬件示例**：GPIO 引脚模式 —— `Pin<Input>` 拥有 `read()` 方法但没有 `write()` 方法。

### 层次 3 —— 协议正确性 (Protocol Correctness)
**使无效的交互无法被表达。**

```rust,ignore
use std::io;

trait IpmiCmd {
    type Response;
    fn parse_response(&self, raw: &[u8]) -> io::Result<Self::Response>;
}

// 为便于说明进行了简化 —— 完整特性请参阅第 2 章，
// 包含 net_fn()、cmd_byte()、payload() 和 parse_response()。

struct ReadTemp { sensor_id: u8 }
impl IpmiCmd for ReadTemp {
    type Response = Celsius;
    fn parse_response(&self, raw: &[u8]) -> io::Result<Celsius> {
        Ok(Celsius(raw[0] as i8 as f64))
    }
}

# #[derive(Debug)] struct Celsius(f64);

fn execute<C: IpmiCmd>(cmd: &C, raw: &[u8]) -> io::Result<C::Response> {
    cmd.parse_response(raw)
}
// ReadTemp 始终返回 Celsius —— 不会意外地得到 Rpm
```

**硬件示例**：IPMI、Redfish、NVMe Admin 命令 —— 请求类型决定了响应类型。

## 类型作为编译器检查的保证

当你编写如下代码时：

```rust,ignore
fn execute<C: IpmiCmd>(cmd: &C) -> io::Result<C::Response>
```

你不仅仅是在编写一个函数 —— 你是在声明一个 **保证**：“对于任何实现了 `IpmiCmd` 的命令类型 `C`，执行它必定产生 `C::Response`。”编译器在每次构建代码时都会 **验证** 这个保证。如果类型不匹配，程序就无法通过编译。

这就是为什么 Rust 的类型系统如此强大 —— 它不仅仅是在捕捉错误，它是在 **编译期强制执行正确性**。

## 何时 不 使用这些模式

构造即正确 (Correct-by-construction) 并不总是最佳选择：

| 场景 | 建议 |
|-----------|---------------|
| 安全批判性边界 (上电序列、加密) | ✅ 始终使用 —— 这里的 bug 会损毁硬件或泄露秘密 |
| 跨模块的公共 API | ✅ 通常建议使用 —— 误用应当导致编译错误 |
| 拥有 3 个以上状态的状态机 | ✅ 通常建议使用 —— 类型状态 (Type-state) 可防止错误的转换 |
| 单个 50 行函数内部的辅助工具 | ❌ 过度设计 —— 简单的 `assert!` 足矣 |
| 原型设计 / 探索未知硬件 | ❌ 先使用原始类型 —— 在理解行为后再进行细化 |
| 面向用户的 CLI 解析 | ⚠️ 在边界处使用 `clap` + `TryFrom`，内部使用原始类型即可 |

关键问题在于：**“如果这个 bug 在生产环境中发生，后果有多严重？”**

- 风扇停止 → GPU 损毁 → **使用类型**
- 错误的 DER 记录 → 客户收到错误数据 → **使用类型**
- 调试日志消息稍微出错 → **使用 `assert!`**

## 关键要点

1. **正确性的三个层次** —— 值 (新类型)、状态 (类型状态)、协议 (关联类型) —— 每一层都消除了更广泛的一类 bug。
2. **类型作为保证** —— 每个泛型函数签名都是一份合同，编译器在每次构建时都会对其进行检查。
3. **成本问题** —— “如果这个 bug 发布了，后果有多严重？”决定了类型还是测试才是正确的工具。
4. **类型是对测试的补充** —— 它们消除了整个 **类别** 的错误；测试则覆盖特定的 **数值** 和边界情况。
5. **知道何时停止** —— 内部辅助工具和临时原型很少需要类型级的强制约束。

***
