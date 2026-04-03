[English Original](../en/ch08-crates-and-modules.md)

# Rust Crates 与 Modules (单元包与模块)

> **你将学到：** Rust 如何将代码组织为模块 (Modules) 和单元包 (Crates) —— 默认私有的可见性规则、`pub` 修饰符、工作区 (Workspaces) 以及 `crates.io` 生态系统。这些将取代 C/C++ 中的头文件、`#include` 以及 CMake 的依赖管理。

- 在 Crates 内部，**模块 (Modules)** 是组织代码的基本单位。
    - 每个源文件 (.rs) 都是一个独立的模块，并且可以使用 `mod` 关键字创建嵌套子模块。
    - 模块（及其子模块）内部的所有类型默认为**私有 (Private)**。除非显式标记为 `pub` (Public)，否则它们在当前 Crate 外部是不可见的。`pub` 的作用域可以进一步限定，例如 `pub(crate)` 等。
    - 即使一个类型是公开的，它也不会在另一个模块的作用域内自动可见，除非使用 `use` 关键字将其导入。子模块可以使用 `use super::` 来引用父级作用域中的类型。
    - 源文件 (.rs) 不会自动被包含到 Crate 中，除非在 `main.rs`（可执行文件入口）或 `lib.rs`（库出口）中显式列出。

# 练习：模块与函数
- 接下来我们将修改之前的 [hello world](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=522d86dbb8c4af71ff2ec081fb76aee7) 程序，尝试调用另一个函数。
    - 正如之前提到的，函数使用 `fn` 关键字定义。`->` 关键字用于声明函数的返回值类型（默认为空，即 void），例如 `u32`（无符号 32 位整数）。
    - 函数的作用域受模块限制，这意味着在两个不同模块中定义同名函数不会导致命名冲突。
        - 模块作用域同样适用于所有类型（例如：`mod a { struct foo; }` 中的 `foo` 类型是 `a::foo`，这与 `mod b { struct foo; }` 中的 `b::foo` 是截然不同的两个类型）。

**初始代码** —— 请补全函数实现：
```rust
mod math {
    // TODO: 实现 pub fn add(a: u32, b: u32) -> u32
}

fn greet(name: &str) -> String {
    // TODO: 返回 "Hello, <name>! The secret number is <math::add(21,21)>"
    todo!()
}

fn main() {
    println!("{}", greet("Rustacean"));
}
```

<details><summary>参考答案 (点击展开)</summary>

```rust
mod math {
    pub fn add(a: u32, b: u32) -> u32 {
        a + b
    }
}

fn greet(name: &str) -> String {
    format!("Hello, {}! The secret number is {}", name, math::add(21, 21))
}

fn main() {
    println!("{}", greet("Rustacean"));
}
// 输出: Hello, Rustacean! The secret number is 42
```

</details>

---

## 工作区 (Workspaces) 与单元包 (Crates/Packages)

- 任何稍具规模的 Rust 项目都应当使用**工作区**来组织构成该项目的各单元包 (Crates)。
    - 工作区可以包含一系列被编译进目标二进制文件的本地 Crates。在工作区根目录下的 `Cargo.toml` 应当包含指向这些构成包 (Packages/Crates) 的指针。

```toml
[workspace]
resolver = "2"
members = ["package1", "package2"]
```

```text
workspace_root/
|-- Cargo.toml      # 工作区配置
|-- package1/
|   |-- Cargo.toml  # 包 (Package) 1 配置
|   `-- src/
|       `-- lib.rs  # 包 (Package) 1 源代码
|-- package2/
|   |-- Cargo.toml  # 包 (Package) 2 配置
|   `-- src/
|       `-- main.rs # 包 (Package) 2 源代码
```

---

## 练习：使用工作区与包依赖
- 接下来我们将创建一个简单的工作区，并在我们的 `hello world` 程序中使用包。
- 首先创建工作区目录：
```bash
mkdir workspace
cd workspace
```
- 创建 `Cargo.toml` 文件并添加如下内容。这将创建一个空工作区。
```toml
[workspace]
resolver = "2"
members = []
```
- 添加包（`cargo new --lib` 用于创建一个库而非可执行程序）：
```bash
cargo new hello
cargo new --lib hellolib
```

