# 7. Closures and Higher-Order Functions 🟢

> **What you'll learn:**
> - The three closure traits (`Fn`, `FnMut`, `FnOnce`) and how capture works
> - Passing closures as parameters and returning them from functions
> - Combinator chains (`map`, `filter`, `fold`)
> - The `with` pattern for bracketed resource access

## Fn, FnMut, FnOnce — The Closure Traits

Rust closures implement one of three traits based on how they capture variables:

```rust
// FnOnce — consumes captured values (called once)
let name = String::from("Alice");
let greet = move || { drop(name); };

// FnMut — mutably borrows captured values (can be called repeatedly)
let mut count = 0;
let mut inc = || { count += 1; };

// Fn — immutably borrows (can be called concurrently)
let prefix = "ID";
let display = |x| println!("{prefix}: {x}");
```

**Hierarchy**: `Fn` : `FnMut` : `FnOnce`. Every `Fn` is also `FnMut`, and every `FnMut` is also `FnOnce`.

> **API Design Tip**: Accept `FnMut` as the default bound for parameters—it is the most flexible for callers.

---

## Combinator Chains

Higher-order functions shine with iterators. They are lazy and optimized by LLVM to be as fast as loops.

```rust
let result: Vec<i32> = data.iter()
    .filter(|&&x| x % 2 == 0) // Keep evens
    .map(|&x| x * x)         // Square them
    .collect();              // Gather into Vec
```

---

## The `with` Pattern (Bracketed Access)

Instead of exposing a resource and relying on the user to clean up, **lend it through a closure**.

```rust
impl GpioController {
    pub fn with_pin_output<R>(&self, pin: u8, mut f: impl FnMut(&GpioPin) -> R) -> R {
        self.set_direction(pin, Direction::Out); // Setup
        let result = f(&GpioPin { pin });         // Execute
        self.set_direction(pin, Direction::In);  // Cleanup
        result
    }
}

// Usage:
gpio.with_pin_output(4, |pin| {
    pin.write(true);
}); // Direction is automatically restored even if the closure panics
```

This pattern is safer than RAII/Drop when the resource **must not escape** the operation's scope.

***
