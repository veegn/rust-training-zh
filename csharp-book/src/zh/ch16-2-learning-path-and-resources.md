[English Original](../en/ch16-2-learning-path-and-resources.md)

## 学习路线图与后续步骤

> **你将学到：** 结构化的学习路线图（第 1-2 周，第 1-3 个月及以后）；推荐的书籍与资源；C# 开发者常见的坑（所有权的困惑、与借用检查器“斗争”）；以及使用 `tracing` 与 `ILogger` 的结构化可观测性对比。
>
> **难度：** 🟢 入门

### 即刻开始（第 1-2 周）
1. **搭建环境**
   - 通过 [rustup.rs](https://rustup.rs/) 安装 Rust。
   - 配置带有 rust-analyzer 扩展插件的 VS Code。
   - 创建你的第一个 `cargo new hello_world` 项目。

2. **掌握基础**
   - 通过简单的练习实践所有权 (Ownership) 概念。
   - 编写带有不同参数类型（`&str`, `String`, `&mut`）的函数。
   - 实现基础的结构体及其方法。

3. **错误处理实操**
   - 将 C# 的 try-catch 代码转换为基于 Result 的模式。
   - 练习使用 `?` 运算符和 `match` 语句。
   - 实现自定义错误类型。

### 中期目标（第 1-2 个月）
1. **集合与迭代器**
   - 掌握 `Vec<T>`, `HashMap<K,V>` 以及 `HashSet<T>`。
   - 学习迭代方法：`map`, `filter`, `collect`, `fold` 等。
   - 练习对比 `for` 循环与迭代器链。

2. **特性与泛型**
   - 实现常用的特性：`Debug`, `Clone`, `PartialEq` 等。
   - 编写泛型函数和结构体。
   - 理解特性约束 (Trait bounds) 和 `where` 子句。

3. **项目结构组织**
   - 将代码组织到模块 (Modules) 中。
   - 理解 `pub` 可见性关键字。
   - 学习从 crates.io 使用外部 Crate。

### 进阶课题（第 3 个月及以后）
1. **并发编程**
   - 学习 `Send` 和 `Sync` 特性。
   - 使用 `std::thread` 进行基础的并行处理。
   - 探索用于异步编程的 `tokio`。

2. **内存管理**
   - 理解以此实现共享所有权的 `Rc<T>` 和 `Arc<T>`。
   - 学习何时使用 `Box<T>` 进行堆分配。
   - 掌握应对复杂场景的生命周期 (Lifetimes) 概念。

3. **实战项目**
   - 使用 `clap` 构建一个命令行工具 (CLI tool)。
   - 使用 `axum` 或 `warp` 创建一个 Web API。
   - 编写一个库并发布到 crates.io。

### 推荐的学习资源

#### 书籍
- **《Rust 程序设计语言》(The Rust Programming Language)** (在线免费阅读) —— 官方权威指南。
- **《通过例子学 Rust》(Rust by Example)** (在线免费阅读) —— 侧重于动手实践。
- **《Rust 权威指南》(Programming Rust)** (作者 Jim Blandy) —— 深度讲解底层技术。

#### 在线资源
- [Rust Playground](https://play.rust-lang.org/) —— 在浏览器中尝试代码。
- [Rustlings](https://github.com/rust-lang/rustlings) —— 通过交互式练习学习。
- [通过例子学 Rust](https://doc.rust-lang.org/rust-by-example/) —— 包含大量实际代码案例。

#### 实操建议项目
1. **命令行计算器** —— 练习枚举 (Enums) 和模式匹配。
2. **文件整理器** —— 操作文件系统并进行错误处理。
3. **JSON 处理器** —— 学习 serde 和数据转换。
4. **HTTP 服务器** —— 理解异步编程与网络通信。
5. **数据库访问库** —— 掌握特性、泛型以及错误处理。

### C# 开发者常见的坑

#### 1. 所有权困惑
```rust
// 错误示例：尝试使用已被移动的值
fn wrong_way() {
    let s = String::from("hello");
    takes_ownership(s);
    // println!("{}", s); // 错误：s 的所有权已被移动
}

// 正确示例：根据需要使用引用或克隆
fn right_way() {
    let s = String::from("hello");
    borrows_string(&s);
    println!("{}", s); // 正常：此处仍拥有 s 的所有权
}

fn takes_ownership(s: String) { /* s 所有权移动至此 */ }
fn borrows_string(s: &str) { /* s 被借用至此 */ }
```

#### 2. 与借用检查器“斗争”
```rust
// 错误示例：存在多个可变引用
fn wrong_borrowing() {
    let mut v = vec![1, 2, 3];
    let r1 = &mut v;
    // let r2 = &mut v; // 错误：无法多次进行可变借用
}

// 正确示例：限制可变借用的作用域
fn right_borrowing() {
    let mut v = vec![1, 2, 3];
    {
        let r1 = &mut v;
        r1.push(4);
    } // r1 在此处离开作用域并释放借用
    
    let r2 = &mut v; // 正常：当前不存在其他可变借用
    r2.push(5);
}
```

#### 3. 习惯性寻找 Null 值
```rust
// 错误示例：期待类似 null 的行为
fn no_null_in_rust() {
    // let s: String = null; // Rust 中没有 null！
}

// 正确示例：显式使用 Option<T>
fn use_option_instead() {
    let maybe_string: Option<String> = None;
    
    match maybe_string {
        Some(s) => println!("获取到字符串：{}", s),
        None => println!("当前没有字符串"),
    }
}
```

### 最后的建议

1. **拥抱编译器** —— Rust 的编译器报错是非常有帮助的提示，而非阻碍。
2. **从小处着手** —— 先从简单的程序开始，逐步增加复杂度。
3. **阅读开源代码** —— 在 GitHub 上研究流行的 Crate。
4. **积极寻求帮助** —— Rust 社区非常友好且乐于助人。
5. **勤加练习** —— 随着不断实践，Rust 的核心概念会变得自然而然。

请记住：Rust 确实存在一定的学习曲线，但它带来的内存安全、极致性能以及无畏并发是非常值得的。起初看似具有约束力的所有权系统，最终会成为你编写正确、高效程序的强大武器。

---

**恭喜你！** 你现在已经具备了从 C# 转型向 Rust 的坚实基础。请从简单的项目开始，保持耐心，逐步深入复杂的应用开发。Rust 带来的安全性和性能收益将证明你最初的学习投入是物超所值的。

---

## 结构化可观测性：`tracing` vs ILogger 和 Serilog

C# 开发者习惯于通过 `ILogger`, **Serilog** 或 **NLog** 进行**结构化日志记录** —— 日志消息中携带有类型的键值对属性。Rust 的 `log` crate 提供了基础的分级日志功能，但在生产环境中，**`tracing`** 才是结构化可观测性的标准方案，它支持 Span（跨度）、异步感知以及分布式追踪。

### 为什么选择 `tracing` 而非 `log`

| 特性 | `log` crate | `tracing` crate | C# 对应项 |
|---------|------------|-----------------|----------------|
| 日志级别消息 | ✅ `info!()`, `error!()` | ✅ `info!()`, `error!()` | `ILogger.LogInformation()` |
| 结构化字段 | ❌ 仅支持字符串插值 | ✅ 带有类型的键值字段 | Serilog `Log.Information("{User}", user)` |
| Span (作用域上下文) | ❌ | ✅ `#[instrument]`, `span!()` | `ILogger.BeginScope()` |
| 异步感知 | ❌ 跨 `.await` 会丢失上下文 | ✅ Span 可以跨越 `.await` 传递 | `Activity` / `DiagnosticSource` |
| 分布式追踪 | ❌ | ✅ 支持 OpenTelemetry 集成 | `System.Diagnostics.Activity` |
| 多种输出格式 | 基础 | 支持 JSON, Pretty, Compact, OTLP | Serilog Sinks |

### 开始使用
```toml
# Cargo.toml
[dependencies]
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter", "json"] }
```

### 基本用法：结构化日志
```csharp
// C# Serilog
Log.Information("正在为客户 {Customer} 处理订单 {OrderId}, 总金额 {Total:C}",
    orderId, customer.Name, order.Total);
// 输出：正在为客户 Alice 处理订单 12345, 总金额 $99.95
// JSON:  {"OrderId": 12345, "Customer": "Alice", "Total": 99.95, ...}
```

```rust
use tracing::{info, warn, error, debug, instrument};

// 结构化字段 —— 是带有类型的，而非简单的字符串插值
info!(order_id = 12345, customer = "Alice", total = 99.95,
      "正在处理订单");
// 输出：INFO 正在处理订单 order_id=12345 customer="Alice" total=99.95
// JSON:  {"order_id": 12345, "customer": "Alice", "total": 99.95, ...}

// 动态数值
let order_id = 12345;
info!(order_id, "已收到订单");  // 字段名 = 变量名 的简写形式

// 条件化字段
if let Some(promo) = promo_code {
    info!(order_id, promo_code = %promo, "已应用优惠码");
    //                        ^ % 表示使用 Display 格式化
    //                        ? 则表示使用 Debug 格式化
}
```

### Span：异步代码的杀手锏特性

Span 是能够跨函数调用和 `.await` 点传递字段的作用域上下文 —— 类似于 `ILogger.BeginScope()` 但它是异步安全的。

```csharp
// C# — Activity / BeginScope
using var activity = new Activity("ProcessOrder").Start();
activity.SetTag("order_id", orderId);

using (_logger.BeginScope(new Dictionary<string, object> { ["OrderId"] = orderId }))
{
    _logger.LogInformation("开始处理");
    await ProcessPaymentAsync();
    _logger.LogInformation("支付完成");  // OrderId 仍在作用域内
}
```

```rust
use tracing::{info, instrument, Instrument};

// #[instrument] 自动创建一个 Span，并将函数参数作为字段
#[instrument(skip(db), fields(customer_name))]
async fn process_order(order_id: u64, db: &Database) -> Result<(), AppError> {
    let order = db.get_order(order_id).await?;
    
    // 动态地向当前 Span 添加字段
    tracing::Span::current().record("customer_name", &order.customer_name.as_str());
    
    info!("开始处理");
    process_payment(&order).await?;        // Span 上下文在跨越 .await 时被保留！
    info!(items = order.items.len(), "支付完成");
    Ok(())
}
// 此函数内部的每一条日志消息都会自动包含：
//   order_id=12345 customer_name="Alice"
// 即便是在嵌套的异步调用中也同样有效！

// 手动创建 Span (类似于 BeginScope)
async fn batch_process(orders: Vec<u64>, db: &Database) {
    for order_id in orders {
        let span = tracing::info_span!("process_order", order_id);
        
        // .instrument(span) 将 Span 附加到 Future 上
        process_order(order_id, db)
            .instrument(span)
            .await
            .unwrap_or_else(|e| error!("失败：{e}"));
    }
}
```

### 订阅者配置 (类似于 Serilog Sinks)

```rust
use tracing_subscriber::{fmt, EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};

fn init_tracing() {
    // 开发环境：易读的、彩色的控制台输出
    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| "my_app=debug,tower_http=info".into()))
        .with(fmt::layer().pretty())  // 带有颜色和缩进的 Span
        .init();
}

fn init_tracing_production() {
    // 生产环境：用于日志聚合的 JSON 输出 (类似于 Serilog 的 JSON Sink)
    tracing_subscriber::registry()
        .with(EnvFilter::new("my_app=info"))
        .with(fmt::layer().json())  // 结构化 JSON
        .init();
    // 输出：{"timestamp":"...","level":"INFO","fields":{"order_id":123},...}
}
```

```bash
# 通过环境变量控制日志级别 (类似于 Serilog 的 MinimumLevel)
RUST_LOG=my_app=debug,hyper=warn cargo run
RUST_LOG=trace cargo run  # 输出全量日志
```

### Serilog → tracing 迁移速查表

| Serilog / ILogger | tracing | 备注 |
|-------------------|---------|-------|
| `Log.Information("{Key}", val)` | `info!(key = val, "消息内容")` | 字段是带类型的，而非简单的文本插值 |
| `Log.ForContext("Key", val)` | `span.record("key", val)` | 向当前 Span 添加字段 |
| `using BeginScope(...)` | `#[instrument]` 或 `info_span!()` | 使用 `#[instrument]` 可自动实现 |
| `.WriteTo.Console()` | `fmt::layer()` | 人类易读格式 |
| `.WriteTo.Seq()` / `.File()` | `fmt::layer().json()` + 文件重定向 | 或使用 `tracing-appender` |
| `.Enrich.WithProperty()` | `span!(Level::INFO, "name", key = val)` | Span 字段 |
| `LogEventLevel.Debug` | `tracing::Level::DEBUG` | 概念相同 |
| `{@Object}` 自省式解构 | `field = ?value` (Debug) 或 `%value` (Display) | `?` 表示 Debug, `%` 表示 Display |

### OpenTelemetry 集成
```toml
# 用于分布式追踪 (类似于 System.Diagnostics + OTLP 导出器)
[dependencies]
tracing-opentelemetry = "0.22"
opentelemetry = "0.21"
opentelemetry-otlp = "0.14"
```

```rust
// 在控制台输出的基础上添加 OpenTelemetry 层
use tracing_opentelemetry::OpenTelemetryLayer;

fn init_otel() {
    let tracer = opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(opentelemetry_otlp::new_exporter().tonic())
        .install_batch(opentelemetry_sdk::runtime::Tokio)
        .expect("创建 OTLP Tracer 失败");

    tracing_subscriber::registry()
        .with(OpenTelemetryLayer::new(tracer))  // 发送 Span 到 Jaeger/Tempo
        .with(fmt::layer())                      // 同时打印到控制台
        .init();
}
// 现在 #[instrument] 创建的 Span 会自动转换为分布式追踪数据！
```
