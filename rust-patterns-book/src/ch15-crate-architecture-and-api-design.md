# 15. Crate Architecture and API Design / 15. Crate 架构与 API 设计 🟡

> **What you'll learn / 你将学到：**
> - Module layout conventions and re-export strategies / 模块布局惯例与重导出策略
> - The public API design checklist for polished crates / 打磨精品 crate 的公共 API 设计清单
> - Ergonomic parameter patterns: `impl Into`, `AsRef`, `Cow` / 易用的参数模式：`impl Into`、`AsRef`、`Cow`
> - "Parse, don't validate" with `TryFrom` and validated types / 使用 `TryFrom` 和校验后的类型践行“以解析代替校验”
> - Feature flags, conditional compilation, and workspace organization / 特性标志（Feature flags）、条件编译及工作空间组织

## Module Layout Conventions / 模块布局惯例

```text
my_crate/
├── Cargo.toml
├── src/
│   ├── lib.rs          # Crate root — re-exports and public API / Crate 根 —— 重导出与公共 API
│   ├── config.rs       # Feature module / 功能模块
│   ├── parser/         # Complex module with sub-modules / 带有子模块的复杂模块
│   │   ├── mod.rs      # or parser.rs at parent level (Rust 2018+) / 或父级的 parser.rs
│   │   ├── lexer.rs
│   │   └── ast.rs
│   ├── error.rs        # Error types / 错误类型
│   └── utils.rs        # Internal helpers (pub(crate)) / 内部辅助程序
├── tests/
│   └── integration.rs  # Integration tests / 集成测试
├── benches/
│   └── perf.rs         # Benchmarks / 基准测试
└── examples/
    └── basic.rs        # cargo run --example basic / 示例代码
```

```rust
// lib.rs — curate your public API with re-exports:
// lib.rs — 通过重导出打磨你的公共 API：
mod config;
mod error;
mod parser;
mod utils;

// Re-export what users need:
// 重导出用户需要的项：
pub use config::Config;
pub use error::Error;
pub use parser::Parser;

// Public types are at the crate root — users write:
// 公共类型位于 crate 根部 —— 调用者可以这样写：
// use my_crate::Config;
// NOT: use my_crate::config::Config;
// 而非：use my_crate::config::Config;
```

**Visibility modifiers / 可见性修饰符**：

| Modifier / 修饰符 | Visible To / 可见范围 |
|----------|-----------|
| `pub` | Everyone / 所有人 |
| `pub(crate)` | This crate only / 仅限当前 crate |
| `pub(super)` | Parent module / 父模块 |
| `pub(in path)` | Specific ancestor module / 特定的祖先模块 |
| (none / 无) | Current module and its children / 当前模块及其子模块 |

### Public API Design Checklist / 公共 API 设计清单

1. **Accept references, return owned / 接收引用，返回所有权** — `fn process(input: &str) -> String`
2. **Use `impl Trait` for parameters / 为参数使用 `impl Trait`** — 使用 `fn read(r: impl Read)` 而非 `fn read<R: Read>(r: R)` 以获得更整洁的签名
3. **Return `Result`, not `panic!` / 返回 `Result` 而非 `panic!`** — 让调用者决定如何处理错误
4. **Implement standard traits / 实现标准 trait** — `Debug`、`Display`、`Clone`、`Default`、`From`/`Into`
5. **Make invalid states unrepresentable / 使无效状态无法表示** — 使用类型状态（type states）和新类型（newtypes）
6. **Follow the builder pattern for complex configuration / 对复杂配置采用建造者模式** — 如果字段是必填的，请结合使用类型状态
7. **Seal traits you don't want users to implement / 密封不希望用户实现的 trait** — `pub trait Sealed: private::Sealed {}`
8. **Mark types and functions `#[must_use]` / 将类型和函数标注为 `#[must_use]`** — 防止静默丢弃重要的 `Result`、guard 或数值。适用于任何忽略其返回值几乎肯定会导致 bug 的类型：
   ```rust
#[must_use = "dropping the guard immediately releases the lock"]
#[must_use = "丢弃 guard 会立即释放锁"]
pub struct LockGuard<'a, T> { /* ... */ }

#[must_use]
pub fn validate(input: &str) -> Result<ValidInput, ValidationError> { /* ... */ }
```

// Sealed trait pattern — users can use but not implement:
// 密封 trait 模式 —— 用户可以使用但无法实现该 trait：
mod private {
    pub trait Sealed {}
}

pub trait DatabaseDriver: private::Sealed {
    fn connect(&self, url: &str) -> Connection;
}

// Only types in THIS crate can implement Sealed → only we can implement DatabaseDriver
// 只有当前 crate 中的类型才能实现 Sealed → 只有我们能实现 DatabaseDriver
pub struct PostgresDriver;
impl private::Sealed for PostgresDriver {}
impl DatabaseDriver for PostgresDriver {
    fn connect(&self, url: &str) -> Connection { /* ... */ }
}
```

> **`#[non_exhaustive]`** — mark public enums and structs so that adding variants or fields is not a breaking change. Downstream crates must use a wildcard arm (`_ =>`) in match statements, and cannot construct the type with struct literal syntax:
>
> **`#[non_exhaustive]`** —— 标注公共枚举和结构体，使得添加变体或字段不再是破坏性变更。下游 crate 在 match 语句中必须包含通配符分支（`_ =>`），并且不能使用结构体字面量语法来构造该类型：
>
> ```rust
> #[non_exhaustive]
> pub enum DiagError {
>     Timeout,
>     HardwareFault,
>     // Adding a new variant in a future release is NOT a semver break.
>     // 在未来版本中添加新变体 不属于破坏性的 semver 变更。
> }
> ```

### Ergonomic Parameter Patterns — `impl Into`, `AsRef`, `Cow` / 易用的参数模式：`impl Into`、`AsRef`、`Cow`

