[English Original](../en/ch12-tricks-from-the-trenches.md)

# 一线实践技巧 🟡

> **你将学到：**
> - 经过实战检验、且不便归入其他章节的工程模式
> - 常见陷阱及其修复方法 —— 从 CI 不稳定到二进制膨胀
> - 可立即应用于任何 Rust 项目的“快赢”技术
>
> **相关章节：** 本书的每一章 —— 这些技巧横跨了所有主题

本章收集了在生产级 Rust 代码库中反复出现的工程模式。每个技巧都是独立的 —— 你可以按任何顺序阅读。

---

### 1. `deny(warnings)` 陷阱

**问题**：在源码中使用 `#![deny(warnings)]` 会在 Clippy 添加新 Lint 后导致构建失败 —— 昨天还能编译的代码今天就由于新规则报错了。

**修复**：在 CI 中使用 `CARGO_ENCODED_RUSTFLAGS` 代替源码级的属性：

```yaml
# CI: 将警告视为错误，而不触及源码
env:
  CARGO_ENCODED_RUSTFLAGS: "-Dwarnings"
```

或者使用 `[workspace.lints]` 进行更精细的控制：

```toml
# Cargo.toml
[workspace.lints.rust]
unsafe_code = "deny"

[workspace.lints.clippy]
all = { level = "deny", priority = -1 }
pedantic = { level = "warn", priority = -1 }
```

> 参见 [编译期工具，工作区 Lint](ch08-compile-time-and-developer-tools.md) 了解完整模式。

---

### 2. 编译一次，到处测试

**问题**：在 `--lib`、`--doc` 和 `--test` 之间切换时，`cargo test` 会重新编译，因为它们使用的是不同的配置项 (Profiles)。

**修复**：使用 `cargo nextest` 运行单元测试和集成测试，并单独运行文档测试：

```bash
cargo nextest run --workspace        # 快：并行运行，有缓存
cargo test --workspace --doc         # 文档测试 (nextest 无法运行此类测试)
```

> 参见 [编译期工具](ch08-compile-time-and-developer-tools.md) 了解 `cargo-nextest` 的配置。

---

### 3. 特性标志 (Feature Flag) 健康管理

**问题**：一个库 crate 拥有 `default = ["std"]`，但没有人测试过 `--no-default-features` 情况。某天一个嵌入式用户反馈由于该原因无法编译。

**修复**：在 CI 中添加 `cargo-hack`：

```yaml
- name: 特性矩阵检查
  run: |
    cargo hack check --each-feature --no-dev-deps
    cargo check --no-default-features
    cargo check --all-features
```

> 参见 [`no_std` 与特性验证](ch09-no-std-and-feature-verification.md) 了解完整模式。

---

### 4. 琐事：Lock 文件该提交还是忽略？

**经验法则：**

| Crate 类型 | 是否提交 `Cargo.lock`？ | 原因 |
|------------|---------------------|-----|
| 二进制 / 应用程序 | **是** | 确保构建可复现 |
| 库 (Library) | **否** (加入 `.gitignore`) | 让下游用户自行选择版本 |
| 包含两者的工作区 | **是** | 以二进制应用为准 |

添加一个 CI 检查项，确保 Lock 文件保持最新：

```yaml
- name: 检查 Lock 文件
  run: cargo update --locked  # 如果 Cargo.lock 已过期则报错
```

---

### 5. 带有优化依赖项的调试构建

**问题**：Debug 构建慢得令人痛苦，因为依赖项（尤其是 `serde`, `regex`）没有被优化。

**修复**：在 dev 配置中优化依赖项，但保持你自己的代码不被优化以实现快速重新编译：

```toml
# Cargo.toml
[profile.dev.package."*"]
opt-level = 2  # 在开发模式下优化所有依赖项
```

这会稍微减慢第一次构建的速度，但在开发期间会显著提升运行效率。这对于依赖数据库的服务和解析器尤为重要。

> 参见 [发布配置](ch07-release-profiles-and-binary-size.md) 了解针对单个 crate 的配置覆盖 (Profile Overrides)。

---

### 6. CI 缓存抖动 (Thrashing)

**问题**：`Swatinem/rust-cache@v2` 在每个 PR 上都保存一个新缓存，导致存储膨胀并减慢还原速度。

**修复**：仅从 `main` 分支保存缓存，从任何地方还原缓存：

```yaml
- uses: Swatinem/rust-cache@v2
  with:
    save-if: ${{ github.ref == 'refs/heads/main' }}
```

对于包含多个二进制文件的工作区，可以添加一个 `shared-key`：

```yaml
- uses: Swatinem/rust-cache@v2
  with:
    shared-key: "ci-${{ matrix.target }}"
    save-if: ${{ github.ref == 'refs/heads/main' }}
```

> 参见 [CI/CD 流水线](ch11-putting-it-all-together-a-production-cic.md) 了解完整工作流。

---

### 7. `RUSTFLAGS` vs `CARGO_ENCODED_RUSTFLAGS`

**问题**：`RUSTFLAGS="-Dwarnings"` 会应用于 *一切* —— 包括构建脚本和过程宏。如果 `serde_derive` 的 build.rs 里有一个警告，你的 CI 就会挂掉。

**修复**：使用 `CARGO_ENCODED_RUSTFLAGS`，它只应用于顶层 crate：

