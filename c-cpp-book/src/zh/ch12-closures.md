[English Original](../en/ch12-closures.md)

# Rust 闭包 (Closures)

> **你将学到：** 闭包作为匿名函数、三种捕获特性 (`Fn`、`FnMut`、`FnOnce`)、`move` 闭包，以及 Rust 闭包与 C++ lambda 的对比 —— 具有自动捕获分析功能，无需像 C++ 那样手动指定 `[&]` 或 `[=]`。

- 闭包是能够捕获其环境的匿名函数。
    - C++ 等价物：lambdas (`[&](int x) { return x + 1; }`)。
    - 关键区别：Rust 闭包具有**三种**捕获特性 (`Fn`、`FnMut`、`FnOnce`)，编译器会自动选择。
    - C++ 的捕获模式 (`[=]`、`[&]`、`[this]`) 需要手动指定且容易出错（例如悬垂的 `[&]`！）。
    - Rust 的借用检查器在编译时即可防止悬垂捕获。
- 闭包可以通过 `||` 符号来识别。参数类型包含在 `||` 中，并且支持类型推导。
- 闭包经常与迭代器（下一节的主题）配合使用。
```rust
fn add_one(x: u32) -> u32 {
    x + 1
}
fn main() {
    let add_one_v1 = |x : u32| {x + 1}; // 显式指定类型
    let add_one_v2 = |x| {x + 1};       // 类型由调用处推导
    let add_one_v3 = |x| x + 1;         // 对于单行函数允许省略大括号
    println!("{} {} {} {}", add_one(42), add_one_v1(42), add_one_v2(42), add_one_v3(42) );
}
```

---

# 练习：闭包与捕获

🟡 **中级**

- 创建一个捕获外层作用域 `String` 并向其追加内容的闭包（提示：使用 `move`）。
- 创建一个闭包向量：`Vec<Box<dyn Fn(i32) -> i32>>`，其中包含分别执行加 1、乘以 2 和对输入取平方的闭包。遍历该向量并将每个闭包应用于数字 5。

<details><summary>参考答案 (点击展开)</summary>

```rust
fn main() {
    // 第一部分：捕获并向 String 追加内容的闭包
    let mut greeting = String::from("Hello");
    let mut append = |suffix: &str| {
        greeting.push_str(suffix);
    };
    append(", world");
    append("!");
    println!("{greeting}");  // "Hello, world!"

    // 第二部分：闭包向量
    let operations: Vec<Box<dyn Fn(i32) -> i32>> = vec![
        Box::new(|x| x + 1),      // 加 1
        Box::new(|x| x * 2),      // 乘以 2
        Box::new(|x| x * x),      // 取平方
    ];

    let input = 5;
    for (i, op) in operations.iter().enumerate() {
        println!("对 {input} 执行操作 {i}: {}", op(input));
    }
}
```
**输出示例：**
```text
Hello, world!
对 5 执行操作 0: 6
对 5 执行操作 1: 10
对 5 执行操作 2: 25
```

</details>

---

# Rust 迭代器 (Iterators)
- 迭代器是 Rust 最强大的特性之一。它们能以非常优雅的方式对集合执行各种操作，包括过滤 (`filter()`)、变换 (`map()`)、查找 (`find()`) 等等。
- 在下例中，`|&x| *x >= 42` 是一个执行比较操作的闭包。`|x| println!("{x}")` 是另一个闭包。
```rust
fn main() {
    let a = [0, 1, 2, 3, 42, 43];
    for x in &a {
        if *x >= 42 {
            println!("{x}");
        }
    }
    // 与上述逻辑等效
    a.iter().filter(|&x| *x >= 42).for_each(|x| println!("{x}"))
}
```

---

