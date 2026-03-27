## Logging and Tracing: syslog/printf → `log` + `tracing` / 日志与追踪：syslog/printf → `log` + `tracing`
 
 > **What you'll learn / 你将学到：** Rust's two-layer logging architecture (facade + backend), the `log` and `tracing` crates, structured logging with spans, and how this replaces `printf`/`syslog` debugging.
 >
 > Rust 的两层日志架构（门面 + 后端）、`log` 和 `tracing` crate、带有 Span 的结构化日志，以及这些工具如何替代 `printf`/`syslog` 调试。
 
- C++ diagnostic code typically uses `printf`, `syslog`, or custom logging frameworks.
+ C++ 诊断代码通常使用 `printf`、`syslog` 或自定义日志框架。
- Rust has a standardized two-layer logging architecture: a **facade** crate (`log` or
+ Rust 具有标准化的两层日志架构：一个**门面（Facade）** crate（`log` 或
- `tracing`) and a **backend** (the actual logger implementation).
+ `tracing`）和一个**后端（Backend）**（即实际的日志记录器实现）。
 
- ### The `log` facade — Rust's universal logging API
+ ### The `log` facade — Rust's universal logging API / `log` 门面 —— Rust 通用日志 API
 
- The `log` crate provides macros that mirror syslog severity levels. Libraries use
+ `log` crate 提供的宏镜像了 syslog 的严重程度级别。库使用
- `log` macros; binaries choose a backend:
+ `log` 宏；二进制程序则通过选择一个后端来实现：
 
 ```rust
 // Cargo.toml
 // [dependencies]
 // log = "0.4"
 // env_logger = "0.11"    # One of many backends
+// env_logger = "0.11"    # 众多后端之一
 
- use log::{info, warn, error, debug, trace};
+ use log::{info, warn, error, debug, trace}; // 导入不同级别的宏
 
 fn check_sensor(id: u32, temp: f64) {
-    trace!("Reading sensor {id}");           // Finest granularity
+    trace!("Reading sensor {id}");           // Trace / 最细粒度
-    debug!("Sensor {id} raw value: {temp}"); // Development-time detail
+    debug!("Sensor {id} raw value: {temp}"); // Debug / 开发阶段详情
 
     if temp > 85.0 {
-        warn!("Sensor {id} high temperature: {temp}°C");
+        warn!("Sensor {id} high temperature: {temp}°C"); // Warn / 警告
     }
     if temp > 95.0 {
-        error!("Sensor {id} CRITICAL: {temp}°C — initiating shutdown");
+        error!("Sensor {id} CRITICAL: {temp}°C — initiating shutdown"); // Error / 错误
     }
-    info!("Sensor {id} check complete");     // Normal operation
+    info!("Sensor {id} check complete");     // Info / 正常运行信息
 }
 
 fn main() {
-    // Initialize the backend — typically done once in main()
+    // Initialize / 初始化后端 —— 通常在 main() 中执行一次
-    env_logger::init();  // Controlled by RUST_LOG env var
+    env_logger::init();  // Controlled / 由 RUST_LOG 环境变量控制
 
     check_sensor(0, 72.5);
     check_sensor(1, 91.0);
 }
 ```
 
 ```bash
- # Control log level via environment variable
+ # 通过环境变量控制日志级别
- RUST_LOG=debug cargo run          # Show debug and above
+ RUST_LOG=debug cargo run          # 显示 debug 及以上级别
- RUST_LOG=warn cargo run           # Show only warn and error
+ RUST_LOG=warn cargo run           # 仅显示 warn 和 error
- RUST_LOG=my_crate=trace cargo run # Per-module filtering
+ RUST_LOG=my_crate=trace cargo run # 针对特定模块进行过滤
- RUST_LOG=my_crate::gpu=debug,warn cargo run  # Mix levels
+ RUST_LOG=my_crate::gpu=debug,warn cargo run  # 混合级别配置
 ```
 
- ### C++ comparison
+ ### C++ comparison / C++ 对比
 
-| C++ | Rust (`log`) | Notes |
+| **C++** | **Rust (`log`)** | **Notes / 说明** |
 |-----|-------------|-------|
