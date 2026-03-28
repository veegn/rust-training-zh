# 12. Closures and Iterators 🟡

> **What you'll learn:**
> - Rust closures (`|args| body`) vs Python lambdas
> - Iterator chains: The Rust version of List Comprehensions
> - Lazy evaluation: Why `collect()` is required
> - Macros: Rust's compile-time metaprogramming

## Closures: Beyond Lambdas

In Python, `lambda` is limited to a single expression. Rust closures can be multi-line and are much more powerful.

### Python Lambda
```python
double = lambda x: x * 2
```

### Rust Closure
```rust
let double = |x: i32| x * 2;

// Multi-line closure:
let complex = |x: i32| {
    let y = x + 10;
    y * 2
};
```

### Capturing the Environment
Use the `move` keyword to force the closure to take ownership of captured variables (essential for multi-threading).
```rust
let data = vec![1, 2, 3];
let closure = move || println!("{data:?}");
// data is now moved into the closure and cannot be used here!
```

---

## Iterator Chains: The Rust "List Comprehension"

Rust doesn't have list comprehension syntax (`[x for x in list]`). Instead, it uses **Iterator Chains**.

| Python | Rust Iterator Chain |
|--------|----------------------|
| `[x*x for x in nums]` | `nums.iter().map(|x| x * x).collect()` |
| `[x for x in nums if x > 0]` | `nums.iter().filter(|x| x > 0).collect()` |
| `any(x > 0 for x in nums)` | `nums.iter().any(|x| x > 0)` |

### Why .collect()?
Rust iterators are **lazy**. `map` and `filter` don't actually process any data until you call a "consuming" method like `.collect()`, `.sum()`, or `.count()`.

```rust
let nums = vec![1, 2, 3];
let doubled = nums.iter().map(|x| x * 2); // Nothing has happened yet!
let result: Vec<_> = doubled.collect();   // Now it's processed.
```

---

## Macros: Compile-time Code Generation

Python uses decorators and metaclasses for metaprogramming. Rust uses **Macros** to generate code at compile time.

### Common Macros
- `println!("...")`: Print with formatting.
- `vec![1, 2, 3]`: Create a Vector.
- `panic!("...")`: Crash the program (like `raise Exception`).
- `todo!()`: Placeholder for unfinished code.

### The `dbg!` Macro
The `dbg!` macro is a lifesaver. It prints the file, line number, and the value of an expression.
```rust
let x = 5;
dbg!(x * 2); // Prints "[src/main.rs:2] x * 2 = 10"
```

---

## Exercises

<details>
<summary><strong>🏋️ Exercise: Squares of Evens</strong></summary>

**Challenge**: Given a vector `vec![1, 2, 3, 4, 5, 6]`, use an iterator chain to:
1. Filter only even numbers.
2. Square each number.
3. Collect them into a new `Vec<i32>`.

<details>
<summary>🔑 Solution</summary>

```rust
fn main() {
    let nums = vec![1, 2, 3, 4, 5, 6];
    let result: Vec<i32> = nums.iter()
        .filter(|&x| x % 2 == 0)
        .map(|x| x * x)
        .collect();
    
    println!("{result:?}"); // [4, 16, 36]
}
```

</details>
</details>

***
