# Performance Comparison: Managed vs Native

> **What you'll learn:** Real-world performance differences between C# and Rust, including startup time, memory usage, and CPU-intensive workloads.
>
> **Difficulty:** Intermediate

One of the main reasons teams migrate from C# to Rust is performance. While .NET has made incredible strides (especially with .NET 8), Rust's "zero-cost abstractions" and lack of a Garbage Collector (GC) offer a different level of efficiency and predictability.

---

## The Numbers at a Glance
| **Metric** | **C# (.NET 8)** | **Rust** | **Why?** |
| :--- | :--- | :--- | :--- |
| **Startup** | ~50-200ms | ~1-5ms | No JIT compilation or runtime overhead. |
| **Memory usage** | ~30MB+ (Base) | <1MB (Base) | No GC heap or metadata overhead. |
| **Binary size** | ~10-50MB | ~1-5MB | Self-contained vs native binary. |
| **p99 Latency** | Variable (GC) | Constant | No "Stop the World" GC pauses. |

---

## Case Study: JSON Processing
Processing a 100MB JSON file is a common task.
*   **C#**: Uses `System.Text.Json`. Highly optimized, but still requires the GC to clean up thousands of temporary strings and objects created during parsing.
*   **Rust**: Uses `Serde`. Can parse data "in-place" without creating new strings, meaning almost zero memory allocation during the entire process.

---

## CPU-Intensive Work: Mandelbrot
In a parallel Mandelbrot calculation (a common CPU benchmark):
*   **C#**: Performance is great, but managing many threads can lead to GC contention if not careful.
*   **Rust**: Using the **Rayon** crate, you get perfect CPU scaling with zero-cost data parallelism. The compiler ensures no data races occur, allowing you to push the hardware to 100% safely.

---

## When to Stay in C#
Rust isn't always the answer. Stick with C# if:
1.  **Development speed** is prioritized over raw performance.
2.  Your app is primarily **I/O bound** (waiting on databases/APIs) and .NET's `async/await` is already handling the load fine.
3.  You need a rich **Desktop UI** (WPF/WinForms/MAUI).

---

## When to Move to Rust
Consider moving to Rust if:
1.  You have a **bottleneck** that C# profile tools show is spending too much time in GC collection.
2.  You are running in a **serverless** (AWS Lambda/Azure Functions) environment where cold start time directly costs money.
3.  You need to run on **resource-constrained** hardware (IoT/Edge devices).

---

## Summary for C# Developers
*   **Predictability is King**: Rust's biggest performance win isn't just that it's "faster"—it's that it's **consistently** fast.
*   **Lower Your Costs**: Rust services often run on smaller, cheaper cloud instances because they use 90% less RAM.
*   **Native AOT**: If you're not ready for Rust, try .NET's Native AOT first to see if it solves your startup and memory issues.

---

## Exercise: Run a Benchmark
**Challenge:** Take a simple loop calculation in C# and rewrite it in Rust. Use `std::time::Instant` to measure the difference. Notice both the execution time and the lack of memory growth in the Rust version.

**Takeaway:** Rust gives you the performance of C++ with the safety (and often better ergonomics) of C#.