-| `printf("DEBUG: %s\n", msg)` | `debug!("{msg}")` | Format checked at compile time |
+| `printf("DEBUG: %s\n", msg)` | `debug!("{msg}")` | Compile-time check / 编译时检查格式 |
+| `syslog(LOG_ERR, "...")` | `error!("...")` | Backend decides output / 后端决定输出去向 |
+| `#ifdef DEBUG` around log calls | `trace!` / `debug!` compiled out at max_level | Zero-cost when disabled / 禁用时零成本 |
+| Custom `Logger::log(level, msg)` | `log::info!("...")` — all crates use same API | Universal facade / 通用门面，可插拔后端 |
+| Per-file log verbosity | `RUST_LOG=crate::module=level` | Env-based / 基于环境配置，无需重新编译 |
 
- ### The `tracing` crate — structured logging with spans
+ ### The `tracing` crate — structured logging with spans / `tracing` crate —— 带有 Span 的结构化日志
 
- `tracing` extends `log` with **structured fields** and **spans** (timed scopes).
+ `tracing` 通过**结构化字段**和 **Span（具有时间属性的作用域）**扩展了 `log` 的功能。
- This is especially useful for diagnostics code where you want to track context:
+ 这对于需要跟踪上下文的诊断代码特别有用：
 
 ```rust
 // Cargo.toml
 // [dependencies]
 // tracing = "0.1"
 // tracing-subscriber = { version = "0.3", features = ["env-filter"] }
 
 use tracing::{info, warn, error, instrument, info_span};
 
- #[instrument(skip(data), fields(gpu_id = gpu_id, data_len = data.len()))]
+ #[instrument(skip(data), fields(gpu_id = gpu_id, data_len = data.len()))] // 自动创建 Span
 fn run_gpu_test(gpu_id: u32, data: &[u8]) -> Result<(), String> {
     info!("Starting GPU test");
 
-    let span = info_span!("ecc_check", gpu_id);
+    let span = info_span!("ecc_check", gpu_id); // 手动创建 Span
-    let _guard = span.enter();  // All logs inside this scope include gpu_id
+    let _guard = span.enter();  // Guard / 此作用域内的所有日志都会包含 gpu_id
 
     if data.is_empty() {
         error!(gpu_id, "No test data provided");
         return Err("empty data".to_string());
     }
 
-    // Structured fields — machine-parseable, not just string interpolation
+    // Structured fields / 结构化字段 —— 机器可解析，而不只是字符串插值
     info!(
         gpu_id,
         temp_celsius = 72.5,
         ecc_errors = 0,
         "ECC check passed"
     );
 
     Ok(())
 }
 
 fn main() {
-    // Initialize tracing subscriber
+    // Initialize / 初始化 tracing 订阅者
     tracing_subscriber::fmt()
-        .with_env_filter("debug")  // Or use RUST_LOG env var
+        .with_env_filter("debug")  // Filter / 或使用 RUST_LOG 环境变量
-        .with_target(true)          // Show module path
+        .with_target(true)          // Target / 显示模块路径
-        .with_thread_ids(true)      // Show thread IDs
+        .with_thread_ids(true)      // Thread ID / 显示线程 ID
         .init();
 
     let _ = run_gpu_test(0, &[1, 2, 3]);
 }
 ```
 
- Output with `tracing-subscriber`:
+ 使用 `tracing-subscriber` 的输出示例：
 ```rust
 2026-02-15T10:30:00.123Z DEBUG ThreadId(01) run_gpu_test{gpu_id=0 data_len=3}: my_crate: Starting GPU test
 2026-02-15T10:30:00.124Z  INFO ThreadId(01) run_gpu_test{gpu_id=0 data_len=3}:ecc_check{gpu_id=0}: my_crate: ECC check passed gpu_id=0 temp_celsius=72.5 ecc_errors=0
 ```
 
- ### `#[instrument]` — automatic span creation
+ ### `#[instrument]` — automatic span creation / `#[instrument]` —— 自动创建 Span
 
