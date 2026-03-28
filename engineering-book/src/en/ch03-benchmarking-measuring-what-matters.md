# Benchmarking — Measuring What Matters 🟡

> **What you'll learn:**
> - Why naive timing with `Instant::now()` produces unreliable results
> - Statistical benchmarking with Criterion.rs and the lighter Divan alternative
> - Profiling hot spots with `perf`, flamegraphs, and PGO
> - Setting up continuous benchmarking in CI to catch regressions automatically
>
> **Cross-references:** [Release Profiles](ch07-release-profiles-and-binary-size.md) — once you find the hot spot, optimize the binary · [CI/CD Pipeline](ch11-putting-it-all-together-a-production-cic.md) — benchmark job in the pipeline · [Code Coverage](ch04-code-coverage-seeing-what-tests-miss.md) — coverage tells you what's tested, benchmarks tell you what's fast

"We should forget about small efficiencies, say about 97% of the time: premature
optimization is the root of all evil. Yet we should not pass up our opportunities
in that critical 3%." — Donald Knuth

The hard part isn't *writing* benchmarks — it's writing benchmarks that produce
**meaningful, reproducible, actionable** numbers. This chapter covers the tools
and techniques that get you from "it seems fast" to "we have statistical evidence
that PR #347 regressed parsing throughput by 4.2%."

### Why Not `std::time::Instant`?

The temptation:

```rust
// ❌ Naive benchmarking — unreliable results
use std::time::Instant;

fn main() {
    let start = Instant::now();
    let result = parse_device_query_output(&sample_data);
    let elapsed = start.elapsed();
    println!("Parsing took {:?}", elapsed);
    // Problem 1: Compiler may optimize away `result` (dead code elimination)
    // Problem 2: Single sample — no statistical significance
    // Problem 3: CPU frequency scaling, thermal throttling, other processes
    // Problem 4: Cold cache vs warm cache not controlled
}
```

Problems with manual timing:
1. **Dead code elimination** — the compiler may skip the computation entirely if
   the result isn't used.
2. **No warm-up** — the first run includes cache misses, JIT effects (irrelevant
   in Rust, but OS page faults apply), and lazy initialization.
3. **No statistical analysis** — a single measurement tells you nothing about
   variance, outliers, or confidence intervals.
4. **No regression detection** — you can't compare against previous runs.

### Criterion.rs — Statistical Benchmarking

[Criterion.rs](https://bheisler.github.io/criterion.rs/book/) is the de facto
standard for Rust micro-benchmarks. It uses statistical methods to produce
reliable measurements and detects performance regressions automatically.

**Setup:**

```toml
# Cargo.toml
[dev-dependencies]
criterion = { version = "0.5", features = ["html_reports", "cargo_bench_support"] }

[[bench]]
name = "parsing_bench"
harness = false  # Use Criterion's harness, not the built-in test harness
```

**A complete benchmark:**

```rust
// benches/parsing_bench.rs
use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};

// ... function under test ...

fn bench_parse_gpu_csv(c: &mut Criterion) {
    let small_input = "0, Acme Accel-V1-80GB, 32, 65.5\n1, Acme Accel-V1-80GB, 34, 67.2\n";
    c.bench_function("parse_2_gpus", |b| {
        b.iter(|| parse_gpu_csv(black_box(small_input)))
    });
}

criterion_group!(benches, bench_parse_gpu_csv);
criterion_main!(benches);
```

**What `black_box()` does**: It's a compiler hint that prevents dead-code
elimination and over-aggressive constant folding. The compiler cannot see
through `black_box`, so it must actually compute the result.

### Divan — A Lighter Alternative

[Divan](https://github.com/nvzqz/divan) is a newer benchmarking framework that
uses attribute macros instead of Criterion's macro DSL:

```rust
#[divan::bench]
fn parse_2_gpus() -> Vec<GpuInfo> {
    parse_gpu_csv(divan::black_box(SMALL_INPUT))
}
```

### Profiling with `perf` and Flamegraphs

Benchmarks tell you *how fast* — profiling tells you *where the time goes*.

```bash
# Generate a flamegraph
cargo flamegraph --root -- --run-diagnostics
```

**Reading a flamegraph:**
- **Width** = time spent in that function (wider = slower)
- **Top** = leaf functions doing actual work — look for wide plateaus at the top.

### Continuous Benchmarking in CI

Detect performance regressions before they ship by running benchmarks in your
CI pipeline and comparing against historical data.

### 🏋️ Exercises

#### 🟢 Exercise 1: First Criterion Benchmark

Create a crate with a function that sorts a `Vec<u64>` of 10,000 random elements. Write a Criterion benchmark for it, then switch to `.sort_unstable()` and observe the performance difference.

#### 🟡 Exercise 2: Flamegraph Hot Spot

Build a project with `debug = true` in `[profile.release]`, then generate a flamegraph. Identify the top 3 widest stacks.

### Key Takeaways

- Never benchmark with `Instant::now()` — use Criterion.rs for statistical rigor.
- `black_box()` prevents the compiler from optimizing away your benchmark target.
- `hyperfine` measures wall-clock time for the whole binary; Criterion measures individual functions.
- Flamegraphs show *where* time is spent; benchmarks show *how much* time is spent.

***
