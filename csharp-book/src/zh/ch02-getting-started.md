[English Original](../en/ch02-getting-started.md)

## 安装与设置

> **你将学到：** 如何安装 Rust 并配置集成开发环境（IDE），Cargo 构建系统与 MSBuild/NuGet 的对比，你的第一个 Rust 程序与 C# 的对比，以及如何读取命令行输入。
>
> **难度：** 🟢 初级

### 安装 Rust
```bash
# 安装 Rust (适用于 Windows, macOS, Linux)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 在 Windows 上，你也可以从以下地址下载安装程序：https://rustup.rs/
```

### Rust 工具 vs C# 工具
| C# 工具 | Rust 对应项 | 用途 |
|---------|----------------|---------|
| `dotnet new` | `cargo new` | 创建新项目 |
| `dotnet build` | `cargo build` | 编译项目 |
| `dotnet run` | `cargo run` | 运行项目 |
| `dotnet test` | `cargo test` | 运行测试 |
| NuGet | Crates.io | 包仓库 |
| MSBuild | Cargo | 构建系统 |
| Visual Studio | VS Code + rust-analyzer | 集成开发环境 (IDE) |

### IDE 设置
1. **VS Code** (对初学者推荐)
   - 安装 "rust-analyzer" 扩展
   - 安装 "CodeLLDB" 用于调试

2. **Visual Studio** (Windows)
   - 安装 Rust 支持扩展 (Rust support extension)

3. **JetBrains RustRover** (完整 IDE)
   - 类似于 C# 的 Rider

***

## 你的第一个 Rust 程序

### C# 版 Hello World
```csharp
// Program.cs
using System;

namespace HelloWorld
{
    class Program
    {
        static void Main(string[] args)
        {
            Console.WriteLine("Hello, World!");
        }
    }
}
```

### Rust 版 Hello World
```rust
// main.rs
fn main() {
    println!("Hello, World!");
}
```

### C# 开发者的关键差异
1. **无需强制使用类** - 函数可以存在于顶层（文件层级）。
2. **无需命名空间 (namespaces)** - 使用模块系统代替。
3. **`println!` 是一个宏** - 注意末尾带有 `!`。
4. **分号至关重要** - 省略末尾的分号会将语句转变为返回表达式。
5. **没有显式返回类型** - `main` 函数默认返回 `()`（单元类型/unit type）。

### 创建你的第一个项目
```bash
# 创建新项目 (类似于 'dotnet new console')
cargo new hello_rust
cd hello_rust

# 创建的项目结构：
# hello_rust/
# ├── Cargo.toml      (类似于 .csproj 文件)
# └── src/
#     └── main.rs     (类似于 Program.cs)

# 运行项目 (类似于 'dotnet run')
cargo run
```

***

## Cargo vs NuGet/MSBuild

### 项目配置

**C# (.csproj)**
```xml
<Project Sdk="Microsoft.NET.Sdk">
  <PropertyGroup>
    <OutputType>Exe</OutputType>
    <TargetFramework>net8.0</TargetFramework>
  </PropertyGroup>
  
  <PackageReference Include="Newtonsoft.Json" Version="13.0.3" />
  <PackageReference Include="Serilog" Version="3.0.1" />
</Project>
```

**Rust (Cargo.toml)**
```toml
[package]
name = "hello_rust"
version = "0.1.0"
edition = "2021"

[dependencies]
serde_json = "1.0"    # 类似于 Newtonsoft.Json
log = "0.4"           # 类似于 Serilog
```

### 常用的 Cargo 命令
```bash
# 创建新项目
cargo new my_project
cargo new my_project --lib  # 创建库项目 (library project)

# 构建与运行
cargo build          # 类似于 'dotnet build'
cargo run            # 类似于 'dotnet run'
cargo test           # 类似于 'dotnet test'

# 包管理
cargo add serde      # 添加依赖 (类似于 'dotnet add package')
cargo update         # 更新依赖项

# 发布构建 (Release build)
cargo build --release  # 优化后的构建
cargo run --release    # 运行优化后的版本

# 文档
cargo doc --open     # 生成并打开项目文档
```

### 工作区 (Workspace) vs 解决方案 (Solution)

**C# 解决方案 (.sln)**
```text
MySolution/
├── MySolution.sln
├── WebApi/
│   └── WebApi.csproj
├── Business/
│   └── Business.csproj
└── Tests/
    └── Tests.csproj
```

**Rust 工作区 (Cargo.toml)**
```toml
[workspace]
members = [
    "web_api",
    "business", 
    "tests"
]
```

***

## 读取输入与命令行参数

每位 C# 开发者都熟悉 `Console.ReadLine()`。以下是 Rust 处理用户输入、环境变量和命令行参数的方法。

