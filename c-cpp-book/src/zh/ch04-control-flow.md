[English Original](../en/ch04-control-flow.md)

# Rust if 关键字

> **你将学到：** Rust 的控制流结构 —— 作为表达式的 `if`/`else`、`loop`/`while`/`for`、`match`，以及它们如何区别于 C/C++。关键点：大多数 Rust 控制流结构都会返回一个值。

- 在 Rust 中，`if` 实际上是一个表达式（Expression），也就是说它可以被用于赋值操作，但同时也像语句（Statement）一样工作。[▶ 点击尝试](https://play.rust-lang.org/)

```rust
fn main() {
    let x = 42;
    if x < 42 {
        println!("比生命之秘要小");
    } else if x == 42 {
        println!("等于生命之秘");
    } else {
        println!("比生命之秘要大");
    }
    let is_secret_of_life = if x == 42 {true} else {false};
    println!("{}", is_secret_of_life);
}
```

# 使用 while 和 for 实现循环

- `while` 关键字可用于在表达式为真时进行循环：
```rust
fn main() {
    let mut x = 40;
    while x != 42 {
        x += 1;
    }
}
```
- `for` 关键字可用于在范围内进行迭代：
```rust
fn main() {
    // 不会打印 43；欲包含最后一个元素请使用 40..=43
    for x in 40..43 {
        println!("{}", x);
    } 
}
```

---

# 使用 loop 实现循环

- `loop` 关键字创建一个无限循环，直到遇到 `break`：
```rust
fn main() {
    let mut x = 40;
    // 将其改为 'here: loop 以便为该循环指定可选标签
    loop {
        if x == 42 {
            break; // 使用 break x; 可直接返回 x 的值
        }
        x += 1;
    }
}
```
- `break` 语句可以包含一个可选的表达式，用于从 `loop` 表达式返回一个值。
- `continue` 关键字可用于直接返回到循环体顶部。
- 循环标签（Loop Labels）可与 `break` 或 `continue` 搭配使用，在处理嵌套循环时非常有用。

# Rust 表达式块

- Rust 表达式块（Expression Blocks）仅仅是一系列包裹在 `{}` 中的表达式。块的评估值就是块中最后一个表达式的值。
```rust
fn main() {
    let x = {
        let y = 40;
        y + 2 // 注意：分号 ; 必须省略
    };
    println!("{x}");
}
```
- Rust 的习惯用法是利用这种特性在函数中省略 `return` 关键字：
```rust
fn is_secret_of_life(x: u32) -> bool {
    // 等效于 if x == 42 {true} else {false}
    x == 42 // 注意：分号 ; 必须省略 
}
fn main() {
    println!("{}", is_secret_of_life(42));
}
```
---
