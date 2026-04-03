[English Original](../en/ch17-4-logging-and-tracing-ecosystem.md)

## 日志与追踪：从 syslog/printf 到 `log` + `tracing`

> **你将学到：** Rust 的两层日志架构（外观层 + 后端层）、`log` 与 `tracing` crate、带有 Span（跨度）的结构化日志，以及这些如何取代 `printf`/`syslog` 调试。

- C++ 诊断代码通常使用 `printf`、`syslog` 或自定义日志框架。
- Rust 拥有一套标准化的两层日志架构：**外观层 (Facade)** crate （`log` 或 `tracing`）以及 **后端层 (Backend)**（即实际的日志记录器实现）。

### `log` 外观层 —— Rust 通用的日志 API

`log` crate 提供的宏与 syslog 的严重程度级别相对应。库（Libraries）通常使用 `log` 宏，而二进制程序（Binaries）则负责选择具体的后端实现：

```rust
// Cargo.toml
// [dependencies]
// log = "0.4"
// env_logger = "0.11"    # 众多后端中的一种

use log::{info, warn, error, debug, trace};

fn check_sensor(id: u32, temp: f64) {
    trace!("正在读取传感器 {id}");           // 最细粒度
    debug!("传感器 {id} 原始值: {temp}"); // 开发阶段的细节

    if temp > 85.0 {
        warn!("传感器 {id} 温度过高: {temp}°C");
    }
    if temp > 95.0 {
        error!("传感器 {id} 严重报警: {temp}°C —— 正在启动关机程序");
    }
    info!("传感器 {id} 检查完成");     // 正常运行信息
}

fn main() {
    // 初始化后端 —— 通常在 main() 中执行一次
    env_logger::init();  // 由 RUST_LOG 环境变量控制

    check_sensor(0, 72.5);
    check_sensor(1, 91.0);
---

```bash
# 通过环境变量控制日志级别
RUST_LOG=debug cargo run          # 显示 debug 及以上级别
RUST_LOG=warn cargo run           # 仅显示 warn 和 error
RUST_LOG=my_crate=trace cargo run # 针对特定模块进行过滤
RUST_LOG=my_crate::gpu=debug,warn cargo run  # 混合不同级别的过滤
```

### C++ 对比

| C++ | Rust (`log`) | 说明 |
|-----|-------------|-------|
| `printf("DEBUG: %s\n", msg)` | `debug!("{msg}")` | 在编译期检查格式 |
| `syslog(LOG_ERR, "...")` | `error!("...")` | 后端层决定日志输出的目的地 |
| 在 log 调用处使用 `#ifdef DEBUG` | 在 max_level 下，`trace!` / `debug!` 会在编译时被剔除 | 禁用时无运行时开销 |
| 自定义的 `Logger::log(level, msg)` | `log::info!("...")` —— 所有的 crate 均使用同一个 API | 通用外观，可更换后端 |
| 各文件的日志详细程度配置 | `RUST_LOG=crate::module=level` | 基于环境变量配置，无需重新编译 |

---

### `tracing` crate —— 带有跨度的结构化日志

`tracing` 是对 `log` 的扩展，加入了 **结构化字段 (Structured Fields)** 和 **Span（跨度，即带有计时信息的作用域）**。这在诊断代码中尤为实用，你可以跟踪具体的上下文：

