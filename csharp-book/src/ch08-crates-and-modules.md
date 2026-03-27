## Modules and Crates: Code Organization | 模块与 Crate：代码组织方式

> **What you'll learn:** Rust's module system vs C# namespaces and assemblies, `pub`/`pub(crate)`/`pub(super)` visibility,
> file-based module organization, and how crates map to .NET assemblies.
>
> **你将学到什么：** Rust 的模块系统如何对应 C# 的命名空间与程序集，`pub`/`pub(crate)`/`pub(super)` 的可见性差异，
> 基于文件的模块组织方式，以及 crate 与 .NET 程序集之间的映射关系。
>
> **Difficulty:** Beginner
>
> **难度：** 初级

Understanding Rust's module system is essential for organizing code and managing dependencies. For C# developers, this is analogous to understanding namespaces, assemblies, and NuGet packages.

理解 Rust 的模块系统，是组织代码和管理依赖的基础。对于 C# 开发者来说，这可以类比为理解命名空间、程序集和 NuGet 包的关系。

### Rust Modules vs C# Namespaces | Rust 模块 vs C# 命名空间

#### C# Namespace Organization | C# 命名空间组织
```csharp
// File: Models/User.cs
namespace MyApp.Models
{
    public class User
    {
        public string Name { get; set; }
        public int Age { get; set; }
    }
}

// File: Services/UserService.cs
using MyApp.Models;

namespace MyApp.Services
{
    public class UserService
    {
        public User CreateUser(string name, int age)
        {
            return new User { Name = name, Age = age };
        }
    }
}

// File: Program.cs
using MyApp.Models;
using MyApp.Services;

namespace MyApp
{
    class Program
    {
        static void Main(string[] args)
        {
            var service = new UserService();
            var user = service.CreateUser("Alice", 30);
        }
    }
}
```

#### Rust Module Organization | Rust 模块组织
```rust
// File: src/models.rs
pub struct User {
    pub name: String,
    pub age: u32,
}

impl User {
    pub fn new(name: String, age: u32) -> User {
        User { name, age }
    }
}

// File: src/services.rs
use crate::models::User;

pub struct UserService;

impl UserService {
    pub fn create_user(name: String, age: u32) -> User {
        User::new(name, age)
    }
}

// File: src/lib.rs (or main.rs)
pub mod models;
pub mod services;

use models::User;
use services::UserService;

fn main() {
    let service = UserService;
    let user = UserService::create_user("Alice".to_string(), 30);
}
```

### Module Hierarchy and Visibility | 模块层级与可见性

```mermaid
graph TD
    Crate["crate (root)"] --> ModA["mod data"]
    Crate --> ModB["mod api"]
    ModA --> SubA1["pub struct Repo"]
    ModA --> SubA2["fn helper  (private)"]
    ModB --> SubB1["pub fn handle()"]
    ModB --> SubB2["pub(crate) fn internal()"]
    ModB --> SubB3["pub(super) fn parent_only()"]

    style SubA1 fill:#c8e6c9,color:#000
    style SubA2 fill:#ffcdd2,color:#000
    style SubB1 fill:#c8e6c9,color:#000
    style SubB2 fill:#fff9c4,color:#000
    style SubB3 fill:#fff9c4,color:#000
```

> Green = public everywhere | Yellow = restricted visibility | Red = private
>
> 绿色 = 全局公开 | 黄色 = 受限可见 | 红色 = 私有

#### C# Visibility Modifiers | C# 可见性修饰符
```csharp
namespace MyApp.Data
{
    // public - accessible from anywhere
    public class Repository
    {
        // private - only within this class
        private string connectionString;
        
        // internal - within this assembly
        internal void Connect() { }
        
        // protected - this class and subclasses
        protected virtual void Initialize() { }
        
        // public - accessible from anywhere
        public void Save(object data) { }
    }
}
```

#### Rust Visibility Rules | Rust 可见性规则
```rust
// Everything is private by default in Rust
mod data {
    struct Repository {  // Private struct
        connection_string: String,  // Private field
    }
    
    impl Repository {
        fn new() -> Repository {  // Private function
            Repository {
                connection_string: "localhost".to_string(),
            }
        }
        
        pub fn connect(&self) {  // Public method
            // Only accessible within this module and its children
        }
        
        pub(crate) fn initialize(&self) {  // Crate-level public
            // Accessible anywhere in this crate
        }
        
        pub(super) fn internal_method(&self) {  // Parent module public
            // Accessible in parent module
        }
    }
    
    // Public struct - accessible from outside the module
    pub struct PublicRepository {
        pub data: String,  // Public field
        private_data: String,  // Private field (no pub)
    }
}

pub use data::PublicRepository;  // Re-export for external use
```

### Module File Organization | 模块文件组织

#### C# Project Structure | C# 项目结构
```text
MyApp/
|- MyApp.csproj
|- Models/
|  |- User.cs
|  |- Product.cs
|- Services/
|  |- UserService.cs
|  |- ProductService.cs
|- Controllers/
|  |- ApiController.cs
|- Program.cs
```

#### Rust Module File Structure | Rust 模块文件结构
```text
my_app/
|- Cargo.toml
|- src/
    |- main.rs (or lib.rs)
    |- models/
    |  |- mod.rs        // Module declaration
    |  |- user.rs
    |  |- product.rs
    |- services/
    |  |- mod.rs        // Module declaration
    |  |- user_service.rs
    |  |- product_service.rs
    |- controllers/
        |- mod.rs
        |- api_controller.rs
```

