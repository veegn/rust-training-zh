## Rust closures / Rust 闭包
 
 > **What you'll learn / 你将学到：** Closures as anonymous functions, the three capture traits (`Fn`, `FnMut`, `FnOnce`), `move` closures, and how Rust closures compare to C++ lambdas — with automatic capture analysis instead of manual `[&]`/`[=]` specifications.
 >
 > 闭包作为匿名函数、三种捕获 trait（`Fn`、`FnMut`、`FnOnce`）、`move` 闭包，以及 Rust 闭包与 C++ lambda 的对比 —— Rust 具有自动捕获分析，而非手动的 `[&]` / `[=]` 规范。
 
 - Closures are anonymous functions that can capture their environment
+ - 闭包是可以捕获其环境的匿名函数。
-     - C++ equivalent: lambdas (`[&](int x) { return x + 1; }`)
+     - C++ 等价物：lambda 表达式（`[&](int x) { return x + 1; }`）
-     - Key difference: Rust closures have **three** capture traits (`Fn`, `FnMut`, `FnOnce`) that the compiler selects automatically
+     - 关键区别：Rust 闭包有**三种**捕获 trait（`Fn`、`FnMut`、`FnOnce`），编译器会自动选择。
-     - C++ capture modes (`[=]`, `[&]`, `[this]`) are manual and error-prone (dangling `[&]`!)
+     - C++ 捕获模式（`[=]`、`[&]`、`[this]`）是手动的且容易出错（容易产生悬挂引用的 `[&]`！）。
-     - Rust's borrow checker prevents dangling captures at compile time
+     - Rust 的借用检查器在编译时防止悬挂捕获。
- - Closures can be identified by the `||` symbol. The parameters for the types are enclosed within the `||` and can use type inference
+ - 闭包通过 `||` 符号来识别。参数类型包含在 `||` 之内，并且可以使用类型推断。
- - Closures are frequently used in conjunction with iterators (next topic)
+ - 闭包经常与迭代器（下一个主题）结合使用。
 ```rust
 fn add_one(x: u32) -> u32 {
     x + 1
 }
 fn main() {
-    let add_one_v1 = |x : u32| {x + 1}; // Explicitly specified type
+    let add_one_v1 = |x : u32| {x + 1}; // Explicitly specified / 显式指定类型
-    let add_one_v2 = |x| {x + 1};   // Type is inferred from call site
+    let add_one_v2 = |x| {x + 1};   // Inferred / 类型由调用处推断
-    let add_one_v3 = |x| x+1;   // Permitted for single line functions
+    let add_one_v3 = |x| x+1;   // Allowed for single line / 允许用于单行函数
     println!("{} {} {} {}", add_one(42), add_one_v1(42), add_one_v2(42), add_one_v3(42) );
 }
 ```
 
- # Exercise: Closures and capturing
+ # Exercise: Closures and capturing / 练习：闭包与捕获
 
- 🟡 **Intermediate**
+ 🟡 **Intermediate / 中级**
 
- - Create a closure that captures a `String` from the enclosing scope and appends to it (hint: use `move`)
+ - 创建一个从外层作用域捕获 `String` 并向其追加内容的闭包（提示：使用 `move`）。
- - Create a vector of closures: `Vec<Box<dyn Fn(i32) -> i32>>` containing closures that add 1, multiply by 2, and square the input. Iterate over the vector and apply each closure to the number 5
+ - 创建一个闭包向量：`Vec<Box<dyn Fn(i32) -> i32>>`，其中包含加 1、乘 2 和平方输入的闭包。遍历向量并将每个闭包应用于数字 5。
 
