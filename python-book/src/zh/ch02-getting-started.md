[English Original](../en/ch02-getting-started.md)

## 安装与环境配置

> **你将学到：** 如何安装 Rust 及其工具链、Cargo 构建系统与 pip/Poetry 的对比、IDE 配置、你的第一个 `Hello, world!` 程序，以及映射到 Python等价概念的核心 Rust 关键字。
>
> **难度：** 🟢 初级

### 安装 Rust
```bash
# 在 Linux/macOS/WSL 上通过 rustup 安装 Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 验证安装
rustc --version     # Rust 编译器
cargo --version     # 构建工具 + 包管理器（相当于 pip + setuptools 的结合体）

# 更新 Rust
rustup update
```

### Rust 工具 vs Python 工具

| 用途 | Python | Rust |
|---------|--------|------|
| 语言运行时 | `python` (解释器) | `rustc` (编译器，很少直接调用) |
| 包管理器 | `pip` / `poetry` / `uv` | `cargo` (内建) |
| 项目配置 | `pyproject.toml` | `Cargo.toml` |
| 锁文件 | `poetry.lock` / `requirements.txt` | `Cargo.lock` |
| 虚拟环境 | `venv` / `conda` | 不需要（依赖按项目管理） |
| 格式化工具 | `black` / `ruff format` | `rustfmt` (内建: `cargo fmt`) |
| 静态分析 (Linter) | `ruff` / `flake8` / `pylint` | `clippy` (内建: `cargo clippy`) |
| 类型检查器 | `mypy` / `pyright` | 已集成进编译器 (始终开启) |
| 测试运行器 | `pytest` | `cargo test` (内建) |
| 文档工具 | `sphinx` / `mkdocs` | `cargo doc` (内建) |
| REPL | `python` / `ipython` | 无 (使用 `cargo test` 或 Rust Playground) |

### IDE 配置

**VS Code** (推荐):
```text
需要安装的扩展:
- rust-analyzer        ← 核心: 提供 IDE 功能、类型提示、自动补全
- Even Better TOML     ← Cargo.toml 的语法高亮
- CodeLLDB             ← 调试器支持

# 与 Python 扩展的等价映射:
# rust-analyzer ≈ Pylance (但具备 100% 类型覆盖率)
# cargo clippy  ≈ ruff (但除了风格外更注重正确性检查)
```

---

## 你的第一个 Rust 程序

### Python 版 Hello World
```python
# hello.py — 直接运行即可
print("Hello, World!")

# 运行:
# python hello.py
```

### Rust 版 Hello World
```rust
// src/main.rs — 必须先进行编译
fn main() {
    println!("Hello, World!");   // println! 是一个宏 (注意末尾的 !)
}

// 构建并运行:
// cargo run
```

### 给 Python 开发者的关键差异点

```text
Python:                              Rust:
─────────                            ─────
- 不需要 main()                      - fn main() 是程序的入口
- 缩进即代码块                       - 花括号 {} 即代码块
- print() 是一个函数                 - println!() 是一个宏 (末尾有 !)
- 不需要分号                         - 使用分号结束语句
- 无需类型声明                       - 类型虽然是推导的，但始终明确
- 解释执行 (直接运行)                - 编译执行 (cargo build 后运行)
- 报错发生在运行时                    - 大多数错误发生在编译时
```

### 创建你的第一个项目
```bash
# Python                              # Rust
mkdir myproject                        cargo new myproject
cd myproject                           cd myproject
python -m venv .venv                   # 无需虚拟环境
source .venv/bin/activate              # 无需激活
# 需要手动创建文件                      # src/main.rs 已自动创建

# Python 项目结构:                    Rust 项目结构:
# myproject/                           myproject/
# ├── pyproject.toml                   ├── Cargo.toml        (类似 pyproject.toml)
# ├── src/                             ├── src/
# │   └── myproject/                   │   └── main.rs       (程序入口)
# │       ├── __init__.py              └── (无需 __init__.py 文件)
# │       └── main.py
# └── tests/
#     └── test_main.py
```

