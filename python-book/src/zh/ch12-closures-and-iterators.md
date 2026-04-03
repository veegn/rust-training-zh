[English Original](../en/ch12-closures-and-iterators.md)

## Rust 闭包 vs Python Lambda

> **你将学到：** 多行闭包（不仅仅是单表达式的 lambda）、`Fn`/`FnMut`/`FnOnce` 捕获语义、迭代器链与列表推导式的对比、`map`/`filter`/`fold` 的映射关系，以及 `macro_rules!` 宏的基础知识。
>
> **难度：** 🟡 中级

### Python 中的闭包与 Lambda
```python
# Python — lambda 是单表达式的匿名函数
double = lambda x: x * 2
result = double(5)  # 10

# 完整的闭包从封闭作用域中捕获变量：
def make_adder(n):
    def adder(x):
        return x + n    # 从外部作用域捕获 `n`
    return adder

add_5 = make_adder(5)
print(add_5(10))  # 15

# 高阶函数：
numbers = [1, 2, 3, 4, 5]
doubled = list(map(lambda x: x * 2, numbers))
evens = list(filter(lambda x: x % 2 == 0, numbers))
```

### Rust 中的闭包
```rust
// Rust — 闭包使用 |参数| 表达式 的语法
let double = |x: i32| x * 2;
let result = double(5);  // 10

// 闭包从封闭作用域中捕捉变量：
fn make_adder(n: i32) -> impl Fn(i32) -> i32 {
    move |x| x + n    // `move` 将 `n` 的所有权转移进闭包
}

let add_5 = make_adder(5);
println!("{}", add_5(10));  // 15

// 配合迭代器的高阶函数：
let numbers = vec![1, 2, 3, 4, 5];
let doubled: Vec<i32> = numbers.iter().map(|x| x * 2).collect();
let evens: Vec<i32> = numbers.iter().filter(|&&x| x % 2 == 0).copied().collect();
```

### 闭包语法对比
```text
Python:                              Rust:
─────────                            ─────
lambda x: x * 2                      |x| x * 2
lambda x, y: x + y                   |x, y| x + y
lambda: 42                           || 42

# 多行闭包
def f(x):                            |x| {
    y = x * 2                            let y = x * 2;
    return y + 1                         y + 1
                                       }
```

### 闭包捕获机制 — Rust 的不同之处
```python
# Python — 闭包通过引用捕获 (延迟绑定！)
funcs = [lambda: i for i in range(3)]
print([f() for f in funcs])  # [2, 2, 2] — 惊讶吗？它们全部捕捉到了同一个 `i`

# 修复方法（使用默认参数的小技巧）：
funcs = [lambda i=i: i for i in range(3)]
print([f() for f in funcs])  # [0, 1, 2]
```

```rust
// Rust — 闭包捕获是正确的 (不存在延迟绑定的陷阱)
let funcs: Vec<Box<dyn Fn() -> i32>> = (0..3)
    .map(|i| Box::new(move || i) as Box<dyn Fn() -> i32>)
    .collect();

let results: Vec<i32> = funcs.iter().map(|f| f()).collect();
println!("{:?}", results);  // [0, 1, 2] — 正确！

// `move` 关键字为每个闭包捕获了 `i` 的一份副本 — 不会有任何延迟绑定的意外。
```

### 三种闭包 Trait
```rust
// Rust 闭包会实现以下一个或多个 Trait：

// Fn — 可多次调用，不修改捕获到的变量 (最常用)
fn apply(f: impl Fn(i32) -> i32, x: i32) -> i32 { f(x) }

// FnMut — 可多次调用，可能会修改捕获到的变量
fn apply_mut(mut f: impl FnMut(i32) -> i32, x: i32) -> i32 { f(x) }

// FnOnce — 只能调用“一次” (会消耗掉捕获的内容)
fn apply_once(f: impl FnOnce() -> String) -> String { f() }

// Python 中没有与之对应的概念 — 其闭包行为始终类似于 Fn。
// 在 Rust 中，编译器会自动确定使用哪种 Trait。
```

