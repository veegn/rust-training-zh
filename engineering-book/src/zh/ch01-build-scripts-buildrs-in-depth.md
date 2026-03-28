# 构建脚本：深入理解 `build.rs` 🟢

> **你将学到：**
> - `build.rs` 如何嵌入 Cargo 构建流水线及其运行机制
> - 五种生产实践模式：编译期常量、C/C++ 编译、Protobuf 代码生成、`pkg-config` 链接和特性检测
> - 会拖慢构建或破坏交叉编译的反模式
> - 如何权衡可追溯性与可复现构建 (Reproducible Builds)
>
> **相关章节：** [交叉编译](ch02-cross-compilation-one-source-many-target.md) 使用构建脚本实现目标平台感知构建 · [`no_std` 与特性验证](ch09-no-std-and-feature-verification.md) 扩展了此处设置的 `cfg` 标志 · [CI/CD 流水线](ch11-putting-it-all-together-a-production-cic.md) 在自动化中编排构建脚本

每个 Cargo 包都可以在 crate 根目录下包含一个名为 `build.rs` 的文件。
Cargo 会在编译你的 crate *之前* 编译并执行该文件。构建脚本通过 stdout 上的 `println!` 指令与 Cargo 进行通信。

### 什么是 build.rs 以及它何时运行

```text
┌─────────────────────────────────────────────────────────┐
│                    Cargo 构建流水线                      │
│                                                         │
│  1. 解析依赖                                            │
│  2. 下载 crate                                          │
│  3. 编译 build.rs  ← 普通 Rust 代码，在 HOST（宿机）运行  │
│  4. 执行 build.rs  ← stdout → Cargo 指令                │
│  5. 编译 crate（使用步骤 4 中的指令）                      │
│  6. 链接                                                │
└─────────────────────────────────────────────────────────┘
```

关键事实：
- `build.rs` 在 **宿主 (Host)** 机器上运行，而不是在目标 (Target) 机器上。在交叉编译期间，构建脚本在你的开发机上运行，即使最终二进制文件针对的是不同的架构。
- 构建脚本的作用范围仅限于其所属的包。它无法影响其他 crate 的编译方式 —— 除非该包在 `Cargo.toml` 中声明了 `links` 键，这允许通过 `cargo::metadata=KEY=VALUE` 向下游 crate 传递元数据。
- 只要 Cargo 检测到变更，它就会 **每次** 运行 —— 除非你发出 `cargo::rerun-if-changed` 指令来限制重新运行。

> **注意 (Rust 1.71+)**：自 Rust 1.71 起，Cargo 会对编译后的 `build.rs` 二进制文件进行指纹识别 —— 如果二进制文件完全相同，即使源代码时间戳改变了，它也不会重新运行。然而，`cargo::rerun-if-changed=build.rs` 仍然很有价值：如果没有 *任何* `rerun-if-changed` 指令，Cargo 会在 **包内的任何文件** 发生变化时重新运行 `build.rs`（而不仅仅是 `build.rs` 发生变化）。发出 `cargo::rerun-if-changed=build.rs` 可以将重新运行限制在仅当 `build.rs` 本身发生变化时 —— 这在大型 crate 中能显著节省编译时间。

### Cargo 指令协议

构建脚本通过在标准输出打印指令来与 Cargo 通信。自 Rust 1.77 起，首选前缀是 `cargo::`（取代了旧的单冒号 `cargo:` 形式）。

| 指令 | 用途 |
|-------------|---------|
| `cargo::rerun-if-changed=PATH` | 仅当 PATH 变更时重新运行 build.rs |
| `cargo::rerun-if-env-changed=VAR` | 仅当环境变量 VAR 变更时重新运行 |
| `cargo::rustc-link-lib=NAME` | 链接原生库 NAME |
| `cargo::rustc-link-search=PATH` | 向库搜索路径添加 PATH |
| `cargo::rustc-cfg=KEY` | 为条件编译设置 `#[cfg(KEY)]` 标志 |
| `cargo::rustc-env=KEY=VALUE` | 设置可通过 `env!()` 访问的环境变量 |
| `cargo::warning=MESSAGE` | 在编译期间显示警告 |

```rust
// build.rs — 极简示例
fn main() {
    // 仅在 build.rs 本身变化时重新运行
    println!("cargo::rerun-if-changed=build.rs");

    // 设置编译期环境变量
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs().to_string())
        .unwrap_or_else(|_| "0".into());
    println!("cargo::rustc-env=BUILD_TIMESTAMP={timestamp}");
}
```

