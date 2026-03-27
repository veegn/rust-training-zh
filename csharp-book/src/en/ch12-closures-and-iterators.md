# Closures and Iterators

> **What you'll learn:** Closures with ownership-aware captures (`Fn`/`FnMut`/`FnOnce`) vs C# lambdas, Rust iterators as a zero-cost replacement for LINQ, and lazy evaluation.
>
> **Difficulty:** Intermediate

In C#, you use Lambdas and LINQ for data processing. Rust provides **Closures** and **Iterators** that offer similar functional power but with zero runtime overhead and explicit memory management.

---

## Closures vs Lambdas
Rust closures are like C# lambdas but must handle **Ownership**.

### C# Lambda (Captured by Reference)
```csharp
int multiplier = 3;
Func<int, int> multiply = x => x * multiplier;
```

### Rust Closure (Capture Modes)
Rust closures can borrow or take ownership of variables from their environment.
```rust
let multiplier = 3;
let multiply = |x| x * multiplier; // Borrows 'multiplier'

let data = vec![1, 2, 3];
let owns_data = move || println!("{:?}", data); // Takes ownership of 'data'
```

### The Three Traits
1.  **`Fn`**: Borrows values immutably (read-only).
2.  **`FnMut`**: Borrows values mutably (can modify).
3.  **`FnOnce`**: Consumes the values (can only be called once).

---

## Iterators: The LINQ of Rust
Rust iterators are **lazy** and **zero-cost**. They compile down to the same machine code as a manual `for` loop.

### C# LINQ
```csharp
var result = numbers
    .Where(n => n % 2 == 0)
    .Select(n => n * n)
    .ToList();
```

### Rust Iterators
```rust
let result: Vec<i32> = numbers.iter()
    .filter(|&&n| n % 2 == 0)
    .map(|&n| n * n)
    .collect(); // 'collect' is the 'ToList' of Rust
```

---

## Key Differences from LINQ
1.  **Laziness**: Nothing happens until you call a "terminal" method like `collect`, `sum`, or `find`.
2.  **Efficiency**: Rust's iterator chains are often faster than manual loops because the compiler can optimize across the entire chain.
3.  **Ownership**: You must decide whether to iterate over references (`iter()`), mutable references (`iter_mut()`), or consume the collection (`into_iter()`).

---

## Summary for C# Developers
| **Concept** | **C# / LINQ** | **Rust Iterator** |
| :--- | :--- | :--- |
| **Mapping** | `.Select()` | `.map()` |
| **Filtering** | `.Where()` | `.filter()` |
| **Folding** | `.Aggregate()` | `.fold()` |
| **Execution** | Eager/Lazy mix | Strictly Lazy |
| **Materialize** | `.ToList()`, `.ToArray()` | `.collect::<Vec<_>>()` |

---

## Exercise: Filter and Transform
**Challenge:** Given a list of names, filter out those shorter than 5 characters, convert the rest to uppercase, and collect them into a new vector.

```rust
let names = vec!["Alice", "Bob", "Charlie", "Dave"];
let result: Vec<String> = names.iter()
    .filter(|n| n.len() >= 5)
    .map(|n| n.to_uppercase())
    .collect();
```
**Takeaway:** Iterators are the idiomatic way to process collections in Rust. They combine the readability of functional programming with the performance of systems programming.