---

## 迭代器 vs 生成器

### Python 生成器
```python
# Python — 使用 yield 的生成器
def fibonacci():
    a, b = 0, 1
    while True:
        yield a
        a, b = b, a + b

# 惰性求值 — 按需计算数值
fib = fibonacci()
first_10 = [next(fib) for _ in range(10)]

# 生成器表达式 — 类似惰性的列表推导式
squares = (x ** 2 for x in range(1000000))  # 不分配内存
first_5 = [next(squares) for _ in range(5)]
```

### Rust 迭代器
```rust
// Rust — Iterator Trait (概念相似，语法不同)
struct Fibonacci {
    a: u64,
    b: u64,
}

impl Fibonacci {
    fn new() -> Self {
        Fibonacci { a: 0, b: 1 }
    }
}

impl Iterator for Fibonacci {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.a;
        self.a = self.b;
        self.b = current + self.b;
        Some(current)
    }
}

// 惰性求值 — 按需计算数值 (就像 Python 的生成器一样)
let first_10: Vec<u64> = Fibonacci::new().take(10).collect();

// 迭代器链 — 就像生成器表达式
let squares: Vec<u64> = (0..1_000_000u64).map(|x| x * x).take(5).collect();
```

---

## 推导式 vs 迭代器链

本节将 Python 的各种推导式语法映射到 Rust 的迭代器链上。

### 列表推导式 (List Comprehension) → map/filter/collect
```python
# Python 推导式：
squares = [x ** 2 for x in range(10)]
evens = [x for x in range(20) if x % 2 == 0]
names = [user.name for user in users if user.active]
pairs = [(x, y) for x in range(3) for y in range(3)]
flat = [item for sublist in nested for item in sublist]
```

```mermaid
flowchart LR
    A["源数据\n[1,2,3,4,5]"] -->|.iter\(\)| B["迭代器"]
    B -->|.filter\(\|x\| x%2==0\)| C["[2, 4]"]
    C -->|.map\(\|x\| x*x\)| D["[4, 16]"]
    D -->|.collect\(\)| E["Vec&lt;i32&gt;\n[4, 16]"]
    style A fill:#ffeeba
    style E fill:#d4edda
```

> **关键洞见**：Rust 的迭代器是惰性的 —— 只有在调用 `.collect()` 后才会触发计算。Python 的生成器也有同样的机制，但列表推导式则是会立即进行求值的。

```rust
// Rust 迭代器链：
let squares: Vec<i32> = (0..10).map(|x| x * x).collect();
let evens: Vec<i32> = (0..20).filter(|x| x % 2 == 0).collect();
let names: Vec<&str> = users.iter()
    .filter(|u| u.active)
    .map(|u| u.name.as_str())
    .collect();
let pairs: Vec<(i32, i32)> = (0..3)
    .flat_map(|x| (0..3).map(move |y| (x, y)))
    .collect();
let flat: Vec<i32> = nested.iter()
    .flat_map(|sublist| sublist.iter().copied())
    .collect();
```

### 字典推导式 (Dict Comprehension) → collect 为 HashMap
```python
# Python
word_lengths = {word: len(word) for word in words}
inverted = {v: k for k, v in mapping.items()}
```

```rust
// Rust
let word_lengths: HashMap<&str, usize> = words.iter()
    .map(|w| (*w, w.len()))
    .collect();
let inverted: HashMap<&V, &K> = mapping.iter()
    .map(|(k, v)| (v, k))
    .collect();
```

### 集合推导式 (Set Comprehension) → collect 为 HashSet
```python
# Python
unique_lengths = {len(word) for word in words}
```

```rust
// Rust
let unique_lengths: HashSet<usize> = words.iter()
    .map(|w| w.len())
    .collect();
```

### 常用迭代器方法对比

