# 17.2 Avoiding Unchecked Indexing 🟢

In Rust, using the index operator (`[]`) on a collection like a `Vec` or `HashMap` will cause the program to panic if the index is out of bounds or the key is not present. This is a common source of crashes in Rust programs.

### 1. The Danger of `[]`
While `[]` is convenient, it assumes that the index or key is always valid. If this assumption is ever wrong, your program will crash.

```rust
fn main() {
    let v = vec![1, 2, 3];
    
    // BAD: this will panic because index 10 is out of bounds
    let x = v[10]; 
}
```

---

### 2. Use `.get()` Instead
The `.get()` method returns an `Option<&T>`, allowing you to handle the case where the index or key is missing without crashing.

```rust
fn main() {
    let v = vec![1, 2, 3];

    // GOOD: handle the missing index gracefully
    match v.get(10) {
        Some(x) => println!("Value is: {}", x),
        None => println!("Index is out of bounds!"),
    }

    // Or use unwrap_or for a default value
    let x = v.get(10).unwrap_or(&0);
}
```

---

### 3. Iterating instead of Indexing
Often, you can avoid indexing altogether by using an iterator. This is not only safer but often more efficient and idiomatic.

```rust
fn main() {
    let v = vec![1, 2, 3, 4, 5];

    // BAD: manual indexing is error-prone
    for i in 0..v.len() {
        println!("{}", v[i]);
    }

    // GOOD: use an iterator
    for x in &v {
        println!("{}", x);
    }
}
```

---

### 4. Boundary Checks and Slicing
If you need to work with a sub-section of a collection, use slicing with `.get(start..end)`. This also returns an `Option`, ensuring you don't accidentally cross a boundary.

```rust
fn main() {
    let v = vec![1, 2, 3, 4, 5];

    if let Some(sub_slice) = v.get(1..3) {
        println!("Sub-slice: {:?}", sub_slice); // [2, 3]
    }
}
```

---

### Summary for C/C++ Developers
- **In C/C++**: `v[i]` on a `std::vector` does not perform bounds checking, leading to undefined behavior (reading/writing arbitrary memory) if the index is out of bounds. You might use `v.at(i)` which throws an exception, but this is less common.
- **In Rust**: `v[i]` **always** performs bounds checking and panics on failure. This prevents memory safety issues but can still cause your program to exit. Using `.get()` is the Rust idiomatic way to handle potentially "out-of-bounds" access safely and explicitly.

***
