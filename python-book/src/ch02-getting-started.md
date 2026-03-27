## Installation and Setup / 安装与环境配置

> **What you'll learn / 你将学到：** How to install Rust and its toolchain, the Cargo build system vs pip/Poetry, IDE setup, your first `Hello, world!` program, and essential Rust keywords mapped to Python equivalents.
>
> 如何安装 Rust 及其工具链、Cargo 与 pip/Poetry 的区别、IDE 配置、你的第一个 `Hello, world!` 程序，以及若干对 Python 开发者特别重要的 Rust 关键字。
>
> **Difficulty / 难度：** 🟢 Beginner / 初级

### Installing Rust / 安装 Rust
```bash
# Install Rust via rustup (Linux/macOS/WSL)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Verify installation
rustc --version     # Rust compiler
cargo --version     # Build tool + package manager (like pip + setuptools combined)

# Update Rust
rustup update
```

### Rust Tools vs Python Tools / Rust 工具与 Python 工具对照

| Purpose / 用途 | Python | Rust |
|---------|--------|------|
| Language runtime / 语言执行核心 | `python` (interpreter) / 解释器 | `rustc` (compiler, rarely called directly) / 编译器（通常不直接手动调用） |
| Package manager / 包管理 | `pip` / `poetry` / `uv` | `cargo` (built-in) / 内建 `cargo` |
| Project config / 项目配置 | `pyproject.toml` | `Cargo.toml` |
| Lock file / 锁文件 | `poetry.lock` / `requirements.txt` | `Cargo.lock` |
| Virtual env / 虚拟环境 | `venv` / `conda` | Not needed (deps are per-project) / 不需要（依赖天然按项目隔离） |
| Formatter / 格式化工具 | `black` / `ruff format` | `rustfmt` (built-in: `cargo fmt`) |
| Linter / 静态检查 | `ruff` / `flake8` / `pylint` | `clippy` (built-in: `cargo clippy`) |
| Type checker / 类型检查 | `mypy` / `pyright` | Built into compiler (always on) / 编译器内建、始终启用 |
| Test runner / 测试运行器 | `pytest` | `cargo test` (built-in) |
| Docs / 文档生成 | `sphinx` / `mkdocs` | `cargo doc` (built-in) |
| REPL / 交互解释器 | `python` / `ipython` | None (use `cargo test` or Rust Playground) / 无标准 REPL（可用 `cargo test` 或 Rust Playground） |

### IDE Setup / IDE 配置

**VS Code** (recommended / 推荐)：
```text
Extensions to install:
- rust-analyzer        -> Essential: IDE features, type hints, completions
- Even Better TOML     -> Syntax highlighting for Cargo.toml
- CodeLLDB             -> Debugger support

# Python equivalent mapping:
# rust-analyzer ~= Pylance (but with 100% type coverage, always)
# cargo clippy  ~= ruff (but checks correctness, not just style)
```

***

## Your First Rust Program / 你的第一个 Rust 程序

### Python Hello World / Python 版本 Hello World
```python
# hello.py - just run it
print("Hello, World!")

# Run:
# python hello.py
```

### Rust Hello World / Rust 版本 Hello World
```rust
// src/main.rs - must be compiled first
fn main() {
    println!("Hello, World!");   // println! is a macro (note the !)
}

// Build and run:
// cargo run
```

### Key Differences for Python Developers / 面向 Python 开发者的关键差异

```text
Python:                              Rust:
--------                             -----
- No main() needed                   - fn main() is the entry point
- Indentation = blocks               - Curly braces {} = blocks
- print() is a function              - println!() is a macro (the ! matters)
- No semicolons                      - Semicolons end statements
- No type declarations               - Types inferred but always known
- Interpreted (run directly)         - Compiled (cargo build, then run)
- Errors at runtime                  - Most errors at compile time
```

