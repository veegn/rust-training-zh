[English Original](../en/ch03-built-in-types-and-variables.md)

## 变量与可变性

> **你将学到：** Rust 的变量声明与可变性模型 vs C# 的 `var`/`const`，原始类型映射，至关重要的 `String` 与 `&str` 区别，类型推断，以及 Rust 与 C# 在类型转换（casting/conversions）处理上的不同。
>
> **难度：** 🟢 初级

### C# 变量声明
```csharp
// C# - 变量默认是可变的
int count = 0;           // 可变
count = 5;               // ✅ 正常运行

// readonly 字段 (仅限于类级别，不适用于局部变量)
// readonly int maxSize = 100;  // 初始化后不可变

const int BUFFER_SIZE = 1024; // 编译时常量 (可作为局部变量或字段)
```

### Rust 变量声明
```rust
// Rust - 变量默认是不可变的 (immutable)
let count = 0;           // 默认不可变
// count = 5;            // ❌ 编译错误：无法对不可变变量进行二次赋值

let mut count = 0;       // 显式声明为可变 (mutable)
count = 5;               // ✅ 正常运行

const BUFFER_SIZE: usize = 1024; // 编译时常量
```

### C# 开发者的关键思维转变
```rust
// 可以将 'let' 视为将 C# 的 readonly 字段语义应用到了所有变量上
let name = "John";       // 类似于 readonly 字段：一旦设置，不可更改
let mut age = 30;        // 类似于：int age = 30;

// 变量遮蔽 (Variable shadowing，Rust 特有)
let spaces = "   ";      // 类型为 String (或 &str)
let spaces = spaces.len(); // 现在它是一个数字 (usize)
// 这与“修改 (mutation)”不同 - 我们是在创建一个全新的变量并重用旧名称
```

### 实战示例：计数器
```csharp
// C# 版本
public class Counter
{
    private int value = 0;
    
    public void Increment()
    {
        value++;  // 修改
    }
    
    public int GetValue() => value;
}
```

```rust
// Rust 版本
pub struct Counter {
    value: i32,  // 默认私有
}

impl Counter {
    pub fn new() -> Counter {
        Counter { value: 0 }
    }
    
    pub fn increment(&mut self) {  // 修改数据需要使用 &mut self
        self.value += 1;
    }
    
    pub fn get_value(&self) -> i32 {
        self.value
    }
}
```

***

## 数据类型对比

### 原始类型 (Primitive Types)

| C# 类型 | Rust 类型 | 大小 | 范围/说明 |
|---------|-----------|------|-------|
| `byte` | `u8` | 8 bits | 0 到 255 |
| `sbyte` | `i8` | 8 bits | -128 到 127 |
| `short` | `i16` | 16 bits | -32,768 到 32,767 |
| `ushort` | `u16` | 16 bits | 0 到 65,535 |
| `int` | `i32` | 32 bits | -2³¹ 到 2³¹-1 |
| `uint` | `u32` | 32 bits | 0 到 2³²-1 |
| `long` | `i64` | 64 bits | -2⁶³ 到 2⁶³-1 |
| `ulong` | `u64` | 64 bits | 0 到 2⁶⁴-1 |
| `float` | `f32` | 32 bits | IEEE 754 |
| `double` | `f64` | 64 bits | IEEE 754 |
| `bool` | `bool` | 1 bit | true/false |
| `char` | `char` | 32 bits | Unicode 标量值 (Scalar) |

### 大小相关类型 (极其重要！)
```csharp
// C# - int 始终是 32 位
int arrayIndex = 0;
long fileSize = file.Length;
```

```rust
// Rust - 指定大小类型会匹配指针大小 (32 位或 64 位系统对应不同大小)
let array_index: usize = 0;    // 类似于 C 语言中的 size_t，用于索引
let file_size: u64 = file.len(); // 显式的 64 位
```

### 类型推断
```csharp
// C# - var 关键字
var name = "John";        // 类型为 string
var count = 42;           // 类型为 int
var price = 29.99;        // 类型为 double
```