One of Rust's most impactful API patterns is accepting the **most general type** in function parameters, so callers don't need repetitive `.to_string()`, `&*s`, or `.as_ref()` at every call site. This is the Rust-specific version of "be liberal in what you accept."

Rust 中最有影响力的 API 模式之一就是在函数参数中接收 **最宽泛的类型**，这样调用者就不需要在每个调用处重复编写 `.to_string()`、`&*s` 或 `.as_ref()`。这是“宽以待人，严于律己”原则在 Rust 中的具体应用。

#### `impl Into<T>` — Accept Anything Convertible / 接受任何可转换的类型

```rust
// ❌ Friction: callers must convert manually
// ❌ 阻碍：调用者必须手动转换
fn connect(host: String, port: u16) -> Connection {
    // ...
}
connect("localhost".to_string(), 5432);  // Annoying .to_string() / 烦人的 .to_string()
connect(hostname.clone(), 5432);          // Unnecessary clone if we already have String / 如果已有 String，则 clone 是多余的

// ✅ Ergonomic: accept anything that converts to String
// ✅ 易用：接收任何可以转换为 String 的类型
fn connect(host: impl Into<String>, port: u16) -> Connection {
    let host = host.into();  // Convert once, inside the function
                             // 在函数内部进行一次转换
    // ...
}
connect("localhost", 5432);     // &str — zero friction / 零阻碍
connect(hostname, 5432);        // String — moved, no clone / 移动所有权，无需 clone
connect(arc_str, 5432);         // Arc<str> if From is implemented / 如果实现了 From，则支持 Arc<str>
```

This works because Rust's `From`/`Into` trait pair provides blanket conversions. When you accept `impl Into<T>`, you're saying: "give me anything that knows how to become a `T`."

这是得益于 Rust 的 `From`/`Into` trait 对所提供的覆盖式转换（blanket conversions）。当你接收 `impl Into<T>` 时，你的意思是：“给我任何知道如何变成 `T` 的东西。”

#### `AsRef<T>` — Borrow as a Reference / 作为引用借用

`AsRef<T>` is the borrowing counterpart to `Into<T>`. Use it when you only need to *read* the data, not take ownership:

`AsRef<T>` 是 `Into<T>` 在借用方面的对应物。当你只需要 *读取* 数据而不需要获取所有权时，请使用它：

```rust
use std::path::Path;

// ❌ Forces callers to convert to &Path
// ❌ 强制调用者转换为 &Path
fn file_exists(path: &Path) -> bool {
    path.exists()
}
file_exists(Path::new("/tmp/test.txt"));  // Awkward / 略显笨拙

// ✅ Accept anything that can behave as a &Path
// ✅ 接收任何可以表现为 &Path 的类型
fn file_exists(path: impl AsRef<Path>) -> bool {
    path.as_ref().exists()
}
file_exists("/tmp/test.txt");                    // &str ✅
file_exists(String::from("/tmp/test.txt"));      // String ✅
file_exists(Path::new("/tmp/test.txt"));         // &Path ✅
file_exists(PathBuf::from("/tmp/test.txt"));     // PathBuf ✅

// Same pattern for string-like parameters:
// 对于类字符串参数同样适用：
fn log_message(msg: impl AsRef<str>) {
    println!("[LOG] {}", msg.as_ref());
}
log_message("hello");                    // &str ✅
log_message(String::from("hello"));      // String ✅
```

#### `Cow<T>` — Clone on Write / 写时克隆

`Cow<'a, T>` (Clone on Write) delays allocation until mutation is needed. It holds either a borrowed `&T` or an owned `T::Owned`. This is perfect when most calls don't need to modify the data:

`Cow<'a, T>`（写时克隆）将分配推迟到需要修改时。它要么持有一个借用的 `&T`，要么持有一个具有所有权的 `T::Owned`。当大多数调用不需要修改数据时，它是完美的选择：

```rust
use std::borrow::Cow;

/// Normalizes a diagnostic message — only allocates if changes are needed.
/// 规范化诊断消息 —— 仅在需要更改时才分配内存。
fn normalize_message(msg: &str) -> Cow<'_, str> {
    if msg.contains('\t') || msg.contains('\r') {
        // Must allocate — we need to modify the content
        // 必须分配 —— 我们需要修改内容
        Cow::Owned(msg.replace('\t', "    ").replace('\r', ""))
    } else {
        // No allocation — just borrow the original
        // 无需分配 —— 直接借用原本的内容
        Cow::Borrowed(msg)
    }
}

// Most messages pass through without allocation:
// 大多消息不需要分配内存即可通过：
let clean = normalize_message("All tests passed");          // Borrowed — free / 借用 —— 零开销
let fixed = normalize_message("Error:\tfailed\r\n");        // Owned — allocated / 具有所有权 —— 已分配内存

// Cow<str> implements Deref<Target=str>, so it works like &str:
// Cow<str> 实现了 Deref<Target=str>，所以它用起来像 &str：
println!("{}", clean);
println!("{}", fixed.to_uppercase());
```

#### Quick Reference: Which to Use / 快速参考：该使用哪一个

```text
Do you need ownership of the data inside the function? / 函数内部需要获取数据所有权吗？
├── YES / 是 → impl Into<T>
│             "Give me anything that can become a T"
│             “给我任何能变成 T 的东西”
└── NO / 否 → Do you only need to read it? / 你是否只需要读取它？
     ├── YES / 是 → impl AsRef<T> or &T
     │             "Give me anything I can borrow as a &T"
     │             “给我任何能借用为 &T 的东西”
     └── MAYBE / 可能 (might need to modify sometimes? / 有时可能需要修改？)
          └── Cow<'_, T>
              "Borrow if possible, clone only when you must"
              “尽可能借用，仅在必须时才克隆”
```

