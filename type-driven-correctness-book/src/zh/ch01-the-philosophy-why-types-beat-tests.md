[English Original](../en/ch01-the-philosophy-why-types-beat-tests.md)

# 核心理念：为什么类型优于测试 🟢

> **你将学到什么？** 编译期正确性的三个层次（值、状态、能力）；测试覆盖率如何因未能表达“不可能状态”而误导开发者；以及如何在业务逻辑进入第一行 `if` 语句之前，就捕捉到逻辑错误。

## 引言：测试的局限性

软件开发中的传统智慧是：“如果您没有测试它，它就是损坏的。” 虽然这对于验证业务逻辑很有价值，但测试本质上是 **反应性** 的。它们验证 *已知* 的输入集。

相比之下，类型驱动开发是 **原生** 的。它验证 *所有可能* 的状态集。

### 案例研究：传感器读数的“幽灵 Bug”

考虑一段典型的系统编程代码，它从传感器读取原始字节：

```rust
fn handle_sensor(sensor_type: &str, raw: &[u8]) {
    match sensor_type {
        "temperature" => {
             let val = raw[0] as i8 as f64;
             println!("摄氏度: {}", val);
        }
        "voltage" => {
             let val = u16::from_be_bytes([raw[0], raw[1]]) as f64 / 1000.0;
             println!("电压: {}V", val);
        }
        _ => panic!("未知传感器类型"),
    }
}
```

这段代码充满了可能（且在实践中经常发生）被单元测试漏掉的问题：
1. **错义 (Mis-interpretation)**：将电压传递给原本期待温度处理的代码。
2. **越界 (Out of bounds)**：如果 `raw` 只有 1 个字节，电压分支会发生 Panic。
3. **单位混淆 (Unit confusion)**：传感器输出了 mV，但代码可能期待的是 V。

单元测试能抓到这些吗？当然。但你必须 **预先想到** 每一种错误情况。如果你忘记测试 1 字节的 `raw` 数组，错误就会进入生产。

## 正确性的三个层次

本书将教你如何利用 Rust 强大的类型系统，在三个不断深化的层次上强制执行不变量。

### 层次 1：值正确性 (Value Correctness)

**目标：使无效值无法被表达。**

如果不允许构造无效的对象，你就永远不需要编写代码来处理它们。

*   **无法捕获的失败模式**：函数接受 `u16` 作为端口号，但业务逻辑禁止使用端口 0。
*   **类型化解决方案**：使用 `NonZeroU16` 或 `Newtype` 包装器。

```rust
pub struct Port(u16); // 私有字段

impl TryFrom<u16> for Port {
    type Error = &'static str;
    fn try_from(v: u16) -> Result<Self, Self::Error> {
        if v == 0 { return Err("端口 0 无效识别"); }
        Ok(Port(v))
    }
}
```

现在，下游的每一个函数（如 `listen(port: Port)`）都 **绝对保证** 不会收到 0，且不需要在自己的函数体内增加 `if v == 0` 的检查。

### 层次 2：状态正确性 (State Correctness)

**目标：使无效的转换无法表达。**

这是 **Type-state 模式** 进场的地方。与其使用 `enum State` 并在运行时检查，不如使用类型来表示对象的状态。

*   **无法捕获的失败模式**：在调用 `.start()` 之前调用了 `.stop()`。
*   **类型化解决方案**：状态机。

```rust
struct Waiting;
struct Running;

struct Engine<S> { _state: S }

impl Engine<Waiting> {
    fn start(self) -> Engine<Running> { Engine { _state: Running } }
}

impl Engine<Running> {
    fn stop(self) -> Engine<Waiting> { Engine { _state: Waiting } }
}
```

在 `Engine<Waiting>` 上调用 `.stop()` 甚至 **无法编译**。编译器不再仅仅是一个检查器，它变成了你的架构蓝图的强制执行者。

### 层次 3：能力正确性 (Capability Correctness)

**目标：必须持有“能力令牌”才能执行受限操作。**

这是本书中最强大的模式。你可以在编译期证明调用者已经获得了特定的授权或完成了特定的先验步骤。

*   **无法捕获的失败模式**：在尚未验证用户身份的情况下执行敏感的操作。
*   **类型化解决方案**：能力令牌。

```rust
struct AuthenticatedToken { user_id: u64 };

fn delete_record(token: &AuthenticatedToken, record_id: u64) {
    // 只有能出示令牌，才能调用此函数
}
```

## 为什么类型胜过测试

总结来说，类型不仅仅是数据标签：

1.  **类型是普适的**：测试验证输入子集；类型验证全部取值空间。
2.  **类型是持久的**：测试会在 CI 中失败；类型在代码生命周期内都在强制执行。
3.  **类型是强制性的**：你可以跳过测试；但你无法绕过编译器（除非使用 `unsafe`，这通常也是我们要极力规避的）。

在本指南中，我们将从最基础的模式开始，逐步深入到复杂的协议机和硬件抽象，目标始终只有一个：**让你的软件在构建之初就保持正确性。**

***
