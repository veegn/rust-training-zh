[English Original](../en/ch11-from-and-into-traits.md)

# 11. From 与 Into Trait 🟢

`From` 和 `Into` trait 在 Rust 中被用于执行类型转换。它们被设计为互补的一对：如果你为一个类型实现了 `From`，你将免费获得对应的 `Into` 的实现。

### 1. `From` Trait
`From` trait 允许一个类型定义如何从另一个类型创建它自己。

```rust
let my_str = "hello";
let my_string = String::from(my_str); // 将 &str 转换为 String
```

我们可以为我们自己的类型实现它：
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
    println!("我的值是 {}", num.value);
}
```

---

### 2. `Into` Trait
`Into` trait 是 `From` trait 的逆过程。如果你已经为你的类型实现了 `From` trait，那么 `Into` 会在必要时调用它。

```rust
fn main() {
    let int = 5;
    let num: Number = int.into(); // 将 int 转换为 Number
    println!("我的值是 {}", num.value);
}
```

---

### 3. `TryFrom` 与 `TryInto`
与 `From` 和 `Into` 类似，`TryFrom` 和 `TryInto` 被用于那些可能失败的转换（可失败的转换）。它们返回一个 `Result`。

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

### 对 C/C++ 开发者的总结
- **在 C++ 中**：你使用转换构造函数（例如 `Point(const Tuple& t)`）或转换运算符（例如 `operator int()`）。这些转换经常会隐式发生，从而导致微妙的 Bug。
- **在 Rust 中**：转换通过 `From` 和 `Into` 显式进行。Rust 中 **不存在隐式类型转换**。这使得跨类型的数据流更易于跟踪和调试。

***