| Pattern / 模式 | Ownership / 所有权 | Allocation / 内存分配 | When to use / 适用场景 |
|---------|-----------|------------|-------------|
| `&str` | Borrowed / 借用 | Never / 从不 | Simple string params / 简单的字符串参数 |
| `impl AsRef<str>` | Borrowed / 借用 | Never / 从不 | Accept String, &str, etc. — read only / 接收 String、&str 等 —— 仅限读取 |
| `impl Into<String>` | Owned / 所有权 | On conversion / 转换时 | Accept &str, String — will store/own / 接收 &str、String —— 用于存储/持有所有权 |
| `Cow<'_, str>` | Either / 任意 | Only if modified / 仅在修改时 | Processing that usually doesn't modify / 通常不需要修改的处理过程 |
| `&[u8]` / `impl AsRef<[u8]>` | Borrowed / 借用 | Never / 从不 | Byte-oriented APIs / 面向字节的 API |

> **`Borrow<T>` vs `AsRef<T>`**：Both provide `&T`, but `Borrow<T>` additionally guarantees that `Eq`, `Ord`, and `Hash` are **consistent** between the original and borrowed form. This is why `HashMap<String, V>::get()` accepts `&Q where String: Borrow<Q>` — not `AsRef`. Use `Borrow` when the borrowed form is used as a lookup key; use `AsRef` for general "give me a reference" parameters.
>
> **`Borrow<T>` vs `AsRef<T>`**：两者都提供 `&T`，但 `Borrow<T>` 额外保证了 `Eq`、`Ord` 和 `Hash` 在原始形式和借用形式之间是 **一致的**。这就是为什么 `HashMap<String, V>::get()` 接收的是 `&Q where String: Borrow<Q>` 而非 `AsRef`。当借用形式被用作查找键时，请使用 `Borrow`；对于普通的“给我一个引用”参数，请使用 `AsRef`。

#### Composing Conversions in APIs / 在 API 中组合使用转换

```rust
/// A well-designed diagnostic API using ergonomic parameters:
/// 一个使用易用参数精心设计的诊断 API：
pub struct DiagRunner {
    name: String,
    config_path: PathBuf,
    results: HashMap<String, TestResult>,
}

impl DiagRunner {
    /// Accept any string-like type for name, any path-like type for config.
    /// name 接收任何类字符串类型，config 接收任何类路径类型。
    pub fn new(
        name: impl Into<String>,
        config_path: impl Into<PathBuf>,
    ) -> Self {
        DiagRunner {
            name: name.into(),
            config_path: config_path.into(),
        }
    }

    /// Accept any AsRef<str> for read-only lookup.
    /// 接收任何 AsRef<str> 用于只读查找。
    pub fn get_result(&self, test_name: impl AsRef<str>) -> Option<&TestResult> {
        self.results.get(test_name.as_ref())
    }
}

// All of these work with zero caller friction:
// 以下所有调用都能正常工作，且对调用者零阻碍：
let runner = DiagRunner::new("GPU Diag", "/etc/diag_tool/config.json");
let runner = DiagRunner::new(format!("Diag-{}", node_id), config_path);
let runner = DiagRunner::new(name_string, path_buf);
```

***

## Case Study: Designing a Public Crate API — Before & After / 案例研究：设计公共 Crate API —— 之前与之后

A real-world example of evolving a stringly-typed internal API into an ergonomic, type-safe public API. Consider a configuration parser crate:

一个将“字符串类型化”（stringly-typed）的内部 API 演变为易用、类型安全的公共 API 的真实案例。考虑一个配置解析器 crate：

**Before / 之前** (stringly-typed, easy to misuse / 字符串类型化，容易误用)：

```rust
// ❌ All parameters are strings — no compile-time validation
// ❌ 所有参数都是字符串 —— 没有编译时验证
pub fn parse_config(path: &str, format: &str, strict: bool) -> Result<Config, String> {
    // What formats are valid? "json"? "JSON"? "Json"?
    // 哪些格式是有效的？"json"？"JSON"？"Json"？
    // Is path a file path or URL?
    // path 是文件路径还是 URL？
    // What does "strict" even mean?
    // "strict" 到底是什么意思？
    todo!()
}
```

**After / 之后** (type-safe, self-documenting / 类型安全，自描述)：

```rust
use std::path::Path;

/// Supported configuration formats.
/// 支持的配置格式。
#[derive(Debug, Clone, Copy)]
#[non_exhaustive]  // Adding formats won't break downstream / 添加格式不会破坏下游代码
pub enum Format {
    Json,
    Toml,
    Yaml,
}

/// Controls parsing strictness.
/// 控制解析的严格程度。
#[derive(Debug, Clone, Copy, Default)]
pub enum Strictness {
    /// Reject unknown fields (default for libraries)
    /// 拒绝未知字段（库的默认设置）
    #[default]
    Strict,
    /// Ignore unknown fields (useful for forward-compatible configs)
    /// 忽略未知字段（对向前兼容的配置很有用）
    Lenient,
}

pub fn parse_config(
    path: &Path,          // Type-enforced: must be a filesystem path / 类型强制：必须是文件系统路径
    format: Format,       // Enum: impossible to pass invalid format / 枚举：不可能传入无效格式
    strictness: Strictness,  // Named alternatives, not a bare bool / 命名选项，而非裸布尔值
) -> Result<Config, ConfigError> {
    todo!()
}
```

**What improved / 改进之处**：

| Aspect / 方面 | Before / 之前 | After / 之后 |
|--------|--------|-------|
| Format validation / 格式校验 | Runtime string comparison / 运行时字符串比较 | Compile-time enum / 编译时枚举 |
| Path type / 路径类型 | Raw `&str` (could be anything) / 原始 `&str`（可能是任何内容） | `&Path` (filesystem-specific) / `&Path`（特定于文件系统） |
| Strictness / 严格程度 | Mystery `bool` / 神秘的 `bool` | Self-documenting enum / 自描述枚举 |
| Error type / 错误类型 | `String` (opaque) / `String`（不透明） | `ConfigError` (structured) / `ConfigError`（结构化） |
| Extensibility / 可扩展性 | Breaking changes / 破坏性变更 | `#[non_exhaustive]` |

