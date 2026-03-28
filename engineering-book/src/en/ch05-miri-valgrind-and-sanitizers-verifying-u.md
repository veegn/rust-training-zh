# Miri, Valgrind, and Sanitizers — Verifying Unsafe Code 🔴

> **What you'll learn:**
> - Miri as a MIR interpreter — what it catches (aliasing, UB, leaks) and what it can't (FFI, syscalls)
> - Valgrind memcheck, Helgrind (data races), Callgrind (profiling), and Massif (heap)
> - LLVM sanitizers: ASan, MSan, TSan, LSan with nightly `-Zbuild-std`
> - `cargo-fuzz` for crash discovery and `loom` for concurrency model checking
> - A decision tree for choosing the right verification tool
>
> **Cross-references:** [Code Coverage](ch04-code-coverage-seeing-what-tests-miss.md) — coverage finds untested paths, Miri verifies the tested ones · [`no_std` & Features](ch09-no-std-and-feature-verification.md) — `no_std` code often requires `unsafe` that Miri can verify · [CI/CD Pipeline](ch11-putting-it-all-together-a-production-cic.md) — Miri job in the pipeline

Safe Rust guarantees memory safety at compile time. But the moment you write
`unsafe` — for FFI, hand-rolled data structures, or performance tricks — those
guarantees become *your* responsibility. This chapter covers the tools that
verify your `unsafe` code actually upholds the safety contracts it claims.

### Miri — An Interpreter for Unsafe Rust

[Miri](https://github.com/rust-lang/miri) is an **interpreter** for Rust's
Mid-level Intermediate Representation (MIR). It executes your program
step-by-step with exhaustive checks for undefined behavior.

```bash
# Install and run
rustup +nightly component add miri
cargo +nightly miri test
```

**Miri catches:**
- Out-of-bounds access
- Use-after-free
- Invalid values (e.g., `bool` not 0 or 1)
- Data races
- **Stacked Borrows/Tree Borrows violation** (aliasing rules)

### Valgrind and Its Rust Integration

[Valgrind](https://valgrind.org/) works on compiled Rust binaries, checking for
memory errors at the machine-code level. It's especially useful for FFI-heavy code.

```bash
valgrind --tool=memcheck --leak-check=full ./target/debug/my_app
```

### AddressSanitizer (ASan) and ThreadSanitizer (TSan)

LLVM sanitizers are faster than Valgrind (2-5× overhead vs 10-50×) but require
nightly and `-Zbuild-std`.

```bash
# ASan example
RUSTFLAGS="-Zsanitizer=address" cargo +nightly test -Zbuild-std --target x86_64-unknown-linux-gnu
```

### Related Tools: Fuzzing and Concurrency

- **`cargo-fuzz`**: Coverage-guided fuzzing for parsers and decoders.
- **`loom`**: Concurrency model checker for lock-free data structures.

### When to Use Which Tool

| Tool | Best For | Requirement |
|------|----------|-------------|
| **Miri** | Pure Rust `unsafe` | Nightly |
| **Valgrind** | FFI / C interop | Linux/macOS |
| **ASan** | Fast crash discovery | Nightly |
| **TSan** | Race detection | Nightly |
| **fuzz** | Complex parsing | Nightly |
| **loom** | Lock-free logic | Stable |

### 🏋️ Exercises

#### 🟡 Exercise 1: Trigger a Miri UB Detection

Write an `unsafe` function that creates two `&mut` references to the same `i32` (aliasing violation). Run `cargo +nightly miri test`.

#### 🔴 Exercise 2: ASan Out-of-Bounds Detection

Create a test with `unsafe` out-of-bounds array access. Build with `RUSTFLAGS="-Zsanitizer=address"` on nightly and observe the report.

### Key Takeaways

- **Miri** is the tool for pure-Rust `unsafe` — it catches aliasing violations.
- **Valgrind** is the tool for FFI/C interop.
- **Sanitizers** (ASan, TSan) are faster than Valgrind but require nightly.
- **`loom`** is for verifying lock-free concurrent logic.

***
