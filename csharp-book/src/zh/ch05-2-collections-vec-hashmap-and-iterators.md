# 向量、映射与迭代器

> **你将学到什么：** `Vec<T>` 与 `List<T>`、`HashMap` 与 `Dictionary` 的对应关系、安全访问模式（为什么 Rust 返回 `Option` 而不是抛出异常），以及集合操作与所有权之间的微妙关系。
>
> **难度：** 🟢 初级

## `Vec<T>` vs `List<T>`
`Vec<T>` 是 Rust 对应 C# `List<T>` 的类型，但它深受所有权语法的制约。

### C# vs Rust 概念传递
在 C# 中，将 `List<T>` 传递给方法本质上是传递引用。而在 Rust 中，除非你传递引用 (`&Vec<T>`)，否则传递 `Vec<T>` 会发生**所有权转移 (Move)**。

```rust
let mut numbers = vec![1, 2, 3];
process_vec(numbers); 
// 此处 numbers 已不再可用，所有权已经移至函数内部！

let mut numbers = vec![1, 2, 3];
process_vec_borrowed(&mut numbers);
// numbers 仍然可以使用！
```

### 安全访问
Rust 极其厌恶非预期的运行时异常。虽然 `vec[index]` 在索引越界时会发生 panic（类似于抛异常），但标准库提供的 `vec.get(index)` 则会安全地返回一个 `Option`。

```rust
let first = vec.get(0); // 返回 Some(&value) 或 None
```

---

## HashMap vs Dictionary
`HashMap` 是 Rust 版本的 C# `Dictionary<K, V>`。

### 常见操作对照表
| **操作** | **C# Dictionary** | **Rust HashMap** |
| :--- | :--- | :--- |
| **添加/更新** | `dict["key"] = val` | `map.insert(key, val)` |
| **检查键** | `dict.ContainsKey(key)` | `map.contains_key(key)` |
| **移除** | `dict.Remove(key)` | `map.remove(key)` |
| **安全获取** | `dict.TryGetValue(key, out v)` | `map.get(key)` -> `Option<&V>` |

### Entry API
Rust 的 `HashMap` 拥有一个独特且强大的 "Entry API"，专门用于高效地处理“如果不存在则插入”等逻辑。
```rust
// 仅当 "key" 不存在时插入 42
map.entry("key".to_string()).or_insert(42);
```

---

## 迭代器 (Rust 中的 LINQ)
Rust 迭代器提供类似于 LINQ 的流式处理体验，但它们直接内置在核心语言中，且性能极度强悍（零成本抽象）。

### LINQ 到 Rust 的映射
| **LINQ** | **Rust 迭代器** | **备注** |
| :--- | :--- | :--- |
| `.Select(x => ...)` | `.map(|x| ...)` | 惰性计算 (Lazy) |
| `.Where(x => ...)` | `.filter(|x| ...)` | 惰性计算 (Lazy) |
| `.ToList()` | `.collect::<Vec<_>>()` | 立即执行 (Eager) |
| `.Take(n)` | `.take(n)` | 惰性计算 (Lazy) |
| `.FirstOrDefault()` | `.next()` | |

### 迭代器类型
1.  **`.iter()`**: 为不可变引用生成迭代器 (`&T`)。
2.  **`.iter_mut()`**: 为可变引用生成迭代器 (`&mut T`)。
3.  **`.into_iter()`**: 消耗集合本身，并生成获取所有权的迭代器 (`T`)。

---

## 练习：LINQ 改写为迭代器
**挑战：** 将一个标准的 C# LINQ 查询（过滤、排序、选择、取前 N 名）翻译成 Rust 的惯用模式。

```rust
fn top_students(students: &mut [Student]) -> Vec<String> {
    students.sort_by(|a, b| b.grade.cmp(&a.grade)); // 原地排序，立即执行
    students.iter()
        .filter(|s| s.grade >= 90)
        .take(3)
        .map(|s| format!("{}: {}", s.name, s.grade))
        .collect()
}
```
**关键点：** Rust 的迭代器链（如 map/filter）是惰性的，但在排序这一环节，Rust 通常通过原地操作立即完成。所以你需要先排序，再衔接后续处理流程。
