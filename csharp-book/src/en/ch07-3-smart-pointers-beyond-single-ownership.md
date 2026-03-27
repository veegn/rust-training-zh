# Smart Pointers: Beyond Single Ownership

> **What you'll learn:** `Box<T>`, `Rc<T>`, `Arc<T>`, `RefCell<T>`, and `Cow<'a, T>` - when to use each, `Drop` as Rust's `IDisposable`, and a decision tree for choosing the right smart pointer.
>
> **Difficulty:** Advanced

In C#, every object is essentially reference-counted by the GC. In Rust, single ownership is the default—but sometimes you need shared ownership, heap allocation, or interior mutability.

---

## The Common Smart Pointers

### 1. `Box<T>` (Heap Allocation)
The most basic smart pointer. Use it when you need to put a value on the heap instead of the stack.
*   **Use case:** Recursive data structures (whose size isn't known at compile time).
```rust
enum List {
    Cons(i32, Box<List>),
    Nil,
}
```

### 2. `Rc<T>` (Reference Counting)
Allows **multiple owners** for the same data on a single thread.
*   **Use case:** Graph nodes or shared configurations.
```rust
let shared = Rc::new(vec![1, 2, 3]);
let a = Rc::clone(&shared);
let b = Rc::clone(&shared); // Ref count is now 3
```

### 3. `Arc<T>` (Atomic Reference Counting)
The thread-safe version of `Rc<T>`.
*   **Use case:** Sharing data across multiple threads.
```rust
let shared_data = Arc::new(vec![10, 20]);
thread::spawn(move || {
    println!("{:?}", shared_data);
});
```

### 4. `RefCell<T>` (Interior Mutability)
Allows you to mutate data even when you have an immutable reference (`&T`) to the container. It moves borrow checking from compile-time to **runtime**.
*   **Use case:** Mock objects or complex state where single mutability is hard to express.

---

## `Drop`: Rust's `IDisposable`
In C#, you use `using` and `IDisposable` to clean up resources. In Rust, you implement the `Drop` trait. The key difference is that `Drop` is **automatic**.

```rust
struct TempFile { path: String }

impl Drop for TempFile {
    fn drop(&mut self) {
        println!("Deleting {}", self.path);
        // Guaranteed cleanup when 'temp' goes out of scope
    }
}
```
**Takeaway:** You never "forget" to dispose in Rust. As soon as the owner is gone, the resource is cleaned up.

---

## Decision Tree

| **Need** | **Smart Pointer** |
| :--- | :--- |
| **Heap allocation (Single Owner)** | `Box<T>` |
| **Shared Ownership (Single Thread)** | `Rc<T>` |
| **Shared Ownership (Multi-Thread)** | `Arc<T>` |
| **Mutation via `&T` (Single Thread)** | `RefCell<T>` |
| **Mutation via `&T` (Multi-Thread)** | `Mutex<T>` or `RwLock<T>` |

---

## Exercise: Choose the Smart Pointer
**Challenge:** Which pointer would you use for a shared configuration object accessed by multiple threads?

**Answer:** `Arc<T>`. If it needs to be updated, use `Arc<Mutex<T>>`.
**Takeaway:** Start with the simplest pointer and only upgrade when the compiler (or your architectural needs) requires it.