- The `#[instrument]` attribute automatically creates a span with the function name
+ `#[instrument]` 属性会自动创建一个包含函数名及其参数的 Span：
- and its arguments:
 
 ```rust
 use tracing::instrument;
 
- #[instrument]
+ #[instrument] // 自动记录参数
 fn parse_sel_record(record_id: u16, sensor_type: u8, data: &[u8]) -> Result<(), String> {
-    // Every log inside this function automatically includes:
+    // Included / 此函数内的每条日志都会自动包含：
-    // record_id, sensor_type, and data (if Debug)
+    // record_id, sensor_type 以及 data (如果实现了 Debug)
     tracing::debug!("Parsing SEL record");
     Ok(())
 }
 
- // skip: exclude large/sensitive args from the span
+ // skip: / 跳过：从 Span 中排除大型或敏感参数
- // fields: add computed fields
+ // fields: / 字段：添加计算出的字段
 #[instrument(skip(raw_buffer), fields(buf_len = raw_buffer.len()))]
 fn decode_ipmi_response(raw_buffer: &[u8]) -> Result<Vec<u8>, String> {
     tracing::trace!("Decoding {} bytes", raw_buffer.len());
     Ok(raw_buffer.to_vec())
 }
 ```
 
- ### `log` vs `tracing` — which to use
+ ### `log` vs `tracing` — which to use / `log` vs `tracing` —— 应该使用哪一个
 
-| Aspect | `log` | `tracing` |
+| **Aspect / 维度** | **`log`** | **`tracing`** |
 |--------|-------|-----------|
-| **Complexity** | Simple — 5 macros | Richer — spans, fields, instruments |
-| **Complexity / 复杂度** | Simple / 简单 —— 只有 5 个宏 | Richer / 丰富 —— 包含 Span、字段和 instrument |
-| **Structured data** | String interpolation only | Key-value fields: `info!(gpu_id = 0, "msg")` |
-| **Structured / 结构化** | String only / 仅字符串插值 | Key-value / 键值字段：`info!(gpu_id = 0, "msg")` |
-| **Timing / spans** | No | Yes — `#[instrument]`, `span.enter()` |
-| **Timing / Span** | ❌ 否 | ✅ 是 —— `#[instrument]`、`span.enter()` |
-| **Async support** | Basic | First-class — spans propagate across `.await` |
-| **Async / 异步支持** | Basic / 基础 | First-class / 一流支持 —— Span 可跨越 `.await` 传播 |
-| **Compatibility** | Universal facade | Compatible with `log` (has a `log` bridge) |
-| **Compatibility / 兼容性** | Universal / 通用门面 | Compatible / 通过桥接兼容 `log` |
-| **When to use** | Simple applications, libraries | Diagnostic tools, async code, observability |
-| **When to use / 场景** | Simple / 简单应用或库 | Tools / 诊断工具、异步代码、可观测性需求 |
 
- > **Recommendation**: Use `tracing` for production diagnostic-style projects (diagnostic tools
+ > **建议**：对于生产环境中的诊断类项目（具有结构化输出需求的诊断工具），请使用 `tracing`。对于希望最小化依赖的简单库，请使用 `log`。`tracing` 包含一个兼容层，因此使用 `log` 宏的库仍然可以与 `tracing` 订阅者配合工作。
- > with structured output). Use `log` for simple libraries where you want minimal
- > dependencies. `tracing` includes a compatibility layer so libraries using `log`
- > macros still work with a `tracing` subscriber.
 
- ### Backend options
+ ### Backend options / 后端可选项
 
-| Backend Crate | Output | Use Case |
+| **Backend Crate / 后端 Crate** | **Output / 输出** | **Use Case / 使用场景** |
 |--------------|--------|----------|
-| `env_logger` | stderr, colored | Development, simple CLI tools |
-| `env_logger` | stderr, 带颜色 | Development / 开发调试、简单 CLI 工具 |
-| `tracing-subscriber` | stderr, formatted | Production with `tracing` |
-| `tracing-subscriber` | stderr, 格式化 | Production / 使用 `tracing` 的生产环境 |
-| `syslog` | System syslog | Linux system services |
-| `syslog` | 系统 syslog | Linux 系统服务 |
-| `tracing-journald` | systemd journal | systemd-managed services |
-| `tracing-journald` | systemd 日志 | systemd 管理的服务 |
-| `tracing-appender` | Rotating log files | Long-running daemons |
-| `tracing-appender` | 轮转日志文件 | Long-running / 长期运行的守护进程 |
-| `tracing-opentelemetry` | OpenTelemetry collector | Distributed tracing |
-| `tracing-opentelemetry` | OpenTelemetry 收集器 | Distributed / 分布式追踪 |
