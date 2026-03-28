# 核心理念：为什么类型优于测试 🟢

> **你将学到：** 编译期正确性的三个层次（值、状态、协议），泛型函数签名如何作为编译器检查的保证，以及何时“构造即正确”模式值得投入（以及何时不值得）。
>
> **相关章节：** [ch02](ch02-typed-command-interfaces-request-determi.md)（类型化命令）、[ch05](ch05-protocol-state-machines-type-state-for-r.md) (type-state)、[ch13](ch13-reference-card.md)（速查卡）

## 运行时检查的代价

考虑一个典型的硬件诊断代码中的运行时守卫：

```rust
fn read_sensor(sensor_type: &str, raw: &[u8]) -> f64 {
    match sensor_type {
        "temperature" => raw[0] as i8 as f64,          // 有符号字节
        "fan_speed"   => u16::from_le_bytes([raw[0], raw[1]]) as f64,
        "voltage"     => u16::from_le_bytes([raw[0], raw[1]]) as f64 / 1000.0,
        _             => panic!("unknown sensor type: {sensor_type}"),
    }
}
```

该函数包含 **四个编译器无法捕获的失败模式**：拼写错误、错误的 `raw` 数据长度、逻辑错误（混淆单位）以及新类型添加时忘记更新 match 分支。

## 正确性的三个层次

### 第 1 层 —— 值正确性 (Value Correctness)
**使无效值无法被表达。**

```rust
pub struct Port(u16);  // 私有字段

impl TryFrom<u16> for Port {
    type Error = &'static str;
    fn try_from(v: u16) -> Result<Self, Self::Error> {
        if v > 0 { Ok(Port(v)) } else { Err("port must be > 0") }
    }
}
```

### 第 2 层 —— 状态正确性 (State Correctness)
**使无效的状态转换无法被表达。**

```rust
struct Socket<State> {
    fd: i32,
    _state: PhantomData<State>,
}

impl Socket<Disconnected> {
    fn connect(self) -> Socket<Connected> { ... }
}

impl Socket<Connected> {
    fn send(&mut self, data: &[u8]) { ... }
}
```

### 第 3 层 —— 协议正确性 (Protocol Correctness)
**使无效的交互无法被表达。**

```rust
trait IpmiCmd {
    type Response;
    fn parse_response(&self, raw: &[u8]) -> io::Result<Self::Response>;
}
```

## 何时不该使用这些模式

| 场景 | 建议 |
|-----------|---------------|
| 安全关键边界 | ✅ 始终建议使用 |
| 跨模块公共 API | ✅ 通常建议使用 |
| 包含 3 个以上状态的状态机 | ✅ 通常建议使用 |
| 50 行以内的内部辅助函数 | ❌ 过度设计 |
| 原型开发 | ❌ 先写原始类型 |

## 关键收获

1. **正确性的三个层次** —— 值 (newtype)、状态 (type-state)、协议 (关联类型)。
2. **类型即保证** —— 每一个泛型签名都是一份编译器在每次构建时都会检查的契约。
3. **代价问题** —— “如果这个 bug 出现在生产环境，会有多糟糕？” 决定了使用类型还是测试。
4. **类型是测试的补充** —— 类型消除了整类错误，测试则覆盖具体的值和边缘情况。

***
