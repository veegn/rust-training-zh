# 14. Unsafe Rust and FFI 🟢

Rust has a "safe" core, but also allows you to opt-out of some of its safety guarantees by using the `unsafe` keyword. This is necessary for low-level system programming, interfacing with other languages, or building high-performance data structures.

### 1. The `unsafe` Keyword
The `unsafe` keyword allows you to perform five actions that are not allowed in safe Rust:
- Dereference a raw pointer.
- Call an unsafe function or method.
- Access or modify a mutable static variable.
- Implement an unsafe trait.
- Access fields of `union`s.

```rust
fn main() {
    let mut num = 5;

    let r1 = &num as *const i32; // Raw pointer
    let r2 = &mut num as *mut i32;

    unsafe {
        println!("r1 is: {}", *r1); // Dereferencing raw pointer
        println!("r2 is: {}", *r2);
    }
}
```

---

### 2. Calling C Code from Rust (FFI)
Rust provides a Foreign Function Interface (FFI) to call functions from other languages like C.

```rust
extern "C" {
    fn abs(input: i32) -> i32;
}

fn main() {
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
}
```

---

### 3. Calling Rust Code from C
You can also export Rust functions to be called by C code by using the `extern` keyword and `#[no_mangle]` attribute.

```rust
#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
}
```

---

### 4. Creating Safe Wrappers
A common pattern in Rust is to wrap unsafe operations inside a safe function to provide a safer API for users.

```rust
use std::slice;

fn main() {
    let address = 0x01234usize;
    let r = address as *const i32;

    // Creating a slice from a raw pointer is unsafe
    let values: &[i32] = unsafe { slice::from_raw_parts(r, 10000) };
}
```

---

### Summary for C/C++ Developers
- **In C/C++**: Everything is "unsafe" by Rust's standards. You are responsible for manual memory management and ensuring no data races occur.
- **In Rust**: You specify exactly where you are bypassing safety checks by using `unsafe`. This makes it much easier to audit your code for potential memory safety issues. Most Rust code should be safe, and `unsafe` should be used sparingly and wrapped in safe abstractions.

***
