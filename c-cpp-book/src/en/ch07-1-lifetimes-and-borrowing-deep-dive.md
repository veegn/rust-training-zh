# 7.1 Lifetimes and Borrowing Deep Dive 🟢

Lifetimes are the way Rust's compiler ensures that all borrows are valid. Every reference in Rust has a **lifetime**, which is the scope for which that reference is valid.

### 1. The Borrow Checker
The borrow checker compares scopes to determine whether all borrows are valid.

```rust
fn main() {
    let r;                // ---------+-- 'a
                          //          |
    {                     //          |
        let x = 5;        // --+-- 'b |
        r = &x;           //   |      |
    }                     // --+      |
                          //          |
    // println!("r: {r}"); // ❌ ERROR: `x` does not live long enough
}                         // ---------+
```

---

### 2. Lifetime Elision Rules
In most cases, you don't need to write explicit lifetime annotations because the compiler follows three deterministic rules:

1. **Rule 1**: Each parameter that is a reference gets its own lifetime parameter.
2. **Rule 2**: If there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters.
3. **Rule 3**: If there are multiple input lifetime parameters, but one of them is `&self` or `&mut self`, the lifetime of `self` is assigned to all output lifetime parameters.

---

### 3. Explicit Lifetime Annotations
Sometimes the compiler needs your help to understand the relationship between multiple references. Annotations use a `'` prefix (e.g., `'a`).

#### Function Signatures
```rust
// This tells the compiler that the returned reference will live
// as long as the shorter of the two inputs.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

#### Struct Definitions
If a struct holds a reference, it **must** define a lifetime for that reference.
```rust
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
}
```

---

### 4. The `'static` Lifetime
The `'static` lifetime is a special lifetime that lasts for the **entire duration** of the program. All string literals have the `'static` lifetime.

```rust
let s: &'static str = "I have a static lifetime.";
```

---

### Summary for C/C++ Developers
- **In C/C++**: You manually track pointer validity. If you get it wrong, you get a segfault or UB.
- **In Rust**: You (sometimes) annotate the relationships between references. If you get it wrong, the code **won't compile**.

***