# Rust 迭代器
- 迭代器的一个关键特性是它们大多数都是**惰性的 (Lazy)**。也就是说，在它们被实际评估之前，什么也不会做。例如，如果没有 `for_each`，`a.iter().filter(|&x| *x >= 42);` 将**不会执行任何操作**。Rust 编译器在检测到这种情形时会发出明确的警告。
```rust
fn main() {
    let a = [0, 1, 2, 3, 42, 43];
    // 为每个元素加 1 并打印
    let _ = a.iter().map(|x| x + 1).for_each(|x| println!("{x}"));
    let found = a.iter().find(|&x| *x == 42);
    println!("{found:?}");
    // 统计元素数量
    let count = a.iter().count();
    println!("{count}");
}
```

---

# Rust 迭代器
- `collect()` 方法可用于将结果收集到一个单独的集合中。
    - 下例中 `Vec<_>` 中的 `_` 相当于接收 `map` 返回类型的通配符。例如，我们甚至可以从 `map` 中返回 `String`。
```rust
fn main() {
    let a = [0, 1, 2, 3, 42, 43];
    let squared_a : Vec<_> = a.iter().map(|x| x * x).collect();
    for x in &squared_a {
        println!("{x}");
    }
    let squared_a_strings : Vec<_> = a.iter().map(|x| (x * x).to_string()).collect();
    // 这些实际上是字符串表示
    for x in &squared_a_strings {
        println!("{x}");
    }
}
```

---

# 练习：Rust 迭代器

平衡性 **入门**
- 创建一个包含奇数和偶数元素的整数数组。遍历该数组并将其拆分为两个不同的向量，分别包含偶数和奇数。
- 这能否在单次遍历中完成？（提示：使用 `partition()`）

<details><summary>参考答案 (点击展开)</summary>

```rust
fn main() {
    let numbers = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // 方法 1：手动分发
    let mut evens = Vec::new();
    let mut odds = Vec::new();
    for n in numbers {
        if n % 2 == 0 {
            evens.push(n);
        } else {
            odds.push(n);
        }
    }
    println!("偶数: {evens:?}");
    println!("奇数: {odds:?}");

    // 方法 2：使用 partition() 进行单次处理
    let (evens, odds): (Vec<i32>, Vec<i32>) = numbers
        .into_iter()
        .partition(|n| n % 2 == 0);
    println!("偶数 (partition): {evens:?}");
    println!("奇数 (partition): {odds:?}");
}
```
**输出示例：**
```text
偶数: [2, 4, 6, 8, 10]
奇数: [1, 3, 5, 7, 9]
偶数 (partition): [2, 4, 6, 8, 10]
奇数 (partition): [1, 3, 5, 7, 9]
```

</details>

