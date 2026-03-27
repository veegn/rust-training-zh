# Constructor Patterns

> **What you'll learn:** How to create Rust structs without traditional constructors - `new()` conventions, the `Default` trait, factory methods, and the builder pattern for complex initialization.
>
> **Difficulty:** 🟢 Beginner

## C# vs Rust Constructors
In C#, you rely on class constructors with the same name as the class. Rust uses **associated functions** that return the struct type by convention.

### C# Constructor
```csharp
public class Config {
    public string Url { get; set; }
    public Config(string url) { Url = url; }
}
```

### Rust Constructor Convention
```rust
pub struct Config {
    pub url: String,
}

impl Config {
    // There is no special 'constructor' keyword. 
    // 'new' is just a standard function name used by convention.
    pub fn new(url: String) -> Self {
        Self { url }
    }
}
```

---

## The Default Trait
The `Default` trait is the standard way to provide a value for a type when one isn't specified (similar to a parameterless constructor in C#).

```rust
#[derive(Default)]
pub struct Options {
    pub port: u32,       // Defaults to 0
    pub logging: bool,   // Defaults to false
}

let opt = Options::default();
```

---

## The Builder Pattern
For complex objects with many optional parameters, Rust developers favor the **Builder Pattern** over constructor overloading.

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
        let host = self.host.ok_or("Host is required")?;
        Ok(Server { host, port: self.port })
    }
}

// Usage
let server = ServerBuilder::new()
    .host("localhost".to_string())
    .build()?;
```
**Key Advantage:** This is type-safe and avoids "telescoping constructors" (constructors with 10+ parameters).

---

## Exercise: Email Builder
**Challenge:** Create an `EmailBuilder` where `to` and `subject` are mandatory, and `body` is optional.

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
        let to = self.to.ok_or("To is missing")?;
        let subject = self.subject.ok_or("Subject is missing")?;
        Ok(Email { to, subject, body: self.body })
    }
}
```
**Takeaway:** The Builder pattern in Rust often uses `Result` to return a fully validated object or a descriptive error.
