## Iterator Power Tools Reference / 迭代器进阶工具参考
 
 > **What you'll learn / 你将学到：** Advanced iterator combinators beyond `filter`/`map`/`collect` — `enumerate`, `zip`, `chain`, `flat_map`, `scan`, `windows`, and `chunks`. Essential for replacing C-style indexed `for` loops with safe, expressive Rust iterators.
 >
 > 除 `filter` / `map` / `collect` 之外的高级迭代器组合器 —— `enumerate`、`zip`、`chain`、`flat_map`、`scan`、`windows` 和 `chunks`。这些工具对于用安全、极具表现力的 Rust 迭代器取代 C 风格的索引 `for` 循环至关重要。
 
- The basic `filter`/`map`/`collect` chain covers many cases, but Rust's iterator library
+ 基础的 `filter` / `map` / `collect` 链条覆盖了许多场景，但 Rust 的迭代器库远比这丰富。
- is far richer. This section covers the tools you'll reach for daily — especially when
+ 本节涵盖了你日常会用到的工具 —— 特别是在
- translating C loops that manually track indices, accumulate results, or process
+ 翻译那些手动跟踪索引、累加结果或按固定大小分块处理数据的 C 风格循环时。
- data in fixed-size chunks.
 
- ### Quick Reference Table
+ ### Quick Reference Table / 快速参考表
 
-| Method | C Equivalent | What it does | Returns |
+| **Method / 方法** | **C Equivalent / C 对应物** | **What it does / 功能** | **Returns / 返回** |
 |--------|-------------|-------------|---------|
-| `enumerate()` | `for (int i=0; ...)` | Pairs each element with its index | `(usize, T)` |
+| `enumerate()` | `for (int i=0; ...)` | Pairs with index / 将每个元素与其索引配对 | `(usize, T)` |
-| `zip(other)` | Parallel arrays with same index | Pairs elements from two iterators | `(A, B)` |
+| `zip(other)` | Parallel arrays / 并行数组 | Pairs from two iterators / 配对两个迭代器的元素 | `(A, B)` |
-| `chain(other)` | Process array1 then array2 | Concatenates two iterators | `T` |
+| `chain(other)` | Process seq1 then seq2 | Concatenates two iterators / 连接两个迭代器 | `T` |
-| `flat_map(f)` | Nested loops | Maps then flattens one level | `U` |
+| `flat_map(f)` | Nested loops / 嵌套循环 | Maps and flattens / 映射并拍平一层 | `U` |
-| `windows(n)` | `for (int i=0; i<len-n+1; i++) &arr[i..i+n]` | Overlapping slices of size `n` | `&[T]` |
+| `windows(n)` | Sliding window / 滑动窗口 | Overlapping slices / 大小为 `n` 的重叠切片 | `&[T]` |
-| `chunks(n)` | Process `n` elements at a time | Non-overlapping slices of size `n` | `&[T]` |
+| `chunks(n)` | Process `n` at a time | Fixed-size groups / 大小为 `n` 的非重叠分块 | `&[T]` |
-| `fold(init, f)` | `int acc = init; for (...) acc = f(acc, x);` | Reduce to single value | `Acc` |
+| `fold(init, f)` | `std::accumulate` | Reduce to single value / 归约为单个值 | `Acc` |
-| `scan(init, f)` | Running accumulator with output | Like `fold` but yields intermediate results | `Option<B>` |
+| `scan(init, f)` | Running total / 运行总计 | yields intermediate results / 产出中间结果 | `Option<B>` |
-| `take(n)` / `skip(n)` | Start loop at offset / limit | First `n` / skip first `n` elements | `T` |
+| `take(n)` / `skip(n)` | Loop limit / offset | First `n` / skip first `n` / 获取前 `n` 个 / 跳过前 `n` 个 | `T` |
-| `take_while(f)` / `skip_while(f)` | `while (pred) {...}` | Take/skip while predicate holds | `T` |
+| `take_while(f)` | `while (pred)` | Condition-based limit / 基于条件的获取/跳过 | `T` |
-| `peekable()` | Lookahead with `arr[i+1]` | Allows `.peek()` without consuming | `T` |
+| `peekable()` | Lookahead / 预读 | `.peek()` without consuming / 在不消耗的情况下预读 | `T` |
-| `step_by(n)` | `for (i=0; i<len; i+=n)` | Take every nth element | `T` |
+| `step_by(n)` | `i += n` | Take every nth / 每隔 `n` 个取一个 | `T` |
-| `sum()` / `product()` | Accumulate sum/product | Reduce with `+` or `*` | `T` |
+| `sum()` / `product()` | Sum/Product / 求和/乘积 | Reduce with `+` or `*` / 通过 `+` 或 `*` 归约 | `T` |
-| `any(f)` / `all(f)` | `bool found = false; for (...) ...` | Short-circuit boolean search | `bool` |
+| `any(f)` / `all(f)` | Boolean check / 布尔检查 | Short-circuit search / 短路搜索 | `bool` |
 
