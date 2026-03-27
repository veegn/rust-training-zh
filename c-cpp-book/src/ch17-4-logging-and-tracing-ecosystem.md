## Logging and Tracing: syslog/printf → `log` + `tracing` / 日志与追踪：syslog/printf → `log` + `tracing`

> **What you'll learn / 你将学到：** Rust's two-layer logging architecture (facade + backend), the `log` and `tracing` crates, structured logging with spans, and how this replaces `printf`/`syslog` debugging.
>
> Rust 的两层日志架构（门面 + 后端）、`log` 和 `tracing` crate、带有 Span 的结构化日志，以及这些工具如何替代 `printf`/`syslog` 调试。

*C++ diagnostic code typically uses `printf`, `syslog`, or custom logging frameworks. Rust has a standardized two-layer logging architecture: a **facade** crate (`log` or `tracing`) and a **backend** (the actual logger implementation).*

C++ 诊断代码通常使用 `printf`、`syslog` 或自定义日志框架。Rust 具有标准化的两层日志架构：一个**门面（Facade）** crate（`log` 或 `tracing`）和一个**后端（Backend）**（即实际的日志记录器实现）。

---

### The `log` facade — Rust's universal logging API / `log` 门面 —— Rust 通用日志 API

*The `log` crate provides macros that mirror syslog severity levels. Libraries use `log` macros; binaries choose a backend:*

`log` crate 提供的宏镜像了 syslog 的严重程度级别。库使用 `log` 宏；二进制程序则通过选择一个后端来实现：

```rust
// Cargo.toml
// env_logger = "0.11"    # 众多后端之一

use log::{info, warn, error, debug, trace}; // 导入不同级别的宏

fn check_sensor(id: u32, temp: f64) {
    trace!("Reading sensor {id}");           // Trace / 最细粒度
    debug!("Sensor {id} raw value: {temp}"); // Debug / 开发阶段详情

    if temp > 85.0 {
        warn!("Sensor {id} high temperature: {temp}°C"); // Warn / 警告
    }
    if temp > 95.0 {
        error!("Sensor {id} CRITICAL: {temp}°C — initiating shutdown"); // Error / 错误
    }
    info!("Sensor {id} check complete");     // Info / 正常运行信息
}

fn main() {
    // Initialize / 初始化后端 —— 通常在 main() 中执行一次
    env_logger::init();  // Controlled / 由 RUST_LOG 环境变量控制
    check_sensor(0, 72.5);
}
```

```bash
# 通过环境变量控制日志级别
RUST_LOG=debug cargo run          # 显示 debug 及以上级别
RUST_LOG=warn cargo run           # 仅显示 warn 和 error
RUST_LOG=my_crate=trace cargo run # 针对特定模块进行过滤
```

| **C++** | **Rust (`log`)** | **Notes / 说明** |
|-----|-------------|-------|
| `printf("DEBUG: %s\n", msg)` | `debug!("{msg}")` | Compile-time check / 编译时检查格式 |
| `syslog(LOG_ERR, "...")` | `error!("...")` | Backend decides output / 后端决定输出去向 |
| `#ifdef DEBUG` | `trace!` / `debug!` | Zero-cost when disabled / 禁用时零成本 |
| Custom `Logger` | `log::info!` | Universal facade / 通用门面，可插拔后端 |

---

### The `tracing` crate — structured logging with spans / `tracing` crate —— 带有 Span 的结构化日志

*`tracing` extends `log` with **structured fields** and **spans** (timed scopes). This is especially useful for diagnostics code where you want to track context:*

`tracing` 通过**结构化字段**和 **Span（具有时间属性的作用域）**扩展了 `log` 的功能。这对于需要跟踪上下文的诊断代码特别有用：

```rust
use tracing::{info, warn, error, instrument, info_span};

#[instrument(skip(data), fields(gpu_id = gpu_id, data_len = data.len()))] // 自动创建 Span
fn run_gpu_test(gpu_id: u32, data: &[u8]) -> Result<(), String> {
    info!("Starting GPU test");

    let span = info_span!("ecc_check", gpu_id); // 手动创建 Span
    let _guard = span.enter();  // Guard / 此作用域内的所有日志都会包含 gpu_id

    // Structured fields / 结构化字段 —— 机器可解析
    info!(gpu_id, temp_celsius = 72.5, ecc_errors = 0, "ECC check passed");
    Ok(())
}

fn main() {
    // Initialize / 初始化 tracing 订阅者
    tracing_subscriber::fmt().with_env_filter("debug").init();
    let _ = run_gpu_test(0, &[1, 2, 3]);
}
```

### `#[instrument]` — automatic span creation / `#[instrument]` —— 自动创建 Span

*The `#[instrument]` attribute automatically creates a span with the function name and its arguments:*

`#[instrument]` 属性会自动创建一个包含函数名及其参数的 Span：

```rust
use tracing::instrument;

#[instrument] // 自动记录参数
fn parse_sel_record(record_id: u16, sensor_type: u8, data: &[u8]) -> Result<(), String> {
    // Every log inside this function automatically includes record_id, sensor_type, and data
    tracing::debug!("Parsing SEL record");
    Ok(())
}
```

---

### `log` vs `tracing` — which to use / `log` vs `tracing` —— 应该使用哪一个

| **Aspect / 维度** | **`log`** | **`tracing`** |
|--------|-------|-----------|
| **Complexity / 复杂度** | Simple / 简单 —— 只有 5 个宏 | Richer / 丰富 —— 包含 Span、字段和 instrument |
| **Structured / 结构化** | String only / 仅字符串插值 | Key-value / 键值字段：`info!(gpu_id = 0, "msg")` |
| **Timing / Span** | ❌ 否 | ✅ 是 —— `#[instrument]`、`span.enter()` |
| **Async / 异步支持** | Basic / 基础 | First-class / 一流支持 —— Span 可跨越 `.await` 传播 |

---

### Backend options / 后端可选项

| **Backend Crate / 后端 Crate** | **Output / 输出** | **Use Case / 使用场景** |
|--------------|--------|----------|
| `env_logger` | stderr, 带颜色 | Development / 开发调试、简单 CLI 工具 |
| `tracing-subscriber` | stderr, 格式化 | Production / 使用 `tracing` 的生产环境 |
| `syslog` | 系统 syslog | Linux 系统服务 |
| `tracing-journald` | systemd 日志 | systemd 管理的服务 |
| `tracing-appender` | 轮转日志文件 | Long-running / 长期运行的守护进程 |
| `tracing-opentelemetry` | OpenTelemetry 收集器 | Distributed / 分布式追踪 |
