# Modules and Crates: Code Organization

> **What you'll learn:** Rust's module system vs C# namespaces and assemblies, `pub` visibility rules, file-based organization, and how crates map to .NET assemblies.
>
> **Difficulty:** Beginner

Understanding Rust's module system is essential for organizing code. For C# developers, this is analogous to understanding namespaces, projects, and solutions.

---

## Modules vs Namespaces
In C#, namespaces are decoupled from the file system. In Rust, the module hierarchy **is** the file system (mostly).

### C# Namespace
```csharp
namespace MyApp.Models {
    public class User { }
}
```

### Rust Module
In Rust, you explicitly declare modules using the `mod` keyword.
```rust
// In lib.rs or main.rs
pub mod models; 

// In models.rs
pub struct User { }
```

---

## Visibility Modifiers
Rust's visibility is more "granular" and "nested" than C#'s.

| **Modifier** | **C# Equivalent** | **Rust Meaning** |
| :--- | :--- | :--- |
| **`pub`** | `public` | Visible to everyone |
| **(default)** | `private` | Visible only in the current module |
| **`pub(crate)`** | `internal` | Visible to the entire crate (assembly) |
| **`pub(super)`** | N/A | Visible only to the parent module |

---

## Crates: The Unit of Compilation
A **crate** is the fundamental unit of code in Rust, similar to a .NET **Assembly** (`.dll` or `.exe`).

*   **Binary Crate**: A standalone program with a `main.rs`.
*   **Library Crate**: Reusable code with a `lib.rs`.

### Cargo.toml (The `.csproj` equivalent)
```toml
[package]
name = "my_app"
version = "0.1.0"

[dependencies]
serde = "1.0" # Like a NuGet reference
```

---

## Workspaces (The `.sln` equivalent)
A **Workspace** allows you to manage multiple related crates in a single directory.

```toml
[workspace]
members = [
    "web_api",
    "business_logic",
    "data_layer",
]
```

---

## Summary for C# Developers
| **C# Concept** | **Rust Equivalent** |
| :--- | :--- |
| **Namespace** | `mod` (Module) |
| **Project (`.csproj`)** | `Package` (in `Cargo.toml`) |
| **Assembly (`.dll`)** | `Crate` |
| **Solution (`.sln`)** | `Workspace` |
| **NuGet** | `crates.io` |

---

## Exercise: Design a Module Tree
**Challenge:** Organize a project with a `services` module and a `models` module. Make `AuthService` public and `TokenStore` internal to the services module.

```rust
// lib.rs
pub mod models;
pub mod services;

// services/mod.rs
pub mod auth_service;
mod token_store; // Private to services
```
**Takeaway:** Explicitly declaring modules makes the relationship between code files clear and easy to navigate for both the developer and the compiler.
