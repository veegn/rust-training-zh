[English Original](../en/ch11-from-and-into-traits.md)

# Rust From 和 Into 特性

> **你将学到：** Rust 的类型转换特性 —— 用于无损转换的 `From<T>` 和 `Into<T>`，以及用于可能失败的转换的 `TryFrom` 和 `TryInto`。实现 `From` 即可免费获得 `Into` 的实现。它们取代了 C++ 的转换运算符和构造函数。

- `From` 和 `Into` 是互补的特性，旨在简化类型转换。
- 类型通常实现 `From` 特性。例如 `String::from()` 可以将 `&str` 转换为 `String`，同时编译器可以自动推导出 `&str.into()`。
```rust
struct Point {x: u32, y: u32}
// 从元组构造一个 Point
impl From<(u32, u32)> for Point {
    fn from(xy : (u32, u32)) -> Self {
        Point {x : xy.0, y: xy.1}       // 使用元组元素构造 Point
    }
}
fn main() {
    let s = String::from("Rust");
    let x = u32::from(true);
    let p = Point::from((40, 42));
    // let p : Point = (40, 42).into(); // 上述代码的另一种形式
    println!("s: {s} x:{x} p.x:{} p.y {}", p.x, p.y);   
}
```

---

# 练习：From 和 Into
- 为 `Point` 实现 `From` 特性，将其转换为名为 `TransposePoint` 的类型。`TransposePoint` 会交换 `Point` 的 `x` 和 `y` 元素。

<details><summary>参考答案 (点击展开)</summary>

```rust
struct Point { x: u32, y: u32 }
struct TransposePoint { x: u32, y: u32 }

impl From<Point> for TransposePoint {
    fn from(p: Point) -> Self {
        TransposePoint { x: p.y, y: p.x }
    }
}

fn main() {
    let p = Point { x: 10, y: 20 };
    let tp = TransposePoint::from(p);
    println!("转置后: x={}, y={}", tp.x, tp.y);  // x=20, y=10

    // 使用 .into() —— 在实现 From 后会自动生效
    let p2 = Point { x: 3, y: 7 };
    let tp2: TransposePoint = p2.into();
    println!("转置后: x={}, y={}", tp2.x, tp2.y);  // x=7, y=3
}
```
**输出示例：**
```text
转置后: x=20, y=10
转置后: x=7, y=3
```

</details>

---

# Rust Default 特性
- `Default` 可用于为类型实现默认值。
    - 类型可以使用带有 `Default` 的 `Derive` 宏进行派生，或提供一个自定义实现。
```rust
#[derive(Default, Debug)]
struct Point {x: u32, y: u32}
#[derive(Debug)]
struct CustomPoint {x: u32, y: u32}

impl Default for CustomPoint {
    fn default() -> Self {
        CustomPoint {x: 42, y: 42}
    }
}

fn main() {
    let x = Point::default();   // 创建 Point{0, 0}
    println!("{x:?}");
    let y = CustomPoint::default();
    println!("{y:?}");
}
```

---

### Rust Default 特性
- `Default` 特性有几个典型的用例，包括：
    - 进行部分复制，并对剩余部分使用默认初始化。
    - 在 `unwrap_or_default()` 等方法中，为 `Option` 类型提供默认备选方案。
```rust
#[derive(Debug)]
struct CustomPoint {x: u32, y: u32}
impl Default for CustomPoint {
    fn default() -> Self {
        CustomPoint {x: 42, y: 42}
    }
}
fn main() {
    let x = CustomPoint::default();
    // 覆盖 y，但其余元素保留默认值
    let y = CustomPoint {y: 43, ..CustomPoint::default()};
    println!("{x:?} {y:?}");
    let z : Option<CustomPoint> = None;
    // 尝试将 unwrap_or_default() 更改为 unwrap() 看看效果
    println!("{:?}", z.unwrap_or_default());
}
```

---

### 其他 Rust 类型转换
- Rust 不支持隐式类型转换，可以使用 `as` 进行“显式”转换。
- 应谨慎使用 `as`，因为它在进行窄化转换等操作时可能会造成数据丢失。通常情况下，尽可能优先使用 `into()` 或 `from()`。
```rust
fn main() {
    let f = 42u8;
    // let g : u32 = f;    // 将无法编译
    let g = f as u32;      // 可以，但不推荐。受窄化转换规则约束
    let g : u32 = f.into(); // 最推荐的形式；无损且受编译器检查
    // let k : u8 = g.into();  // 无法编译；窄化转换可能导致数据丢失
    
    // 尝试进行窄化操作需要使用 try_into
    if let Ok(k) = TryInto::<u8>::try_into(g) {
        println!("{k}");
    }
}
```

---
