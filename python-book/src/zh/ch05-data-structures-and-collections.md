# 5. 数据结构与集合 🟢

> **你将学到：**
> - Rust 元组与解构赋值 vs Python
> - 数组 (Arrays) 与切片 (Slices) —— 固定长度 vs 借用视图
> - 结构体 (Structs)：Rust 替代对象类的核心手段
> - 动态数组 `Vec<T>` (list) 与 哈希表 `HashMap<K, V>` (dict)
> - 从 Python 的“魔术方法”到 Rust Trait 的映射

## 元组与解构

### Python 元组
```python
point = (3.0, 4.0)
x, y = point  # 解包 (Unpacking)
```

### Rust 元组
```rust
let point: (f64, f64) = (3.0, 4.0);
let (x, y) = point; // 解构 (Destructuring)

// 通过索引访问 (使用 .0, .1)
let first = point.0;
```

---

## 数组与切片

Rust 严格区分**拥有数据**和**查看数据**。

### 1. 数组 (Array)
```rust
let numbers: [i32; 5] = [1, 2, 3, 4, 5]; // 长度是类型的一部分，固定不变
```

### 2. 切片 (Slice)
在 Python 中，`data[1:4]` 会创建一个**副本**。在 Rust 中，`&data[1..4]` 创建一个**视图**（零成本，无拷贝，无内存分配）。
```rust
let data = [10, 20, 30, 40, 50];
let slice: &[i32] = &data[1..4]; // [20, 30, 40] — 只是引用，不是拷贝
```

---

## 结构体 (Structs) vs 类 (Classes)

Rust 没有类和继承，而是使用**结构体**存数据，使用 **impl 块**定义方法。

### Python 类 (数据 + 方法)
```python
@dataclass
class Rectangle:
    width: float
    height: float
    def area(self): return self.width * self.height
```

### Rust 结构体 (Data + impl)
```rust
struct Rectangle {
    width: f64,
    height: f64,
}

impl Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}
```

### 关键映射：魔术方法 → Traits
| Python | Rust |
|--------|------|
| `__str__` | `impl Display` |
| `__repr__` | `#[derive(Debug)]` |
| `__eq__` | `#[derive(PartialEq)]` |
| `__init__` | `fn new()` (惯用法) |
| `__del__` | `impl Drop` (落域自动触发) |

---

## 常用集合

### 1. Vec<T> (对应 Python 的 list)
```rust
let mut nums = vec![1, 2, 3];
nums.push(4);             // 对应 append
let last = nums.pop();    // 返回 Option (对应 pop)
let len = nums.len();     // 对应 len()
```

### 2. HashMap<K, V> (对应 Python 的 dict)
```rust
use std::collections::HashMap;

let mut scores = HashMap::new();
scores.insert("Alice", 100);

// Entry API (等效于 defaultdict 或 setdefault)
*scores.entry("Bob").or_insert(0) += 10;
```

---

## 练习

<details>
<summary><strong>🏋️ 练习：词频统计器</strong> (点击展开)</summary>

**挑战**：实现一个函数，接受一段文字并返回单词出现的次数（忽略大小写）。Python 等效代码：`Counter(text.lower().split())`。

<details>
<summary>参考答案</summary>

```rust
use std::collections::HashMap;

fn word_frequencies(text: &str) -> HashMap<String, usize> {
    let mut counts = HashMap::new();
    for word in text.split_whitespace() {
        let key = word.to_lowercase();
        *counts.entry(key).or_insert(0) += 1;
    }
    counts
}

fn main() {
    let text = "the quick brown fox jumps over the lazy fox";
    let freq = word_frequencies(text);
    println!("{:?}", freq);
}
```
</details>
</details>

***
