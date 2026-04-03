[English Original](../en/ch12-1-iterator-power-tools.md)

# 迭代器高阶工具参考手册

> **你将学到：** 除了 `filter`/`map`/`collect` 之外的高级迭代器组合器 —— `enumerate`、`zip`、`chain`、`flat_map`、`scan`、`windows` 和 `chunks`。这些工具对于将带有手动索引、结果累加或固定大小块处理的 C 风格 `for` 循环替换为安全、富有表现力的 Rust 迭代器至关重要。

基础的 `filter`/`map`/`collect` 链条可以覆盖许多场景，但 Rust 的迭代器库远不止于此。本节涵盖了你日常会用到的工具 —— 尤其是在翻译那些手动追踪索引、累积结果或以固定大小块分批处理数据的 C 循环时。

### 快速参考表

| 方法 | C 语言等价物 | 功能说明 | 返回值类型 |
|--------|-------------|-------------|---------|
| `enumerate()` | `for (int i=0; ...)` | 将每个元素与其索引配对 | `(usize, T)` |
| `zip(other)` | 具有相同索引的并行数组 | 配对来自两个迭代器的元素 | `(A, B)` |
| `chain(other)` | 先处理数组 1，再处理数组 2 | 连接两个迭代器 | `T` |
| `flat_map(f)` | 嵌套循环 | 先映射再扁平化一层 | `U` |
| `windows(n)` | `for (int i=0; i<len-n+1; i++) &arr[i..i+n]` | 大小为 `n` 的重叠切片（滑动窗口） | `&[T]` |
| `chunks(n)` | 每次处理 `n` 个元素 | 大小为 `n` 的非重叠切片 | `&[T]` |
| `fold(init, f)` | `int acc = init; for (...) acc = f(acc, x);` | 归约为单个值 | `Acc` |
| `scan(init, f)` | 带有输出的运行累加器 | 类似 `fold` 但产出中间结果 | `Option<B>` |
| `take(n)` / `skip(n)` | 从偏移处开始循环 / 限制循环次数 | 获取前 `n` 个 / 跳过前 `n` 个元素 | `T` |
| `take_while(f)` / `skip_while(f)` | `while (pred) {...}` | 当谓词成立时获取/跳过 | `T` |
| `peekable()` | 使用 `arr[i+1]` 进行前瞻 | 允许在不消耗的情况下执行 `.peek()` | `T` |
| `step_by(n)` | `for (i=0; i<len; i+=n)` | 每隔 `n` 个元素取一个 | `T` |
| `unzip()` | 拆分并行数组 | 将对（Pairs）收集到两个集合中 | `(A, B)` |
| `sum()` / `product()` | 累加总和/乘积 | 使用 `+` 或 `*` 进行归约 | `T` |
| `min()` / `max()` | 寻找极值 | 返回 `Option<T>` | `Option<T>` |
| `any(f)` / `all(f)` | `bool found = false; for (...) ...` | 短路布尔搜索 | `bool` |
| `position(f)` | `for (i=0; ...) if (pred) return i;` | 第一个匹配项的索引 | `Option<usize>` |

---

### `enumerate` —— 索引 + 值（取代 C 语言索引循环）

```rust
fn main() {
    let sensors = ["GPU_TEMP", "CPU_TEMP", "FAN_RPM", "PSU_WATT"];

    // C 风格：for (int i = 0; i < 4; i++) printf("[%d] %s\n", i, sensors[i]);
    for (i, name) in sensors.iter().enumerate() {
        println!("[{i}] {name}");
    }

    // 查找特定传感器的索引
    let gpu_idx = sensors.iter().position(|&s| s == "GPU_TEMP");
    println!("GPU 传感器索引: {gpu_idx:?}");  // Some(0)
}
```

---

### `zip` —— 并行迭代（取代并行数组循环）

```rust
fn main() {
    let names = ["accel_diag", "nic_diag", "cpu_diag"];
    let statuses = [true, false, true];
    let durations_ms = [1200, 850, 3400];

    // C 语言方式：for (int i=0; i<3; i++) printf("%s: %s (%d ms)\n", names[i], ...);
    for ((name, passed), ms) in names.iter().zip(&statuses).zip(&durations_ms) {
        let status = if *passed { "通过" } else { "失败" };
        println!("{name}: {status} ({ms} ms)");
    }
}
```

---

### `chain` —— 连接迭代器

```rust
fn main() {
    let critical = vec!["ECC error", "Thermal shutdown"];
    let warnings = vec!["Link degraded", "Fan slow"];

    // 按优先级顺序处理所有事件
    let all_events: Vec<_> = critical.iter().chain(warnings.iter()).collect();
    println!("{all_events:?}");
    // ["ECC error", "Thermal shutdown", "Link degraded", "Fan slow"]
}
```

---

### `flat_map` —— 扁平化嵌套结果

```rust
fn main() {
    let lines = vec!["gpu:42:ok", "nic:99:fail", "cpu:7:ok"];

    // 从冒号分隔的行中提取所有数值
    let numbers: Vec<u32> = lines.iter()
        .flat_map(|line| line.split(':'))
        .filter_map(|token| token.parse::<u32>().ok())
        .collect();
    println!("{numbers:?}");  // [42, 99, 7]
}
```

