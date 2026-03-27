# 包管理：Cargo 与 NuGet

> **你将学到什么：** `Cargo.toml` 与 `.csproj` 的对应关系、版本说明符、`Cargo.lock` 的作用、用于条件编译的 feature flag，以及常用 Cargo 命令。
>
> **难度：** 初级

### 依赖声明
在 C# 中，你使用 `.csproj` 或 `NuGet.config`。而在 Rust 中，你需要使用 `Cargo.toml`。

#### C# .csproj
```xml
<PackageReference Include="Newtonsoft.Json" Version="13.0.3" />
```

#### Rust Cargo.toml
```toml
[dependencies]
serde_json = "1.0"
```

---

## 版本管理与锁文件
Rust 默认使用语义化版本 (SemVer)。
*   **`"1.0"`**：表示版本号应该是 `>=1.0.0` 且 `<2.0.0`。
*   **`Cargo.lock`**：此文件会自动生成，它确保了项目中每一个开发者所使用的每一个依赖包的版本是**绝对一致的**。这非常类似于现代 .NET 项目中的 `packages.lock.json`。

---

## Feature Flags: 条件编译
Cargo 包含一个非常强大的功能，即 **Features**。它可以让你将某些依赖或代码块声明为可选，从而减少编译时间和二进制文件的体积。

```toml
# Cargo.toml
[dependencies]
serde = { version = "1.0", features = ["derive"] }
```

在你的代码中，你可以通过 `#[cfg(feature = "...")]` 来条件性地包含某些代码。相比 C# 中的 `#if DEBUG` 等预处理指令，这是一个更加清晰和标准化的解决方案。

---

## 常用命令对照表

| **操作** | **`dotnet` 命令** | **`cargo` 命令** |
| :--- | :--- | :--- |
| **构建项目** | `dotnet build` | `cargo build` |
| **运行项目** | `dotnet run` | `cargo run` |
| **运行测试** | `dotnet test` | `cargo test` |
| **添加依赖包** | `dotnet add package X` | `cargo add X` |
| **更新依赖包** | `dotnet restore` | `cargo update` |

---

## 常见第三方库 (Crates) 对应物
| **C# 库** | **Rust 对应物** | **用途** |
| :--- | :--- | :--- |
| **Newtonsoft.Json** | `serde_json` | 序列化/反序列化 |
| **HttpClient** | `reqwest` | HTTP 请求 |
| **Entity Framework** | `diesel` 或 `sqlx` | 数据库/ORM |
| **Serilog / NLog** | `tracing` 或 `log` | 日志记录 |
| **xUnit / NUnit** | 内置 `#[test]` | 单元测试 |

---

## 练习：添加一个依赖
**挑战：** 向你的项目中添加用于处理日期时间的 `chrono` 库，并启用其 `serde` 功能。

**答案：**
```toml
[dependencies]
chrono = { version = "0.4", features = ["serde"] }
```
**关键理解：** Cargo 不仅仅是一个简单的包管理器；它还是一个构建系统和测试运行器，为 Rust 开发人员提供了一个统一、舒心的开发体验。
