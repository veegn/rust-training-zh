# Code Coverage — Seeing What Tests Miss 🟢

> **What you'll learn:**
> - Source-based coverage with `cargo-llvm-cov` (the most accurate Rust coverage tool)
> - Quick coverage checks with `cargo-tarpaulin` and Mozilla's `grcov`
> - Setting up coverage gates in CI with Codecov and Coveralls
> - A coverage-guided testing strategy that prioritizes high-risk blind spots
>
> **Cross-references:** [Miri and Sanitizers](ch05-miri-valgrind-and-sanitizers-verifying-u.md) — coverage finds untested code, Miri finds UB in tested code · [Benchmarking](ch03-benchmarking-metasuring-what-matters.md) — coverage shows *what's tested*, benchmarks show *what's fast* · [CI/CD Pipeline](ch11-putting-it-all-together-a-production-cic.md) — coverage gate in the pipeline

Code coverage measures which lines, branches, or functions your tests actually
execute. It doesn't prove correctness (a covered line can still have bugs), but
it reliably reveals **blind spots** — code paths that no test exercises at all.

### Source-Based Coverage with `llvm-cov`

Rust uses LLVM, which provides source-based coverage instrumentation — the most
accurate coverage method available. The recommended tool is
[`cargo-llvm-cov`](https://github.com/taiki-e/cargo-llvm-cov):

```bash
# Install
cargo install cargo-llvm-cov
rustup component add llvm-tools-preview

# Run tests and show summary
cargo llvm-cov

# Generate HTML report
cargo llvm-cov --html
```

**Coverage types explained:**

| Type | What It Measures |
|------|------------------|
| **Line coverage** | Which source lines were executed |
| **Branch coverage** | Which `if`/`match` arms were taken |
| **Function coverage** | Which functions were called |

### cargo-tarpaulin — The Quick Path

[`cargo-tarpaulin`](https://github.com/xd009642/tarpaulin) is a Linux-specific
coverage tool that's simpler to set up (no LLVM components needed):

```bash
cargo tarpaulin --out Html
```

### Coverage in CI: Codecov and Coveralls

Upload coverage data to a tracking service for historical trends and PR annotations:

```yaml
# GitHub Action step example
- name: Generate coverage
  run: cargo llvm-cov --workspace --lcov --output-path lcov.info

- name: Upload to Codecov
  uses: codecov/codecov-action@v4
  with:
    files: lcov.info
```

### Coverage-Guided Testing Strategy

1. **High coverage, high risk**: Good — maintain it.
2. **Low coverage, high risk**: **RED ALERT** — write tests now.
3. **Exclusion**: Don't chase 100% coverage. Exclude noise like generated code or test files.

### 🏋️ Exercises

#### 🟢 Exercise 1: First Coverage Report

Install `cargo-llvm-cov`, run it on any Rust project, and open the HTML report. Find the three files with the lowest line coverage.

#### 🟡 Exercise 2: CI Coverage Gate

Add a coverage gate to a GitHub Actions workflow that fails if line coverage drops below 60%. Verify it works by commenting out a test.

### Key Takeaways

- `cargo-llvm-cov` is the most accurate coverage tool for Rust.
- Coverage doesn't prove correctness, but **zero coverage proves zero testing**.
- Set a coverage gate in CI (e.g., `--fail-under-lines 80`) to prevent regressions.
- Focus on high-risk code paths (error handling, unsafe, parsing).

***
