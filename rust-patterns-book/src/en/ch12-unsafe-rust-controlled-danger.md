# 12. Unsafe Rust — Controlled Danger 🔶

> **What you'll learn:**
> - The five unsafe superpowers.
> - Writing sound abstractions: safe API, unsafe internals.
> - FFI patterns (Calling C from Rust).
> - Custom allocators: Arena and Slab patterns.

## The Five Unsafe Superpowers

`unsafe` allows you to:
1. Dereference a raw pointer (`*const T`, `*mut T`).
2. Call an `unsafe` function or method.
3. Access or modify a mutable static variable.
4. Implement an `unsafe` trait.
5. Access fields of a `union`.

> **Key Rule**: `unsafe` does not disable the borrow checker. It only enables these 5 specific actions.

---

## Writing Sound Abstractions

The goal of `unsafe` is to wrap dangerous low-level operations in a **Safe API**.

```rust
pub struct MyBuffer<T, const N: usize> {
    data: [MaybeUninit<T>; N],
    len: usize,
}

impl<T, const N: usize> MyBuffer<T, N> {
    pub fn get(&self, index: usize) -> Option<&T> {
        if index < self.len {
            // SAFETY: index is checked to be within initialized range.
            Some(unsafe { self.data[index].assume_init_ref() })
        } else {
            None
        }
    }
}
```

---

## FFI: Calling C Code

Use `extern "C"` and `#[repr(C)]` to interface with C libraries.

```rust
extern "C" {
    fn strlen(s: *const c_char) -> usize;
}

pub fn safe_strlen(s: &str) -> usize {
    let c_str = CString::new(s).unwrap();
    // SAFETY: c_str is a valid null-terminated C string.
    unsafe { strlen(c_str.as_ptr()) }
}
```

---

## Custom Allocators

- **Arena (Bump) Allocator**: Allocates by moving a pointer. Extremely fast (~2ns). Memory is freed all at once when the arena is dropped. Perfect for request-scoped data.
- **Slab Allocator**: A pool of fixed-size slots. O(1) allocation and deallocation. Prevents fragmentation.

| Pattern | C Equivalent | Rust Tool |
|---------|--------------|-----------|
| Arena | `obstack` | `bumpalo` |
| Slab | `kmem_cache` | `slab` |
| Local Pool | `alloca` | `FixedVec` (custom) |

***
