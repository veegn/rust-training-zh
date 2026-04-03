[English Original](../en/ch15-crate-architecture-and-api-design.md)

# 第 15 章：Crate 架构与 API 设计 🟡

> **你将学到：**
> - **模块布局惯例** 与重导出策略。
> - 完善的 crate **公共 API 设计清单**。
> - **易用的参数模式**：`impl Into`、`AsRef`、`Cow`。
> - **“以解析代替校验”**：使用 `TryFrom` 和经过验证的类型。
> - **特性标志 (Feature flags)**、条件编译与工作空间组织。

## 15.1 模块布局惯例

```text
my_crate/
├── Cargo.toml
├── src/
│   ├── lib.rs          # Crate 根节点 —— 重导出与公共 API
│   ├── config.rs       # 功能模块
│   ├── parser/         # 带有子模块的复杂模块
│   │   ├── mod.rs      # 或在父级目录下的 parser.rs (Rust 2018+ 风格)
│   │   ├── lexer.rs
│   │   └── ast.rs
│   ├── error.rs        # 错误类型
│   └── utils.rs        # 内部辅助工具 (pub(crate))
├── tests/
│   └── integration.rs  # 集成测试
├── benches/
│   └── perf.rs         # 基准测试
└── examples/
    └── basic.rs        # 通过 cargo run --example basic 运行
```

```rust
// lib.rs —— 通过重导出来精挑细选你的公共 API：
mod config;
mod error;
mod parser;
mod utils;

// 重导出用户需要的项：
pub use config::Config;
pub use error::Error;
pub use parser::Parser;

// 公共类型位于 crate 根部 —— 用户只需编写：
// use my_crate::Config;
// 而不是：use my_crate::config::Config;
```

**可见性修饰符**：

| 修饰符 | 对谁可见 |
|----------|-----------|
| `pub` | 所有人 |
| `pub(crate)` | 仅限当前 crate |
| `pub(super)` | 父模块 |
| `pub(in path)` | 指定的祖先模块 |
| (不写) | 当前模块及其子模块 |

### 公共 API 设计清单

1. **接收引用，返回所有权类型** —— `fn process(input: &str) -> String`。
2. **在参数中使用 `impl Trait`** —— 相比 `fn read<R: Read>(r: R)`，使用 `fn read(r: impl Read)` 能让签名更整洁。
3. **返回 `Result` 而非使用 `panic!`** —— 让调用者决定如何处理错误。
4. **实现标准特性** —— 如 `Debug`、`Display`、`Clone`、`Default`、`From`/`Into`。
5. **使非法状态无法被表示** —— 使用类型状态 (Type states) 和新类型 (Newtypes)。
6. **针对复杂的配置使用构建器模式 (Builder pattern)** —— 如果有必填字段，可结合类型状态。
7. **密封那些你不希望用户实现的特性** —— `pub trait Sealed: private::Sealed {}`。
8. **为类型和函数标记 `#[must_use]`** —— 防止调用者无意中忽略重要的 `Result`、Guard 或返回值。对于任何忽视其返回值几乎必然导致 bug 的类型，都应应用此标记：
   ```rust
   #[must_use = "立即丢弃 guard 会导致锁被立即释放"]
   pub struct LockGuard<'a, T> { /* ... */ }

   #[must_use]
   pub fn validate(input: &str) -> Result<ValidInput, ValidationError> { /* ... */ }
   ```

```rust
// 密封特性 (Sealed trait) 模式 —— 用户可以使用但无法实现：
mod private {
    pub trait Sealed {}
}

pub trait DatabaseDriver: private::Sealed {
    fn connect(&self, url: &str) -> Connection;
}

// 只有当前 crate 中的类型才能实现 Sealed → 也就意味着只有我们能实现 DatabaseDriver
pub struct PostgresDriver;
impl private::Sealed for PostgresDriver {}
impl DatabaseDriver for PostgresDriver {
    fn connect(&self, url: &str) -> Connection { /* ... */ }
}
```

