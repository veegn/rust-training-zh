## Learning Path and Next Steps | 学习路径与下一步

> **What you'll learn:** A structured learning roadmap (weeks 1-2, months 1-3+), recommended books and resources,
> common pitfalls for C# developers (ownership confusion, fighting the borrow checker),
> and structured observability with `tracing` vs `ILogger`.
>
> **你将学到什么：** 一条结构化的学习路线图（第 1-2 周、1-3+ 月）、推荐书籍与资源，
> C# 开发者常见的坑（所有权困惑、和借用检查器“对抗”），
> 以及 `tracing` 与 `ILogger` 的结构化可观测性对比。
>
> **Difficulty:** Beginner
>
> **难度：** 初级

### Immediate Next Steps (Week 1-2) | 立即开始的下一步（第 1-2 周）
1. **Set up your environment**
1. **搭建开发环境**
   - Install Rust via [rustup.rs](https://rustup.rs/)
   - 通过 [rustup.rs](https://rustup.rs/) 安装 Rust
   - Configure VS Code with rust-analyzer extension
   - 在 VS Code 中配置 rust-analyzer 扩展
   - Create your first `cargo new hello_world` project
   - 创建第一个 `cargo new hello_world` 项目

2. **Master the basics**
2. **掌握基础**
   - Practice ownership with simple exercises
   - 通过简单练习熟悉所有权
   - Write functions with different parameter types (`&str`, `String`, `&mut`)
   - 练习不同参数类型的函数（`&str`、`String`、`&mut`）
   - Implement basic structs and methods
   - 实现基础的结构体与方法

3. **Error handling practice**
3. **练习错误处理**
   - Convert C# try-catch code to Result-based patterns
   - 把 C# 的 try-catch 代码改写成基于 Result 的模式
   - Practice with `?` operator and `match` statements
   - 练习使用 `?` 操作符和 `match`
   - Implement custom error types
   - 实现自定义错误类型

### Intermediate Goals (Month 1-2) | 中期目标（第 1-2 个月）
1. **Collections and iterators**
1. **集合与迭代器**
   - Master `Vec<T>`, `HashMap<K,V>`, and `HashSet<T>`
   - 熟练掌握 `Vec<T>`、`HashMap<K,V>` 和 `HashSet<T>`
   - Learn iterator methods: `map`, `filter`, `collect`, `fold`
   - 学习迭代器方法：`map`、`filter`、`collect`、`fold`
   - Practice with `for` loops vs iterator chains
   - 对比练习 `for` 循环与迭代器链

2. **Traits and generics**
2. **Trait 与泛型**
   - Implement common traits: `Debug`, `Clone`, `PartialEq`
   - 实现常见 trait：`Debug`、`Clone`、`PartialEq`
   - Write generic functions and structs
   - 编写泛型函数和泛型结构体
   - Understand trait bounds and where clauses
   - 理解 trait bound 和 `where` 子句

3. **Project structure**
3. **项目结构**
   - Organize code into modules
   - 学会用模块组织代码
   - Understand `pub` visibility
   - 理解 `pub` 可见性
   - Work with external crates from crates.io
   - 学会使用 crates.io 上的外部 crate

### Advanced Topics (Month 3+) | 进阶主题（第 3 个月以后）
1. **Concurrency**
1. **并发**
   - Learn about `Send` and `Sync` traits
   - 理解 `Send` 与 `Sync`
   - Use `std::thread` for basic parallelism
   - 使用 `std::thread` 做基础并行
   - Explore `tokio` for async programming
   - 学习用 `tokio` 做异步编程

2. **Memory management**
2. **内存管理**
   - Understand `Rc<T>` and `Arc<T>` for shared ownership
   - 理解共享所有权中的 `Rc<T>` 与 `Arc<T>`
   - Learn when to use `Box<T>` for heap allocation
   - 学会什么时候该用 `Box<T>` 做堆分配
   - Master lifetimes for complex scenarios
   - 在复杂场景中真正掌握生命周期

3. **Real-world projects**
3. **真实项目**
   - Build a CLI tool with `clap`
   - 用 `clap` 做一个 CLI 工具
   - Create a web API with `axum` or `warp`
   - 用 `axum` 或 `warp` 写一个 Web API
   - Write a library and publish to crates.io
   - 写一个库并发布到 crates.io

### Recommended Learning Resources | 推荐学习资源

#### Books | 书籍
- **"The Rust Programming Language"** (free online) - The official book
- **《The Rust Programming Language》**（可免费在线阅读）- 官方教材
- **"Rust by Example"** (free online) - Hands-on examples
- **《Rust by Example》**（可免费在线阅读）- 动手导向示例
- **"Programming Rust"** by Jim Blandy - Deep technical coverage
- **Jim Blandy 的《Programming Rust》** - 更深入的技术讲解

#### Online Resources | 在线资源
- [Rust Playground](https://play.rust-lang.org/) - Try code in browser
- [Rust Playground](https://play.rust-lang.org/) - 在浏览器里直接试代码
- [Rustlings](https://github.com/rust-lang/rustlings) - Interactive exercises
- [Rustlings](https://github.com/rust-lang/rustlings) - 交互式练习
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/) - Practical examples
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/) - 实用示例集合

#### Practice Projects | 练手项目
1. **Command-line calculator** - Practice with enums and pattern matching
1. **命令行计算器** - 练习枚举和模式匹配
2. **File organizer** - Work with filesystem and error handling
2. **文件整理工具** - 练习文件系统与错误处理
3. **JSON processor** - Learn serde and data transformation
3. **JSON 处理器** - 学习 serde 与数据转换
4. **HTTP server** - Understand async programming and networking
4. **HTTP 服务** - 理解异步编程与网络
5. **Database library** - Master traits, generics, and error handling
5. **数据库库项目** - 熟悉 trait、泛型与错误处理

### Common Pitfalls for C# Developers | C# 开发者常见陷阱

#### Ownership Confusion | 所有权困惑
```rust
// DON'T: Trying to use moved values
fn wrong_way() {
    let s = String::from("hello");
    takes_ownership(s);
    // println!("{}", s); // ERROR: s was moved
}

// DO: Use references or clone when needed
fn right_way() {
    let s = String::from("hello");
    borrows_string(&s);
    println!("{}", s); // OK: s is still owned here
}

fn takes_ownership(s: String) { /* s is moved here */ }
fn borrows_string(s: &str) { /* s is borrowed here */ }
```

#### Fighting the Borrow Checker | 和借用检查器“对抗”
```rust
// DON'T: Multiple mutable references
fn wrong_borrowing() {
    let mut v = vec![1, 2, 3];
    let r1 = &mut v;
    // let r2 = &mut v; // ERROR: cannot borrow as mutable more than once
}

// DO: Limit scope of mutable borrows
fn right_borrowing() {
    let mut v = vec![1, 2, 3];
    {
        let r1 = &mut v;
        r1.push(4);
    } // r1 goes out of scope here
    
    let r2 = &mut v; // OK: no other mutable borrows exist
    r2.push(5);
}
```

#### Expecting Null Values | 期待出现 null
```rust
// DON'T: Expecting null-like behavior
fn no_null_in_rust() {
    // let s: String = null; // NO null in Rust!
}

// DO: Use Option<T> explicitly
fn use_option_instead() {
    let maybe_string: Option<String> = None;
    
    match maybe_string {
        Some(s) => println!("Got string: {}", s),
        None => println!("No string available"),
    }
}
```

### Final Tips | 最后建议

1. **Embrace the compiler** - Rust's compiler errors are helpful, not hostile
1. **拥抱编译器** - Rust 编译器报错是在帮你，不是在跟你作对
2. **Start small** - Begin with simple programs and gradually add complexity
2. **从小处开始** - 先写简单程序，再逐步增加复杂度
3. **Read other people's code** - Study popular crates on GitHub
3. **多读别人的代码** - 去研究 GitHub 上流行 crate 的实现
4. **Ask for help** - The Rust community is welcoming and helpful
4. **多提问** - Rust 社区整体很友好
5. **Practice regularly** - Rust's concepts become natural with practice
5. **持续练习** - Rust 的很多概念都需要靠重复练习变自然

Remember: Rust has a learning curve, but it pays off with memory safety, performance, and fearless concurrency. The ownership system that seems restrictive at first becomes a powerful tool for writing correct, efficient programs.

要记住：Rust 确实有学习曲线，但它换来的是内存安全、性能和更可靠的并发。那些一开始看似“限制很多”的所有权规则，最终会变成帮助你写出正确高效程序的强大工具。

---

**Congratulations!** You now have a solid foundation for transitioning from C# to Rust. Start with simple projects, be patient with the learning process, and gradually work your way up to more complex applications. The safety and performance benefits of Rust make the initial learning investment worthwhile.

**恭喜你！** 你现在已经具备了从 C# 过渡到 Rust 的坚实基础。先从简单项目开始，对学习曲线保持耐心，再逐步进入更复杂的应用场景。Rust 在安全性和性能上的收益，值得这段前期投入。

## Structured Observability: `tracing` vs ILogger and Serilog | 结构化可观测性：`tracing` vs `ILogger` / Serilog

C# developers are accustomed to **structured logging** via `ILogger`, **Serilog**, or **NLog** - where log messages carry typed key-value properties. Rust's `log` crate provides basic leveled logging, but **`tracing`** is the production standard for structured observability with spans, async awareness, and distributed tracing support.

C# 开发者通常已经习惯用 `ILogger`、**Serilog** 或 **NLog** 做**结构化日志**，也就是让日志消息附带类型化的键值字段。Rust 的 `log` crate 只提供基础的分级日志，而 **`tracing`** 才是生产环境中做结构化可观测性的主流方案，它支持 span、异步上下文和分布式追踪。

### Why `tracing` Over `log` | 为什么优先用 `tracing` 而不是 `log`

| Feature | `log` crate | `tracing` crate | C# Equivalent |
|---------|------------|-----------------|----------------|
| Leveled messages | `info!()`, `error!()` | `info!()`, `error!()` | `ILogger.LogInformation()` |
| 分级日志 | `info!()`、`error!()` | `info!()`、`error!()` | `ILogger.LogInformation()` |
| Structured fields | String interpolation only | Typed key-value fields | Serilog `Log.Information("{User}", user)` |
| 结构化字段 | 基本依赖字符串插值 | 类型化键值字段 | Serilog `Log.Information("{User}", user)` |
| Spans (scoped context) | - | `#[instrument]`, `span!()` | `ILogger.BeginScope()` |
| Span（作用域上下文） | - | `#[instrument]`、`span!()` | `ILogger.BeginScope()` |
| Async-aware | Loses context across `.await` | Spans follow across `.await` | `Activity` / `DiagnosticSource` |
| 异步感知 | 跨 `.await` 时容易丢上下文 | span 能跨 `.await` 延续 | `Activity` / `DiagnosticSource` |
| Distributed tracing | - | OpenTelemetry integration | `System.Diagnostics.Activity` |
| 分布式追踪 | - | 可对接 OpenTelemetry | `System.Diagnostics.Activity` |
| Multiple output formats | Basic | JSON, pretty, compact, OTLP | Serilog sinks |
| 多输出格式 | 基础 | JSON、pretty、compact、OTLP | Serilog sinks |

### Getting Started | 快速开始
```toml
# Cargo.toml
[dependencies]
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter", "json"] }
```

### Basic Usage: Structured Logging | 基础用法：结构化日志
```csharp
// C# Serilog
Log.Information("Processing order {OrderId} for {Customer}, total {Total:C}",
    orderId, customer.Name, order.Total);
// Output: Processing order 12345 for Alice, total $99.95
// JSON:  {"OrderId": 12345, "Customer": "Alice", "Total": 99.95, ...}
```

```rust
use tracing::{info, warn, error, debug, instrument};

// Structured fields - typed, not string-interpolated
info!(order_id = 12345, customer = "Alice", total = 99.95,
      "Processing order");
// Output: INFO Processing order order_id=12345 customer="Alice" total=99.95
// JSON:  {"order_id": 12345, "customer": "Alice", "total": 99.95, ...}

// Dynamic values
let order_id = 12345;
info!(order_id, "Order received");  // field name = variable name shorthand

// Conditional fields
if let Some(promo) = promo_code {
    info!(order_id, promo_code = %promo, "Promo applied");
    //                        ^ % means use Display formatting
    //                        ? would use Debug formatting
}
```

### Spans: The Killer Feature for Async Code | Span：异步代码里的关键能力

Spans are scoped contexts that carry fields across function calls and `.await` points - like `ILogger.BeginScope()` but async-safe.

Span 是一种作用域上下文，可以把字段信息跨函数调用、跨 `.await` 一直带下去。你可以把它理解成异步安全版的 `ILogger.BeginScope()`。

```csharp
// C# - Activity / BeginScope
using var activity = new Activity("ProcessOrder").Start();
activity.SetTag("order_id", orderId);

using (_logger.BeginScope(new Dictionary<string, object> { ["OrderId"] = orderId }))
{
    _logger.LogInformation("Starting processing");
    await ProcessPaymentAsync();
    _logger.LogInformation("Payment complete");  // OrderId still in scope
}
```

```rust
use tracing::{info, instrument, Instrument};

// #[instrument] automatically creates a span with function args as fields
#[instrument(skip(db), fields(customer_name))]
async fn process_order(order_id: u64, db: &Database) -> Result<(), AppError> {
    let order = db.get_order(order_id).await?;
    
    // Add a field to the current span dynamically
    tracing::Span::current().record("customer_name", &order.customer_name.as_str());
    
    info!("Starting processing");
    process_payment(&order).await?;        // span context preserved across .await!
    info!(items = order.items.len(), "Payment complete");
    Ok(())
}
// Every log message inside this function automatically includes:
//   order_id=12345 customer_name="Alice"
// Even in nested async calls!

// Manual span creation (like BeginScope)
async fn batch_process(orders: Vec<u64>, db: &Database) {
    for order_id in orders {
        let span = tracing::info_span!("process_order", order_id);
        
        // .instrument(span) attaches the span to the future
        process_order(order_id, db)
            .instrument(span)
            .await
            .unwrap_or_else(|e| error!("Failed: {e}"));
    }
}
```

### Subscriber Configuration (Like Serilog Sinks) | Subscriber 配置（类似 Serilog Sink）

```rust
use tracing_subscriber::{fmt, EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};

fn init_tracing() {
    // Development: human-readable, colored output
    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| "my_app=debug,tower_http=info".into()))
        .with(fmt::layer().pretty())  // Colored, indented spans
        .init();
}

fn init_tracing_production() {
    // Production: JSON output for log aggregation (like Serilog JSON sink)
    tracing_subscriber::registry()
        .with(EnvFilter::new("my_app=info"))
        .with(fmt::layer().json())  // Structured JSON
        .init();
    // Output: {"timestamp":"...","level":"INFO","fields":{"order_id":123},...}
}
```

```bash
# Control log levels via environment variable (like Serilog MinimumLevel)
RUST_LOG=my_app=debug,hyper=warn cargo run
RUST_LOG=trace cargo run  # everything
```

### Serilog -> tracing Migration Cheat Sheet | Serilog -> tracing 迁移速查表

| Serilog / ILogger | tracing | Notes |
|-------------------|---------|-------|
| `Log.Information("{Key}", val)` | `info!(key = val, "message")` | Fields are typed, not interpolated |
| `Log.Information("{Key}", val)` | `info!(key = val, "message")` | 字段是类型化的，不只是字符串插值 |
| `Log.ForContext("Key", val)` | `span.record("key", val)` | Add fields to current span |
| `Log.ForContext("Key", val)` | `span.record("key", val)` | 给当前 span 添加字段 |
| `using BeginScope(...)` | `#[instrument]` or `info_span!()` | Automatic with `#[instrument]` |
| `using BeginScope(...)` | `#[instrument]` 或 `info_span!()` | `#[instrument]` 最省事 |
| `.WriteTo.Console()` | `fmt::layer()` | Human-readable |
| `.WriteTo.Console()` | `fmt::layer()` | 人类可读输出 |
| `.WriteTo.Seq()` / `.File()` | `fmt::layer().json()` + file redirect | Or use `tracing-appender` |
| `.WriteTo.Seq()` / `.File()` | `fmt::layer().json()` + 文件重定向 | 或配合 `tracing-appender` |
| `.Enrich.WithProperty()` | `span!(Level::INFO, "name", key = val)` | Span fields |
| `.Enrich.WithProperty()` | `span!(Level::INFO, "name", key = val)` | 通过 span 附加字段 |
| `LogEventLevel.Debug` | `tracing::Level::DEBUG` | Same concept |
| `LogEventLevel.Debug` | `tracing::Level::DEBUG` | 概念一致 |
| `{@Object}` destructuring | `field = ?value` (Debug) or `%value` (Display) | `?` = Debug, `%` = Display |
| `{@Object}` 解构输出 | `field = ?value`（Debug）或 `%value`（Display） | `?` 表示 Debug，`%` 表示 Display |

### OpenTelemetry Integration | OpenTelemetry 集成
```toml
# For distributed tracing (like System.Diagnostics + OTLP exporter)
[dependencies]
tracing-opentelemetry = "0.22"
opentelemetry = "0.21"
opentelemetry-otlp = "0.14"
```

```rust
// Add OpenTelemetry layer alongside console output
use tracing_opentelemetry::OpenTelemetryLayer;

fn init_otel() {
    let tracer = opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(opentelemetry_otlp::new_exporter().tonic())
        .install_batch(opentelemetry_sdk::runtime::Tokio)
        .expect("Failed to create OTLP tracer");

    tracing_subscriber::registry()
        .with(OpenTelemetryLayer::new(tracer))  // Send spans to Jaeger/Tempo
        .with(fmt::layer())                      // Also print to console
        .init();
}
// Now #[instrument] spans automatically become distributed traces!
```

***