```rust
// Rust - 自动类型推断
let name = "John";        // 类型为 &str (字符串切片)
let count = 42;           // 类型为 i32 (默认整数类型)
let price = 29.99;        // 类型为 f64 (默认浮点数类型)

// 显式类型注解 (Explicit type annotations)
let count: u32 = 42;
let price: f32 = 29.99;
```

### 数组与集合概览
```csharp
// C# - 引用类型，在堆 (heap) 上分配
int[] numbers = new int[5];        // 固定大小
List<int> list = new List<int>();  // 动态大小
```

```rust
// Rust - 多种选项
let numbers: [i32; 5] = [1, 2, 3, 4, 5];  // 栈 (stack) 数组，固定大小
let mut list: Vec<i32> = Vec::new();       // 堆上的向量 (vector)，动态大小
```

***

## 字符串类型：String vs &str

这是令 C# 开发者最感困惑的概念之一，让我们通过对比来透彻理解。

### C# 字符串处理
```csharp
// C# - 简单的字符串模型
string name = "John";           // 字符串字面量
string greeting = "Hello, " + name;  // 字符串拼接
string upper = name.ToUpper();  // 方法调用
```

### Rust 字符串类型
```rust
// Rust - 有两种主要的字符串类型

// 1. &str (字符串切片) - 类似于 C# 中的 ReadOnlySpan<char>
let name: &str = "John";        // 字符串字面量 (不可变，借用)

// 2. String - 类似于 StringBuilder 或可变字符串
let mut greeting = String::new();       // 创建空字符串
greeting.push_str("Hello, ");          // 追加内容
greeting.push_str(name);               // 追加内容

// 或者直接创建
let greeting = String::from("Hello, John");
let greeting = "Hello, John".to_string();  // 将 &str 转换为 String
```

### 应该使用哪一种？

| 场景 | 使用类型 | C# 对应概念 |
|----------|-----|---------------|
| 字符串字面量 | `&str` | `string` 字面量 |
| 函数参数 (只读) | `&str` | `string` 或 `ReadOnlySpan<char>` |
| 拥有所有权的、可变的字符串 | `String` | `StringBuilder` |
| 返回拥有所有权的字符串 | `String` | `string` |

### 实战案例
```rust
// 该函数可以接受任何字符串类型
fn greet(name: &str) {  // 同时接受 String 和 &str
    println!("Hello, {}!", name);
}

fn main() {
    let literal = "John";                    // &str
    let owned = String::from("Jane");        // String
    
    greet(literal);                          // 正常运行
    greet(&owned);                           // 正常运行 (将 String 借用为 &str)
    greet("Bob");                            // 正常运行
}

// 返回拥有所有权的字符串的函数
fn create_greeting(name: &str) -> String {
    format!("Hello, {}!", name)  // format! 宏返回一个 String
}
```

### C# 开发者的理解思路
```rust
// &str 就像 ReadOnlySpan<char> —— 它是对字符串数据的“视图”
// String 就像一个你拥有所有权且可以修改的 char[]

let borrowed: &str = "我不拥有这段数据";
let owned: String = String::from("我拥有这段数据");

// 在两者之间转换
let owned_copy: String = borrowed.to_string();  // 复制并转为拥有者模式
let borrowed_view: &str = &owned;               // 从 String 借用一个视图
```

***

## 打印与字符串格式化

C# 开发者高度依赖 `Console.WriteLine` 和字符串内插 (`$""`)。Rust 的格式化系统同样强大，但使用的是宏和格式说明符。

### 基础输出
```csharp
// C# 输出
Console.Write("无换行");
Console.WriteLine("带换行");
Console.Error.WriteLine("输出到 stderr");

// 字符串内插 (C# 6+)
string name = "Alice";
int age = 30;
Console.WriteLine($"{name} is {age} years old");
```

```rust
// Rust 输出 — 全部是宏 (注意末尾带有 !)
print!("无换行");                   // → 输出到 stdout，无换行
println!("带换行");                 // → 输出到 stdout 并带有换行
eprint!("输出到 stderr");           // → 输出到 stderr，无换行  
eprintln!("输出到 stderr 并换行");   // → 输出到 stderr 并带有换行

// 字符串格式化 (类似于 $"" 内插)
let name = "Alice";
let age = 30;
println!("{name} is {age} years old");     // 行内变量捕获 (Rust 1.58+)
println!("{} is {} years old", name, age); // 位置参数
```