```bash
# 不佳 —— 可能会因为第三方库构建脚本的警告而报错
RUSTFLAGS="-Dwarnings" cargo build

# 推荐 —— 仅影响你自己的 crate
CARGO_ENCODED_RUSTFLAGS="-Dwarnings" cargo build

# 同样推荐 —— 在 Cargo.toml 中设置工作区 Lint 
[workspace.lints.rust]
warnings = "deny"
```

---

### 8. 使用 `SOURCE_DATE_EPOCH` 实现可复现构建

**问题**：在 `build.rs` 中嵌入 `chrono::Utc::now()` 会导致构建无法复现 —— 每次构建都会产生不同的二进制哈希。

**修复**：遵循 `SOURCE_DATE_EPOCH`：

```rust
// build.rs
let timestamp = std::env::var("SOURCE_DATE_EPOCH")
    .ok()
    .and_then(|s| s.parse::<i64>().ok())
    .unwrap_or_else(|| chrono::Utc::now().timestamp());
println!("cargo:rustc-env=BUILD_TIMESTAMP={timestamp}");
```

> 参见 [构建脚本](ch01-build-scripts-buildrs-in-depth.md) 了解完整的 build.rs 模式。

---

### 9. `cargo tree` 去重工作流

**问题**：`cargo tree --duplicates` 显示有 5 个版本的 `syn` 和 3 个版本的 `tokio-util`。编译慢如蜗牛。

**修复**：系统性地去重：

```bash
# 第 1 步：查找重复项
cargo tree --duplicates

# 第 2 步：查找是谁拉取了旧版本
cargo tree --invert --package syn@1.0.109

# 第 3 步：更新导致问题的依赖
cargo update -p serde_derive  # 可能会因此拉取 syn 2.x

# 第 4 步：如果无法更新，在 [patch] 中手动锁定
# [patch.crates-io]
# old-crate = { git = "...", branch = "syn2-migration" }

# 第 5 步：验证
cargo tree --duplicates  # 列表应该变短了
```

> 参见 [依赖管理](ch06-dependency-management-and-supply-chain-s.md) 了解 `cargo-deny` 和供应链安全。

---

### 10. 推送前的冒烟测试 (Smoke Test)

**问题**：你推送了代码，CI 跑了 10 分钟，最后因为格式问题失败了。

**修复**：在推送前先在本地运行快速检查：

```toml
# Makefile.toml (cargo-make)
[tasks.pre-push]
description = "推送前的本地冒烟测试"
script = '''
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace --lib
'''
```

```bash
cargo make pre-push  # 耗时 < 30 秒
git push
```

或者使用 git pre-push 钩子：

```bash
#!/bin/sh
# .git/hooks/pre-push
cargo fmt --all -- --check && cargo clippy --workspace -- -D warnings
```

> 参见 [CI/CD 流水线](ch11-putting-it-all-together-a-production-cic.md) 了解 `Makefile.toml` 模式。

---

### 🏋️ 练习

#### 🟢 练习 1：应用三个技巧

从本章挑选三个技巧，并应用到一个现有的 Rust 项目中。哪一个对你的影响最大？

<details>
<summary>答案</summary>

典型的“高收益”组合：

1. **`[profile.dev.package."*"] opt-level = 2`** — 立即提升开发模式下的运行速度（对于解析密集型代码通常提速 2-10 倍）。

2. **`CARGO_ENCODED_RUSTFLAGS`** — 消除了由于第三方库警告导致的 CI 误报。

3. **`cargo-hack --each-feature`** — 通常能在包含 3 个以上特性的项目中找出至少一个失效的特性组合。

```bash
# 应用技巧 5:
echo '[profile.dev.package."*"]' >> Cargo.toml
echo 'opt-level = 2' >> Cargo.toml

# 在 CI 中应用技巧 7:
# 将 RUSTFLAGS 替换为 CARGO_ENCODED_RUSTFLAGS

# 应用技巧 3:
cargo install cargo-hack
cargo hack check --each-feature --no-dev-deps
```
</details>

#### 🟡 练习 2：去重你的依赖树

在一个真实项目上运行 `cargo tree --duplicates`。消除至少一处重复项。测量优化前后的编译耗时。

<details>
<summary>答案</summary>

```bash
# 优化前
time cargo build --release 2>&1 | tail -1
cargo tree --duplicates | wc -l  # 统计重复行数

# 查找并修复一处重复
cargo tree --duplicates
cargo tree --invert --package <duplicate-crate>@<old-version>
cargo update -p <parent-crate>

# 优化后
time cargo build --release 2>&1 | tail -1
cargo tree --duplicates | wc -l  # 数值应该变小了

# 典型结果：每消除一个重复项（尤其是像 syn, tokio 这种重型 crate），
# 编译时间可缩短 5-15%。
```
</details>

### 关键收获

- 使用 `CARGO_ENCODED_RUSTFLAGS` 代替 `RUSTFLAGS` 可避免由于第三方库构建脚本报错。
- `[profile.dev.package."*"] opt-level = 2` 是提升开发体验最立竿见影的技巧。
- 缓存微调（仅在 main 分支执行 `save-if`）可防止活跃代码库的 CI 存储膨胀。
- `cargo tree --duplicates` + `cargo update` 是不花钱的编译期提速法 —— 建议每月执行一次。
- 使用 `cargo make pre-push` 在本地执行快速检查，避免因琐事在 CI 往返上浪费时间。

---