### 模式 1：编译期常量

最常见的用例：将构建元数据（如 Git 哈希、构建日期、CI 任务 ID）写入二进制文件，以便在运行时报告。

```rust
// build.rs
use std::process::Command;

fn main() {
    println!("cargo::rerun-if-changed=.git/HEAD");
    println!("cargo::rerun-if-changed=.git/refs");

    // Git commit hash
    let output = Command::new("git")
        .args(["rev-parse", "--short", "HEAD"])
        .output()
        .expect("git not found");
    let git_hash = String::from_utf8_lossy(&output.stdout).trim().to_string();
    println!("cargo::rustc-env=GIT_HASH={git_hash}");
}
```

### 模式 2：使用 `cc` crate 编译 C/C++ 代码

当你的 Rust crate 封装了 C 库或需要小型 C 辅助程序时，[`cc`](https://docs.rs/cc) crate 简化了 inside build.rs 的编译工作。

```rust
// build.rs
fn main() {
    println!("cargo::rerun-if-changed=csrc/");

    cc::Build::new()
        .file("csrc/ipmi_raw.c")
        .include("csrc/include")
        .compile("diag_helpers");
}
```

### 模式 3：Protocol Buffers 与代码生成

构建脚本非常擅长代码生成 —— 在编译时将 `.proto` 等模式文件转换为 Rust 源码。

### 模式 4：使用 `pkg-config` 链接系统库

对于提供 `.pc` 文件的系统库，[`pkg-config`](https://docs.rs/pkg-config) crate 会探测系统并发出正确的链接指令。

### 模式 5：特性检测与条件编译

构建脚本可以探测编译环境并设置 `cfg` 标志，供 main crate 用于条件代码路径。

> ⚠️ **反模式演示** —— 下面的代码显示了一种诱人但有问题的做法。**请勿在生产环境中使用。**

```rust
// build.rs — 坏习惯：在构建时进行运行时硬件探测
fn main() {
    // 反模式：二进制文件与构建机器的硬件绑定了。
    // 如果你在带 GPU 的机器上构建并部署到不带 GPU 的机器，
    // 二进制文件会默认为存在 GPU。
}
```

> ⚠️ **为什么这是错的**：对于可选硬件，运行时设备检测几乎总是优于构建时检测。上面产生的二进制文件会 *与构建机器的硬件配置绑定*。仅对那些在编译时确实固定下来的能力（架构、操作系统、库的可用性）使用构建时检测。

### 反模式与坑点

| 反模式 | 危害 | 修正 |
|-------------|-------------|-----|
| 缺少 `rerun-if-changed` | build.rs 在 *每次* 构建时都会运行，拖慢迭代速度 | 始终至少发出 `cargo::rerun-if-changed=build.rs` |
| 在 build.rs 中发起网络请求 | 离线构建失败，不可复现 | 使用 Vendor 或单独的 fetch 步骤 |
| 写入 `src/` 目录 | Cargo 不期望源码在构建期间改变 | 写入 `OUT_DIR` 并使用 `include!()` |
| 忽略交叉编译 | 直接使用 `Command::new("gcc")` 而不尊重 `$CC` | 使用能正确处理交叉编译工具链的 `cc` crate |

### 可复现构建 (Reproducible Builds)

在本章中学到的嵌入时间戳和 Git 哈希会 **破坏可复现构建** —— 即相同的源码始终产生相同的二进制文件。

**实际解决方案：**
在构建脚本中尊重 `SOURCE_DATE_EPOCH`：
```rust
let timestamp = std::env::var("SOURCE_DATE_EPOCH")
    .unwrap_or_else(|_| { /* 获取当前时间 */ });
```

### 🏋️ 练习

#### 🟢 练习 1：版本标记
创建一个包含 `build.rs` 的最小 crate，将 Git 哈希嵌入环境变量，并在 `main()` 中打印。

#### 🟡 练习 2：条件系统库
使用 `pkg-config` 探测 `libz`，如果找到则发出 `cfg` 标志。

### 关键收获
- `build.rs` 在 **宿主机** 编译时运行 —— 始终发出 `cargo::rerun-if-changed`。
- 使用 `cc` crate 而不是原生指令，处理交叉编译。
- 将生成的代码写入 `OUT_DIR`，永远不要写入 `src/`。
- 对于可选硬件，运行时探测优于构建时探测。
- 嵌入时间戳时使用 `SOURCE_DATE_EPOCH` 以保证可复现性。

***
