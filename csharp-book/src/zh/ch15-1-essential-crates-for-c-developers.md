[English Original](../en/ch15-1-essential-crates-for-c-developers.md)

## C# 开发者的核心 Crate 指南

> **你将学到：** 常用 .NET 库对应的 Rust Crate —— serde (JSON.NET), reqwest (HttpClient), tokio (Task/async), sqlx (Entity Framework)；以及对 serde 属性系统与 `System.Text.Json` 的深度对比。
>
> **难度：** 🟡 中级

### 核心功能对应关系

```rust
// C# 开发者常用的 Cargo.toml 依赖项
[dependencies]
# 序列化 (类似于 Newtonsoft.Json 或 System.Text.Json)
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

# HTTP 客户端 (类似于 HttpClient)
reqwest = { version = "0.11", features = ["json"] }

# 异步运行时 (类似于 Task.Run, async/await)
tokio = { version = "1.0", features = ["full"] }

# 错误处理 (类似于自定义异常)
thiserror = "1.0"
anyhow = "1.0"

# 日志记录 (类似于 ILogger, Serilog)
log = "0.4"
env_logger = "0.10"

# 日期/时间 (类似于 DateTime)
chrono = { version = "0.4", features = ["serde"] }

# UUID (类似于 System.Guid)
uuid = { version = "1.0", features = ["v4", "serde"] }

# 集合 (类似于 List<T>, Dictionary<K,V>)
# 标准库已内置，高级集合可使用：
indexmap = "2.0"  # 有序 HashMap

# 配置管理 (类似于 IConfiguration)
config = "0.13"

# 数据库操作 (类似于 Entity Framework)
sqlx = { version = "0.7", features = ["runtime-tokio-rustls", "postgres", "uuid", "chrono"] }

# 测试 (类似于 xUnit, NUnit)
# 标准库已内置，更多特性可使用：
rstest = "0.18"  # 参数化测试

# Mock (类似于 Moq)
mockall = "0.11"

# 并行处理 (类似于 Parallel.ForEach)
rayon = "1.7"
```

### 典型使用模式示例

```rust
use serde::{Deserialize, Serialize};
use reqwest;
use tokio;
use thiserror::Error;
use chrono::{DateTime, Utc};
use uuid::Uuid;

// 数据模型 (类似于带有属性标签的 C# POCO)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub created_at: DateTime<Utc>,
}

// 自定义错误类型 (类似于自定义异常)
#[derive(Error, Debug)]
pub enum ApiError {
    #[error("HTTP 请求失败: {0}")]
    Http(#[from] reqwest::Error),
    
    #[error("序列化失败: {0}")]
    Serialization(#[from] serde_json::Error),
    
    #[error("未找到用户: {id}")]
    UserNotFound { id: Uuid },
    
    #[error("验证失败: {message}")]
    Validation { message: String },
}

// 业务类等效项 (Service class)
pub struct UserService {
    client: reqwest::Client,
    base_url: String,
}

impl UserService {
    pub fn new(base_url: String) -> Self {
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .expect("创建 HTTP 客户端失败");
            
        UserService { client, base_url }
    }
    
    // 异步方法 (类似于 C# 的 async Task<User>)
    pub async fn get_user(&self, id: Uuid) -> Result<User, ApiError> {
        let url = format!("{}/users/{}", self.base_url, id);
        
        let response = self.client
            .get(&url)
            .send()
            .await?;
        
        if response.status() == 404 {
            return Err(ApiError::UserNotFound { id });
        }
        
        let user = response.json::<User>().await?;
        Ok(user)
    }
    
    // 创建用户
    pub async fn create_user(&self, name: String, email: String) -> Result<User, ApiError> {
        if name.trim().is_empty() {
            return Err(ApiError::Validation {
                message: "名称不能为空".to_string(),
            });
        }
        
        let new_user = User {
            id: Uuid::new_v4(),
            name,
            email,
            created_at: Utc::now(),
        };
        
        let response = self.client
            .post(&format!("{}/users", self.base_url))
            .json(&new_user)
            .send()
            .await?;
        
        let created_user = response.json::<User>().await?;
        Ok(created_user)
    }
}

// 用法示例 (类似于 C# 的 Main 方法)
#[tokio::main]
async fn main() -> Result<(), ApiError> {
    // 初始化日志记录 (类似于配置 ILogger)
    env_logger::init();
    
    let service = UserService::new("https://api.example.com".to_string());
    
    // 创建用户
    let user = service.create_user(
        "张三".to_string(),
        "zhangsan@example.com".to_string(),
    ).await?;
    
    println!("已创建用户: {:?}", user);
    
    // 获取用户
    let retrieved_user = service.get_user(user.id).await?;
    println!("已获取用户: {:?}", retrieved_user);
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]  // 类似于 C# 的 [Test] 或 [Fact]
    async fn test_user_creation() {
        let service = UserService::new("http://localhost:8080".to_string());
        
        let result = service.create_user(
            "测试用户".to_string(),
            "test@example.com".to_string(),
        ).await;
        
        assert!(result.is_ok());
        let user = result.unwrap();
        assert_eq!(user.name, "测试用户");
        assert_eq!(user.email, "test@example.com");
    }
    
    #[test]
    fn test_validation() {
        // 同步测试
        let error = ApiError::Validation {
            message: "输入无效".to_string(),
        };
        
        assert_eq!(error.to_string(), "验证失败: 输入无效");
    }
}
```

