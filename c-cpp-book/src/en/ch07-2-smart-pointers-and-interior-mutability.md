# 7.2 Smart Pointers and Interior Mutability 🟢

Smart pointers are data structures that act like a pointer but also have additional metadata and capabilities (like reference counting).

### 1. `Box<T>` for Heap Allocation
The most straightforward smart pointer is a **box**, which allows you to store data on the heap rather than the stack.

```rust
fn main() {
    let b = Box::new(5); // 5 is on the heap
    println!("b = {b}");
} // b is dropped here, and the heap memory is freed.
```

Use `Box<T>` when:
- You have a type whose size can’t be known at compile time (recursive types).
- You want to transfer ownership of a large amount of data without copying.
- You want to own a value that implements a specific trait (trait objects).

---

### 2. `Rc<T>` for Shared Ownership
In some cases, a single value might have multiple owners. `Rc<T>` (Reference Counted) tracks the number of references to a value.

```rust
use std::rc::Rc;

fn main() {
    let a = Rc::new(String::from("hello"));
    let b = Rc::clone(&a); // Increases reference count
    let c = Rc::clone(&a); // Increases reference count again

    println!("Count after c: {}", Rc::strong_count(&a)); // 3
} // All go out of scope, count becomes 0, and memory is freed.
```

**Note**: `Rc<T>` is only for single-threaded scenarios. For multi-threading, use `Arc<T>`.

---

### 3. Interior Mutability: `Cell<T>` and `RefCell<T>`
Sometimes you need to mutate a value even when you have an immutable reference to it. This pattern is called **Interior Mutability**.

#### `Cell<T>`
Works for types that implement `Copy`. It allows you to get and set values without needing a mutable reference.
```rust
use std::cell::Cell;

struct User {
    id: u32,
    active: Cell<bool>,
}

let user = User { id: 1, active: Cell::new(true) };
user.active.set(false); // ✅ Works even though `user` is immutable
```

#### `RefCell<T>`
Works for any type. It moves the borrowing rules from compile-time to **runtime**.

```rust
use std::cell::RefCell;

let data = RefCell::new(vec![1, 2, 3]);

{
    let mut mutable_borrow = data.borrow_mut();
    mutable_borrow.push(4);
} // mutable_borrow dropped here

println!("Data: {:?}", data.borrow());
```

**Caution**: If you violate the borrowing rules at runtime (e.g., calling `borrow_mut()` when another borrow is active), the program will **panic**.

---

### 4. Summary Table

| Pointer | Use Case | Mutability |
|---------|----------|------------|
| `Box<T>` | Single ownership on heap | Inherited (mutable if variable is `mut`) |
| `Rc<T>` | Multiple ownership (single-threaded) | Immutable |
| `RefCell<T>` | Interior mutability for any type | Mutable (checked at runtime) |
| `Cell<T>` | Interior mutability for `Copy` types | Mutable (set/get) |

***
