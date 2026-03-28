# Windows 与条件编译 🟡

> **你将学到：**
> - Windows 支持模式：`windows-sys`/`windows` crate 以及 `cargo-xwin`
> - 使用 `#[cfg]` 进行条件编译 —— 由编译器而非预处理器检查
> - 平台抽象架构：何时使用 `#[cfg]` 块，何时使用 trait
> - 从 Linux 交叉编译到 Windows
>
> **相关章节：** [`no_std` 与特性验证](ch09-no-std-and-feature-verification.md) — `cargo-hack` 与特性验证 · [交叉编译](ch02-cross-compilation-one-source-many-target.md) — 通用交叉构建设置 · [构建脚本](ch01-build-scripts-buildrs-in-depth.md) — 由 `build.rs` 发出的 `cfg` 标志

### Windows 支持 — 平台抽象

Rust 的 `#[cfg()]` 属性和 Cargo 特性 (features) 允许单个代码库同时针对 Linux 和 Windows 进行构建。

```rust
pub fn exec_cmd(cmd: &str) -> Result<Output, Error> {
    #[cfg(windows)]
    let mut child = Command::new("cmd").args(["/C", cmd]).spawn()?;

    #[cfg(not(windows))]
    let mut child = Command::new("sh").args(["-c", cmd]).spawn()?;

    child.wait_with_output()
}
```

### `windows-sys` 与 `windows` Crate

用于直接调用 Windows API：

- **`windows-sys`**：原始 FFI 绑定。极小的二进制开销，编译迅速。
- **`windows`**：安全、符合 Rust 习惯的封装。虽然开销稍大，但更易用。

```toml
[target.'cfg(windows)'.dependencies]
windows-sys = { version = "0.59", features = ["Win32_System_Power"] }
```

### 从 Linux 交叉编译到 Windows

- **MinGW (`gnu`)**：通过 `rustup target add x86_64-pc-windows-gnu` 轻松设置。
- **MSVC (`msvc`)**：生产环境推荐。使用 `cargo-xwin` 从 Linux 进行交叉编译。

```bash
cargo install cargo-xwin
cargo xwin build --target x86_64-pc-windows-msvc
```

### 平台抽象架构

在管理多个平台时，通过 trait 解耦业务逻辑与操作系统特定的实现：

1. **应用逻辑**：使用 `impl HardwareAccess`。
2. **平台抽象层**：定义 `trait HardwareAccess`。
3. **平台实现层**：分别为 `LinuxHardware` 和 `WindowsHardware` (通过 cfg 隔离)。

### 🏋️ 练习

#### 🟢 练习 1：平台相关模块
创建一个包含 `#[cfg(unix)]` 和 `#[cfg(windows)]` 实现的 `get_hostname()` 函数。验证两者都能通过 `cargo check`。

#### 🟡 练习 2：使用 `cargo-xwin` 交叉编译
安装 `cargo-xwin` 并从 Linux 为 `x86_64-pc-windows-msvc` 构建一个简单的二进制文件，并验证生成的是 `.exe`。

### 关键收获
- 简单的操作系统差异使用 `#[cfg]`；复杂的抽象使用 trait。
- `windows-sys` 提供原始 FFI；`windows` crate 提供安全封装。
- `cargo-xwin` 允许你在 Linux 上构建 Windows (MSVC) 的二进制文件。
- 即使你的主要目标是 Linux，也要在 CI 中检查 Windows 的编译情况。

***