```mermaid
graph TD
    subgraph Python ["Python 项目"]
        PP["pyproject.toml"] --- PS["src/"]
        PS --- PM["myproject/"]
        PM --- PI["__init__.py"]
        PM --- PMN["main.py"]
        PP --- PT["tests/"]
    end
    subgraph Rust ["Rust 项目"]
        RC["Cargo.toml"] --- RS["src/"]
        RS --- RM["main.rs"]
        RC --- RTG["target/ (自动生成)"]
    end
    style Python fill:#ffeeba
    style Rust fill:#d4edda
```

> **关键差异**：Rust 项目结构更简洁 —— 没有 `__init__.py`，没有虚拟环境，也没有 `setup.py` vs `setup.cfg` vs `pyproject.toml` 之间的混乱。只有 `Cargo.toml` + `src/`。

---

## Cargo vs pip/Poetry

### 项目配置

```toml
# Python — pyproject.toml
[project]
name = "myproject"
version = "0.1.0"
requires-python = ">=3.10"
dependencies = [
    "requests>=2.28",
    "pydantic>=2.0",
]

[project.optional-dependencies]
dev = ["pytest", "ruff", "mypy"]
```

```toml
# Rust — Cargo.toml
[package]
name = "myproject"
version = "0.1.0"
edition = "2021"          # Rust 版本 (类似 Python 3.10)

[dependencies]
reqwest = "0.12"          # HTTP 客户端 (类似 requests)
serde = { version = "1.0", features = ["derive"] }  # 序列化 (类似 pydantic)

[dev-dependencies]
# 测试依赖 — 仅在 `cargo test` 时编译
# (无需单独配置测试运行器 — `cargo test` 是内建的)
```

### 常用 Cargo 命令对比
```bash
# Python 等价                      # Rust
pip install requests               cargo add reqwest
pip install -r requirements.txt    cargo build           # 自动安装依赖
pip install -e .                   cargo build            # 总是“可编辑的”
python -m pytest                   cargo test
python -m mypy .                   # 集成进编译器 — 总是运行
ruff check .                       cargo clippy
ruff format .                      cargo fmt
python main.py                     cargo run
python -c "..."                    # 无等价物 — 使用 cargo run 或测试

# Rust 特有:
cargo new myproject                # 创建新项目
cargo build --release              # 优化构建 (比 debug 慢，但运行快 10-100 倍)
cargo doc --open                   # 生成并浏览 API 文档
cargo update                       # 更新依赖 (类似 pip install --upgrade)
```

---

## 给 Python 开发者的 Rust 核心关键字

### 变量与可变性关键字

```rust
// let — 声明变量 (类似 Python 赋值，但默认不可变)
let name = "Alice";          // Python: name = "Alice" (但 Python 总是可变的)
// name = "Bob";             // ❌ 编译错误！默认不可变

// mut — 声明可变性
let mut count = 0;           // Python: count = 0 (在 Python 中总是可变的)
count += 1;                  // ✅ 允许，因为带有 `mut`

// const — 编译时常量 (类似 Python 全大写的常量约定，但 Rust 强制执行)
const MAX_SIZE: usize = 1024;   // Python: MAX_SIZE = 1024 (仅为约定)

// static — 全局变量 (谨慎使用；Python 有模块级全局变量)
static VERSION: &str = "1.0";
```

### 所有权与借用关键字

```rust
// 这里的概念在 Python 中没有等价物 — 它们是 Rust 特有的核心

// & — 借用 (只读引用)
fn print_name(name: &str) { }    // Python: def print_name(name: str) — 但 Python 总是传引用

// &mut — 可变借用
fn append(list: &mut Vec<i32>) { }  // Python: def append(lst: list) — Python 中总是可变的

// move — 转移所有权 (在 Rust 中隐式发生，在 Python 中绝不发生)
let s1 = String::from("hello");
let s2 = s1;    // s1 被转移 (MOVED) 给了 s2 — s1 后续不再有效
// println!("{}", s1);  // ❌ 编译错误: value moved
```

