# Release Profiles and Binary Size 🟡

> **What you'll learn:**
> - Release profile anatomy: LTO, codegen-units, panic strategy, strip, opt-level
> - Thin vs Fat vs Cross-Language LTO trade-offs
> - Binary size analysis with `cargo-bloat`
> - Dependency trimming with `cargo-udeps`, `cargo-machete` and `cargo-shear`
>
> **Cross-references:** [Compile-Time Tools](ch08-compile-time-and-developer-tools.md) — the other half of optimization · [Benchmarking](ch03-benchmarking-measuring-what-matters.md) — measure runtime before you optimize · [Dependencies](ch06-dependency-management-and-supply-chain-s.md) — trimming deps reduces both size and compile time

The default `cargo build --release` is already good. But for production
deployment — especially single-binary tools deployed to thousands of servers —
there's a significant gap between "good" and "optimized." This chapter covers
the profile knobs and the tools to measure binary size.

### Release Profile Anatomy

Cargo profiles control how `rustc` compiles your code. The defaults are
conservative — designed for broad compatibility, not maximum performance:

```toml
[profile.release]
opt-level = 3        # Optimization level (0=none, 1=basic, 2=good, 3=aggressive)
lto = false          # Link-time optimization OFF
codegen-units = 16   # Parallel compilation units (faster compile, less optimization)
panic = "unwind"     # Stack unwinding on panic (larger binary)
strip = "none"       # Keep all symbols and debug info
```

**Production-optimized profile:**

```toml
[profile.release]
lto = true           # Full cross-crate optimization
codegen-units = 1    # Single codegen unit — maximum optimization opportunity
panic = "abort"      # No unwinding overhead — smaller, faster
strip = true         # Remove all symbols — smaller binary
```

### LTO in Depth — Thin vs Fat

Link-Time Optimization lets LLVM optimize across crate boundaries — inlining
functions from dependencies, removing dead code, etc.

- `lto = true` (Fat LTO): Maximum optimization, slowest compile.
- `lto = "thin"` (Thin LTO): 90% optimization, much faster than fat. Good default.

### Binary Size Analysis with `cargo-bloat`

[`cargo-bloat`](https://github.com/RazrFalcon/cargo-bloat) answers:
"What functions and crates are taking up the most space in my binary?"

```bash
# Show by crate
cargo bloat --release --crates
```

### Trimming Dependencies

- **`cargo-udeps`**: Finds unused dependencies (requires nightly).
- **`cargo-machete`**: Fast, heuristic-based removal.
- **`cargo-shear`**: Balanced and reliable removal tool.

### 🏋️ Exercises

#### 🟢 Exercise 1: Measure LTO Impact

Build a project with default release settings, then with `lto = true` + `codegen-units = 1` + `strip = true`. Compare binary size and compile time.

#### 🟡 Exercise 2: Find Your Biggest Crate

Run `cargo bloat --release --crates` on a project. Identify the largest dependency. Can you reduce it by disabling default features?

### Key Takeaways

- `lto = true` + `codegen-units = 1` + `strip = true` + `panic = "abort"` is the production release profile.
- Thin LTO (`lto = "thin"`) is the best balance for most projects.
- `cargo-bloat` tells you exactly which crates take up space.
- `cargo-udeps` finds dead dependencies that waste compile time and binary size.

***
