## Installation and Setup / 安装与环境配置

> **What you'll learn / 你将学到：** How to install Rust and set up your IDE, the Cargo build system vs MSBuild/NuGet, your first Rust program compared to C#, and how to read command-line input.
>
> 如何安装 Rust 及其工具链、如何配置 IDE、Cargo 与 MSBuild/NuGet 的差异、你的第一个 Rust 程序与 C# 的对比，以及如何读取命令行输入。
>
> **Difficulty / 难度：** 🟢 Beginner / 初级

### Installing Rust / 安装 Rust
```bash
# Install Rust (works on Windows, macOS, Linux)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# On Windows, you can also download from: https://rustup.rs/
```

### Rust Tools vs C# Tools / Rust 工具与 C# 工具对照
| C# Tool | Rust Equivalent | Purpose / 用途 |
|---------|----------------|---------|
| `dotnet new` | `cargo new` | Create new project / 创建新项目 |
| `dotnet build` | `cargo build` | Compile project / 编译项目 |
| `dotnet run` | `cargo run` | Run project / 运行项目 |
| `dotnet test` | `cargo test` | Run tests / 运行测试 |
| NuGet | Crates.io | Package repository / 包仓库 |
| MSBuild | Cargo | Build system / 构建系统 |
| Visual Studio | VS Code + rust-analyzer | IDE / 集成开发环境 |

### IDE Setup / IDE 配置
1. **VS Code** (Recommended for beginners / 推荐初学者使用)
   - Install "rust-analyzer" extension / 安装 `rust-analyzer` 扩展
   - Install "CodeLLDB" for debugging / 安装 `CodeLLDB` 用于调试

2. **Visual Studio** (Windows)
   - Install Rust support extension / 安装 Rust 支持扩展

3. **JetBrains RustRover** (Full IDE / 完整 IDE)
   - Similar to Rider for C# / 类似于 C# 世界中的 Rider

***

## Your First Rust Program / 你的第一个 Rust 程序

### C# Hello World / C# 版本 Hello World
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

### Rust Hello World / Rust 版本 Hello World
```rust
// main.rs
fn main() {
    println!("Hello, World!");
}
```

### Key Differences for C# Developers / 面向 C# 开发者的关键差异
1. **No classes required** - Functions can exist at the top level  
   **不需要类**：函数可以直接存在于顶层
2. **No namespaces** - Uses module system instead  
   **没有命名空间**：改用模块系统
3. **`println!` is a macro** - Notice the `!`  
   **`println!` 是宏**：注意末尾的 `!`
4. **No semicolon after println!** - Expression vs statement  
   **`println!` 后面这里没有多余语义变化**：要理解表达式与语句的区别
5. **No explicit return type** - `main` returns `()` (unit type)  
   **无需显式返回类型**：`main` 默认返回 `()`（单元类型）

### Creating Your First Project / 创建你的第一个项目
```bash
# Create new project (like 'dotnet new console')
cargo new hello_rust
cd hello_rust

# Project structure created:
# hello_rust/
# ├── Cargo.toml      (like .csproj file)
# └── src/
#     └── main.rs     (like Program.cs)

# Run the project (like 'dotnet run')
cargo run
```

***

## Cargo vs NuGet/MSBuild / Cargo 与 NuGet/MSBuild 对比

### Project Configuration / 项目配置

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
serde_json = "1.0"    # Like Newtonsoft.Json
log = "0.4"           # Like Serilog
```

### Common Cargo Commands / 常用 Cargo 命令
```bash
# Create new project
cargo new my_project
cargo new my_project --lib  # Create library project

# Build and run
cargo build          # Like 'dotnet build'
cargo run            # Like 'dotnet run'
cargo test           # Like 'dotnet test'

# Package management
cargo add serde      # Add dependency (like 'dotnet add package')
cargo update         # Update dependencies

# Release build
cargo build --release  # Optimized build
cargo run --release    # Run optimized version

