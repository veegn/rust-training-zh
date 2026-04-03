[English Original](../en/ch13-quick-reference-card.md)

# 速查卡

### 备忘录：命令一览

```bash
# ─── 构建脚本 (Build Scripts) ───
cargo build                          # 先编译 build.rs，再编译 crate
cargo build -vv                      # 详细模式 —— 显示 build.rs 的输出

# ─── 交叉编译 (Cross-Compilation) ───
rustup target add x86_64-unknown-linux-musl
cargo build --release --target x86_64-unknown-linux-musl
cargo zigbuild --release --target x86_64-unknown-linux-gnu.2.17
cross build --release --target aarch64-unknown-linux-gnu

# ─── 基准测试 (Benchmarking) ───
cargo bench                          # 运行所有基准测试
cargo bench -- parse                 # 运行名称匹配 "parse" 的基准测试
cargo flamegraph -- --args           # 根据二进制程序生成火焰图
perf record -g ./target/release/bin  # 记录性能数据
perf report                          # 交互式查看性能数据

# ─── 代码覆盖率 (Coverage) ───
cargo llvm-cov --html                # 生成 HTML 报告
cargo llvm-cov --lcov --output-path lcov.info
cargo llvm-cov --workspace --fail-under-lines 80
cargo tarpaulin --out Html           # 备选工具

# ─── 安全验证 (Safety Verification) ───
cargo +nightly miri test             # 在 Miri 下运行测试
MIRIFLAGS="-Zmiri-disable-isolation" cargo +nightly miri test
valgrind --leak-check=full ./target/debug/binary
RUSTFLAGS="-Zsanitizer=address" cargo +nightly test -Zbuild-std --target x86_64-unknown-linux-gnu

# ─── 审计与供应链 (Audit & Supply Chain) ───
cargo audit                          # 已知漏洞扫描
cargo audit --deny warnings          # 若存在漏洞通告则使 CI 失败
cargo deny check                     # 许可证 + 漏洞通告 + 禁止项 + 源码检查
cargo deny list                      # 列出依赖树中的所有许可证
cargo vet                            # 供应链信任验证
cargo outdated --workspace           # 查找过期的依赖项
cargo semver-checks                  # 检测破坏性的 API 变更
cargo geiger                         # 统计依赖树中的 unsafe 代码行数

# ─── 二进制优化 (Binary Optimization) ───
cargo bloat --release --crates       # 统计每个 crate 的体积占用
cargo bloat --release -n 20          # 找出体积最大的 20 个函数
cargo +nightly udeps --workspace     # 查找未使用的依赖项
cargo machete                        # 快速检测未使用的依赖项
cargo expand --lib module::name      # 查看宏展开后的结果
cargo msrv find                      # 探测支持的最低 Rust 版本
cargo clippy --fix --workspace --allow-dirty  # 自动修复静态检查出的警告

# ─── 编译期优化 (Compile-Time Optimization) ───
export RUSTC_WRAPPER=sccache         # 开启共享编译缓存
sccache --show-stats                 # 查看缓存命中率统计
cargo nextest run                    # 更快的测试运行器
cargo nextest run --retries 2        # 自动重试不稳定的测试

# ─── 平台工程 (Platform Engineering) ───
cargo check --target thumbv7em-none-eabihf   # 验证 no_std 构建情况
cargo build --target x86_64-pc-windows-gnu   # 交叉编译至 Windows (MinGW)
cargo xwin build --target x86_64-pc-windows-msvc  # 交叉编译至 Windows (MSVC ABI)
cfg!(target_os = "linux")                    # 编译期配置 (求值为 bool 值)

# ─── 发布 (Release) ───
cargo release patch --dry-run        # 预览发布操作
cargo release patch --execute        # 提升版本、提交、打标签并发布
cargo dist plan                      # 预览分发产物
```

### 决策表：在什么场景使用什么工具

