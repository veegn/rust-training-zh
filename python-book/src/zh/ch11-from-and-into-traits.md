# 11. From 与 Into 特征 🟢

> **你将学到：**
> - 使用 `From` 和 `Into` 实现零成本类型转换
> - 用 `TryFrom` 和 `TryInto` 处理可能失败的转换
> - 字符串转换三要素 (`to_string`, `parse`, `format!`)

## Rust 里的类型转换

Python 通过各种构造函数 (如 `int("42")`, `str(42)`) 来处理转换。而 Rust 使用 **`From`** 和 **`Into`** 两个特征来规定一种类型如何安全、高效地转换为另一种类型。

### 实现 From
如果你为类型 B 实现了 `From<A>`，编译器会自动为你为 A 实现 `Into<B>`。

```rust
struct Seconds(i32);
struct Minutes(i32);

impl From<Minutes> for Seconds {
    fn from(m: Minutes) -> Self {
        Seconds(m.0 * 60)
    }
}

// 现在两种方式都行：
let s1 = Seconds::from(Minutes(1)); // 显式 From
let s2: Seconds = Minutes(5).into(); // 自动推理出的 Into
```

---

## TryFrom：可能失败的转换

并非所有转换都是百分之百成功的 (比如把一个很大的 `i64` 转成 `i8`)。Python 这种情况下会抛出异常；而在 Rust 里，这些方法会返回一个 `Result`。

```rust
use std::convert::TryInto;

let my_i64: i64 = 100;
let my_i8: Result<i8, _> = my_i64.try_into();

match my_i8 {
    Ok(n) => println!("成功转换: {n}"),
    Err(_) => println!("数值太大，i8 放不下！"),
}
```

---

## 字符串转换：最常用的三个模式

### 1. 任意类型 → 字符串 (`to_string`)
前提是你要为该类型实现 `Display` 特征（或者该类型是原生基础类型）。
```rust
let s = 42.to_string();
```

### 2. 字符串 → 其它类型 (`parse`)
前提是目标类型实现了 `FromStr`。注意这会返回一个 `Result`。
```rust
let n: i32 = "42".parse().expect("这不是个数字！");
```

### 3. &str 与 String 互转
- **`String::from("你好")`**: 把字符串切片变成拥有的字符串。
- **`s.as_str()`**: 把拥有的字符串借出成切片。

---

## 快速转换参考表

| Python | Rust | 结果类型 |
|--------|------|-------------|
| `str(x)`| `x.to_string()` | `String` |
| `int(s)`| `s.parse::<i32>()`| `Result<i32, ...>` |
| `float(s)`| `s.parse::<f64>()`| `Result<f64, ...>` |
| `list(range(5))`| `(0..5).collect::<Vec<_>>()` | `Vec<i32>` |
| `MyClass(old_obj)`| `MyClass::from(old_obj)` | `MyClass` |

---

## 练习

<details>
<summary><strong>🏋️ 练习：从元组创建 Point</strong> (点击展开)</summary>

**挑战**：定义一个结构体 `Point { x: i32, y: i32 }`。为它实现 `From<(i32, i32)>` 特征。测试时，将元组 `(10, 20)` 通过 `.into()` 转换成 `Point` 类型。

<details>
<summary>参考答案</summary>

```rust
#[derive(Debug)]
struct Point { x: i32, y: i32 }

impl From<(i32, i32)> for Point {
    fn from(tuple: (i32, i32)) -> Self {
        Point { x: tuple.0, y: tuple.1 }
    }
}

fn main() {
    let p: Point = (10, 20).into();
    println!("{:?}", p);
}
```
</details>
</details>

***
