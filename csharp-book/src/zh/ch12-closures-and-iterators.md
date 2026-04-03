[English Original](../en/ch12-closures-and-iterators.md)

## Rust 闭包 (Closures)

> **你将学到：** 具有所有权感知捕获能力的闭包 (`Fn`/`FnMut`/`FnOnce`) 与 C# Lambda 表达式的对比；作为 LINQ 零成本替代方案的 Rust 迭代器；延迟加载 (Lazy) 与及早求值 (Eager)；以及使用 `rayon` 实现的并行迭代。
>
> **难度：** 🟡 中级

Rust 中的闭包类似于 C# 的 Lambda 表达式和委托 (Delegates)，但增加了对所有权感知的捕获能力。

### C# Lambda 表达式与委托
```csharp
// C# - Lambda 表达式通过引用捕获
Func<int, int> doubler = x => x * 2;
Action<string> printer = msg => Console.WriteLine(msg);

// 捕获外部变量的闭包
int multiplier = 3;
Func<int, int> multiply = x => x * multiplier;
Console.WriteLine(multiply(5)); // 15

// LINQ 广泛使用了 Lambda 表达式
var evens = numbers.Where(n => n % 2 == 0).ToList();
```

### Rust 闭包
```rust
// Rust 闭包 - 具有所有权感知能力
let doubler = |x: i32| x * 2;
let printer = |msg: &str| println!("{}", msg);

// 默认通过引用进行捕获 (对于不可变项)
let multiplier = 3;
let multiply = |x: i32| x * multiplier; // 借用 multiplier
println!("{}", multiply(5)); // 15
println!("{}", multiplier); // 依然可以访问

// 通过 move 关键字捕获所有权
let data = vec![1, 2, 3];
let owns_data = move || {
    println!("{:?}", data); // data 被移动到了闭包内部
};
owns_data();
// println!("{:?}", data); // ❌ 错误：data 已经被移动了

// 在迭代器中使用闭包
let numbers = vec![1, 2, 3, 4, 5];
let evens: Vec<&i32> = numbers.iter().filter(|&&n| n % 2 == 0).collect();
```

### 闭包类型
```rust
// Fn - 以不可变方式借用捕获的值
fn apply_fn(f: impl Fn(i32) -> i32, x: i32) -> i32 {
    f(x)
}

// FnMut - 以可变方式借用捕获的值
fn apply_fn_mut(mut f: impl FnMut(i32), values: &[i32]) {
    for &v in values {
        f(v);
    }
}

// FnOnce - 获取被捕获值的所有权
fn apply_fn_once(f: impl FnOnce() -> Vec<i32>) -> Vec<i32> {
    f() // 只能调用一次
}

fn main() {
    // Fn 示例
    let multiplier = 3;
    let result = apply_fn(|x| x * multiplier, 5);
    
    // FnMut 示例
    let mut sum = 0;
    apply_fn_mut(|x| sum += x, &[1, 2, 3, 4, 5]);
    println!("总和: {}", sum); // 15
    
    // FnOnce 示例
    let data = vec![1, 2, 3];
    let result = apply_fn_once(move || data); // 移动数据
}
```

---

## LINQ vs Rust 迭代器

### C# LINQ (语言集成查询)
```csharp
// C# LINQ - 声明式数据处理
var numbers = new[] { 1, 2, 3, 4, 5, 6, 7, 8, 9, 10 };

var result = numbers
    .Where(n => n % 2 == 0)           // 过滤偶数
    .Select(n => n * n)               // 平方运算
    .Where(n => n > 10)               // 过滤大于 10 的项
    .OrderByDescending(n => n)        // 降序排列
    .Take(3)                          // 取前 3 项
    .ToList();                        // 实体化

// 处理复杂对象的 LINQ
var users = GetUsers();
var activeAdults = users
    .Where(u => u.IsActive && u.Age >= 18)
    .GroupBy(u => u.Department)
    .Select(g => new {
        Department = g.Key,
        Count = g.Count(),
        AverageAge = g.Average(u => u.Age)
    })
    .OrderBy(x => x.Department)
    .ToList();

// 异步 LINQ (需要额外类库支持)
var results = await users
    .ToAsyncEnumerable()
    .WhereAwait(async u => await IsActiveAsync(u.Id))
    .SelectAwait(async u => await EnrichUserAsync(u))
    .ToListAsync();
```

