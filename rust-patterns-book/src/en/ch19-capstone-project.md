# 19. Capstone Project: Type-Safe Task Scheduler ★★★ 🟢

This project integrates the patterns learned throughout the book into a single, production-style system. You will build a **type-safe, concurrent task scheduler**.

### Project Features
1. **Typed Lifecycle**: Tasks move through `Pending → Running → Completed` using the **Type-State Pattern**.
2. **Concurrent Workers**: A pool of worker threads pulls tasks from a shared channel.
3. **Safety First**: Invalid state transitions (e.g., running a completed task) result in **compile-time errors**.
4. **Error Propagation**: Use `thiserror` to define a structured error hierarchy for the scheduler.

---

### Step 1: Task State Machine
Define markers for states and a generic `Task` struct. Use `PhantomData` to track the state without runtime overhead.

```rust
struct Pending;
struct Running;
struct Completed;

struct Task<S, R> {
    id: u64,
    _state: PhantomData<S>,
    _result: PhantomData<R>,
}

impl<R> Task<Pending, R> {
    fn start(self) -> Task<Running, R> { ... }
}
```

---

### Step 2: The Scheduler
The scheduler manages a `Sender` to dispatch `WorkItem`s and a `Receiver` to collect results.

```rust
struct Scheduler<R> {
    sender: mpsc::Sender<WorkItem<R>>,
    results: mpsc::Receiver<TaskResult<R>>,
}
```

---

### Step 3: Worker Implementation
Each worker runs in its own thread, locking a shared receiver to pull the next available task.

---

### Step 4: Verification
Write a test suite that:
1. Submits 10 concurrent tasks and verifies their results.
2. Ensures that tasks failing internally return a structured `Err` result rather than panicking.
3. Uses `proptest` to fuzzy-test the scheduler with varying numbers of tasks.

***
