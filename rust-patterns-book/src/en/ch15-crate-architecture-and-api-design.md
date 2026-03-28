# 15. Crate Architecture and API Design 🟡

> **What you'll learn:**
> - Module layout and re-exports.
> - Public API design checklist.
> - Ergonomic parameters: `impl Into`, `AsRef`, `Cow`.
> - "Parse, don't validate" pattern.
> - Feature flags and workspaces.

## Module Layout

Organize your crate logically and use `pub use` to create a clean, flat public API at the root.

```text
src/
├── lib.rs (pub use modules here)
├── error.rs
├── parser.rs
└── connection.rs
```

---

## Ergonomic Parameters

Make your API easy to call by accepting the most general types.

### `impl Into<T>`
Accept any type that can be converted into the type you need.
```rust
// Instead of: fn set_name(name: String)
fn set_name(name: impl Into<String>) {
    let name = name.into();
}
// Now callers can pass "literal", String, etc.
```

### `AsRef<T>`
Accept any type that can be borrowed as a reference.
```rust
// Instead of: fn read_file(path: &Path)
fn read_file(path: impl AsRef<Path>) {
    let path = path.as_ref();
}
// Now callers can pass "/tmp/test.txt", PathBuf, etc.
```

---

## Parse, Don't Validate

Don't just check if data is valid and then pass around raw types (like `u16` for a port). Instead, **parse** it into a newtype that *guarantees* validity.

```rust
pub struct Port(u16);

impl TryFrom<u16> for Port {
    type Error = &'static str;
    fn try_from(p: u16) -> Result<Self, Self::Error> {
        if p == 0 { Err("invalid port") } else { Ok(Port(p)) }
    }
}

// Logic functions now take 'Port', not 'u16'.
fn start_server(port: Port) { ... }
```

---

## API Design Checklist

- **`#[must_use]`**: Use on types (like Results or Guards) that should not be ignored.
- **`#[non_exhaustive]`**: Use on enums/structs to allow adding fields/variants without breaking semver.
- **Sealed Traits**: Prevent users from implementing your traits if internal invariants depend on them.
- **Feature Flags**: Use `[features]` in `Cargo.toml` to keep the default build lightweight.

---

## Workspaces

For multi-crate projects, use a workspace to share dependencies and the `Cargo.lock` file.

```toml
[workspace]
members = ["core", "cli", "server"]
```

***
