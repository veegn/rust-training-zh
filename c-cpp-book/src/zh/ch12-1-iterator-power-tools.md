# 12.1 迭代器进阶工具 🟢

Rust 的迭代器库既丰富又强大，允许你以函数式且极具表现力的方式链式处理数据。

### 1. `enumerate`
返回当前的迭代计数以及对应的值。

```rust
let v = vec!['a', 'b', 'c'];
for (i, val) in v.iter().enumerate() {
    println!("索引 {} 的值为 {}", i, val);
}
```

---

### 2. `zip`
将两个迭代器合并为一个由对 (Pairs) 组成的单一迭代器。

```rust
let names = vec!["Alice", "Bob"];
let ages = vec![25, 30];
let combined: Vec<_> = names.iter().zip(ages.iter()).collect();
// 结果：[(&"Alice", &25), (&"Bob", &30)]
```

---

### 3. `map` 与 `filter`
- **`map`**：转换每个元素。
- **`filter`**：仅保留满足条件的元素。

```rust
let numbers = vec![1, 2, 3, 4, 5, 6];
let result: Vec<_> = numbers.into_iter()
    .filter(|x| x % 2 == 0) // 保留偶数
    .map(|x| x * x)         // 进行平方
    .collect();
// 结果：[4, 16, 36]
```

---

### 4. `flat_map`
将每个元素映射到一个迭代器，并将结果“拍平 (Flatten)”。

```rust
let words = vec!["hello", "world"];
let chars: Vec<char> = words.into_iter()
    .flat_map(|s| s.chars())
    .collect();
// 结果：['h', 'e', 'l', 'l', 'o', 'w', 'o', 'r', 'l', 'd']
```

---

### 5. `fold`
使用初始值和闭包将整个迭代器归约为单个值。

```rust
let numbers = vec![1, 2, 3, 4];
let sum = numbers.iter().fold(0, |acc, x| acc + x);
// 结果：10
```

---

### 对 C/C++ 开发者的总结
- **In C++**：你使用 `<algorithm>` 函数，如 `std::transform`、`std::copy_if` 或循环。将这些操作链式调用通常很笨重。
- **In Rust**：迭代器就是为了链式调用而设计的。它们是 **惰性的 (Lazy)**，这意味着在调用诸如 `collect()`、`sum()` 或 `for_each()` 之类的“终结”方法之前，它们不会执行任何操作。

***
