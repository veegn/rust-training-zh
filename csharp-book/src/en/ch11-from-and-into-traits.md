# Type Conversions: From and Into

> **What you'll learn:** `From`/`Into` traits vs C#'s implicit/explicit operators, `TryFrom`/`TryInto` for fallible conversions, and `FromStr` for parsing strings.
>
> **Difficulty:** Intermediate

C# uses implicit and explicit conversion operators. Rust uses a set of standard traits: `From`, `Into`, `TryFrom`, and `TryInto`.

---

## The `From` and `Into` Traits
These are for conversion that **always succeed**.
*   **`From<T>`**: Defines how to create `Self` from a type `T`.
*   **`Into<T>`**: This is the reciprocal of `From`. If you implement `From<T> for U`, Rust automatically provides `Into<U> for T`.

### Rust Example
```rust
struct Seconds(i32);

impl From<i32> for Seconds {
    fn from(val: i32) -> Self { Seconds(val) }
}

let s = Seconds::from(60);
let s: Seconds = 60.into(); // Into is automatic
```

### C# Equivalent
```csharp
public struct Seconds {
    public int Value { get; }
    public Seconds(int v) => Value = v;
    public static implicit operator Seconds(int v) => new Seconds(v);
}
```

---

## `TryFrom` and `TryInto`
For conversions that **might fail**, Rust uses `TryFrom` and `TryInto`. These return a `Result`.

```rust
impl TryFrom<i32> for EvenNumber {
    type Error = String;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value % 2 == 0 {
            Ok(EvenNumber(value))
        } else {
            Err("Not even".to_string())
        }
    }
}
```

---

## `FromStr` for String Parsing
Instead of `int.Parse()` or `DateTime.TryParse()`, you implement `FromStr` for your type. This enables the `.parse()` method on strings.

```rust
let point: Point = "10,20".parse().expect("Invalid format");
```

---

## Summary for C# Developers
| **Concept** | **C# Feature** | **Rust Trait** |
| :--- | :--- | :--- |
| **Implicit Cast** | `implicit operator` | `From` / `Into` |
| **Explicit Cast** | `explicit operator` | `From` / `Into` (still explicit) |
| **Fail-Safe Conversion** | Custom `Try...` method | `TryFrom` / `TryInto` |
| **String to Type** | `T.Parse()` | `FromStr` |
| **Type to String** | `T.ToString()` | `Display` trait |

---

## Exercise: Use `Into` in an API
**Challenge:** Write a function `print_seconds` that accepts anything that can be converted into a `Seconds` struct.

```rust
fn print_seconds(time: impl Into<Seconds>) {
    let s: Seconds = time.into();
    println!("Time is {} seconds", s.0);
}

// Both work!
print_seconds(60); 
print_seconds(Seconds(120));
```
**Takeaway:** Using `impl Into<T>` in your function arguments makes your API much more flexible and "idiomatic" Rust.
