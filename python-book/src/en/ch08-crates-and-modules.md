# 8. Crates and Modules 🟢

> **What you'll learn:**
> - `mod` and `use` vs Python's `import`
> - Visibility (`pub`) vs Python's convention-based privacy
> - `Cargo.toml` vs `pyproject.toml`
> - Workspaces: Rust's native support for Monorepos

## Rust Modules vs Python Packages

| Concept | Python | Rust |
|---------|--------|------|
| Module = file | ✅ Automatic | Must declare with **`mod`** |
| Package = directory | `__init__.py` | **`mod.rs`** (or main file) |
| Public by default | ✅ Everything | ❌ **Private** by default |
| Make public | `_prefix` convention | **`pub`** keyword |
| Import syntax | `from x import y` | **`use x::y;`** |

### Visibility — Private by Default
In Python, "private" is a gentleman's agreement using the `_` prefix. In Rust, the compiler **enforces** privacy.

```rust
pub struct User {
    pub name: String,      // Anyone can access
    age: i32,              // ONLY this module can access
}

impl User {
    pub fn new(name: &str, age: i32) -> Self {
        User { name: name.to_string(), age }
    }
}

// Outside:
let u = User::new("Alice", 30);
println!("{}", u.name); // ✅ OK
// println!("{}", u.age);  // ❌ Compile error!
```

---

## Crates vs PyPI Packages

### Python (PyPI)
```bash
pip install requests
# versions in requirements.txt
```

### Rust (crates.io)
```bash
cargo add reqwest
# versions in Cargo.toml (auto-locked in Cargo.lock)
```

### Essential Crate Mapping
| Python Library | Rust Crate |
|---------------|------------|
| `requests` | `reqwest` |
| `pydantic` | `serde` |
| `json` | `serde_json` |
| `asyncio` | `tokio` |
| `fastapi` | `axum` / `actix-web` |
| `click` | `clap` |

---

## Workspaces (Monorepos)

Rust has first-class support for monorepos built into Cargo. All crates in a workspace share a single `Cargo.lock`, ensuring consistent versions across your entire project.

```toml
# Main Cargo.toml
[workspace]
members = ["api", "core", "cli"]
```

---

## Exercises

<details>
<summary><strong>🏋️ Exercise: Module Visibility</strong></summary>

**Challenge**: Which of these calls in `main()` will fail?

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
<summary>🔑 Solution</summary>

`internal::private()` will fail because it is not marked with `pub`. In Rust, everything is private to its module by default.

</details>
</details>

***
