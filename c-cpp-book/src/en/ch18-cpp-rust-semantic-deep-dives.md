# 18. C++ to Rust Semantic Deep Dives 🟢

Transitioning from C++ to Rust involves more than just learning new syntax; it requires a shift in how you think about memory, types, and program structure. This chapter dives deep into the semantic differences and mappings between C++ and Rust.

### 1. RAII (Resource Acquisition Is Initialization)
Both C++ and Rust use RAII to manage resources. In C++, this is tied to constructors and destructors. In Rust, it's tied to ownership and the `Drop` trait.

```rust
struct MyResource {
    name: String,
}

impl Drop for MyResource {
    fn drop(&mut self) {
        println!("Dropping resource: {}", self.name);
        // Resource cleanup happens here automatically
    }
}

fn main() {
    {
        let _res = MyResource { name: String::from("File handle") };
    } // `_res` goes out of scope and is dropped here
}
```

---

### 2. Move Semantics
In C++, move semantics are explicit (using `std::move`) and objects are left in a "valid but unspecified" state. In Rust, **moves are the default** and the compiler prevents you from using an object after it has been moved.

```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1; // s1 is MOVED to s2

    // println!("{}", s1); // ERROR: s1 has been moved
    println!("{}", s2); // OK
}
```

---

### 3. Smart Pointers
Rust's smart pointers map closely to C++ ones, but with stricter safety rules.

| C++ | Rust | Description |
|-----|------|-------------|
| `std::unique_ptr<T>` | `Box<T>` | Single ownership on the heap. |
| `std::shared_ptr<T>` | `Arc<T>` | Thread-safe reference counting. |
| `std::weak_ptr<T>` | `Weak<T>` | Non-owning reference to an `Arc`. |
| `T*` (raw pointer) | `*const T`, `*mut T` | Unsafe pointers, used sparingly. |

---

### 4. Zero-Cost Abstractions
Both languages pride themselves on "not paying for what you don't use." Rust's traits and generics are compiled to efficient machine code, often using monomorphization (similar to C++ templates).

```rust
fn print_it<T: std::fmt::Display>(item: T) {
    println!("{}", item);
}

fn main() {
    print_it(42);      // Compiled for i32
    print_it("hello"); // Compiled for &str
}
```

---

### Summary for C/C++ Developers
- **In C++**: You have immense power but also immense responsibility. Many "best practices" are guidelines that the compiler doesn't strictly enforce.
- **In Rust**: The compiler enforces memory safety and thread safety by default. While this might feel restrictive at first, it allows you to build complex systems with a level of confidence that is difficult to achieve in C++. Understanding these semantic mappings is the key to becoming a proficient Rust developer.

***
