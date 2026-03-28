[English Original](../en/ch04-control-flow.md)

# 4. 控制流 🟢

> **你将学到：**
> - 作为表达式的 `if`/`else`（一切皆有值）
> - 循环控制：`loop`、`while` 和 `for` 及其与 Python 的区别
> - 范围 (Ranges) 与迭代器基础
> - 函数签名与“隐式返回”规则

## 条件语句

### if/else
```rust
// Rust — 必须有花括号，无需括号，使用 `else if` 而不是 `elif`
if temperature > 100 {
    println!("太烫了！");
} else if temperature < 0 {
    println!("太冷了！");
} else {
    println!("刚刚好");
}

// if 是个表达式 —— 可以返回值 (类似 Python 的三元运算符，但更强大)
let status = if temperature > 100 { "烫" } else { "好" };
```

### Truthiness (再也没有 `if x:`)
在 Python 中，很多值都是“假”的 (0, None, [], "")。在 Rust 中，**只有 `bool` 能用于条件判断**。
```rust
let x = 42;
// if x { }          // ❌ 错误：期望 bool，得到 i32
if x != 0 { }        // ✅ 必须进行显式比较

let items: Vec<i32> = vec![];
if items.is_empty() { } // ✅ 显式检查是否为空
```

---

## 循环与迭代

### for 循环
```rust
// range(5) → 0..5 (不包含 5)
for i in 0..5 {
    println!("{i}");
}

// range(1, 6) → 1..=5 (包含 5)
for i in 1..=5 {
    println!("{i}");
}

// Enumerate: for i, item in enumerate(list):
for (i, item) in ["a", "b", "c"].iter().enumerate() {
    println!("{i}: {item}");
}
```

### 无限循环
使用 `loop` 表示真正的无限循环。它比 `while true` 更受推崇。
```rust
loop {
    let data = get_input();
    if data == "quit" {
        break;
    }
}

// loop 还可以返回值！
let result = loop {
    let input = get_input();
    if let Ok(num) = input.parse::<i32>() {
        break num; // 从循环中返回一个值
    }
};
```

### 迭代器链 (Rust 的“列表推导式”)
Python 的列表推导式是“立即执行”的。Rust 的迭代器链是**惰性 (Lazy)** 的 —— 除非你写了 `.collect()`，否则它们什么都不做。
```rust
// Python: [x**2 for x in range(10) if x % 2 == 0]
let evens: Vec<i32> = (0..10)
    .filter(|x| x % 2 == 0)
    .map(|x| x * x)
    .collect();
```

---

## 万物皆表达式

在 Rust 中，花括号 `{}` 包裹的代码块是表达式。**最后一行如果没有分号**，该行就是整个块的返回值。

```rust
let value = {
    let x = 5;
    let y = 10;
    x + y    // 没有分号 → 这就是块的返回值 (15)
};
```

如果加上分号 `x + y;`，它就变成了一条普通语句，块的返回值会变成 `()` (空/单元类型)。

---

## 函数

Rust 的函数参数和返回值必须明确标注类型。

```rust
// 隐式返回：最后一行不加分号
fn add(a: i32, b: i32) -> i32 {
    a + b
}

// 提前返回：使用 `return` 关键字
fn divide(a: f64, b: f64) -> Option<f64> {
    if b == 0.0 {
        return None; 
    }
    Some(a / b)
}
```

### 方法中的 self
- `&self`：只读借用 (最常见)。
- `&mut self`：可变借用 (用于修改字段)。
- `self`：消耗对象 (对象被“移动”了，之后不能再用)。

---

## 练习

<details>
<summary><strong>🏋️ 练习：表达式版本的 FizzBuzz</strong> (点击展开)</summary>

**挑战**：为数字 1 到 30 编写 FizzBuzz。使用 `for` 循环和 `match` 表达式。不要用嵌套的 `if`，直接对元组 `(n % 3, n % 5)` 进行 `match`。

<details>
<summary>参考答案</summary>

```rust
fn main() {
    for n in 1..=30 {
        let result = match (n % 3, n % 5) {
            (0, 0) => String::from("FizzBuzz"),
            (0, _) => String::from("Fizz"),
            (_, 0) => String::from("Buzz"),
            _ => n.to_string(),
        };
        println!("{result}");
    }
}
```
</details>
</details>

***
