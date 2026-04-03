[English Original](../en/ch05-2-collections-vec-hashmap-and-iterators.md)

## `Vec<T>` vs `List<T>`

> **你将学到：** `Vec<T>` 与 `List<T>` 的对比，`HashMap` 与 `Dictionary` 的对比，安全访问模式（为什么 Rust 返回 `Option` 而不是抛出异常），以及集合在所有权方面的含义。
>
> **难度：** 🟢 初级

`Vec<T>` 是 Rust 中对应 C# `List<T>` 的类型，但它带有所有权语义。

### C# `List<T>`
```csharp
// C# List<T> - 引用类型，堆上分配
var numbers = new List<int>();
numbers.Add(1);
numbers.Add(2);
numbers.Add(3);

// 传给方法 - 复制引用
ProcessList(numbers);
Console.WriteLine(numbers.Count);  // 依然可以访问

void ProcessList(List<int> list)
{
    list.Add(4);  // 修改原始列表
    Console.WriteLine($"方法内的计数: {list.Count}");
}
```

### Rust `Vec<T>`
```rust
// Rust Vec<T> - 拥有所有权的类型，堆上分配
let mut numbers = Vec::new();
numbers.push(1);
numbers.push(2);
numbers.push(3);

// 获取所有权的方法 (Takes ownership)
process_vec(numbers);
// println!("{:?}", numbers);  // ❌ Error: numbers 已经被移动 (moved) 了

// 借用的方法 (Borrows)
let mut numbers = vec![1, 2, 3];  // 使用 vec! 宏快速创建
process_vec_borrowed(&mut numbers);
println!("{:?}", numbers);  // ✅ 依然可以访问

fn process_vec(mut vec: Vec<i32>) {  // 获取所有权
    vec.push(4);
    println!("方法内的计数: {}", vec.len());
    // vec 在这里被释放 (dropped)
}

fn process_vec_borrowed(vec: &mut Vec<i32>) {  // 可变借用
    vec.push(4);
    println!("方法内的计数: {}", vec.len());
}
```

### 创建与初始化 Vector
```csharp
// C# List 初始化
var numbers = new List<int> { 1, 2, 3, 4, 5 };
var empty = new List<int>();
var sized = new List<int>(10);  // 初始容量 (capacity)
```

```rust
// Rust Vec 初始化
let numbers = vec![1, 2, 3, 4, 5];  // vec! 宏
let empty: Vec<i32> = Vec::new();   // 空 Vec 通常需要类型注解
let sized = Vec::with_capacity(10); // 预分配容量

// 从迭代器创建
let from_range: Vec<i32> = (1..=5).collect();
```

### 常用操作对比
```csharp
// C# List 操作
var list = new List<int> { 1, 2, 3 };

list.Add(4);                    // 添加元素
list.Insert(0, 0);              // 在索引处插入
list.Remove(2);                 // 删除第一个匹配项
list.RemoveAt(1);               // 删除索引处元素
list.Clear();                   // 清空

int first = list[0];            // 索引访问
int count = list.Count;         // 获取数量
bool contains = list.Contains(3); // 是否包含
```

```rust
// Rust Vec 操作
let mut vec = vec![1, 2, 3];

vec.push(4);                    // 添加元素
vec.insert(0, 0);               // 在索引处插入
vec.retain(|&x| x != 2);        // 删除特定元素 (函数式风格)
vec.remove(1);                  // 删除索引处元素
vec.clear();                    // 清空

let first = vec[0];             // 索引访问 (若越界会 panic)
let safe_first = vec.get(0);    // 安全访问，返回 Option<&T>
let count = vec.len();          // 获取数量
let contains = vec.contains(&3); // 是否包含
```

### 安全访问模式
```csharp
// C# - 基于异常的边界检查
public int SafeAccess(List<int> list, int index)
{
    try
    {
        return list[index];
    }
    catch (ArgumentOutOfRangeException)
    {
        return -1;  // 默认值
    }
}
```

```rust
// Rust - 基于 Option 的安全访问
fn safe_access(vec: &[i32], index: usize) -> Option<i32> {
    vec.get(index).copied()  // 返回 Option<i32>
}

fn main() {
    let vec = vec![1, 2, 3];
    
    // 匹配安全访问结果
    match vec.get(10) {
        Some(value) => println!("值: {}", value),
        None => println!("索引越界"),
    }
    
    // 或者使用 unwrap_or 提供默认值
    let value = vec.get(10).copied().unwrap_or(-1);
    println!("值: {}", value);
}
```