### 类型定义关键字

```rust
// struct — 类似 Python 的 dataclass 或 NamedTuple
struct Point {               // @dataclass
    x: f64,                  // class Point:
    y: f64,                  //     x: float
}                            //     y: float

// enum — 类似 Python 的 enum 但远更强大 (可携带数据)
enum Shape {                 // 无直接 Python 等价物
    Circle(f64),             // 每个变体可以持有不同的数据
    Rectangle(f64, f64),
}

// impl — 为类型关联方法 (类似在类中定义方法)
impl Point {                 // class Point:
    fn distance(&self) -> f64 {  //     def distance(self) -> float:
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

// trait — 类似 Python 的 ABC 或 Protocol (PEP 544)
trait Drawable {             // class Drawable(Protocol):
    fn draw(&self);          //     def draw(self) -> None: ...
}

// type — 类型别名 (类似 Python 的 TypeAlias)
type UserId = i64;           // UserId = int  (or TypeAlias)
```

### 控制流关键字

```rust
// match — 穷尽模式匹配 (类似 Python 3.10+ 的 match，但由编译器强制执行)
match value {
    1 => println!("one"),
    2 | 3 => println!("two or three"),
    _ => println!("other"),          // _ = 通配符 (类似 Python 的 case _:)
}

// if let — 解构 + 条件 (类似 Python: if (m := regex.match(s)):)
if let Some(x) = optional_value {
    println!("{}", x);
}

// loop — 无限循环 (类似 while True:)
loop {
    break;  // 必须使用 break 退出
}

// for — 迭代 (类似 Python 的 for，但通常需要使用 .iter())
for item in collection.iter() {      // for item in collection:
    println!("{}", item);
}

// while let — 带有解构的循环
while let Some(item) = stack.pop() {
    process(item);
}
```

### 可见性关键字

```rust
// pub — 公有的 (Python 中没有真正的私有；通常使用 _ 约定)
pub fn greet() { }           // def greet():  — Python 中一切都是“公有”的

// pub(crate) — 仅在当前项目 (crate) 内可见
pub(crate) fn internal() { } // def _internal():  — 单下划线约定

// (默认无关键字) — 对当前模块为私有
fn private_helper() { }      // def __private():  — 双下划线名称改写 (name mangling)

// 在 Python 中，“私有”只是“君子协定”
// 在 Rust 中，私有是由编译器强制执行的
```

---

## 练习

<details>
<summary><strong>🏋️ 练习：你的第一个 Rust 程序</strong>（点击展开）</summary>

**挑战**：创建一个新的 Rust 项目并编写一段程序：
1. 声明一个名为 `name` 的变量并存储你的名字 (类型为 `&str`)
2. 声明一个从 0 开始的可变变量 `count` 
3. 使用 `for` 循环（范围为 `1..=5`）递增 `count` 并打印 `"Hello, {name}! (count: {count})"`
4. 循环结束后，使用 `match` 表达式判断 `count` 是奇数还是偶数

<details>
<summary>🔑 答案</summary>

```bash
cargo new hello_rust && cd hello_rust
```

```rust
// src/main.rs
fn main() {
    let name = "Pythonista";
    let mut count = 0u32;

    for _ in 1..=5 {
        count += 1;
        println!("Hello, {name}! (count: {count})");
    }

    let parity = match count % 2 {
        0 => "even",
        _ => "odd",
    };
    println!("Final count {count} is {parity}");
}
```

**核心要点**:
- `let` 默认不可变（你需要 `mut` 来改变 `count`）
- `1..=5` 是包含上限的范围 (相当于 Python 的 `range(1, 6)`)
- `match` 是一个表达式，可以返回一个值
- 没有 `self`，没有 `if __name__ == "__main__"` —— 入口即是 `fn main()`

</details>
</details>

---