### 控制台输入
```csharp
// C# — 读取用户输入
Console.Write("Enter your name: ");
string? name = Console.ReadLine();  // .NET 6+ 返回 string?
Console.WriteLine($"Hello, {name}!");

// 解析输入
Console.Write("Enter a number: ");
if (int.TryParse(Console.ReadLine(), out int number))
{
    Console.WriteLine($"You entered: {number}");
}
else
{
    Console.WriteLine("That's not a valid number.");
}
```

```rust
use std::io::{self, Write};

fn main() {
    // 读取一行输入
    print!("Enter your name: ");
    io::stdout().flush().unwrap(); // 因为 print! 不会自动刷新，所以需要手动 flush

    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Failed to read line");
    let name = name.trim(); // 去除末尾的换行符
    println!("Hello, {name}!");

    // 解析输入
    print!("Enter a number: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read");
    match input.trim().parse::<i32>() {
        Ok(number) => println!("You entered: {number}"),
        Err(_)     => println!("That's not a valid number."),
    }
}
```

### 命令行参数
```csharp
// C# — 读取命令行参数 (args)
static void Main(string[] args)
{
    if (args.Length < 1)
    {
        Console.WriteLine("Usage: program <filename>");
        return;
    }
    string filename = args[0];
    Console.WriteLine($"Processing {filename}");
}
```

```rust
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    //  args[0] = 程序名称 (类似于 C# 的 Assembly 名称)
    //  args[1..] = 实际传入的参数

    if args.len() < 2 {
        eprintln!("Usage: {} <filename>", args[0]); // eprintln! → 打印到 stderr
        std::process::exit(1);
    }
    let filename = &args[1];
    println!("Processing {filename}");
}
```

### 环境变量
```csharp
// C#
string dbUrl = Environment.GetEnvironmentVariable("DATABASE_URL") ?? "localhost";
```

```rust
use std::env;

let db_url = env::var("DATABASE_URL").unwrap_or_else(|_| "localhost".to_string());
// env::var 返回 Result<String, VarError> — 没有 null！
```

### 使用 `clap` 构建生产级命令行应用

对于简单的参数解析之外的任何需求，请使用 **`clap`** crate —— 它是 Rust 中对应 `System.CommandLine` 或 `CommandLineParser` 等库的工具。

```toml
# Cargo.toml
[dependencies]
clap = { version = "4", features = ["derive"] }
```

```rust
use clap::Parser;

/// 一个简单的文件处理器 — 此处的文档注释将成为 --help 的文本
#[derive(Parser, Debug)]
#[command(name = "processor", version, about)]
struct Args {
    /// 要处理的输入文件
    #[arg(short, long)]
    input: String,

    /// 输出文件 (默认为标准输出)
    #[arg(short, long)]
    output: Option<String>,

    /// 开启详细日志
    #[arg(short, long, default_value_t = false)]
    verbose: bool,

    /// 工作线程数量
    #[arg(short = 'j', long, default_value_t = 4)]
    threads: usize,
}

fn main() {
    let args = Args::parse(); // 自动解析、验证并生成 --help 帮助文本

    if args.verbose {
        println!("Input:   {}", args.input);
        println!("Output:  {:?}", args.output);
        println!("Threads: {}", args.threads);
    }

    // 接下来可以使用 args.input, args.output 等字段
}
```

```bash
# 自动生成的帮助文本：
$ processor --help
A simple file processor

Usage: processor [OPTIONS] --input <INPUT>

Options:
  -i, --input <INPUT>      Input file to process
  -o, --output <OUTPUT>    Output file (defaults to stdout)
  -v, --verbose            Enable verbose logging
  -j, --threads <THREADS>  Number of worker threads [default: 4]
  -h, --help               Print help
  -V, --version            Print version
```

```csharp
// 使用 System.CommandLine 的 C# 等效项 (样板代码较多):
var inputOption = new Option<string>("--input", "Input file") { IsRequired = true };
var verboseOption = new Option<bool>("--verbose", "Enable verbose logging");
var rootCommand = new RootCommand("A simple file processor");
rootCommand.AddOption(inputOption);
rootCommand.AddOption(verboseOption);
rootCommand.SetHandler((input, verbose) => { /* ... */ }, inputOption, verboseOption);
await rootCommand.InvokeAsync(args);
// clap 的 derive 宏方法更加简洁且类型安全
```

| 功能项 | C# | Rust | 备注 |
|----|------|-------|-------|
| 读取行 | `Console.ReadLine()` | `io::stdin().read_line(&mut buf)` | 必须提供缓冲区，返回 `Result` |
| 解析整数 | `int.TryParse(s, out n)` | `s.parse::<i32>()` | 返回 `Result<i32, ParseIntError>` |
| 命令行参数 | `args[0]` | `env::args().nth(1)` | Rust 的 args[0] 是程序本身路径 |
| 环境变量 | `Environment.GetEnvironmentVariable` | `env::var("KEY")` | 返回 `Result` 而非可为空的值 |
| 命令行库 | `System.CommandLine` | `clap` | 基于派生宏，自动生成帮助信息 |

***