> **`#[non_exhaustive]`** —— 为公共枚举和结构体打上此标记，这样添加新变体或字段就不属于破坏性变更。下游 crate 在 match 语句中必须使用通配符分支 (`_ =>`)，且无法通过结构体字面量语法构造该类型：
> ```rust
> #[non_exhaustive]
> pub enum DiagError {
>     Timeout,
>     HardwareFault,
>     // 在未来的版本中添加新变体 不会 破坏语义化版本 (semver)。
> }
> ```

### 易用的参数模式 —— `impl Into`、`AsRef`、`Cow`

Rust 中影响力最大的 API 模式之一是在函数参数中接收 **最通用的类型**，这样调用者就不必在每个调用处重复编写 `.to_string()`、`&*s` 或 `.as_ref()`。这是 Rust 特有的“在接收时保持宽容”的设计哲学。

#### `impl Into<T>` —— 接收任何可转换的类型

```rust
// ❌ 不便之处：调用者必须手动转换
fn connect(host: String, port: u16) -> Connection {
    // ...
}
connect("localhost".to_string(), 5432);  // 烦人的 .to_string()
connect(hostname.clone(), 5432);          // 如果我们已经有了 String，这会导致不必要的克隆

// ✅ 易用：接收任何能转换为 String 的类型
fn connect(host: impl Into<String>, port: u16) -> Connection {
    let host = host.into();  // 在函数内部进行一次转换
    // ...
}
connect("localhost", 5432);     // &str —— 零摩擦
connect(hostname, 5432);        // String —— 直接转移所有权，无克隆
```

之所以可行，是因为 Rust 的 `From`/`Into` 特性对提供了“一揽子转换”功能。当你接收 `impl Into<T>` 时，你的意思是：“给我任何知道如何变成 `T` 的东西。”

#### `AsRef<T>` —— 作为引用进行借用

`AsRef<T>` 是 `Into<T>` 在借用方面的对应物。当你只需要 **读取** 数据而不需要获取其所有权时，请使用它：

```rust
use std::path::Path;

// ❌ 强制调用者转换为 &Path
fn file_exists(path: &Path) -> bool {
    path.exists()
}
file_exists(Path::new("/tmp/test.txt"));  // 比较笨拙

// ✅ 接收任何能作为 &Path 使用的类型
fn file_exists(path: impl AsRef<Path>) -> bool {
    path.as_ref().exists()
}
file_exists("/tmp/test.txt");                    // &str ✅
file_exists(String::from("/tmp/test.txt"));      // String ✅
file_exists(Path::new("/tmp/test.txt"));         // &Path ✅
file_exists(PathBuf::from("/tmp/test.txt"));     // PathBuf ✅

// 对于类字符串参数，使用同样的模式：
fn log_message(msg: impl AsRef<str>) {
    println!("[LOG] {}", msg.as_ref());
}
log_message("hello");                    // &str ✅
log_message(String::from("hello"));      // String ✅
```

#### `Cow<T>` —— 写时克隆 (Clone on Write)

`Cow<'a, T>` (Clone on Write) 会将内存分配推迟到需要修改时。它持有借用的 `&T` 或拥有所有权类型的 `T::Owned`。这非常适合那些大多数调用都不需要修改数据的场景：

```rust
use std::borrow::Cow;

/// 对诊断消息进行规范化 —— 仅在需要修改时才进行分配。
fn normalize_message(msg: &str) -> Cow<'_, str> {
    if msg.contains('\t') || msg.contains('\r') {
        // 必须进行分配 —— 我们需要修改内容
        Cow::Owned(msg.replace('\t', "    ").replace('\r', ""))
    } else {
        // 无分配 —— 直接借用原始字符串
        Cow::Borrowed(msg)
    }
}

// 大多数消息在没有分配的情况下通过：
let clean = normalize_message("All tests passed");          // 借用方式 —— 免费
let fixed = normalize_message("Error:\tfailed\r\n");        // 所有权方式 —— 发生了分配

// Cow<str> 实现了 Deref<Target=str>，因此它的用法与 &str 类似：
println!("{}", clean);
println!("{}", fixed.to_uppercase());
```

