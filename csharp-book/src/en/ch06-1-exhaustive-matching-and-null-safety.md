# Null Safety and Exhaustive Matching

> **What you'll learn:** Why C# `switch` expressions can silently miss cases while Rust's `match` catches them at compile time, `Option<T>` vs `Nullable<T>` for null safety, and error handling with `Result<T, E>`.
>
> **Difficulty:** 🟡 Intermediate

## The Danger of Incomplete Switches
In C#, `switch` expressions look exhaustive but aren't strictly guaranteed at compile time.

### C# Switch (Warnings Only)
```csharp
public enum Status { Ok, NotFound, Error }

public string Handle(Status s) => s switch {
    Status.Ok => "Success",
    Status.NotFound => "Not Found",
    // Missing 'Error' case! 
    // Compiles with a warning, but throws SwitchExpressionException at runtime.
};
```

### Rust Match (Compilation Error)
In Rust, if you miss a case, your program **simply will not compile**. This makes the compiler your safety net during refactoring.
```rust
enum Status { Ok, NotFound, Error }

fn handle(s: Status) -> &'static str {
    match s {
        Status::Ok => "Success",
        Status::NotFound => "Not Found",
        // ERROR: non-exhaustive patterns: `Error` not covered
    }
}
```

---

## Null Safety: `Option<T>`
Rust has no `null`. Instead, it uses the `Option<T>` enum to express that a value might be missing.

| **Feature** | **C# Nullable** | **Rust `Option<T>`** |
| :--- | :--- | :--- |
| **Mechanic** | `T?` or `Nullable<T>` | `Option<T>` |
| **Safety** | Warnings (if enabled) | **Compiler Enforcement** |
| **Access** | `obj?.Prop` or `.Value` | `match`, `if let`, or combinators |

### Combinators (Rust's `?.` Operator)
In C#, you use `?.` to chain null checks. In Rust, you use `and_then` and `map`.

```rust
// C#
string? name = user?.Address?.City?.ToUpper();

// Rust
let name = user.and_then(|u| u.address.as_ref())
               .and_then(|a| a.city.as_ref())
               .map(|c| c.to_uppercase());
```

---

## Errors: `Result<T, E>`
Rust handles errors using the `Result` enum instead of exceptions.

```rust
fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err("Division by zero".to_string())
    } else {
        Ok(a / b)
    }
}
```

### The `?` Operator
The `?` operator is a concise way to propagate errors, similar to a `try-catch` that automatically re-throws.
```rust
fn total_score() -> Result<i32, String> {
    let s1 = get_score("Math")?; // If Err, returns early
    let s2 = get_score("Science")?;
    Ok(s1 + s2)
}
```

---

## Exercise: Option Combinators
**Challenge:** Rewrite a nested C# null-check into a single Rust `Option` chain.

```rust
fn get_city_name(user: Option<&User>) -> String {
    user.and_then(|u| u.address.as_ref())
        .and_then(|a| a.city.as_ref())
        .map(|c| c.to_uppercase())
        .unwrap_or_else(|| "UNKNOWN".to_string())
}
```
**Takeaway:** `and_then` allows you to navigate through nested optional data safely without deep nesting or manual matching.
