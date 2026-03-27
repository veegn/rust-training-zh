# Lifetimes: Proving Reference Validity

> **What you'll learn:** Why lifetimes exist (the compiler needs proof of safety), lifetime annotation syntax (`'a`), elision rules (why you often don't need them), and struct lifetimes.
>
> **Difficulty:** Advanced

C# developers never think about reference lifetimes—the Garbage Collector (GC) handles reachability. In Rust, the compiler needs **proof** that every reference is valid for as long as it's used. Lifetimes are that proof.

---

## Why Lifetimes Exist
Consider a function that takes two references and returns one of them:
```rust
fn longest(a: &str, b: &str) -> &str {
    if a.len() > b.len() { a } else { b }
}
```
The compiler rejects this because it doesn't know if the returned reference borrows from `a` or `b`. If `b` goes out of scope while the caller is still using the result, you'd have a dangling pointer.

---

## Lifetime Annotation Syntax
You use the `'a` syntax to tell the compiler: "The returned reference will live at least as long as the inputs."

```rust
fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() > b.len() { a } else { b }
}
```
**Important:** Lifetime annotations do not change how long a value lives. They only **describe** the relationship between different references so the compiler can verify them.

---

## Lifetime Elision Rules
Most of the time, you don't need to write `'a`. The compiler applies three simple rules automatically:
1.  **Each input reference** gets its own lifetime.
2.  **If there is exactly one input reference**, its lifetime is assigned to all outputs.
3.  **If one input is `&self` or `&mut self`**, that lifetime is assigned to all outputs.

```rust
// The compiler automatically turns this:
fn first_word(s: &str) -> &str { ... }

// Into this:
fn first_word<'a>(s: &'a str) -> &'a str { ... }
```

---

## Struct Lifetimes
If a struct contains a reference, it **must** have a lifetime annotation. This ensures the struct cannot outlive the data it points to.

```rust
struct Excerpt<'a> {
    text: &'a str, 
}

let novel = String::from("Call me Ishmael.");
let first_sentence = Excerpt { text: &novel }; 
// 'first_sentence' cannot exist if 'novel' is dropped.
```

---

## The `'static` Lifetime
The `'static` lifetime means the reference **could** live for the entire duration of the program.
*   **String Literals**: `"Hello"` is always `&'static str` because it's baked into the program binary.
*   **Global Constants**: Also typically `'static`.

---

## Summary for C# Developers
| **Concept** | **C# Equivalent** | **Rust Reality** |
| :--- | :--- | :--- |
| **Object Lifetime** | Managed by GC | Defined by Scope/Ownership |
| **Reference Tracking** | Runtime (reachable?) | Compile-time (Life'a?) |
| **Ref to Local** | Forbidden/Safe via Box | Blocked by Borrow Checker |
| **Struct with Ref** | Not possible for classes | Requires `<'a>` |

---

## Exercise: Lifetime Annotations
**Challenge:** Add annotations to a struct and function where multiple references are involved.

```rust
struct Profile<'a> {
    username: &'a str,
}

fn get_username<'a>(p: &'a Profile) -> &'a str {
    p.username
}
```
**Takeaway:** While elision handles many cases, understanding `'a` is crucial for building complex data structures that borrow data for performance.
