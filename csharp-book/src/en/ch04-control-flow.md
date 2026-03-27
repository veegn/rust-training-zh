# Functions and Control Flow

> **What you'll learn:** Functions and methods in Rust vs C#, the critical distinction between expressions and statements, and how Rust's expression-oriented design eliminates the need for ternary operators.
>
> **Difficulty:** 🟢 Beginner

## Functions vs Methods

### C# Method Declaration
In C#, methods must live inside classes or structs.
```csharp
public class Calculator {
    public int Add(int a, int b) {
        return a + b;
    }
}
```

### Rust Function Declaration
Rust supports standalone functions.
```rust
fn add(a: i32, b: i32) -> i32 {
    a + b // No 'return' needed for the final expression
}

fn main() {
    let result = add(5, 3);
    println!("5 + 3 = {result}");
}
```

---

## Expressions vs Statements (Important!)
This is the most significant conceptual shift in Rust's control flow.

*   **Statements**: Perform an action but do not return a value. They always end with a semicolon `;`.
*   **Expressions**: Evaluate to a value. They **do not** end with a semicolon.

```rust
fn get_value(condition: bool) -> i32 {
    if condition {
        42 // Expression (no semicolon)
    } else {
        0  // Expression (no semicolon)
    }
}
```
In Rust, the `if` block is an expression. Its value is the value of whichever branch is executed.

---

## Control Flow Basics

### If Expressions
Because `if` is an expression, it replaces the ternary operator (`? :`) from C#.
```rust
let x = 5;
let message = if x > 10 { "Big" } else { "Small" };
```

### Loops
Rust provides three types of loops:
1.  **`loop`**: An infinite loop.
2.  **`while`**: Runs while a condition is true.
3.  **`for`**: Iterates over a range or a collection.

```rust
// Range-based for loop
for i in 0..5 { // 0 to 4
    println!("{i}");
}

// Loop with break value
let result = loop {
    counter += 1;
    if counter == 10 {
        break counter * 2; // Returns value from loop
    }
};
```

---

## Exercise: Temperature Converter
**Challenge:** Convert a C# temperature switch to idiomatic Rust using enums and expressions.

```rust
enum TempUnit { Celsius, Fahrenheit }

fn convert(value: f64, from: TempUnit, to: TempUnit) -> f64 {
    let celsius = match from {
        TempUnit::Fahrenheit => (value - 32.0) * 5.0 / 9.0,
        TempUnit::Celsius => value,
    };
    match to {
        TempUnit::Fahrenheit => celsius * 9.0 / 5.0 + 32.0,
        TempUnit::Celsius => celsius,
    }
}
```
**Key Takeaway:** Using `match` as an expression makes the logic more concise and prevents "forgetting" a return.
