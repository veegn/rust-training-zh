## Rust Closures | Rust 闭包

> **What you'll learn:** Closures with ownership-aware captures (`Fn`/`FnMut`/`FnOnce`) vs C# lambdas,
> Rust iterators as a zero-cost replacement for LINQ, lazy vs eager evaluation,
> and parallel iteration with `rayon`.
>
> **你将学到什么：** Rust 闭包与 C# lambda 的区别，特别是基于所有权的捕获方式（`Fn`/`FnMut`/`FnOnce`），
> Rust 迭代器如何以零成本方式替代 LINQ，惰性求值与急切求值的差异，
> 以及如何使用 `rayon` 实现并行迭代。
>
> **Difficulty:** Intermediate
>
> **难度：** 中级

Closures in Rust are similar to C# lambdas and delegates, but with ownership-aware captures.

Rust 的闭包和 C# 的 lambda、delegate 很像，但最大的区别在于：Rust 的捕获方式受所有权系统约束。

### C# Lambdas and Delegates | C# Lambda 与委托
```csharp
// C# - Lambdas capture by reference
Func<int, int> doubler = x => x * 2;
Action<string> printer = msg => Console.WriteLine(msg);

// Closure capturing outer variables
int multiplier = 3;
Func<int, int> multiply = x => x * multiplier;
Console.WriteLine(multiply(5)); // 15

// LINQ uses lambdas extensively
var evens = numbers.Where(n => n % 2 == 0).ToList();
```

### Rust Closures | Rust 闭包
```rust
// Rust closures - ownership-aware
let doubler = |x: i32| x * 2;
let printer = |msg: &str| println!("{}", msg);

// Closure capturing by reference (default for immutable)
let multiplier = 3;
let multiply = |x: i32| x * multiplier; // borrows multiplier
println!("{}", multiply(5)); // 15
println!("{}", multiplier); // still accessible

// Closure capturing by move
let data = vec![1, 2, 3];
let owns_data = move || {
    println!("{:?}", data); // data moved into closure
};
owns_data();
// println!("{:?}", data); // ERROR: data was moved

// Using closures with iterators
let numbers = vec![1, 2, 3, 4, 5];
let evens: Vec<&i32> = numbers.iter().filter(|&&n| n % 2 == 0).collect();
```

### Closure Types | 闭包类型
```rust
// Fn - borrows captured values immutably
fn apply_fn(f: impl Fn(i32) -> i32, x: i32) -> i32 {
    f(x)
}

// FnMut - borrows captured values mutably
fn apply_fn_mut(mut f: impl FnMut(i32), values: &[i32]) {
    for &v in values {
        f(v);
    }
}

// FnOnce - takes ownership of captured values
fn apply_fn_once(f: impl FnOnce() -> Vec<i32>) -> Vec<i32> {
    f() // can only call once
}

fn main() {
    // Fn example
    let multiplier = 3;
    let result = apply_fn(|x| x * multiplier, 5);
    
    // FnMut example
    let mut sum = 0;
    apply_fn_mut(|x| sum += x, &[1, 2, 3, 4, 5]);
    println!("Sum: {}", sum); // 15
    
    // FnOnce example
    let data = vec![1, 2, 3];
    let result = apply_fn_once(move || data); // moves data
}
```

```text
这三个 trait 的核心区别，不是“语法不同”，而是闭包对外部捕获值的使用方式不同：
`Fn` 只读，`FnMut` 可修改，`FnOnce` 会消费所有权。
```

***

## LINQ vs Rust Iterators | LINQ vs Rust 迭代器

### C# LINQ (Language Integrated Query) | C# LINQ（语言集成查询）
```csharp
// C# LINQ - Declarative data processing
var numbers = new[] { 1, 2, 3, 4, 5, 6, 7, 8, 9, 10 };

var result = numbers
    .Where(n => n % 2 == 0)           // Filter even numbers
    .Select(n => n * n)               // Square them
    .Where(n => n > 10)               // Filter > 10
    .OrderByDescending(n => n)        // Sort descending
    .Take(3)                          // Take first 3
    .ToList();                        // Materialize

// LINQ with complex objects
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

// Async LINQ (with additional libraries)
var results = await users
    .ToAsyncEnumerable()
    .WhereAwait(async u => await IsActiveAsync(u.Id))
    .SelectAwait(async u => await EnrichUserAsync(u))
    .ToListAsync();
```

