# 2. 少说多练：先看代码 🟢

### 你的第一个 Rust 程序
如果你接触过 C 风格的语言，那么对于一个简单的 Rust 程序，其语法应该会让你感到似曾相识。

```rust
fn main() {
    println!("Hello, world!");
}
```

- **`fn`**：Rust 中定义所有函数的关键字。
- **`main()`**：每一个二进制可执行程序的默认入口点。
- **`println!`**：这是一个 **宏 (Macro)**，而不是函数（注意那个感叹号 `!`）。与 C 预处理器宏不同，Rust 的宏是类型安全且卫生的。

---

### 运行 Rust
有几种快速运行 Rust 代码的方法：
1. **[Rust Playground](https://play.rust-lang.org/)**：无需安装，直接在浏览器中运行代码。
2. **本地 REPL**：安装 `evcxr_repl` 以获得类似于 Python 的交互式环境。
   ```bash
   cargo install evcxr_repl
   evcxr
   ```

---

### 本地安装
我们建议使用 **`rustup`**，它是 Rust 的官方版本管理器。
- **Windows**：[下载 rustup-init.exe](https://rustup.rs/)
- **Linux/macOS**：`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

这将安装：
- **`rustc`**：Rust 编译器。
- **`cargo`**：Rust 的“万用军刀”——它处理构建、测试、依赖管理等各种任务。

---

### Cargo：构建系统与包管理器
Cargo 是管理 Rust 项目（称为 **Crates**）的标准工具。它通过一个简单的 `Cargo.toml` 文件取代了复杂的 Makefiles 或 CMake 系统。

#### 常用命令
| 命令 | 动作 |
|---------|--------|
| `cargo new my_app` | 创建一个新项目。 |
| `cargo build` | 编译项目。 |
| `cargo run` | 编译并运行项目。 |
| `cargo check` | 在不生成二进制文件的情况下快速验证代码。 |
| `cargo test` | 运行单元测试和集成测试。 |

#### 项目结构
```text
my_project/
├── Cargo.toml      # 项目配置和依赖。
├── src/
│   └── main.rs     # 源代码。
└── target/         # 编译产物（会被 git 忽略）。
```

***
