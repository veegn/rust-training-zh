# Rust 中的类型转换：From 与 Into

> **你将学到什么：** `From`/`Into` trait 与 C# 隐式/显式转换运算符的对比，`TryFrom`/`TryInto` 如何表示可能失败的转换，以及 `FromStr` 如何用于解析字符串。
>
> **难度：** 中级

C# 使用隐式和显式转换运算符来实现类型间的变换。而在 Rust 中，你需要通过一套标准的 Trait：`From`、`Into`、`TryFrom` 以及 `TryInto` 来完成类似的工作。

---

## `From` 与 `Into` Trait
这些 Trait 是为那些**总是会成功**的转换而设计的。
*   **`From<T>`**：定义了如何从类型 `T` 构建 `Self`。
*   **`Into<T>`**：这是 `From` 的对称关联。如果你为类型 `U` 实现了 `From<T>`，Rust 编译器会自动为类型 `T` 也会提供一个 `Into<U>` 实现。

### Rust 示例
```rust
struct Seconds(i32);

impl From<i32> for Seconds {
    fn from(val: i32) -> Self { Seconds(val) }
}

let s = Seconds::from(60);
let s: Seconds = 60.into(); // Into 是编译器自动生成的
```

### C# 对应物
```csharp
public struct Seconds {
    public int Value { get; }
    public Seconds(int v) => Value = v;
    public static implicit operator Seconds(int v) => new Seconds(v);
}
```

---

## `TryFrom` 与 `TryInto`
对于那些**可能失败**的转换（可能会根据输入值决定是否报错），Rust 提供了一套 `TryFrom` 和 `TryInto` Trait。它们的转换方法会返回一个 `Result`。

```rust
impl TryFrom<i32> for EvenNumber {
    type Error = String;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value % 2 == 0 {
            Ok(EvenNumber(value))
        } else {
            Err("不是偶数".to_string())
        }
    }
}
```

---

## `FromStr`：字符串解析
Rust 中没有像 `int.Parse()` 或 `DateTime.TryParse()` 这样的静态方法，而是通过为你的类型实现 `FromStr` Trait。这能让你可以直接在字符串上调用 `.parse()` 方法。

```rust
let point: Point = "10,20".parse().expect("无效的坐标格式");
```

---

## C# 开发者总结表
| **概念** | **C# 特性** | **Rust Trait** |
| :--- | :--- | :--- |
| **隐式转换** | `implicit operator` | `From` / `Into` |
| **显式转换** | `explicit operator` | `From` / `Into` (在 Rust 中仍是显式的) |
| **安全的转换失败** | 自定义的 `Try...` 方法 | `TryFrom` / `TryInto` |
| **字符串转类型** | `T.Parse()` | `FromStr` |
| **类型转字符串** | `T.ToString()` | `Display` trait |

---

## 练习：在 API 中使用 `Into`
**挑战：** 编写一个 `print_seconds` 函数，要求它能接收任何**可以被转换为 `Seconds` 结构体**的值作为参数。

```rust
fn print_seconds(time: impl Into<Seconds>) {
    let s: Seconds = time.into();
    println!("时间是 {} 秒", s.0);
}

// 这两种写法现在都行得通了！
print_seconds(60); 
print_seconds(Seconds(120));
```
**关键理解：** 在函数参数中使用 `impl Into<T>` 能够极大得增强你 API 的灵活性。它既能让调用方传具体类型，也能传能转换成该类型的其他值，这在 Rust 中非常地“地道（Idiomatic）”。
