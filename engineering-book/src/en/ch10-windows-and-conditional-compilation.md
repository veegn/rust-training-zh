# Windows and Conditional Compilation 🟡

> **What you'll learn:**
> - Windows support patterns: `windows-sys`/`windows` crates, `cargo-xwin`
> - Conditional compilation with `#[cfg]` — checked by the compiler, not the preprocessor
> - Platform abstraction architecture: when `#[cfg]` blocks suffice vs when to use traits
> - Cross-compiling for Windows from Linux
>
> **Cross-references:** [`no_std` & Features](ch09-no-std-and-feature-verification.md) — `cargo-hack` and feature verification · [Cross-Compilation](ch02-cross-compilation-one-source-many-target.md) — general cross-build setup · [Build Scripts](ch01-build-scripts-buildrs-in-depth.md) — `cfg` flags emitted by `build.rs`

### Windows Support — Platform Abstractions

Rust's `#[cfg()]` attributes and Cargo features allow a single codebase to
target both Linux and Windows cleanly.

```rust
pub fn exec_cmd(cmd: &str) -> Result<Output, Error> {
    #[cfg(windows)]
    let mut child = Command::new("cmd").args(["/C", cmd]).spawn()?;

    #[cfg(not(windows))]
    let mut child = Command::new("sh").args(["-c", cmd]).spawn()?;

    child.wait_with_output()
}
```

### The `windows-sys` and `windows` Crates

For calling Windows APIs directly:

- **`windows-sys`**: Raw FFI bindings. Minimal binary size, fast compile.
- **`windows`**: Safe, idiomatic Rust wrappers. Heavier but easier to use.

```toml
[target.'cfg(windows)'.dependencies]
windows-sys = { version = "0.59", features = ["Win32_System_Power"] }
```

### Cross-Compiling for Windows from Linux

- **MinGW (`gnu`)**: Easy to set up on Linux with `rustup target add x86_64-pc-windows-gnu`.
- **MSVC (`msvc`)**: Recommended for production. Use `cargo-xwin` to cross-compile from Linux.

```bash
cargo install cargo-xwin
cargo xwin build --target x86_64-pc-windows-msvc
```

### Platform Abstraction Architecture

When managing multiple platforms, use traits to decouple logic from OS-specific implementations:

1. **Application Logic**: Uses `impl HardwareAccess`.
2. **Platform Abstraction**: Defines `trait HardwareAccess`.
3. **Implementations**: `LinuxHardware` and `WindowsHardware` (cfg-gated).

### 🏋️ Exercises

#### 🟢 Exercise 1: Platform-Conditional Module

Create a module with `#[cfg(unix)]` and `#[cfg(windows)]` implementations of a `get_hostname()` function. Verify both compile with `cargo check`.

#### 🟡 Exercise 2: Cross-Compile for Windows with `cargo-xwin`

Install `cargo-xwin` and build a simple binary for `x86_64-pc-windows-msvc` from Linux. Verify the output is a `.exe`.

### Key Takeaways

- Use `#[cfg]` for simple OS differences; use traits for complex abstractions.
- `windows-sys` is for raw FFI; the `windows` crate provides safe wrappers.
- `cargo-xwin` allows building for Windows (MSVC) from Linux.
- Always check Windows compilation in CI, even if your primary target is Linux.

***
