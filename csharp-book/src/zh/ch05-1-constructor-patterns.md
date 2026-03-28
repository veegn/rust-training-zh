[English Original](../en/ch05-1-constructor-patterns.md)

# 构造器模式

> **你将学到什么：** 如何在没有传统构造函数语法的情况下创建 Rust 结构体，包括 `new()` 约定、`Default` trait、工厂方法，以及用于复杂初始化的 builder 模式。
>
> **难度：** 🟢 初级

## C# 与 Rust 的构造器对比
在 C# 中，构造函数是具有特定名称（与类名相同）的方法。Rust 则使用了在约定上返回结构体类型的**关联函数**。

### C# 构造函数
```csharp
public class Config {
    public string Url { get; set; }
    public Config(string url) { Url = url; }
}
```

### Rust 构造函数约定
```rust
pub struct Config {
    pub url: String,
}

impl Config {
    // Rust 中并没有特殊的 'constructor' 关键字。
    // 'new' 只是一个在约定俗成下被广泛使用的普通函数名。
    pub fn new(url: String) -> Self {
        Self { url }
    }
}
```

---

## Default Trait
`Default` trait 是为某种类型提供默认值的标准方式（类似于 C# 中的无参数构造函数）。

```rust
#[derive(Default)]
pub struct Options {
    pub port: u32,       // 默认值为 0
    pub logging: bool,   // 默认值为 false
}

let opt = Options::default();
```

---

## Builder 模式
对于具有许多可选参数的复杂对象，Rust 开发者更倾向于使用 **Builder 模式**，而不是构造函数重载。

```rust
pub struct Server {
    host: String,
    port: u16,
}

pub struct ServerBuilder {
    host: Option<String>,
    port: u16,
}

impl ServerBuilder {
    pub fn new() -> Self {
        Self { host: None, port: 8080 }
    }

    pub fn host(mut self, host: String) -> Self {
        self.host = Some(host);
        self
    }

    pub fn build(self) -> Result<Server, String> {
        let host = self.host.ok_or("必须提供 Host")?;
        Ok(Server { host, port: self.port })
    }
}

// 使用示例
let server = ServerBuilder::new()
    .host("localhost".to_string())
    .build()?;
```
**核心优势：** Builder 模式是类型安全的，且能有效避免编写具有十几个参数的超长构造函数。

---

## 练习：Email 构造器
**挑战：** 创建一个 `EmailBuilder`，要求 `to` 和 `subject` 是强制性的，而 `body` 是可选的。

```rust
struct Email {
    to: String,
    subject: String,
    body: Option<String>,
}

struct EmailBuilder {
    to: Option<String>,
    subject: Option<String>,
    body: Option<String>,
}

impl EmailBuilder {
    fn build(self) -> Result<Email, String> {
        let to = self.to.ok_or("缺少收信人")?;
        let subject = self.subject.ok_or("缺少主题")?;
        Ok(Email { to, subject, body: self.body })
    }
}
```
**要点：** Rust 中的 Builder 模式通常利用 `Result` 来返回一个完整合法的对象或一个详细的错误说明。