| Python | Rust | 说明 |
|--------|------|-------|
| `map(f, iter)` | `.map(f)` | 转换每一个元素 |
| `filter(f, iter)` | `.filter(f)` | 保留匹配的元素 |
| `sum(iter)` | `.sum()` | 求和 |
| `min(iter)` / `max(iter)` | `.min()` / `.max()` | 返回 `Option` |
| `any(f(x) for x in iter)` | `.any(f)` | 是否有任何项匹配 |
| `all(f(x) for x in iter)` | `.all(f)` | 是否全部匹配 |
| `enumerate(iter)` | `.enumerate()` | 产生索引 + 值的元组 |
| `zip(a, b)` | `a.zip(b)` | 将两个迭代器项成对合并 |
| `len(list)` | `.count()` (会消耗完！) 或 `.len()` | 计算项数 |
| `list(reversed(x))` | `.rev()` | 反向迭代 |
| `itertools.chain(a, b)` | `a.chain(b)` | 拼接两个迭代器 |
| `next(iter)` | `.next()` | 获取下一项 |
| `next(iter, default)` | `.next().unwrap_or(default)` | 且带默认值 |
| `list(iter)` | `.collect::<Vec<_>>()` | 实体化为集合 |
| `sorted(iter)` | 先 Collect, 随后再执行 `.sort()` | 无惰性的排序迭代器 |
| `functools.reduce(f, iter)` | `.fold(初始值, f)` 或 `.reduce(f)` | 累加/折叠 |

### 核心差异
```text
Python 迭代器:                        Rust 迭代器:
─────────────────                     ──────────────
- 默认惰性 (针对生成器)                - 默认惰性 (所有环节)
- yield 用于创建生成器                 - 实现 Iterator { fn next() }
- StopIteration 表示迭代完毕           - 返回 None 为终止
- 只能被消耗一次                       - 只能被消耗一次
- 缺乏类型安全性                       - 完全的类型安全性
- 稍慢 (解释器执行)                    - 零成本 (编译期消除)
```

---

## 为什么 Rust 中存在宏

Python 并没有宏系统 —— 它通过装饰器 (Decorators)、元类 (Metaclasses) 以及运行时内省 (Introspection) 来进行元编程。而 Rust 使用宏在编译期生成代码。

### Python 元编程 vs Rust 宏
```python
# Python — 使用装饰器和元类进行元编程
from dataclasses import dataclass
from functools import wraps

@dataclass              # 在导入时生成 __init__, __repr__, __eq__
class Point:
    x: float
    y: float

# 自定义装饰器
def log_calls(func):
    @wraps(func)
    def wrapper(*args, **kwargs):
        print(f"正在调用 {func.__name__}")
        return func(*args, **kwargs)
    return wrapper

@log_calls
def process(data):
    return data.upper()
```

```rust
// Rust — 使用 derive 宏和声明式宏进行代码生成
#[derive(Debug, Clone, PartialEq)]  // 在“编译期”生成 Debug, Clone, PartialEq 的实现
struct Point {
    x: f64,
    y: f64,
}

// 声明式宏 (类似模板)
macro_rules! log_call {
    ($func_name:expr, $body:expr) => {
        println!("正在调用 {}", $func_name);
        $body
    };
}

fn process(data: &str) -> String {
    log_call!("process", data.to_uppercase())
}
```

### 常见的内置宏
```rust
// 这些宏在 Rust 中随处可见：

println!("你好, {}!", name);            // 格式化打印
format!("数值为: {}", x);               // 创建格式化 String
vec![1, 2, 3];                          // 创建 Vec
assert_eq!(2 + 2, 4);                  // 测试相等断言
assert!(value > 0, "必须为正数");        // 布尔值断言
dbg!(expression);                       // 调试打印：打印表达式及其值
todo!();                                // 占位符 — 可编译但运行到此处会 panic
unimplemented!();                       // 标记尚未实现的代码
panic!("出错了");                        // 带着消息崩溃 (类似 raise RuntimeError)

// 为什么这些是宏而不是函数？
// - println! 接收可变数量的参数 (Rust 函数做不到)
// - vec! 为任何类型和长度生成初始化代码
// - assert_eq! 知道你所比较对象的源代码信息
// - dbg! 知道文件名和行号
```

