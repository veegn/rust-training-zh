# Compile-Time and Developer Tools 🟡

> **What you'll learn:**
> - Compilation caching with `sccache` for local and CI builds
> - Faster linking with `mold` (3-10× faster than the default linker)
> - `cargo-nextest`: a faster, more informative test runner
> - Developer visibility tools: `cargo-expand`, `cargo-geiger`, `cargo-watch`
> - Workspace lints, MSRV policy, and documentation-as-CI
>
> **Cross-references:** [Release Profiles](ch07-release-profiles-and-binary-size.md) — LTO and binary size optimization · [CI/CD Pipeline](ch11-putting-it-all-together-a-production-cic.md) — these tools integrate into your pipeline · [Dependencies](ch06-dependency-management-and-supply-chain-s.md) — fewer deps = faster compiles

### Compile-Time Optimization: sccache, mold, cargo-nextest

Long compile times are the #1 developer pain point in Rust. These tools
collectively can cut iteration time by 50-80%:

**`sccache` — Shared compilation cache:**
Caches compilation artifacts locally or on cloud storage (S3/GCS).

```bash
cargo install sccache
export RUSTC_WRAPPER=sccache
```

**`mold` — A faster linker:**
Linking is often the slowest phase. `mold` is 3-10× faster than the default linker on Linux.

```toml
# .cargo/config.toml
[target.x86_64-unknown-linux-gnu]
rustflags = ["-C", "link-arg=-fuse-ld=mold"]
```

**`cargo-nextest` — A faster test runner:**
Parallelizes test execution and provides better failure reporting.

```bash
cargo install cargo-nextest
cargo nextest run
```

### Developer Visibility Tools

- **`cargo-expand`**: See the result of macro expansion.
- **`cargo-geiger`**: Count `unsafe` usage across your dependency tree.
- **`cargo-watch`**: Automatically re-run commands on file changes.

### Workspace Lints

Centralize Clippy and compiler lints in your root `Cargo.toml`:

```toml
[workspace.lints.clippy]
unwrap_used = "warn"
dbg_macro = "deny"

[workspace.lints.rust]
unsafe_code = "deny"
```

### 🏋️ Exercises

#### 🟢 Exercise 1: Set Up sccache + mold

Install `sccache` and `mold`, configure them in `.cargo/config.toml`, and measure the speedup on a clean rebuild.

#### 🟡 Exercise 2: Switch to cargo-nextest

Install `cargo-nextest` and run your test suite. Compare the wall-clock time with `cargo test`.

### Key Takeaways

- `sccache` shares compilation cache across your team and CI.
- `mold` is the fastest ELF linker — reduces link times to milliseconds.
- `cargo-nextest` parallelizes tests and supports retries.
- Use `[workspace.lints]` to ensure consistent code quality across many crates.

***
