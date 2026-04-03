[English Original](../en/ch05-1-constructor-patterns.md)

## 构造函数模式

> **你将学到：** 如何在没有传统构造函数的情况下创建 Rust 结构体 —— `new()` 约定、`Default` 特性、工厂方法以及用于复杂初始化的生成器模式 (Builder pattern)。
>
> **难度：** 🟢 初级

### C# 构造函数模式
```csharp
public class Configuration
{
    public string DatabaseUrl { get; set; }
    public int MaxConnections { get; set; }
    public bool EnableLogging { get; set; }
    
    // 默认构造函数
    public Configuration()
    {
        DatabaseUrl = "localhost";
        MaxConnections = 10;
        EnableLogging = false;
    }
    
    // 参数化构造函数
    public Configuration(string databaseUrl, int maxConnections)
    {
        DatabaseUrl = databaseUrl;
        MaxConnections = maxConnections;
        EnableLogging = false;
    }
    
    // 工厂方法
    public static Configuration ForProduction()
    {
        return new Configuration("prod.db.server", 100)
        {
            EnableLogging = true
        };
    }
}
```

### Rust 构造函数模式
```rust
#[derive(Debug)]
pub struct Configuration {
    pub database_url: String,
    pub max_connections: u32,
    pub enable_logging: bool,
}

impl Configuration {
    // 惯用的“默认”构造函数 (new() 约定)
    pub fn new() -> Configuration {
        Configuration {
            database_url: "localhost".to_string(),
            max_connections: 10,
            enable_logging: false,
        }
    }
    
    // 参数化构造函数
    pub fn with_database(database_url: String, max_connections: u32) -> Configuration {
        Configuration {
            database_url,
            max_connections,
            enable_logging: false,
        }
    }
    
    // 工厂方法
    pub fn for_production() -> Configuration {
        Configuration {
            database_url: "prod.db.server".to_string(),
            max_connections: 100,
            enable_logging: true,
        }
    }
    
    // 简单的生成器模式方法 (链式调用)
    pub fn enable_logging(mut self) -> Configuration {
        self.enable_logging = true;
        self  // 返回 self 以支持链式调用
    }
    
    pub fn max_connections(mut self, count: u32) -> Configuration {
        self.max_connections = count;
        self
    }
}

// 实现 Default 特性 (类似 C# 的无参构造函数)
impl Default for Configuration {
    fn default() -> Self {
        Self::new()
    }
}

fn main() {
    // 不同的编写模式
    let config1 = Configuration::new();
    let config2 = Configuration::with_database("localhost:5432".to_string(), 20);
    let config3 = Configuration::for_production();
    
    // 生成器模式调用
    let config4 = Configuration::new()
        .enable_logging()
        .max_connections(50);
    
    // 使用 Default 特性
    let config5 = Configuration::default();
    
    println!("{:?}", config4);
}
```

### 生成器模式 (Builder Pattern) 的深入实现
```rust
// 针对更加复杂的配置，常规做法是创建一个专门的 Builder 结构体
#[derive(Debug)]
pub struct DatabaseConfig {
    host: String,
    port: u16,
    username: String,
    password: Option<String>,
    ssl_enabled: bool,
    timeout_seconds: u64,
}

pub struct DatabaseConfigBuilder {
    host: Option<String>,
    port: Option<u16>,
    username: Option<String>,
    password: Option<String>,
    ssl_enabled: bool,
    timeout_seconds: u64,
}

impl DatabaseConfigBuilder {
    pub fn new() -> Self {
        DatabaseConfigBuilder {
            host: None,
            port: None,
            username: None,
            password: None,
            ssl_enabled: false,
            timeout_seconds: 30,
        }
    }
    
    pub fn host(mut self, host: impl Into<String>) -> Self {
        self.host = Some(host.into());
        self
    }
    
    pub fn port(mut self, port: u16) -> Self {
        self.port = Some(port);
        self
    }
    
    pub fn username(mut self, username: impl Into<String>) -> Self {
        self.username = Some(username.into());
        self
    }
    
    pub fn password(mut self, password: impl Into<String>) -> Self {
        self.password = Some(password.into());
        self
    }
    
    pub fn enable_ssl(mut self) -> Self {
        self.ssl_enabled = true;
        self
    }
    
    pub fn timeout(mut self, seconds: u64) -> Self {
        self.timeout_seconds = seconds;
        self
    }
    
    // 最后执行 build() 进行构建并验证
    pub fn build(self) -> Result<DatabaseConfig, String> {
        let host = self.host.ok_or("Host 是必填项")?;
        let port = self.port.ok_or("Port 是必填项")?;
        let username = self.username.ok_or("Username 是必填项")?;
        
        Ok(DatabaseConfig {
            host,
            port,
            username,
            password: self.password,
            ssl_enabled: self.ssl_enabled,
            timeout_seconds: self.timeout_seconds,
        })
    }
}

fn main() {
    let config = DatabaseConfigBuilder::new()
        .host("localhost")
        .port(5432)
        .username("admin")
        .password("secret123")
        .enable_ssl()
        .timeout(60)
        .build()
        .expect("无法构建配置");
    
    println!("{:?}", config);
}
```

---

## 练习

<details>
<summary><strong>🏋️ 练习：带有验证功能的生成器</strong> (点击展开)</summary>

创建一个 `EmailBuilder`，要求：
1. `to`（收件人）和 `subject`（主题）是必填项（若缺失则 `build()` 失败）。
2. 提供可选的 `body`（正文）和 `cc`（抄送名单，使用 `Vec` 存储地址）。
3. `build()` 返回 `Result<Email, String>` —— 如果 `to` 或 `subject` 为空，则拒绝创建。
4. 编写测试用例，证明当输入缺失时会被拒绝。

<details>
<summary>🔑 参考答案</summary>

```rust
#[derive(Debug)]
struct Email {
    to: String,
    subject: String,
    body: Option<String>,
    cc: Vec<String>,
}

#[derive(Default)]
struct EmailBuilder {
    to: Option<String>,
    subject: Option<String>,
    body: Option<String>,
    cc: Vec<String>,
}

impl EmailBuilder {
    fn new() -> Self { Self::default() }

    fn to(mut self, to: impl Into<String>) -> Self {
        self.to = Some(to.into()); self
    }
    fn subject(mut self, subject: impl Into<String>) -> Self {
        self.subject = Some(subject.into()); self
    }
    fn body(mut self, body: impl Into<String>) -> Self {
        self.body = Some(body.into()); self
    }
    fn cc(mut self, addr: impl Into<String>) -> Self {
        self.cc.push(addr.into()); self
    }
    fn build(self) -> Result<Email, String> {
        let to = self.to.filter(|s| !s.is_empty())
            .ok_or("'to' 是必填项")?;
        let subject = self.subject.filter(|s| !s.is_empty())
            .ok_or("'subject' 是必填项")?;
        Ok(Email { to, subject, body: self.body, cc: self.cc })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn valid_email() {
        let email = EmailBuilder::new()
            .to("alice@example.com")
            .subject("Hello")
            .build();
        assert!(email.is_ok());
    }
    #[test]
    fn missing_to_fails() {
        let email = EmailBuilder::new().subject("Hello").build();
        assert!(email.is_err());
    }
}
```

</details>
</details>

***
