# Rust Bootstrap Course for C/C++ Programmers 🟢

Welcome to the **Rust Bootstrap Course**. This material is specifically designed for developers coming from a C or C++ background who want to leverage their systems programming experience while adopting Rust's safety and modern features.

## Course Overview

- **The Case for Rust**: Why C/C++ developers need Rust and what problems it eliminates.
- **Foundations**: Types, functions, control flow, and pattern matching.
- **Tooling**: Modules, Cargo, and workspaces.
- **Abstraction**: Traits, generics, and closures.
- **Memory & Concurrency**: Lifetimes, smart pointers, and thread safety.
- **Systems & FFI**: Unsafe Rust, Foreign Function Interface, and `no_std`.
- **Case Studies**: Real-world architectural patterns for translating C++ to Rust.

> [!NOTE]
> This course does **not** cover `async` Rust. For a deep dive into futures, executors, and `tokio`, see the companion [Async Rust Training](../async-book/).

---

## Self-Study Guide

| Phase | Topics | Suggested Time | Checkpoint |
|-------|--------|----------------|------------|
| 1 | Setup, Types, Control Flow | 1 Day | Build a CLI calculator. |
| 2 | Data Structures, Ownership | 1-2 Days | Understand why `let s2 = s1` moves `s1`. |
| 3 | Modules, Error Handling | 1 Day | Propagate errors using the `?` operator. |
| 4 | Traits, Generics, Closures | 1-2 Days | Write generic functions with trait bounds. |
| 5 | Concurrency, Unsafe/FFI | 1 Day | Create a thread-safe counter. |

### How to Use the Exercises
Every chapter contains hands-on exercises marked by difficulty:
- 🟢 **Starter**
- 🟡 **Intermediate**
- 🔶 **Challenge**

**Tip**: Always try the exercise yourself for at least 15 minutes. Fighting the borrow checker is where the actual learning happens. If you're stuck, study the solution, then try to write it again from scratch.

***
