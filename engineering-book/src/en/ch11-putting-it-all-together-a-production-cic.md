# Putting It All Together — A Production CI/CD Pipeline 🟡

> **What you'll learn:**
> - Structuring a multi-stage GitHub Actions CI workflow (check → test → coverage → security → cross → release)
> - Caching strategies with `rust-cache` and `save-if` tuning
> - Running Miri and sanitizers on a nightly schedule
> - Task automation with `Makefile.toml` and pre-commit hooks
> - Automated releases with `cargo-dist`
>
> **Cross-references:** Chapters 1–10 cover the individual tools integrated here.

Individual tools are useful. A pipeline that orchestrates them automatically on
every push is transformative. This chapter assembles the tools from chapters 1–10
into a cohesive CI/CD workflow.

### The Complete GitHub Actions Workflow

A recommended multi-stage pipeline:

1. **Check**: `clippy`, `rustfmt`, `cargo check`. (Fastest feedback)
2. **Test**: `cargo test` on Ubuntu and Windows.
3. **Cross**: Build for ARM and MUSL targets.
4. **Coverage**: `cargo llvm-cov` with minimum threshold enforcement.
5. **Safety**: `cargo miri test` for unsafe verification.
6. **Security**: `cargo audit` and `cargo deny check`.

### CI Caching Strategies

Use [`Swatinem/rust-cache@v2`](https://github.com/Swatinem/rust-cache) to speed up builds.

```yaml
- uses: Swatinem/rust-cache@v2
  with:
    save-if: ${{ github.ref == 'refs/heads/main' }}
```

### Task Automation with `cargo-make`

[`cargo-make`](https://sagiegurari.github.io/cargo-make/) provides a portable
task runner to replace complex shell scripts or platform-dependent Makefiles.

```toml
# Makefile.toml
[tasks.dev]
description = "Full local verification"
dependencies = ["check", "test", "clippy", "fmt-check"]
```

### Automated Releases with `cargo-dist`

[`cargo-dist`](https://github.com/axodotdev/cargo-dist) automates the creation
of GitHub Releases, covering multiple platforms and generating installer scripts.

```bash
cargo dist init
cargo dist plan
```

### 🏋️ Exercises

#### 🟢 Exercise 1: Create a Basic CI Workflow

Create a `.github/workflows/ci.yml` that runs `cargo check`, `cargo test`, and `cargo clippy`.

#### 🟡 Exercise 2: Local Workflow with `cargo-make`

Install `cargo-make`, create a `Makefile.toml` that runs your tests and coverage, and verify it works locally.

### Key Takeaways

- Structure CI as parallel stages with fast feedback first.
- Use `rust-cache` with `save-if` tuning to avoid cache thrashing.
- Automate common developer workflows with `cargo-make`.
- Use `cargo-dist` to handle the complexity of multi-platform releases.

***
