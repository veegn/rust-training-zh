[English Original](../en/ch05-data-structures.md)

### Rust 数组类型

> **你将学到：** Rust 的核心数据结构 —— 数组 (Arrays)、元组 (Tuples)、切片 (Slices)、字符串 (Strings)、结构体 (Structs)、`Vec` 以及 `HashMap`。这是一个内容密集的章节；请重点理解 `String` 与 `&str` 的区别，以及结构体的工作原理。你将在第 7 章深入复习引用与借用。

- 数组包含固定数量的相同类型的元素。
    - 与所有其他 Rust 类型一样，数组默认是不可变的（除非显式使用 `mut`）。
    - 数组使用 `[]` 进行索引，并且会进行边界检查。可以使用 `len()` 方法获取数组的长度。
```rust
    fn get_index(y : usize) -> usize {
        y+1        
    }
    
    fn main() {
        // 初始化一个包含 3 个元素的数组，并将它们全设为 42
        let a : [u8; 3] = [42; 3];
        // 替代语法
        // let a = [42u8, 42u8, 42u8];
        for x in a {
            println!("{x}");
        }
        let y = get_index(a.len());
        // 取消下方注释将导致运行时的 Panic (崩溃)
        //println!("{}", a[y]);
    }
```

---
### 数组类型 (续)
- 数组可以嵌套。
    - Rust 有几种内建的打印格式化程序。在下方代码中，`:?` 是 `debug` 打印格式化程序。使用 `:#?` 则可以进行“美化打印” (pretty print)。这些格式化程序可以针对每个类型进行自定义（后续会详细介绍）。
```rust
    fn main() {
        let a = [
            [40, 0], // 定义嵌套数组
            [41, 0],
            [42, 1],
        ];
        for x in a {
            println!("{x:?}");
        }
    }
```
---
### Rust 元组 (Tuples)
- 元组具有固定大小，可以将任意类型组合成单个复合类型。
    - 组成的各个类型可以通过它们的相对位置（.0, .1, .2, ...）进行索引。空元组 `()` 被称为**单元值 (Unit Value)**，等同于 C 语言中的 `void` 返回值。
    - Rust 支持元组解构（Destructuring），方便将变量绑定到各个元素上。
```rust
fn get_tuple() -> (u32, bool) {
    (42, true)        
}

fn main() {
   let t : (u8, bool) = (42, true);
   let u : (u32, bool) = (43, false);
   println!("{}, {}", t.0, t.1);
   println!("{}, {}", u.0, u.1);
   let (num, flag) = get_tuple(); // 元组解构
   println!("{num}, {flag}");
}
```

---

### Rust 引用 (References)
- Rust 中的引用大致等同于 C 中的指针，但存在一些关键区别：
    - 在任何时间点，可以有任意数量的**只读 (不可变)** 变量引用。引用不能超出变量的作用域（这是一个名为**生命周期 (Lifetime)** 的核心概念；稍后详细讨论）。
    - 对一个可变变量，只允许有一个**可写 (可变)** 引用，且该引用不能与其他任何引用重叠。
```rust
fn main() {
    let mut a = 42;
    {
        let b = &a;
        let c = b;
        println!("{} {}", *b, *c); // 编译器会自动解引用 *c
        
        let d = &mut a;
        
        /* 
         * 取消下方注释将导致程序无法编译，
         * 因为在可变引用 `d` 处于当前作用域活跃状态时使用了 `b`。
         * 
         * 你不能在同一作用域内同时使用可变引用和不可变引用！
         */
        // println!("{}", *b);
    }
    let d = &mut a; // OK: b 和 c 已不在作用域内
    *d = 43;
}
```

---

# Rust 切片 (Slices)
- Rust 引用可用于创建数组的子集：
    - 与长度在编译时即固定的数组不同，切片的大小可以是任意的。在内部，切片是通过“胖指针 (Fat-pointer)”实现的，其中包含切片的长度以及指向原始数组起始元素的指针。
