# Unsafe Rust and FFI

> **What you'll learn:** What `unsafe` permits (raw pointers, FFI, unchecked casts), safe wrapper patterns, and C# P/Invoke vs Rust FFI.
>
> **Difficulty:** Advanced

Rust is famous for its safety, but sometimes you need to step outside the borrow checker's rules to talk to hardware, call C libraries, or build low-level data structures. This is where `unsafe` comes in.

---

## What is `unsafe`?
The `unsafe` keyword doesn't turn off the borrow checker; it just gives you five "superpowers":
1.  Dereference a **raw pointer** (`*const T`, `*mut T`).
2.  Call an `unsafe` function.
3.  Access or modify a **mutable static** variable.
4.  Implement an **unsafe trait**.
5.  Access fields of a `union`.

### Rust Example: Raw Pointers
```rust
let mut num = 5;
let r1 = &num as *const i32;
let r2 = &mut num as *mut i32;

unsafe {
    println!("r1 is: {}", *r1);
    *r2 = 10;
}
```

---

## The "Safe Wrapper" Pattern
The goal in Rust isn't to *avoid* `unsafe` at all costs, but to **encapsulate** it. You write a small amount of `unsafe` code and wrap it in a 100% safe API.

```rust
pub fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    let ptr = values.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (
            std::slice::from_raw_parts_mut(ptr, mid),
            std::slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}
```

---

## Interop with C# (FFI)
You can call Rust from C# using **P/Invoke**. To do this, Rust must export functions using the C calling convention.

### Rust Side (`lib.rs`)
```rust
#[no_mangle]
pub extern "C" fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

### C# Side
```csharp
[DllImport("my_rust_lib")]
public static extern int add(int a, int b);
```

---

## Summary for C# Developers
| **Concept** | **C# Equivalent** | **Rust Reality** |
| :--- | :--- | :--- |
| **Unsafe Code** | `unsafe { ... }` | `unsafe { ... }` |
| **Pointers** | `int* ptr` | `*mut i32` |
| **P/Invoke** | `DllImport` | `extern "C"` |
| **Pinning** | `fixed` statement | `Box::into_raw` / `Box::from_raw` |
| **Memory Leak** | GC handles most things | `Drop` trait + `unsafe` cleanup |

---

## Exercise: Write an Unsafe Function
**Challenge:** Write an `unsafe` function that swaps two integers using raw pointers. Then, call it from a safe wrapper.

```rust
unsafe fn raw_swap(a: *mut i32, b: *mut i32) {
    let temp = *a;
    *a = *b;
    *b = temp;
}

fn safe_swap(a: &mut i32, b: &mut i32) {
    unsafe { raw_swap(a, b); }
}
```
**Takeaway:** `unsafe` is a contract. The compiler trusts you that you've verified the safety invariants. By wrapping `unsafe` in safe functions, you keep the "unsafety" from leaking into the rest of your codebase.