- 请查看 `hello` 和 `hellolib` 目录下生成的 `Cargo.toml`。注意，它们都已被添加到上一层的 `Cargo.toml` 中。
- `hellolib` 目录下的 `lib.rs` 表明它是一个库 (Library) 包（更多自定义选项详见 [Cargo 目标配置](https://doc.rust-lang.org/cargo/reference/cargo-targets.html)）。
- 在 `hello` 的 `Cargo.toml` 中为 `hellolib` 添加依赖项：
```toml
[dependencies]
hellolib = {path = "../hellolib"}
```
- 调用 `hellolib` 中的 `add()` 函数：
```rust
fn main() {
    println!("Hello, world! {}", hellolib::add(21, 21));
}
```

<details><summary>参考答案 (点击展开)</summary>

完整的工作区配置如下：

```bash
# 终端命令
mkdir workspace && cd workspace

# 创建名为 Cargo.toml 的工作区配置文件
cat > Cargo.toml << 'EOF'
[workspace]
resolver = "2"
members = ["hello", "hellolib"]
EOF

cargo new hello
cargo new --lib hellolib
```

```toml
# hello/Cargo.toml —— 添加依赖项
[dependencies]
hellolib = {path = "../hellolib"}
```

```rust
// hellolib/src/lib.rs —— 执行 cargo new --lib 时通常会自动生成 add()
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}
```

```rust,ignore
// hello/src/main.rs
fn main() {
    println!("Hello, world! {}", hellolib::add(21, 21));
}
// 输出: Hello, world! 42
```

</details>

---

# 使用来自 crates.io 的社区单元包 (Crates)
- Rust 拥有一个充满活力的社区单元包生态系统（详见 [crates.io](https://crates.io/)）。
    - Rust 的哲学是保持标准库的精简，并将具体功能外包给社区单元包。
    - 关于使用社区单元包并没有死板的规定，但一般的原则是确保该单元包具有一定的成熟度（由版本号体现）且正被积极维护。如果对某个单元包有疑问，请咨询内部资源。
- 在 `crates.io` 上发布的每个单元包都有一个主版本号 (Major) 和次版本号 (Minor)。
    - 单元包应当遵守此处定义的 `SemVer`（语义化版本控制）指南：[Cargo 语义化版本控制](https://doc.rust-lang.org/cargo/reference/semver.html)。
    - 简而言之：在同一个次版本号内不应有破坏性变更 (Breaking changes)。例如，v0.11 必须与 v0.15 兼容（但 v0.20 可能会有破坏性变更）。

# Crates 依赖与语义化版本 (SemVer)
- Crates 可以定义对某个单元包特定版本、特定次/主版本或者任意版本的依赖。以下示例展示了在 `Cargo.toml` 中声明对 `rand` 单元包依赖的不同方式。
- 至少是 `0.10.0`，且任何 `< 0.11.0` 的版本均可：
```toml
[dependencies]
rand = { version = "0.10.0"}
```
- 仅限 `0.10.0`，不接受其他版本：
```toml
[dependencies]
rand = { version = "=0.10.0"}
```
- 任意版本；`cargo` 将选择最新版本：
```toml
[dependencies]
rand = { version = "*"}
```
- 参考文档：[指定依赖项](https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html)。

---

# 练习：使用 rand 单元包
- 修改之前的 `helloworld` 程序，尝试打印一个随机数。
- 使用 `cargo add rand` 命令添加依赖项。
- 参考 [rand 官方文档](https://docs.rs/rand/latest/rand/) 获取 API 信息。

**初始代码** —— 在运行 `cargo add rand` 后，将以下内容添加至 `main.rs`：
```rust,ignore
use rand::RngExt;

fn main() {
    let mut rng = rand::rng();
    // TODO: 生成并打印一个在 1..=100 范围内的随机 u32
    // TODO: 生成并打印一个随机布尔值 (bool)
    // TODO: 生成并打印一个随机浮点数 (f64)
}
```

<details><summary>参考答案 (点击展开)</summary>

```rust
use rand::RngExt;

fn main() {
    let mut rng = rand::rng();
    let n: u32 = rng.random_range(1..=100);
    println!("随机数 (1-100): {n}");

    // 生成一个随机布尔值
    let b: bool = rng.random();
    println!("随机布尔值: {b}");

    // 生成一个介于 0.0 和 1.0 之间的随机浮点数
    let f: f64 = rng.random();
    println!("随机浮点数: {f:.4}");
}
```

</details>

---

# Cargo.toml 与 Cargo.lock
- 正如前文所述，`Cargo.lock` 是根据 `Cargo.toml` 自动生成的。
    - `Cargo.lock` 的核心作用是确保构建的**可复现性 (Reproducible builds)**。例如，如果 `Cargo.toml` 指定的版本是 `0.10.0`，cargo 可以在符合语义化版本规则的情况下自由选择任何 `< 0.11.0` 的版本。
    - `Cargo.lock` 记录了在某次具体构建中所使用的单元包的**确切**版本。
    - 建议将 `Cargo.lock` 文件提交至 Git 仓库，以确保所有开发者在构建时使用完全一致的依赖版本。

## Cargo 的测试 (Test) 功能
- 按惯例，Rust 的单元测试与源代码位于同一文件中，并通常被组织在一个独立的模块内。
    - 测试代码绝不会被包含在最终生成的二进制文件中。这得益于 `#[cfg(test)]` (配置项测试) 装饰器。配置项装饰器在编写针对特定平台（如 `Linux` vs `Windows`）的代码时非常有用。
    - 测试可以通过 `cargo test` 命令执行。
    - 参考：[条件编译](https://doc.rust-lang.org/reference/conditional-compilation.html)。

```rust
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}
// 该模块仅在执行测试时被包含
#[cfg(test)]
mod tests {
    use super::*; // 让父级作用域的所有类型在此可见
    #[test]
    fn it_works() {
        let result = add(2, 2); // 或者使用 super::add(2, 2);
        assert_eq!(result, 4);
    }
}
```

# 其他 Cargo 功能
- `cargo` 还提供了若干实用的功能：
    - `cargo clippy`: 用于代码静态分析 (Lint)。通常应修复这些警告（除非极个别情况下确需抑制）。
    - `cargo format`: 运行 `rustfmt` 工具来格式化源代码。使用此工具可确保代码风格的统一，杜绝关于风格冲突的争论。
    - `cargo doc`: 用于根据 `///` 风格的注释生成文档。`crates.io` 上所有单元包的文档均是通过此方法生成的。

---

### 构建配置 (Build Profiles)：控制优化级别

在 C 语言中，你会向 `gcc`/`clang` 传递 `-O0`, `-O2`, `-Os`, `-flto` 等参数。而在 Rust 中，你可以在 `Cargo.toml` 中配置构建配置：

```toml
# Cargo.toml —— 构建配置项

[profile.dev]
opt-level = 0          # 无优化 (编译速度快，类似于 -O0)
debug = true           # 完整的调试符号 (类似于 -g)

[profile.release]
opt-level = 3          # 最大程度优化 (类似于 -O3)
lto = "fat"            # 链接时优化 (类似于 -flto)
strip = true           # 移除符号表 (类似于 strip 命令)
codegen_units = 1      # 单个代码生成单元 —— 编译慢，但优化效果更好
panic = "abort"        # 无展开表 (Unwind tables)，减小二进制体积
```

| C/GCC 标志 | Cargo.toml 配置项 | 可选值 |
|------------|---------------|--------|
| `-O0` / `-O2` / `-O3` | `opt-level` | `0`, `1`, `2`, `3`, `"s"`, `"z"` |
| `-flto` | `lto` | `false`, `"thin"`, `"fat"` |
| `-g` / no `-g` | `debug` | `true`, `false`, `"line-tables-only"` |
| `strip` 命令 | `strip` | `"none"`, `"debuginfo"`, `"symbols"`, `true`/`false` |
| — | `codegen_units` | `1` = 最佳优化，最慢编译 |

```bash
cargo build              # 使用 [profile.dev]
cargo build --release    # 使用 [profile.release]
```

---

### 构建脚本 (`build.rs`)：链接 C 语言库

在 C 语言中，你通过 Makefiles 或 CMake 来链接库并执行代码生成。而 Rust 在单元包的根目录下使用 `build.rs` 文件：

```rust
// build.rs —— 在编译单元包之前运行

fn main() {
    // 链接系统 C 库 (类似于 gcc 中的 -lbmc_ipmi)
    println!("cargo::rustc-link-lib=bmc_ipmi");

    // 指定库的查找路径 (类似于 -L/usr/lib/bmc)
    println!("cargo::rustc-link-search=/usr/lib/bmc");

    // 如果 C 语言头文件发生变化，则重新运行
    println!("cargo::rerun-if-changed=wrapper.h");
}
```

你甚至可以直接从 Rust 单元包中编译 C 语言源码：

```toml
# Cargo.toml
[build-dependencies]
cc = "1"  # C 编译器集成
```

```rust
// build.rs
fn main() {
    cc::Build::new()
        .file("src/c_helpers/ipmi_raw.c")
        .include("/usr/include/bmc")
        .compile("ipmi_raw");   // 生成 libipmi_raw.a 并自动完成链接
    println!("cargo::rerun-if-changed=src/c_helpers/ipmi_raw.c");
}
```

| C / Make / CMake | Rust 的 `build.rs` |
|-----------------|-----------------|
| `-lfoo` | `println!("cargo::rustc-link-lib=foo")` |
| `-L/path` | `println!("cargo::rustc-link-search=/path")` |
| 编译 C 源码 | `cc::Build::new().file("foo.c").compile("foo")` |
| 生成代码 | 将文件写入 `$OUT_DIR`，然后通过 `include!()` 引入 |

---

### 交叉编译 (Cross-Compilation)

在 C 语言中，交叉编译需要安装独立的工具链（如 `arm-linux-gnueabihf-gcc`）并配置 Make/CMake。而 Rust 的操作如下：

```bash
# 安装一个交叉编译目标
rustup target add aarch64-unknown-linux-gnu

# 执行交叉编译
cargo build --target aarch64-unknown-linux-gnu --release
```

在 `.cargo/config.toml` 中指定链接器：

```toml
[target.aarch64-unknown-linux-gnu]
linker = "aarch64-linux-gnu-gcc"
```

| C 语言交叉编译 | Rust 对应方法 |
|-----------------|-----------------|
| `apt install gcc-aarch64-linux-gnu` | `rustup target add aarch64-unknown-linux-gnu` + 安装相应链接器 |
| `CC=aarch64-linux-gnu-gcc make` | 在 `.cargo/config.toml` 中配置 `[target.X] linker = "..."` |
| `#ifdef __aarch64__` | `#[cfg(target_arch = "aarch64")]` |
| 独立的 Makefile target | 使用 `cargo build --target ...` |

---

### 特性标记 (Feature Flags)：条件编译

C 语言使用 `#ifdef` 和 `-DFOO` 进行条件编译。Rust 则使用在 `Cargo.toml` 中定义的特性标记：

```toml
# Cargo.toml
[features]
default = ["json"]         # 默认启用
json = ["dep:serde_json"]  # 可选依赖
verbose = []               # 仅作为标记，不带依赖
gpu = ["dep:cuda-sys"]     # 可选的 GPU 支持
```

```rust
// 由特性标记管控的代码：
#[cfg(feature = "json")]
pub fn parse_config(data: &str) -> Result<Config, Error> {
    serde_json::from_str(data).map_err(Error::from)
}

#[cfg(feature = "verbose")]
macro_rules! verbose {
    ($($arg:tt)*) => { eprintln!("[VERBOSE] {}", format!($($arg)*)); }
}
#[cfg(not(feature = "verbose"))]
macro_rules! verbose {
    ($($arg:tt)*) => {}; // 编译为空操作
}
```

| C 预处理器 | Rust 特性标记 |
|---------------|-------------------|
| `gcc -DDEBUG` | `cargo build --features verbose` |
| `#ifdef DEBUG` | `#[cfg(feature = "verbose")]` |
| `#define MAX 100` | `const MAX: u32 = 100;` |
| `#ifdef __linux__` | `#[cfg(target_os = "linux")]` |

---

### 集成测试 (Integration Tests) vs 单元测试 (Unit Tests)

单元测试与代码位于同一文件中并标有 `#[cfg(test)]`。而**集成测试**则位于项目根目录下的 `tests/` 目录中，且**只能**测试该单元包的**公开 API**：

```rust
// tests/smoke_test.rs —— 此处不需要 #[cfg(test)]
use my_crate::parse_config;

#[test]
fn parse_valid_config() {
    let config = parse_config("test_data/valid.json").unwrap();
    assert_eq!(config.max_retries, 5);
}
```

| 测试维度 | 单元测试 (`#[cfg(test)]`) | 集成测试 (`tests/`) |
|--------|----------------------------|------------------------------|
| 位置 | 与源码同文件 | 独立的 `tests/` 目录 |
| 访问权限 | 能够访问私有及公开项 | **仅限公开 API** |
| 运行命令 | `cargo test` | `cargo test --test smoke_test` |

---

### 测试模式与策略

对于 C 语言固件开发团队来说，通常需要使用 CUnit、CMocka 或自定义框架来编写测试，且伴随着大量的样板代码。而 Rust 内置的测试框架功能远比此强大。本节将涵盖生产环境代码中所需的测试模式。

#### `#[should_panic]` —— 测试预期的失败

```rust
// 测试特定条件是否会触发 Panic（类似于 C 语言中的断言失败）
#[test]
#[should_panic(expected = "index out of bounds")]
fn test_bounds_check() {
    let v = vec![1, 2, 3];
    let _ = v[10];  // 应当触发 Panic
}

#[test]
#[should_panic(expected = "temperature exceeds safe limit")]
fn test_thermal_shutdown() {
    fn check_temperature(celsius: f64) {
        if celsius > 105.0 {
            panic!("temperature exceeds safe limit: {celsius}°C");
        }
    }
    check_temperature(110.0);
}
```

#### `#[ignore]` —— 耗时较长或依赖硬件的测试

```rust
// 标记需要特殊外部条件的测试（类似于 C 语言中的 #ifdef HARDWARE_TEST）
#[test]
#[ignore = "requires GPU hardware"]
fn test_gpu_ecc_scrub() {
    // 该测试仅在带有 GPU 的机器上运行
    // 运行方式：cargo test -- --ignored
    // 运行方式：cargo test -- --include-ignored  (运行包括被忽略测试在内的所有测试)
}
```

#### 返回 Result 的测试 (取代大量的 `unwrap` 链)

```rust
// 相比于写一堆会掩盖实际错误原因的 unwrap()，这样做更好：
#[test]
fn test_config_parsing() -> Result<(), Box<dyn std::error::Error>> {
    let json = r#"{"hostname": "node-01", "port": 8080}"#;
    let config: ServerConfig = serde_json::from_str(json)?;  // 使用 ? 而非 unwrap()
    assert_eq!(config.hostname, "node-01");
    assert_eq!(config.port, 8080);
    Ok(())  // 如果代码运行到此处且未出错，则测试通过
}
```

---

#### 使用 Builder 函数构建测试脚手架 (Test Fixtures)

C 语言通常使用 `setUp()`/`tearDown()` 函数。而 Rust 则利用 Helper 函数和 `Drop` Trait 来实现：

```rust
struct TestFixture {
    temp_dir: std::path::PathBuf,
    config: Config,
}

impl TestFixture {
    fn new() -> Self {
        let temp_dir = std::env::temp_dir().join(format!("test_{}", std::process::id()));
        std::fs::create_dir_all(&temp_dir).unwrap();
        let config = Config {
            log_dir: temp_dir.clone(),
            max_retries: 3,
            ..Default::default()
        };
        Self { temp_dir, config }
    }
}

impl Drop for TestFixture {
    fn drop(&mut self) {
        // 自动清理 —— 类似于 C 语言中的 tearDown()，但绝不会被遗忘
        let _ = std::fs::remove_dir_all(&self.temp_dir);
    }
}

#[test]
fn test_with_fixture() {
    let fixture = TestFixture::new();
    // 使用 fixture.config, fixture.temp_dir...
    assert!(fixture.temp_dir.exists());
    // fixture 在此处被自动释放 (Drop) ——> 清理逻辑自动运行
}
```

---

#### 针对硬件接口构建 Trait Mock

在 C 语言中，模拟 (Mocking) 硬件往往需要预处理器技巧或函数指针替换。而在 Rust 中，使用 Trait 是一种非常自然的选择：

```rust
// 用于 IPMI 通信的生产环境 Trait
trait IpmiTransport {
    fn send_command(&self, cmd: u8, data: &[u8]) -> Result<Vec<u8>, String>;
}

// 真实实现 (用于生产环境)
struct RealIpmi { /* BMC 链接细节 */ }
impl IpmiTransport for RealIpmi {
    fn send_command(&self, cmd: u8, data: &[u8]) -> Result<Vec<u8>, String> {
        // 与真实的 BMC 硬件通信
        todo!("Real IPMI call")
    }
}

// Mock 实现 (用于测试)
struct MockIpmi {
    responses: std::collections::HashMap<u8, Vec<u8>>,
}
impl IpmiTransport for MockIpmi {
    fn send_command(&self, cmd: u8, _data: &[u8]) -> Result<Vec<u8>, String> {
        self.responses.get(&cmd)
            .cloned()
            .ok_or_else(|| format!("未配置 cmd 0x{cmd:02x} 的 Mock 响应"))
    }
}

// 既能配合真实实现，也能配合 Mock 实现工作的通用函数
fn read_sensor_temperature(transport: &dyn IpmiTransport) -> Result<f64, String> {
    let response = transport.send_command(0x2D, &[])?;
    if response.len() < 2 {
        return Err("响应长度过短".into());
    }
    Ok(response[0] as f64 + (response[1] as f64 / 256.0))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_temperature_reading() {
        let mut mock = MockIpmi { responses: std::collections::HashMap::new() };
        mock.responses.insert(0x2D, vec![72, 128]); // 72.5°C

        let temp = read_sensor_temperature(&mock).unwrap();
        assert!((temp - 72.5).abs() < 0.01);
    }

    #[test]
    fn test_short_response() {
        let mock = MockIpmi { responses: std::collections::HashMap::new() };
        // 未配置 Mock 响应 ——> 报错
        assert!(read_sensor_temperature(&mock).is_err());
    }
}
```

---

#### 利用 `proptest` 进行基于属性的测试 (Property-Based Testing)

与其编写特定的测试用例，不如测试那些在任何输入下都应当成立的**属性 (Properties)**。`proptest` 会生成随机输入并寻找能使测试失败的最小用例：

```rust
// Cargo.toml: [dev-dependencies] proptest = "1"
use proptest::prelude::*;

fn parse_sensor_id(s: &str) -> Option<u32> {
    s.strip_prefix("sensor_")?.parse().ok()
}

fn format_sensor_id(id: u32) -> String {
    format!("sensor_{id}")
}

proptest! {
    #[test]
    fn roundtrip_sensor_id(id in 0u32..10000) {
        // 属性：格式化后再解析应当得到原始值
        let formatted = format_sensor_id(id);
        let parsed = parse_sensor_id(&formatted);
        prop_assert_eq!(parsed, Some(id));
    }

    #[test]
    fn parse_rejects_garbage(s in "[^s].*") {
        // 属性：不以 's' 开头的字符串应当解析失败
        let result = parse_sensor_id(&s);
        prop_assert!(result.is_none());
    }
}
```

---

#### C 语言 vs Rust 测试对比

| C 语言测试 | Rust 对应方法 |
|-----------|----------------|
| `CUnit`, `CMocka`, 自定义框架 | 内置的 `#[test]` + `cargo test` |
| `setUp()` / `tearDown()` | Builder 函数 + `Drop` Trait |
| `#ifdef TEST` 模拟函数 | 基于 Trait 的依赖注入 |
| `assert(x == y)` | `assert_eq!(x, y)` 且带自动的差异 (Diff) 输出 |
| 独立的测试可执行文件 | 与源码位于同一二进制文件，通过 `#[cfg(test)]` 进行条件编译 |
| `valgrind --leak-check=full ./test` | `cargo test` (默认内存安全) + `cargo miri test` |
| 代码覆盖率：`gcov` / `lcov` | `cargo tarpaulin` 或 `cargo llvm-cov` |
| 测试发现：手动注册 | 自动发现 —— 任何带有 `#[test]` 的函数都会被识别 |

---
