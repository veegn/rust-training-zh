## Capstone Project: Build a CLI Weather Tool | 综合项目：构建一个命令行天气工具

> **What you'll learn:** How to combine everything - structs, traits, error handling, async, modules,
> serde, and CLI argument parsing - into a working Rust application. This mirrors the kind of tool
> a C# developer would build with `HttpClient`, `System.Text.Json`, and `System.CommandLine`.
>
> **你将学到什么：** 如何把 struct、trait、错误处理、async、模块、
> serde 和命令行参数解析组合起来，做成一个真正可运行的 Rust 应用。
> 这基本对应于 C# 开发者会用 `HttpClient`、`System.Text.Json` 和 `System.CommandLine` 构建的那类工具。
>
> **Difficulty:** Intermediate
>
> **难度：** 中级

This capstone pulls together concepts from every part of the book. You'll build `weather-cli`, a command-line tool that fetches weather data from an API and displays it. The project is structured as a mini-crate with proper module layout, error types, and tests.

这个综合项目会把本书前面几乎所有重要概念串联起来。你将构建一个名为 `weather-cli` 的命令行工具，它会从天气 API 拉取数据并展示结果。整个项目会按照一个小型 crate 的方式组织，包含清晰的模块布局、错误类型和测试。

### Project Overview | 项目概览

```mermaid
graph TD
    CLI["main.rs\nclap CLI parser"] --> Client["client.rs\nreqwest + tokio"]
    Client -->|"HTTP GET"| API["Weather API"]
    Client -->|"JSON -> struct"| Model["weather.rs\nserde Deserialize"]
    Model --> Display["display.rs\nfmt::Display"]
    CLI --> Err["error.rs\nthiserror"]
    Client --> Err

    style CLI fill:#bbdefb,color:#000
    style Err fill:#ffcdd2,color:#000
    style Model fill:#c8e6c9,color:#000
```

**What you'll build:**

**你将做出的效果：**

```text
$ weather-cli --city "Seattle"
Seattle: 12 degC, Overcast clouds
    Humidity: 82%  Wind: 5.4 m/s
```

**Concepts exercised:**

**会练到的知识点：**

| Book Chapter | Concept Used Here |
|---|---|
| Ch05 (Structs) | `WeatherReport`, `Config` data types |
| Ch05（Struct） | `WeatherReport`、`Config` 等数据类型 |
| Ch08 (Modules) | `src/lib.rs`, `src/client.rs`, `src/display.rs` |
| Ch08（模块） | `src/lib.rs`、`src/client.rs`、`src/display.rs` |
| Ch09 (Errors) | Custom `WeatherError` with `thiserror` |
| Ch09（错误处理） | 基于 `thiserror` 的自定义 `WeatherError` |
| Ch10 (Traits) | `Display` impl for formatted output |
| Ch10（Trait） | 为格式化输出实现 `Display` |
| Ch11 (From/Into) | JSON deserialization via `serde` |
| Ch11（From/Into） | 借助 `serde` 做 JSON 反序列化 |
| Ch12 (Iterators) | Processing API response arrays |
| Ch12（迭代器） | 处理 API 返回的数组数据 |
| Ch13 (Async) | `reqwest` + `tokio` for HTTP calls |
| Ch13（异步） | 使用 `reqwest` + `tokio` 发起 HTTP 调用 |
| Ch14-1 (Testing) | Unit tests + integration test |
| Ch14-1（测试） | 单元测试 + 集成测试 |

---

### Step 1: Project Setup | 第 1 步：项目初始化

```bash
cargo new weather-cli
cd weather-cli
```

Add dependencies to `Cargo.toml`:

把依赖加入 `Cargo.toml`：

```toml
[package]
name = "weather-cli"
version = "0.1.0"
edition = "2021"

[dependencies]
clap = { version = "4", features = ["derive"] }   # CLI args (like System.CommandLine)
reqwest = { version = "0.12", features = ["json"] } # HTTP client (like HttpClient)
serde = { version = "1", features = ["derive"] }    # Serialization (like System.Text.Json)
serde_json = "1"
thiserror = "2"                                      # Error types
tokio = { version = "1", features = ["full"] }       # Async runtime
```