- <details><summary>Solution (click to expand)</summary>
+ <details><summary>Solution (click to expand) / 解决方案（点击展开）</summary>
 
 ```rust
 fn main() {
-    // Part 1: Closure that captures and appends to a String
+    // Part 1: Capture and append / 第 1 部分：捕获并追加到 String
     let mut greeting = String::from("Hello");
     let mut append = |suffix: &str| {
         greeting.push_str(suffix);
     };
     append(", world");
     append("!");
     println!("{greeting}");  // "Hello, world!"
 
-    // Part 2: Vector of closures
+    // Part 2: Vector of closures / 第 2 部分：闭包向量
     let operations: Vec<Box<dyn Fn(i32) -> i32>> = vec![
-        Box::new(|x| x + 1),      // add 1
+        Box::new(|x| x + 1),      // add 1 / 加 1
-        Box::new(|x| x * 2),      // multiply by 2
+        Box::new(|x| x * 2),      // multiply by 2 / 乘 2
-        Box::new(|x| x * x),      // square
+        Box::new(|x| x * x),      // square / 平方
     ];
 
     let input = 5;
     for (i, op) in operations.iter().enumerate() {
         println!("Operation {i} on {input}: {}", op(input));
     }
 }
- // Output:
+ // Output / 输出：
 // Hello, world!
 // Operation 0 on 5: 6
 // Operation 1 on 5: 10
 // Operation 2 on 5: 25
 ```
 
 </details>
 
- # Rust iterators
+ # Rust iterators / Rust 迭代器
- - Iterators are one of the most powerful features of Rust. They enable very elegant methods for perform operations on collections, including filtering (```filter()```), transformation (```map()```), filter and map (```filter_and_map()```), searching (```find()```) and much more
+ - 迭代器是 Rust 最强大的特性之一。它们为在集合上执行操作提供了非常优雅的方法，包括过滤（```filter()```）、转换（```map()```）、过滤并转换（```filter_and_map()```）、搜索（```find()```）等等。
- - In the example below, the ```|&x| *x >= 42``` is a closure that performs the same comparison. The ```|x| println!("{x}")``` is another closure
+ - 在下面的示例中，```|&x| *x >= 42``` 是一个执行相同比较的闭包。```|x| println!("{x}")``` 是另一个闭包。
 ```rust
 fn main() {
     let a = [0, 1, 2, 3, 42, 43];
     for x in &a {
         if *x >= 42 {
             println!("{x}");
         }
     }
-    // Same as above
+    // Same as above / 与上面相同
     a.iter().filter(|&x| *x >= 42).for_each(|x| println!("{x}"))
 }
 ```
 
- # Rust iterators
+ # Rust iterators continued / Rust 迭代器（续）
- - A key feature of iterators is that most of them are ```lazy```, i.e., they do not do anything until they are evaluated. For example, ```a.iter().filter(|&x| *x >= 42);``` wouldn't have done *anything* without the ```for_each```. The Rust compiler emits an explicit warning when it detects such a situation
+ - 迭代器的一个关键特性是大多数迭代器都是**惰性（lazy）**的，即在被求值之前它们不会执行任何操作。例如，如果没有 ```for_each```，```a.iter().filter(|&x| *x >= 42);``` 什么也不会做。当 Rust 编译器检测到这种情况时，会发出显式的警告。
 ```rust
 fn main() {
     let a = [0, 1, 2, 3, 42, 43];
-    // Add one to each element and print it
+    // Add one and print / 对每个元素加 1 并打印
     let _ = a.iter().map(|x|x + 1).for_each(|x|println!("{x}"));
     let found = a.iter().find(|&x|*x == 42);
     println!("{found:?}");
-    // Count elements
+    // Count elements / 统计元素数量
     let count = a.iter().count();
     println!("{count}");
 }
 ```
 
- # Rust iterators
+ # Rust iterators continued / Rust 迭代器（续）
- - The ```collect()``` method can be used to gather the results into a separate collection
+ - ```collect()``` 方法可用于将结果收集到一个单独的集合中。
-     - In the below the ```_``` in ```Vec<_>``` is the equivalent of a wildcard character for the type returned by the ```map```. For example, we can even return a ```String``` from ```map``` 
+     - 在下面，```Vec<_>``` 中的 ```_``` 相当于 ```map``` 返回类型的通配符。例如，我们甚至可以从 ```map``` 中返回一个 ```String```。
 ```rust
 fn main() {
     let a = [0, 1, 2, 3, 42, 43];
     let squared_a : Vec<_> = a.iter().map(|x|x*x).collect();
     for x in &squared_a {
         println!("{x}");
     }
-    let squared_a_strings : Vec<_> = a.iter().map(|x|(x*x).to_string()).collect();
+    let squared_a_strings : Vec<_> = a.iter().map(|x|(x*x).to_string()).collect(); // 收集为字符串向量
-    // These are actually string representations
+    // These are actually string representations / 这些实际上是字符串表示
     for x in &squared_a_strings {
         println!("{x}");
     }
 }
 ```
 