> **生产环境模式**：关于生产环境 Rust 代码中的真实迭代器链（如 `.map().collect()`、`.filter().collect()`、`.find_map()`），请参考[通过闭包消除分支陷阱](ch17-3-collapsing-assignment-pyramids.md#使用闭包消除赋值金字塔)。

---

### 迭代器高阶工具：取代 C++ 循环的方法

以下迭代器适配器在生产环境的 Rust 代码中被**广泛**使用。C++ 虽有 `<algorithm>` 和 C++20 的 ranges，但 Rust 的迭代器链更具可组合性，且使用频率更高。

#### `enumerate` —— 索引 + 值（取代 `for (int i = 0; ...)`）

```rust
let sensors = vec!["temp0", "temp1", "temp2"];
for (idx, name) in sensors.iter().enumerate() {
    println!("传感器 {idx}: {name}");
}
// 传感器 0: temp0
// 传感器 1: temp1
// 传感器 2: temp2
```

C++ 等价物：`for (size_t i = 0; i < sensors.size(); ++i) { auto& name = sensors[i]; ... }`

---

#### `zip` —— 配对两个迭代器的元素（取代并行索引循环）

```rust
let names = ["gpu0", "gpu1", "gpu2"];
let temps = [72.5, 68.0, 75.3];

let report: Vec<String> = names.iter()
    .zip(temps.iter())
    .map(|(name, temp)| format!("{name}: {temp}°C"))
    .collect();
println!("{report:?}");
// ["gpu0: 72.5°C", "gpu1: 68.0°C", "gpu2: 75.3°C"]

// 在较短的迭代器处停止 —— 无越界风险
```

C++ 等价物：`for (size_t i = 0; i < std::min(names.size(), temps.size()); ++i) { ... }`

---

#### `flat_map` —— 对嵌套集合进行映射并扁平化

```rust
// 每块 GPU 都有多个 PCIe BDF；收集所有 GPU 上的所有 BDF
let gpu_bdfs = vec![
    vec!["0000:01:00.0", "0000:02:00.0"],
    vec!["0000:41:00.0"],
    vec!["0000:81:00.0", "0000:82:00.0"],
];

let all_bdfs: Vec<&str> = gpu_bdfs.iter()
    .flat_map(|bdfs| bdfs.iter().copied())
    .collect();
println!("{all_bdfs:?}");
// ["0000:01:00.0", "0000:02:00.0", "0000:41:00.0", "0000:81:00.0", "0000:82:00.0"]
```

C++ 等价物：使用嵌套的 `for` 循环并将结果推入（push）到单个 vector 中。

---

#### `chain` —— 连接两个迭代器

```rust
let critical_gpus = vec!["gpu0", "gpu3"];
let warning_gpus = vec!["gpu1", "gpu5"];

// 处理所有被标记的 GPU，优先处理关键（critical）GPU
for gpu in critical_gpus.iter().chain(warning_gpus.iter()) {
    println!("已标记: {gpu}");
}
```

---

#### `windows` 和 `chunks` —— 切片上的滑动窗口/固定大小视图

```rust
let temps = [70, 72, 75, 73, 71, 68, 65];

// windows(3): 大小为 3 的滑动窗口 —— 用于检测趋势
let rising = temps.windows(3)
    .any(|w| w[0] < w[1] && w[1] < w[2]);
println!("检测到上涨趋势: {rising}"); // true (70 < 72 < 75)

// chunks(2): 固定大小的分组 —— 进行成对处理
for pair in temps.chunks(2) {
    println!("配对: {pair:?}");
}
// 配对: [70, 72]
// 配对: [75, 73]
// 配对: [71, 68]
// 配对: [65]       ← 最后一组可以更小
```

C++ 等价物：使用 `i` 和 `i+1`/`i+2` 进行的手动索引算术运算。

---

#### `fold` —— 累加为单个值（取代 `std::accumulate`）

```rust
let errors = vec![
    ("gpu0", 3u32),
    ("gpu1", 0),
    ("gpu2", 7),
    ("gpu3", 1),
];

// 在单次遍历中统计错误总数并构建摘要
let (total, summary) = errors.iter().fold(
    (0u32, String::new()),
    |(count, mut s), (name, errs)| {
        if *errs > 0 {
            s.push_str(&format!("{name}:{errs} "));
        }
        (count + errs, s)
    },
);
println!("错误总数: {total}, 详情: {summary}");
// 错误总数: 11, 详情: gpu0:3 gpu2:7 gpu3:1
```

---

#### `scan` —— 有状态转换（累加总量、增量检测）

```rust
let readings = [100, 105, 103, 110, 108];

// 计算连续读数之间的增量
let deltas: Vec<i32> = readings.iter()
    .scan(None::<i32>, |prev, &val| {
        let delta = prev.map(|p| val - p);
        *prev = Some(val);
        Some(delta)
    })
    .flatten()  // 移除初始的 None
    .collect();
println!("增量: {deltas:?}"); // [5, -2, 7, -2]
```

---

#### 快速参考：C++ 循环 → Rust 迭代器

| **C++ 模式** | **Rust 迭代器** | **示例** |
|----------------|------------------|------------|
| `for (int i = 0; i < v.size(); i++)` | `.enumerate()` | `v.iter().enumerate()` |
| 带有索引的并行迭代 | `.zip()` | `a.iter().zip(b.iter())` |
| 嵌套循环 → 扁平结果 | `.flat_map()` | `vecs.iter().flat_map(|v| v.iter())` |
| 连接两个容器 | `.chain()` | `a.iter().chain(b.iter())` |
| 滑动窗口 `v[i..i+n]` | `.windows(n)` | `v.windows(3)` |
| 按固定大小分组处理 | `.chunks(n)` | `v.chunks(4)` |
| `std::accumulate` / 手动累加器 | `.fold()` | `.fold(init, |acc, x| ...)` |
| 运行总量 / 增量追踪 | `.scan()` | `.scan(state, |s, x| ...)` |
| `while (it != end && count < n) { ++it; ++count; }` | `.take(n)` | `.iter().take(5)` |
| `while (it != end && !pred(*it)) { ++it; }` | `.skip_while()` | `.skip_while(|x| x < &threshold)` |
| `std::any_of` | `.any()` | `.iter().any(|x| x > &limit)` |
| `std::all_of` | `.all()` | `.iter().all(|x| x.is_valid())` |
| `std::none_of` | `!.any()` | `!iter.any(|x| x.failed())` |
| `std::count_if` | `.filter().count()` | `.filter(|x| x > &0).count()` |
| `std::min_element` / `std::max_element` | `.min()` / `.max()` | `.iter().max()` → `Option<&T>` |
| `std::unique` | `.dedup()` (针对有序序列) | `v.dedup()` (在 Vec 上原地执行) |

---

### 练习：迭代器链

给定传感器数据 `Vec<(String, f64)>` (名称, 温度)，编写一个**单一的迭代器链**来完成以下任务：
1. 过滤掉温度 > 80.0 的传感器。
2. 按温度（降序）对它们进行排序。
3. 将每一项格式化为 `"{name}: {temp}°C [ALARM]"`。
4. 收集到 `Vec<String>` 中。

提示：由于排序需要 `Vec`，所以你需要在调用 `.sort_by()` 之前先执行一次 `.collect()`。

<details><summary>参考答案 (点击展开)</summary>

```rust
fn alarm_report(sensors: &[(String, f64)]) -> Vec<String> {
    let mut hot: Vec<_> = sensors.iter()
        .filter(|(_, temp)| *temp > 80.0)
        .collect();
    hot.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    hot.iter()
        .map(|(name, temp)| format!("{name}: {temp}°C [ALARM]"))
        .collect()
}

fn main() {
    let sensors = vec![
        ("gpu0".to_string(), 72.5),
        ("gpu1".to_string(), 85.3),
        ("gpu2".to_string(), 91.0),
        ("gpu3".to_string(), 78.0),
        ("gpu4".to_string(), 88.7),
    ];
    for line in alarm_report(&sensors) {
        println!("{line}");
    }
}
```
**输出示例：**
```text
gpu2: 91°C [ALARM]
gpu4: 88.7°C [ALARM]
gpu1: 85.3°C [ALARM]
```

</details>

---

# Rust 迭代器
- `Iterator` 特性用于为用户定义类型实现迭代功能（参考：https://doc.rust-lang.org/std/iter/trait.IntoIterator.html）。
    - 在下例中，我们将为一个斐波那契（Fibonacci）序列实现迭代器，该序列从 1, 1, 2, ... 开始，后继项是前两项之和。
    - `Iterator` 中的`关联类型` (`type Item = u32;`) 定义了迭代器输出的类型 (`u32`)。
    - `next()` 方法包含实现迭代器的逻辑。在本例中，所有状态信息都保存在 `Fibonacci` 结构体中。
    - 我们本可以实现另一个名为 `IntoIterator` 的特性，从而为更特殊的迭代器实现 `into_iter()` 方法。
```rust
struct Fibonacci {
    curr: u32,
    next: u32,
}

impl Default for Fibonacci {
    fn default() -> Self {
        Fibonacci { curr: 0, next: 1 }
    }
}

impl Iterator for Fibonacci {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let new_next = self.curr + self.next;
        self.curr = self.next;
        self.next = new_next;

        Some(self.curr)
    }
}

fn main() {
    let mut fib = Fibonacci::default();
    for n in fib.take(10) {
        println!("{n}");
    }
}
```
- [▶ 在 Rust Playground 中尝试](https://play.rust-lang.org/)

---
