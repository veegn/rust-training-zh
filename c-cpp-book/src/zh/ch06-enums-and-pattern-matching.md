# 6. 枚举与模式匹配 🟢

Rust 中的枚举 (Enums) 比 C 或 C++ 中的枚举强大得多。它们是 **和类型 (Sum Types)**（也称为标签联合或判别式联合），可以携带数据。

### 1. 基础枚举
枚举可以定义一组可能的变体。

```rust
enum IpAddrKind {
    V4,
    V6,
}

let four = IpAddrKind::V4;
```

---

### 2. 携带数据的枚举
与 C/C++ 枚举不同，Rust 枚举可以为每个变体关联数据。

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

let msg = Message::Move { x: 10, y: 20 };
```

对于 C++ 开发者：这类似于 `std::variant`，但更深度地集成到了语言中。

---

### 3. 使用 `match` 进行模式匹配
`match` 表达式允许你将一个值与一系列模式进行比较。它是 **详尽的 (Exhaustive)**，这意味着你必须处理每一种可能的情况。

```rust
fn process_message(msg: Message) {
    match msg {
        Message::Quit => println!("正在退出"),
        Message::Move { x, y } => println!("移动到 x: {x}, y: {y}"),
        Message::Write(text) => println!("文本消息: {text}"),
        Message::ChangeColor(r, g, b) => println!("颜色更改为 R:{r}, G:{g}, B:{b}"),
    }
}
```

#### `_` 通配符
使用 `_` 来处理所有剩余的情况（类似于 switch 中的 `default`）。
```rust
let some_u8_value = 0u8;
match some_u8_value {
    1 => println!("一"),
    3 => println!("三"),
    _ => (), // 对所有其他值不执行任何操作
}
```

---

### 4. 为类型实现方法
你可以使用 `impl` 块为 `structs` 和 `enums` 实现方法。

```rust
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // 关联函数（类似于 C++ 中的静态方法）
    fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }

    // 方法（接收 &self, &mut self 或 self）
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect = Rectangle::new(30, 50);
    println!("面积：{}", rect.area());
}
```

---

### 关联方法与 `self`
- **`&self`**：不可变借用。用于读取数据的方法，最为常用。
- **`&mut self`**：可变借用。用于修改实例的情况。
- **`self`**：获取所有权。实例被“消耗”，在方法调用后无法再被使用。

***
