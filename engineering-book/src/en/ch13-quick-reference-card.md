# Quick Reference Card

### Cheat Sheet: Commands at a Glance

```bash
# ─── Build Scripts ───
cargo build -vv                      # Show build.rs output

# ─── Cross-Compilation ───
cargo build --target x86_64-unknown-linux-musl
cargo zigbuild --target x86_64-unknown-linux-gnu.2.17
cross build --target aarch64-unknown-linux-gnu

# ─── Benchmarking ───
cargo bench                          # Run all benchmarks
cargo flamegraph -- --args           # Generate flamegraph

# ─── Coverage ───
cargo llvm-cov --html                # Generate HTML report
cargo llvm-cov --fail-under-lines 80 # CI gate

# ─── Safety Verification ───
cargo +nightly miri test             # Run tests under Miri
valgrind --leak-check=full ./bin     # Memory checker

# ─── Audit & Supply Chain ───
cargo audit                          # Vulnerability scan
cargo deny check                     # License/Advisory/Ban/Source
cargo geiger                         # Count unsafe code

# ─── Binary Optimization ───
cargo bloat --release --crates       # Size analysis
cargo +nightly udeps --workspace     # Find unused deps
cargo clippy --fix                   # Auto-fix lints

# ─── Compile-Time Optimization ───
export RUSTC_WRAPPER=sccache         # Compilation cache
cargo nextest run                    # Faster test runner

# ─── Platform Engineering ───
cargo xwin build --target x86_64-pc-windows-msvc
cargo hack check --each-feature      # Verify feature flags

# ─── Release ───
cargo release patch --execute        # Bump & tag
cargo dist plan                      # Preview distribution
```

### Decision Table: Which Tool When

| Goal | Tool |
|------|------|
| Traceability | `build.rs` (SOURCE_DATE_EPOCH) |
| Static Binary | `musl` target |
| Regression detection | Criterion.rs |
| Coverage gates | `cargo-llvm-cov` |
| Unsafe verification | Miri / Valgrind |
| Supply Chain | `cargo-audit` / `cargo-deny` |
| Binary Size | `cargo-bloat` / `LTO` |
| Fast Linker | `mold` |
| Auto-rebuild | `cargo-watch` |
| CI Speed | `rust-cache` + `sccache` |

***
*Version 1.3 — Reference for Rust Training Engineering Book.*
