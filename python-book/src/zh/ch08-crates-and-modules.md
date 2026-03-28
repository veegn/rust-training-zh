[English Original](../en/ch08-crates-and-modules.md)

# 8. Crate 与 模块 🟢

> **你将学到：**
> - `mod` 和 `use` 关键字 vs Python 的 `import`
> - 显式可见性 (`pub`) vs Python 基于约定的隐私策略
> - `Cargo.toml` 与 `pyproject.toml` 的对应关系
> - 工作区 (Workspaces)：Rust 原生支持的单仓 (Monorepo) 模式

## Rust 模块 vs Python 包

| 概念 | Python | Rust |
|---------|--------|------|
| 模块 = 文件 | ✅ 自动识别 | 必须显式用 **`mod`** 声明 |
| 包 = 目录 | `__init__.py` | **`mod.rs`** (或特定的入口文件) |
| 默认是否公开 | ✅ 全部公开 | ❌ **默认私有** |
| 使其公开 | `_前缀` 约定 | **`pub`** 关键字 |
| 导入语法 | `from x import y` | **`use x::y;`** |

### 可见性 — 默认私有
在 Python 中，“私有”只是以 `_` 开头的绅士协议。而在 Rust 中，编译器会**强制**执行可见性规则。

```rust
pub struct User {
    pub name: String,      // 公开：任何人均可访问
    age: i32,              // 私有：仅限本模块访问
}

impl User {
    pub fn new(name: &str, age: i32) -> Self {
        User { name: name.to_string(), age }
    }
}

// 在模块外部调用：
let u = User::new("李雷", 30);
println!("{}", u.name); // ✅ 正常运行
// println!("{}", u.age);  // ❌ 编译报错：该字段为私有！
```

---

## Crate vs PyPI 包

### Python (PyPI)
```bash
pip install requests
# 版本记录在 requirements.txt 或 poetry.lock
```

### Rust (crates.io)
```bash
cargo add reqwest
# 依赖记录在 Cargo.toml 中 (自动在 Cargo.lock 中锁定版本)
```

### 给 Python 开发者的常用 Crate 映射
| Python 库 | Rust 对应 Crate | 用途 |
|---------------|------------|---------|
| `requests` | `reqwest` | HTTP 客户端 |
| `pydantic` | `serde` | 序列化/验证 |
| `json` | `serde_json` | JSON 解析 |
| `asyncio` | `tokio` | 异步运行时 |
| `fastapi` | `axum` / `actix-web` | Web 框架 |
| `click` | `clap` | 命令行参数解析 |

---

## 工作区 (Workspaces)

Rust 原生支持单仓 (Monorepo) 模型。工作区内的所有项目共用一个 `Cargo.lock` 文件，确保整个项目依赖版本的绝对一致性。

```toml
# 根目录下的 Cargo.toml
[workspace]
members = ["api", "core", "cli"]
```

---

## 练习

<details>
<summary><strong>🏋️ 练习：模块可见性</strong> (点击展开)</summary>

**挑战**：判断以下 `main()` 函数中的哪一行会编译失败？

```rust
mod internal {
    fn private() {}
    pub fn public() {}
}

fn main() {
    internal::public();
    internal::private();
}
```

<details>
<summary>参考答案</summary>

`internal::private()` 会编译失败。在 Rust 中，如果没有显式标记为 `pub`，该项内容对其所属模块之外的一切都是不可见的。

</details>
</details>

***
