## Package Management: Cargo vs NuGet | 包管理：Cargo 与 NuGet

> **What you'll learn:** `Cargo.toml` vs `.csproj`, version specifiers, `Cargo.lock`,
> feature flags for conditional compilation, and common Cargo commands mapped to their NuGet/dotnet equivalents.
>
> **你将学到什么：** `Cargo.toml` 与 `.csproj` 的对应关系、版本说明符、`Cargo.lock` 的作用、
> 用于条件编译的 feature flag，以及常见 Cargo 命令与 NuGet / `dotnet` 命令之间的映射。
>
> **Difficulty:** Beginner
>
> **难度：** 初级

### Dependency Declaration | 依赖声明

#### C# NuGet Dependencies | C# 的 NuGet 依赖
```xml
<!-- MyApp.csproj -->
<Project Sdk="Microsoft.NET.Sdk">
  <PropertyGroup>
    <TargetFramework>net8.0</TargetFramework>
  </PropertyGroup>
  
  <PackageReference Include="Newtonsoft.Json" Version="13.0.3" />
  <PackageReference Include="Serilog" Version="3.0.1" />
  <PackageReference Include="Microsoft.AspNetCore.App" />
  
  <ProjectReference Include="../MyLibrary/MyLibrary.csproj" />
</Project>
```

#### Rust Cargo Dependencies | Rust 的 Cargo 依赖
```toml
# Cargo.toml
[package]
name = "my_app"
version = "0.1.0"
edition = "2021"

[dependencies]
serde_json = "1.0"               # From crates.io (like NuGet)
serde = { version = "1.0", features = ["derive"] }  # With features
log = "0.4"
tokio = { version = "1.0", features = ["full"] }

# Local dependencies (like ProjectReference)
my_library = { path = "../my_library" }

# Git dependencies
my_git_crate = { git = "https://github.com/user/repo" }

# Development dependencies (like test packages)
[dev-dependencies]
criterion = "0.5"               # Benchmarking
proptest = "1.0"               # Property testing
```

### Version Management | 版本管理

#### C# Package Versioning | C# 包版本控制
```xml
<!-- Centralized package management (Directory.Packages.props) -->
<Project>
  <PropertyGroup>
    <ManagePackageVersionsCentrally>true</ManagePackageVersionsCentrally>
  </PropertyGroup>
  
  <PackageVersion Include="Newtonsoft.Json" Version="13.0.3" />
  <PackageVersion Include="Serilog" Version="3.0.1" />
</Project>

<!-- packages.lock.json for reproducible builds -->
```

#### Rust Version Management | Rust 版本管理
```toml
# Cargo.toml - Semantic versioning
[dependencies]
serde = "1.0"        # Compatible with 1.x.x (>=1.0.0, <2.0.0)
log = "0.4.17"       # Compatible with 0.4.x (>=0.4.17, <0.5.0)
regex = "=1.5.4"     # Exact version
chrono = "^0.4"      # Caret requirements (default)
uuid = "~1.3.0"      # Tilde requirements (>=1.3.0, <1.4.0)

# Cargo.lock - Exact versions for reproducible builds (auto-generated)
[[package]]
name = "serde"
version = "1.0.163"
# ... exact dependency tree
```

### Package Sources | 包源配置

#### C# Package Sources | C# 包源
```xml
<!-- nuget.config -->
<configuration>
  <packageSources>
    <add key="nuget.org" value="https://api.nuget.org/v3/index.json" />
    <add key="MyCompanyFeed" value="https://pkgs.dev.azure.com/company/_packaging/feed/nuget/v3/index.json" />
  </packageSources>
</configuration>
```

#### Rust Package Sources | Rust 包源
```toml
# .cargo/config.toml
[source.crates-io]
replace-with = "my-awesome-registry"

[source.my-awesome-registry]
registry = "https://my-intranet:8080/index"

# Alternative registries
[registries]
my-registry = { index = "https://my-intranet:8080/index" }

# In Cargo.toml
[dependencies]
my_crate = { version = "1.0", registry = "my-registry" }
```

### Common Commands Comparison | 常用命令对照

