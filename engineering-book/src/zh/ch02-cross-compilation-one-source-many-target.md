[English Original](../en/ch02-cross-compilation-one-source-many-target.md)

# 交叉编译：一份源码，多种目标 🟡

> **你将学到：**
> - Rust 目标三元组 (Target Triples) 的工作原理及如何通过 `rustup` 添加它们
> - 为容器/云部署构建静态 musl 二进制文件
> - 使用原生工具链、`cross` 以及 `cargo-zigbuild` 交叉编译到 ARM (aarch64)
> - 为多架构 CI 设置 GitHub Actions 矩阵构建
>
> **相关章节：** [构建脚本](ch01-build-scripts-buildrs-in-depth.md) — 交叉编译期间 build.rs 在 HOST 运行 · [发布配置](ch07-release-profiles-and-binary-size.md) — 交叉编译发布版二进制文件的 LTO 和 strip 设置 · [Windows](ch10-windows-and-conditional-compilation.md) — Windows 交叉编译与 `no_std` 目标

交叉编译意味着在一台机器（**宿主机 Host**）上构建可在另一台不同机器（**目标机 Target**）上运行的可执行文件。宿主机可能是你的 x86_64 笔记本，目标机可能是 ARM 服务器、基于 musl 的容器，甚至是 Windows 机器。
Rust 让这一切变得非常可行，因为 `rustc` 本身就是一个交叉编译器 —— 它只需要正确的目标库和兼容的链接器。

### 目标三元组 (Target Triple) 结构

每个 Rust 编译目标都由一个 **目标三元组** 标识（尽管名字叫三元组，但通常包含四个部分）：

```text
<架构>-<厂商>-<操作系统>-<环境>

示例：
  x86_64  - unknown - linux  - gnu      ← 标准 Linux (glibc)
  x86_64  - unknown - linux  - musl     ← 静态 Linux (musl libc)
  aarch64 - unknown - linux  - gnu      ← ARM 64 位 Linux
  x86_64  - pc      - windows- msvc     ← Windows (MSVC)
  aarch64 - apple   - darwin             ← 苹果芯片上的 macOS
```

列出所有可用目标：

```bash
# 显示 rustc 可以编译到的所有目标（约 250 个）
rustc --print target-list

# 显示系统中已安装的目标
rustup target list --installed
```

### 使用 rustup 安装工具链

```bash
# 添加目标库（该目标的 Rust 标准库）
rustup target add x86_64-unknown-linux-musl
rustup target add aarch64-unknown-linux-gnu

# 现在可以进行交叉编译了：
cargo build --target x86_64-unknown-linux-musl
```

**`rustup target add` 提供了什么**：预编译好的该目标的 `std`、`core` 和 `alloc` 库。它 **不** 提供 C 链接器或 C 库。对于大多数 `gnu` 目标，你需要单独安装 C 工具链。

### `.cargo/config.toml` — 针对目标的配置

无需在每个命令中传递 `--target`，可以在项目的 `.cargo/config.toml` 中配置默认值：

```toml
# 针对 aarch64 交叉编译的链接器
[target.aarch64-unknown-linux-gnu]
linker = "aarch64-linux-gnu-gcc"
```

### 使用 musl 构建静态二进制文件

为了部署到极致精简的容器（如 Alpine 或 scratch 镜像），推荐使用 musl 构建：

```bash
# 构建完全静态的二进制文件
cargo build --release --target x86_64-unknown-linux-musl

# 验证其是否为静态链接
file target/x86_64-unknown-linux-musl/release/diag_tool
# → ... statically linked
```

### 交叉编译到 ARM (aarch64)

ARM 服务器（如 AWS Graviton）越来越普遍。从 x86_64 宿主机交叉编译：

```bash
# 1. 安装目标和交叉链接器
rustup target add aarch64-unknown-linux-gnu
sudo apt install gcc-aarch64-linux-gnu

# 2. 构建
cargo build --release --target aarch64-unknown-linux-gnu
```

### `cross` 工具 — 基于 Docker 的交叉编译

[`cross`](https://github.com/cross-rs/cross) 提供了一种“零设置”的交叉编译体验，它使用预配置的 Docker 镜像：

```bash
# 交叉编译 —— 无需手动设置工具链！
cross build --release --target aarch64-unknown-linux-gnu
```

### 使用 Zig 作为交叉编译链接器

[Zig](https://ziglang.org/) 内置了适用于约 40 个目标的 C 编译器和系统根目录。这使其成为非常方便的 Rust 交叉链接器：

```bash
# 安装 cargo-zigbuild
cargo install cargo-zigbuild

# 为特定 glibc 版本构建（如 CentOS 7 兼容）
cargo zigbuild --release --target x86_64-unknown-linux-gnu.2.17
```

### CI 流水线：GitHub Actions 矩阵

生产级的 CI 工作流会针对多个目标进行构建：

```yaml
# 示例矩阵配置
strategy:
  matrix:
    include:
      - target: x86_64-unknown-linux-musl
        os: ubuntu-latest
      - target: aarch64-unknown-linux-gnu
        os: ubuntu-latest
```

### 关键收获
- Rust 的 `rustc` 本身就是交叉编译器。
- **musl** 产生完全静态、无运行时依赖的二进制文件 —— 容器部署的理想选择。
- **`cargo-zigbuild`** 解决了企业级 Linux 目标的 glibc 版本依赖问题。
- **`cross`** 是处理 ARM 等异构目标最简单的方法。

***
