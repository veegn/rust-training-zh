# 面向 C# 开发者的常用 Crate

> **你将学到什么：** 常见的 .NET 库在 Rust 生态里的对应 Crate，例如 Serde (JSON.NET)、Reqwest (HttpClient) 以及 Tokio (Task/async)。
>
> **难度：** 中级

C# 开发者在刚接触 Rust 时，最常问的一个问题就是：“这个功能在 Rust 里对应哪个库？”这一章节为你提供了一个关于 Rust 生态系统中核心 Crate（即库）的快速对照。

---

## “三巨头” Crate
如果你正在构建一个 Web 服务或一个命令行工具，你几乎百分之百会用到这三个 Crate：

### 1. Serde (序列化与反序列化)
*   **C# 对应物**：`Newtonsoft.Json` 或 `System.Text.Json`。
*   **用途**：实现 Rust 数据结构与 JSON、YAML、TOML 等格式之间相互转换的通用框架。
*   **核心特性**：使用 `#[derive(Serialize, Deserialize)]` 在编译期生成零成本的转换代码。

### 2. Reqwest (HTTP 客户端)
*   **C# 对应物**：`HttpClient`。
*   **用途**：一个符合人体工程学的、功能完备的 HTTP 请求发送工具。
*   **核心特性**：原生支持 `async/await` 异步语法，并与 Serde 完美集成。

### 3. Tokio (异步运行时)
*   **C# 对应物**：.NET 任务调度器 (Task Scheduler) 与 线程池 (Thread Pool)。
*   **用途**：实现事件驱动架构的运行时环境，能够高效地同时运行成千上万个并发任务。
*   **核心特性**：为绝大多数现代 Rust 网络库及 Web 框架提供了底层基础设施支撑。

---

## 库/框架对照表
| **分类** | **.NET 对应物** | **Rust 常用 Crate** |
| :--- | :--- | :--- |
| **异步任务/调度** | `Task` / `Task.Run` | `Tokio` |
| **JSON 处理** | `System.Text.Json` | `Serde` (由 `serde_json` 支撑) |
| **HTTP 客户端** | `HttpClient` | `Reqwest` |
| **日志系统** | `Serilog` / `ILogger` | `Tracing` 或 `Log` |
| **数据库/ORM** | `Entity Framework` | `SQLx` (纯 SQL) 或 `SeaORM` |
| **单元测试** | `xUnit` / `NUnit` | 内置测试框架 + `rstest` |
| **Mock 工具** | `Moq` | `Mockall` |
| **命令行参数解析** | `CommandLineParser` | `Clap` |

---

## 示例：组合使用 Crate
下面这段仅 10 行的 Rust 程序同时使用了“三巨头”来抓取一个 JSON API：

```rust
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct User { id: u32, name: String }

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let url = "https://jsonplaceholder.typicode.com/users/1";
    let user: User = reqwest::get(url).await?.json().await?; // 发送请求并解析结果
    println!("获取到的用户信息: {:?}", user);
    Ok(())
}
```

---

## C# 开发者需要了解的概念
*   **Crates 就像 NuGet 包**：你可以把它们完全理解为 NuGet 包。
*   **`Cargo.toml` 相当于 `csproj`**：这是你声明所有依赖项目的地方。
*   **特性标志 (Feature Flags)**：许多 Crate (如 Tokio 或 Reqwest) 允许你只开启其中特定的部分功能，旨在尽可能减小最终生成的二进制文件体积。

---

## 练习：探索一个 Crate
**挑战**：前往 [crates.io](https://crates.io) 并搜索 "chrono"。查看其文档中的 "Features" 部分。思考一下为什么你会想为它开启 `serde` 这个 Feature？

**关键理解**：Rust 生态系统是高度模块化的。与 .NET 那种“全家桶级框架”不同，Rust 开发者倾向于将许多小型、专业的 Crate 组合在一起。
