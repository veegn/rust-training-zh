# 4. Control Flow 🟢

In Rust, control flow constructs like `if`, `match`, and `loop` are often **expressions**, meaning they can return a value.

### Conditional: `if`
Unlike C/C++, Rust's `if` is an expression. You can use it to assign values.

```rust
fn main() {
    let x = 42;
    
    // As a statement
    if x < 42 {
        println!("Too small");
    } else {
        println!("Just right");
    }

    // As an expression
    let status = if x == 42 { "Winner" } else { "Loser" };
    println!("Status: {status}");
}
```

---

### Looping: `loop`, `while`, and `for`

#### 1. `loop`
An infinite loop. You can use `break` to return a value from the loop.

```rust
fn main() {
    let mut x = 0;
    
    let result = loop {
        x += 1;
        if x == 10 {
            break x * 2; // Returns 20
        }
    };
    println!("Result: {result}");
}
```

#### 2. `while`
Standard while loop.
```rust
let mut n = 3;
while n != 0 {
    println!("{n}!");
    n -= 1;
}
```

#### 3. `for`
Used to iterate over collections or ranges.
```rust
fn main() {
    // Range 1 to 4 (exclusive)
    for i in 1..5 {
        println!("{i}");
    }

    // Range 1 to 5 (inclusive)
    for i in 1..=5 {
        println!("{i}");
    }
}
```

---

### Expression Blocks
A block of code `{}` is also an expression. The value of the last line (without a semicolon) is the value of the block.

```rust
fn main() {
    let x = {
        let y = 10;
        let z = 20;
        y + z // Value of x is 30
    };
    
    println!("x is {x}");
}
```

#### Idiomatic Returns
In Rust, it's idiomatic to omit the `return` keyword at the end of a function.
```rust
fn is_even(n: i32) -> bool {
    n % 2 == 0 // Implicit return
}
```

***
