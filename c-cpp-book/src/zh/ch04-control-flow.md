[English Original](../en/ch04-control-flow.md)

# 4. 控制流 🟢

在 Rust 中，诸如 `if`、`match` 和 `loop` 之类的控制流结构通常是 **表达式 (Expressions)**，这意味着它们可以返回一个值。

### 条件：`if`
与 C/C++ 不同，Rust 的 `if` 是一个表达式。你可以使用它来为变量赋值。

```rust
fn main() {
    let x = 42;
    
    // 作为语句
    if x < 42 {
        println!("太小了");
    } else {
        println!("正好");
    }

    // 作为表达式
    let status = if x == 42 { "赢家" } else { "输家" };
    println!("状态：{status}");
}
```

---

### 循环：`loop`、`while` 和 `for`

#### 1. `loop`
无限循环。你可以使用 `break` 来从循环中返回一个值。

```rust
fn main() {
    let mut x = 0;
    
    let result = loop {
        x += 1;
        if x == 10 {
            break x * 2; // 返回 20
        }
    };
    println!("结果：{result}");
}
```

#### 2. `while`
标准的 while 循环。
```rust
let mut n = 3;
while n != 0 {
    println!("{n}!");
    n -= 1;
}
```

#### 3. `for`
用于迭代集合或范围（Ranges）。
```rust
fn main() {
    // 范围 1 到 4（不包含 5）
    for i in 1..5 {
        println!("{i}");
    }

    // 范围 1 到 5（包含 5）
    for i in 1..=5 {
        println!("{i}");
    }
}
```

---

### 表达式块
代码块 `{}` 也是一个表达式。最后一行（不带分号）的值就是该块的值。

```rust
fn main() {
    let x = {
        let y = 10;
        let z = 20;
        y + z // x 的值为 30
    };
    
    println!("x 是 {x}");
}
```

#### 惯用返回
在 Rust 中，惯例是在函数末尾省略 `return` 关键字。
```rust
fn is_even(n: i32) -> bool {
    n % 2 == 0 // 隐式返回
}
```

***
