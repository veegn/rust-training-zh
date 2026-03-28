[English Original](../en/ch02-getting-started.md)

# 安装与环境配置

> **你将学到：** 如何安装 Rust 及其工具链、如何配置 IDE、Cargo 与 MSBuild/NuGet 的差异、你的第一个 Rust 程序与 C# 的对比，以及如何读取命令行输入。
>
> **难度：** 🟢 初级

---

## 安装 Rust

```bash
# 安装 Rust (适用于 Windows, macOS, Linux)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 在 Windows 上，你也可以从以下网址下载：https://rustup.rs/
```

### Rust 工具与 C# 工具对照

| **C# 工具** | **Rust 等效** | **用途** |
| :--- | :--- | :--- |
| `dotnet new` | `cargo new` | 创建新项目 |
| `dotnet build` | `cargo build` | 编译项目 |
| `dotnet run` | `cargo run` | 运行项目 |
| `dotnet test` | `cargo test` | 运行测试 |
| **NuGet** | **Crates.io** | 包仓库 |
| **MSBuild** | **Cargo** | 构建系统 |
| **Visual Studio** | **VS Code + rust-analyzer** | IDE |

### IDE 配置

1.  **VS Code** (推荐初学者使用)
    *   安装 `rust-analyzer` 扩展。
    *   安装 `CodeLLDB` 用于调试。
2.  **Visual Studio** (Windows)
    *   安装 Rust 支持扩展。
3.  **JetBrains RustRover** (完全形态的 IDE)
    *   类似于 C# 世界中的 Rider。

---

## 你的第一个 Rust 程序

### C# 版本 Hello World
```csharp
// Program.cs
using System;
namespace HelloWorld {
    class Program {
        static void Main(string[] args) {
            Console.WriteLine("Hello, World!");
        }
    }
}
```

### Rust 版本 Hello World
```rust
// main.rs
fn main() {
    println!("Hello, World!");
}
```

### 面向 C# 开发者的关键差异
1.  **不需要类**：函数可以直接存在于顶层。
2.  **没有命名空间**：改用模块系统代替。
3.  **`println!` 是宏**：注意末尾的 `!`，它以类型安全的方式处理变长参数。
4.  **无需显式返回类型**：`main` 默认返回 `()`（单元类型）。

---

## 读取输入与命令行参数

每个 C# 开发者都熟悉 `Console.ReadLine()`。下面看看在 Rust 中如何处理用户输入、环境变量和命令行参数。

### 控制台输入
```csharp
// C# – 读取用户输入
Console.Write("Enter your name: ");
string? name = Console.ReadLine(); 
Console.WriteLine($"Hello, {name}!");
```

```rust
// Rust – 读取用户输入
use std::io::{self, Write};

fn main() {
    print!("Enter your name: ");
    io::stdout().flush().unwrap(); // 因为 print! 不会自动刷新缓冲区

    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Failed to read line");
    let name = name.trim(); // 移除尾随换行符
    println!("Hello, {name}!");
}
```

### 使用 `clap` 构建生产级 CLI
只要参数解析稍微复杂一点，就应该使用 **`clap`** crate。它相当于 Rust 世界里的 `System.CommandLine`。

```rust
use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "processor", version, about)]
struct Args {
    #[arg(short, long)]
    input: String,
}

fn main() {
    let args = Args::parse();
    println!("正在处理: {}", args.input);
}
```