| Task | C# Command | Rust Command |
|------|------------|-------------|
| Restore packages | `dotnet restore` | `cargo fetch` |
| 还原依赖包 | `dotnet restore` | `cargo fetch` |
| Add package | `dotnet add package Newtonsoft.Json` | `cargo add serde_json` |
| 添加依赖包 | `dotnet add package Newtonsoft.Json` | `cargo add serde_json` |
| Remove package | `dotnet remove package Newtonsoft.Json` | `cargo remove serde_json` |
| 删除依赖包 | `dotnet remove package Newtonsoft.Json` | `cargo remove serde_json` |
| Update packages | `dotnet update` | `cargo update` |
| 更新依赖包 | `dotnet update` | `cargo update` |
| List packages | `dotnet list package` | `cargo tree` |
| 查看依赖包 | `dotnet list package` | `cargo tree` |
| Audit security | `dotnet list package --vulnerable` | `cargo audit` |
| 安全审计 | `dotnet list package --vulnerable` | `cargo audit` |
| Clean build | `dotnet clean` | `cargo clean` |
| 清理构建产物 | `dotnet clean` | `cargo clean` |

### Features: Conditional Compilation | Features：条件编译

#### C# Conditional Compilation | C# 条件编译
```csharp
#if DEBUG
    Console.WriteLine("Debug mode");
#elif RELEASE
    Console.WriteLine("Release mode");
#endif

// Project file features
<PropertyGroup Condition="'$(Configuration)'=='Debug'">
    <DefineConstants>DEBUG;TRACE</DefineConstants>
</PropertyGroup>
```

#### Rust Feature Gates | Rust Feature 开关
```toml
# Cargo.toml
[features]
default = ["json"]              # Default features
json = ["serde_json"]          # Feature that enables serde_json
xml = ["serde_xml"]            # Alternative serialization
advanced = ["json", "xml"]     # Composite feature

[dependencies]
serde_json = { version = "1.0", optional = true }
serde_xml = { version = "0.4", optional = true }
```

```rust
// Conditional compilation based on features
#[cfg(feature = "json")]
use serde_json;

#[cfg(feature = "xml")]
use serde_xml;

pub fn serialize_data(data: &MyStruct) -> String {
    #[cfg(feature = "json")]
    return serde_json::to_string(data).unwrap();
    
    #[cfg(feature = "xml")]
    return serde_xml::to_string(data).unwrap();
    
    #[cfg(not(any(feature = "json", feature = "xml")))]
    return "No serialization feature enabled".to_string();
}
```

### Using External Crates | 使用第三方 Crate

#### Popular Crates for C# Developers | 适合 C# 开发者的常见 Crate

| C# Library | Rust Crate | Purpose |
|------------|------------|---------|
| Newtonsoft.Json | `serde_json` | JSON serialization |
| Newtonsoft.Json | `serde_json` | JSON 序列化 |
| HttpClient | `reqwest` | HTTP client |
| HttpClient | `reqwest` | HTTP 客户端 |
| Entity Framework | `diesel` / `sqlx` | ORM / SQL toolkit |
| Entity Framework | `diesel` / `sqlx` | ORM / SQL 工具集 |
| NLog/Serilog | `log` + `env_logger` | Logging |
| NLog/Serilog | `log` + `env_logger` | 日志 |
| xUnit/NUnit | Built-in `#[test]` | Unit testing |
| xUnit/NUnit | 内置 `#[test]` | 单元测试 |
| Moq | `mockall` | Mocking |
| Moq | `mockall` | Mock |
| Flurl | `url` | URL manipulation |
| Flurl | `url` | URL 处理 |
| Polly | `tower` | Resilience patterns |
| Polly | `tower` | 弹性/容错模式 |

#### Example: HTTP Client Migration | 示例：HTTP 客户端迁移
```csharp
// C# HttpClient usage
public class ApiClient
{
    private readonly HttpClient _httpClient;
    
    public async Task<User> GetUserAsync(int id)
    {
        var response = await _httpClient.GetAsync($"/users/{id}");
        var json = await response.Content.ReadAsStringAsync();
        return JsonConvert.DeserializeObject<User>(json);
    }
}
```

```rust
// Rust reqwest usage
use reqwest;
use serde::Deserialize;

#[derive(Deserialize)]
struct User {
    id: u32,
    name: String,
}

struct ApiClient {
    client: reqwest::Client,
}

impl ApiClient {
    async fn get_user(&self, id: u32) -> Result<User, reqwest::Error> {
        let user = self.client
            .get(&format!("https://api.example.com/users/{}", id))
            .send()
            .await?
            .json::<User>()
            .await?;
        
        Ok(user)
    }
}
```

***
