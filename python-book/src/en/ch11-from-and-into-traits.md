# 11. From and Into Traits 🟢

> **What you'll learn:**
> - `From` and `Into` for zero-cost type conversions
> - `TryFrom` and `TryInto` for conversions that can fail
> - Common String conversion patterns (`to_string`, `parse`, `format!`)

## Type Conversions in Rust

Python handles type conversions with constructor calls like `int("42")` or `str(42)`. Rust uses the **`From`** and **`Into`** traits to define how one type can be transformed into another safely and efficiently.

### Implementing From
If you implement `From<A> for B`, you get `Into<B> for A` for free!

```rust
struct Seconds(i32);
struct Minutes(i32);

impl From<Minutes> for Seconds {
    fn from(m: Minutes) -> Self {
        Seconds(m.0 * 60)
    }
}

// Now you can use both:
let s1 = Seconds::from(Minutes(1));
let s2: Seconds = Minutes(5).into(); // Automatically works!
```

---

## TryFrom: Conversions that can Fail

Not every conversion is guaranteed to succeed (e.g., converting a large `i64` to `i8`). Python would raise an exception; Rust returns a `Result`.

```rust
use std::convert::TryInto;

let my_i64: i64 = 100;
let my_i8: Result<i8, _> = my_i64.try_into();

match my_i8 {
    Ok(n) => println!("Converted: {n}"),
    Err(_) => println!("Too big for i8!"),
}
```

---

## String Conversions: The "Big Three"

### 1. Anything → String (`to_string`)
Requires implementing the `Display` trait.
```rust
let s = 42.to_string();
```

### 2. String → Anything (`parse`)
Requires the type to implement `FromStr`. Returns a `Result`.
```rust
let n: i32 = "42".parse().expect("Not a number!");
```

### 3. &str ↔ String
- **`String::from("hi")`**: Create owned string from slice.
- **`s.as_str()`**: Get a slice from an owned string.

---

## Conversion Quick Reference

| Python | Rust | Result Type |
|--------|------|-------------|
| `str(x)`| `x.to_string()` | `String` |
| `int(s)`| `s.parse::<i32>()`| `Result<i32, ...>` |
| `float(s)`| `s.parse::<f64>()`| `Result<f64, ...>` |
| `list(range(5))`| `(0..5).collect::<Vec<_>>()` | `Vec<i32>` |
| `MyClass(old_obj)`| `MyClass::from(old_obj)` | `MyClass` |

---

## Exercises

<details>
<summary><strong>🏋️ Exercise: Point from Tuple</strong></summary>

**Challenge**: Define a struct `Point { x: i32, y: i32 }`. Implement `From<(i32, i32)>` for `Point`. Test it by converting a tuple `(10, 20)` into a `Point` using `.into()`.

<details>
<summary>🔑 Solution</summary>

```rust
#[derive(Debug)]
struct Point { x: i32, y: i32 }

impl From<(i32, i32)> for Point {
    fn from(tuple: (i32, i32)) -> Self {
        Point { x: tuple.0, y: tuple.1 }
    }
}

fn main() {
    let p: Point = (10, 20).into();
    println!("{p:?}");
}
```

</details>
</details>

***
