# 4. PhantomData — Types That Carry No Data 🔶

> **What you'll learn:**
> - Why `PhantomData<T>` exists and the three problems it solves
> - Lifetime branding for compile-time scope enforcement
> - The unit-of-measure pattern for dimension-safe arithmetic
> - Variance (covariant, contravariant, invariant) and how PhantomData controls it

## What PhantomData Solves

`PhantomData<T>` is a zero-sized type that tells the compiler "this struct is logically associated with `T`, even though it doesn't contain a `T`." It affects variance, drop checking, and auto-trait inference — without using any memory.

```rust
use std::marker::PhantomData;

struct Slice<'a, T> {
    ptr: *const T,
    len: usize,
    _marker: PhantomData<&'a T>,
    // Now the compiler knows:
    // 1. This struct borrows data with lifetime 'a
    // 2. It's covariant over 'a (lifetimes can shrink)
    // 3. Drop check considers T
}
```

### 1. Lifetime Branding

Use `PhantomData` to prevent mixing values from different "sessions" or "contexts":

```rust
struct ArenaHandle<'arena> {
    index: usize,
    _brand: PhantomData<&'arena ()>,
}
```

This ensures that a handle from `Arena A` cannot be used with `Arena B`, even if they have the same internal representation.

### 2. Unit-of-Measure Pattern

Prevent mixing incompatible units at compile time, with zero runtime cost:

```rust
struct Meters;
struct Seconds;

struct Quantity<Unit> {
    value: f64,
    _unit: PhantomData<Unit>,
}

let dist = Quantity::<Meters>::new(100.0);
let time = Quantity::<Seconds>::new(10.0);
// let sum = dist + time; // ❌ Compile error: Meters != Seconds
```

---

## Variance: Why It Matters

**Variance** determines if a generic type can be substituted with a sub- or super-type.

| Variance | "Can I substitute..." | Rust example |
|----------|-----------------------|--------------|
| **Covariant** | `'long` where `'short` expected ✅ | `&'a T` |
| **Contravariant** | `'short` where `'long` expected ✅ | `fn(T)` (parameter) |
| **Invariant** | No substitution allowed ❌ | `&mut T`, `Cell<T>` |

### PhantomData Variance Cheat Sheet

| PhantomData type | Variance over `T` | Variance over `'a` |
|------------------|-------------------|--------------------|
| `PhantomData<T>` | Covariant | — |
| `PhantomData<&'a T>` | Covariant | Covariant |
| `PhantomData<&'a mut T>` | **Invariant** | Covariant |
| `PhantomData<*mut T>` | **Invariant** | — |
| `PhantomData<fn(T)>` | **Contravariant** | — |

> **Practical Rule**: Default to `PhantomData<&'a T>` (covariant). Only use `&'a mut T` (invariant) if you hand out mutable access to the internal data.

***
