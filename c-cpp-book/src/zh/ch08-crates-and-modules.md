[English Original](../en/ch08-crates-and-modules.md)

# 8. Crate 与模块 🟢

Rust 拥有一个强大的模块系统，它允许你组织代码并控制其内部项的可见性。

### 1. Crate (箱子)
**Crate** 是 Rust 编译器在某一时刻所考虑的最小代码单元。
- **二进制 Crate**：你可以运行的程序（具有 `main` 函数）。
- **库 Crate**：旨在供其他程序使用的代码（没有 `main` 函数）。

---

### 2. 模块与可见性 (Modules and Visibility)
模块让你能够为了可读性和复用而在 Crate 中以组的形式组织代码。默认情况下，Rust 中的一切都是 **私有 (Private)** 的。

```rust
mod front_of_house {
    pub mod hosting { // `pub` 使得它可以从外部进行访问
        pub fn add_to_waitlist() {}
    }

    fn seat_at_table() {} // 默认情况下为私有
}

pub fn eat_at_restaurant() {
    // 绝对路径
    crate::front_of_house::hosting::add_to_waitlist();

    // 相对路径
    front_of_house::hosting::add_to_waitlist();
}
```

---

### 3. `use` 关键字
为了避免键入冗长的路径，你可以使用 `use` 关键字将模块带入作用域。

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

### 4. 将模块拆分为多个文件
随着项目的增长，你可以将模块移动到各自的文件中。

文件结构：
```text
src/
├── main.rs
└── front_of_house.rs
```

在 `src/main.rs` 中：
```rust
mod front_of_house; // 告诉 Rust 去寻找 src/front_of_house.rs

use crate::front_of_house::hosting;

fn main() {
    hosting::add_to_waitlist();
}
```

在 `src/front_of_house.rs` 中：
```rust
pub mod hosting {
    pub fn add_to_waitlist() {}
}
```

---

### 5. `Cargo.toml` 与外部 Crate
Rust 的包管理器 **Cargo** 让你能非常轻松地使用来自 [crates.io](https://crates.io) 的外部库 (Crates)。

`Cargo.toml` 示例：
```toml
[dependencies]
rand = "0.8.5"
```

随后在你的代码中：
```rust
use rand::Rng;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);
}
```

---

### 对 C/C++ 开发者的总结
- **在 C/C++ 中**：你使用 `#include` 和头文件卫士 (Header Guards)。可见性通常通过 `static` 或类中的 private/public 来控制。
- **在 Rust 中**：没有头文件。模块系统负责组织和可见性 (`pub`)。Cargo 负责处理所有的依赖管理。

***