### Creating Your First Project / 创建你的第一个项目
```bash
# Python                              # Rust
mkdir myproject                        cargo new myproject
cd myproject                           cd myproject
python -m venv .venv                   # No virtual env needed
source .venv/bin/activate              # No activation needed
# Create files manually               # src/main.rs already created

# Python project structure:            Rust project structure:
# myproject/                           myproject/
# ├── pyproject.toml                   ├── Cargo.toml        (like pyproject.toml)
# ├── src/                             ├── src/
# │   └── myproject/                   │   └── main.rs       (entry point)
# │       ├── __init__.py              └── (no __init__.py needed)
# │       └── main.py
# └── tests/
#     └── test_main.py
```

```mermaid
graph TD
    subgraph Python ["Python Project"]
        PP["pyproject.toml"] --- PS["src/"]
        PS --- PM["myproject/"]
        PM --- PI["__init__.py"]
        PM --- PMN["main.py"]
        PP --- PT["tests/"]
    end
    subgraph Rust ["Rust Project"]
        RC["Cargo.toml"] --- RS["src/"]
        RS --- RM["main.rs"]
        RC --- RTG["target/ (auto-generated)"]
    end
    style Python fill:#ffeeba
    style Rust fill:#d4edda
```

> **Key difference / 关键差异：** Rust projects are simpler - no `__init__.py`, no virtual environments, no `setup.py` vs `setup.cfg` vs `pyproject.toml` confusion. Just `Cargo.toml` + `src/`.
>
> Rust 项目结构通常更简单：没有 `__init__.py`，不需要虚拟环境，也没有 `setup.py`、`setup.cfg`、`pyproject.toml` 的多套配置混乱。核心就是 `Cargo.toml` 加 `src/`。

***

## Cargo vs pip/Poetry / Cargo 与 pip/Poetry 对比

### Project Configuration / 项目配置

```toml
# Python - pyproject.toml
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
# Rust - Cargo.toml
[package]
name = "myproject"
version = "0.1.0"
edition = "2021"          # Rust edition (like Python version)

[dependencies]
reqwest = "0.12"          # HTTP client (like requests)
serde = { version = "1.0", features = ["derive"] }  # Serialization (like pydantic)

[dev-dependencies]
# Test dependencies - only compiled for `cargo test`
# (No separate test config needed - `cargo test` is built in)
```

### Common Cargo Commands / 常用 Cargo 命令
```bash
# Python equivalent                # Rust
pip install requests               cargo add reqwest
pip install -r requirements.txt    cargo build           # auto-installs deps
pip install -e .                   cargo build            # always "editable"
python -m pytest                   cargo test
python -m mypy .                   # Built into compiler - always runs
ruff check .                       cargo clippy
ruff format .                      cargo fmt
python main.py                     cargo run
python -c "..."                    # No equivalent - use cargo run or tests

# Rust-specific:
cargo new myproject                # Create new project
cargo build --release              # Optimized build (10-100x faster than debug)
cargo doc --open                   # Generate and browse API docs
cargo update                       # Update deps (like pip install --upgrade)
```

***


## Essential Rust Keywords for Python Developers / 面向 Python 开发者的 Rust 核心关键字

### Variable and Mutability Keywords / 变量与可变性关键字

```rust
// let - declare a variable (like Python assignment, but immutable by default)
let name = "Alice";          // Python: name = "Alice" (but mutable)
// name = "Bob";             // Compile error! Immutable by default

// mut - opt into mutability
let mut count = 0;           // Python: count = 0 (always mutable in Python)
count += 1;                  // Allowed because of `mut`

// const - compile-time constant (like Python's convention of UPPER_CASE, but enforced)
const MAX_SIZE: usize = 1024;   // Python: MAX_SIZE = 1024 (convention only)

// static - global variable (use sparingly; Python has module-level globals)
static VERSION: &str = "1.0";
```

### Ownership and Borrowing Keywords / 所有权与借用关键字