### 格式说明符 (Format Specifiers)
```csharp
// C# 格式说明符
Console.WriteLine($"{price:F2}");         // 固定小数：29.99
Console.WriteLine($"{count:D5}");         // 填充整数：00042
Console.WriteLine($"{value,10}");         // 右对齐，宽度 10
Console.WriteLine($"{value,-10}");        // 左对齐，宽度 10
Console.WriteLine($"{hex:X}");            // 十六进制：FF
Console.WriteLine($"{ratio:P1}");         // 百分比：85.0%
```

```rust
// Rust 格式说明符
println!("{price:.2}");          // 2 位小数：29.99
println!("{count:05}");          // 零填充，宽度 5：00042
println!("{value:>10}");         // 右对齐，宽度 10
println!("{value:<10}");         // 左对齐，宽度 10
println!("{value:^10}");         // 居中对齐，宽度 10
println!("{hex:#X}");            // 带有前缀的十六进制：0xFF
println!("{hex:08X}");           // 十六进制零填充：000000FF
println!("{bits:#010b}");        // 带有前缀的二进制：0b00001010
println!("{big}", big = 1_000_000); // 命名参数
```

### Debug vs Display 打印
```rust
// {:?}  — Debug 特性 (面向开发者，可自动派生)
// {:#?} — “漂亮”打印模式的 Debug (有缩进，多行)
// {}    — Display 特性 (面向终端用户，必须手动实现)

#[derive(Debug)] // 自动生成 Debug 输出支持
struct Point { x: f64, y: f64 }

let p = Point { x: 1.5, y: 2.7 };

println!("{:?}", p);   // Point { x: 1.5, y: 2.7 }   — 紧凑调试信息
println!("{:#?}", p);  // Point {                     — 易读调试信息
                        //     x: 1.5,
                        //     y: 2.7,
                        // }
// println!("{}", p);  // ❌ ERROR: Point 未实现 Display
```

```csharp
// C# 对应概念:
// {:?}  ≈ object.GetType().ToString() 或反射转储 (Reflection dump)
// {}    ≈ object.ToString()
// 在 C# 中你重写 ToString(); 在 Rust 中你实现 Display
```

### 快速参考

| C# 功能项 | Rust 对应项 | 输出效果 |
|----|------|--------|
| `Console.WriteLine(x)` | `println!("{x}")` | Display 格式化 |
| `$"{x}"` (内插) | `format!("{x}")` | 返回一个 `String` |
| `x.ToString()` | `x.to_string()` | 要求实现 `Display` 特性 |
| 重写 `ToString()` | `impl Display` | 面向用户的输出 |
| 调试视图 | `{:?}` 或 `dbg!(x)` | 开发者视角输出 |
| `String.Format("{0:F2}", x)` | `format!("{x:.2}")` | 格式化后的 `String` |

***

## 类型转换与强制转换

C# 拥有隐式转换、显式强制转换 `(int)x` 以及 `Convert.To*()`。Rust 则更加严格 —— 不允许任何隐式的数值转换。

### 数值转换
```csharp
// C# — 隐式和显式转换
int small = 42;
long big = small;              // 隐式加宽：OK
double d = small;              // 隐式加宽：OK
int truncated = (int)3.14;     // 显式缩窄：3
byte b = (byte)300;            // 默不作声的溢出：44
```

```rust
// Rust — 所有数值转换必须是显式的
let small: i32 = 42;
let big: i64 = small as i64;       // 加宽：使用 'as' 显式转换
let d: f64 = small as f64;         // 整数转浮点数：显式
let truncated: i32 = 3.14_f64 as i32; // 缩窄：3 (直接截断)
let b: u8 = 300_u16 as u8;        // 溢出：回绕至 44 (类似于 C# 的 unchecked)

// 使用 TryFrom 进行安全转换
use std::convert::TryFrom;
let safe: Result<u8, _> = u8::try_from(300_u16); // Err — 超出范围
let ok: Result<u8, _>   = u8::try_from(42_u16);  // Ok(42)

// 字符串解析 — 返回 Result，而不是 bool + out 参数
let parsed: Result<i32, _> = "42".parse::<i32>();   // Ok(42)
let bad: Result<i32, _>    = "abc".parse::<i32>();  // Err(ParseIntError)
```