### Rust Iterators | Rust 迭代器
```rust
// Rust iterators - Lazy, zero-cost abstractions
let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

let result: Vec<i32> = numbers
    .iter()
    .filter(|&&n| n % 2 == 0)        // Filter even numbers
    .map(|&n| n * n)                 // Square them
    .filter(|&n| n > 10)             // Filter > 10
    .collect::<Vec<_>>()             // Collect to Vec
    .into_iter()
    .rev()                           // Reverse (descending sort)
    .take(3)                         // Take first 3
    .collect();                      // Materialize

// Complex iterator chains
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
            entry.0 += 1;  // count
            entry.1 += user.age as f64;  // sum of ages
            acc
        })
        .into_iter()
        .map(|(dept, (count, sum))| (dept, (count, sum / count as f64)))  // average
        .collect()
}

// Parallel processing with rayon
use rayon::prelude::*;

fn parallel_processing(numbers: Vec<i32>) -> Vec<i32> {
    numbers
        .par_iter()                  // Parallel iterator
        .filter(|&&n| n % 2 == 0)
        .map(|&n| expensive_computation(n))
        .collect()
}

fn expensive_computation(n: i32) -> i32 {
    // Simulate heavy computation
    (0..1000).fold(n, |acc, _| acc + 1)
}
```

```mermaid
graph TD
    subgraph "C# LINQ Characteristics"
        CS_LINQ["LINQ Expression"]
        CS_EAGER["Often eager evaluation<br/>(ToList(), ToArray())"]
        CS_REFLECTION["[ERROR] Some runtime reflection<br/>Expression trees"]
        CS_ALLOCATIONS["[ERROR] Intermediate collections<br/>Garbage collection pressure"]
        CS_ASYNC["[OK] Async support<br/>(with additional libraries)"]
        CS_SQL["[OK] LINQ to SQL/EF integration"]
        
        CS_LINQ --> CS_EAGER
        CS_LINQ --> CS_REFLECTION
        CS_LINQ --> CS_ALLOCATIONS
        CS_LINQ --> CS_ASYNC
        CS_LINQ --> CS_SQL
    end
    
    subgraph "Rust Iterator Characteristics"
        RUST_ITER["Iterator Chain"]
        RUST_LAZY["[OK] Lazy evaluation<br/>No work until .collect()"]
        RUST_ZERO["[OK] Zero-cost abstractions<br/>Compiles to optimal loops"]
        RUST_NO_ALLOC["[OK] No intermediate allocations<br/>Stack-based processing"]
        RUST_PARALLEL["[OK] Easy parallelization<br/>(rayon crate)"]
        RUST_FUNCTIONAL["[OK] Functional programming<br/>Immutable by default"]
        
        RUST_ITER --> RUST_LAZY
        RUST_ITER --> RUST_ZERO
        RUST_ITER --> RUST_NO_ALLOC
        RUST_ITER --> RUST_PARALLEL
        RUST_ITER --> RUST_FUNCTIONAL
    end
    
    subgraph "Performance Comparison"
        CS_PERF["C# LINQ Performance<br/>[ERROR] Allocation overhead<br/>[ERROR] Virtual dispatch<br/>[OK] Good enough for most cases"]
        RUST_PERF["Rust Iterator Performance<br/>[OK] Hand-optimized speed<br/>[OK] No allocations<br/>[OK] Compile-time optimization"]
    end
    
    style CS_REFLECTION fill:#ffcdd2,color:#000
    style CS_ALLOCATIONS fill:#fff3e0,color:#000
    style RUST_ZERO fill:#c8e6c9,color:#000
    style RUST_LAZY fill:#c8e6c9,color:#000
    style RUST_NO_ALLOC fill:#c8e6c9,color:#000
    style CS_PERF fill:#fff3e0,color:#000
    style RUST_PERF fill:#c8e6c9,color:#000
```

***