#### 快速参考：该使用哪一个

```text
你是否需要在函数内部获取数据的所有权？
├── 是 → impl Into<T>
│         "给我任何能变成 T 的东西"
└── 否  → 你是否只需要读取它？
     ├── 是 → impl AsRef<T> 或 &T
     │         "给我任何我能作为 &T 借用的东西"
     └── 也许 (有时可能需要修改？)
          └── Cow<'_, T>
              "尽可能借用，仅在必须时克隆"
```

| 模式 | 所有权状态 | 分配情况 | 何时使用 |
|---------|-----------|------------|-------------|
| `&str` | 借用 | 从不 | 简单的字符串参数 |
| `impl AsRef<str>` | 借用 | 从不 | 接收 String, &str 等 —— 仅限读取 |
| `impl Into<String>` | 所有权 | 转换时可能发生 | 接收 &str, String —— 将存储/拥有它 |
| `Cow<'_, str>` | 二者择一 | 仅在修改时 | 处理过程通常不修改数据的场景 |
| `&[u8]` / `AsRef<[u8]>` | 借用 | 从不 | 面向字节的 API |

> **`Borrow<T>` vs `AsRef<T>`**：二者都能提供 `&T`，但 `Borrow<T>` 额外保证了原始形式与借用形式之间的 `Eq`、`Ord` 和 `Hash` 是 **一致的**。这就是为什么 `HashMap<String, V>::get()` 接收的是 `&Q where String: Borrow<Q>` —— 而不是 `AsRef`。当借用形式被用作查找键 (Lookup key) 时请使用 `Borrow`；对于通用的“给我一个引用”参数，请使用 `AsRef`。

#### 在 API 中组合使用转换

```rust
/// 一个设计良好的、使用了易用参数模式的诊断 API：
pub struct DiagRunner {
    name: String,
    config_path: PathBuf,
    results: HashMap<String, TestResult>,
}

impl DiagRunner {
    /// 针对名称接收任何类字符串类型，针对配置接收任何类路径类型。
    pub fn new(
        name: impl Into<String>,
        config_path: impl Into<PathBuf>,
    ) -> Self {
        DiagRunner {
            name: name.into(),
            config_path: config_path.into(),
        }
    }

    /// 为只读查找接收任何 AsRef<str>。
    pub fn get_result(&self, test_name: impl AsRef<str>) -> Option<&TestResult> {
        self.results.get(test_name.as_ref())
    }
}

// 所有这些调用在调用方都没有摩擦：
let runner = DiagRunner::new("GPU Diag", "/etc/diag_tool/config.json");
let runner = DiagRunner::new(format!("Diag-{}", node_id), config_path);
let runner = DiagRunner::new(name_string, path_buf);
```

---

## 15.2 案例研究：设计公共 Crate API —— 演进过程

一个将强依赖字符串的内部 API 演变为易用、类型安全的公共 API 的真实案例。考虑一个配置文件解析 crate：

**重构前** (强依赖字符串，容易被误用)：

```rust
// ❌ 所有参数都是字符串 —— 缺乏编译时校验
pub fn parse_config(path: &str, format: &str, strict: bool) -> Result<Config, String> {
    // 哪些格式是有效的？"json"？"JSON"？"Json"？
    // path 是文件路径还是 URL？
    // "strict" 到底是什么意思？
    todo!()
}
```

**重构后** (类型安全，自带文档说明)：

```rust
use std::path::Path;

/// 支持的配置格式。
#[derive(Debug, Clone, Copy)]
#[non_exhaustive]  // 添加格式不会破坏下游代码
pub enum Format {
    Json,
    Toml,
    Yaml,
}

/// 控制解析的严格程度。
#[derive(Debug, Clone, Copy, Default)]
pub enum Strictness {
    /// 拒绝未知字段 (库的默认行为)
    #[default]
    Strict,
    /// 忽略未知字段 (适用于向前兼容的配置)
    Lenient,
}

pub fn parse_config(
    path: &Path,          // 类型强制：必须是文件系统路径
    format: Format,       // 枚举：不可能传入无效格式
    strictness: Strictness,  // 命名的选项，而非裸布尔值
) -> Result<Config, ConfigError> {
    todo!()
}
```