> **Rule of thumb / 经验准则**：If you find yourself writing a `match` on string values, consider replacing the parameter with an enum. If a parameter is a boolean that isn't obvious from context, use a two-variant enum instead.
>
> 如果你发现自己在对字符串值进行 `match` 操作，请考虑将参数替换为枚举。如果一个布尔参数在上下文中含义不明显，请改用具有两个变体的枚举。

***

### Parse Don't Validate — `TryFrom` and Validated Types / 以解析代替校验 —— `TryFrom` 与校验后的类型

"Parse, don't validate" is a principle that says: **don't check data and then pass around the raw unchecked form — instead, parse it into a type that can only exist if the data is valid.** Rust's `TryFrom` trait is the standard tool for this.

“以解析代替校验”（Parse, don't validate）原则指出：**不要在校验数据后继续传递原始的、未经验证的形式 —— 相反，应该将其解析为一个只有当数据有效时才能存在的类型。** Rust 的 `TryFrom` trait 是实现这一目标的标准工具。

#### The Problem: Validation Without Enforcement / 问题所在：缺乏强制力的校验

```rust
// ❌ Validate-then-use: nothing prevents using an invalid value after the check
// ❌ 先校验后使用：没有任何机制能阻止在检查后使用无效值
fn process_port(port: u16) {
    if port == 0 || port > 65535 {
        panic!("Invalid port");           // We checked, but... / 我们确实检查了，但是……
    }
    start_server(port);                    // What if someone calls start_server(0) directly?
                                           // 如果有人直接调用 start_server(0) 怎么办？
}

// ❌ Stringly-typed: an email is just a String — any garbage gets through
// ❌ 字符串类型化：电子邮件只是一个 String —— 任何垃圾数据都能混进来
fn send_email(to: String, body: String) {
    // Is `to` actually a valid email? We don't know.
    // `to` 真的是有效的电子邮件吗？我们不知道。
    // Someone could pass "not-an-email" and we only find out at the SMTP server.
    // 有人可能会传入 "not-an-email"，而我们只有在连接 SMTP 服务器时才会发现。
}
```

#### The Solution: Parse Into Validated Newtypes with `TryFrom` / 解决方案：使用 `TryFrom` 解析为校验后的新类型

```rust
use std::convert::TryFrom;
use std::fmt;

/// A validated TCP port number (1–65535).
/// If you have a `Port`, it is guaranteed valid.
/// 一个经过校验的 TCP 端口号 (1–65535)。
/// 如果你拥有一个 `Port` 实例，它就保证是有效的。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Port(u16);

impl TryFrom<u16> for Port {
    type Error = PortError;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        if value == 0 {
            Err(PortError::Zero)
        } else {
            Ok(Port(value))
        }
    }
}

impl Port {
    pub fn get(&self) -> u16 { self.0 }
}

#[derive(Debug)]
pub enum PortError {
    Zero,
    InvalidFormat,
}

impl fmt::Display for PortError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PortError::Zero => write!(f, "port must be non-zero"),
            PortError::InvalidFormat => write!(f, "invalid port format"),
        }
    }
}

impl std::error::Error for PortError {}

// Now the type system enforces validity:
// 现在类型系统强制保证了有效性：
fn start_server(port: Port) {
    // No validation needed — Port can only be constructed via TryFrom,
    // which already verified it's valid.
    // 无需再次校验 —— Port 只能通过 TryFrom 构造，而 TryFrom 已经验证过它的有效性。
    println!("Listening on port {}", port.get());
}

// Usage / 使用：
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let port = Port::try_from(8080)?;   // ✅ Validated once at the boundary / 在边界处进行一次校验
    start_server(port);                  // No re-validation anywhere downstream / 下游任何地方都不需要重新校验

    let bad = Port::try_from(0);         // ❌ Err(PortError::Zero)
    Ok(())
}
```

#### Real-World Example: Validated IPMI Address / 真实案例：经过校验的 IPMI 地址

```rust
/// A validated IPMI slave address (0x20–0xFE, even only).
/// 经过校验的 IPMI 从站地址（0x20–0xFE，且仅限偶数）。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IpmiAddr(u8);

#[derive(Debug)]
pub enum IpmiAddrError {
    Odd(u8),
    OutOfRange(u8),
}

impl fmt::Display for IpmiAddrError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IpmiAddrError::Odd(v) => write!(f, "IPMI address 0x{v:02X} must be even"),
            IpmiAddrError::OutOfRange(v) => {
                write!(f, "IPMI address 0x{v:02X} out of range (0x20..=0xFE)")
            }
        }
    }
}

impl TryFrom<u8> for IpmiAddr {
    type Error = IpmiAddrError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value % 2 != 0 {
            Err(IpmiAddrError::Odd(value))
        } else if value < 0x20 || value > 0xFE {
            Err(IpmiAddrError::OutOfRange(value))
        } else {
            Ok(IpmiAddr(value))
        }
    }
}

impl IpmiAddr {
    pub fn get(&self) -> u8 { self.0 }
}

// Downstream code never needs to re-check:
// 下游代码永远不需要重新检查：
fn send_ipmi_command(addr: IpmiAddr, cmd: u8, data: &[u8]) -> Result<Vec<u8>, IpmiError> {
    // addr.get() is guaranteed to be a valid, even IPMI address
    // addr.get() 保证是一个有效的、且为偶数的 IPMI 地址
    raw_ipmi_send(addr.get(), cmd, data)
}
```

#### Parsing Strings with `FromStr` / 使用 `FromStr` 解析字符串

For types that are commonly parsed from text (CLI args, config files), implement `FromStr`:

对于通常需要从文本（CLI 参数、配置文件）中解析的类型，请实现 `FromStr`：

