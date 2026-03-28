[English Original](../en/ch13-quick-reference-card.md)

# 快速参考卡片

### 速查表：常用命令一览

```bash
# ─── 构建脚本 (Build Scripts) ───
cargo build -vv                      # 显示 build.rs 的输出

# ─── 交叉编译 (Cross-Compilation) ───
cargo build --target x86_64-unknown-linux-musl  # 静态编译
cargo zigbuild --target x86_64-unknown-linux-gnu.2.17  # 兼容旧 glibc
cross build --target aarch64-unknown-linux-gnu  # 跨平台构建

# ─── 基准测试 (Benchmarking) ───
cargo bench                          # 运行基准测试
cargo flamegraph -- --args           # 生成火焰图进行剖析

# ─── 代码覆盖率 (Coverage) ───
cargo llvm-cov --html                # 生成 HTML 报告
cargo llvm-cov --fail-under-lines 80 # 如果覆盖率低于 80% 则报错

# ─── 安全验证 (Safety Verification) ───
cargo +nightly miri test             # 在 Miri 下运行测试
valgrind --leak-check=full ./bin     # 内存检查器

# ─── 审核与供应链 (Audit & Supply Chain) ───
cargo audit                          # 漏洞扫描
cargo deny check                     # 许可证、咨询建议、黑名单、来源
cargo geiger                         # 统计整个依赖树中的 unsafe 代码

# ─── 二进制优化 (Binary Optimization) ───
cargo bloat --release --crates       # crate 空间贡献分析
cargo +nightly udeps --workspace     # 查找未使用的依赖
cargo clippy --fix                   # 自动修复部分代码警告

# ─── 编译期优化 (Compile-Time Optimization) ───
export RUSTC_WRAPPER=sccache         # 编译缓存
cargo nextest run                    # 更快的并行测试运行器

# ─── 平台工程 (Platform Engineering) ───
cargo xwin build --target x86_64-pc-windows-msvc  # MSVC ABI
cargo hack check --each-feature      # 验证各种特性组合

# ─── 发布 (Release) ───
cargo release patch --execute        # 升级版本、打标签并推送
cargo dist plan                      # 预览发布分发产物
```

### 决策表：对应需求及对应工具

| 目标 | 对应工具 |
|------|------|
| 可追溯性 (Git Hash/时间) | `build.rs` (SOURCE_DATE_EPOCH) |
| 静态 Linux 二进制文件 | `musl` 编译目标 |
| 性能退化探测 (Regression) | Criterion.rs |
| 覆盖率门禁 (CI Gate) | `cargo-llvm-cov` |
| Unsafe 代码验证 | Miri / Valgrind |
| 供应链安全审核 | `cargo-audit` / `cargo-deny` |
| 二进制体积分析 | `cargo-bloat` / `LTO` |
| 极致链接速度 | `mold` |
| 监听保存自动构建 | `cargo-watch` |
| CI 构建速度 | `rust-cache` + `sccache` |

***
*版本 1.3 —— Rust 训练营：工程实践参考。*
***
