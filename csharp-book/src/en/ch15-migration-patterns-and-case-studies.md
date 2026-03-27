# Migration Patterns and Case Studies

> **What you'll learn:** How to translate common C# patterns (Repository, Builder, DI) to idiomatic Rust and real-world results from companies that have made the switch.
>
> **Difficulty:** Intermediate

When moving a project from C# to Rust, you're not just changing syntax; you're changing your **mental model**. This chapter maps familiar C# architectural patterns to their Rust equivalents.

---

## Pattern Mapping: C# to Rust
| **C# Pattern** | **Rust Equivalent** | **Notes** |
| :--- | :--- | :--- |
| **Repository** | Traits + Generics | `trait Repo<T>` |
| **Dependency Injection** | Constructor Injection | Pass dependencies to `new()` |
| **Builder** | Consuming Builder | Each step takes `self` and returns `Self` |
| **LINQ** | Iterator Chains | `.iter().filter().map()` |
| **Entity Framework** | SQLx or SeaORM | SQLx is popular for its "SQL first" approach |
| **Try/Catch** | `Result<T, E> + ?` | Explicit error propagation |

---

## Case Study: CLI Tool Migration
**Problem:** A C# tool used 4GB of RAM to process 500MB CSV files because it loaded everything into memory via `ToList()`.
**Solution:** Rewritten in Rust using streaming iterators.
**Result:**
*   **Memory:** 4GB -> 12MB
*   **Speed:** 45s -> 3s
*   **Binary:** Self-contained executable, no .NET runtime needed.

---

## Case Study: Microservice Replacement
**Problem:** A high-traffic Auth microservice (10k req/s) suffered from unpredictable p99 latency spikes (up to 200ms) due to Garbage Collection.
**Solution:** Replaced with a Rust service using **Axum**.
**Result:**
*   **p99 Latency:** 200ms -> 4ms (GC spikes eliminated)
*   **Docker Image:** 210MB -> 12MB
*   **Cold Start:** 2.1s -> 0.05s

---

## The Migration Mindset
1.  **Don't build "Class Hierarchies"**: Use flat structures and Traits.
2.  **Avoid `Clone` as a default**: In C#, everything is a reference. In Rust, cloning can be expensive. Think about ownership first.
3.  **Iterators are your friend**: They are the key to high-performance, low-memory data processing in Rust.

---

## Summary for C# Developers
| **Old Habit** | **New Habit** |
| :--- | :--- |
| Throw an Exception | Return a `Result` |
| Use a Base Class | Use a Trait |
| Create a `List` first | Use an Iterator until the end |
| Let GC handle it | Use `Drop` for cleanup |

---

## Exercise: Identify a Migration Target
**Challenge:** Look at your current C# project. Identify a single service or tool that is either:
1.  Suffering from high memory usage.
2.  Experiencing unpredictable GC pauses.
3.  Need to run as a small, fast CLI tool.

**Takeaway:** Successful migrations start with **hot paths** where Rust's performance and memory guarantees provide the most immediate value.
