# 5. Data Structures 🟢

This chapter covers Rust's fundamental data structures. We will explore how Rust manages collections and composite types, many of which map directly to C/C++ concepts but with added safety guarantees.

---

### 1. Arrays and Tuples

#### Arrays
Arrays have a fixed number of elements of the same type, determined at compile time. They are stored on the stack.
```rust
fn main() {
    let a: [i32; 3] = [1, 2, 3];
    let b = [0; 5]; // [0, 0, 0, 0, 0]
    
    println!("First element: {}", a[0]);
    // a[10]; // ❌ Runtime Panic (Bounds Check)
}
```

#### Tuples
Tuples have a fixed size and can group different types.
```rust
fn main() {
    let t: (i32, f64, &str) = (500, 6.4, "hello");
    let (x, y, z) = t; // Destructuring
    
    println!("Value of y: {}", t.1); // Access via dot notation
}
```

---

### 2. Slices
Slices are a view into a contiguous sequence of elements in a collection. They are "fat pointers" containing a pointer to the data and a length.
```rust
fn main() {
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..4]; // [2, 3, 4]
    
    println!("Length: {}", slice.len());
}
```

---

### 3. Strings: `String` vs `&str`
Rust has two main string types.

| Aspect | `String` | `&str` (String Slice) |
|--------|----------|-----------------------|
| **Memory** | Heap-allocated, Growable | Borrowed, Fixed-size |
| **Ownership** | Owned | Borrowed (Reference) |
| **C++ Equivalent** | `std::string` | `std::string_view` |

```rust
fn main() {
    let s_slice: &str = "Hello"; // String literal (stored in binary)
    let mut s_owned = String::from("Hello"); // Heap allocated
    s_owned.push_str(", world!");
    
    let borrow: &str = &s_owned; // Borrow String as &str
}
```

---

### 4. Structs

#### Named-Field Structs
```rust
struct User {
    username: String,
    active: bool,
}

fn main() {
    let user1 = User {
        username: String::from("alice"),
        active: true,
    };
}
```

#### Tuple Structs
Useful for creating distinct types from primitives (the "Newtype" pattern).
```rust
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

let black = Color(0, 0, 0);
let origin = Point(0, 0, 0);
```

---

### 5. Collections: `Vec` and `HashMap`

#### `Vec<T>` (Vector)
A growable, heap-allocated array. Equivalent to `std::vector` in C++.
```rust
fn main() {
    let mut v = vec![1, 2, 3];
    v.push(4);
    
    // Pathological indexing
    let third = &v[2]; // Can panic
    let safe_third = v.get(2); // Returns Option<&T>
}
```

#### `HashMap<K, V>`
Key-value pairs. Equivalent to `std::unordered_map` in C++.
```rust
use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
}
```

---

### Deep Dive: C++ vs Rust References
For C++ developers, it's important to note:
1. **No Rvalue/Universal References**: Rust uses ownership and moves by default instead of `&&`.
2. **Bitwise Moves**: Moving in Rust is always a shallow `memcpy`. There are no move constructors. 
3. **Auto-Deref**: The compiler can automatically dereference through layers of pointers (e.g., `Box<String>` to `&str`) using the `Deref` trait.

***
