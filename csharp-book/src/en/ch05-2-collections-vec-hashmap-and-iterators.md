# Vectors, HashMaps, and Iterators

> **What you'll learn:** `Vec<T>` vs `List<T>`, `HashMap` vs `Dictionary`, safe access patterns (why Rust returns `Option` instead of throwing), and the ownership implications of collections.
>
> **Difficulty:** 🟢 Beginner

## `Vec<T>` vs `List<T>`
`Vec<T>` is Rust's equivalent to C#'s `List<T>`, but with ownership semantics.

### C# vs Rust
In C#, passing a `List<T>` to a method passes a reference. In Rust, passing a `Vec<T>` moves ownership unless you pass a reference (`&Vec<T>`).

```rust
let mut numbers = vec![1, 2, 3];
process_vec(numbers); 
// numbers is no longer accessible here!

let mut numbers = vec![1, 2, 3];
process_vec_borrowed(&mut numbers);
// numbers is still accessible!
```

### Safe Access
Rust encourages avoiding exceptions. While `vec[index]` will panic if the index is out of bounds (similar to an exception), `vec.get(index)` returns an `Option`.

```rust
let first = vec.get(0); // Some(&value) or None
```

---

## HashMap vs Dictionary
`HashMap` is Rust's equivalent to C#'s `Dictionary<K, V>`.

### Common Operations
| **Action** | **C# Dictionary** | **Rust HashMap** |
| :--- | :--- | :--- |
| **Add/Update** | `dict["key"] = val` | `map.insert(key, val)` |
| **Check Key** | `dict.ContainsKey(key)` | `map.contains_key(key)` |
| **Remove** | `dict.Remove(key)` | `map.remove(key)` |
| **Safe Get** | `dict.TryGetValue(key, out v)` | `map.get(key)` -> `Option<&V>` |

### The Entry API
Rust has a unique and powerful "Entry API" for manipulating Map contents efficiently.
```rust
// Insert 42 only if "key" doesn't exist
map.entry("key".to_string()).or_insert(42);
```

---

## Iterators (LINQ for Rust)
Rust iterators provide functionality similar to LINQ, but they are built into the core language and are extremely high-performance.

### LINQ to Rust Mapping
| **LINQ** | **Rust Iterator** | **Note** |
| :--- | :--- | :--- |
| `.Select(x => ...)` | `.map(|x| ...)` | Lazy |
| `.Where(x => ...)` | `.filter(|x| ...)` | Lazy |
| `.ToList()` | `.collect::<Vec<_>>()` | Eager (consumes) |
| `.Take(n)` | `.take(n)` | Lazy |
| `.FirstOrDefault()` | `.next()` | |

### Iterator Types
1.  **`.iter()`**: Borrows elements (`&T`).
2.  **`.iter_mut()`**: Borrows elements mutably (`&mut T`).
3.  **`.into_iter()`**: Consumes the collection and takes ownership of elements (`T`).

---

## Exercise: LINQ to Iterators
**Challenge:** Translate a C# LINQ query (Filter, Sort, Select, Take) to Rust.

```rust
fn top_students(students: &mut [Student]) -> Vec<String> {
    students.sort_by(|a, b| b.grade.cmp(&a.grade)); // Sort eager
    students.iter()
        .filter(|s| s.grade >= 90)
        .take(3)
        .map(|s| format!("{}: {}", s.name, s.grade))
        .collect()
}
```
**Key Difference:** Rust iterators are lazy, but sorting in Rust is typically an eager, in-place operation. You sort first, then perform the lazy transformations.