### Rust 迭代器
```rust
// Rust 迭代器 - 延迟加载、零成本抽象
let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

let result: Vec<i32> = numbers
    .iter()
    .filter(|&&n| n % 2 == 0)        // 过滤偶数
    .map(|&n| n * n)                 // 平方运算
    .filter(|&n| n > 10)             // 过滤大于 10 的项
    .collect::<Vec<_>>()             // 收集到 Vec
    .into_iter()
    .rev()                           // 反转迭代顺序
    .take(3)                         // 取前 3 项
    .collect();                      // 实体化

// 复杂的迭代器链
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct User {
    name: String,
    age: u32,
    department: String,
    is_active: bool,
}

fn process_users(users: Vec<User>) -> HashMap<String, (usize, f64)> {
    users
        .into_iter()
        .filter(|u| u.is_active && u.age >= 18)
        .fold(HashMap::new(), |mut acc, user| {
            let entry = acc.entry(user.department.clone()).or_insert((0, 0.0));
            entry.0 += 1;  // 计数
            entry.1 += user.age as f64;  // 年龄总和
            acc
        })
        .into_iter()
        .map(|(dept, (count, sum))| (dept, (count, sum / count as f64)))  // 计算平均值
        .collect()
}

// 使用 rayon 进行并行处理
use rayon::prelude::*;

fn parallel_processing(numbers: Vec<i32>) -> Vec<i32> {
    numbers
        .par_iter()                  // 并行迭代器
        .filter(|&&n| n % 2 == 0)
        .map(|&n| expensive_computation(n))
        .collect()
}

fn expensive_computation(n: i32) -> i32 {
    // 模拟重度计算
    (0..1000).fold(n, |acc, _| acc + 1)
}
```

```mermaid
graph TD
    subgraph "C# LINQ 特性"
        CS_LINQ["LINQ 表达式"]
        CS_EAGER["常为及早求值 (Eager)<br/>(ToList(), ToArray())"]
        CS_REFLECTION["[错误] 部分运行时反射<br/>表达式树 (Expression trees)"]
        CS_ALLOCATIONS["[错误] 中间集合<br/>GC (垃圾回收) 压力"]
        CS_ASYNC["[OK] 异步支持<br/>(需额外库支持)"]
        CS_SQL["[OK] LINQ to SQL/EF 集成"]
        
        CS_LINQ --> CS_EAGER
        CS_LINQ --> CS_REFLECTION
        CS_LINQ --> CS_ALLOCATIONS
        CS_LINQ --> CS_ASYNC
        CS_LINQ --> CS_SQL
    end
    
    subgraph "Rust 迭代器特性"
        RUST_ITER["迭代器链"]
        RUST_LAZY["[OK] 延迟加载 (Lazy)<br/>在 .collect() 之前不执行操作"]
        RUST_ZERO["[OK] 零成本抽象<br/>编译后等效于最优循环"]
        RUST_NO_ALLOC["[OK] 无中间过程内存分配<br/>基于栈的处理方式"]
        RUST_PARALLEL["[OK] 极其简单的并行化<br/>(rayon 库)"]
        RUST_FUNCTIONAL["[OK] 函数式编程风格<br/>默认不可变"]
        
        RUST_ITER --> RUST_LAZY
        RUST_ITER --> RUST_ZERO
        RUST_ITER --> RUST_NO_ALLOC
        RUST_ITER --> RUST_PARALLEL
        RUST_ITER --> RUST_FUNCTIONAL
    end
    
    subgraph "性能对比"
        CS_PERF["C# LINQ 性能<br/>[错误] 分配内存开销<br/>[错误] 虚方法分发<br/>[OK] 对多数场景已足够"]
        RUST_PERF["Rust 迭代器性能<br/>[OK] 手工优化级别的速度<br/>[OK] 无内存分配<br/>[OK] 编译时优化"]
    end
    
    style CS_REFLECTION fill:#ffcdd2,color:#000
    style CS_ALLOCATIONS fill:#fff3e0,color:#000
    style RUST_ZERO fill:#c8e6c9,color:#000
    style RUST_LAZY fill:#c8e6c9,color:#000
    style RUST_NO_ALLOC fill:#c8e6c9,color:#000
    style CS_PERF fill:#fff3e0,color:#000
    style RUST_PERF fill:#c8e6c9,color:#000
```