***

## HashMap vs Dictionary

`HashMap` 是 Rust 中对应 C# `Dictionary<K,V>` 的类型。

### C# Dictionary
```csharp
// C# Dictionary<TKey, TValue>
var scores = new Dictionary<string, int>
{
    ["Alice"] = 100,
    ["Bob"] = 85
};

// 安全访问
if (scores.TryGetValue("Eve", out int score))
{
    Console.WriteLine($"Eve 的分数: {score}");
}
```

### Rust HashMap
```rust
use std::collections::HashMap;

// 创建并初始化 HashMap
let mut scores = HashMap::new();
scores.insert("Alice".to_string(), 100);
scores.insert("Bob".to_string(), 85);

// 或者通过迭代器创建
let scores: HashMap<String, i32> = [
    ("Alice".to_string(), 100),
    ("Bob".to_string(), 85),
].into_iter().collect();

// 安全访问
match scores.get("Eve") {
    Some(score) => println!("Eve 的分数: {}", score),
    None => println!("未找到 Eve"),
}
```

### Entry API 用于高效更新
```rust
// Rust 的 Entry API 允许进行高级的检查并更新操作
let mut map = HashMap::new();

// 如果不存在则插入
map.entry("key".to_string()).or_insert(42); 

// 如果存在则修改
map.entry("key".to_string()).and_modify(|v| *v += 1); 
```

***

## 迭代模式 (Iteration Patterns)

### C# vs Rust 迭代对比
```csharp
// C# 迭代
foreach (int num in numbers)
{
    Console.WriteLine(num);
}

// LINQ 方式
var doubled = numbers.Select(x => x * 2).ToList();
```

```rust
// Rust 迭代
// 1. iter() - 借用元素 (&T)
for item in vec.iter() {
    println!("{}", item);  // item 类型为 &i32
}

// 2. into_iter() - 获取所有权 (T)
for item in vec.into_iter() {
    println!("{}", item);  // item 类型为 i32
}
// vec 在此之后不再可用

// 迭代器方法 (类似于 LINQ)
let doubled: Vec<i32> = numbers.iter().map(|x| x * 2).collect();
```

***

## 练习

<details>
<summary><strong>🏋️ 练习：从 LINQ 到迭代器</strong> (点击展开)</summary>

将这段 C# LINQ 查询翻译为地道的 Rust 迭代器代码：

```csharp
var result = students
    .Where(s => s.Grade >= 90)
    .OrderByDescending(s => s.Grade)
    .Select(s => $"{s.Name}: {s.Grade}")
    .Take(3)
    .ToList();
```

使用以下结构体：
```rust
struct Student { name: String, grade: u32 }
```

要求返回前 3 名分数 ≥ 90 的学生，格式为 `"Name: Grade"`。

<details>
<summary>🔑 参考答案</summary>

```rust
#[derive(Debug)]
struct Student { name: String, grade: u32 }

fn top_students(students: &mut [Student]) -> Vec<String> {
    // Rust 迭代器是惰性的，但排序 (sort_by) 是及时的原地操作
    students.sort_by(|a, b| b.grade.cmp(&a.grade)); 
    
    students.iter()
        .filter(|s| s.grade >= 90)
        .take(3)
        .map(|s| format!("{}: {}", s.name, s.grade))
        .collect()
}

fn main() {
    let mut students = vec![
        Student { name: "Alice".into(), grade: 95 },
        Student { name: "Bob".into(), grade: 88 },
        Student { name: "Carol".into(), grade: 92 },
        Student { name: "Dave".into(), grade: 97 },
        Student { name: "Eve".into(), grade: 91 },
    ];
    let result = top_students(&mut students);
    assert_eq!(result, vec!["Dave: 97", "Alice: 95", "Carol: 92"]);
    println!("{result:?}");
}
```

**与 C# 的关键区别**：Rust 的迭代器也是惰性求值的（类似于 LINQ），但没有惰性的 `OrderBy`。通常先进行及时的 `sort_by` 排序，然后再链式调用惰性的过滤和映射操作。

</details>
</details>

***