---

## Serde 深度探索：面向 C# 开发者的 JSON 序列化手册

C# 开发者高度依赖 `System.Text.Json` 或 `Newtonsoft.Json`。而在 Rust 中，**serde** (serialize/deserialize) 是通用的序列化框架 —— 掌握它的属性系统即可应对绝大多数数据处理场景。

### 基础派生：起点
```rust
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct User {
    name: String,
    age: u32,
    email: String,
}

let user = User { name: "Alice".into(), age: 30, email: "alice@co.com".into() };
let json = serde_json::to_string_pretty(&user)?;
let parsed: User = serde_json::from_str(&json)?;
```

```csharp
// C# 对比
public class User
{
    public string Name { get; set; }
    public int Age { get; set; }
    public string Email { get; set; }
}
var json = JsonSerializer.Serialize(user, new JsonSerializerOptions { WriteIndented = true });
var parsed = JsonSerializer.Deserialize<User>(json);
```

### 字段级属性 (类似于 `[JsonProperty]`)

```rust
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct ApiResponse {
    // 重命名 JSON 输出中的字段 (类似于 [JsonPropertyName("user_id")])
    #[serde(rename = "user_id")]
    id: u64,

    // 为序列化和反序列化使用不同的名称
    #[serde(rename(serialize = "userName", deserialize = "user_name"))]
    name: String,

    // 完全忽略此字段 (类似于 [JsonIgnore])
    #[serde(skip)]
    internal_cache: Option<String>,

    // 仅在序列化时跳过
    #[serde(skip_serializing)]
    password_hash: String,

    // 如果 JSON 中缺失则使用默认值 (类似于无参构造函数赋初值)
    #[serde(default)]
    is_active: bool,

    // 使用自定义默认值
    #[serde(default = "default_role")]
    role: String,

    // 将嵌套结构体的内容扁平化到父级中 (类似于 [JsonExtensionData])
    #[serde(flatten)]
    metadata: Metadata,

    // 如果数值为 None 则跳过 (即不序列化 null 字段)
    #[serde(skip_serializing_if = "Option::is_none")]
    nickname: Option<String>,
}

fn default_role() -> String { "viewer".into() }

#[derive(Serialize, Deserialize, Debug)]
struct Metadata {
    created_at: String,
    version: u32,
}
```

```csharp
// C# 的等效属性标签
public class ApiResponse
{
    [JsonPropertyName("user_id")]
    public ulong Id { get; set; }

    [JsonIgnore]
    public string? InternalCache { get; set; }

    [JsonExtensionData]
    public Dictionary<string, JsonElement>? Metadata { get; set; }
}
```

### 枚举的表现形式 (与 C# 的关键区别)

Rust 的 serde 支持**四种不同的 JSON 枚举表现形式** —— 这是一个 C# 中不存在的概念，因为 C# 的枚举始终是整数或字符串。

