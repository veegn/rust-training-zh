[English Original](../en/ch08-crates-and-modules.md)

## 模块与 Crate：代码组织

> **你将学到：** Rust 的模块系统与 C# 命名空间及程序集 (Assemblies) 的对比；`pub`/`pub(crate)`/`pub(super)` 可见性控制；基于文件的模块组织方式；以及 Crate 是如何映射到 .NET 程序集的。
>
> **难度：** 🟢 初级

理解 Rust 的模块系统对于组织代码和管理依赖至关重要。对于 C# 开发者来说，这类似于理解命名空间、程序集以及 NuGet 包。

### Rust 模块 vs C# 命名空间

#### C# 命名空间组织方式
```csharp
// 文件：Models/User.cs
namespace MyApp.Models
{
    public class User
    {
        public string Name { get; set; }
        public int Age { get; set; }
    }
}

// 文件：Services/UserService.cs
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

// 文件：Program.cs
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

#### Rust 模块组织方式
```rust
// 文件：src/models.rs
pub struct User {
    pub name: String,
    pub age: u32,
}

impl User {
    pub fn new(name: String, age: u32) -> User {
        User { name, age }
    }
}

// 文件：src/services.rs
use crate::models::User;

pub struct UserService;

impl UserService {
    pub fn create_user(name: String, age: u32) -> User {
        User::new(name, age)
    }
}

// 文件：src/lib.rs (或 main.rs)
pub mod models;
pub mod services;

use models::User;
use services::UserService;

fn main() {
    let service = UserService;
    let user = UserService::create_user("Alice".to_string(), 30);
}
```

### 模块层级与可见性

```mermaid
graph TD
    Crate["crate (根节点)"] --> ModA["mod data"]
    Crate --> ModB["mod api"]
    ModA --> SubA1["pub struct Repo"]
    ModA --> SubA2["fn helper (私有)"]
    ModB --> SubB1["pub fn handle()"]
    ModB --> SubB2["pub(crate) fn internal()"]
    ModB --> SubB3["pub(super) fn parent_only()"]

    style SubA1 fill:#c8e6c9,color:#000
    style SubA2 fill:#ffcdd2,color:#000
    style SubB1 fill:#c8e6c9,color:#000
    style SubB2 fill:#fff9c4,color:#000
    style SubB3 fill:#fff9c4,color:#000
```

> 🟢 绿色 = 全局公开 &nbsp;|&nbsp; 🟡 黄色 = 受限公开 &nbsp;|&nbsp; 🔴 红色 = 私有

#### C# 可见性修饰符
```csharp
namespace MyApp.Data
{
    // public - 随处可访问
    public class Repository
    {
        // private - 仅限此类内部
        private string connectionString;
        
        // internal - 仅限此程序集内部
        internal void Connect() { }
        
        // protected - 此类及子类
        protected virtual void Initialize() { }
        
        // public - 随处可访问
        public void Save(object data) { }
    }
}
```

#### Rust 可见性规则
```rust
// 在 Rust 中，所有内容默认都是私有的
mod data {
    struct Repository {  // 私有结构体
        connection_string: String,  // 私有字段
    }
    
    impl Repository {
        fn new() -> Repository {  // 私有函数
            Repository {
                connection_string: "localhost".to_string(),
            }
        }
        
        pub fn connect(&self) {  // 公开方法
            // 仅在此模块及其子模块中可访问
        }
        
        pub(crate) fn initialize(&self) {  // Crate 级别公开
            // 在此 Crate 的任何地方均可访问
        }
        
        pub(super) fn internal_method(&self) {  // 父模块级别公开
            // 在父模块中可访问
        }
    }
    
    // 公开结构体 - 从模块外部可访问
    pub struct PublicRepository {
        pub data: String,  // 公开字段
        private_data: String,  // 私有字段 (无 pub)
    }
}

pub use data::PublicRepository;  // 重新导出 (Re-export) 供外部使用
```

### 模块的文件组织方式

#### C# 项目结构
```text
MyApp/
├── MyApp.csproj
├── Models/
│   ├── User.cs
│   └── Product.cs
├── Services/
│   ├── UserService.cs
│   └── ProductService.cs
├── Controllers/
│   └── ApiController.cs
└── Program.cs
```

#### Rust 模块文件结构
```text
my_app/
├── Cargo.toml
└── src/
    ├── main.rs (或 lib.rs)
    ├── models/
    │   ├── mod.rs        // 模块声明
    │   ├── user.rs
    │   └── product.rs
    ├── services/
    │   ├── mod.rs        // 模块声明
    │   ├── user_service.rs
    │   └── product_service.rs
    └── controllers/
        ├── mod.rs
        └── api_controller.rs
