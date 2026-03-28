# 7. Ownership and Borrowing 🟡

> **What you'll learn:**
> - Why Rust has ownership (and no GC!)
> - Move semantics vs Python's reference counting
> - Borrowing (`&` and `&mut`) and the single-writer rule
> - Lifetime basics and smart pointers (`Box`, `Rc`, `Arc`)

## Understanding Ownership

This is often the biggest hurdle for Python developers. In Python, you never think about who "owns" data; the Garbage Collector (GC) handles it. In Rust, every value has exactly **one owner**, and the compiler tracking this allows it to free memory without a GC.

### Python: Shared References
In Python, assignment copies the **reference**.
```python
a = [1, 2, 3]
b = a          # Both point to the SAME list
b.append(4)
print(a)        # [1, 2, 3, 4] — a changed too!
```

### Rust: Single Ownership (The Move)
In Rust, assignment **moves** the value.
```rust
let a = vec![1, 2, 3];
let b = a;           // Ownership MOVES to b. 'a' is now invalid.
// println!("{:?}", a); // ❌ Compile error!
```

## The Borrowing Rules

To use data without taking ownership, you **borrow** it using references (`&`).

1. **Many Immutable Borrows (`&T`)**: Multiple people can read the data at the same time.
2. **One Mutable Borrow (`&mut T`)**: Only one person can write to the data, and NO ONE can read it while they are writing.

```rust
let mut data = vec![1, 2, 3];

let r1 = &data; // Fine
let r2 = &data; // Fine (multiple readers)

// let r3 = &mut data; // ❌ Error! Cannot borrow as mutable while immutable exists.
```

## Move Semantics vs Reference Counting

| Concept | Python | Rust |
|---------|--------|------|
| Simple Types | Copied (ints, floats) | `Copy` types (i32, f64) |
| Complex Types | Shared Reference | **Moved** (Ownership transfer) |
| Memory Cleanup | GC (Ref count + Cycle) | **Deterministic** (at end of scope) |
| Deep Copy | `copy.deepcopy(x)` | `x.clone()` |

## Smart Pointers (Opt-in Shared Ownership)

If you *really* need shared ownership like Python, you can opt-in using smart pointers:

- **`Box<T>`**: Simple heap allocation.
- **`Rc<T>`**: Reference Counting (like Python). Single-threaded only.
- **`Arc<T>`**: Atomic Reference Counting. Safe for multi-threading.
- **`RefCell<T>`**: "Interior Mutability" — check borrow rules at runtime instead of compile time.

---

## Exercises

<details>
<summary><strong>🏋️ Exercise: Fix the Borrow Checker</strong></summary>

**Challenge**: Fix the following code without using `.clone()`.

```rust
fn main() {
    let mut names = vec!["Alice".to_string()];
    let first = &names[0];     // Immutable borrow
    names.push("Bob".to_string()); // ❌ Mutation while borrowed!
    println!("{first}");
}
```

<details>
<summary>🔑 Solution</summary>

```rust
fn main() {
    let mut names = vec!["Alice".to_string()];
    {
        let first = &names[0];
        println!("{first}"); // Use the borrow before it conflicts
    } // first goes out of scope here
    names.push("Bob".to_string()); // Now it's safe to mutate
}
```

</details>
</details>

***
