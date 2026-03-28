# 7. Ownership and Borrowing 🟢

Ownership is Rust's most unique feature and it enables Rust to make memory safety guarantees without needing a garbage collector.

### 1. The Three Rules of Ownership
1. Each value in Rust has a variable that’s called its **owner**.
2. There can only be **one owner** at a time.
3. When the owner goes out of scope, the value will be **dropped** (memory is freed).

---

### 2. Move Semantics
When you assign an owned value to another variable, the ownership is **moved**. The original variable becomes invalid.

```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1; // s1's ownership is moved to s2

    // println!("{s1}"); // ❌ Compile Error: value used after move
}
```

This is different from C++, where `std::move` leaves the original object in a "valid but unspecified" state. In Rust, the compiler strictly forbids access to the moved-from variable.

---

### 3. Borrowing (References)
Instead of transferring ownership, you can **borrow** a value by creating a reference (`&`).

#### The Rules of Borrowing
At any given time, you can have either:
- **Any number of immutable references** (`&T`).
- **Exactly one mutable reference** (`&mut T`).

And references must always be valid (they cannot outlive the owner).

```rust
fn main() {
    let mut s = String::from("hello");

    let r1 = &s; // Immutable borrow
    let r2 = &s; // Another immutable borrow
    println!("{r1} and {r2}"); // ✅ Works

    // let r3 = &mut s; // ❌ ERROR: cannot borrow as mutable because it's already borrowed as immutable
    
    {
        let r4 = &mut s; // ✅ Works: r1 and r2 are no longer used here
        r4.push_str(", world");
    }
}
```

---

### 4. The `Copy` Trait
Types that are stored entirely on the stack (like `i32`, `bool`, `f64`) don't have move semantics. They are **copied** instead.

```rust
fn main() {
    let x = 5;
    let y = x; // x is copied to y; both are valid
    println!("x={x}, y={y}"); // ✅ Works
}
```

---

### 5. The `Drop` Trait (RAII)
The `Drop` trait allows you to customize what happens when a value goes out of scope. This is equivalent to a **Destructor** in C++.

```rust
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn main() {
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    println!("CustomSmartPointer created.");
} // `c` goes out of scope here, and `drop` is called automatically.
```

***