---

## 使用 macro_rules! 编写简单的宏
```rust
// 对标 Python 的 dict()
// Python: d = dict(a=1, b=2)
// Rust:   let d = hashmap!{ "a" => 1, "b" => 2 };

macro_rules! hashmap {
    ($($key:expr => $value:expr),* $(,)?) => {
        {
            let mut map = std::collections::HashMap::new();
            $(map.insert($key, $value);)*
            map
        }
    };
}

let scores = hashmap! {
    "小明" => 100,
    "阿强" => 85,
    "阿珍" => 90,
};
```

## Derive 宏 — 自动实现 Trait
```rust
// #[derive(...)] 是 Rust 中对 Python @dataclass 装饰器的对应实现

// Python:
// @dataclass(frozen=True, order=True)
// class Student:
//     name: str
//     grade: int

// Rust:
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Student {
    name: String,
    grade: i32,
}

// 常见的 derive 宏：
// Debug         → 提供 {:?} 的格式化支持 (类似 __repr__)
// Clone         → 提供 .clone() 的深拷贝支持
// Copy          → 提供隐式拷贝支持 (仅适用于简单类型)
// PartialEq, Eq → 提供 == 比较支持 (类似 __eq__)
// PartialOrd, Ord → 提供 <、> 以及排序支持 (类似 __lt__ 等)
// Hash          → 使其可作为 HashMap 的键使用 (类似 __hash__)
// Default       → 使其支持 MyType::default() (类似无参数的 __init__)

// 由外部 Crate 提供的常用 derive 宏：
// Serialize, Deserialize (serde 库) → JSON/YAML/TOML 序列化支持
//                                  (类似 Python 的 json.dumps/loads，但是类型安全的)
```

### Python 装饰器与 Rust Derive 的映射

| Python 装饰器 | Rust Derive | 用途 |
|-----------------|-------------|---------|
| `@dataclass` | `#[derive(Debug, Clone, PartialEq)]` | 数据类 |
| `@dataclass(frozen=True)` | 默认即为不可变 | 不可变性 |
| `@dataclass(order=True)` | `#[derive(Ord, PartialOrd)]` | 比较/排序 |
| `@total_ordering` | `#[derive(PartialOrd, Ord)]` | 完整排序支持 |
| JSON `json.dumps(obj.__dict__)` | `#[derive(Serialize)]` | 序列化 |
| JSON `MyClass(**json.loads(s))` | `#[derive(Deserialize)]` | 反序列化 |

---

## 练习

<details>
<summary><strong>🏋️ 练习：Derive 与自定义 Debug 实现</strong>（点击展开）</summary>

**挑战**：创建一个包含 `name: String`、`email: String` 和 `password_hash: String` 字段的 `User` 结构体。为其通过派生 (derive) 方式实现 `Clone` 和 `PartialEq`，但需要手动实现 `Debug`，以便在打印时能输出姓名和邮箱，但要隐藏密码（显示为 `"***"`）。

<details>
<summary>🔑 答案</summary>

```rust
use std::fmt;

#[derive(Clone, PartialEq)]
struct User {
    name: String,
    email: String,
    password_hash: String,
}

impl fmt::Debug for User {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("User")
            .field("name", &self.name)
            .field("email", &self.email)
            .field("password_hash", &"***")
            .finish()
    }
}

fn main() {
    let user = User {
        name: "阿强".into(),
        email: "qiang@example.com".into(),
        password_hash: "a1b2c3d4e5f6".into(),
    };
    println!("{user:?}");
    // 输出: User { name: "阿强", email: "qiang@example.com", password_hash: "***" }
}
```

**核心要点**: 与 Python 的 `__repr__` 不同，Rust 允许你免费通过派生获得 `Debug` 实现，但你仍保留了针对敏感字段进行重写的灵活性。相比 Python 这种容易在 `print(user)` 时不小心泄露私密信息的机制，Rust 的做法更具安全性。

</details>
</details>

---