# Documentation
cargo doc --open     # Generate and open docs
```

### Workspace vs Solution / Workspace 与 Solution

**C# Solution (.sln)**
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

**Rust Workspace (Cargo.toml)**
```toml
[workspace]
members = [
    "web_api",
    "business", 
    "tests"
]
```

***

## Reading Input and CLI Arguments / 读取输入与命令行参数

Every C# developer knows `Console.ReadLine()`. Here's how to handle user input, environment variables, and command-line arguments in Rust.

每个 C# 开发者都熟悉 `Console.ReadLine()`。下面看看在 Rust 中如何处理用户输入、环境变量和命令行参数。

### Console Input / 控制台输入
```csharp
// C# – reading user input
Console.Write("Enter your name: ");
string? name = Console.ReadLine();  // Returns string? in .NET 6+
Console.WriteLine($"Hello, {name}!");

// Parsing input
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
    // Reading a line of input
    print!("Enter your name: ");
    io::stdout().flush().unwrap(); // flush because print! doesn't auto-flush

    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Failed to read line");
    let name = name.trim(); // remove trailing newline
    println!("Hello, {name}!");

    // Parsing input
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

### Command-Line Arguments / 命令行参数
```csharp
// C# – reading CLI args
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
    //  args[0] = program name (like C#'s Assembly name)
    //  args[1..] = actual arguments

    if args.len() < 2 {
        eprintln!("Usage: {} <filename>", args[0]); // eprintln! -> stderr
        std::process::exit(1);
    }
    let filename = &args[1];
    println!("Processing {filename}");
}
```

### Environment Variables / 环境变量
```csharp
// C#
string dbUrl = Environment.GetEnvironmentVariable("DATABASE_URL") ?? "localhost";
```

```rust
use std::env;

let db_url = env::var("DATABASE_URL").unwrap_or_else(|_| "localhost".to_string());
// env::var returns Result<String, VarError> – no nulls!
```

### Production CLI Apps with `clap` / 使用 `clap` 构建生产级 CLI

For anything beyond trivial argument parsing, use the **`clap`** crate - it's the Rust equivalent of `System.CommandLine` or libraries like `CommandLineParser`.

只要参数解析稍微复杂一点，就应该使用 **`clap`** crate。它相当于 Rust 世界里的 `System.CommandLine` 或 `CommandLineParser` 一类库。

```toml
# Cargo.toml
[dependencies]
clap = { version = "4", features = ["derive"] }
```

```rust
use clap::Parser;

/// A simple file processor – this doc comment becomes the help text
#[derive(Parser, Debug)]
#[command(name = "processor", version, about)]
struct Args {
    /// Input file to process
    #[arg(short, long)]
    input: String,

    /// Output file (defaults to stdout)
    #[arg(short, long)]
    output: Option<String>,

    /// Enable verbose logging
    #[arg(short, long, default_value_t = false)]
    verbose: bool,

    /// Number of worker threads
    #[arg(short = 'j', long, default_value_t = 4)]
    threads: usize,
}

fn main() {
    let args = Args::parse(); // auto-parses, validates, generates --help

    if args.verbose {
        println!("Input:   {}", args.input);
        println!("Output:  {:?}", args.output);
        println!("Threads: {}", args.threads);
    }

    // Use args.input, args.output, etc.
}
```

```bash
# Auto-generated help:
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
// C# equivalent with System.CommandLine (more boilerplate):
var inputOption = new Option<string>("--input", "Input file") { IsRequired = true };
var verboseOption = new Option<bool>("--verbose", "Enable verbose logging");
var rootCommand = new RootCommand("A simple file processor");
rootCommand.AddOption(inputOption);
rootCommand.AddOption(verboseOption);
rootCommand.SetHandler((input, verbose) => { /* ... */ }, inputOption, verboseOption);
await rootCommand.InvokeAsync(args);
// clap's derive macro approach is more concise and type-safe
```

| C# | Rust | Notes / 说明 |
|----|------|-------|
| `Console.ReadLine()` | `io::stdin().read_line(&mut buf)` | Must provide buffer, returns `Result` / 需要提供缓冲区，并返回 `Result` |
| `int.TryParse(s, out n)` | `s.parse::<i32>()` | Returns `Result<i32, ParseIntError>` / 返回 `Result<i32, ParseIntError>` |
| `args[0]` | `env::args().nth(1)` | Rust args[0] = program name / Rust 中 `args[0]` 是程序名 |
| `Environment.GetEnvironmentVariable` | `env::var("KEY")` | Returns `Result`, not nullable / 返回 `Result`，不是可空值 |
| `System.CommandLine` | `clap` | Derive-based, auto-generates help / 基于 derive，自动生成帮助文本 |

***
