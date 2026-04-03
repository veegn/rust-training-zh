[English Original](../en/ch16-3-rust-tooling-ecosystem.md)

## C# 开发者的核心 Rust 工具链指南

> **你将学到：** Rust 开发工具与 C# 等效工具的映射关系 —— Clippy (Roslyn 分析器), rustfmt (dotnet format), cargo doc (XML 文档), cargo watch (dotnet watch) 以及 VS Code 扩展。
>
> **难度：** 🟢 入门

### 工具对比表

| C# 工具 | Rust 等效项 | 安装方式 | 用途 |
|---------|----------------|---------|---------|
| Roslyn 分析器 | **Clippy** | `rustup component add clippy` | Lint 检查 + 代码风格建议 |
| `dotnet format` | **rustfmt** | `rustup component add rustfmt` | 自动格式化代码 |
| XML 文档注释 | **`cargo doc`** | 内置 | 生成 HTML 格式文档 |
| OmniSharp / Roslyn | **rust-analyzer** | VS Code 扩展插件 | IDE 语言支持 |
| `dotnet watch` | **cargo-watch** | `cargo install cargo-watch` | 保存文件时自动重新构建 |
| — | **cargo-expand** | `cargo install cargo-expand` | 查看宏展开后的代码 |
| `dotnet audit` | **cargo-audit** | `cargo install cargo-audit` | 安全漏洞扫描 |

### Clippy：你的自动化代码审查员
```bash
# 在你的项目上运行 Clippy
cargo clippy

# 将警告视为错误 (用于 CI/CD 流水线)
cargo clippy -- -D warnings

# 自动修复部分建议
cargo clippy --fix
```

```rust
// Clippy 可以捕获数百种反模式 (Anti-patterns)：

// 使用 Clippy 之前：
if x == true { }           // 警告：不必要的布尔值相等性检查
let _ = vec.len() == 0;    // 警告：请改用 .is_empty()
for i in 0..vec.len() { }  // 警告：请改用 .iter().enumerate()

// 根据 Clippy 建议修改后：
if x { }
let _ = vec.is_empty();
for (i, item) in vec.iter().enumerate() { }
```

### rustfmt：保持统一的代码格式
```bash
# 格式化所有文件
cargo fmt

# 仅检查格式是否正确而不修改 (用于 CI/CD)
cargo fmt -- --check
```

```toml
# rustfmt.toml —— 自定义格式化规则 (类似于 .editorconfig)
max_width = 100
tab_spaces = 4
use_field_init_shorthand = true
```

### cargo doc：文档生成工具
```bash
# 生成并打开文档 (包含所有依赖项的文档)
cargo doc --open

# 运行文档中的测试示例 (Doc-tests)
cargo test --doc
```

```rust
/// 计算圆的面积。
///
/// # 参数
/// * `radius` - 圆的半径 (必须为非负数)
///
/// # 示例
/// ```
/// let area = my_crate::circle_area(5.0);
/// assert!((area - 78.54).abs() < 0.01);
/// ```
///
/// # Panics
/// 如果 `radius` 为负数，则会发生 Panic。
pub fn circle_area(radius: f64) -> f64 {
    assert!(radius >= 0.0, "半径必须为非负数");
    std::f64::consts::PI * radius * radius
}
// 在 /// ``` 代码块中的代码会在运行 `cargo test` 时被编译并执行！
```

### cargo watch：自动重构/运行
```bash
# 文件变动时自动重新构建 (类似于 dotnet watch)
cargo watch -x check          # 仅进行类型检查 (速度最快)
cargo watch -x test           # 保存时运行测试
cargo watch -x 'run -- args'  # 保存时运行程序
cargo watch -x clippy         # 保存时运行 Lint 检查
```

### cargo expand：查看宏生成的代码
```bash
# 查看派生宏展开后的具体代码
cargo expand --lib            # 展开 lib.rs
cargo expand module_name      # 展开特定模块
```

### 推荐的 VS Code 扩展

| 扩展插件 | 用途 |
|-----------|---------|
| **rust-analyzer** | 代码补全、内联错误提示、代码重构 |
| **CodeLLDB** | 调试器 (类似于 Visual Studio 调试器) |
| **Even Better TOML** | Cargo.toml 语法高亮 |
| **crates** | 在 Cargo.toml 中显示最新的 Crate 版本 |
| **Error Lens** | 将错误/警告信息直接显示在代码行末 |

---

若想深入探索本指南中提到的进阶课题，请参阅配套的训练文档：

- **[Rust 设计模式](../../rust-patterns-book/src/SUMMARY.md)** —— 固定投影 (Pin projections)、自定义分配器、Arena 模式、无锁数据结构以及高级不安全 (Unsafe) 模式。
- **[异步 Rust 训练](../../async-book/src/SUMMARY.md)** —— 深度解析 tokio、异步取消安全性、流处理以及生产环境下的异步架构。
- **[面向 C++ 开发者的 Rust 训练](../../c-cpp-book/src/SUMMARY.md)** —— 如果你的团队也有 C++ 经验，此文档涵盖了移动语义映射、RAII 差异以及模板与泛型的对比。
- **[面向 C 开发者的 Rust 训练](../../c-cpp-book/src/SUMMARY.md)** —— 适用于互操作场景，涵盖了 FFI 模式、嵌入式 Rust 调试以及 `no_std` 编程。