```csharp
// C# equivalent dependencies:
// dotnet add package System.CommandLine
// dotnet add package System.Net.Http.Json
// (System.Text.Json and HttpClient are built-in)
```

### Step 2: Define Your Data Types | 第 2 步：定义数据类型

Create `src/weather.rs`:

创建 `src/weather.rs`：

```rust
use serde::Deserialize;

/// Raw API response (matches JSON shape)
#[derive(Deserialize, Debug)]
pub struct ApiResponse {
    pub main: MainData,
    pub weather: Vec<WeatherCondition>,
    pub wind: WindData,
    pub name: String,
}

#[derive(Deserialize, Debug)]
pub struct MainData {
    pub temp: f64,
    pub humidity: u32,
}

#[derive(Deserialize, Debug)]
pub struct WeatherCondition {
    pub description: String,
    pub icon: String,
}

#[derive(Deserialize, Debug)]
pub struct WindData {
    pub speed: f64,
}

/// Our domain type (clean, decoupled from API)
#[derive(Debug, Clone)]
pub struct WeatherReport {
    pub city: String,
    pub temp_celsius: f64,
    pub description: String,
    pub humidity: u32,
    pub wind_speed: f64,
}

impl From<ApiResponse> for WeatherReport {
    fn from(api: ApiResponse) -> Self {
        let description = api.weather
            .first()
            .map(|w| w.description.clone())
            .unwrap_or_else(|| "Unknown".to_string());

        WeatherReport {
            city: api.name,
            temp_celsius: api.main.temp,
            description,
            humidity: api.main.humidity,
            wind_speed: api.wind.speed,
        }
    }
}
```

```csharp
// C# equivalent:
// public record ApiResponse(MainData Main, List<WeatherCondition> Weather, ...);
// public record WeatherReport(string City, double TempCelsius, ...);
// Manual mapping or AutoMapper
```

**Key difference:** `#[derive(Deserialize)]` + `From` impl replaces C#'s `JsonSerializer.Deserialize<T>()` + AutoMapper. Both happen at compile time in Rust - no reflection.

**关键区别：** Rust 用 `#[derive(Deserialize)]` 加 `From` 实现，来替代 C# 里的 `JsonSerializer.Deserialize<T>()` + AutoMapper。整个过程不依赖运行时反射，而是在编译期把很多约束固定下来。

### Step 3: Error Type | 第 3 步：定义错误类型

Create `src/error.rs`:

创建 `src/error.rs`：

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum WeatherError {
    #[error("HTTP request failed: {0}")]
    Http(#[from] reqwest::Error),

    #[error("City not found: {0}")]
    CityNotFound(String),

    #[error("API key not set - export WEATHER_API_KEY")]
    MissingApiKey,
}

pub type Result<T> = std::result::Result<T, WeatherError>;
```

### Step 4: HTTP Client | 第 4 步：实现 HTTP 客户端

Create `src/client.rs`:

创建 `src/client.rs`：

```rust
use crate::error::{WeatherError, Result};
use crate::weather::{ApiResponse, WeatherReport};

pub struct WeatherClient {
    api_key: String,
    http: reqwest::Client,
}

impl WeatherClient {
    pub fn new(api_key: String) -> Self {
        WeatherClient {
            api_key,
            http: reqwest::Client::new(),
        }
    }

    pub async fn get_weather(&self, city: &str) -> Result<WeatherReport> {
        let url = format!(
            "https://api.openweathermap.org/data/2.5/weather?q={}&appid={}&units=metric",
            city, self.api_key
        );

        let response = self.http.get(&url).send().await?;

        if response.status() == reqwest::StatusCode::NOT_FOUND {
            return Err(WeatherError::CityNotFound(city.to_string()));
        }

        let api_data: ApiResponse = response.json().await?;
        Ok(WeatherReport::from(api_data))
    }
}
```

```csharp
// C# equivalent:
// var response = await _httpClient.GetAsync(url);
// if (response.StatusCode == HttpStatusCode.NotFound)
//     throw new CityNotFoundException(city);
// var data = await response.Content.ReadFromJsonAsync<ApiResponse>();
```

**Key differences:**
- `?` operator replaces `try/catch` - errors propagate automatically via `Result`
- `WeatherReport::from(api_data)` uses the `From` trait instead of AutoMapper
- No `IHttpClientFactory` - `reqwest::Client` handles connection pooling internally

**关键差异：**
- `?` 操作符替代了 `try/catch`，错误会沿着 `Result` 自动向上传播
- `WeatherReport::from(api_data)` 用 `From` trait 替代 AutoMapper
- 不需要 `IHttpClientFactory`；`reqwest::Client` 内部已经处理连接复用与池化

### Step 5: Display Formatting | 第 5 步：实现展示格式

Create `src/display.rs`:

创建 `src/display.rs`：

```rust
use std::fmt;
use crate::weather::WeatherReport;

impl fmt::Display for WeatherReport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let icon = weather_icon(&self.description);
        writeln!(f, "{}  {}: {:.0}degC, {}",
            icon, self.city, self.temp_celsius, self.description)?;
        write!(f, "    Humidity: {}%  Wind: {:.1} m/s",
            self.humidity, self.wind_speed)
    }
}

