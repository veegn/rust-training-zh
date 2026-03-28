# 2. Enough talk already: Show me some code 🟢

### Your First Rust Program
The syntax of a simple Rust program should feel familiar if you've worked with C-style languages.

```rust
fn main() {
    println!("Hello, world!");
}
```

- **`fn`**: The keyword used to define all functions in Rust.
- **`main()`**: The default entry point for every binary executable.
- **`println!`**: This is a **macro**, not a function (note the `!`). Unlike C preprocessor macros, Rust macros are type-safe and hygienic.

---

### Running Rust
There are several ways to run Rust code quickly:
1. **[Rust Playground](https://play.rust-lang.org/)**: Run code in your browser with no installation.
2. **Local REPL**: Install `evcxr_repl` for an interactive environment similar to Python's.
   ```bash
   cargo install evcxr_repl
   evcxr
   ```

---

### Local Installation
We recommend using **`rustup`**, the official version manager for Rust.
- **Windows**: [Download rustup-init.exe](https://rustup.rs/)
- **Linux/macOS**: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

This will install:
- **`rustc`**: The Rust compiler.
- **`cargo`**: The Swiss Army knife of Rust—it handles building, testing, dependency management, and more.

---

### Cargo: The Build System & Package Manager
Cargo is the standard tool for managing Rust projects (called **crates**). It replaces complex Makefiles or CMake systems with a simple `Cargo.toml` file.

#### Common Commands
| Command | Action |
|---------|--------|
| `cargo new my_app` | Create a new project. |
| `cargo build` | Compile the project. |
| `cargo run` | Compile and run the project. |
| `cargo check` | Quickly verify code without producing a binary. |
| `cargo test` | Run your unit and integration tests. |

#### Project Structure
```text
my_project/
├── Cargo.toml      # Project configuration and dependencies.
├── src/
│   └── main.rs     # Your source code.
└── target/         # Compiled artifacts (ignored by git).
```

***