- # Exercise: Rust iterators
+ # Exercise: Rust iterators / 练习：Rust 迭代器
 
- 🟢 **Starter**
+ 🟢 **Starter / 入门级**
- - Create an integer array composed of odd and even elements. Iterate over the array and split it into two different vectors with even and odd elements in each
+ - 创建一个包含奇数和偶数元素的整数数组。遍历该数组并将其拆分为两个分别包含偶数和奇数元素的向量。
- - Can this be done in a single pass (hint: use ```partition()```)?
+ - 这能在一次遍历中完成吗（提示：使用 ```partition()```）？
 
- <details><summary>Solution (click to expand)</summary>
+ <details><summary>Solution (click to expand) / 解决方案（点击展开）</summary>
 
 ```rust
 fn main() {
     let numbers = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
 
-    // Approach 1: Manual iteration
+    // Approach 1: Manual iteration / 方法 1：手动迭代
     let mut evens = Vec::new();
     let mut odds = Vec::new();
     for n in numbers {
         if n % 2 == 0 {
             evens.push(n);
         } else {
             odds.push(n);
         }
     }
     println!("Evens: {evens:?}");
     println!("Odds:  {odds:?}");
 
-    // Approach 2: Single pass with partition()
+    // Approach 2: Single pass with partition() / 方法 2：使用 partition() 一次完成
     let (evens, odds): (Vec<i32>, Vec<i32>) = numbers
         .into_iter()
         .partition(|n| n % 2 == 0);
     println!("Evens (partition): {evens:?}");
     println!("Odds  (partition): {odds:?}");
 }
- // Output:
+ // Output / 输出：
 // Evens: [2, 4, 6, 8, 10]
 // Odds:  [1, 3, 5, 7, 9]
 // Evens (partition): [2, 4, 6, 8, 10]
 // Odds  (partition): [1, 3, 5, 7, 9]
 ```
 
 </details>
 
