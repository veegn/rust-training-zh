# 5. Channels and Message Passing 🟢

> **What you'll learn:**
> - `std::sync::mpsc` basics and when to upgrade to crossbeam-channel
> - Channel selection with `select!` for multi-source message handling
> - Bounded vs unbounded channels and backpressure strategies
> - The actor pattern for encapsulating concurrent state

## std::sync::mpsc — The Standard Channel

Rust's standard library provides a multi-producer, single-consumer channel:

```rust
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone(); 
    thread::spawn(move || {
        tx1.send("msg from p1").unwrap();
    });

    thread::spawn(move || {
        tx.send("msg from p2").unwrap();
    });

    for msg in rx {
        println!("Received: {msg}");
    }
}
```

**Key properties**:
- **Unbounded** by default (can fill memory if consumer is slow).
- `mpsc::sync_channel(N)` creates a **bounded** channel with backpressure.
- `rx.recv()` blocks; `rx.try_recv()` does not.

---

## crossbeam-channel — The Production Workhorse

For most production use cases, use `crossbeam-channel`. It is faster, supports **Multiple Consumers (MPMC)**, and has a better API.

```rust
let (tx, rx) = crossbeam_channel::bounded(100);
// rx.clone() works! You can have multiple threads receiving from one channel.
```

### Channel Selection (select!)

Listen on multiple channels simultaneously, similar to Go's `select`:

```rust
loop {
    select! {
        recv(work_rx) -> msg => println!("Job: {msg:?}"),
        recv(ticker) -> _ => println!("Heartbeat"),
        recv(deadline) -> _ => break,
    }
}
```

---

## The Actor Pattern

Use channels to serialize access to mutable state instead of using a `Mutex`. This prevents complex lock ordering issues.

```rust
enum Msg { Increment, Get(Sender<i64>) }

fn actor(rx: Receiver<Msg>) {
    let mut count = 0;
    while let Ok(msg) = rx.recv() {
        match msg {
            Msg::Increment => count += 1,
            Msg::Get(tx) => tx.send(count).unwrap(),
        }
    }
}
```

> **Rule of Thumb**: 
> - Use **Mutex** for shared data with very short critical sections.
> - Use **Actors/Channels** for complex state, long operations, or distributed systems.

***