```rust
fn main() {
    let a = [40, 41, 42, 43];
    let b = &a[1..a.len()]; // 包含从第二个元素开始的切片
    let c = &a[1..]; // 与上方等效
    let d = &a[..]; // 与 &a[0..] 或 &a[0..a.len()] 等效
    println!("{b:?} {c:?} {d:?}");
}
```

---

# Rust 常量 (Constants) 与 静态变量 (Statics)
- `const` 关键字可用于定义常量。常量值在**编译时**进行求值，并会内联到程序中。
- `static` 关键字用于定义类似于 C/C++ 中的全局变量。静态变量具有可寻址的内存位置，且在程序整个生命周期内只被创建一次。
```rust
const SECRET_OF_LIFE: u32 = 42;
static GLOBAL_VARIABLE : u32 = 2;
fn main() {
    println!("生命之秘是 {}", SECRET_OF_LIFE);
    println!("全局变量的值是 {GLOBAL_VARIABLE}")
}
```

---

# Rust 字符串：String vs &str

- Rust 有 **两类** 字符串类型，分别用于不同目的：
    - `String` —— 有所有权的、堆分配的、可增长的（类似于 C 语言中使用 `malloc` 分配的缓冲区，或 C++ 中的 `std::string`）。
    - `&str` —— 借用的、轻量级的引用（类似于 C 语言中带有长度信息的 `const char*`，或 C++ 中的 `std::string_view` —— 但 `&str` 是经过**生命周期检查**的，因此永远不会产生悬空引用）。
    - 与 C 语言中以 null 结尾的字符串不同，Rust 字符串会追踪其长度，并保证是有效的 UTF-8 编码。

> **针对 C++ 开发者**：`String` ≈ `std::string`，`&str` ≈ `std::string_view`。与 `std::string_view` 不同的是，`&str` 通过借用检查器保证在其整个生命周期内都是有效的。

## String vs &str：所有权与借用

