# 4. Control Flow 🟢

> **What you'll learn:**
> - `if`/`else` as expressions (everything returns a value)
> - Loops: `loop`, `while`, and `for` vs Python's iteration
> - Ranges and iterator basics
> - Function signatures and the "implicit return" rule

## Conditional Statements

### if/else
```rust
// Rust — braces required, no parentheses, `else if` not `elif`
if temperature > 100 {
    println!("Too hot!");
} else if temperature < 0 {
    println!("Too cold!");
} else {
    println!("Just right");
}

// if is an EXPRESSION — returns a value (like Python ternary, but more powerful)
let status = if temperature > 100 { "hot" } else { "ok" };
```

### Truthiness (No more `if x:`)
In Python, many values are "falsy" (0, None, [], ""). In Rust, **only `bool` works in conditions**.
```rust
let x = 42;
// if x { }          // ❌ Error: expected bool, found i32
if x != 0 { }        // ✅ Explicit comparison required

let items: Vec<i32> = vec![];
if items.is_empty() { } // ✅ Explicit check
```

---

## Loops and Iteration

### for Loops
```rust
// range(5) → 0..5 (excludes end)
for i in 0..5 {
    println!("{i}");
}

// range(1, 6) → 1..=5 (includes end)
for i in 1..=5 {
    println!("{i}");
}

// Enumerate: for i, item in enumerate(list):
for (i, item) in ["a", "b", "c"].iter().enumerate() {
    println!("{i}: {item}");
}
```

### Infinite Loops
Use `loop` for a true infinite loop. It's preferred over `while true`.
```rust
loop {
    let data = get_input();
    if data == "quit" {
        break;
    }
}

// loop can return a value!
let result = loop {
    let input = get_input();
    if let Ok(num) = input.parse::<i32>() {
        break num; // Return value from loop
    }
};
```

### Iterator Chains (The Rust "List Comprehension")
Python's list comprehensions are "eager" (run immediately). Rust's iterator chains are **lazy** — they don't do anything until you `.collect()` them.
```rust
// Python: [x**2 for x in range(10) if x % 2 == 0]
let evens: Vec<i32> = (0..10)
    .filter(|x| x % 2 == 0)
    .map(|x| x * x)
    .collect();
```

---

## Everything is an Expression

In Rust, blocks (code inside `{}`) are expressions. The last line **without a semicolon** is the return value of that block.

```rust
let value = {
    let x = 5;
    let y = 10;
    x + y    // No semicolon → this is the value of the block (15)
};
```

Adding a semicolon `x + y;` makes it a statement, and the block would return `()` (empty/unit type).

---

## Functions

Rust requires types for all function parameters and return values.

```rust
// Implicit return: no semicolon on the last line
fn add(a: i32, b: i32) -> i32 {
    a + b
}

// Early return: use the `return` keyword
fn divide(a: f64, b: f64) -> Option<f64> {
    if b == 0.0 {
        return None; 
    }
    Some(a / b)
}
```

### self in Methods
- `&self`: Read-only borrow (most common)
- `&mut self`: Mutable borrow (to change fields)
- `self`: Consumes the object (it's "moved" and can't be used again)

---

## Exercises

<details>
<summary><strong>🏋️ Exercise: FizzBuzz Expression</strong></summary>

**Challenge**: Write FizzBuzz for numbers 1 to 30 using a `for` loop and a `match` expression. Instead of nested `if`s, match on the tuple `(n % 3, n % 5)`.

<details>
<summary>🔑 Solution</summary>

```rust
fn main() {
    for n in 1..=30 {
        let result = match (n % 3, n % 5) {
            (0, 0) => String::from("FizzBuzz"),
            (0, _) => String::from("Fizz"),
            (_, 0) => String::from("Buzz"),
            _ => n.to_string(),
        };
        println!("{result}");
    }
}
```

</details>
</details>

***
