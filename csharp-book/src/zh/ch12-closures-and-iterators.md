[English Original](../en/ch12-closures-and-iterators.md)

# 闭包与迭代器

> **你将学到什么：** Rust 闭包与 C# lambda 的区别（基于所有权的捕获）、Rust 迭代器如何以零成本替代 LINQ，以及惰性求值。
>
> **难度：** 中级

在 C# 中，你通常使用 Lambda 表达式和 LINQ 进行数据处理。而在 Rust 中，你需要通过 **闭包 (Closures)** 与 **迭代器 (Iterators)** 来实现同样的功能。它们不仅拥有函数式编程的表达力，而且运行时开销极低，并能配合显式的内存管理。

---

## 闭包 vs Lambda
Rust 闭包和 C# 的 Lambda 非常类似，但核心区别在于：Rust 闭包必须显式处理**所有权**。

### C# Lambda (按引用捕获)
```csharp
int multiplier = 3;
Func<int, int> multiply = x => x * multiplier;
```

### Rust 闭包 (捕获模式)
Rust 闭包可以借用（Borrow）或夺取（Take ownership）其所在环境（Environment）中的变量。
```rust
let multiplier = 3;
let multiply = |x| x * multiplier; // 借用 'multiplier'

let data = vec![1, 2, 3];
let owns_data = move || println!("{:?}", data); // 夺取 'data' 的所有权
```

### 三大闭包 Trait
1.  **`Fn`**：以不可变方式借用捕获值（只读）。
2.  **`FnMut`**：以可变方式借用捕获值（可进行修改）。
3.  **`FnOnce`**：消费掉捕获的值（只能被调用一次）。

---

## 迭代器：Rust 中的 LINQ
Rust 的迭代器是 **惰性的 (Lazy)** 且 **零成本 (Zero-cost)** 的。这组链式调用会被编译器直接优化为等价的手写 `for` 循环机器码。

### C# LINQ
```csharp
var result = numbers
    .Where(n => n % 2 == 0)
    .Select(n => n * n)
    .ToList();
```

### Rust 迭代器
```rust
let result: Vec<i32> = numbers.iter()
    .filter(|&&n| n % 2 == 0)
    .map(|&n| n * n)
    .collect(); // 'collect' 相当于 Rust 中的 'ToList'
```

---

## 相比 LINQ 的核心差异
1.  **惰性执行**：在你调用如 `collect`、`sum` 或 `find` 这样的“终结方法”之前，任何计算都不会真正发生。
2.  **高效率**：Rust 的迭代器链条往往比手写的普通循环更快，因为编译器能够跨越整个链条进行全方位的全局优化。
3.  **所有权控制**：你需要明确决定是迭代引用 (`iter()`)、可变引用 (`iter_mut()`)，还是直接消费掉整个集合的所有权 (`into_iter()`)。

---

## C# 开发者总结表
| **概念** | **C# / LINQ** | **Rust 迭代器** |
| :--- | :--- | :--- |
| **映射** | `.Select()` | `.map()` |
| **过滤** | `.Where()` | `.filter()` |
| **折叠** | `.Aggregate()` | `.fold()` |
| **执行时机** | 惰性/急切 混合 | 严格惰性 |
| **物化 (实例化)** | `.ToList()`, `.ToArray()` | `.collect::<Vec<_>>()` |

---

## 练习：过滤与转换
**挑战：** 给定一个姓名列表，要求过滤掉那些少于 5 个字符的名字，将其余名字全部转为大写，并最后收集进一个新的动态数组 (Vector) 中。

```rust
let names = vec!["Alice", "Bob", "Charlie", "Dave"];
let result: Vec<String> = names.iter()
    .filter(|n| n.len() >= 5)
    .map(|n| n.to_uppercase())
    .collect();
```
**关键理解：** 迭代器是 Rust 处理集合数据的“地道”方式。它将函数式编程的优雅可读性，与底层系统级编程的高性能完美结合在了一起。