> **生产环境模式**：参见 [JSON 处理：nlohmann::json → serde](ch17-2-avoiding-unchecked-indexing.md#json-handling-nlohmannjson--serde) 了解在生产代码中字符串处理如何与 serde 配合工作。

| **维度** | **C `char*`** | **C++ `std::string`** | **Rust `String`** | **Rust `&str`** |
|------------|--------------|----------------------|-------------------|----------------|
| **内存** | 手动管理 (`malloc`/`free`) | 堆分配，拥有缓冲区 | 堆分配，自动释放 | 借用引用 (生命周期检查) |
| **可变性** | 始终可通过指针修改 | 可变 | 使用 `mut` 时可变 | 始终不可变 |
| **长度信息** | 无 (依赖 `'\0'`) | 追踪长度与容量 | 追踪长度与容量 | 追踪长度 (胖指针) |
| **编码** | 未指定 (通常为 ASCII) | 未指定 (通常为 ASCII) | 保证为有效的 UTF-8 | 保证为有效的 UTF-8 |
| **Null 终止符** | 必须有 | 必须有 (`c_str()`) | 不使用 | 不使用 |

```rust
fn main() {
    // &str - 字符串切片 (借用的、不可变的，通常是字符串字面量)
    let greeting: &str = "你好";  // 指向只读内存

    // String - 有所有权的、堆分配的、可增长的
    let mut owned = String::from(greeting);  // 将数据复制到堆中
    owned.push_str("，Rust 世界！");        // 增长字符串
    owned.push('!');                       // 追加单个字符

    // 在 String 和 &str 之间转换
    let slice: &str = &owned;          // String -> &str (开销极低，仅为借用)
    let owned2: String = slice.to_string();  // &str -> String (涉及内存分配)
    let owned3: String = String::from(slice); // 与上方等效

    // 字符串拼接 (注意：+ 会消耗左侧的操作数)
    let hello = String::from("Hello");
    let world = String::from(", World!");
    let combined = hello + &world;  // hello 被移动 (消耗)，world 被借用
    // println!("{hello}");  // 无法编译：hello 已经被移动了

    // 使用 format! 宏避免移动问题
    let a = String::from("Hello");
    let b = String::from("World");
    let combined = format!("{a}, {b}!");  // a 和 b 都不会被消耗

    println!("{combined}");
}
```

## 为什么不能直接使用 `[]` 索引字符串
```rust
fn main() {
    let s = String::from("hello");
    // let c = s[0];  // 无法编译！Rust 字符串是 UTF-8 编码，而非简单的字节数组

    // 安全的替代方案：
    let first_char = s.chars().next();           // Option<char>: Some('h')
    let as_bytes = s.as_bytes();                 // &[u8]: 原始 UTF-8 字节
    let substring = &s[0..1];                    // &str: "h" (字节范围，必须在有效的 UTF-8 边界上)

    println!("首字符: {:?}", first_char);
    println!("字节序列: {:?}", &as_bytes[..5]);
}
```

## 练习：字符串操作

🟢 **入门级**
- 编写一个函数 `fn count_words(text: &str) -> usize`，用于计算字符串中由空格分隔的单词数量。
- 编写一个函数 `fn longest_word(text: &str) -> &str`，返回字符串中最长的单词（提示：你需要思考生命周期 —— 为什么返回类型必须是 `&str` 而不是 `String`？）。

<details><summary>参考答案 (点击展开)</summary>

```rust
fn count_words(text: &str) -> usize {
    text.split_whitespace().count()
}

fn longest_word(text: &str) -> &str {
    text.split_whitespace()
        .max_by_key(|word| word.len())
        .unwrap_or("")
}

fn main() {
    let text = "the quick brown fox jumps over the lazy dog";
    println!("单词数: {}", count_words(text));       // 9
    println!("最长单词: {}", longest_word(text));     // "jumps"
}
```

</details>

---

# Rust 结构体 (Structs)
- `struct` 关键字用于声明用户自定义的结构体类型。
    - `struct` 成员既可以是命名的，也可以是匿名的（元组结构体）。
- 与 C++ 等语言不同，Rust 中没有“数据继承”的概念。
```rust
fn main() {
    struct MyStruct {
        num: u32,
        is_secret_of_life: bool,
    }
    let x = MyStruct {
        num: 42,
        is_secret_of_life: true,
    };
    let y = MyStruct {
        num: x.num,
        is_secret_of_life: x.is_secret_of_life,
    };
    let z = MyStruct { num: x.num, ..x }; // ..x 表示复制剩余的未显式指定的字段
    println!("{} {} {}", x.num, y.is_secret_of_life, z.num);
}
```

# Rust 元组结构体 (Tuple Structs)
- Rust 元组结构体与元组类似，其具体的各个字段没有名称。
    - 与元组一样，各个元素通过 .0, .1, .2, ... 进行访问。元组结构的一个常见用例是包装原始类型以创建自定义类型。**这对于避免混淆同一类型的不同含义非常有用。**
```rust
struct WeightInGrams(u32);
struct WeightInMilligrams(u32);
fn to_weight_in_grams(kilograms: u32) -> WeightInGrams {
    WeightInGrams(kilograms * 1000)
}

fn to_weight_in_milligrams(w : WeightInGrams) -> WeightInMilligrams  {
    WeightInMilligrams(w.0 * 1000)
}

fn main() {
    let x = to_weight_in_grams(42);
    let y = to_weight_in_milligrams(x);
    // let z : WeightInGrams = x;  // 无法编译：x 已经在调用 to_weight_in_milligrams() 时移动了 (Move)
    // let a : WeightInGrams = y;   // 无法编译：类型不匹配 (WeightInMilligrams 与 WeightInGrams 不同)
}
```

**注意**：`#[derive(...)]` 属性可以为结构体和枚举自动生成常见的 Trait 实现。你会在本课程中经常看到它：
```rust
#[derive(Debug, Clone, PartialEq)]
struct Point { x: i32, y: i32 }

fn main() {
    let p = Point { x: 1, y: 2 };
    println!("{:?}", p);           // Debug: 因为 #[derive(Debug)] 而生效
    let p2 = p.clone();           // Clone: 因为 #[derive(Clone)] 而生效
    assert_eq!(p, p2);            // PartialEq: 因为 #[derive(PartialEq)] 而生效
}
```
我们稍后会深入探讨 Trait 系统，但 `#[derive(Debug)]` 非常实用，你应该为几乎每个创建的 `struct` 和 `enum` 都加上它。

---

# Rust Vec 类型
- `Vec<T>` 类型实现了动态的堆分配缓冲区（类似于 C 语言中手动管理的 `malloc`/`realloc` 数组，或 C++ 中的 `std::vector`）。
    - 与大小固定的数组不同，`Vec` 可以在运行时增长或缩小。
    - `Vec` 拥有其数据的所有权，并自动管理内存的分配与释放。
- 常用操作：`push()`、`pop()`、`insert()`、`remove()`、`len()`、`capacity()`。
```rust
fn main() {
    let mut v = Vec::new();    // 创建空向量，类型根据后续使用推导
    v.push(42);                // 在末尾添加元素 - Vec<i32>
    v.push(43);                
    
    // 安全迭代 (推荐方式)
    for x in &v {              // 借用元素，不消耗向量的所有权
        println!("{x}");
    }
    
    // 初始化快捷方式
    let mut v2 = vec![1, 2, 3, 4, 5];           // 使用宏进行初始化
    let v3 = vec![0; 10];                       // 初始化为 10 个 0
    
    // 安全的访问方法 (优于通过索引访问)
    match v2.get(0) {
        Some(first) => println!("首个元素: {first}"),
        None => println!("空向量"),
    }
    
    // 实用方法
    println!("长度: {}, 容量: {}", v2.len(), v2.capacity());
    if let Some(last) = v2.pop() {             // 移除并返回最后一个元素
        println!("弹出的元素: {last}");
    }
    
    // 危险操作：直接索引访问 (可能导致 Panic！)
    // println!("{}", v2[100]);  // 将在运行时导致崩溃
}
```
> **生产环境模式**：参见 [避免未检查的索引访问](ch17-2-avoiding-unchecked-indexing.md#avoiding-unchecked-indexing) 了解生产环境 Rust 代码中关于 `.get()` 的安全模式。

# Rust HashMap 类型
- `HashMap` 实现了通用的 `键 (Key)` -> `值 (Value)` 查找（也称为“字典”或“映射”）。
```rust
fn main() {
    use std::collections::HashMap;      // 与 Vec 不同，HashMap 需要显式导入
    let mut map = HashMap::new();       // 分配一个空的 HashMap
    map.insert(40, false);  // 类型被推导为 int -> bool
    map.insert(41, false);
    map.insert(42, true);
    for (key, value) in map {
        println!("{key} {value}");
    }
    let map = HashMap::from([(40, false), (41, false), (42, true)]);
    if let Some(x) = map.get(&43) {
        println!("43 映射到了 {:?}", x);
    } else {
        println!("未找到 43 的映射");
    }
    let x = map.get(&43).or(Some(&false));  // 如果未找到键，则提供默认值
    println!("{x:?}"); 
}
```

# 练习：Vec 与 HashMap

🟢 **入门级**
- 创建一个带有若干条目的 `HashMap<u32, bool>`（确保其中有些值为 `true`，有些为 `false`）。遍历该 HashMap 的所有元素，将键 (Keys) 放入一个 `Vec` 中，将值 (Values) 放入另一个 `Vec` 中。

<details><summary>参考答案 (点击展开)</summary>

```rust
use std::collections::HashMap;

fn main() {
    let map = HashMap::from([(1, true), (2, false), (3, true), (4, false)]);
    let mut keys = Vec::new();
    let mut values = Vec::new();
    for (k, v) in &map {
        keys.push(*k);
        values.push(*v);
    }
    println!("键 (Keys):   {:?}", keys);
    println!("值 (Values): {:?}", values);

    // 替代方案：使用带有 unzip() 的迭代器
    let (keys2, values2): (Vec<u32>, Vec<bool>) = map.into_iter().unzip();
    println!("键 (unzip):   {:?}", keys2);
    println!("值 (unzip):   {:?}", values2);
}
```

</details>

---

## 深度解析：C++ 引用 vs Rust 引用

> **针对 C++ 开发者**：C++ 程序员通常假设 Rust 的 `&T` 与 C++ 的 `T&` 工作方式相同。虽然表面上相似，但存在一些容易引起混淆的根本区别。C 开发者可以跳过此部分 —— 关于 Rust 引用的内容在 [所有权与借用](ch07-ownership-and-borrowing.md) 中有详细介绍。

#### 1. 没有右值引用 (Rvalue References) 或万能引用 (Universal References)

在 C++ 中，`&&` 根据上下文有两种含义：

```cpp
// C++: && 代表不同的含义：
int&& rref = 42;           // 右值引用 — 绑定到临时变量
void process(Widget&& w);   // 右值引用 — 调用者必须显式调用 std::move

// 万能（转发）引用 — 模板推导上下文：
template<typename T>
void forward(T&& arg) {     // 注意：这不一定是右值引用！取决于推导为 T& 还是 T&&
    inner(std::forward<T>(arg));  // 完美转发
}
```

**在 Rust 中：这些都不存在。** `&&` 仅仅是逻辑“与 (AND)”运算符。

```rust
// Rust: && 仅仅是逻辑与运算符
let a = true && false; // false

// Rust 没有右值引用，没有万能引用，也没有完美转发。
// 取而代之的是：
//   - 对于非 Copy 类型，移动 (Move) 是默认行为（无需显式调用 std::move）
//   - 泛型 + Trait 约束取代了万能引用
//   - 没有“绑定到临时变量”的特殊区分 —— 值就是值

fn process(w: Widget) { }      // 获取所有权（类似于 C++ 的值传递参数 + 隐式移动）
fn process_ref(w: &Widget) { } // 不可变借用（类似于 C++ 的 const T&）
fn process_mut(w: &mut Widget) { } // 可变借用（类似于 C++ 的 T&，但是具有排他性）
```

| C++ 概念 | Rust 等价概念 | 备注 |
|-------------|-----------------|-------|
| `T&` (左值引用) | `&T` 或 `&mut T` | Rust 将其拆分为共享引用与独占引用 |
| `T&&` (右值引用) | 直接使用 `T` | 按值接收 = 获取所有权 |
| 模板中的 `T&&` (万能引用) | `impl Trait` 或 `<T: Trait>` | 泛型取代了转发机制 |
| `std::move(x)` | `x` (直接使用) | 移动是默认行为 |
| `std::forward<T>(x)` | 无需等价物 | 没有万能引用需要转发 |

---

#### 2. 移动是按字节进行的 —— 没有移动构造函数

在 C++ 中，移动是一个*用户定义的操作*（通过移动构造函数 / 移动赋值运算符实现）。而在 Rust 中，移动始终是对值的 **按字节进行的内存拷贝 (bitwise memcpy)**，并且原变量会失效：

```rust
// Rust 的移动 = 拷贝字节，并将原变量标记为无效
let s1 = String::from("hello");
let s2 = s1; // s1 的字节被拷贝到 s2 的栈槽中
              // s1 现在失效了 —— 编译器会强制执行这一点
// println!("{s1}"); // ❌ 编译错误：值在移动后被使用
```

```cpp
// C++ 的移动 = 调用移动构造函数 (用户定义的！)
std::string s1 = "hello";
std::string s2 = std::move(s1); // 调用字符串的移动构造函数
// s1 现在处于“有效但未指定状态”的“僵尸”状态
std::cout << s1; // 能够编译！打印结果... 不确定 (通常是空字符串)
```

**结论**：
- Rust 没有“五法则”（Rule of Five）—— 不需要定义拷贝构造函数、移动构造函数、拷贝赋值、移动赋值或析构函数。
- 没有移动后的“僵尸”状态 —— 编译器直接禁止访问。
- 移动时无需考虑 `noexcept` —— 按字节拷贝不会抛出异常。

#### 3. 自动解引用 (Auto-Deref)：编译器透视间接引用

Rust 通过 `Deref` trait 自动对多层指针/包装器进行解引用。这在 C++ 中没有等价物：

```rust
use std::sync::{Arc, Mutex};

// 嵌套包装：Arc<Mutex<Vec<String>>>
let data = Arc::new(Mutex::new(vec!["hello".to_string()]));

// 在 C++ 中，你需要在每一层进行显式解锁和手动解引用。
// 在 Rust 中，编译器会自动通过 Arc → Mutex → MutexGuard → Vec 进行解引用：
let guard = data.lock().unwrap(); // Arc 自动解引用为 Mutex
let first: &str = &guard[0];      // MutexGuard→Vec (Deref), Vec[0] (Index),
                                   // &String→&str (Deref 强制转换)
println!("首个元素: {first}");

// 方法调用也支持自动解引用：
let boxed_string = Box::new(String::from("hello"));
println!("长度: {}", boxed_string.len());  // Box→String，然后调用 String::len()
// 无需写成 (*boxed_string).len() 或 boxed_string->len()
```

**Deref 强制转换 (Deref coercion)** 也适用于函数参数 —— 编译器会自动插入解引用动作以使类型匹配：

```rust
fn greet(name: &str) {
    println!("你好，{name}");
}

fn main() {
    let owned = String::from("Alice");
    let boxed = Box::new(String::from("Bob"));
    let arced = std::sync::Arc::new(String::from("Carol"));

    greet(&owned);  // &String → &str  (1 次解引用转换)
    greet(&boxed);  // &Box<String> → &String → &str  (2 次解引用转换)
    greet(&arced);  // &Arc<String> → &String → &str  (2 次解引用转换)
    greet("Dave");  // 本身就是 &str — 无需转换
}
// 在 C++ 中，你可能需要为每种情况调用 .c_str() 或进行显式转换。
```

**Deref 链**：当你调用 `x.method()` 时，Rust 的方法解析器会尝试接收类型 `T`，然后是 `&T`，接着是 `&mut T`。如果都不匹配，它会通过 `Deref` trait 进行解引用，并对目标类型重复上述过程。这一过程可以穿透多层 —— 这就是为什么 `Box<Vec<T>>` 可以像 `Vec<T>` 一样“无缝工作”的原因。**Deref 强制转换**（针对函数参数）是一个相关但独立的机制，它通过链接 `Deref` 实现将 `&Box<String>` 自动转换为 `&str`。

---

#### 4. 没有空引用，也没有可选引用

```cpp
// C++: 引用理论上不能为 null，但指针可以，且两者界限模糊
Widget& ref = *ptr;  // 如果 ptr 为 null → 触发未定义行为 (UB)
Widget* opt = nullptr;  // 通过指针实现“可选”引用
```

```rust
// Rust: 引用始终有效 —— 由借用检查器保证
// 在安全代码中无法创建空引用或悬空引用
let r: &i32 = &42; // 始终有效

// “可选引用”必须显式表达：
let opt: Option<&Widget> = None; // 意图清晰，非空指针
if let Some(w) = opt {
    w.do_something(); // 仅当引用存在时才可访问
}
```

#### 5. 引用不可被“重定向 (Reseated)”

```cpp
// C++: 引用是别名 — 一旦绑定即不可更改
int a = 1, b = 2;
int& r = a;
r = b;  // 这是将 b 的值赋给 a — 而不是让 r 重新绑定到 b！
// a 的值现在是 2，r 依然指向 a
```

```rust
// Rust: let 绑定可以被遮蔽 (Shadowing)，但引用遵循不同的规则
let a = 1;
let b = 2;
let r = &a;
// r = &b;   // ❌ 无法为不可变变量重新赋值
let r = &b;  // ✅ 但你可以使用新的绑定“遮蔽”原有的 r
             // 旧的绑定已不复存在，但这并非重定向原有引用

// 使用 mut 关键字：
let mut r = &a;
r = &b;      // ✅ r 现在指向了 b —— 这就是重定向 (而不是通过引用赋值)
```

> **心智模型**：在 C++ 中，引用是一个对象的永久别名。在 Rust 中，引用是一个值（一个带有生命周期保证的指针），它遵循普通的变量绑定规则 —— 默认不可变，只有在声明为 `mut` 时才可以重新绑定。

---
