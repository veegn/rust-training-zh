[English Original](../en/ch11-putting-it-all-together-a-production-cic.md)

# 融会贯通：生产级 CI/CD 流水线 🟡

> **你将学到：**
> - 构建多阶段 GitHub Actions CI 工作流（检查 → 测试 → 覆盖率 → 安全 → 交叉编译 → 发布）
> - 使用 `rust-cache` 的缓存策略与 `save-if` 调优
> - 定期并在 nightly 环境下运行 Miri 和 Sanitizer
> - 使用 `Makefile.toml` 和 pre-commit 钩子实现任务自动化
> - 使用 `cargo-dist` 实现自动化发布
>
> **相关章节：** 第 1 至 10 章涵盖了此处集成的各种独立工具。

单个工具很有用，但一个能在每次 Push 时自动编排它们的流水线则是革命性的。本章将前 10 章中介绍的工具组合成一个凝聚力强的 CI/CD 工作流。

### 完整的 GitHub Actions 工作流

建议的多阶段流水线：

1. **Check（检查）**：运行 `clippy`、`rustfmt` 和 `cargo check`。（最快的反馈循环）
2. **Test（测试）**：在 Ubuntu 和 Windows 上运行 `cargo test`。
3. **Cross（交叉编译）**：针对 ARM 和 MUSL 目标进行构建。
4. **Coverage（覆盖率）**：运行 `cargo llvm-cov` 并强制执行最低阈值门禁。
5. **Safety（安全性）**：运行 `cargo miri test` 以验证 unsafe 代码。
6. **Security（安全审核）**：运行 `cargo audit` 和 `cargo deny check`。

### CI 缓存策略

使用 [`Swatinem/rust-cache@v2`](https://github.com/Swatinem/rust-cache) 加速构建。

```yaml
- uses: Swatinem/rust-cache@v2
  with:
    # 仅在 main 分支更新缓存，防止 PR 任务写坏缓存
    save-if: ${{ github.ref == 'refs/heads/main' }}
```

### 使用 `cargo-make` 实现任务自动化

[`cargo-make`](https://sagiegurari.github.io/cargo-make/) 提供了一个可移植的任务运行器，用于取代复杂的 Shell 脚本或依赖特定平台的 Makefile。

```toml
# Makefile.toml
[tasks.dev]
description = "完整的本地验证流程"
dependencies = ["check", "test", "clippy", "fmt-check"]
```

### 使用 `cargo-dist` 自动化发布

[`cargo-dist`](https://github.com/axodotdev/cargo-dist) 自动化了 GitHub Release 的创建过程，涵盖多个平台并自动生成安装脚本。

```bash
cargo dist init
cargo dist plan
```

### 🏋️ 练习

#### 🟢 练习 1：创建基础 CI 工作流
创建一个 `.github/workflows/ci.yml`，包含 `cargo check`、`cargo test` 和 `cargo clippy`。

#### 🟡 练习 2：使用 `cargo-make` 的本地工作流
安装 `cargo-make`，创建一个包含测试和覆盖率检查的 `Makefile.toml`，并验证其在本地可行。

### 关键收获
- 将 CI 结构化为并行阶段，并将最快、成本最低的反馈放在第一阶。
- 使用 `rust-cache` 并进行 `save-if` 调优以避免缓存抖动。
- 使用 `cargo-make` 封装复杂的本地开发工作流。
- 使用 `cargo-dist` 处理多平台发布的复杂性。

***
