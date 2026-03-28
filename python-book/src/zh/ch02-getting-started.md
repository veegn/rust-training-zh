## 快速开始

> **你将学到：** 如何安装 Rust 及其工具链、Cargo 与 pip/Poetry 的区别、IDE 配置、你的第一个 `Hello, world!` 程序，以及若干对 Python 开发者特别重要的 Rust 关键字。
>
> **难度：** 🟢 初级

### 安装 Rust
```bash
# 通过 rustup 安装 (Linux/macOS/WSL)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 验证安装
rustc --version     # Rust 编译器
cargo --version     # 构建工具 + 包管理器 (相当于 pip + setuptools 的合体)

# 更新版本
rustup update
```

### Rust 工具与 Python 工具对照

| 用途 | Python | Rust |
|---------|--------|------|
| 语言核心/运行时 | `python` (解释器) | `rustc` (编译器，很少手动调用) |
| 包管理器 | `pip` / `poetry` / `uv` | `cargo` (内建) |
| 项目配置 | `pyproject.toml` | `Cargo.toml` |
| 依赖锁定 | `requirements.txt` / `poetry.lock` | `Cargo.lock` |
| 虚拟环境 | `venv` / `conda` | 不需要 (依赖通常按项目隔离) |
| 格式化工具 | `black` / `ruff format` | `rustfmt` (内建: `cargo fmt`) |
| 静态检查/Linter | `pylint` / `ruff` | `clippy` (内建: `cargo clippy`) |
| 类型检查 | `mypy` / `pyright` | 编译器内建 (始终开启) |
| 测试运行器 | `pytest` | `cargo test` (内建) |

### IDE 配置

**VS Code** (强烈推荐)：
安装以下插件：
- **rust-analyzer**：提供智能代码补全、跳转、类型提示（必备）。
- **Even Better TOML**：支持 `Cargo.toml` 语法高亮。
- **CodeLLDB**：支持调试。

***

## 你的第一个 Rust 程序

### Python 版本
```python
# hello.py
print("Hello, World!")
```

### Rust 版本
```rust
// src/main.rs - 必须先编译
fn main() {
    println!("Hello, World!"); // 注意那个感叹号，它表示宏
}

// 运行命令：
// cargo run
```

### 面向 Python 开发者的关键差异

| 条目 | Python | Rust |
|------|--------|------|
| 入口点 | 无需 main 函数 | 必须在 `fn main()` 中开始 |
| 代码块 | 缩进 | 花括号 `{}` |
| 语句结尾 | 换行 | 分号 `;` |
| 变量赋值 | 默认可变 | 默认不可变 |
| 执行方式 | 直接运行脚本 | 编译后运行二进制 |
| 错误发现 | 运行时 (Runtime) | 编译时 (Compile-time) |

### 创建项目
```bash
# Python                              # Rust
mkdir myproject                        cargo new myproject
cd myproject                           cd myproject
python -m venv .venv                   # 不需要虚拟环境
```

> **核心差异**：Rust 项目结构更简洁，没有 `__init__.py`，没有复杂的虚拟环境激活过程，只需 `Cargo.toml` 和 `src/` 源码目录。

***

## Cargo 与 pip/Poetry 对比

### 常用命令对照表

| 动作 | Python | Rust |
|------|--------|------|
| 添加依赖 | `pip install requests` | `cargo add reqwest` |
| 安装全部依赖 | `pip install -r req.txt` | `cargo build` (自动完成) |
| 运行测试 | `pytest` | `cargo test` |
| 整理代码格式 | `black .` | `cargo fmt` |
| 代码静态扫描 | `ruff .` | `cargo clippy` |
| 生成文档 | `sphinx-build` | `cargo doc --open` |

***

## 核心关键字

- **`let`**：定义变量。默认情况下，变量是不可变的。
- **`mut`**：如果希望变量可被修改，必须显式加上 `mut`。
- **`match`**：强化的模式匹配，比 Python 的 `match` 更严谨（必须处理所有可能性）。
- **`impl`**：为结构体或枚举实现方法，类似于在 Python 类里定义函数。
- **`trait`**：类似于 Python 的协议 (Protocol) 或抽象基类 (ABC)，定义了一组行为规范。

---

## 练习

<details>
<summary><strong>练习：创建你的第一个项目</strong> (点击展开)</summary>

**挑战**：创建一个新项目，并在 `main.rs` 中：
1. 用 `let` 定义一个字符串变量 `name`。
2. 用 `let mut` 定义一个计数器 `count`。
3. 使用 `for _ in 1..=5` 循环，将计数器加 1 并打印。
4. 使用 `match` 判断最终计数结果的属性。

<details>
<summary>参考答案</summary>

```rust
fn main() {
    let name = "Rustacean";
    let mut count = 0;

    for i in 1..=5 {
        count += 1;
        println!("{}, 第 {} 次打招呼", name, count);
    }

    match count {
        5 => println!("任务完成！"),
        _ => println!("发生了一些错误。"),
    }
}
```
</details>
</details>

***
