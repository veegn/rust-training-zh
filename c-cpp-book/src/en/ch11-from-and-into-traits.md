# 11. From and Into Traits 🟢

The `From` and `Into` traits are used for type conversions in Rust. They are designed to be used together: if you implement `From` for a type, you get the `Into` implementation for free.

### 1. The `From` Trait
The `From` trait allows a type to define how to create itself from another type.

```rust
let my_str = "hello";
let my_string = String::from(my_str); // Converting &str to String
```

We can implement it for our own types:
```rust
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

fn main() {
    let num = Number::from(30);
    println!("My number is {}", num.value);
}
```

---

### 2. The `Into` Trait
The `Into` trait is the reciprocal of the `From` trait. If you have implemented the `From` trait for your type, `Into` will call it when necessary.

```rust
fn main() {
    let int = 5;
    let num: Number = int.into(); // int is converted into Number
    println!("My number is {}", num.value);
}
```

---

### 3. `TryFrom` and `TryInto`
Similar to `From` and `Into`, `TryFrom` and `TryInto` are used for conversions that can fail (fallible conversions). They return a `Result`.

```rust
use std::convert::TryFrom;

struct EvenNumber(i32);

impl TryFrom<i32> for EvenNumber {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value % 2 == 0 {
            Ok(EvenNumber(value))
        } else {
            Err(())
        }
    }
}
```

---

### Summary for C/C++ Developers
- **In C++**: You use conversion constructors (e.g., `Point(const Tuple& t)`) or conversion operators (e.g., `operator int()`). These can often happen implicitly, leading to subtle bugs.
- **In Rust**: Conversions are explicit using `From` and `Into`. There are **no implicit type conversions** in Rust. This makes the flow of data across types much easier to track and debug.

***
