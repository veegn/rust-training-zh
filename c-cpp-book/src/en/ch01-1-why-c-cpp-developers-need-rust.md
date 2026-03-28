# 1.1 Why C/C++ Developers Need Rust 🟢

Safe Rust **structurally prevents** the most common and dangerous bugs in systems programming. This isn't just a recommendation—it's enforced by the compiler.

### What Rust Eliminates

| Issue | How Rust Prevents It |
|-------|----------------------|
| **Buffer Overflows** | All arrays and slices are bounds-checked at runtime. |
| **Dangling Pointers** | The lifetime system ensures references never outlive their data. |
| **Use-After-Free** | Ownership rules make it impossible to use memory after it's freed. |
| **Use-After-Move** | Moves are destructive; the original variable becomes inaccessible. |
| **NULL Dereferences** | Rust has no null pointers; `Option<T>` forces explicit handling. |
| **Data Races** | `Send` and `Sync` traits ensure thread safety at compile time. |
| **Iterator Invalidation** | The borrow checker prevents modifying a collection while iterating. |

---

### Why C++ Mitigations Aren't Enough

C++ introduced smart pointers and RAII to handle these issues, but they are often "bandaids" rather than structural cures.

#### Use-After-Move in C++ vs Rust
In C++, `std::move` leaves the original object in a "valid but unspecified state." You can still use it, potentially causing crashes or logic errors.
```cpp
auto vec2 = std::move(vec);
vec->size(); // Compiles! Runtime crash if vec was unique_ptr.
```
In Rust, this is a **compile-time error**:
```rust
let vec2 = vec; // move happens
// vec.len();  // error: use of moved value
```

#### Implicit Reference Cycles
C++ `shared_ptr` can easily create reference cycles that leak memory silently. Rust's `Rc<T>` and `Arc<T>` combined with `Weak<T>` make cycles explicit and breakable.

---

### Security by Design
Over **70% of security vulnerabilities** in large codebases (like Chrome or Windows) are memory safety issues. By eliminating these at compile time, Rust significantly reduces the security attack surface of your application.

***