```rust
// Cargo.toml
// [dependencies]
// tracing = "0.1"
// tracing-subscriber = { version = "0.3", features = ["env-filter"] }

use tracing::{info, warn, error, instrument, info_span};

#[instrument(skip(data), fields(gpu_id = gpu_id, data_len = data.len()))]
fn run_gpu_test(gpu_id: u32, data: &[u8]) -> Result<(), String> {
    info!("正在启动 GPU 测试");

    let span = info_span!("ecc_check", gpu_id);
    let _guard = span.enter();  // 此作用域内的所有日志均会自动带上 gpu_id

    if data.is_empty() {
        error!(gpu_id, "未提供测试数据");
        return Err("数据为空".to_string());
    }

    // 结构化字段 —— 机器可解析，不仅仅是字符串插值
    info!(
        gpu_id,
        temp_celsius = 72.5,
        ecc_errors = 0,
        "ECC 检查通过"
    );

    Ok(())
}

fn main() {
    // 初始化 tracing 记录器
    tracing_subscriber::fmt()
        .with_env_filter("debug")  // 或者使用 RUST_LOG 环境变量
        .with_target(true)          // 显示模块路径
        .with_thread_ids(true)      // 显示线程 ID
        .init();

    let _ = run_gpu_test(0, &[1, 2, 3]);
}
```

---

使用 `tracing-subscriber` 的输出示例：
```rust
2026-02-15T10:30:00.123Z DEBUG ThreadId(01) run_gpu_test{gpu_id=0 data_len=3}: my_crate: 正在启动 GPU 测试
2026-02-15T10:30:00.124Z  INFO ThreadId(01) run_gpu_test{gpu_id=0 data_len=3}:ecc_check{gpu_id=0}: my_crate: ECC 检查通过 gpu_id=0 temp_celsius=72.5 ecc_errors=0
```

### `#[instrument]` —— 自动创建 Span

`#[instrument]` 属性会自动创建一个以函数名命名的 Span，并将其参数作为结构化字段：

```rust
use tracing::instrument;

#[instrument]
fn parse_sel_record(record_id: u16, sensor_type: u8, data: &[u8]) -> Result<(), String> {
    // 此函数内的每一条日志都会自动包含：
    // record_id、sensor_type 以及 data（如果实现了 Debug）
    tracing::debug!("正在解析 SEL 记录");
    Ok(())
}

// skip: 在 Span 中排除大型或敏感的参数
// fields: 添加计算得出的字段
#[instrument(skip(raw_buffer), fields(buf_len = raw_buffer.len()))]
fn decode_ipmi_response(raw_buffer: &[u8]) -> Result<Vec<u8>, String> {
    tracing::trace!("正在解码 {} 字节", raw_buffer.len());
    Ok(raw_buffer.to_vec())
}
```

---

### `log` 对比 `tracing` —— 该选哪一个

| 维度 | `log` | `tracing` |
|--------|-------|-----------|
| **复杂程度** | 简单 —— 仅 5 个宏 | 更丰富 —— 包含 Span、字段、Instrument 属性 |
| **结构化数据** | 仅限字符串插值 | 键值字段：`info!(gpu_id = 0, "msg")` |
| **计时 / Span** | 否 | 是 —— `#[instrument]`, `span.enter()` |
| **异步支持** | 基础 | 一等公民 —— Span 可跨越 `.await` 进行传播 |
| **兼容性** | 通用的外观层 | 兼容 `log`（提供 `log` 桥接） |
| **适用场景** | 简单的应用、库 (Library) | 诊断工具、异步代码、可观测性相关项目 |

> **建议**：在生产级别的诊断类项目（即需要结构化输出的诊断工具）中使用 `tracing`。在希望依赖最小化的简单库中使用 `log`。`tracing` 包含一个兼容层，因此使用 `log` 宏的库仍然可以配合 `tracing` 记录器 (Subscriber) 工作。

### 后端选项

| 后端 Crate | 输出 | 适用场景 |
|--------------|--------|----------|
| `env_logger` | stderr，带颜色 | 开发阶段、简单的 CLI 工具 |
| `tracing-subscriber` | stderr，格式化输出 | 使用 `tracing` 的生产环境 |
| `syslog` | 系统 syslog | Linux 系统服务 |
| `tracing-journald` | systemd journal | 由 systemd 管理的服务 |
| `tracing-appender` | 轮转的日志文件 | 长期运行的守护进程 |
| `tracing-opentelemetry` | OpenTelemetry 收集器 | 分布式追踪 |

---
