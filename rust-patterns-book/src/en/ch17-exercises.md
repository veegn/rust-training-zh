# 17. Exercises 🟢

Practice the patterns learned in this book with these hands-on challenges.

---

### Exercise 1: Type-Safe State Machine ★★
Build a traffic light state machine using the **Type-State Pattern**. The light must transition `Red → Green → Yellow → Red` and no other order should be possible at compile time.

```rust
struct Red;
struct Green;
struct Yellow;

struct TrafficLight<S> { _s: std::marker::PhantomData<S> }

impl TrafficLight<Red> {
    fn go(self) -> TrafficLight<Green> { ... }
}
// Implement transitions for Green -> Yellow and Yellow -> Red.
```

---

### Exercise 2: Unit-of-Measure with PhantomData ★★
Extend the `Qty<Unit>` pattern to support division. If you divide `Qty<Meters>` by `Qty<Seconds>`, the result should be `Qty<MetersPerSecond>`.

---

### Exercise 3: Worker Pool with Channels ★★★
Build a thread-safe worker pool where:
1. A dispatcher sends `Job` structs through a channel.
2. N workers consume jobs concurrently.
3. Workers send `Result` structs back through a separate channel.

---

### Exercise 4: Custom serde Deserializer ★★★
Create a `HumanDuration` struct that can deserialize from strings like `"30s"`, `"5m"`, or `"2h"` into a `std::time::Duration`.

---

### Exercise 5: Safe Wrapper around Unsafe ★★★
Implement `FixedVec<T, const N: usize>`, a stack-allocated vector with a fixed capacity. Use `MaybeUninit<T>` for storage and ensure all public methods are safe.

---

### Exercise 6: Async Pipeline ★★★
Create a producer-transformer-consumer pipeline using `tokio::sync::mpsc` channels. 
- **Producer**: Sends numbers 1..100.
- **Transformer**: Multiplies numbers by 2.
- **Consumer**: Collects and prints the results.
Demonstrate back-pressure by using bounded channels.

***