**改进之处**：

| 维度 | 重构前 | 重构后 |
|--------|--------|-------|
| **格式校验** | 运行时的字符串比较 | 编译时的枚举 |
| **路径类型** | 原始的 `&str` (可以是任何内容) | `&Path` (文件系统专用) |
| **严格程度** | 神秘的 `bool` | 自带文档说明的枚举 |
| **错误类型** | `String` (不透明) | `ConfigError` (结构化的) |
| **可扩展性** | 容易造成破坏性变更 | `#[non_exhaustive]` |

---

### “以解析代替校验” —— `TryFrom` 与验证过的类型

“以解析代替校验 (Parse, don't validate)”原则指出：**不要检查数据后依然传递原始的、未经检查的形式 —— 相反，应将其解析为一种只有在数据有效时才能存在的类型。** Rust 的 `TryFrom` 特性是实现这一目标的标准工具。

#### 问题所在：只有校验，没有约束

```rust
// ❌ 先校验后使用：检查后没有什么能阻止使用无效值
fn process_port(port: u16) {
    if port == 0 || port > 65535 {
        panic!("无效端口");           // 我们检查了，但是...
    }
    start_server(port);                    // 如果有人直接调用 start_server(0) 怎么办？
}

// ❌ 强依赖字符串：电子邮件只是一个 String —— 任何垃圾数据都能混进来
fn send_email(to: String, body: String) {
    // `to` 真的是有效的邮件地址吗？我们不知道。
    // 有人可能会传入 "not-an-email"，而我们只有到了 SMTP 服务器端才能发现。
}
```

#### 解决方案：通过 `TryFrom` 解析为经过验证的新类型

```rust
use std::convert::TryFrom;
use std::fmt;

/// 一个经过验证的 TCP 端口号 (1–65535)。
/// 如果你拥有一个 `Port` 实例，它就被保证是有效的。
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
            PortError::Zero => write!(f, "端口号不能为零"),
            PortError::InvalidFormat => write!(f, "无效的端口格式"),
        }
    }
}

impl std::error::Error for PortError {}

// 现在类型系统强制了有效性：
fn start_server(port: Port) {
    // 无需校验 —— Port 只能通过 TryFrom 构造，
    // 而 TryFrom 已经验证过它的有效性。
    println!("正在监听端口 {}", port.get());
}

// 使用方式：
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let port = Port::try_from(8080)?;   // ✅ 在系统边界处进行一次验证
    start_server(port);                  // 下游的任何地方都无需重复验证

    let bad = Port::try_from(0);         // ❌ 返回 Err(PortError::Zero)
    Ok(())
}
```

#### 字符串解析与 `FromStr`

对于通常从文本 (CLI 参数、配置文件) 中解析出的类型，请实现 `FromStr`：

```rust
use std::str::FromStr;

impl FromStr for Port {
    type Err = PortError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let n: u16 = s.parse().map_err(|_| PortError::InvalidFormat)?;
        Port::try_from(n)
    }
}

// 现在可以配合 .parse() 使用：
let port: Port = "8080".parse()?;   // 一步完成验证

// 也可以配合 clap 进行 CLI 解析：
// #[derive(Parser)]
// struct Args {
//     #[arg(short, long)]
//     port: Port,   // clap 会自动调用 FromStr
// }
```

#### 总结：校验 vs 解析

| 方法 | 是否检查数据？ | 编译器是否强制执行有效性？ | 是否需要重复校验？ |
|----------|:---:|:---:|:---:|
| **运行时检查 (if/assert)** | ✅ | ❌ | 每个函数边界都需要 |
| **验证后的新类型 + `TryFrom`** | ✅ | ✅ | 从不需要 —— 类型就是证明 |

规则是：**在边界处解析，在内部各处都使用验证后的类型。** 原始字符串、整数和字节切片进入你的系统，通过 `TryFrom`/`FromStr` 解析为经过验证的类型，之后类型系统就会保证它们的有效性。

### 特性标志与条件编译

```toml
# Cargo.toml
[features]
default = ["json"]          # 默认开启
json = ["dep:serde_json"]   # 开启 JSON 支持
xml = ["dep:quick-xml"]     # 开启 XML 支持
full = ["json", "xml"]      # 元特性：开启所有功能

[dependencies]
serde = "1"
serde_json = { version = "1", optional = true }
quick-xml = { version = "0.31", optional = true }
```

```rust
// 基于特性的条件编译：
#[cfg(feature = "json")]
pub fn to_json<T: serde::Serialize>(value: &T) -> String {
    serde_json::to_string(value).unwrap()
}

#[cfg(feature = "xml")]
pub fn to_xml<T: serde::Serialize>(value: &T) -> String {
    quick_xml::se::to_string(value).unwrap()
}

// 如果一个必须的特性没有开启，则产生编译错误：
#[cfg(not(any(feature = "json", feature = "xml")))]
compile_error!("必须至少开启一个格式特性 (json, xml)");
```

**最佳实践**：
- 保持 `default` 特性尽可能少 —— 让用户按需开启。
- 使用 `dep:` 语法 (Rust 1.60+) 来定义可选依赖，避免创建隐式特性。
- 在你的 README 和 crate 级文档中记录特性标志。

### 工作空间 (Workspace) 组织

对于大型项目，请使用 Cargo 工作空间来共享依赖项和编译产物：

```toml
# 根目录的 Cargo.toml
[workspace]
members = [
    "core",         // 共享的类型和特性
    "parser",       // 解析库
    "server",       // 二进制程序 —— 主应用
    "client",       // 客户端库
    "cli",          // CLI 二进制程序
]

# 共享依赖版本：
[workspace.dependencies]
serde = { version = "1", features = ["derive"] }
tokio = { version = "1", features = ["full"] }
tracing = "0.1"

# 在每个成员的 Cargo.toml 中：
# [dependencies]
# serde = { workspace = true }
```

**优势**：
- 单一的 `Cargo.lock` —— 所有 crate 使用相同的依赖版本。
- `cargo test --workspace` 运行所有测试。
- 共享编译缓存 —— 编译一个 crate 会使所有 crate 受益。
- 组件之间拥有清晰的依赖边界。

### `.cargo/config.toml`：项目级配置

`.cargo/config.toml` 文件（位于工作空间根目录或 `$HOME/.cargo/` 中）可以在不修改 `Cargo.toml` 的情况下自定义 Cargo 行为：

```toml
# .cargo/config.toml

# 当前工作空间的默认目标平台
[build]
target = "x86_64-unknown-linux-gnu"

# 自定义运行程序 —— 例如，通过 QEMU 运行交叉编译的二进制程序
[target.aarch64-unknown-linux-gnu]
runner = "qemu-aarch64-static"
linker = "aarch64-linux-gnu-gcc"

# Cargo 别名 —— 自定义快捷命令
[alias]
xt = "test --workspace --release"        # cargo xt = 以发布模式运行所有测试
ci = "clippy --workspace -- -D warnings" # cargo ci = 将警告视为错误进行 lint 检查
cov = "llvm-cov --workspace"             # cargo cov = 覆盖率检查 (需要 cargo-llvm-cov)

# 编译脚本的环境变量
[env]
IPMI_LIB_PATH = "/usr/lib/bmc"
```

### 编译时环境变量：`env!()` 与 `option_env!()`

Rust 可以在编译时将环境变量嵌入二进制程序中 —— 对于版本字符串、编译元数据和配置非常有用：

```rust
// env!() —— 如果变量缺失，则在编译时产生 panic
const VERSION: &str = env!("CARGO_PKG_VERSION"); // 来自 Cargo.toml 的 "0.1.0"
const PKG_NAME: &str = env!("CARGO_PKG_NAME");   // 来自 Cargo.toml 的 crate 名称

// option_env!() —— 返回 Option<&str>，如果缺失则不产生 panic
const BUILD_SHA: Option<&str> = option_env!("GIT_SHA");
const BUILD_TIME: Option<&str> = option_env!("BUILD_TIMESTAMP");

fn print_version() {
    println!("{PKG_NAME} v{VERSION}");
    if let Some(sha) = BUILD_SHA {
        println!("  提交 ID: {sha}");
    }
    if let Some(time) = BUILD_TIME {
        println!("  编译时间: {time}");
    }
}
```

Cargo 会自动设置许多有用的环境变量：

| 变量 | 值 | 使用场景 |
|----------|-------|----------|
| `CARGO_PKG_VERSION` | `"1.2.3"` | 版本报告 |
| `CARGO_PKG_NAME` | `"diag_tool"` | 二进制标识 |
| `CARGO_MANIFEST_DIR` | `Cargo.toml` 的绝对路径 | 定位测试数据文件 |
| `OUT_DIR` | 编译输出目录 | `build.rs` 的代码生成目标 |

你也可以从 `build.rs` 中设置自定义环境变量：
```rust
// build.rs
fn main() {
    println!("cargo::rustc-env=GIT_SHA={}", git_sha());
    println!("cargo::rustc-env=BUILD_TIMESTAMP={}", timestamp());
}
```

### `cfg_attr`：条件属性

`cfg_attr` 仅当条件满足时才会应用属性。这比 `#[cfg()]` 更有针对性，因为后者会包含或排除整个项：

```rust
// 仅当开启了 "serde" 特性时才派生 Serialize：
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone)]
pub struct DiagResult {
    pub fc: u32,
    pub passed: bool,
    pub message: String,
}
// 没有 "serde" 特性时：完全不需要依赖 serde
// 开启 "serde" 特性后：DiagResult 变得可序列化

// 测试时的条件属性：
#[cfg_attr(test, derive(PartialEq))]  // 仅在测试构建中派生 PartialEq
pub struct LargeStruct { /* ... */ }

// platform 特定的函数属性：
#[cfg_attr(target_os = "linux", link_name = "ioctl")]
#[cfg_attr(target_os = "freebsd", link_name = "__ioctl")]
extern "C" fn platform_ioctl(fd: i32, request: u64) -> i32;
```

| 模式 | 作用 |
|---------|-------------|
| `#[cfg(feature = "x")]` | 包含或排除整个项 |
| `#[cfg_attr(feature = "x", derive(Foo))]` | 仅在特性 "x" 开启时添加 `derive(Foo)` |
| `#[cfg_attr(test, allow(unused))]` | 仅在测试构建中抑制警告 |
| `#[cfg_attr(doc, doc = "...")]` | 仅在运行 `cargo doc` 时可见的文档 |

### `cargo deny` 与 `cargo audit`：供应链安全

```bash
# 安装安全审计工具
cargo install cargo-deny
cargo install cargo-audit

# 检查依赖项中的已知漏洞 (CVE)
cargo audit

# 进行全面检查：许可证、封禁列表、公告、来源
cargo deny check
```

可以在工作空间根目录通过 `deny.toml` 来配置 `cargo deny`：

```toml
# deny.toml
[advisories]
vulnerability = "deny"      # 发现已知漏洞时失败
unmaintained = "warn"        # 对停止维护的 crate 发出警告

[licenses]
allow = ["MIT", "Apache-2.0", "BSD-2-Clause", "BSD-3-Clause"]
deny = ["GPL-3.0"]          # 拒绝“左版” (Copyleft) 许可证

[bans]
multiple-versions = "warn"  # 如果同一 crate 存在多个版本则警告
deny = [
    { name = "openssl" },   # 强制改用 rustls
]

[sources]
allow-git = []              # 生产环境中不允许使用 git 依赖
```

### 文档测试：文档内部的测试

Rust 的文档注释 (`///`) 可以包含代码块，这些代码块会被 **作为测试编译并运行**：

```rust
/// 从字符串中解析诊断错误码 (Fault Code)。
///
/// # 示例
///
/// ```
/// use my_crate::parse_fc;
///
/// let fc = parse_fc("FC:12345").unwrap();
/// assert_eq!(fc, 12345);
/// ```
///
/// 无效输入会返回错误：
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
cargo test --doc  # 仅运行文档测试
cargo test        # 运行单元测试 + 集成测试 + 文档测试
```

**模块级文档** 在文件顶部使用 `//!`：

```rust
//! # 诊断框架 (Diagnostic Framework)
//!
//! 本 crate 提供了核心的诊断执行引擎。
//! 它支持运行诊断测试、收集结果，并经由 IPMI 向 BMC 报告。
//!
//! ## 快速开始
//!
//! ```no_run
//! use diag_framework::Framework;
//!
//! let mut fw = Framework::new("config.json")?;
//! fw.run_all_tests()?;
//! ```
```

### 使用 Criterion 进行基准测试

> **完整覆盖**：关于 `criterion` 的完整设置、API 示例以及与 `cargo bench` 的对比表，请参阅第 14 章（测试与基准模式）中的[使用 criterion 进行基准测试](ch14-testing-and-benchmarking-patterns.md#使用-criterion-进行基准测试)部分。以下是针对架构设计的快速参考。

在对你的 crate 的公共 API 进行基准测试时，请将基准测试代码放置在 `benches/` 目录下，并专注于 **热点路径 (Hot path)** —— 通常是解析器、序列化器或校验边界：

```bash
cargo bench                  # 运行所有基准测试
cargo bench -- parse_config  # 运行特定的基准测试
# 结果位于 target/criterion/，附带 HTML 报告
```

> **关键要点 —— 架构与 API 设计**
> - **接收最通用的类型** (`impl Into`、`impl AsRef`、`Cow`)；**返回最具体的类型**信号。
> - **以解析代替校验**：使用 `TryFrom` 创建“构造即有效”的类型。
> - 在公共枚举上使用 **`#[non_exhaustive]`** 可以防止添加新变体时造成破坏性变更。
> - **`#[must_use]`** 能捕获那些被无意忽略的重要返回值。

> **另请参阅：** [第 10 章](ch10-error-handling-patterns.md) 了解公共 API 中的错误类型设计。[第 14 章](ch14-testing-and-benchmarking-patterns.md) 了解如何测试你的 crate 公共 API。

---

### 练习：Crate API 重构 ★★ (~30 分钟)

将以下“强依赖字符串”的 API 重构为使用 `TryFrom`、新类型和构建器模式的 API：

```rust,ignore
// 重构前：容易被误用
fn create_server(host: &str, port: &str, max_conn: &str) -> Server { ... }
```

设计一个带有如下验证过类型的 `ServerConfig`：`Host`、`Port` (1–65535) 和 `MaxConnections` (1–10000) ，要求在解析时就能拒绝无效值。

<details>
<summary>🔑 参考答案</summary>

```rust
#[derive(Debug, Clone)]
struct Host(String);

impl TryFrom<&str> for Host {
    type Error = String;
    fn try_from(s: &str) -> Result<Self, String> {
        if s.is_empty() { return Err("主机名不能为空".into()); }
        if s.contains(' ') { return Err("主机名不能包含空格".into()); }
        Ok(Host(s.to_string()))
    }
}

#[derive(Debug, Clone, Copy)]
struct Port(u16);

impl TryFrom<u16> for Port {
    type Error = String;
    fn try_from(p: u16) -> Result<Self, String> {
        if p == 0 { return Err("端口号必须 >= 1".into()); }
        Ok(Port(p))
    }
}

#[derive(Debug, Clone, Copy)]
struct MaxConnections(u32);

impl TryFrom<u32> for MaxConnections {
    type Error = String;
    fn try_from(n: u32) -> Result<Self, String> {
        if n == 0 || n > 10_000 {
            return Err(format!("max_connections 必须在 1–10000 之间，得到了 {n}"));
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

    // 无效值在解析时就会被捕获：
    assert!(Host::try_from("").is_err());
    assert!(Port::try_from(0).is_err());
    assert!(MaxConnections::try_from(99999).is_err());
}
```

</details>

***