```rust
use serde::{Deserialize, Serialize};

// 1. 外部标签 (默认形式) —— 最常用
#[derive(Serialize, Deserialize)]
enum Message {
    Text(String),
    Image { url: String, width: u32 },
    Ping,
}
// Text 变体：  {"Text": "hello"}
// Image 变体： {"Image": {"url": "...", "width": 100}}
// Ping 变体：  "Ping"

// 2. 内部标签 —— 类似于其他语言中的可辨识联合 (Discriminated unions)
#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
enum Event {
    Created { id: u64, name: String },
    Deleted { id: u64 },
    Updated { id: u64, fields: Vec<String> },
}
// {"type": "Created", "id": 1, "name": "Alice"}
// {"type": "Deleted", "id": 1}

// 3. 相邻标签 —— 标签和内容位于不同的字段中
#[derive(Serialize, Deserialize)]
#[serde(tag = "t", content = "c")]
enum ApiResult {
    Success(UserData),
    Error(String),
}
// {"t": "Success", "c": {"name": "Alice"}}
// {"t": "Error", "c": "not found"}

// 4. 无标签 —— serde 会按顺序尝试匹配每一个变体
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
enum FlexibleValue {
    Integer(i64),
    Float(f64),
    Text(String),
    Bool(bool),
}
// 42, 3.14, "hello", true —— serde 会自动检测对应的类型
```

### 自定义序列化 (类似于 `JsonConverter`)
```rust
use serde::{Deserialize, Deserializer, Serialize, Serializer};

// 为特定字段自定义序列化逻辑
#[derive(Serialize, Deserialize)]
struct Config {
    #[serde(serialize_with = "serialize_duration", deserialize_with = "deserialize_duration")]
    timeout: std::time::Duration,
}

fn serialize_duration<S: Serializer>(dur: &std::time::Duration, s: S) -> Result<S::Ok, S::Error> {
    s.serialize_u64(dur.as_millis() as u64)
}

fn deserialize_duration<'de, D: Deserializer<'de>>(d: D) -> Result<std::time::Duration, D::Error> {
    let ms = u64::deserialize(d)?;
    Ok(std::time::Duration::from_millis(ms))
}
// 映射结果：JSON {"timeout": 5000}  ↔  Rust Config { timeout: Duration::from_millis(5000) }
```

### 容器级属性

```rust
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]  // 将所有字段在 JSON 中转换为小驼峰命名
struct UserProfile {
    first_name: String,      // → "firstName"
    last_name: String,       // → "lastName"
    email_address: String,   // → "emailAddress"
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]  // 拒绝带有额外字段的 JSON (严格解析)
struct StrictConfig {
    port: u16,
    host: String,
}
// 如果输入 r#"{"port":8080,"host":"localhost","extra":true}"#
// 会报错：unknown field `extra`
```

### 快速参考：Serde 属性大全

| 属性标签 | 应用层级 | C# 对应项 | 用途 |
|-----------|-------|---------------|---------|
| `#[serde(rename = "...")]` | 字段 | `[JsonPropertyName]` | JSON 中的重命名 |
| `#[serde(skip)]` | 字段 | `[JsonIgnore]` | 完全忽略 |
| `#[serde(default)]` | 字段 | 默认构造值 | 缺失则使用 `Default::default()` |
| `#[serde(flatten)]` | 字段 | `[JsonExtensionData]` | 展开嵌套结构体到父级 |
| `#[serde(skip_serializing_if = "...")]` | 字段 | `JsonIgnoreCondition` | 基于条件的忽略 |
| `#[serde(rename_all = "camelCase")]` | 容器 | `PropertyNamingPolicy` | 命名风格转换约定 |
| `#[serde(deny_unknown_fields)]` | 容器 | — | 严格的反序列化模式 |
| `#[serde(tag = "type")]` | 枚举 | 鉴别器 (Discriminator) | 内部打标签模式 |
| `#[serde(untagged)]` | 枚举 | — | 按顺序尝试匹配各个变体 |
| `#[serde(with = "...")]` | 字段 | `[JsonConverter]` | 自定义序列化/反序列化 |

### 超越 JSON：Serde 同样支持其他格式
```rust
// 同一套派生标签可以支持“所有”格式 —— 只需更改使用的库
let user = User { name: "Alice".into(), age: 30, email: "a@b.com".into() };

let json  = serde_json::to_string(&user)?;        // JSON
let toml  = toml::to_string(&user)?;               // TOML (配置文件)
let yaml  = serde_yaml::to_string(&user)?;          // YAML
let cbor  = serde_cbor::to_vec(&user)?;             // CBOR (二进制、极致紧凑)
let msgpk = rmp_serde::to_vec(&user)?;              // MessagePack (二进制)

// 只需一个 #[derive(Serialize, Deserialize)] —— 免费支持各种流行格式
```

---