---

<details>
<summary><strong>🏋️ 练习：将 LINQ 翻译为迭代器</strong> (点击展开)</summary>

**挑战**：将这段 C# LINQ 工作流翻译为惯用的 Rust 迭代器。

```csharp
// C# — 翻译为 Rust
record Employee(string Name, string Dept, int Salary);

var result = employees
    .Where(e => e.Salary > 50_000)
    .GroupBy(e => e.Dept)
    .Select(g => new {
        Department = g.Key,
        Count = g.Count(),
        AvgSalary = g.Average(e => e.Salary)
    })
    .OrderByDescending(x => x.AvgSalary)
    .ToList();
```

<details>
<summary>🔑 参考答案</summary>

```rust
use std::collections::HashMap;

struct Employee { name: String, dept: String, salary: u32 }

#[derive(Debug)]
struct DeptStats { department: String, count: usize, avg_salary: f64 }

fn department_stats(employees: &[Employee]) -> Vec<DeptStats> {
    let mut by_dept: HashMap<&str, Vec<u32>> = HashMap::new();
    for e in employees.iter().filter(|e| e.salary > 50_000) {
        by_dept.entry(&e.dept).or_default().push(e.salary);
    }

    let mut stats: Vec<DeptStats> = by_dept
        .into_iter()
        .map(|(dept, salaries)| {
            let count = salaries.len();
            let avg = salaries.iter().sum::<u32>() as f64 / count as f64;
            DeptStats { department: dept.to_string(), count, avg_salary: avg }
        })
        .collect();

    stats.sort_by(|a, b| b.avg_salary.partial_cmp(&a.avg_salary).unwrap());
    stats
}
```

**关键收获**：
- Rust 迭代器没有内置的 `group_by` —— 使用 `HashMap` + `fold`/`for` 是最地道的模式。
- `itertools` 库提供了 `.group_by()`，可以实现更接近 LINQ 的语法。
- 迭代器链是零成本的 —— 编译器会将其优化为简单的循环代码。

</details>
</details>

## itertools：增强版 LINQ 工具库

Rust 标准库迭代器涵盖了 `map`, `filter`, `fold`, `take` 和 `collect`。但对于习惯了 `GroupBy`, `Zip`, `Chunk`, `SelectMany` 和 `Distinct` 的 C# 开发者来说，可能会感到有所缺憾。**`itertools`** 库填补了这些空白。

```toml
# Cargo.toml
[dependencies]
itertools = "0.12"
```

### 功能对比：LINQ vs itertools

```csharp
// C# — GroupBy
var byDept = employees.GroupBy(e => e.Department)
    .Select(g => new { Dept = g.Key, Count = g.Count() });

// C# — Chunk (批处理)
var batches = items.Chunk(100);  // IEnumerable<T[]>

// C# — Distinct / DistinctBy
var unique = users.DistinctBy(u => u.Email);

// C# — SelectMany (扁平化)
var allTags = posts.SelectMany(p => p.Tags);

// C# — Zip
var pairs = names.Zip(scores, (n, s) => new { Name = n, Score = s });

// C# — 滑动窗口 (Sliding window)
var windows = data.Zip(data.Skip(1), data.Skip(2))
    .Select(triple => (triple.First + triple.Second + triple.Third) / 3.0);
```

