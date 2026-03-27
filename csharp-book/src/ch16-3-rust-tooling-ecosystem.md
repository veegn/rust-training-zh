## Essential Rust Tooling for C# Developers | 面向 C# 开发者的 Rust 核心工具链

> **What you'll learn:** Rust's development tools mapped to their C# equivalents - Clippy (Roslyn analyzers),
> rustfmt (dotnet format), cargo doc (XML docs), cargo watch (dotnet watch), and VS Code extensions.
>
> **你将学到什么：** Rust 开发工具如何对应到 C# 生态中的工具，包括 Clippy（类似 Roslyn 分析器）、
> rustfmt（类似 `dotnet format`）、cargo doc（类似 XML 文档生成）、cargo watch（类似 `dotnet watch`），以及常见 VS Code 扩展。
>
> **Difficulty:** Beginner
>
> **难度：** 初级

### Tool Comparison | 工具对照

| C# Tool | Rust Equivalent | Install | Purpose |
|---------|----------------|---------|---------|
| Roslyn analyzers | **Clippy** | `rustup component add clippy` | Lint + style suggestions |
| Roslyn 分析器 | **Clippy** | `rustup component add clippy` | Lint 与代码风格建议 |
| `dotnet format` | **rustfmt** | `rustup component add rustfmt` | Auto-formatting |
| `dotnet format` | **rustfmt** | `rustup component add rustfmt` | 自动格式化 |
| XML doc comments | **`cargo doc`** | Built-in | Generate HTML docs |
| XML 文档注释 | **`cargo doc`** | 内置 | 生成 HTML 文档 |
| OmniSharp / Roslyn | **rust-analyzer** | VS Code extension | IDE support |
| OmniSharp / Roslyn | **rust-analyzer** | VS Code 扩展 | IDE 支持 |
| `dotnet watch` | **cargo-watch** | `cargo install cargo-watch` | Auto-rebuild on save |
| `dotnet watch` | **cargo-watch** | `cargo install cargo-watch` | 保存后自动重建 |
| - | **cargo-expand** | `cargo install cargo-expand` | See macro expansion |
| - | **cargo-expand** | `cargo install cargo-expand` | 查看宏展开结果 |
| `dotnet audit` | **cargo-audit** | `cargo install cargo-audit` | Security vulnerability scan |
| `dotnet audit` | **cargo-audit** | `cargo install cargo-audit` | 安全漏洞扫描 |

### Clippy: Your Automated Code Reviewer | Clippy：你的自动代码审查员
```bash
# Run Clippy on your project
cargo clippy

# Treat warnings as errors (CI/CD)
cargo clippy -- -D warnings

# Auto-fix suggestions
cargo clippy --fix
```

```rust
// Clippy catches hundreds of anti-patterns:

// Before Clippy:
if x == true { }           // warning: equality check with bool
let _ = vec.len() == 0;    // warning: use .is_empty() instead
for i in 0..vec.len() { }  // warning: use .iter().enumerate()

// After Clippy suggestions:
if x { }
let _ = vec.is_empty();
for (i, item) in vec.iter().enumerate() { }
```

### rustfmt: Consistent Formatting | rustfmt：统一格式风格
```bash
# Format all files
cargo fmt

# Check formatting without changing (CI/CD)
cargo fmt -- --check
```

```toml
# rustfmt.toml - customize formatting (like .editorconfig)
max_width = 100
tab_spaces = 4
use_field_init_shorthand = true
```

### cargo doc: Documentation Generation | cargo doc：文档生成
```bash
# Generate and open docs (including dependencies)
cargo doc --open

# Run documentation tests
cargo test --doc
```

```rust
/// Calculate the area of a circle.
///
/// # Arguments
/// * `radius` - The radius of the circle (must be non-negative)
///
/// # Examples
/// ```
/// let area = my_crate::circle_area(5.0);
/// assert!((area - 78.54).abs() < 0.01);
/// ```
///
/// # Panics
/// Panics if `radius` is negative.
pub fn circle_area(radius: f64) -> f64 {
    assert!(radius >= 0.0, "radius must be non-negative");
    std::f64::consts::PI * radius * radius
}
// The code in /// ``` blocks is compiled and run during `cargo test`!
```

### cargo watch: Auto-Rebuild | cargo watch：自动重建
```bash
# Rebuild on file changes (like dotnet watch)
cargo watch -x check          # Type-check only (fastest)
cargo watch -x test           # Run tests on save
cargo watch -x 'run -- args'  # Run program on save
cargo watch -x clippy         # Lint on save
```

### cargo expand: See What Macros Generate | cargo expand：查看宏到底生成了什么
```bash
# See the expanded output of derive macros
cargo expand --lib            # Expand lib.rs
cargo expand module_name      # Expand specific module
```

### Recommended VS Code Extensions | 推荐的 VS Code 扩展

| Extension | Purpose |
|-----------|---------|
| **rust-analyzer** | Code completion, inline errors, refactoring |
| **rust-analyzer** | 代码补全、行内报错、重构支持 |
| **CodeLLDB** | Debugger (like Visual Studio debugger) |
| **CodeLLDB** | 调试器（类似 Visual Studio 调试体验） |
| **Even Better TOML** | Cargo.toml syntax highlighting |
| **Even Better TOML** | 为 Cargo.toml 提供更好的高亮与编辑体验 |
| **crates** | Show latest crate versions in Cargo.toml |
| **crates** | 显示 Cargo.toml 中 crate 的最新版本 |
| **Error Lens** | Inline error/warning display |
| **Error Lens** | 直接在代码行内显示错误与警告 |

***

For deeper exploration of advanced topics mentioned in this guide, see the companion training documents:

如果你想继续深入本书中提到的进阶主题，可以阅读这些配套训练文档：

- **[Rust Patterns](../../rust-patterns-book/src/SUMMARY.md)** - Pin projections, custom allocators, arena patterns, lock-free data structures, and advanced unsafe patterns
- **[Rust Patterns](../../rust-patterns-book/src/SUMMARY.md)** - 讲 Pin projection、自定义分配器、arena 模式、无锁数据结构和高级 unsafe 模式
- **[Async Rust Training](../../async-book/src/SUMMARY.md)** - Deep dive into tokio, async cancellation safety, stream processing, and production async architectures
- **[Async Rust Training](../../async-book/src/SUMMARY.md)** - 深入 tokio、异步取消安全、流处理与生产级 async 架构
- **[Rust Training for C++ Developers](../../c-cpp-book/src/SUMMARY.md)** - Useful if your team also has C++ experience; covers move semantics mapping, RAII differences, and template vs generics
- **[Rust Training for C++ Developers](../../c-cpp-book/src/SUMMARY.md)** - 如果团队也有 C++ 背景会很有帮助，涵盖 move 语义映射、RAII 差异、模板与泛型
- **[Rust Training for C Developers](../../c-cpp-book/src/SUMMARY.md)** - Relevant for interop scenarios; covers FFI patterns, embedded Rust debugging, and `no_std` programming
- **[Rust Training for C Developers](../../c-cpp-book/src/SUMMARY.md)** - 适合互操作场景，涵盖 FFI 模式、嵌入式 Rust 调试和 `no_std` 编程