- > **Production patterns**: See [Collapsing assignment pyramids with closures](ch17-3-collapsing-assignment-pyramids.md#collapsing-assignment-pyramids-with-closures) for real iterator chains (`.map().collect()`, `.filter().collect()`, `.find_map()`) from production Rust code.
+ > **Production patterns / 生产环境模式**：有关生产环境 Rust 代码中真正的迭代器链（`.map().collect()`、`.filter().collect()`、`.find_map()`）示例，请参阅 [Collapsing assignment pyramids with closures / 使用闭包精简赋值金字塔](ch17-3-collapsing-assignment-pyramids.md#collapsing-assignment-pyramids-with-closures)。
 
- ### Iterator power tools: the methods that replace C++ loops
+ ### Iterator power tools: the methods that replace C++ loops / 迭代器进阶工具：取代 C++ 循环的方法
 
- The following iterator adapters are used *extensively* in production Rust code. C++ has
+ 以下迭代器适配器在生产级 Rust 代码中被**广泛**使用。虽然 C++ 有
- `<algorithm>` and C++20 ranges, but Rust's iterator chains are more composable
+ `<algorithm>` 和 C++20 ranges，但 Rust 的迭代器链更具组合性
- and more commonly used.
+ 且使用更普遍。
 
- #### `enumerate` — index + value (replaces `for (int i = 0; ...)`)
+ #### `enumerate` — index + value (replaces `for (int i = 0; ...)` / 替换索引循环)
 
 ```rust
 let sensors = vec!["temp0", "temp1", "temp2"];
 for (idx, name) in sensors.iter().enumerate() {
     println!("Sensor {idx}: {name}");
 }
 // Sensor 0: temp0
 // Sensor 1: temp1
 // Sensor 2: temp2
 ```
 
- C++ equivalent: `for (size_t i = 0; i < sensors.size(); ++i) { auto& name = sensors[i]; ... }`
+ C++ 等价物：`for (size_t i = 0; i < sensors.size(); ++i) { auto& name = sensors[i]; ... }`
 
- #### `zip` — pair elements from two iterators (replaces parallel index loops)
+ #### `zip` — pair elements from two iterators (replaces parallel index loops / 替换并行索引循环)
 
 ```rust
 let names = ["gpu0", "gpu1", "gpu2"];
 let temps = [72.5, 68.0, 75.3];
 
 let report: Vec<String> = names.iter()
     .zip(temps.iter())
     .map(|(name, temp)| format!("{name}: {temp}°C"))
     .collect();
 println!("{report:?}");
 // ["gpu0: 72.5°C", "gpu1: 68.0°C", "gpu2: 75.3°C"]
 
- // Stops at the shorter iterator — no out-of-bounds risk
+ // Stops at the shorter iterator / 在较短的迭代器处停止 —— 无越界风险
 ```
 
- C++ equivalent: `for (size_t i = 0; i < std::min(names.size(), temps.size()); ++i) { ... }`
+ C++ 等价物：`for (size_t i = 0; i < std::min(names.size(), temps.size()); ++i) { ... }`
 
- #### `flat_map` — map + flatten nested collections
+ #### `flat_map` — map + flatten nested collections / map + 拍平嵌套集合
 
 ```rust
- // Each GPU has multiple PCIe BDFs; collect all BDFs across all GPUs
+ // Each GPU has multiple PCIe BDFs / 每个 GPU 有多个 PCIe BDF；收集所有 GPU 的所有 BDF
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
 
- C++ equivalent: nested `for` loop pushing into a single vector.
+ C++ 等价物：嵌套 `for` 循环并将结果推入单个 vector。
 
- #### `chain` — concatenate two iterators
+ #### `chain` — concatenate two iterators / 连接两个迭代器
 
 ```rust
 let critical_gpus = vec!["gpu0", "gpu3"];
 let warning_gpus = vec!["gpu1", "gpu5"];
 
- // Process all flagged GPUs, critical first
+ // Process all flagged GPUs / 处理所有标记的 GPU，优先处理 critical（危急）
 for gpu in critical_gpus.iter().chain(warning_gpus.iter()) {
     println!("Flagged: {gpu}");
 }
 ```
 
- #### `windows` and `chunks` — sliding/fixed-size views over slices
+ #### `windows` and `chunks` — sliding/fixed-size views over slices / 切片上的滑动/固定大小视图
 
 ```rust
 let temps = [70, 72, 75, 73, 71, 68, 65];
 
- // windows(3): sliding window of size 3 — detect trends
+ // windows(3): sliding window of size 3 / 窗口大小为 3 的滑动窗口 —— 检测趋势
 let rising = temps.windows(3)
     .any(|w| w[0] < w[1] && w[1] < w[2]);
- println!("Rising trend detected: {rising}"); // true (70 < 72 < 75)
+ println!("Rising trend detected: {rising}"); // true / 是 (70 < 72 < 75)
 
- // chunks(2): fixed-size groups — process in pairs
+ // chunks(2): fixed-size groups / 固定大小的分组 —— 成对处理
 for pair in temps.chunks(2) {
     println!("Pair: {pair:?}");
 }
 // Pair: [70, 72]
 // Pair: [75, 73]
 // Pair: [71, 68]
- // Pair: [65]       ← last chunk can be smaller
+ // Pair: [65]       ← last chunk can be smaller / 最后一个分块可以更小
 ```
 
- C++ equivalent: manual index arithmetic with `i` and `i+1`/`i+2`.
+ C++ 等价物：手动的索引算术，如 `i` 和 `i+1` / `i+2`。
 
- #### `fold` — accumulate into a single value (replaces `std::accumulate`)
+ #### `fold` — accumulate into a single value (replaces `std::accumulate` / 替换累加)
 
 ```rust
 let errors = vec![
     ("gpu0", 3u32),
     ("gpu1", 0),
     ("gpu2", 7),
     ("gpu3", 1),
 ];
 
- // Count total errors and build summary in one pass
+ // Count total errors and summary / 一次完成总错误统计及详情摘要构建
 let (total, summary) = errors.iter().fold(
     (0u32, String::new()),
     |(count, mut s), (name, errs)| {
         if *errs > 0 {
             s.push_str(&format!("{name}:{errs} "));
         }
         (count + errs, s)
     },
 );
 println!("Total errors: {total}, details: {summary}");
 // Total errors: 11, details: gpu0:3 gpu2:7 gpu3:1
 ```
 
- #### `scan` — stateful transform (running total, delta detection)
+ #### `scan` — stateful transform (running total, delta detection) / 有状态转换（运行总计、增量检测）
 
 ```rust
 let readings = [100, 105, 103, 110, 108];
 
- // Compute deltas between consecutive readings
+ // Compute deltas / 计算连续读数之间的增量
 let deltas: Vec<i32> = readings.iter()
     .scan(None::<i32>, |prev, &val| {
         let delta = prev.map(|p| val - p);
         *prev = Some(val);
         Some(delta)
     })
-    .flatten()  // Remove the initial None
+    .flatten()  // Remove initial None / 移除初始的 None
     .collect();
- println!("Deltas: {deltas:?}"); // [5, -2, 7, -2]
+ println!("Deltas: {deltas:?}"); // 结果：[5, -2, 7, -2]
 ```
 
- #### Quick reference: C++ loop → Rust iterator
+ #### Quick reference: C++ loop → Rust iterator / 快速参考：C++ 循环 → Rust 迭代器
 
-| **C++ Pattern** | **Rust Iterator** | **Example** |
+| **C++ Pattern / 模式** | **Rust Iterator / 迭代器** | **Example / 示例** |
 |----------------|------------------|------------|
-| `for (int i = 0; i < v.size(); i++)` | `.enumerate()` | `v.iter().enumerate()` |
+| `for (int i = 0; i < v.size(); i++)` | `.enumerate()` | `v.iter().enumerate()` |
-| Parallel iteration with index | `.zip()` | `a.iter().zip(b.iter())` |
-| Parallel iteration with index | `.zip()` | `a.iter().zip(b.iter())` |
-| Nested loop → flat result | `.flat_map()` | `vecs.iter().flat_map(\|v\| v.iter())` |
-| Nested loop → flat result | `.flat_map()` | `...` |
-| Concatenate two containers | `.chain()` | `a.iter().chain(b.iter())` |
-| Concatenate two containers | `.chain()` | `...` |
-| Sliding window `v[i..i+n]` | `.windows(n)` | `v.windows(3)` |
-| Sliding window `v[i..i+n]` | `.windows(n)` | `v.windows(3)` |
-| Process in fixed-size groups | `.chunks(n)` | `v.chunks(4)` |
-| Process in fixed-size groups | `.chunks(n)` | `v.chunks(4)` |
-| `std::accumulate` / manual accumulator | `.fold()` | `.fold(init, \|acc, x\| ...)` |
-| `std::accumulate` / manual accumulator | `.fold()` | `...` |
-| Running total / delta tracking | `.scan()` | `.scan(state, \|s, x\| ...)` |
-| Running total / delta tracking | `.scan()` | `...` |
-| `while (it != end && count < n) { ++it; ++count; }` | `.take(n)` | `.iter().take(5)` |
-| `while (it != end && count < n) { ++it; ++count; }` | `.take(n)` | `...` |
-| `while (it != end && !pred(*it)) { ++it; }` | `.skip_while()` | `.skip_while(\|x\| x < &threshold)` |
+| `while (it != end && !pred(*it)) { ++it; }` | `.skip_while()` | `...` |
-| `std::any_of` | `.any()` | `.iter().any(\|x\| x > &limit)` |
-| `std::any_of` | `.any()` | `.iter().any(\|x\| x > &limit)` |
-| `std::all_of` | `.all()` | `.iter().all(\|x\| x.is_valid())` |
-| `std::all_of` | `.all()` | `.iter().all(\|x\| x.is_valid())` |
-| `std::none_of` | `!.any()` | `!iter.any(\|x\| x.failed())` |
-| `std::none_of` | `!.any()` | `...` |
-| `std::count_if` | `.filter().count()` | `.filter(\|x\| x > &0).count()` |
-| `std::count_if` | `.filter().count()` | `...` |
-| `std::min_element` / `std::max_element` | `.min()` / `.max()` | `.iter().max()` → `Option<&T>` |
-| `std::min_element` / `std::max_element` | `.min()` / `.max()` | `...` |
-| `std::unique` | `.dedup()` (on sorted) | `v.dedup()` (in-place on Vec) |
-| `std::unique` | `.dedup()` | `...` |
 
- ### Exercise: Iterator chains
+ ### Exercise: Iterator chains / 练习：迭代器链
 
- Given sensor data as `Vec<(String, f64)>` (name, temperature), write a **single
+ 给定传感器数据为 `Vec<(String, f64)>`（名称、温度），编写一个**单一的迭代器链**，用于：
- iterator chain** that:
- 1. Filters sensors with temp > 80.0
+ 1. 过滤温度 > 80.0 的传感器
- 2. Sorts them by temperature (descending)
+ 2. 按温度排序（降序）
- 3. Formats each as `"{name}: {temp}°C [ALARM]"`
+ 3. 将每个格式化为 `"{name}: {temp}°C [ALARM]"`
- 4. Collects into `Vec<String>`
+ 4. 收集为 `Vec<String>`
 
- Hint: you'll need `.collect()` before `.sort_by()`, since sorting requires a `Vec`.
+ 提示：在调用 `.sort_by()` 之前你需要进行 `.collect()`，因为排序需要一个 `Vec`。
 
- <details><summary>Solution (click to expand)</summary>
+ <details><summary>Solution (click to expand) / 解决方案（点击展开）</summary>
 
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
- // Output:
+ // Output / 输出：
 // gpu2: 91°C [ALARM]
 // gpu4: 88.7°C [ALARM]
 // gpu1: 85.3°C [ALARM]
 ```
 
 </details>
 
 ---
 
- # Rust iterators
+ # Rust iterators continued / Rust 迭代器（续）
- - The ```Iterator``` trait is used to implement iteration over user defined types (https://doc.rust-lang.org/std/iter/trait.IntoIterator.html)
+ - ```Iterator``` trait 用于为用户定义的类型实现迭代功能（https://doc.rust-lang.org/std/iter/trait.IntoIterator.html）。
-     - In the example, we'll implement an iterator for the Fibonacci sequence, which starts with 1, 1, 2, ... and the successor is the sum of the previous two numbers
+     - 在示例中，我们将为斐波那契数列实现一个迭代器，该数列以 1, 1, 2, ... 开始，后继项是前两项之和。
-     - The ```associated type``` in the ```Iterator``` (```type Item = u32;```) defines the output type from our iterator (```u32```)
+     - ```Iterator``` 中的 ```associated type```（关联类型，```type Item = u32;```）定义了我们迭代器的输出类型（```u32```）。
-     - The ```next()``` method simply contains the logic for implementing our iterator. In this case, all state information is available in the ```Fibonacci``` structure
+     - ```next()``` 方法包含了实现迭代器的逻辑。在本例中，所有状态信息都在 ```Fibonacci``` 结构体中。
-     - We could have implemented another trait called ```IntoIterator``` to implement the ```into_iter()``` method for more specialized iterators
+     - 我们还可以实现另一个名为 ```IntoIterator``` 的 trait，以便为更专门的迭代器实现 ```into_iter()``` 方法。
-     - [▶ Try it in the Rust Playground](https://play.rust-lang.org/)
+     - [▶ 在 Rust Playground 中尝试](https://play.rust-lang.org/)
