# 8. Crates and Modules 🟢

Rust has a powerful module system that allows you to organize your code and control the visibility of its items.

### 1. Crates
A **crate** is the smallest unit of code that the Rust compiler considers at a time.
- **Binary Crate**: A program you can run (has a `main` function).
- **Library Crate**: Code intended to be used by other programs (no `main`).

---

### 2. Modules and Visibility
Modules let you organize code within a crate into groups for readability and reuse. By default, everything in Rust is **private**.

```rust
mod front_of_house {
    pub mod hosting { // `pub` makes it accessible from the outside
        pub fn add_to_waitlist() {}
    }

    fn seat_at_table() {} // Private by default
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}
```

---

### 3. The `use` Keyword
To avoid typing long paths, you can bring a module into scope with the `use` keyword.

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
```

---

### 4. Splitting Modules into Different Files
As your project grows, you can move modules into their own files.

File structure:
```text
src/
├── main.rs
└── front_of_house.rs
```

In `src/main.rs`:
```rust
mod front_of_house; // Tells Rust to look for src/front_of_house.rs

use crate::front_of_house::hosting;

fn main() {
    hosting::add_to_waitlist();
}
```

In `src/front_of_house.rs`:
```rust
pub mod hosting {
    pub fn add_to_waitlist() {}
}
```

---

### 5. `Cargo.toml` and External Crates
Rust's package manager, **Cargo**, makes it easy to use external libraries (crates) from [crates.io](https://crates.io).

Example `Cargo.toml`:
```toml
[dependencies]
rand = "0.8.5"
```

Then in your code:
```rust
use rand::Rng;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);
}
```

---

### Summary for C/C++ Developers
- **In C/C++**: You use `#include` and header guards. Visibility is usually controlled via `static` or private/public in classes.
- **In Rust**: There are no header files. The module system handles both organization and visibility (`pub`). Cargo handles all dependency management.

***
