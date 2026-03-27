# Installation and Setup

> **What you'll learn:** How to install Rust and set up your IDE, the Cargo build system vs MSBuild/NuGet, your first Rust program compared to C#, and how to read command-line input.
>
> **Difficulty:** 🟢 Beginner

---

## Installing Rust

```bash
# Install Rust (works on Windows, macOS, Linux)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# On Windows, you can also download from: https://rustup.rs/
```

### Rust Tools vs C# Tools

| **C# Tool** | **Rust Equivalent** | **Purpose** |
| :--- | :--- | :--- |
| `dotnet new` | `cargo new` | Create new project |
| `dotnet build` | `cargo build` | Compile project |
| `dotnet run` | `cargo run` | Run project |
| `dotnet test` | `cargo test` | Run tests |
| **NuGet** | **Crates.io** | Package repository |
| **MSBuild** | **Cargo** | Build system |
| **Visual Studio** | **VS Code + rust-analyzer** | IDE |

### IDE Setup

1.  **VS Code** (Recommended)
    *   Install "rust-analyzer" extension.
    *   Install "CodeLLDB" for debugging.
2.  **Visual Studio** (Windows)
    *   Install Rust support extension.
3.  **JetBrains RustRover** (Full IDE)
    *   Similar to Rider for C#.

---

## Your First Rust Program

### C# Hello World
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

### Rust Hello World
```rust
// main.rs
fn main() {
    println!("Hello, World!");
}
```

### Key Differences for C# Developers
1.  **No classes required** - Functions can exist at the top level.
2.  **No namespaces** - Uses the module system instead.
3.  **`println!` is a macro** - Notice the `!`, it handles variable arguments type-safely.
4.  **No explicit return type** - `main` returns `()` (unit type) by default.

---

## Reading Input and CLI Arguments

Every C# developer knows `Console.ReadLine()`. Here's how to handle user input and command-line arguments in Rust.

### Console Input
```csharp
// C#
Console.Write("Enter your name: ");
string? name = Console.ReadLine();
Console.WriteLine($"Hello, {name}!");
```

```rust
// Rust
use std::io::{self, Write};

fn main() {
    print!("Enter your name: ");
    io::stdout().flush().unwrap(); // Flush because print! doesn't auto-flush

    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Failed to read line");
    let name = name.trim();
    println!("Hello, {name}!");
}
```

### Production CLI Apps with `clap`
For professional CLI tools, use the **`clap`** crate. It's the equivalent of `System.CommandLine`.

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
    println!("Processing: {}", args.input);
}
```