---

### `windows` 和 `chunks` —— 滑动窗口与固定大小分组

```rust
fn main() {
    let temps = [65, 68, 72, 71, 75, 80, 78, 76];

    // windows(3): 重叠的 3 个元素分组（类似滑动平均值）
    // C 风格：for (int i = 0; i <= len-3; i++) avg(arr[i], arr[i+1], arr[i+2]);
    let moving_avg: Vec<f64> = temps.windows(3)
        .map(|w| w.iter().sum::<i32>() as f64 / 3.0)
        .collect();
    println!("滑动平均值: {moving_avg:.1?}");

    // chunks(2): 非重叠的 2 个元素分组
    // C 风格：for (int i = 0; i < len; i += 2) process(arr[i], arr[i+1]);
    for pair in temps.chunks(2) {
        println!("块 (Chunk): {pair:?}");
    }

    // chunks_exact(2): 与上述类似，但如果存在剩余元素则会触发 panic
    // 此外：.remainder() 可以获取剩余处理不了的元素
}
```

---

### `fold` 和 `scan` —— 累加处理

```rust
fn main() {
    let values = [10, 20, 30, 40, 50];

    // fold: 返回单个最终结果（类似 C 语言的累加循环）
    let sum = values.iter().fold(0, |acc, &x| acc + x);
    println!("总和: {sum}");  // 150

    // 使用 fold 构建字符串
    let csv = values.iter()
        .fold(String::new(), |acc, x| {
            if acc.is_empty() { format!("{x}") }
            else { format!("{acc},{x}") }
        });
    println!("CSV 字符串: {csv}");  // "10,20,30,40,50"

    // scan: 类似于 fold，但会产出中间结果
    let running_sum: Vec<i32> = values.iter()
        .scan(0, |state, &x| {
            *state += x;
            Some(*state)
        })
        .collect();
    println!("累计和: {running_sum:?}");  // [10, 30, 60, 100, 150]
}
```

---

### 练习：传感器数据流水线

给定原始传感器读数（每行一个，格式为 `"传感器名称:数值:单位"`），编写一个迭代器流水线来执行以下操作：
1. 将每一行解析为 `(name, f64, unit)`。
2. 过滤掉低于特定阈值的读数。
3. 使用 `fold` 归约到 `HashMap` 中，按传感器名称进行分组。
4. 打印每个传感器的平均读数。

```rust
// 初始代码
fn main() {
    let raw_data = vec![
        "gpu_temp:72.5:C",
        "cpu_temp:65.0:C",
        "gpu_temp:74.2:C",
        "fan_rpm:1200.0:RPM",
        "cpu_temp:63.8:C",
        "gpu_temp:80.1:C",
        "fan_rpm:1150.0:RPM",
    ];
    let threshold = 70.0;
    // 待完成：解析、过滤 >= 阈值的数值、按名称分组、计算平均值
}
```

---

<details><summary>参考答案 (点击展开)</summary>

```rust
use std::collections::HashMap;

fn main() {
    let raw_data = vec![
        "gpu_temp:72.5:C",
        "cpu_temp:65.0:C",
        "gpu_temp:74.2:C",
        "fan_rpm:1200.0:RPM",
        "cpu_temp:63.8:C",
        "gpu_temp:80.1:C",
        "fan_rpm:1150.0:RPM",
    ];
    let threshold = 70.0;

    // 解析 → 过滤 → 分组 → 平均
    let grouped = raw_data.iter()
        .filter_map(|line| {
            let parts: Vec<&str> = line.splitn(3, ':').collect();
            if parts.len() == 3 {
                let value: f64 = parts[1].parse().ok()?;
                Some((parts[0], value, parts[2]))
            } else {
                None
            }
        })
        .filter(|(_, value, _)| *value >= threshold)
        .fold(HashMap::<&str, Vec<f64>>::new(), |mut acc, (name, value, _)| {
            acc.entry(name).or_default().push(value);
            acc
        });

    for (name, values) in &grouped {
        let avg = values.iter().sum::<f64>() / values.len() as f64;
        println!("{name}: 平均值={avg:.1} ({} 次读取)", values.len());
    }
}
```
**输出示例 (顺序可能有所不同):**
```text
gpu_temp: 平均值=75.6 (3 次读取)
fan_rpm: 平均值=1175.0 (2 次读取)
```

</details>

---

# Rust 迭代器
- `Iterator` 特性用于为用户定义类型实现迭代功能（参考：https://doc.rust-lang.org/std/iter/trait.IntoIterator.html）。
    - 在下例中，我们将为一个斐波那契（Fibonacci）序列实现迭代器，该序列从 1, 1, 2, ... 开始，后继项是前两项之和。
    - `Iterator` 中的`关联类型` (`type Item = u32;`) 定义了迭代器输出的类型 (`u32`)。
    - `next()` 方法包含实现迭代器的逻辑。在本例中，所有状态信息都保存在 `Fibonacci` 结构体中。
    - 我们本可以实现另一个名为 `IntoIterator` 的特性，从而为更特殊的迭代器实现 `into_iter()` 方法。
    - [▶ 在 Rust Playground 中尝试](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=ab367dc2611e1b5a0bf98f1185b38f3f)

---
