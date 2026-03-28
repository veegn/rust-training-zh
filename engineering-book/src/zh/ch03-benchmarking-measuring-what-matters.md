# 基准测试：衡量真正重要的指标 🟡

> **你将学到：**
> - 为什么使用 `Instant::now()` 进行简单计时会导致不可靠的结果
> - 使用 Criterion.rs 进行统计学基准测试，以及更轻量级的 Divan 替代方案
> - 使用 `perf`、火焰图和 PGO 剖析性能热点
> - 在 CI 中设置持续基准测试以自动捕获性能退化
>
> **相关章节：** [发布配置](ch07-release-profiles-and-binary-size.md) — 找到热点后优化二进制文件 · [CI/CD 流水线](ch11-putting-it-all-together-a-production-cic.md) — 流水线中的基准测试任务 · [代码覆盖率](ch04-code-coverage-seeing-what-tests-miss.md) — 覆盖率告诉你测试了什么，基准测试告诉你它有多快

“我们应该忘记细小的效率，大约 97% 的情况下：过早优化是万恶之源。但我们不应错过那关键 3% 的机会。” —— 唐纳德·克努斯 (Donald Knuth)

难点不在于 *编写* 基准测试，而在于编写能产生 **有意义、可复现、可操作** 数字的基准测试。本章涵盖了能让你从“似乎很快”提升到“我们有统计证据表明该 PR 导致解析吞吐量退化了 4.2%”的工具和技术。

### 为什么不用 `std::time::Instant`？

常见的诱惑：

```rust
// ❌ 简陋的基准测试 —— 结果不可靠
use std::time::Instant;

fn main() {
    let start = Instant::now();
    let result = parse_data(&sample_data);
    let elapsed = start.elapsed();
    println!("解析耗时 {:?}", elapsed);
    // 问题 1：编译器可能会优化掉 `result`（死代码消除）
    // 问题 2：单次采样 —— 无统计学意义
    // 问题 3：CPU 变频、散热降频、其他进程干扰
}
```

手动计时的缺陷：
1. **死代码消除** —— 如果结果未被使用，编译器可能会完全跳过计算。
2. **缺少预热 (Warm-up)** —— 第一次运行包含缓存未命中、操作系统缺页中断等因素。
3. **缺少统计学分析** —— 单次测量无法告诉你方差、离群值或置信区间。

### Criterion.rs — 统计学基准测试

[Criterion.rs](https://bheisler.github.io/criterion.rs/book/) 是 Rust 微基准测试的事实标准。它使用统计方法产生可靠的测量结果，并能自动检测性能退化。

**运行与阅读结果：**
Criterion 会在提示符中给出置信区间（Confidence Interval）。

### Divan — 更轻量的替代方案

[Divan](https://github.com/nvzqz/divan) 是一个较新的基准测试框架，它使用属性宏而不是 Criterion 的宏 DSL。

### 使用 `perf` 和火焰图剖析性能

基准测试告诉你“有多快”，剖析则告诉你“时间流向了哪里”。

```bash
# 生成火焰图
cargo flamegraph --root -- --run-diagnostics
```

**阅读火焰图：**
- **宽度** = 在该函数中花费的时间（越宽表示越慢）。
- **顶部** = 正在执行实际工作的叶子函数 —— 寻找顶部宽阔的平台。

### CI 中的持续基准测试

通过在 CI 流水线中运行基准测试并将其与历史数据进行对比，在性能退化发布前就捕获到它。

### 🏋️ 练习

#### 🟢 练习 1：第一个 Criterion 基准测试
创建一个对 10,000 个随机元素进行排序的函数，编写 Criterion 基准测试，对比 `sort()` 与 `sort_unstable()` 的性能。

#### 🟡 练习 2：火焰图热点
构建一个包含调试信息的发布版项目，生成火焰图。找出最宽的三个堆栈顶部。

### 关键收获
- 永远不要用 `Instant::now()` 做基准测试 —— 使用 Criterion.rs 以获得统计严谨性。
- `black_box()` 防止编译器优化掉你的基准测试目标。
- `hyperfine` 衡量整个二进制文件的墙钟时间；Criterion 衡量单个函数。
- 火焰图显示时间 *在哪里* 花掉的；基准测试显示花掉了 *多少* 时间。

***
