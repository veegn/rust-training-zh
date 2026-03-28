# 6. Enums and Pattern Matching 🟢

Enums in Rust are much more powerful than in C or C++. They are **Sum Types** (also known as Tagged Unions or Discriminated Unions) that can carry data.

### 1. Basic Enums
Enums can define a set of possible variants.

```rust
enum IpAddrKind {
    V4,
    V6,
}

let four = IpAddrKind::V4;
```

---

### 2. Enums with Data
Unlike C/C++ enums, Rust enums can associate data with each variant.

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

let msg = Message::Move { x: 10, y: 20 };
```

For C++ developers: This is similar to `std::variant`, but more integrated into the language.

---

### 3. Pattern Matching with `match`
The `match` expression allows you to compare a value against a series of patterns. It is **exhaustive**, meaning you must handle every possible case.

```rust
fn process_message(msg: Message) {
    match msg {
        Message::Quit => println!("Quitting"),
        Message::Move { x, y } => println!("Moving to x: {x}, y: {y}"),
        Message::Write(text) => println!("Text message: {text}"),
        Message::ChangeColor(r, g, b) => println!("Change color to R:{r}, G:{g}, B:{b}"),
    }
}
```

#### The `_` Wildcard
Use `_` to handle any remaining cases (like `default` in a switch).
```rust
let some_u8_value = 0u8;
match some_u8_value {
    1 => println!("one"),
    3 => println!("three"),
    _ => (), // Do nothing for all other values
}
```

---

### 4. Method Implementation for Types
You can implement methods on `structs` and `enums` using the `impl` block.

```rust
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // Associated function (like a static method in C++)
    fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }

    // Method (takes &self, &mut self, or self)
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect = Rectangle::new(30, 50);
    println!("Area: {}", rect.area());
}
```

---

### Associated Methods and `self`
- **`&self`**: Immutable borrow. Most common for methods that read data.
- **`&mut self`**: Mutable borrow. Used to modify the instance.
- **`self`**: Takes ownership. The instance is "consumed" and cannot be used after the method call.

***
