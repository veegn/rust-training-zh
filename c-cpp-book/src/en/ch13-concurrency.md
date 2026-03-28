# 13. Concurrency 🟢

Rust’s approach to **concurrency** is guided by the philosophy of "Fearless Concurrency." Rust’s ownership and type systems provide a set of tools that help you write code that is free of subtle bugs and easy to refactor.

### 1. Threads
Rust provides a 1:1 threading model. You can create a new thread using `thread::spawn`.

```rust
use std::thread;
use std::time::Duration;

fn main() {
    thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
}
```

---

### 2. Message Passing with Channels
A popular approach to ensuring safe concurrency is **message passing**, where threads or actors communicate by sending each other messages containing data.

```rust
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}
```

---

### 3. Shared-State Concurrency
Rust also supports sharing data between threads using **Mutexes** (Mutual Exclusion) and **Arc** (Atomic Reference Counting).

```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
```

---

### 4. Extensible Concurrency with `Sync` and `Send` Traits
- **`Send`**: Allows ownership of values to be transferred between threads.
- **`Sync`**: Allows multiple threads to access a value via shared references.

Most Rust types implement these traits automatically, but some (like `Rc<T>`) do not, which prevents them from being used unsafely in a multi-threaded context.

---

### Summary for C/C++ Developers
- **In C++**: You use `std::thread`, `std::mutex`, and `std::atomic`. It is very easy to accidentally create data races (e.g., sharing a non-atomic variable without a mutex).
- **In Rust**: Data races are **compile-time errors**. You cannot share data between threads unless it is wrapped in a thread-safe primitive like `Arc` or `Mutex`. This makes multi-threaded programming in Rust much safer and more reliable.

***