- ### `enumerate` — Index + Value (replaces C index loops)
+ ### `enumerate` — Index + Value (replaces C index loops / 替换 C 索引循环)
 
 ```rust
 fn main() {
     let sensors = ["GPU_TEMP", "CPU_TEMP", "FAN_RPM", "PSU_WATT"];
 
-    // C style: for (int i = 0; i < 4; i++) printf("[%d] %s\n", i, sensors[i]);
+    // C style: for (int i = 0; i < 4; i++) printf("[%d] %s\n", i, sensors[i]); / C 风格循环
     for (i, name) in sensors.iter().enumerate() {
         println!("[{i}] {name}");
     }
 
-    // Find the index of a specific sensor
+    // Find the index of a specific sensor / 查找特定传感器的索引
     let gpu_idx = sensors.iter().position(|&s| s == "GPU_TEMP");
-    println!("GPU sensor at index: {gpu_idx:?}");  // Some(0)
+    println!("GPU sensor at index: {gpu_idx:?}");  // 结果：Some(0)
 }
 ```
 
- ### `zip` — Parallel Iteration (replaces parallel array loops)
+ ### `zip` — Parallel Iteration (replaces parallel array loops / 替换并行数组循环)
 
 ```rust
 fn main() {
     let names = ["accel_diag", "nic_diag", "cpu_diag"];
     let statuses = [true, false, true];
     let durations_ms = [1200, 850, 3400];
 
-    // C: for (int i=0; i<3; i++) printf("%s: %s (%d ms)\n", names[i], ...);
+    // C: for (int i=0; i<3; i++) printf("%s: %s (%d ms)\n", names[i], ...); / C 风格并行数组
     for ((name, passed), ms) in names.iter().zip(&statuses).zip(&durations_ms) {
         let status = if *passed { "PASS" } else { "FAIL" };
         println!("{name}: {status} ({ms} ms)");
     }
 }
 ```
 
- ### `chain` — Concatenate Iterators
+ ### `chain` — Concatenate Iterators / 连接迭代器
 
 ```rust
 fn main() {
     let critical = vec!["ECC error", "Thermal shutdown"];
     let warnings = vec!["Link degraded", "Fan slow"];
 
-    // Process all events in priority order
+    // Process all events in priority order / 按优先级顺序处理所有事件
     let all_events: Vec<_> = critical.iter().chain(warnings.iter()).collect();
     println!("{all_events:?}");
     // ["ECC error", "Thermal shutdown", "Link degraded", "Fan slow"]
 }
 ```
 
- ### `flat_map` — Flatten Nested Results
+ ### `flat_map` — Flatten Nested Results / 拍平嵌套结果
 
 ```rust
 fn main() {
     let lines = vec!["gpu:42:ok", "nic:99:fail", "cpu:7:ok"];
 
-    // Extract all numeric values from colon-separated lines
+    // Extract all numeric values from colon-separated lines / 从冒号分隔的行中提取所有数值
     let numbers: Vec<u32> = lines.iter()
         .flat_map(|line| line.split(':'))
         .filter_map(|token| token.parse::<u32>().ok())
         .collect();
-    println!("{numbers:?}");  // [42, 99, 7]
+    println!("{numbers:?}");  // 结果：[42, 99, 7]
 }
 ```
 
- ### `windows` and `chunks` — Sliding and Fixed-Size Groups
+ ### `windows` and `chunks` — Sliding and Fixed-Size Groups / 滑动窗口与固定大小分组
 
 ```rust
 fn main() {
     let temps = [65, 68, 72, 71, 75, 80, 78, 76];
 
-    // windows(3): overlapping groups of 3 (like a sliding average)
-    // C: for (int i = 0; i <= len-3; i++) avg(arr[i], arr[i+1], arr[i+2]);
+    // windows(3): overlapping groups of 3 / 3 个一组的重叠分组（类似滑动平均）
+    // C: for (int i = 0; i <= len-3; i++) avg(arr[i], arr[i+1], arr[i+2]); / C 风格滑动窗口
     let moving_avg: Vec<f64> = temps.windows(3)
         .map(|w| w.iter().sum::<i32>() as f64 / 3.0)
         .collect();
     println!("Moving avg: {moving_avg:.1?}");
 
-    // chunks(2): non-overlapping groups of 2
-    // C: for (int i = 0; i < len; i += 2) process(arr[i], arr[i+1]);
+    // chunks(2): non-overlapping groups of 2 / 2 个一组的非重叠分块
+    // C: for (int i = 0; i < len; i += 2) process(arr[i], arr[i+1]); / C 风格固定间隔
     for pair in temps.chunks(2) {
         println!("Chunk: {pair:?}");
     }
 
-    // chunks_exact(2): same but panics if remainder exists
+    // chunks_exact(2): same but panics if remainder exists / 同样，但如果存在余数则会 panic
-    // Also: .remainder() gives leftover elements
+    // Also: .remainder() gives leftover elements / 此外：.remainder() 可获取剩余元素
 }
 ```
 