fn weather_icon(description: &str) -> &str {
    let desc = description.to_lowercase();
    if desc.contains("clear") { "sun" }
    else if desc.contains("cloud") { "cloud" }
    else if desc.contains("rain") || desc.contains("drizzle") { "rain" }
    else if desc.contains("snow") { "snow" }
    else if desc.contains("thunder") { "storm" }
    else { "weather" }
}
```

### Step 6: Wire It All Together | 第 6 步：把各模块接起来

`src/lib.rs`:
```rust
pub mod client;
pub mod display;
pub mod error;
pub mod weather;
```

`src/main.rs`:
```rust
use clap::Parser;
use weather_cli::{client::WeatherClient, error::WeatherError};

#[derive(Parser)]
#[command(name = "weather-cli", about = "Fetch weather from the command line")]
struct Cli {
    /// City name to look up
    #[arg(short, long)]
    city: String,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    let api_key = match std::env::var("WEATHER_API_KEY") {
        Ok(key) => key,
        Err(_) => {
            eprintln!("Error: {}", WeatherError::MissingApiKey);
            std::process::exit(1);
        }
    };

    let client = WeatherClient::new(api_key);

    match client.get_weather(&cli.city).await {
        Ok(report) => println!("{report}"),
        Err(WeatherError::CityNotFound(city)) => {
            eprintln!("City not found: {city}");
            std::process::exit(1);
        }
        Err(e) => {
            eprintln!("Error: {e}");
            std::process::exit(1);
        }
    }
}
```

### Step 7: Tests | 第 7 步：添加测试

```rust
// In src/weather.rs or tests/weather_test.rs
#[cfg(test)]
mod tests {
    use super::*;

    fn sample_api_response() -> ApiResponse {
        serde_json::from_str(r#"{
            "main": {"temp": 12.3, "humidity": 82},
            "weather": [{"description": "overcast clouds", "icon": "04d"}],
            "wind": {"speed": 5.4},
            "name": "Seattle"
        }"#).unwrap()
    }

    #[test]
    fn api_response_to_weather_report() {
        let report = WeatherReport::from(sample_api_response());
        assert_eq!(report.city, "Seattle");
        assert!((report.temp_celsius - 12.3).abs() < 0.01);
        assert_eq!(report.description, "overcast clouds");
    }

    #[test]
    fn display_format_includes_icon() {
        let report = WeatherReport {
            city: "Test".into(),
            temp_celsius: 20.0,
            description: "clear sky".into(),
            humidity: 50,
            wind_speed: 3.0,
        };
        let output = format!("{report}");
        assert!(output.contains("sun"));
        assert!(output.contains("20degC"));
    }

    #[test]
    fn empty_weather_array_defaults_to_unknown() {
        let json = r#"{
            "main": {"temp": 0.0, "humidity": 0},
            "weather": [],
            "wind": {"speed": 0.0},
            "name": "Nowhere"
        }"#;
        let api: ApiResponse = serde_json::from_str(json).unwrap();
        let report = WeatherReport::from(api);
        assert_eq!(report.description, "Unknown");
    }
}
```

---

### Final File Layout | 最终文件结构

```text
weather-cli/
|- Cargo.toml
|- src/
|  |- main.rs        # CLI entry point (clap)
|  |- lib.rs         # Module declarations
|  |- client.rs      # HTTP client (reqwest + tokio)
|  |- weather.rs     # Data types + From impl + tests
|  |- display.rs     # Display formatting
|  |- error.rs       # WeatherError + Result alias
|- tests/
    |- integration.rs # Integration tests
