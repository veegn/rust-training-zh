# 16. Best Practices 🟢

> **What you'll learn:**
> - Top 10 habits for writing idiomatic Rust
> - Performance Benchmarks: Python vs Rust
> - A 3-month roadmap for mastering Rust
> - The Python → Rust "Rosetta Stone" (cheat sheet)

## Top 10 Habits to Build

1. **Read the Error Messages**: Rust has the best compiler errors. They often tell you exactly how to fix the code.
2. **Use `match` instead of `if`**: Whenever possible, use pattern matching on enums.
3. **Accept `&str` and `&[T]`**: In function parameters, accept borrowed slices instead of owned `String` or `Vec`.
4. **Iterators > Loops**: Use `.map()`, `.filter()`, and `.fold()` instead of index-based loops.
5. **Rustfmt & Clippy**: Run `cargo fmt` and `cargo clippy` constantly. They are like `Black` and `Pylint` but more powerful.
6. **Prefer `Result` over `Panic`**: Only use `.unwrap()` when you are 100% sure it can't fail (or in prototypes).
7. **Derive Common Traits**: Always add `#[derive(Debug, Clone, PartialEq)]` to your structs.
8. **Don't Fight the Borrow Checker**: If you're struggling, consider if your data ownership model is correct.
9. **State Machines as Enums**: Use enums to represent different states of your application.
10. **Small Modules**: Keep your files small and use Rust's module system to stay organized.

---

## Python → Rust Rosetta Stone

| Python | Rust |
|--------|------|
| `list` | `Vec<T>` |
| `dict` | `HashMap<K, V>` |
| `None` | `Option<T>` |
| `try...except` | `Result<T, E>` |
| `lambda x: x*2` | `|x| x * 2` |
| `[x for x in list]` | `list.iter().map().collect()` |
| `@dataclass` | `#[derive(Debug, Default)] struct` |
| `pip install X` | `cargo add X` |

---

## Mastering Rust: A 3-Month Roadmap

- **Month 1: The Basics**: Ownership, Borrowing, and basic Structs/Enums. Focus on making code compile.
- **Month 2: The Ecosystem**: Error handling (`anyhow`/`thiserror`), Serialization (`serde`), and CLI tools (`clap`).
- **Month 3: Performance & Concurrency**: Async with `tokio`, Parallelism with `rayon`, and Writing PyO3 extensions.

---

## Final Word

Moving from Python to Rust feels like moving from a **bicycle** to a **jet engine**. It's harder to learn, and there are more safety checks, but once you're in the air, you can go faster and farther than ever before.

**Happy Hacking! 🦀**

***
