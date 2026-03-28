# 15. Crate 架构与 API 设计 🟡

> **你将学到：**
> - 模块布局与重导出策略。
> - 公共 API 设计清单。
> - 易用的参数模式：`impl Into`、`AsRef`、`Cow`。
> - “以解析代替校验”模式。
> - 特性标志（Feature flags）与工作空间（Workspaces）。

## 模块布局

合理组织你的 crate，并使用 `pub use` 在根部创建一个整洁、平整的公共 API。

```text
src/
├── lib.rs (在此处重导出模块内容)
├── error.rs
├── parser.rs
└── connection.rs
```

---

## 易用的参数模式

通过接收更宽泛的类型，使你的 API 调用起来更方便。

### `impl Into<T>`
接收任何可以转换为所需类型的类型。
```rust
// 不要：fn set_name(name: String)
fn set_name(name: impl Into<String>) {
    let name = name.into();
}
// 现在调用者可以传入 "字面量"、String、等等。
```

### `AsRef<T>`
接收任何可以借用为引用的类型。
```rust
// 不要：fn read_file(path: &Path)
fn read_file(path: impl AsRef<Path>) {
    let path = path.as_ref();
}
// 现在调用者可以传入 "/tmp/test.txt"、PathBuf、等等。
```

---

## 以解析代替校验 (Parse, Don't Validate)

不要在逻辑中层层校验数据（如校验 `u16` 是否为有效端口），而是将其 **解析** 为一个能够 *保证* 有效性的新类型。

```rust
pub struct Port(u16);

impl TryFrom<u16> for Port {
    type Error = &'static str;
    fn try_from(p: u16) -> Result<Self, Self::Error> {
        if p == 0 { Err("无效端号") } else { Ok(Port(p)) }
    }
}

// 业务逻辑函数现在接收的是 'Port' 而非 'u16'。
fn start_server(port: Port) { ... }
```

---

## API 设计清单

- **`#[must_use]`**：应用于不应被忽略的类型（如 Result 或 Guard）。
- **`#[non_exhaustive]`**：应用于枚举/结构体，以便在不破坏语义化版本（semver）的前提下添加字段/变体。
- **密封 Trait (Sealed Traits)**：如果内部不变量依赖于你的 trait，可防止用户自行实现这些 trait。
- **特性标志 (Feature Flags)**：利用 `Cargo.toml` 中的 `[features]` 保持默认构建的轻量化。

---

## 工作空间 (Workspaces)

对于多 crate 项目，使用工作空间来共享依赖项和 `Cargo.lock` 文件。

```toml
[workspace]
members = ["core", "cli", "server"]
```

***
