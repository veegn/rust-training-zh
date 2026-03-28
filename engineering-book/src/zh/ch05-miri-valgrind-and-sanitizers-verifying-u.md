[English Original](../en/ch05-miri-valgrind-and-sanitizers-verifying-u.md)

# Miri、Valgrind 与 Sanitizer：验证 Unsafe 代码 🔴

> **你将学到：**
> - Miri 作为 MIR 解释器：它能捕获什么（别名规则、UB、泄漏）以及不能做什么（FFI、系统调用）
> - Valgrind memcheck、Helgrind（数据竞争）、Callgrind（剖析）和 Massif（堆内存）
> - LLVM sanitizer：ASan、MSan、TSan、LSan 及 nightly 下的 `-Zbuild-std`
> - `cargo-fuzz` 用于发现崩溃风险，以及 `loom` 用于并发模型检查
> - 选择验证工具的决策树
>
> **相关章节：** [代码覆盖率](ch04-code-coverage-seeing-what-tests-miss.md) — 覆盖率发现未测试路径，Miri 验证已测试路径 · [`no_std` 与特性验证](ch09-no-std-and-feature-verification.md) — `no_std` 代码常需 `unsafe`，Miri 可验证它 · [CI/CD 流水线](ch11-putting-it-all-together-a-production-cic.md) — 流水线中的 Miri 任务

安全 Rust (Safe Rust) 在编译阶段保证了内存安全和无数据竞争。但当你编写 `unsafe` 代码时（用于 FFI、手写数据结构或性能优化），这些保证就成了 *你* 的责任。本章涵盖了验证你的 `unsafe` 代码是否真正履行了其安全契约的工具。

### Miri — Unsafe Rust 解释器

[Miri](https://github.com/rust-lang/miri) 是 Rust 中级中间表示 (MIR) 的 **解释器**。它逐步执行你的程序，并在每次操作时对未定义行为 (Undefined Behavior, UB) 进行详尽检查。

```bash
# 安装并运行
rustup +nightly component add miri
cargo +nightly miri test
```

**Miri 能捕获：**
- 越界访问 (Out-of-bounds)
- 释放后使用 (Use-after-free)
- 无效值（例如 `bool` 不是 0 或 1）
- 数据竞争 (Data Race)
- **Stacked Borrows/Tree Borrows 违规**（Rust 的别名/借用规则）

### Valgrind 与 Rust 集成

[Valgrind](https://valgrind.org/) 直接作用于编译后的二进制文件，在机器码层面检查内存错误。它对于包含大量 FFI 的代码尤为实用。

```bash
valgrind --tool=memcheck --leak-check=full ./target/debug/my_app
```

### 地址检查器 (ASan) 与 线程检查器 (TSan)

LLVM 的 Sanitizer 比 Valgrind 更快（2-5 倍开销 vs 10-50 倍），但需要使用 nightly 版本及 `-Zbuild-std` 重新编译标准库。

```bash
# ASan 示例
RUSTFLAGS="-Zsanitizer=address" cargo +nightly test -Zbuild-std --target x86_64-unknown-linux-gnu
```

### 相关工具：模糊测试与并发验证

- **`cargo-fuzz`**：针对解析器和解码器的覆盖率驱动模糊测试。
- **`loom`**：针对无锁 (Lock-free) 数据结构的并发模型检查器。

### 工具选择建议

| 工具 | 适用场景 | 系统要求 |
|------|----------|-------------|
| **Miri** | 纯 Rust `unsafe` 代码 | Nightly |
| **Valgrind** | FFI / C/C++ 互操作 | Linux/macOS |
| **ASan** | 快速发现内存崩溃 | Nightly |
| **TSan** | 探测数据竞争 | Nightly |
| **fuzz** | 复杂的解析逻辑 | Nightly |
| **loom** | 无锁逻辑验证 | Stable |

### 🏋️ 练习

#### 🟡 练习 1：触发 Miri 的 UB 检测
编写一个 `unsafe` 函数，对同一个 `i32` 创建两个 `&mut` 引用（别名规则违规）。运行 `cargo +nightly miri test`观察报错。

#### 🔴 练习 2：ASan 越界访问检测
编写一个测试，进行 `unsafe` 的数组越界访问。使用 `RUSTFLAGS="-Zsanitizer=address"` 并在 nightly 下构建运行。

### 关键收获
- **Miri** 是验证纯 Rust `unsafe` 代码的首选 —— 它能捕获别名规则违规。
- **Valgrind** 是验证 FFI/C 互操作的首选。
- **Sanitizer** (ASan, TSan) 比 Valgrind 快，但需要特定系统和 nightly。
- **`loom`** 专门用于验证复杂的无锁并发逻辑。

***
