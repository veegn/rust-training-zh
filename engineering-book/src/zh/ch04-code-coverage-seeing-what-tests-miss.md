# 代码覆盖率：发现测试遗漏 🟢

> **你将学到：**
> - 使用 `cargo-llvm-cov` 进行基于源码的覆盖率分析（最准确的 Rust 覆盖率工具）
> - 使用 `cargo-tarpaulin` 或 `grcov` 进行快速覆盖率检查
> - 在 CI 中设置覆盖率门禁（使用 Codecov 或 Coveralls）
> - 覆盖率驱动的测试策略：优先处理高风险的测试盲点
>
> **相关章节：** [Miri 与 Sanitizer](ch05-miri-valgrind-and-sanitizers-verifying-u.md) — 覆盖率发现未测试代码，Miri 发现已测试代码中的 UB · [基准测试](ch03-benchmarking-metasuring-what-matters.md) — 覆盖率告诉你什么被测试了，基准测试告诉你它有多快

代码覆盖率衡量你的测试实际上执行了哪些行、分支或函数。它不能证明代码的正确性（被覆盖的行仍可能有 Bug），但能可靠地揭示 **盲点** —— 即没有任何测试覆盖到的代码路径。

### 使用 `llvm-cov` 进行基于源码的覆盖率分析

Rust 使用 LLVM，它提供了基于源码的覆盖率插桩（这是目前最准确的覆盖率统计方法）。推荐工具是 [`cargo-llvm-cov`](https://github.com/taiki-e/cargo-llvm-cov)：

```bash
# 安装
cargo install cargo-llvm-cov
rustup component add llvm-tools-preview

# 运行测试并查看摘要
cargo llvm-cov

# 生成 HTML 报告
cargo llvm-cov --html
```

**覆盖率类型：**

| 类型 | 衡量内容 |
|------|------------------|
| **行覆盖率 (Line)** | 哪些源代码行被执行了 |
| **分支覆盖率 (Branch)** | `if`/`match` 的哪些分支被执行了 |
| **函数覆盖率 (Function)** | 哪些函数被调用了 |

### cargo-tarpaulin — 快速路径

[`cargo-tarpaulin`](https://github.com/xd009642/tarpaulin) 是一个专门针对 Linux 的覆盖率工具，设置更简单：

```bash
cargo tarpaulin --out Html
```

### CI 中的覆盖率：Codecov 与 Coveralls

将覆盖率数据上传到追踪服务，以获取历史趋势和 PR 注释：

```yaml
# GitHub Action 示例步骤
- name: 生成覆盖率
  run: cargo llvm-cov --workspace --lcov --output-path lcov.info

- name: 上传到 Codecov
  uses: codecov/codecov-action@v4
  with:
    files: lcov.info
```

### 覆盖率驱动的测试策略

1. **高覆盖率，高风险**：很好 —— 请继续保持。
2. **低覆盖率，高风险**：**红色警报** —— 立即编写测试。
3. **排除干扰**：不要盲目追求 100% 覆盖率。排除生成的代码或测试文件等干扰项。

### 🏋️ 练习

#### 🟢 练习 1：第一个覆盖率报告
安装 `cargo-llvm-cov`，对任意项目运行并打开 HTML 报告。找出覆盖率最低的三个文件。

#### 🟡 练习 2：CI 覆盖率门禁
在 GitHub Actions 中添加门禁，如果行覆盖率低于 60% 则报错。

### 关键收获
- `cargo-llvm-cov` 是 Rust 最准确的覆盖率工具。
- 覆盖率无法证明正确性，但 **零覆盖率意味着零测试**。
- 在 CI 中设置覆盖率门禁以防止质量倒退。
- 重点关注高风险路径（错误处理、unsafe、解析等）。

***
