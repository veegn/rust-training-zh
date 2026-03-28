[English Original](../en/ch08-crates-and-modules.md)

# 模块与 Crate：代码组织方式

> **你将学到什么：** Rust 的模块系统如何对应 C# 的命名空间与程序集，`pub` 可见性规则，基于文件的模块组织方式，以及 crate 与 .NET 程序集之间的映射关系。
>
> **难度：** 初级

理解 Rust 的模块系统是组织代码的基础。对于 C# 开发者而言，这相当于理解命名空间、项目和解决方案之间的关系。

---

## 模块 vs 命名空间
在 C# 中，命名空间与文件系统是解耦的。而在 Rust 中，模块的层级结构（Module Hierarchy）通常就是文件系统的层级。

### C# 命名空间
```csharp
namespace MyApp.Models {
    public class User { }
}
```

### Rust 模块
在 Rust 中，你需要使用 `mod` 关键字显式地声明模块。
```rust
// 在 lib.rs 或 main.rs 中
pub mod models; 

// 在 models.rs 中
pub struct User { }
```

---

## 可见性修饰符 (Visibility Modifiers)
Rust 的可见性修饰符支持比 C# 更加“细腻”和“嵌套”的控制。

| **修饰符** | **C# 对应物** | **Rust 含义** |
| :--- | :--- | :--- |
| **`pub`** | `public` | 对外部所有人可见 |
| **(默认)** | `private` | 仅在当前模块内可见 |
| **`pub(crate)`** | `internal` | 在整个 crate（程序集）内可见 |
| **`pub(super)`** | 无 | 仅对父级模块可见 |

---

## Crate：编译的基本单元
**Crate** 是 Rust 代码的基本单位，类似于 .NET 中的 **程序集 (Assembly)**（即 `.dll` 或 `.exe` 文件）。

*   **Binary Crate** (二进制 crate)：一个独立的控制台程序（包含 `main.rs`）。
*   **Library Crate** (库 crate)：可复用的库代码（包含 `lib.rs`）。

### Cargo.toml (对应 `.csproj`)
```toml
[package]
name = "my_app"
version = "0.1.0"

[dependencies]
serde = "1.0" # 类似于 NuGet 依赖引用
```

---

## Workspace (对应 `.sln`)
一个 **Workspace** 允许你在单一目录下管理多个相关的 crate。

```toml
[workspace]
members = [
    "web_api",
    "business_logic",
    "data_layer",
]
```

---

## C# 开发者总结表
| **C# 概念** | **Rust 对应物** |
| :--- | :--- |
| **命名空间 (Namespace)** | `mod` (模块) |
| **项目 (`.csproj`)** | `Package` (出现在 `Cargo.toml` 中) |
| **程序集 (`.dll`)** | `Crate` |
| **解决方案 (`.sln`)** | `Workspace` |
| **NuGet** | `crates.io` |

---

## 练习：设计模块树
**挑战：** 组织一个包含 `services` 和 `models` 模块的项目。确保 `AuthService` 是公开的，而 `TokenStore` 对 `services` 模块内部可见但在外部私有。

```rust
// lib.rs
pub mod models;
pub mod services;

// services/mod.rs
pub mod auth_service;
mod token_store; // 仅对 services 模块私有
```
**关键理解：** 显式地声明模块使得代码文件之间的引用关系变得清晰透明。无论对开发者还是编译器来说，这都更易于进行依赖管理。