```rust
use std::str::FromStr;

impl FromStr for Port {
    type Err = PortError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let n: u16 = s.parse().map_err(|_| PortError::InvalidFormat)?;
        Port::try_from(n)
    }
}

// Now works with .parse():
// 现在可以配合 .parse() 使用：
let port: Port = "8080".parse()?;   // Validates in one step / 一步完成校验

// And with clap CLI parsing:
// 以及 clap 的 CLI 解析：
// #[derive(Parser)]
// struct Args {
//     #[arg(short, long)]
//     port: Port,   // clap calls FromStr automatically / clap 会自动调用 FromStr
// }
```

#### `TryFrom` Chain for Complex Validation / 用于复杂校验的 `TryFrom` 链

```rust
// Stub types for this example — in production these would be in
// separate modules with their own TryFrom implementations.
```

```rust
# struct Hostname(String);
# impl TryFrom<String> for Hostname {
#     type Error = String;
#     fn try_from(s: String) -> Result<Self, String> { Ok(Hostname(s)) }
# }
# struct Timeout(u64);
# impl TryFrom<u64> for Timeout {
#     type Error = String;
#     fn try_from(ms: u64) -> Result<Self, String> {
#         if ms == 0 { Err("timeout must be > 0".into()) } else { Ok(Timeout(ms)) }
#     }
# }
# struct RawConfig { host: String, port: u16, timeout_ms: u64 }
# #[derive(Debug)]
# enum ConfigError {
#     InvalidHost(String),
#     InvalidPort(PortError),
#     InvalidTimeout(String),
# }
# impl From<std::io::Error> for ConfigError {
#     fn from(e: std::io::Error) -> Self { ConfigError::InvalidHost(e.to_string()) }
# }
# impl From<serde_json::Error> for ConfigError {
#     fn from(e: serde_json::Error) -> Self { ConfigError::InvalidHost(e.to_string()) }
# }
/// A validated configuration that can only exist if all fields are valid.
/// 一个经过校验的配置，只有当所有字段都有效时才能存在。
pub struct ValidConfig {
    pub host: Hostname,
    pub port: Port,
    pub timeout_ms: Timeout,
}

impl TryFrom<RawConfig> for ValidConfig {
    type Error = ConfigError;

    fn try_from(raw: RawConfig) -> Result<Self, Self::Error> {
        Ok(ValidConfig {
            host: Hostname::try_from(raw.host)
                .map_err(ConfigError::InvalidHost)?,
            port: Port::try_from(raw.port)
                .map_err(ConfigError::InvalidPort)?,
            timeout_ms: Timeout::try_from(raw.timeout_ms)
                .map_err(ConfigError::InvalidTimeout)?,
        })
    }
}

// Parse once at the boundary, use the validated type everywhere:
// 在边界处解析一次，在后续各处直接使用校验后的类型：
fn load_config(path: &str) -> Result<ValidConfig, ConfigError> {
    let raw: RawConfig = serde_json::from_str(&std::fs::read_to_string(path)?)?;
    ValidConfig::try_from(raw)  // All validation happens here / 所有校验均在此处发生
}
```

#### Summary: Validate vs Parse / 总结：校验 vs 解析

| Approach / 方法 | Data checked? / 数据是否已检查？ | Compiler enforces validity? / 编译器是否强制保证有效性？ | Re-validation needed? / 是否需要重复校验？ |
|----------|:---:|:---:|:---:|
| Runtime checks (if/assert) / 运行时检查 (if/assert) | ✅ | ❌ | Every function boundary / 每个函数边界处 |
| Validated newtype + `TryFrom` / 校验后的新类型 + `TryFrom` | ✅ | ✅ | Never — type is proof / 永不需要 —— 类型即是证明 |

The rule: **parse at the boundary, use validated types everywhere inside.** Raw strings, integers, and byte slices enter your system, get parsed into validated types via `TryFrom`/`FromStr`, and from that point forward the type system guarantees they're valid.

规则：**在边界处解析，在内部各处使用校验后的类型。** 原始字符串、整数和字节切片进入你的系统，通过 `TryFrom`/`FromStr` 解析为校验后的类型，从那一刻起，类型系统将保证它们的有效性。

### Feature Flags and Conditional Compilation / 特性标志与条件编译

# Cargo.toml
[features]
default = ["json"]          # Enabled by default / 默认启用
json = ["dep:serde_json"]   # Enables JSON support / 启用 JSON 支持
xml = ["dep:quick-xml"]     # Enables XML support / 启用 XML 支持
full = ["json", "xml"]      # Meta-feature: enables all / 元特性：启用所有

[dependencies]
serde = "1"
serde_json = { version = "1", optional = true }
quick-xml = { version = "0.31", optional = true }

```rust
// Conditional compilation based on features:
// 基于特性的条件编译：
#[cfg(feature = "json")]
pub fn to_json<T: serde::Serialize>(value: &T) -> String {
    serde_json::to_string(value).unwrap()
}

#[cfg(feature = "xml")]
pub fn to_xml<T: serde::Serialize>(value: &T) -> String {
    quick_xml::se::to_string(value).unwrap()
}

// Compile error if a required feature isn't enabled:
// 如果未启用所需的特性，则抛出编译错误：
#[cfg(not(any(feature = "json", feature = "xml")))]
compile_error!("At least one format feature (json, xml) must be enabled");
```

**Best practices / 最佳实践**：
- Keep `default` features minimal — users can opt in / 保持默认特性最简 —— 让用户自行选择开启
- Use `dep:` syntax (Rust 1.60+) for optional dependencies to avoid creating implicit features / 使用 `dep:` 语法（Rust 1.60+）处理可选依赖，避免创建隐式特性
- Document features in your README and crate-level docs / 在 README 和 crate 级文档中记录特性

### Workspace Organization / 工作空间组织

For large projects, use a Cargo workspace to share dependencies and build artifacts:

