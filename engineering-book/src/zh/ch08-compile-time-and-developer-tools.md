# 编译期与开发者工具 🟡

> **你将学到：**
> - 使用 `sccache` 为本地和 CI 构建提供编译缓存
> - 使用 `mold` 实现极速链接（比默认链接器快 3-10 倍）
> - `cargo-nextest`：更快速、信息更丰富的测试运行器
> - 开发者可见性工具：`cargo-expand`、`cargo-geiger`、`cargo-watch`
> - 工作区 Lint、MSRV 策略以及文档即 CI (Documentation-as-CI)
>
> **相关章节：** [发布配置](ch07-release-profiles-and-binary-size.md) — LTO 与二进制体积优化 · [CI/CD 流水线](ch11-putting-it-all-together-a-production-cic.md) — 将这些工具集成到流水线中 · [依赖管理](ch06-dependency-management-and-supply-chain-s.md) — 更少的依赖 = 更快的编译

### 编译期优化：sccache, mold, cargo-nextest

漫长的编译时间是 Rust 开发中的头号痛点。以下工具可以协同工作，将迭代时间缩短 50-80%：

**`sccache` — 共享编译缓存：**
它可以将编译产物缓存到本地磁盘或云端存储 (S3/GCS)，供团队成员和 CI 流程高效共享。

```bash
cargo install sccache
export RUSTC_WRAPPER=sccache
```

**`mold` — 极速链接器：**
链接阶段往往是最后一个且最慢的环节。`mold` 在 Linux (ELF) 平台上比 LLD 快 3-5 倍，比默认的 GNU LD 快 10-20 倍。

```toml
# .cargo/config.toml
[target.x86_64-unknown-linux-gnu]
rustflags = ["-C", "link-arg=-fuse-ld=mold"]
```

**`cargo-nextest` — 高效测试运行器：**
它是 `cargo test` 的更佳替代品，支持更灵活的并发调度、独立进程运行及重试机制。

```bash
cargo install cargo-nextest
cargo nextest run
```

### 开发者可见性工具

- **`cargo-expand`**：查看宏展开后的源码。对于调试 `#[derive]` 或 `macro_rules!` 非常有用。
- **`cargo-geiger`**：统计整个依赖树中 `unsafe` 代码的使用量。
- **`cargo-watch`**：监听源文件变化并自动重新运行指定命令。

### 工作区 Lint 配置 — `[workspace.lints]`

自 Rust 1.74 起，你可以统一在根目录 `Cargo.toml` 中配置 lint 规则（包括 Clippy），而无需在每个 crate 的头部写一大堆 `#![deny(...)]`：

```toml
[workspace.lints.clippy]
unwrap_used = "warn"
dbg_macro = "deny"

[workspace.lints.rust]
unsafe_code = "deny"
```

### 🏋️ 练习

#### 🟢 练习 1：配置 sccache + mold
安装并启用这两项工具，测量在清理 (cargo clean) 后的重构编译中到底省下了多少时间。

#### 🟡 练习 2：切换到 cargo-nextest
在大型工作区中运行 `cargo nextest run`。与 `cargo test` 相比，感受并行度带来的速度提升。

### 关键收获
- `sccache` 通过云端后端，在团队和 CI 之间共享昂贵的编译产物。
- `mold` 是目前最快的 ELF 链接器 —— 将秒级的链接缩短到毫秒级。
- `cargo-nextest` 并行运行测试用例，并支持 JUnit 导出和失败重试。
- 使用 `[workspace.lints]` 确保整个大型项目代码质量的一致性。

***