<details>
<summary><strong>Exercise: LINQ to Iterators Translation | 练习：把 LINQ 翻译为迭代器</strong> (click to expand / 点击展开)</summary>

**Challenge**: Translate this C# LINQ pipeline to idiomatic Rust iterators.

**挑战：** 把下面这段 C# LINQ 管道翻译成惯用的 Rust 迭代器写法。

```csharp
// C# - translate to Rust
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
<summary>Solution | 参考答案</summary>

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

**Key takeaways**:
- Rust has no built-in `group_by` on iterators - `HashMap` + `fold`/`for` is the idiomatic pattern
- `itertools` crate adds `.group_by()` for more LINQ-like syntax
- Iterator chains are zero-cost - the compiler optimizes them to simple loops

**关键要点：**
- Rust 标准迭代器没有内置 `group_by`，惯用模式通常是 `HashMap` 配合 `fold` 或 `for`
- `itertools` crate 会补上更接近 LINQ 的 `.group_by()` 风格
- 迭代器链本身是零成本抽象，编译器通常会把它优化成简单循环

</details>
</details>

## itertools: The Missing LINQ Operations | `itertools`：缺失的 LINQ 能力补充包

Standard Rust iterators cover `map`, `filter`, `fold`, `take`, and `collect`. But C# developers using `GroupBy`, `Zip`, `Chunk`, `SelectMany`, and `Distinct` will immediately notice gaps. The **`itertools`** crate fills them.

标准 Rust 迭代器已经覆盖了 `map`、`filter`、`fold`、`take` 和 `collect` 等核心操作。但对习惯 `GroupBy`、`Zip`、`Chunk`、`SelectMany`、`Distinct` 的 C# 开发者来说，还是会立刻感到某些能力缺失。**`itertools`** crate 正是用来补齐这些空缺的。

```toml
# Cargo.toml
[dependencies]
itertools = "0.12"
```

### Side-by-Side: LINQ vs itertools | 对照：LINQ vs `itertools`

```csharp
// C# - GroupBy
var byDept = employees.GroupBy(e => e.Department)
    .Select(g => new { Dept = g.Key, Count = g.Count() });

// C# - Chunk (batching)
var batches = items.Chunk(100);  // IEnumerable<T[]>

// C# - Distinct / DistinctBy
var unique = users.DistinctBy(u => u.Email);

// C# - SelectMany (flatten)
var allTags = posts.SelectMany(p => p.Tags);

// C# - Zip
var pairs = names.Zip(scores, (n, s) => new { Name = n, Score = s });

// C# - Sliding window
var windows = data.Zip(data.Skip(1), data.Skip(2))
    .Select(triple => (triple.First + triple.Second + triple.Third) / 3.0);
```

```rust
use itertools::Itertools;

// Rust - group_by (requires sorted input)
let by_dept = employees.iter()
    .sorted_by_key(|e| &e.department)
    .group_by(|e| &e.department);
for (dept, group) in &by_dept {
    println!("{}: {} employees", dept, group.count());
}

// Rust - chunks (batching)
let batches = items.iter().chunks(100);
for batch in &batches {
    process_batch(batch.collect::<Vec<_>>());
}

// Rust - unique / unique_by
let unique: Vec<_> = users.iter().unique_by(|u| &u.email).collect();

// Rust - flat_map (SelectMany equivalent - built-in!)
let all_tags: Vec<&str> = posts.iter().flat_map(|p| &p.tags).collect();

// Rust - zip (built-in!)
let pairs: Vec<_> = names.iter().zip(scores.iter()).collect();

// Rust - tuple_windows (sliding window)
let moving_avg: Vec<f64> = data.iter()
    .tuple_windows::<(_, _, _)>()
    .map(|(a, b, c)| (*a + *b + *c) as f64 / 3.0)
    .collect();
