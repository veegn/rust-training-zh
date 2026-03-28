[English Original](../en/ch02-typed-command-interfaces-request-determi.md)

# 类型化命令接口：请求决定响应 🟡

> **你将学到什么？** 如何利用关联类型（Associated Types）在编译期建立请求与相应之间的 1:1 映射；如何消除“类型擦除”后的动态分发检查；以及为什么这对于 IPMI、PCIe 配置空间或 Redfish 客户端设计至关重要。

## 引言：脆弱的接口

在许多系统接口中，发送一个请求并获取一个响应是一个标准流程。然而，通常这种映射是松散的。

考虑一个典型的、非类型化的命令模式：

```rust
enum Request {
    GetTemperature,
    SetFanSpeed(u8),
}

enum Response {
    Temperature(f64),
    Ack,
}

fn send_command(req: Request) -> Response {
    // 运行时逻辑
}
```

这段代码存在两个严重问题：
1.  **返回值的动态检查**：如果你发送了 `GetTemperature`，调用者必须手动解包 `Response::Temperature` 并处理 `Response::Ack` 这种事实上不可能发生的情况。
2.  **契约漏洞**：没有任何机制阻止你在代码中编写 `if let Response::Ack = send_command(GetTemperature)`，这会导致运行时错误。

## 这种关联类型解决方案

我们可以使用 Trait 和关联类型在类型系统中强行绑定请求与响应。

### 1. 定义 Command Trait

```rust
pub trait Command {
    type Response; // 关联类型：响应类型由请求类型决定
}
```

### 2. 实现具体的命令

```rust
pub struct GetTemperature;
pub struct SetFanSpeed(pub u8);

impl Command for GetTemperature {
    type Response = f64;
}

impl Command for SetFanSpeed {
    type Response = (); // Ack
}
```

### 3. 类型化的转发器

现在我们可以编写一个转发器，它能自动返回 **精确** 的响应类型：

```rust
fn execute<C: Command>(cmd: C) -> C::Response {
    // 实际的硬件通信逻辑
}
```

使用时，类型推导会自动工作：

```rust
let temp: f64 = execute(GetTemperature); // 编译器知道这一定返回 f64
execute(SetFanSpeed(100));              // 编译器知道这返回 ()
```

## 现实世界中的案例：IPMI 命令

在智能平台管理接口（IPMI）中，每个网络函数（NetFn）都有对应的请求和响应结构。使用类型化命令接口可以防止以下错误：
*   **NetFn 混淆**：尝试将一个“底盘控制”请求的响应解析为“传感器读数”。
*   **版本失真**：当硬件规范更新时，编译器会指出所有尚未处理新响应格式的代码路径。

## 为什么这种模式至关重要

1.  **消除 `match` 样板代码**：你不再需要在每个调用点写 `match response { ... }`，因为编译器已经证明了响应类型。
2.  **类型安全的 API**：库的用户无法构造出一个返回错误类型响应的调用。
3.  **零成本抽象**：所有的关联类型映射都在编译期完成，生成的汇编代码中没有任何动态检查。

在本指南的后续部分，我们将这种模式与 `Session` 类型结合，以处理需要多步交互的复杂协议。

***
