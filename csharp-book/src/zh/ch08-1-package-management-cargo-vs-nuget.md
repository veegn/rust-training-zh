[English Original](../en/ch08-1-package-management-cargo-vs-nuget.md)

## 包管理：Cargo vs NuGet

> **你将学到：** `Cargo.toml` 与 `.csproj` 的对比；版本指定方式；`Cargo.lock` 的作用；用于条件编译的特性标志 (Feature flags)；以及常用 Cargo 命令与其 NuGet/dotnet 等效命令的映射。
>
> **难度：** 🟢 初级

### 依赖声明

#### C# NuGet 依赖
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

#### Rust Cargo 依赖
```toml
# Cargo.toml
[package]
name = "my_app"
version = "0.1.0"
edition = "2021"

[dependencies]
serde_json = "1.0"               # 来自 crates.io (类似于 NuGet)
serde = { version = "1.0", features = ["derive"] }  # 带有特性 (features) 开关
log = "0.4"
tokio = { version = "1.0", features = ["full"] }

# 本地依赖 (类似于 ProjectReference)
my_library = { path = "../my_library" }

# Git 依赖
my_git_crate = { git = "https://github.com/user/repo" }

# 开发依赖 (类似于测试相关的包)
[dev-dependencies]
criterion = "0.5"               # 基准测试工具
proptest = "1.0"               # 属性测试工具
```

### 版本管理

#### C# 包版本管理
```xml
<!-- 中心化包管理 (Directory.Packages.props) -->
<Project>
  <PropertyGroup>
    <ManagePackageVersionsCentrally>true</ManagePackageVersionsCentrally>
  </PropertyGroup>
  
  <PackageVersion Include="Newtonsoft.Json" Version="13.0.3" />
  <PackageVersion Include="Serilog" Version="3.0.1" />
</Project>

<!-- 用于可复现构建的 packages.lock.json -->
```

#### Rust 版本管理
```toml
# Cargo.toml - 语义化版本控制 (SemVer)
[dependencies]
serde = "1.0"        # 兼容 1.x.x (>=1.0.0, <2.0.0)
log = "0.4.17"       # 兼容 0.4.x (>=0.4.17, <0.5.0)
regex = "=1.5.4"     # 指定精确版本
chrono = "^0.4"      # 脱字符要求 (默认行为)
uuid = "~1.3.0"      # 波浪号要求 (>=1.3.0, <1.4.0)

# Cargo.lock - 用于可复现构建的精确版本信息 (自动生成)
[[package]]
name = "serde"
version = "1.0.163"
# ... 完整的精确依赖树
```

### 包源码 (Package Sources)

#### C# 包源码配置
```xml
<!-- nuget.config -->
<configuration>
  <packageSources>
    <add key="nuget.org" value="https://api.nuget.org/v3/index.json" />
    <add key="MyCompanyFeed" value="https://pkgs.dev.azure.com/company/_packaging/feed/nuget/v3/index.json" />
  </packageSources>
</configuration>
```

#### Rust 包源码配置
```toml
# .cargo/config.toml
[source.crates-io]
replace-with = "my-awesome-registry"

[source.my-awesome-registry]
registry = "https://my-intranet:8080/index"

# 备用镜像/仓库
[registries]
my-registry = { index = "https://my-intranet:8080/index" }

# 在 Cargo.toml 中使用
[dependencies]
my_crate = { version = "1.0", registry = "my-registry" }
```

### 常用命令对比

| 任务 | C# 命令 | Rust 命令 |
|------|------------|-------------|
| 还原依赖包 | `dotnet restore` | `cargo fetch` |
| 添加依赖包 | `dotnet add package Newtonsoft.Json` | `cargo add serde_json` |
| 移除依赖包 | `dotnet remove package Newtonsoft.Json` | `cargo remove serde_json` |
| 更新依赖包 | `dotnet update` | `cargo update` |
| 列出依赖树 | `dotnet list package` | `cargo tree` |
| 安全审计 | `dotnet list package --vulnerable` | `cargo audit` |
| 清理构建产物 | `dotnet clean` | `cargo clean` |

### 特性 (Features)：条件编译

#### C# 条件编译
```csharp
#if DEBUG
    Console.WriteLine("Debug mode");
#elif RELEASE
    Console.WriteLine("Release mode");
#endif

// 项目文件中的特性定义
<PropertyGroup Condition="'$(Configuration)'=='Debug'">
    <DefineConstants>DEBUG;TRACE</DefineConstants>
</PropertyGroup>
```

#### Rust 特性门控 (Feature Gates)
```toml
# Cargo.toml
[features]
default = ["json"]              # 默认启用的特性
json = ["serde_json"]          # 启用此特性时会带上 serde_json 依赖
xml = ["serde_xml"]            # 另一种序列化方式
advanced = ["json", "xml"]     # 组合特性

[dependencies]
serde_json = { version = "1.0", optional = true }
serde_xml = { version = "0.4", optional = true }
```

```rust
// 基于特性进行条件编译
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
    return "没有启用序列化特性".to_string();
}
```

### 使用外部 Crate

#### 面向 C# 开发者的常用 Crate 映射

| C# 类库 | Rust Crate | 用途 |
|------------|------------|---------|
| System.Text.Json / Newtonsoft.Json | `serde_json` | JSON 序列化 |
| HttpClient | `reqwest` | HTTP 客户端 |
| Entity Framework | `diesel` / `sqlx` | ORM / SQL 工具包 |
| NLog/Serilog | `log` + `env_logger` | 日志记录 |
| xUnit/NUnit | 内置的 `#[test]` | 单元测试 |
| Moq | `mockall` | Mock 测试 |
| Flurl | `url` | URL 操作 |
| Polly | `tower` | 弹性/重试模式 |

#### 示例：HTTP 客户端迁移
```csharp
// C# HttpClient 用法
public class ApiClient
{
    private readonly HttpClient _httpClient;
    
    public async Task<User> GetUserAsync(int id)
    {
        var response = await _httpClient.GetAsync($"/users/{id}");
        var json = await response.Content.ReadAsStringAsync();
        return System.Text.Json.JsonSerializer.Deserialize<User>(json);
    }
}
```

```rust
// Rust reqwest 用法
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

---