#### Module Declaration Patterns | 模块声明模式
```rust
// src/models/mod.rs
pub mod user;      // Declares user.rs as a submodule
pub mod product;   // Declares product.rs as a submodule

// Re-export commonly used types
pub use user::User;
pub use product::Product;

// src/main.rs
mod models;     // Declares models/ as a module
mod services;   // Declares services/ as a module

// Import specific items
use models::{User, Product};
use services::UserService;

// Or import the entire module
use models::user::*;  // Import all public items from user module
```

***

## Crates vs .NET Assemblies | Crate 与 .NET 程序集

### Understanding Crates | 理解 Crate
In Rust, a **crate** is the fundamental unit of compilation and code distribution, similar to how an **assembly** works in .NET.

在 Rust 中，**crate** 是编译与分发代码的基本单位，作用上很接近 .NET 中的**程序集**。

#### C# Assembly Model | C# 程序集模型
```csharp
// MyLibrary.dll - Compiled assembly
namespace MyLibrary
{
    public class Calculator
    {
        public int Add(int a, int b) => a + b;
    }
}

// MyApp.exe - Executable assembly that references MyLibrary.dll
using MyLibrary;

class Program
{
    static void Main()
    {
        var calc = new Calculator();
        Console.WriteLine(calc.Add(2, 3));
    }
}
```

#### Rust Crate Model | Rust Crate 模型
```toml
# Cargo.toml for library crate
[package]
name = "my_calculator"
version = "0.1.0"
edition = "2021"

[lib]
name = "my_calculator"
```

```rust
// src/lib.rs - Library crate
pub struct Calculator;

impl Calculator {
    pub fn add(&self, a: i32, b: i32) -> i32 {
        a + b
    }
}
```

```toml
# Cargo.toml for binary crate that uses the library
[package]
name = "my_app"
version = "0.1.0"
edition = "2021"

[dependencies]
my_calculator = { path = "../my_calculator" }
```

```rust
// src/main.rs - Binary crate
use my_calculator::Calculator;

fn main() {
    let calc = Calculator;
    println!("{}", calc.add(2, 3));
}
```

### Crate Types Comparison | Crate 类型对比

| C# Concept | Rust Equivalent | Purpose |
|------------|----------------|---------|
| Class Library (.dll) | Library crate | Reusable code |
| 类库（.dll） | Library crate | 可复用代码 |
| Console App (.exe) | Binary crate | Executable program |
| 控制台程序（.exe） | Binary crate | 可执行程序 |
| NuGet Package | Published crate | Distribution unit |
| NuGet 包 | Published crate | 分发单位 |
| Assembly (.dll/.exe) | Compiled crate | Compilation unit |
| 程序集（.dll/.exe） | Compiled crate | 编译单位 |
| Solution (.sln) | Workspace | Multi-project organization |
| 解决方案（.sln） | Workspace | 多项目组织方式 |

### Workspace vs Solution | Workspace 与 Solution

#### C# Solution Structure | C# 解决方案结构
```xml
<!-- MySolution.sln structure -->
<Solution>
    <Project Include="WebApi/WebApi.csproj" />
    <Project Include="Business/Business.csproj" />
    <Project Include="DataAccess/DataAccess.csproj" />
    <Project Include="Tests/Tests.csproj" />
</Solution>
```

#### Rust Workspace Structure | Rust Workspace 结构
```toml
# Cargo.toml at workspace root
[workspace]
members = [
    "web_api",
    "business",
    "data_access",
    "tests"
]

[workspace.dependencies]
serde = "1.0"           # Shared dependency versions
tokio = "1.0"
```

```toml
# web_api/Cargo.toml
[package]
name = "web_api"
version = "0.1.0"
edition = "2021"

[dependencies]
business = { path = "../business" }
serde = { workspace = true }    # Use workspace version
tokio = { workspace = true }
```

---

## Exercises | 练习

<details>
<summary><strong>Exercise: Design a Module Tree | 练习：设计模块树</strong> (click to expand / 点击展开)</summary>

Given this C# project layout, design the equivalent Rust module tree:

给定下面这组 C# 项目布局，请设计对应的 Rust 模块树：

```csharp
// C#
namespace MyApp.Services { public class AuthService { } }
namespace MyApp.Services { internal class TokenStore { } }
namespace MyApp.Models { public class User { } }
namespace MyApp.Models { public class Session { } }
```

Requirements:
1. `AuthService` and both models must be public
2. `TokenStore` must be private to the `services` module
3. Provide the file layout **and** the `mod` / `pub` declarations in `lib.rs`

要求：
1. `AuthService` 和两个 model 都必须是公开的
2. `TokenStore` 必须仅对 `services` 模块私有
3. 同时给出文件布局，以及 `lib.rs` 中的 `mod` / `pub` 声明

<details>
<summary>Solution | 参考答案</summary>

File layout:

文件布局：

```text
src/
|- lib.rs
|- services/
|  |- mod.rs
|  |- auth_service.rs
|  |- token_store.rs
|- models/
    |- mod.rs
    |- user.rs
    |- session.rs
```

```rust,ignore
// src/lib.rs
pub mod services;
pub mod models;

// src/services/mod.rs
mod token_store;          // private - like C# internal
pub mod auth_service;     // public

// src/services/auth_service.rs
use super::token_store::TokenStore; // visible within the module

pub struct AuthService;

impl AuthService {
    pub fn login(&self) { /* uses TokenStore internally */ }
}

// src/services/token_store.rs
pub(super) struct TokenStore; // visible to parent (services) only

// src/models/mod.rs
pub mod user;
pub mod session;

// src/models/user.rs
pub struct User {
    pub name: String,
}

// src/models/session.rs
pub struct Session {
    pub user_id: u64,
}
```

</details>
</details>

***