- ### `fold` and `scan` — Accumulation
+ ### `fold` and `scan` — Accumulation / 累加
 
 ```rust
 fn main() {
     let values = [10, 20, 30, 40, 50];
 
-    // fold: single final result (like C's accumulator loop)
+    // fold: single final result / fold：最终得到单个结果（类似 C 的累加循环）
     let sum = values.iter().fold(0, |acc, &x| acc + x);
-    println!("Sum: {sum}");  // 150
+    println!("Sum: {sum}");  // 结果：150
 
-    // Build a string with fold
+    // Build a string with fold / 使用 fold 构建字符串
     let csv = values.iter()
         .fold(String::new(), |acc, x| {
             if acc.is_empty() { format!("{x}") }
             else { format!("{acc},{x}") }
         });
-    println!("CSV: {csv}");  // "10,20,30,40,50"
+    println!("CSV: {csv}");  // 结果："10,20,30,40,50"
 
-    // scan: like fold but yields intermediate results
+    // scan: like fold but yields intermediate results / scan：类似 fold 但产出中间结果
     let running_sum: Vec<i32> = values.iter()
         .scan(0, |state, &x| {
             *state += x;
             Some(*state)
         })
         .collect();
-    println!("Running sum: {running_sum:?}");  // [10, 30, 60, 100, 150]
+    println!("Running sum: {running_sum:?}");  // 结果：[10, 30, 60, 100, 150]
 }
 ```
 
- ### Exercise: Sensor Data Pipeline
+ ### Exercise: Sensor Data Pipeline / 练习：传感器数据流水线
 
- Given raw sensor readings (one per line, format `"sensor_name:value:unit"`), write an
+ 给定原始传感器读数（每行一条，格式为 `"sensor_name:value:unit"`），编写一个
- iterator pipeline that:
- 1. Parses each line into `(name, f64, unit)`
+ 迭代器流水线，用于：
+ 1. 将每一行解析为 `(name, f64, unit)`
- 2. Filters out readings below a threshold
+ 2. 过滤掉低于阈值的读数
- 3. Groups by sensor name using `fold` into a `HashMap`
+ 3. 使用 `fold` 按传感器名称分组并存入 `HashMap`
- 4. Prints the average reading per sensor
+ 4. 打印每个传感器的平均读数
 
 ```rust
- // Starter code
+ // Starter code / 入门代码
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
-    // TODO: Parse, filter values >= threshold, group by name, compute averages
+    // TODO: Parse, filter values >= threshold, group by name, compute averages / 待办：解析、过滤 >= 阈值的值、按名称分组、计算平均值
 }
 ```
 
- <details><summary>Solution (click to expand)</summary>
+ <details><summary>Solution (click to expand) / 解决方案（点击展开）</summary>
 
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
 
-    // Parse → filter → group → average
+    // Pipeline: Parse → filter → group → average / 流水线：解析 → 过滤 → 分组 → 平均
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
         println!("{name}: avg={avg:.1} ({} readings)", values.len());
     }
 }
- // Output (order may vary):
+ // Output / 输出（顺序可能不同）：
 // gpu_temp: avg=75.6 (3 readings)
 // fan_rpm: avg=1175.0 (2 readings)
 ```
 
 </details>
 
- # Rust iterators
+ # Rust iterators continued / Rust 迭代器（续）
- - The ```Iterator``` trait is used to implement iteration over user defined types (https://doc.rust-lang.org/std/iter/trait.IntoIterator.html)
+ - ```Iterator``` trait 用于为用户定义类型实现迭代（https://doc.rust-lang.org/std/iter/trait.IntoIterator.html）。
-     - In the example, we'll implement an iterator for the Fibonacci sequence, which starts with 1, 1, 2, ... and the successor is the sum of the previous two numbers
+     - 在示例中，我们将为斐波那契数列实现一个迭代器，该数列以 1, 1, 2, ... 开始，后继项是前两项之和。
-     - The ```associated type``` in the ```Iterator``` (```type Item = u32;```) defines the output type from our iterator (```u32```)
+     - ```Iterator``` 中的 ```associated type```（关联类型，```type Item = u32;```）定义了我们迭代器的输出类型（```u32```）。
-     - The ```next()``` method simply contains the logic for implementing our iterator. In this case, all state information is available in the ```Fibonacci``` structure
+     - ```next()``` 方法包含了实现迭代器的逻辑。在本例中，所有状态信息都在 ```Fibonacci``` 结构体中。
-     - We could have implemented another trait called ```IntoIterator``` to implement the ```into_iter()``` method for more specialized iterators
+     - 我们还可以实现另一个名为 ```IntoIterator``` 的 trait，以便为更专门的迭代器实现 ```into_iter()``` 方法。
-     - https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=ab367dc2611e1b5a0bf98f1185b38f3f
+     - https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=ab367dc2611e1b5a0bf98f1185b38f3f
