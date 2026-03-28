[English Original](../en/ch12-closures-and-iterators.md)

# 12. 闭包与迭代器 🟡

> **你将学到：**
> - Rust 闭包 (`|args| body`) 与 Python Lambda 的区别
> - 迭代器链：Rust 版的“列表推导式”
> - 惰性求值 (Lazy Evaluation)：为什么必须写 `collect()`
> - 宏 (Macros)：Rust 的编译期元编程工具

## 闭包：超越 Lambda

在 Python 中，`lambda` 被限制在单个表达式内。而 Rust 的闭包功能强大，可以跨越多行，并且能够精确控制对环境变量的捕获方式。

### Python Lambda
```python
double = lambda x: x * 2
```

### Rust 闭包
```rust
let double = |x: i32| x * 2;

// 多行闭包：
let complex = |x: i32| {
    let y = x + 10;
    y * 2
};
```

### 环境变量捕获
使用 `move` 关键字可以强制闭包获取所捕获变量的所有权（这在多线程编程中是必不可少的）。
```rust
let data = vec![1, 2, 3];
let closure = move || println!("{:?}", data);
// data 现在已被“移动”到闭包内部，该行之后不能再使用 data！
```

---

## 迭代器链：Rust 的“列表推导式”

Rust 没有像 `[x for x in list]` 这样的语法糖。相反，它使用**迭代器链**来实现同样的功能，且效率更高。

| Python | Rust 迭代器链 |
|--------|----------------------|
| `[x*x for x in nums]` | `nums.iter().map(|x| x * x).collect()` |
| `[x for x in nums if x > 0]` | `nums.iter().filter(|x| x > 0).collect()` |
| `any(x > 0 for x in nums)` | `nums.iter().any(|x| x > 0)` |

### 为什么需要 .collect()?
Rust 的迭代器是**惰性 (Lazy)** 的。像 `map` 和 `filter` 这样的方法并不会立刻处理数据，直到你调用像 `.collect()` (收集到集合)、`.sum()` (求和) 或 `.count()` (计数) 这样的“消耗性”方法为止。

```rust
let nums = vec![1, 2, 3];
let doubled = nums.iter().map(|x| x * 2); // 这一行代码什么都没发生！
let result: Vec<_> = doubled.collect();   // 这一行才真正开始运行处理逻辑。
```

---

## 宏 (Macros)：编译期的代码生成

Python 使用装饰器和元类进行元编程。而 Rust 使用**宏**在编译阶段直接生成代码。

### 常用宏清单
- `println!("...")`：格式化打印。
- `vec![1, 2, 3]`：创建动态数组 (Vector)。
- `panic!("...")`：让程序崩溃 (对应 `raise Exception`)。
- `todo!()`：暂时没想好怎么写的占位符，编译能过但运行会报错。

### 特别好用的 `dbg!` 宏
`dbg!` 堪称调试神器。它会打印当前文件名、行号以及表达式及其求出的值。
```rust
let x = 5;
dbg!(x * 2); // 打印内容类似 "[src/main.rs:2] x * 2 = 10"
```

---

## 练习

<details>
<summary><strong>🏋️ 练习：偶数平方和</strong> (点击展开)</summary>

**挑战**：给定一个数组 `vec![1, 2, 3, 4, 5, 6]`，使用迭代器链完成以下步骤：
1. 过滤出偶数。
2. 将每个数进行平方。
3. 将结果收集到一个新的 `Vec<i32>` 中。

<details>
<summary>参考答案</summary>

```rust
fn main() {
    let nums = vec![1, 2, 3, 4, 5, 6];
    let result: Vec<i32> = nums.iter()
        .filter(|&&x| x % 2 == 0)
        .map(|x| x * x)
        .collect();
    
    println!("{:?}", result); // [4, 16, 36]
}
```
</details>
</details>

***