对于大型项目，请使用 Cargo 工作空间（workspace）来共享依赖项和构建产物：

```toml
```

# Root Cargo.toml / 根目录下的 Cargo.toml
[workspace]
members = [
    "core",         # Shared types and traits / 共享的类型和 trait
    "parser",       # Parsing library / 解析库
    "server",       # Binary — the main application / 二进制程序 —— 主应用
    "client",       # Client library / 客户端库
    "cli",          # CLI binary / 命令行二进制程序
]

# Shared dependency versions / 共享依赖版本
[workspace.dependencies]
serde = { version = "1", features = ["derive"] }
tokio = { version = "1", features = ["full"] }
tracing = "0.1"

# In each member's Cargo.toml / 在每个成员的 Cargo.toml 中：
# [dependencies]
# serde = { workspace = true }
```

```rust

**Benefits / 优势**：

- Single `Cargo.lock` — all crates use the same dependency versions / 统一的 `Cargo.lock` —— 所有 crate 使用相同的依赖版本
- `cargo test --workspace` runs all tests / `cargo test --workspace` 运行所有测试
- Shared build cache — compiling one crate benefits all / 共享构建缓存 —— 编译一个 crate 会让所有相关 crate 受益
- Clean dependency boundaries between components / 组件之间清晰的依赖边界

### `.cargo/config.toml`: Project-Level Configuration / 项目级配置

The `.cargo/config.toml` file (at the workspace root or in `$HOME/.cargo/`) customizes Cargo behavior without modifying `Cargo.toml`:

`.cargo/config.toml` 文件（位于工作空间根目录或 `$HOME/.cargo/` 中）可以在不修改 `Cargo.toml` 的情况下定制 Cargo 的行为：

```toml
```

# .cargo/config.toml

# Default target for this workspace
# 此工作空间的默认目标
[build]
target = "x86_64-unknown-linux-gnu"

# Custom runner — e.g., run via QEMU for cross-compiled binaries
# 自定义运行程序 —— 例如，对于交叉编译的二进制文件使用 QEMU 运行
[target.aarch64-unknown-linux-gnu]
runner = "qemu-aarch64-static"
linker = "aarch64-linux-gnu-gcc"

# Cargo aliases — custom shortcut commands
# Cargo 别名 —— 自定义快捷命令
[alias]
xt = "test --workspace --release"        # cargo xt = run all tests in release / 以 release 模式运行所有测试
ci = "clippy --workspace -- -D warnings" # cargo ci = lint with errors on warnings / 若有警告则报错
cov = "llvm-cov --workspace"             # cargo cov = coverage (requires cargo-llvm-cov) / 覆盖率测试

# Environment variables for build scripts
# 用于构建脚本的环境变量
[env]
IPMI_LIB_PATH = "/usr/lib/bmc"

# Use a custom registry (for internal packages)
# 使用自定义注册表（用于内部包）
# [registries.internal]
# index = "https://gitlab.internal/crates/index"
Common configuration patterns / 常用的配置模式：

| Setting / 设置 | Purpose / 用途 | Example / 示例 |
|---------|---------|---------|
| `[build] target` | Default compilation target / 默认编译目标 | `x86_64-unknown-linux-musl` for static builds / 用于静态构建 |
| `[target.X] runner` | How to run the binary / 如何运行二进制程序 | `"qemu-aarch64-static"` for cross-compiled / 用于交叉编译 |
| `[target.X] linker` | Which linker to use / 使用哪个链接器 | `"aarch64-linux-gnu-gcc"` |
| `[alias]` | Custom `cargo` subcommands / 自定义 `cargo` 子命令 | `xt = "test --workspace"` |
| `[env]` | Build-time environment variables / 构建时的环境变量 | Library paths, feature toggles / 库路径、特性开关 |
| `[net] offline` | Prevent network access / 禁止网络访问 | `true` for air-gapped builds / 用于离线构建 |

### Compile-Time Environment Variables: `env!()` and `option_env!()` / 编译时环境变量：`env!()` 与 `option_env!()`

Rust can embed environment variables into the binary at compile time — useful for version strings, build metadata, and configuration:

Rust 能够在编译时将环境变量嵌入二进制文件中 —— 这对于版本字符串、构建元数据和配置非常有用：

```rust
// env!() — panics at compile time if the variable is missing
// env!() —— 如果变量缺失，会在编译时触发 panic
const VERSION: &str = env!("CARGO_PKG_VERSION"); // "0.1.0" from Cargo.toml
const PKG_NAME: &str = env!("CARGO_PKG_NAME");   // Crate name from Cargo.toml

// option_env!() — returns Option<&str>, doesn't panic if missing
// option_env!() —— 返回 Option<&str>，变量缺失时不会 panic
const BUILD_SHA: Option<&str> = option_env!("GIT_SHA");
const BUILD_TIME: Option<&str> = option_env!("BUILD_TIMESTAMP");

fn print_version() {
    println!("{PKG_NAME} v{VERSION}");
    if let Some(sha) = BUILD_SHA {
        println!("  commit: {sha}");
    }
    if let Some(time) = BUILD_TIME {
        println!("  built:  {time}");
    }
}
```

Cargo automatically sets many useful environment variables:

Cargo 会自动设置许多有用的环境变量：

| Variable / 变量 | Value / 数值 | Use case / 用途 |
|----------|-------|----------|
| `CARGO_PKG_VERSION` | `"1.2.3"` | Version reporting / 版本汇报 |
| `CARGO_PKG_NAME` | `"diag_tool"` | Binary identification / 程序识别 |
| `CARGO_PKG_AUTHORS` | From `Cargo.toml` / 来自 `Cargo.toml` | About/help text / “关于”/帮助文本 |
| `CARGO_MANIFEST_DIR` | Absolute path to `Cargo.toml` / `Cargo.toml` 的绝对路径 | Locating test data files / 定位测试数据文件 |
| `OUT_DIR` | Build output directory / 构建输出目录 | `build.rs` code generation target / `build.rs` 代码生成的目录 |
| `TARGET` | Target triple / 目标三元组 | Platform-specific logic in `build.rs` / `build.rs` 中的平台特定逻辑 |