```

### itertools Quick Reference | `itertools` 快速对照表

| LINQ Method | itertools Equivalent | Notes |
|------------|---------------------|-------|
| `GroupBy(key)` | `.sorted_by_key().group_by()` | Requires sorted input (unlike LINQ) |
| `GroupBy(key)` | `.sorted_by_key().group_by()` | 需要先排序（不同于 LINQ） |
| `Chunk(n)` | `.chunks(n)` | Returns iterator of iterators |
| `Chunk(n)` | `.chunks(n)` | 返回“迭代器的迭代器” |
| `Distinct()` | `.unique()` | Requires `Eq + Hash` |
| `Distinct()` | `.unique()` | 需要 `Eq + Hash` |
| `DistinctBy(key)` | `.unique_by(key)` | |
| `DistinctBy(key)` | `.unique_by(key)` | |
| `SelectMany()` | `.flat_map()` | Built into std - no crate needed |
| `SelectMany()` | `.flat_map()` | 标准库内置，不需要额外 crate |
| `Zip()` | `.zip()` | Built into std |
| `Zip()` | `.zip()` | 标准库内置 |
| `Aggregate()` | `.fold()` | Built into std |
| `Aggregate()` | `.fold()` | 标准库内置 |
| `Any()` / `All()` | `.any()` / `.all()` | Built into std |
| `Any()` / `All()` | `.any()` / `.all()` | 标准库内置 |
| `First()` / `Last()` | `.next()` / `.last()` | Built into std |
| `First()` / `Last()` | `.next()` / `.last()` | 标准库内置 |
| `Skip(n)` / `Take(n)` | `.skip(n)` / `.take(n)` | Built into std |
| `Skip(n)` / `Take(n)` | `.skip(n)` / `.take(n)` | 标准库内置 |
| `OrderBy()` | `.sorted()` / `.sorted_by()` | `itertools` (std has none) |
| `OrderBy()` | `.sorted()` / `.sorted_by()` | 来自 `itertools`（标准库没有） |
| `ThenBy()` | `.sorted_by(\|a,b\| a.x.cmp(&b.x).then(a.y.cmp(&b.y)))` | Chained `Ordering::then` |
| `ThenBy()` | `.sorted_by(\|a,b\| a.x.cmp(&b.x).then(a.y.cmp(&b.y)))` | 通过 `Ordering::then` 链式比较 |
| `Intersect()` | `HashSet` intersection | No direct iterator method |
| `Intersect()` | `HashSet` intersection | 没有直接的迭代器方法 |
| `Concat()` | `.chain()` | Built into std |
| `Concat()` | `.chain()` | 标准库内置 |
| Sliding window | `.tuple_windows()` | Fixed-size tuples |
| 滑动窗口 | `.tuple_windows()` | 适合固定大小窗口 |
| Cartesian product | `.cartesian_product()` | `itertools` |
| 笛卡尔积 | `.cartesian_product()` | `itertools` |
| Interleave | `.interleave()` | `itertools` |
| 交错合并 | `.interleave()` | `itertools` |
| Permutations | `.permutations(k)` | `itertools` |
| 排列 | `.permutations(k)` | `itertools` |

### Real-World Example: Log Analysis Pipeline | 实战示例：日志分析管道

```rust
use itertools::Itertools;
use std::collections::HashMap;

#[derive(Debug)]
struct LogEntry { level: String, module: String, message: String }

fn analyze_logs(entries: &[LogEntry]) {
    // Top 5 noisiest modules (like LINQ GroupBy + OrderByDescending + Take)
    let noisy: Vec<_> = entries.iter()
        .into_group_map_by(|e| &e.module) // itertools: direct group into HashMap
        .into_iter()
        .sorted_by(|a, b| b.1.len().cmp(&a.1.len()))
        .take(5)
        .collect();

    for (module, entries) in &noisy {
        println!("{}: {} entries", module, entries.len());
    }

    // Error rate per 100-entry window (sliding window)
    let error_rates: Vec<f64> = entries.iter()
        .map(|e| if e.level == "ERROR" { 1.0 } else { 0.0 })
        .collect::<Vec<_>>()
        .windows(100)  // std slice method
        .map(|w| w.iter().sum::<f64>() / 100.0)
        .collect();

    // Deduplicate consecutive identical messages
    let deduped: Vec<_> = entries.iter().dedup_by(|a, b| a.message == b.message).collect();
    println!("Deduped {} -> {} entries", entries.len(), deduped.len());
}
```

***