### 字符串转换
```csharp
// C#
int n = 42;
string s = n.ToString();          // "42"
int back = int.Parse(s);          // 42 或抛出异常
```

```rust
// Rust — 通过 Display 实现 to_string()，通过 FromStr 实现 parse()
let n: i32 = 42;
let s: String = n.to_string();            // "42" (使用 Display 特性)
let back: i32 = s.parse().unwrap();       // 42 或 panic

// &str ↔ String 转换 (这是 Rust 中最常见的转换)
let owned: String = "hello".to_string();    // &str → String
let owned2: String = String::from("hello"); // &str → String (等效)
let borrowed: &str = &owned;                // String → &str (零开销借用)
```

### 引用转换 (不支持继承转型！)
```csharp
// C# — 向上转型 (Upcasting) 和向下转型 (Downcasting)
Animal a = new Dog();              // 向上转型 (隐式)
Dog d = (Dog)a;                    // 向下转型 (显式，可能报错)
if (a is Dog dog) { /* ... */ }    // 安全的向下转型
```

```rust
// Rust — 没有继承关系，因此没有向上/向下转型
// 请使用 Trait 对象实现多态：
let animal: Box<dyn Animal> = Box::new(Dog);

// 实践中，通常使用枚举 (Enums) 而非向下转型：
enum Animal {
    Dog(Dog),
    Cat(Cat),
}
match animal {
    Animal::Dog(d) => { /* 使用 d */ }
    Animal::Cat(c) => { /* 使用 c */ }
}
```

***

## 注释与文档

### 普通注释
```csharp
// C# 注释
// 单行注释
/* 多行
   注释 */

/// <summary>
/// XML 文档注释
/// </summary>
public string Greet(string name) { ... }
```

```rust
// Rust 注释
// 单行注释
/* 多行
   注释 */

/// 文档注释 (类似于 C# 的 ///)
/// 这里的文档注释支持 Markdown 格式。
/// 
/// # 参数
/// 
/// * `name` - 用户名称的字符串切片
/// 
/// # 示例
/// 
/// ```
/// let greeting = greet("Alice");
/// assert_eq!(greeting, "Hello, Alice!");
/// ```
pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}
```

### 生成文档
```bash
# 生成文档 (类似于 C# 中的 XML 文档生成)
cargo doc --open

# 运行文档中的代码示例测试
cargo test --doc
```

---

## 练习

<details>
<summary><strong>🏋️ 练习：类型安全的温度转换</strong> (点击展开)</summary>

创建一个 Rust 程序，完成以下任务：
1. 为摄氏度的绝对零度 (`-273.15`) 声明一个 `const`。
2. 为已执行转换的次数声明一个 `static` 计数器（使用 `AtomicU32`）。
3. 编写一个函数 `celsius_to_fahrenheit(c: f64) -> f64`，如果温度低于绝对零度，则返回 `f64::NAN`（表示拒绝该输入）。
4. 通过遮蔽（shadowing）演示以下过程：将字符串 `"98.6"` 解析为 `f64` 类型，然后再进行转换。

<details>
<summary>🔑 参考答案</summary>

```rust
use std::sync::atomic::{AtomicU32, Ordering};

const ABSOLUTE_ZERO_C: f64 = -273.15;
static CONVERSION_COUNT: AtomicU32 = AtomicU32::new(0);

fn celsius_to_fahrenheit(c: f64) -> f64 {
    if c < ABSOLUTE_ZERO_C {
        return f64::NAN;
    }
    CONVERSION_COUNT.fetch_add(1, Ordering::Relaxed);
    c * 9.0 / 5.0 + 32.0
}

fn main() {
    let temp = "98.6";           // 类型为 &str
    let temp: f64 = temp.parse().unwrap(); // 遮蔽为 f64 类型
    let temp = celsius_to_fahrenheit(temp); // 遮蔽为华氏度结果
    println!("{temp:.1}°F");
    println!("转换次数: {}", CONVERSION_COUNT.load(Ordering::Relaxed));
}
```

</details>
</details>

***
