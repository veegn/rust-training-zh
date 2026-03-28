# 5. Data Structures and Collections 🟢

> **What you'll learn:**
> - Tuples and destructuring vs Python
> - Arrays (fixed length) and Slices (views)
> - Structs: Rust's data-focused replacement for classes
> - `Vec<T>` vs `list` and `HashMap<K, V>` vs `dict`
> - The mapping from Python "dunder" methods to Rust Traits

## Tuples and Destructuring

### Python Tuples
```python
point = (3.0, 4.0)
x, y = point  # Unpacking
```

### Rust Tuples
```rust
let point: (f64, f64) = (3.0, 4.0);
let (x, y) = point; // Destructuring

// Access by index (uses .0, .1)
let first = point.0;
```

---

## Arrays and Slices

Rust distinguishes between **owning data** and **viewing data**.

### 1. Array (Fixed Size)
```rust
let numbers: [i32; 5] = [1, 2, 3, 4, 5]; // Size is part of the type
```

### 2. Slice (A Borrowed View)
In Python, `data[1:4]` creates a **copy**. In Rust, `&data[1..4]` creates a **view** (no copy, no allocation).
```rust
let data = [10, 20, 30, 40, 50];
let slice: &[i32] = &data[1..4]; // [20, 30, 40] — zero cost view
```

---

## Structs vs Classes

Rust doesn't have classes or inheritance. It uses **structs** for data and **traits** for behavior.

### Python Class (Data + Methods)
```python
@dataclass
class Rectangle:
    width: float
    height: float
    def area(self): return self.width * self.height
```

### Rust Struct (Data + impl)
```rust
struct Rectangle {
    width: f64,
    height: f64,
}

impl Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}
```

### Key Mapping: Dunder Methods → Traits
| Python | Rust |
|--------|------|
| `__str__` | `impl Display` |
| `__repr__` | `#[derive(Debug)]` |
| `__eq__` | `#[derive(PartialEq)]` |
| `__init__` | `fn new()` (convention) |
| `__del__` | `impl Drop` (automatic) |

---

## Common Collections

### 1. Vec<T> (The Python `list` equivalent)
```rust
let mut nums = vec![1, 2, 3];
nums.push(4);             // append
let last = nums.pop();    // pop (returns Option)
let len = nums.len();     // len()
```

### 2. HashMap<K, V> (The Python `dict` equivalent)
```rust
use std::collections::HashMap;

let mut scores = HashMap::new();
scores.insert("Alice", 100);

// Entry API (like defaultdict/setdefault)
*scores.entry("Bob").or_insert(0) += 10;
```

---

## Exercises

<details>
<summary><strong>🏋️ Exercise: Word Frequency Counter</strong></summary>

**Challenge**: Write a function that takes a sentence and returns a map of word frequencies. Python equivalent: `Counter(text.lower().split())`.

<details>
<summary>🔑 Solution</summary>

```rust
use std::collections::HashMap;

fn word_frequencies(text: &str) -> HashMap<String, usize> {
    let mut counts = HashMap::new();
    for word in text.split_whitespace() {
        let key = word.to_lowercase();
        *counts.entry(key).or_insert(0) += 1;
    }
    counts
}

fn main() {
    let text = "the quick brown fox jumps over the lazy fox";
    let freq = word_frequencies(text);
    println!("{freq:?}");
}
```

</details>
</details>

***