You can set custom env vars from `build.rs`:

你可以从 `build.rs` 设置自定义环境变量：

```rust
// build.rs
fn main() {
    println!("cargo::rustc-env=GIT_SHA={}", git_sha());
    println!("cargo::rustc-env=BUILD_TIMESTAMP={}", timestamp());
}
```

### `cfg_attr`: Conditional Attributes / `cfg_attr`：条件属性

`cfg_attr` applies an attribute **only when** a condition is true. This is more targeted than `#[cfg()]`, which includes/excludes entire items:

`cfg_attr` **仅当** 条件为真时才应用属性。这比 `#[cfg()]` 更具针对性，因为后者会包含或排除整个项：

```rust
// Derive Serialize only when the "serde" feature is enabled:
// 仅当开启 "serde" 特性时才派生 Serialize：
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone)]
pub struct DiagResult {
    pub fc: u32,
    pub passed: bool,
    pub message: String,
}
// Without "serde" feature: no serde dependency needed at all
// 如果没有 "serde" 特性：则完全不需要 serde 依赖
// With "serde" feature: DiagResult is serializable
// 如果有 "serde" 特性：DiagResult 即可序列化

// Conditional attribute for testing:
// 用于测试的条件属性：
#[cfg_attr(test, derive(PartialEq))]  // Only derive PartialEq in test builds
                                      // 仅在测试构建中派生 PartialEq
pub struct LargeStruct { /* ... */ }

// Platform-specific function attributes:
// 平台特定的函数属性：
#[cfg_attr(target_os = "linux", link_name = "ioctl")]
#[cfg_attr(target_os = "freebsd", link_name = "__ioctl")]
extern "C" fn platform_ioctl(fd: i32, request: u64) -> i32;
```

| Pattern / 模式 | What it does / 作用 |
|---------|-------------|
| `#[cfg(feature = "x")]` | Include/exclude the entire item / 包含/排除整个项 |
| `#[cfg_attr(feature = "x", derive(Foo))]` | Add `derive(Foo)` only when feature "x" is on / 仅当特性 "x" 开启时添加 `derive(Foo)` |
| `#[cfg_attr(test, allow(unused))]` | Suppress warnings only in test builds / 仅在测试构建中消除警告 |
| `#[cfg_attr(doc, doc = "...")]` | Documentation visible only in `cargo doc` / 仅在使用 `cargo doc` 时可见的文档内容 |

### `cargo deny` and `cargo audit`: Supply-Chain Security / `cargo deny` 与 `cargo audit`：供应链安全

```bash
# Install security audit tools / 安装安全审计工具
cargo install cargo-deny
cargo install cargo-audit

# Check for known vulnerabilities in dependencies / 检查依赖项中已知的安全漏洞
cargo audit

# Comprehensive checks: licenses, bans, advisories, sources / 全面检查：许可证、禁用名单、公告、来源
cargo deny check
```

Configure `cargo deny` with a `deny.toml` at the workspace root:

在工作空间根目录下使用 `deny.toml` 配置 `cargo deny`：

```toml
# deny.toml
[advisories]
vulnerability = "deny"      # Fail on known vulnerabilities / 发现已知漏洞时报错
unmaintained = "warn"        # Warn on unmaintained crates / 对没人维护的 crate 发出警告

[licenses]
allow = ["MIT", "Apache-2.0", "BSD-2-Clause", "BSD-3-Clause"]
deny = ["GPL-3.0"]          # Reject copyleft licenses / 拒绝 Copyleft 许可证

[bans]
multiple-versions = "warn"  # Warn if multiple versions of same crate / 如果同一个 crate 有多个版本则发出警告
deny = [
    { name = "openssl" },   # Force use of rustls instead / 强制使用 rustls 代替
]

[sources]
allow-git = []              # No git dependencies in production
```

| Tool / 工具 | Purpose / 用途 | When to run / 何时运行 |
|------|---------|-------------|
| `cargo audit` | Check for known CVEs in dependencies / 检查依赖项中已知的 CVE | CI pipeline, pre-release / CI 流水线、发布前 |
| `cargo deny check` | Licenses, bans, advisories, sources / 许可证、禁用、公告、来源 | CI pipeline / CI 流水线 |
| `cargo deny check licenses` | License compliance only / 仅检查许可证合规性 | Before open-sourcing / 开源前 |
| `cargo deny check bans` | Prevent specific crates / 防止引入特定 crate | Enforce architecture decisions / 强制执行架构决策 |

### Doc Tests: Tests Inside Documentation / 文档测试：文档中的测试

Rust doc comments (`///`) can contain code blocks that are **compiled and run as tests**:

Rust 的文档注释（`///`）可以包含代码块，这些代码块会被 **作为测试编译并运行**：

```rust
/// Parses a diagnostic fault code from a string.
/// 从字符串中解析诊断故障码。
///
/// # Examples / 示例
///
/// ```
/// use my_crate::parse_fc;
///
/// let fc = parse_fc("FC:12345").unwrap();
/// assert_eq!(fc, 12345);
/// ```
///
/// Invalid input returns an error / 无效输入会返回错误：
///
/// ```
/// use my_crate::parse_fc;
///
/// assert!(parse_fc("not-a-fc").is_err());
/// ```
pub fn parse_fc(input: &str) -> Result<u32, ParseError> {
    input.strip_prefix("FC:")
        .ok_or(ParseError::MissingPrefix)?
        .parse()
        .map_err(ParseError::InvalidNumber)
}
```

```bash
cargo test --doc  # Run only doc tests / 仅运行文档测试
cargo test        # Runs unit + integration + doc tests / 运行单元 + 集成 + 文档测试
```

**Module-level documentation** uses `//!` at the top of a file:

