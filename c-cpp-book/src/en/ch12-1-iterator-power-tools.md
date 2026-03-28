# 12.1 Iterator Power Tools 🟢

Rust's iterator library is rich and powerful, allowing you to chain operations to process data in a functional and expressive way.

### 1. `enumerate`
Returns the current iteration count along with the value.

```rust
let v = vec!['a', 'b', 'c'];
for (i, val) in v.iter().enumerate() {
    println!("Index {} has value {}", i, val);
}
```

---

### 2. `zip`
Combines two iterators into a single iterator of pairs.

```rust
let names = vec!["Alice", "Bob"];
let ages = vec![25, 30];
let combined: Vec<_> = names.iter().zip(ages.iter()).collect();
// Result: [(&"Alice", &25), (&"Bob", &30)]
```

---

### 3. `map` and `filter`
- **`map`**: Transforms each element.
- **`filter`**: Keeps only elements that satisfy a condition.

```rust
let numbers = vec![1, 2, 3, 4, 5, 6];
let result: Vec<_> = numbers.into_iter()
    .filter(|x| x % 2 == 0) // Keep even numbers
    .map(|x| x * x)         // Square them
    .collect();
// Result: [4, 16, 36]
```

---

### 4. `flat_map`
Maps each element to an iterator and flattens the result.

```rust
let words = vec!["hello", "world"];
let chars: Vec<char> = words.into_iter()
    .flat_map(|s| s.chars())
    .collect();
// Result: ['h', 'e', 'l', 'l', 'o', 'w', 'o', 'r', 'l', 'd']
```

---

### 5. `fold`
Reduces the entire iterator to a single value using an initial value and a closure.

```rust
let numbers = vec![1, 2, 3, 4];
let sum = numbers.iter().fold(0, |acc, x| acc + x);
// Result: 10
```

---

### Summary for C/C++ Developers
- **In C++**: You use `<algorithm>` functions like `std::transform`, `std::copy_if`, or loops. Chaining these is often clunky.
- **In Rust**: Iterators are designed for chaining. They are **lazy**, meaning they don't do anything until you call a "terminal" method like `collect()`, `sum()`, or `for_each()`.

***
