# 2. Traits In Depth 🟡

> **What you'll learn:**
> - Associated types vs generic parameters — and when to use each
> - GATs, blanket impls, marker traits, and trait object safety rules
> - How vtables and fat pointers work under the hood
> - Extension traits, enum dispatch, and typed command patterns

## Associated Types vs Generic Parameters

Both let a trait work with different types, but they serve different purposes:

```rust
// --- ASSOCIATED TYPE: One implementation per type ---
trait Iterator {
    type Item; // Each iterator produces exactly ONE kind of item

    fn next(&mut self) -> Option<Self::Item>;
}

// A custom iterator that always yields i32 — there's no choice
struct Counter { max: i32, current: i32 }

impl Iterator for Counter {
    type Item = i32; // Exactly one Item type per implementation
    fn next(&mut self) -> Option<i32> {
        if self.current < self.max {
            self.current += 1;
            Some(self.current)
        } else {
            None
        }
    }
}

// --- GENERIC PARAMETER: Multiple implementations per type ---
trait Convert<T> {
    fn convert(&self) -> T;
}

// A single type can implement Convert for MANY target types:
impl Convert<f64> for i32 {
    fn convert(&self) -> f64 { *self as f64 }
}
impl Convert<String> for i32 {
    fn convert(&self) -> String { self.to_string() }
}
```

**When to use which**:

| Use | When |
|-----|------|
| **Associated type** | There's exactly ONE natural output/result per implementing type (e.g., `Iterator::Item`). |
| **Generic parameter** | A type can meaningfully implement the trait for MANY different types (e.g., `From<T>`). |

**Intuition**: If it makes sense to ask "what is the `Item` of this iterator?", use associated type. If it makes sense to ask "can this convert to `f64`? to `String`? to `bool`?", use a generic parameter.

```rust
// Real-world example: std::ops::Add
trait Add<Rhs = Self> {
    type Output; // Associated type — addition has ONE result type
    fn add(self, rhs: Rhs) -> Self::Output;
}

// Rhs is a generic parameter — you can add different types to Meters:
struct Meters(f64);
struct Centimeters(f64);

impl Add<Meters> for Meters {
    type Output = Meters;
    fn add(self, rhs: Meters) -> Meters { Meters(self.0 + rhs.0) }
}
impl Add<Centimeters> for Meters {
    type Output = Meters;
    fn add(self, rhs: Centimeters) -> Meters { Meters(self.0 + rhs.0 / 100.0) }
}
```

---

## Performance Comparison: Static vs Dynamic Dispatch

| Feature | Static Dispatch (`impl Trait`) | Dynamic Dispatch (`dyn Trait`) |
|---------|--------------------------------|--------------------------------|
| **Mechanism** | Monomorphization (Specialization) | Vtable (Fat Pointer) |
| **Call cost** | Zero (Inlinable) | One indirection (Point-to-vtable-to-code) |
| **Inlining** | ✅ Yes | ❌ No |
| **Binary size** | Larger (Multiple copies) | Smaller (Single copy) |
| **Mixed types** | ❌ No (`Vec<T>` where T is one type) | ✅ Yes (`Vec<Box<dyn Trait>>`) |

---

## Advanced Topic: GATs (Generic Associated Types)

Since Rust 1.65, associated types can have generic parameters. This is crucial for **lending iterators**:

```rust
trait LendingIterator {
    type Item<'a> where Self: 'a;
    fn next(&mut self) -> Option<Self::Item<'_>>;
}
```

This allows the iterator to yield items that borrow from the iterator itself.

---

## Key Takeaways: Traits

- Use **Supertraits** (`trait B: A`) when implementing B requires A.
- Use **Blanket Implementations** (`impl<T: A> B for T`) to automatically give B to everything that has A.
- Use **Extension Traits** to add methods to types you didn't define (e.g., adding `.toJson()` to `std::vec::Vec`).
- **Trait Objects** require "Object Safety" (no generic methods, no `Self` returns).

***