**模块级文档** 在文件顶部使用 `//!`：

```rust
//! # Diagnostic Framework / 诊断框架
//!
//! This crate provides the core diagnostic execution engine.
//! 它提供了核心诊断执行引擎。
//! It supports running diagnostic tests, collecting results,
//! and reporting to the BMC via IPMI.
//! 它支持运行诊断测试、收集结果，并通过 IPMI 向 BMC 汇报。
//!
//! ## Quick Start / 快速上手
//!
//! ```no_run
//! use diag_framework::Framework;
//!
//! let mut fw = Framework::new("config.json")?;
//! fw.run_all_tests()?;
//! ```
```

### Benchmarking with Criterion / 使用 Criterion 进行基准测试

> **Full coverage / 完整内容**：See the [Benchmarking with criterion](ch14-testing-and-benchmarking-patterns.md#benchmarking-with-criterion) section in Chapter 14 (Testing and Benchmarking Patterns) for complete `criterion` setup, API examples, and a comparison table vs `cargo bench`. Below is a quick-reference for architecture-specific usage.
>
> 有关完整的 `criterion` 设置、API 示例以及与 `cargo bench` 的对比表，请参阅第 14 章（测试与基准模式）中的 [使用 criterion 进行基准测试](ch14-testing-and-benchmarking-patterns.md#benchmarking-with-criterion) 章节。以下是针对架构特定用途的快速参考。

When benchmarking your crate's public API, place benchmarks in `benches/` and keep them focused on the hot path — typically parsers, serializers, or validation boundaries:

在对 crate 的公共 API 进行基准测试时，请将 benchmark 放在 `benches/` 目录中，并专注于热点路径 —— 通常包括解析器、序列化器或校验边界：

```bash
cargo bench                  # Run all benchmarks / 运行所有基准测试
cargo bench -- parse_config  # Run specific benchmark / 运行特定的基准测试
# Results in target/criterion/ with HTML reports / 结果保存在 target/criterion/，包含 HTML 报告
```

> **Key Takeaways — Architecture & API Design / 关键要点：架构与 API 设计**
> - Accept the most general type (`impl Into`, `impl AsRef`, `Cow`); return the most specific / 接收最通用的类型（`impl Into`、`impl AsRef`、`Cow`）；返回最具体的类型
> - Parse Don't Validate: use `TryFrom` to create types that are valid by construction / 以解析代替校验：使用 `TryFrom` 创建构造即有效的类型
> - `#[non_exhaustive]` on public enums prevents breaking changes when adding variants / 公共枚举上的 `#[non_exhaustive]` 标签能防止添加变体时导致破坏性变更
> - `#[must_use]` catches silent discards of important values / `#[must_use]` 能捕获对重要数值的静默丢弃

> **See also / 延伸阅读**：[Ch 09 — Error Handling](ch09-error-handling-patterns.md) 了解公共 API 中的错误类型设计。[Ch 14 — Testing](ch14-testing-and-benchmarking-patterns.md) 了解如何测试 crate 的公共 API。

---

### Exercise: Crate API Refactoring ★★ (~30 min) / 练习：Crate API 重构

Refactor the following "stringly-typed" API into one that uses `TryFrom`, newtypes, and builder pattern:

将以下“字符串类型化”的 API 重构为使用 `TryFrom`、新类型和建造者模式的 API：

```rust,ignore
// BEFORE: Easy to misuse
// 之前：容易误用
fn create_server(host: &str, port: &str, max_conn: &str) -> Server { ... }
```

Design a `ServerConfig` with validated types `Host`, `Port` (1–65535), and `MaxConnections` (1–10000) that reject invalid values at parse time.

设计一个 `ServerConfig`，包含经过校验的类型 `Host`、`Port` (1–65535) 和 `MaxConnections` (1–10000)，并在解析阶段拒绝无效值。

<details>
<summary>🔑 Solution / 参考答案</summary>

```rust
#[derive(Debug, Clone)]
struct Host(String);

impl TryFrom<&str> for Host {
    type Error = String;
    fn try_from(s: &str) -> Result<Self, String> {
        if s.is_empty() { return Err("host cannot be empty / host 不能为空".into()); }
        if s.contains(' ') { return Err("host cannot contain spaces / host 不能包含空格".into()); }
        Ok(Host(s.to_string()))
    }
}

#[derive(Debug, Clone, Copy)]
struct Port(u16);

impl TryFrom<u16> for Port {
    type Error = String;
    fn try_from(p: u16) -> Result<Self, String> {
        if p == 0 { return Err("port must be >= 1 / 端口必须 >= 1".into()); }
        Ok(Port(p))
    }
}

#[derive(Debug, Clone, Copy)]
struct MaxConnections(u32);

impl TryFrom<u32> for MaxConnections {
    type Error = String;
    fn try_from(n: u32) -> Result<Self, String> {
        if n == 0 || n > 10_000 {
            return Err(format!("max_connections must be 1–10000, got {n} / max_connections 必须在 1-10000 之间，当前为 {n}"));
        }
        Ok(MaxConnections(n))
    }
}

#[derive(Debug)]
struct ServerConfig {
    host: Host,
    port: Port,
    max_connections: MaxConnections,
}

impl ServerConfig {
    fn new(host: Host, port: Port, max_connections: MaxConnections) -> Self {
        ServerConfig { host, port, max_connections }
    }
}

fn main() {
    let config = ServerConfig::new(
        Host::try_from("localhost").unwrap(),
        Port::try_from(8080).unwrap(),
        MaxConnections::try_from(100).unwrap(),
    );
    println!("{config:?}");

    // Invalid values caught at parse time:
    // 无效值在解析时被捕获：
    assert!(Host::try_from("").is_err());
    assert!(Port::try_from(0).is_err());
    assert!(MaxConnections::try_from(99999).is_err());
}
```

</details>

***