| 目标 | 工具 | 何时使用 |
|------|------|-------------|
| 嵌入 Git 哈希 / 构建信息 | `build.rs` | 二进制程序需要可追溯性 |
| 使用 Rust 编译 C 代码 | `build.rs` 中的 `cc` crate | 需要 FFI 调用小型 C 库 |
| 从 Schema 生成代码 | `prost-build` / `tonic-build` | 使用 Protobuf, gRPC, FlatBuffers |
| 链接系统库 | `build.rs` 中的 `pkg-config` | 依赖 OpenSSL, libpci, systemd |
| 静态链接的 Linux 二进制程序 | `--target x86_64-unknown-linux-musl` | 容器化或云端部署 |
| 针对旧版 glibc 进行构建 | `cargo-zigbuild` | 兼容 RHEL 7, CentOS 7 等老系统 |
| ARM 架构服务器程序 | `cross` 或 `cargo-zigbuild` | 部署至 Graviton/Ampere 算力 |
| 统计学意义上的基准测试 | Criterion.rs | 检测性能退化 (Regression) |
| 快速性能检查 | Divan | 开发期间的性能分析 |
| 找出性能热点 | `cargo flamegraph` / `perf` | 基准测试发现慢代码后的深度剖析 |
| 行/分支覆盖率 | `cargo-llvm-cov` | CI 覆盖率门禁、覆盖缺口分析 |
| 快速检查覆盖率 | `cargo-tarpaulin` | 本地开发期间使用 |
| Rust UB (未定义行为) 检测 | Miri | 纯 Rust 的 `unsafe` 代码 |
| C FFI 内存安全检查 | Valgrind memcheck | Rust/C 混合的代码库 |
| 数据竞态检测 | TSan 或 Miri | 并发环境下的 `unsafe` 代码 |
| 缓冲区溢出检测 | ASan | `unsafe` 指针算术运算 |
| 泄漏检测 | Valgrind 或 LSan | 需要长时间运行的服务 |
| 本地模拟 CI | `cargo-make` | 开发者工作流自动化 |
| 提交前检查 (Pre-commit) | `cargo-husky` 或 Git 钩子 | 推送代码前拦截问题 |
| 自动化发布 | `cargo-release` + `cargo-dist` | 版本管理与分发自动化 |
| 依赖项审计 | `cargo-audit` / `cargo-deny` | 供应链安全保障 |
| 许可证合规性 | `cargo-deny` (licenses) | 商业或企业级项目 |
| 供应链信任验证 | `cargo-vet` | 高安全性要求的环境 |
| 查找过期依赖 | `cargo-outdated` | 定期的维护工作 |
| 检测破坏性 API 变更 | `cargo-semver-checks` | 二进制库发布前校验 |
| 依赖树分析 | `cargo tree --duplicates` | 优化并清理依赖图中的冗余 |
| 二进制体积分析 | `cargo-bloat` | 针对体积敏感的部署场景 |
| 查找冗余依赖 | `cargo-udeps` / `cargo-machete` | 缩减编译时间和二进制体积 |
| LTO 微调 | `lto = true` 或 `"thin"` | 优化发布版二进制程序 |
| 体积优化的二进制程序 | `opt-level = "z"` + `strip = true` | 嵌入式 / WASM / 容器环境 |
| Unsafe 使用情况审计 | `cargo-geiger` | 安全政策强制执行 |
| 宏调试 | `cargo-expand` | 调试 derive 或 macro_rules 输出 |
| 链接加速 | `mold` 链接器 | 提升开发人员的本地迭代速度 |
| 编译缓存 | `sccache` | CI 与本地构建加速 |
| 测试加速 | `cargo-nextest` | CI 与本地测试提速 |
| MSRV 兼容性检查 | `cargo-msrv` | 发布库文件时使用 |
| `no_std` 库开发 | `#![no_std]` + `default-features = false` | 嵌入式、UEFI、WASM 环境 |
| Windows 交叉编译 | `cargo-xwin` / MinGW | 在 Linux 上构建 Windows 程序 |
| 平台抽象模式 | `#[cfg]` + Trait 模式 | 支持多操作系统的代码库 |
| 调用 Windows API | `windows-sys` / `windows` crate | 使用 Windows 原生功能 |
| 端到端耗时测量 | `hyperfine` | 整体二进制基准测试及前后对比 |
| 基于属性的测试 | `proptest` | 发现边缘情况、增强解析器健壮性 |
| 快照测试 | `insta` | 验证大型结构化输出 |
| 覆盖率导向型模糊测试 | `cargo-fuzz` | 发现解析器中的崩溃 Bug |
| 并发模型检查 | `loom` | 验证无锁数据结构、原子序 |
| 特性组合测试 | `cargo-hack` | 针对拥有多个 `#[cfg]` 特性的 crate |
| 快速 UB 检查 (近乎原生) | `cargo-careful` | CI 安全门禁，比 Miri 更轻量 |
| 保存自动重构 | `cargo-watch` | 提升开发迭代反馈速度 |
| 工作区文档 | `cargo doc` + rustdoc | API 发现、上手文档、文档链接 CI 检查 |
| 可复现构建 | `--locked` + `SOURCE_DATE_EPOCH` | 验证发布版本的完整性 |
| CI 缓存微调 | `Swatinem/rust-cache@v2` | 缩短构建耗时 (冷构建 → 缓存构建) |
| 工作区 Lint 政策 | Cargo.toml 中的 `[workspace.lints]` | 跨 crate 统一 Clippy 和编译器 Lint 规则 |
| 自动修复 Lint 警告 | `cargo clippy --fix` | 自动化清理琐碎代码问题 |

### 延伸阅读