```

Compare to the C# equivalent:

对照 C# 版本，大致会长这样：

```text
WeatherCli/
|- WeatherCli.csproj
|- Program.cs
|- Services/
|  |- WeatherClient.cs
|- Models/
|  |- ApiResponse.cs
|  |- WeatherReport.cs
|- Tests/
    |- WeatherTests.cs
```

**The Rust version is remarkably similar in structure.** The main differences are:
- `mod` declarations instead of namespaces
- `Result<T, E>` instead of exceptions
- `From` trait instead of AutoMapper
- Explicit `#[tokio::main]` instead of built-in async runtime

**Rust 版本在结构上其实和 C# 很接近。** 主要差异在于：
- 用 `mod` 声明替代命名空间
- 用 `Result<T, E>` 替代异常
- 用 `From` trait 替代 AutoMapper
- 用显式的 `#[tokio::main]` 替代内建 async 运行时

### Bonus: Integration Test Stub | 附加挑战：集成测试骨架

Create `tests/integration.rs` to test the public API without hitting a real server:

创建 `tests/integration.rs`，在不访问真实服务器的情况下测试公开 API：

```rust
// tests/integration.rs
use weather_cli::weather::WeatherReport;

#[test]
fn weather_report_display_roundtrip() {
    let report = WeatherReport {
        city: "Seattle".into(),
        temp_celsius: 12.3,
        description: "overcast clouds".into(),
        humidity: 82,
        wind_speed: 5.4,
    };

    let output = format!("{report}");
    assert!(output.contains("Seattle"));
    assert!(output.contains("12degC"));
    assert!(output.contains("82%"));
}
```

Run with `cargo test` - Rust discovers tests in both `src/` (`#[cfg(test)]` modules) and `tests/` (integration tests) automatically. No test framework configuration needed - compare that to setting up xUnit/NUnit in C#.

运行 `cargo test` 即可。Rust 会自动发现 `src/` 里的单元测试（`#[cfg(test)]` 模块）和 `tests/` 目录下的集成测试，不需要额外测试框架配置。对比在 C# 中配置 xUnit/NUnit，会更轻量一些。

---

### Extension Challenges | 延伸挑战

Once it works, try these to deepen your skills:

当基础版本跑通后，可以继续做这些扩展练习来加深理解：

1. **Add caching** - Store the last API response in a file. On startup, check if it's less than 10 minutes old and skip the HTTP call. This exercises `std::fs`, `serde_json::to_writer`, and `SystemTime`.
1. **加入缓存** - 把最近一次 API 响应存到文件里。启动时先检查缓存是否在 10 分钟内，如果是就跳过 HTTP 请求。这会练到 `std::fs`、`serde_json::to_writer` 和 `SystemTime`。

2. **Add multiple cities** - Accept `--city "Seattle,Portland,Vancouver"` and fetch all concurrently with `tokio::join!`. This exercises concurrent async.
2. **支持多个城市** - 接收 `--city "Seattle,Portland,Vancouver"` 这样的参数，并用 `tokio::join!` 并发拉取多个结果。这会练到异步并发。

3. **Add a `--format json` flag** - Output the report as JSON instead of human-readable text using `serde_json::to_string_pretty`. This exercises conditional formatting and `Serialize`.
3. **增加 `--format json` 参数** - 用 `serde_json::to_string_pretty` 把输出改成 JSON，而不是人类可读文本。这会练到条件格式化和 `Serialize`。

4. **Write an integration test** - Create `tests/integration.rs` that tests the full flow with a mock HTTP server using `wiremock`. This exercises the `tests/` directory pattern from ch14-1.
4. **补一个真正的集成测试** - 用 `wiremock` 建一个假的 HTTP 服务，在 `tests/integration.rs` 里测试完整流程。这会练到 ch14-1 讲过的 `tests/` 目录模式。

***
