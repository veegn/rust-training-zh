# Tricks from the Trenches 🟡

> **What you'll learn:**
> - Battle-tested patterns that don't fit neatly into one chapter
> - Common pitfalls and their fixes — from CI flake to binary bloat
> - Quick-win techniques you can apply to any Rust project today
>
> **Cross-references:** Every chapter in this book — these tricks cut across all topics

This chapter collects engineering patterns that come up repeatedly in
production Rust codebases. Each trick is self-contained — read them in
any order.

### 1. The `deny(warnings)` Trap

**Problem**: `#![deny(warnings)]` breaks builds when Clippy adds new lints.
**Fix**: Use `CARGO_ENCODED_RUSTFLAGS="-Dwarnings"` in CI instead.

### 2. Compile Once, Test Everywhere

**Problem**: `cargo test` recompiles when switching between `--lib` and `--doc`.
**Fix**: Use `cargo nextest` for code tests and run `cargo test --doc` separately.

### 3. Feature Flag Hygiene

**Problem**: Features are often broken when compiled in isolation.
**Fix**: Use `cargo-hack --each-feature` in CI.

### 4. Optimized Dependencies in Debug Builds

**Problem**: Debug builds are slow because dependencies (like `serde`) aren't optimized.
**Fix**: Add this to `Cargo.toml`:

```toml
[profile.dev.package."*"]
opt-level = 2
```

### 5. CI Cache Thrashing

**Problem**: Every PR saves a new cache, wasting space.
**Fix**: Set `save-if: ${{ github.ref == 'refs/heads/main' }}` in your cache action.

### 6. `SOURCE_DATE_EPOCH` for Reproducibility

**Problem**: Using `now()` in `build.rs` makes binaries non-reproducible.
**Fix**: Honor the `SOURCE_DATE_EPOCH` environment variable if it exists.

### 7. Dependency Deduplication

**Problem**: Duplicate crates (e.g., `syn` 1.0 and 2.0) bloat compile times.
**Fix**: Use `cargo tree --duplicates` and `cargo update -p <parent>` to unify them.

### 8. Pre-Push Smoke Test

**Problem**: Waiting for CI to fail on formatting.
**Fix**: Run a local check script (via `cargo-make`) before pushing.

### 🏋️ Exercises

#### 🟢 Exercise 1: Apply Three Tricks

Pick three tricks from this chapter and apply them to an existing Rust project. Which had the biggest impact?

#### 🟡 Exercise 2: Deduplicate Your Dependency Tree

Run `cargo tree --duplicates` on a real project. Eliminate at least one duplicate. Measure compile-time before and after.

### Key Takeaways

- `[profile.dev.package."*"] opt-level = 2` is a massive productivity booster.
- Avoid source-level `deny(warnings)`.
- Use `cargo-hack` to verify all feature combinations.
- Monthly dependency deduplication keeps compile times in check.

***
