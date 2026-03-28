# 实战技巧 🟡

> **你将学到：**
> - 无法归入单一章节的实战经验模式
> - 常见陷阱及其修复方案 —— 从 CI 抖动到二进制膨胀
> - 立即可以应用到任何 Rust 项目的快速制胜技巧
>
> **相关章节：** 本书中的每一章 —— 这些技巧贯穿于所有主题

本章收集了在生产环境 Rust 代码库中反复出现的工程模式。每个技巧都是独立的 —— 可以按任何顺序阅读。

---

### 1. `deny(warnings)` 陷阱

**问题**：在源码中使用 `#![deny(warnings)]` 会在 Clippy 增加新 lint 时导致构建中断。
**修复**：在 CI 中改用 `CARGO_ENCODED_RUSTFLAGS="-Dwarnings"`。

### 2. 编译一次，到处测试

**问题**：在 `--lib` 和 `--doc` 之间切换时，`cargo test` 会重新编译。
**修复**：使用 `cargo nextest` 运行测试，并单独运行 `cargo test --doc`。

### 3. 特性标志 (Feature Flag) 卫生

**问题**：单独编译特性时，它们往往是报错的。
**修复**：在 CI 中使用 `cargo-hack --each-feature`。

### 4. 调试构建中使用优化过的依赖

**问题**：由于依赖项（如 `serde`）未优化，Debug 构建非常缓慢。
**修复**：在 `Cargo.toml` 中添加以下配置：

```toml
[profile.dev.package."*"]
opt-level = 2
```

### 5. CI 缓存抖动

**问题**：每个 PR 都会保存一个新缓存，浪费空间。
**修复**：在缓存 Action 中设置 `save-if: ${{ github.ref == 'refs/heads/main' }}`。

### 6. 使用 `SOURCE_DATE_EPOCH` 实现可复现性

**问题**：在 `build.rs` 中使用 `now()` 会使二进制文件不可复现。
**修复**：如果存在 `SOURCE_DATE_EPOCH` 环境变量，请优先使用它。

### 7. 依赖去重

**问题**：重复的 crate（如 `syn` 1.0 和 2.0）会使编译时间倍增。
**修复**：使用 `cargo tree --duplicates` 查找，并用 `cargo update -p <parent-crate>` 统一版本。

### 8. Push 前的烟雾测试

**问题**：Push 后在 CI 中等待 5 分钟才因格式问题报错。
**修复**：Push 之前在本地运行快速检查脚本（例如通过 `cargo-make`）。

### 🏋️ 练习

#### 🟢 练习 1：应用三个技巧
从本章中挑选三个技巧并应用。哪一个对你影响最大？

#### 🟡 练习 2：去重依赖树
在实际项目上运行 `cargo tree --duplicates`。消除至少一个重复项。

### 关键收获
- `[profile.dev.package."*"] opt-level = 2` 是提升开发体验最明显的技巧。
- 避免在源码级别使用 `deny(warnings)`。
- 结合 `cargo-hack` 验证所有的特性组合。
- 每月进行一次依赖项去重可以显著维持编译时间。

***
