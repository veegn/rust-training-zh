[English Original](../en/ch17-4-logging-and-tracing-ecosystem.md)

# 17.4 日志与追踪生态系统 🟢

日志对于理解应用程序的行为至关重要，特别是在生产环境或复杂的分布式系统中。Rust 拥有一个健壮且模块化的日志和追踪生态系统，它将 *日志接口* (门面) 与 *日志实现* (收集器) 分开。

### 1. `log` 门面 (Facade)
`log` crate 提供了一组标准宏 (`error!`、`warn!`、`info!`、`debug!`、`trace!`)，库可以使用这些宏来发出日志消息，而无需绑定到特定的日志实现。

```rust
use log::{info, warn};

fn main() {
    // 只有库使用这些宏；应用程序必须选择一个日志记录器实现
    info!("正在启动应用程序...");
    warn!("检测到内存不足！");
}
```

---

### 2. 选择日志记录器实现
应用程序必须选择一个日志记录器实现来实际捕获并存储日志消息。常用的选择包括：
- **`env_logger`**：基于环境变量 (`RUST_LOG`) 将日志打印到标准输出的简单日志记录器。
- **`flexi_logger`**：更高级的日志记录器，支持日志轮转和自定义格式。
- **`syslog`**：将日志消息发送到系统的 syslog。

```rust
fn main() {
    // 在应用程序的 main 函数中初始化 env_logger
    env_logger::init();
    
    log::info!("日志记录器已初始化！");
}
```

---

### 3. 使用 `tracing` 进行结构化日志记录
虽然 `log` 非常适用于简单的文本消息，但 **`tracing`** 生态系统提供了 **结构化日志记录**。它允许你将键值对附加到日志消息，并跟踪执行的 "Span" 以获得更好的上下文。

```rust
use tracing::{info, span, Level};

fn main() {
    let span = span!(Level::INFO, "我的 Span", user_id = 42);
    let _enter = span.enter();

    info!("正在处理请求..."); // 此日志将与 user_id = 42 相关联
}
```

---

### 4. 收集追踪 (Traces)
与 `log` 类似，`tracing` 需要一个“订阅者 (Subscriber)”来收集和显示追踪信息。`tracing-subscriber` crate 是最常见的方式。

```rust
use tracing_subscriber;

fn main() {
    // 初始化追踪订阅者
    tracing_subscriber::fmt::init();
    
    tracing::info!("追踪订阅者已初始化！");
}
```

---

### 对于 C/C++ 开发者的总结
- **在 C/C++ 中**：你可能会使用各种日志库（例如 `spdlog`、`glog`、`log4cplus`），且每个库都有自己的 API。通常很难统一来自不同依赖项的日志。
- **In Rust**：`log` 和 `tracing` 门面为所有 crate 提供了统一的诊断发出方式。作为应用程序开发人员，通过选择单个实现或订阅者，你可以完全控制这些日志和追踪的收集和格式化方式。

***