```

#### 模块声明模式
```rust
// src/models/mod.rs
pub mod user;      // 声明 user.rs 为子模块
pub mod product;   // 声明 product.rs 为子模块

// 重新导出常用类型
pub use user::User;
pub use product::Product;

// src/main.rs
mod models;     // 声明 models/ 为一个模块
mod services;   // 声明 services/ 为一个模块

// 导入特定项
use models::{User, Product};
use services::UserService;

// 或者导入整个模块
use models::user::*;  // 从 user 模块导入所有公开项
```

---

## Crate vs .NET 程序集 (Assemblies)

### 理解 Crate
在 Rust 中，**crate** 是编译和代码分发的基本单位，类似于 .NET 中的 **程序集 (assembly)**。

#### C# 程序集模型
```csharp
// MyLibrary.dll - 已编译的程序集
namespace MyLibrary
{
    public class Calculator
    {
        public int Add(int a, int b) => a + b;
    }
}

// MyApp.exe - 引用了 MyLibrary.dll 的可执行程序集
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

#### Rust Crate 模型
```toml
# 库 Crate 的 Cargo.toml
[package]
name = "my_calculator"
version = "0.1.0"
edition = "2021"

[lib]
name = "my_calculator"
```

```rust
// src/lib.rs - 库 Crate
pub struct Calculator;

impl Calculator {
    pub fn add(&self, a: i32, b: i32) -> i32 {
        a + b
    }
}
```

```toml
# 使用该库的二进制 Crate 的 Cargo.toml
[package]
name = "my_app"
version = "0.1.0"
edition = "2021"

[dependencies]
my_calculator = { path = "../my_calculator" }
```

```rust
// src/main.rs - 二进制 Crate
use my_calculator::Calculator;

fn main() {
    let calc = Calculator;
    println!("{}", calc.add(2, 3));
}
```

### Crate 类型对比

| C# 概念 | Rust 对应项 | 用途 |
|------------|----------------|---------|
| 类库 (.dll) | Library crate | 可重用的代码 |
| 控制台应用 (.exe) | Binary crate | 可执行程序 |
| NuGet 包 | Published crate | 分发单位 |
| 程序集 (.dll/.exe) | Compiled crate | 编译单位 |
| 解决方案 (.sln) | Workspace (工作区) | 多项目组织管理 |

### 工作区 vs 解决方案 (Workspace vs Solution)

#### C# 解决方案结构
```xml
<!-- MySolution.sln 结构 -->
<Solution>
    <Project Include="WebApi/WebApi.csproj" />
    <Project Include="Business/Business.csproj" />
    <Project Include="DataAccess/DataAccess.csproj" />
    <Project Include="Tests/Tests.csproj" />
</Solution>
```

#### Rust 工作区结构
```toml
# 工作区根目录下的 Cargo.toml
[workspace]
members = [
    "web_api",
    "business",
    "data_access",
    "tests"
]

[workspace.dependencies]
serde = "1.0"           # 共享依赖版本
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
serde = { workspace = true }    # 使用工作区指定的版本
tokio = { workspace = true }
```

---

## 练习

<details>
<summary><strong>🏋️ 练习：设计模块树</strong> (点击展开)</summary>

根据给出的 C# 项目布局，设计等效的 Rust 模块树：

```csharp
// C#
namespace MyApp.Services { public class AuthService { } }
namespace MyApp.Services { internal class TokenStore { } }
namespace MyApp.Models { public class User { } }
namespace MyApp.Models { public class Session { } }
```

要求：
1. `AuthService` 和两个模型必须是公开的 (public)
2. `TokenStore` 必须在 `services` 模块内部是私有的
3. 提供文件布局 **以及** 在 `lib.rs` 中的 `mod` / `pub` 声明

<details>
<summary>🔑 参考答案</summary>

文件布局：
```
src/
├── lib.rs
├── services/
│   ├── mod.rs
│   ├── auth_service.rs
│   └── token_store.rs
└── models/
    ├── mod.rs
    ├── user.rs
    └── session.rs
```

```rust,ignore
// src/lib.rs
pub mod services;
pub mod models;

// src/services/mod.rs
mod token_store;          // 私有 —— 类似于 C# 的 internal
pub mod auth_service;     // 公开

// src/services/auth_service.rs
use super::token_store::TokenStore; // 模块内可见

pub struct AuthService;

impl AuthService {
    pub fn login(&self) { /* 在内部使用 TokenStore */ }
}

// src/services/token_store.rs
pub(super) struct TokenStore; // 仅对父级 (services) 可见

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