```rust
// These have NO Python equivalents - they're Rust-specific concepts

// & - borrow (read-only reference)
fn print_name(name: &str) { }    // Python: def print_name(name: str) - but Python passes references always

// &mut - mutable borrow
fn append(list: &mut Vec<i32>) { }  // Python: def append(lst: list) - always mutable in Python

// move - transfer ownership (happens implicitly in Rust, never in Python)
let s1 = String::from("hello");
let s2 = s1;    // s1 is MOVED to s2 - s1 is no longer valid
// println!("{}", s1);  // Compile error: value moved
```

### Type Definition Keywords / 类型定义关键字

```rust
// struct - like a Python dataclass or NamedTuple
struct Point {               // @dataclass
    x: f64,                  // class Point:
    y: f64,                  //     x: float
}                            //     y: float

// enum - like Python's enum but MUCH more powerful (carries data)
enum Shape {                 // No direct Python equivalent
    Circle(f64),             // Each variant can hold different data
    Rectangle(f64, f64),
}

// impl - attach methods to a type (like defining methods in a class)
impl Point {                 // class Point:
    fn distance(&self) -> f64 {  //     def distance(self) -> float:
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

// trait - like Python's ABC or Protocol (PEP 544)
trait Drawable {             // class Drawable(Protocol):
    fn draw(&self);          //     def draw(self) -> None: ...
}

// type - type alias (like Python's TypeAlias)
type UserId = i64;           // UserId = int  (or TypeAlias)
```

### Control Flow Keywords / 控制流关键字

```rust
// match - exhaustive pattern matching (like Python 3.10+ match, but enforced)
match value {
    1 => println!("one"),
    2 | 3 => println!("two or three"),
    _ => println!("other"),          // _ = wildcard (like Python's case _:)
}

// if let - destructure + conditional
if let Some(x) = optional_value {
    println!("{}", x);
}

// loop - infinite loop (like while True:)
loop {
    break;  // Must break to exit
}

// for - iteration (like Python's for, but needs .iter() more often)
for item in collection.iter() {      // for item in collection:
    println!("{}", item);
}

// while let - loop with destructuring
while let Some(item) = stack.pop() {
    process(item);
}
```

### Visibility Keywords / 可见性关键字

```rust
// pub - public (Python has no real private; uses _ convention)
pub fn greet() { }           // def greet():  - everything is "public" in Python

// pub(crate) - visible within the crate only
pub(crate) fn internal() { } // def _internal():  - single underscore convention

// (no keyword) - private to the module
fn private_helper() { }      // def __private():  - double underscore name mangling

// In Python, "private" is a gentleman's agreement.
// In Rust, private is enforced by the compiler.
```

---

## Exercises / 练习

<details>
<summary><strong>Exercise: First Rust Program / 练习：第一个 Rust 程序</strong> (click to expand / 点击展开)</summary>

**Challenge / 挑战：** Create a new Rust project and write a program that:

创建一个新的 Rust 项目，并编写程序完成以下任务：
1. Declares a variable `name` with your name (type `&str`)  
   声明一个变量 `name`，保存你的名字（类型为 `&str`）
2. Declares a mutable variable `count` starting at 0  
   声明一个可变变量 `count`，初始值为 0
3. Uses a `for` loop from 1..=5 to increment `count` and print `"Hello, {name}! (count: {count})"`  
   使用 `1..=5` 的 `for` 循环递增 `count`，并打印 `"Hello, {name}! (count: {count})"`
4. After the loop, print whether count is even or odd using a `match` expression  
   循环结束后，使用 `match` 表达式判断 `count` 是偶数还是奇数并打印

<details>
<summary>Solution / 参考答案</summary>

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

**Key takeaways / 关键要点：**
- `let` is immutable by default (you need `mut` to change `count`)  
  `let` 默认不可变；如果要修改 `count`，必须显式加 `mut`
- `1..=5` is inclusive range (Python's `range(1, 6)`)  
  `1..=5` 是包含结束值的区间，对应 Python 的 `range(1, 6)`
- `match` is an expression that returns a value  
  `match` 是能返回值的表达式
- No `self`, no `if __name__ == "__main__"` - just `fn main()`  
  不需要 `self`，也不需要 `if __name__ == "__main__"`，只需 `fn main()`

</details>
</details>

***
