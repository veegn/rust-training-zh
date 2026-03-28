# 发布配置与二进制体积 🟡

> **你将学到：**
> - 发布配置 (Release Profile) 解剖：LTO、codegen-units、panic 策略、strip、opt-level
> - Thin LTO 与 Fat LTO 的权衡
> - 使用 `cargo-bloat` 进行二进制体积分析
> - 使用 `cargo-udeps`、`cargo-machete` 和 `cargo-shear` 修剪冗余依赖
>
> **相关章节：** [编译期工具](ch08-compile-time-and-developer-tools.md) — 优化的另一半 · [基准测试](ch03-benchmarking-measuring-what-matters.md) — 在优化前先测量运行时性能 · [依赖管理](ch06-dependency-management-and-supply-chain-s.md) — 修剪依赖可以同时减少体积和编译时间

默认的 `cargo build --release` 已经很不错了。但对于生产环境部署 —— 尤其是要部署到数千台服务器上的单二进制工具 —— “不错”和“极致优化”之间存在巨大的差距。本章介绍各种配置开关以及测量二进制体积的工具。

### 发布配置解剖

Cargo 配置项控制 `rustc` 如何编译你的代码。默认值往往比较保守，以保证兼容性而非追求极致性能。

**生产级优化配置：**

```toml
[profile.release]
lto = true           # 全局 LTO 优化
codegen-units = 1    # 单个生成单元 —— 提供最大化优化机会
panic = "abort"      # 无需回溯逻辑 —— 更小、更快
strip = true         # 移除所有符号 —— 极大减小体积
```

**各项设置的影响：**

| 设置 | 优化建议 | 体积 | 速度 | 编译时间 |
|---------|---------------------|-------------|---------------|--------------|
| `lto = true` | 开启全量 LTO | -10% 到 -20% | +5% 到 +20% | 慢 2-5 倍 |
| `codegen-units = 1` | 设置为 1 | -5% 到 -10% | +5% 到 +10% | 慢 1.5-2 倍 |
| `strip = true` | 开启 | -50% 到 -70% | 无影响 | 无影响 |

### LTO 深度解析：Thin vs Fat

链接时优化 (LTO) 允许 LLVM 跨 crate 进行优化 —— 例如将依赖库中的函数内联到你的业务代码中。

- `lto = true` (Fat LTO)：全局极致优化，编译非常缓慢。
- `lto = "thin"` (Thin LTO)：性价比最高的选择。

### 使用 `cargo-bloat` 进行体积分析

[`cargo-bloat`](https://github.com/RazrFalcon/cargo-bloat) 回答了一个核心问题：“我的二进制文件中，哪些函数和 crate 占用了最多的空间？”

```bash
# 查看各个 crate 的空间占比
cargo bloat --release --crates
```

### 修剪无效依赖

- **`cargo-udeps`**：找出 `Cargo.toml` 中声明了但代码里没用到的依赖（需要 nightly）。
- **`cargo-machete`**：快速但基于启发式的清理工具。
- **`cargo-shear`**：平衡且可靠的清理工具。

### 🏋️ 练习

#### 🟢 练习 1：测量 LTO 的影响
对比默认 release 和开启 `lto = true` + `strip = true` 后的二进制体积差异。

#### 🟡 练习 2：找出占地最大的 Crate
运行 `cargo bloat --release --crates`。识别出最大的依赖项。能否通过禁用其默认特性 (default-features) 来缩小体积？

### 关键收获
- `lto = true` + `codegen-units = 1` + `strip = true` + `panic = "abort"` 是生产环境推荐的发布配置。
- Thin LTO (`lto = "thin"`) 是大多数项目在性能和编译成本之间的最佳平衡点。
- `cargo-bloat` 明确告诉你哪些 crate 占用了空间。
- `cargo-udeps` 发现并清理冗余依赖，减少攻击面和维护负担。

***
