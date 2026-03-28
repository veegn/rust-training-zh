# 依赖管理与供应链安全 🟢

> **你将学到：**
> - 使用 `cargo-audit` 扫描已知漏洞
> - 使用 `cargo-deny` 强制执行许可证、咨询和来源策略
> - 使用 Mozilla 的 `cargo-vet` 进行供应链信任验证
> - 跟踪过时依赖并探测破坏性 API 变更
> - 可视化并去重依赖树
>
> **相关章节：** [发布配置](ch07-release-profiles-and-binary-size.md) — `cargo-udeps` 修剪此处发现的未使用依赖 · [CI/CD 流水线](ch11-putting-it-all-together-a-production-cic.md) — 流水线中的审核与拒绝任务 · [构建脚本](ch01-build-scripts-buildrs-in-depth.md) — `build-dependencies` 也是供应链的一部分

一个 Rust 二进制文件不仅仅包含你的代码，它还包含 `Cargo.lock` 中的每个传递依赖。该树中任何地方出现的漏洞、许可证冲突或恶意 crate 都会成为 *你的* 问题。本章介绍使依赖管理可审计且自动化的工具。

### cargo-audit — 已知漏洞扫描

[`cargo-audit`](https://github.com/rustsec/rustsec/tree/main/cargo-audit) 会根据 [RustSec 咨询数据库 (RustSec Advisory Database)](https://rustsec.org/) 检查你的 `Cargo.lock`，该数据库追踪已发布 crate 中已知的安全性缺陷。

```bash
# 安装并扫描
cargo install cargo-audit
cargo audit
```

### cargo-deny — 全面策略检查

[`cargo-deny`](https://github.com/EmbarkStudios/cargo-deny) 的功能远不止漏洞扫描，它强制执行四个维度的策略：
1. **Advisories** — 已知漏洞（同 `cargo-audit`）。
2. **Licenses** — 允许/禁止的许可证列表（MIT, Apache-2.0 等）。
3. **Bans** — 禁止的 crate 或重复的版本。
4. **Sources** — 允许的镜像站和 Git 来源。

```bash
# 初始化配置并运行检查
cargo deny init
cargo deny check
```

### cargo-vet — 供应链信任验证

[`cargo-vet`](https://github.com/mozilla/cargo-vet)（由 Mozilla 推出）解决的是不同的问题：不是“这个 crate 有已知 Bug 吗？”，而是“是否有一个受信任的人真正审查过这段代码？”。

### cargo-tree — 依赖可视化与去重

`cargo tree` 是 Cargo 内置的，对于理解依赖图非常有价值：

```bash
# 反向查找某个 crate 为何被包含进来
cargo tree --invert --package openssl-sys

# 查找重复的版本（重要！）
cargo tree --duplicates
```

### 🏋️ 练习

#### 🟢 练习 1：审核你的依赖
在任意 Rust 项目上运行 `cargo audit` 以及 `cargo deny init && cargo deny check`。

#### 🟡 练习 2：发现并消除重复依赖
运行 `cargo tree --duplicates`。找出一个以两个版本出现的 crate。你能更新 `Cargo.toml` 来统一它们吗？

### 关键收获
- `cargo audit` 会捕获已知的 CVE —— 在每次 push 时运行它。
- `cargo deny` 强制执行许可证、黑名单和来源策略。
- 对于多 crate 工作区，使用 `[workspace.dependencies]` 进行集中管理。
- `cargo tree --duplicates` 可以揭示由于版本冗余带来的编译耗时和二进制体积膨胀。

***
