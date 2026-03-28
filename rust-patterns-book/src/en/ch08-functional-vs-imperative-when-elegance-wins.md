# 8. Functional vs. Imperative: When Elegance Wins 🟡

> **What you'll learn:**
> - The core principle: transforming pipelines vs managing state.
> - `Option` and `Result` combinator families.
> - Iterator chains vs for loops: a decision framework.
> - Scoped mutability: internal mutation, external immutability.

## The Core Principle

- **Functional style** shines when you're *transforming data through a pipeline*.
- **Imperative style** shines when you're *managing complex control flow and side effects*.

Most idiomatic Rust code combines both.

---

## Option & Result Combinators

Instead of nested `if let` blocks, use combinators:

| You write... | Instead of... |
|---|---|
| `opt.unwrap_or(default)` | `if let Some(x) = opt { x } else { default }` |
| `opt.map(f)` | `match opt { Some(x) => Some(f(x)), None => None }` |
| `opt.and_then(f)` | `match opt { Some(x) => f(x), None => None }` |
| `res.map_err(f)` | `match res { Ok(x) => Ok(x), Err(e) => Err(f(e)) }` |

---

## Iterators vs Loops

### Use Iterators when:
- Each step is a simple transformation (`filter`, `map`).
- You are computing a single aggregate value (`sum`, `fold`).
- The pipeline is readable (under 4-5 steps).

```rust
let results: Vec<_> = data.iter()
    .filter(|item| item.active)
    .map(|item| item.score)
    .collect();
```

### Use Loops when:
- You need to build multiple outputs simultaneously.
- You have complex side effects (like multi-branch logging/alerts).
- You need to implement a state machine with early exits.

---

## Scoped Mutability

You can keep mutation local to a construction phase for better safety:

```rust
let samples = {
    let mut buf = Vec::new();
    // complex logic with loops, early breaks, and mutation
    buf.push(1);
    buf.push(2);
    buf
}; // buf is moved out and becomes immutable 'samples'
```

This pattern ensures that `samples` cannot be accidentally modified later in the function.

---

## Performance: Zero-Cost Abstractions

In Rust, **iterator chains compile to the same machine code as hand-written loops.** The only common performance overhead is if you use unnecessary `.collect()` calls in the middle of a pipeline.

***