| 主题 | 资料 |
|------|----------|
| Cargo 构建脚本 | [Cargo Book — Build Scripts](https://doc.rust-lang.org/cargo/reference/build-scripts.html) |
| 交叉编译 | [Rust Cross-Compilation](https://rust-lang.github.io/rustup/cross-compilation.html) |
| `cross` 工具 | [cross-rs/cross](https://github.com/cross-rs/cross) |
| `cargo-zigbuild` | [cargo-zigbuild docs](https://github.com/rust-cross/cargo-zigbuild) |
| Criterion.rs | [Criterion 用户手册](https://bheisler.github.io/criterion.rs/book/) |
| Divan | [Divan docs](https://github.com/nvzqz/divan) |
| `cargo-llvm-cov` | [cargo-llvm-cov](https://github.com/taiki-e/cargo-llvm-cov) |
| `cargo-tarpaulin` | [tarpaulin docs](https://github.com/xd009642/tarpaulin) |
| Miri | [Miri GitHub](https://github.com/rust-lang/miri) |
| Rust 中的 Sanitizer | [rustc Sanitizer 文档](https://doc.rust-lang.org/nightly/unstable-book/compiler-flags/sanitizer.html) |
| `cargo-make` | [cargo-make book](https://sagiegurari.github.io/cargo-make/) |
| `cargo-release` | [cargo-release docs](https://github.com/crate-ci/cargo-release) |
| `cargo-dist` | [cargo-dist docs](https://axodotdev.github.io/cargo-dist/book/) |
| Profile-guided optimization | [Rust PGO 指南](https://doc.rust-lang.org/rustc/profile-guided-optimization.html) |
| 火焰图 | [cargo-flamegraph](https://github.com/flamegraph-rs/flamegraph) |
| `cargo-deny` | [cargo-deny docs](https://embarkstudios.github.io/cargo-deny/) |
| `cargo-vet` | [cargo-vet docs](https://mozilla.github.io/cargo-vet/) |
| `cargo-audit` | [cargo-audit](https://github.com/rustsec/rustsec/tree/main/cargo-audit) |
| `cargo-bloat` | [cargo-bloat](https://github.com/RazrFalcon/cargo-bloat) |
| `cargo-udeps` | [cargo-udeps](https://github.com/est31/cargo-udeps) |
| `cargo-geiger` | [cargo-geiger](https://github.com/geiger-rs/cargo-geiger) |
| `cargo-semver-checks` | [cargo-semver-checks](https://github.com/obi1kenobi/cargo-semver-checks) |
| `cargo-nextest` | [nextest docs](https://nexte.st/) |
| `sccache` | [sccache](https://github.com/mozilla/sccache) |
| `mold` 链接器 | [mold](https://github.com/rui314/mold) |
| `cargo-msrv` | [cargo-msrv](https://github.com/foresterre/cargo-msrv) |
| LTO | [rustc 代码生成选项](https://doc.rust-lang.org/rustc/codegen-options/index.html) |
| Cargo Profiles | [Cargo Book — Profiles](https://doc.rust-lang.org/cargo/reference/profiles.html) |
| `no_std` | [Rust 嵌入式手册](https://docs.rust-embedded.org/book/) |
| `windows-sys` crate | [windows-rs](https://github.com/microsoft/windows-rs) |
| `cargo-xwin` | [cargo-xwin docs](https://github.com/rust-cross/cargo-xwin) |
| `cargo-hack` | [cargo-hack](https://github.com/taiki-e/cargo-hack) |
| `cargo-careful` | [cargo-careful](https://github.com/RalfJung/cargo-careful) |
| `cargo-watch` | [cargo-watch](https://github.com/watchexec/cargo-watch) |
| Rust CI 缓存 | [Swatinem/rust-cache](https://github.com/Swatinem/rust-cache) |
| Rustdoc 手册 | [Rustdoc Book](https://doc.rust-lang.org/rustdoc/) |
| 条件编译 | [Rust 参考手册 — cfg](https://doc.rust-lang.org/reference/conditional-compilation.html) |
| 嵌入式 Rust | [Awesome Embedded Rust](https://github.com/rust-embedded/awesome-embedded-rust) |
| `hyperfine` | [hyperfine](https://github.com/sharkdp/hyperfine) |
| `proptest` | [proptest](https://github.com/proptest-rs/proptest) |
| `insta` | [insta 快照测试](https://insta.rs/) |
| `cargo-fuzz` | [cargo-fuzz](https://github.com/rust-fuzz/cargo-fuzz) |
| `loom` | [loom 并发测试](https://github.com/tokio-rs/loom) |

---

*生成参考资料 —— 作为《Rust 设计模式》与《类型驱动的正确性》的配套指南。*

*版本 1.3 —— 为了保证内容的完备性，增加了 cargo-hack, cargo-careful, cargo-watch, cargo doc, 可复现构建, CI 缓存策略, 总结性练习以及章节依赖图。*

---