```rust
use itertools::Itertools;

// Rust — group_by (要求输入已排序)
let by_dept = employees.iter()
    .sorted_by_key(|e| &e.department)
    .group_by(|e| &e.department);
for (dept, group) in &by_dept {
    println!("{}: {} 名员工", dept, group.count());
}

// Rust — chunks (批处理)
let batches = items.iter().chunks(100);
for batch in &batches {
    process_batch(batch.collect::<Vec<_>>());
}

// Rust — unique / unique_by
let unique: Vec<_> = users.iter().unique_by(|u| &u.email).collect();

// Rust — flat_map (等效于 SelectMany —— 标准库自带！)
let all_tags: Vec<&str> = posts.iter().flat_map(|p| &p.tags).collect();

// Rust — zip (标准库自带！)
let pairs: Vec<_> = names.iter().zip(scores.iter()).collect();

// Rust — tuple_windows (滑动窗口)
let moving_avg: Vec<f64> = data.iter()
    .tuple_windows::<(_, _, _)>()
    .map(|(a, b, c)| (*a + *b + *c) as f64 / 3.0)
    .collect();
```

### itertools 快速参考

| LINQ 方法 | itertools 对应项 | 备注 |
|------------|---------------------|-------|
| `GroupBy(key)` | `.sorted_by_key().group_by()` | 需要已排序输入 (不同于 LINQ) |
| `Chunk(n)` | `.chunks(n)` | 返回迭代器的迭代器 |
| `Distinct()` | `.unique()` | 需要实现 `Eq + Hash` |
| `DistinctBy(key)` | `.unique_by(key)` | |
| `SelectMany()` | `.flat_map()` | 标准库内置 —— 无需额外库 |
| `Zip()` | `.zip()` | 标准库内置 |
| `Aggregate()` | `.fold()` | 标准库内置 |
| `Any()` / `All()` | `.any()` / `.all()` | 标准库内置 |
| `First()` / `Last()` | `.next()` / `.last()` | 标准库内置 |
| `Skip(n)` / `Take(n)` | `.skip(n)` / `.take(n)` | 标准库内置 |
| `OrderBy()` | `.sorted()` / `.sorted_by()` | 工具库提供 (标准库未直接提供) |
| `ThenBy()` | `.sorted_by(\|a,b\| a.x.cmp(&b.x).then(a.y.cmp(&b.y)))` | 串联 `Ordering::then` |
| `Intersect()` | `HashSet` 的交集操作 | 无直接的迭代器方法 |
| `Concat()` | `.chain()` | 标准库内置 |
| 滑动窗口 | `.tuple_windows()` | 固定大小的元组 |
| 笛卡尔积 | `.cartesian_product()` | 工具库提供 |
| 交错组合 | `.interleave()` | 工具库提供 |
| 排列组合 | `.permutations(k)` | 工具库提供 |

### 实践案例：日志分析流水线

```rust
use itertools::Itertools;
use std::collections::HashMap;

#[derive(Debug)]
struct LogEntry { level: String, module: String, message: String }

fn analyze_logs(entries: &[LogEntry]) {
    // 找出日志量最大的前 5 个模块 (类似于 LINQ 的 GroupBy + OrderByDescending + Take)
    let noisy: Vec<_> = entries.iter()
        .into_group_map_by(|e| &e.module) // itertools: 直接归组到 HashMap
        .into_iter()
        .sorted_by(|a, b| b.1.len().cmp(&a.1.len()))
        .take(5)
        .collect();

    for (module, entries) in &noisy {
        println!("{}: {} 条日志", module, entries.len());
    }

    // 每 100 条日志为一个窗口计算错误率 (滑动窗口)
    let error_rates: Vec<f64> = entries.iter()
        .map(|e| if e.level == "ERROR" { 1.0 } else { 0.0 })
        .collect::<Vec<_>>()
        .windows(100)  // 标准库切片方法
        .map(|w| w.iter().sum::<f64>() / 100.0)
        .collect();

    // 剔除连续的重复相同消息
    let deduped: Vec<_> = entries.iter().dedup_by(|a, b| a.message == b.message).collect();
    println!("去重：{} → {} 条日志", entries.len(), deduped.len());
}
```

---
