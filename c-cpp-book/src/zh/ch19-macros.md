[English Original](../en/ch19-macros.md)

## Rust 宏：从预处理器到元编程

> **你将学到：** Rust 宏的工作原理、何时应使用宏而非函数或泛型，以及它们是如何替代 C/C++ 预处理器的。在本章结束时，你将能够编写自己的 `macro_rules!` 宏，并理解 `#[derive(Debug)]` 的底层原理。

宏是你在 Rust 中最早接触到的事物之一（第一行代码中的 `println!("hello")`），但也是大多数课程最后才会讲解的内容。本章旨在填补这一空白。

### 为什么需要宏

函数和泛型处理了 Rust 中大部分的代码复用工作。而在类型系统无法触及的领域，宏填补了这些空白：

| 需求 | 函数/泛型？ | 宏？ | 原因 |
|------|-------------------|--------|-----|
| 计算一个值 | ✅ `fn max<T: Ord>(a: T, b: T) -> T` | — | 类型系统足以胜任 |
| 接受可变数量的参数 | ❌ Rust 不支持变长参数函数 (Variadic Functions) | ✅ `println!("{} {}", a, b)` | 宏可以接受任意数量的标记 (Tokens) |
| 生成重复的 `impl` 块 | ❌ 单靠泛型无法实现 | ✅ `macro_rules!` | 宏在编译期生成代码 |
| 在编译期运行代码 | ❌ `const fn` 的功能有限 | ✅ 过程宏 (Procedural Macros) | 可以在编译期运行完整的 Rust 代码 |
| 条件性地包含代码 | ❌ | ✅ `#[cfg(...)]` | 属性宏可以控制编译过程 |

如果你来自 C/C++ 背景，可以将宏看作是**预处理器的唯一正确替代方案** —— 不同之处在于，Rust 宏操作的是语法树 (Syntax Tree) 而非原始文本，因此它们具有卫生性（Hygiene，无意外的命名冲突），并且具备类型感知能力。

> **致 C 开发者：** Rust 宏完全替代了 `#define`。Rust 中不存在文本预处理器。关于预处理器到 Rust 的完整映射关系，请参阅 [第 18 章](ch18-cpp-rust-semantic-deep-dives.md)。

---

## 使用 `macro_rules!` 编写声明式宏

声明式宏（也被称为“示例宏”）是 Rust 中最常见的宏形式。它们在语法上使用模式匹配，类似于在值上使用 `match`。

### 基本语法

```rust
macro_rules! say_hello {
    () => {
        println!("你好！");
    };
}

fn main() {
    say_hello!();  // 展开为：println!("你好！");
}
```

名称后的 `!` 用于告诉调用者（以及编译器）这是一个宏调用。

### 带参数的模式匹配

宏使用“片段说明符 (Fragment Specifiers)”在**标记树 (Token Trees)** 上进行匹配：

```rust
macro_rules! greet {
    // 模式 1：无参数
    () => {
        println!("你好，世界！");
    };
    // 模式 2：有一个表达式参数
    ($name:expr) => {
        println!("你好，{}！", $name);
    };
}

fn main() {
    greet!();           // "你好，世界！"
    greet!("Rust");     // "你好，Rust！"
}
```

#### 片段说明符参考

| 说明符 | 匹配项 | 示例 |
|-----------|---------|---------|
| `$x:expr` | 任何表达式 | `42`, `a + b`, `foo()` |
| `$x:ty` | 类型 | `i32`, `Vec<String>`, `&str` |
| `$x:ident` | 标识符 | `foo`, `my_var` |
| `$x:pat` | 模式 | `Some(x)`, `_`, `(a, b)` |
| `$x:stmt` | 语句 | `let x = 5;` |
| `$x:block` | 代码块 | `{ println!("hi"); 42 }` |
| `$x:literal` | 字面量 | `42`, `"hello"`, `true` |
| `$x:tt` | 单个标记树 | 任何内容 —— 通配符 |
| `$x:item` | 项 (fn, struct, impl, 等) | `fn foo() {}` |

---

### 重复 (Repetition) —— 杀手锏功能

C/C++ 的宏无法进行循环操作。Rust 宏则可以重复特定的模式：

```rust
macro_rules! make_vec {
    // 匹配零个或多个由逗号分隔的表达式
    ( $( $element:expr ),* ) => {
        {
            let mut v = Vec::new();
            $( v.push($element); )*  // 对每一个匹配到的元素重复此操作
            v
        }
    };
}

fn main() {
    let v = make_vec![1, 2, 3, 4, 5];
    println!("{v:?}");  // [1, 2, 3, 4, 5]
}
```

`$( ... ),*` 这一语法表示“匹配零个或多个符合此模式的内容，并以逗号进行分隔”。在展开部分中的 `$( ... )*` 则会对每一个匹配结果重复展开一次。

> **这正是标准库中 `vec![]` 的实现方式。** 实际的源代码如下：
> ```rust
> macro_rules! vec {
>     () => { Vec::new() };
>     ($elem:expr; $n:expr) => { vec::from_elem($elem, $n) };
>     ($($x:expr),+ $(,)?) => { <[_]>::into_vec(Box::new([$($x),+])) };
> }
> ```
> 末尾的 `$(,)?` 允许可选的尾随逗号。

#### 重复操作符

| 操作符 | 含义 | 示例 |
|----------|---------|---------|
| `$( ... )*` | 零个或多个 | `vec![]`, `vec![1]`, `vec![1, 2, 3]` |
| `$( ... )+` | 一个或多个 | 要求至少有一个元素 |
| `$( ... )?` | 零个或一个 | 可选的元素 |

---

### 实战示例：`hashmap!` 构造器

标准库提供了 `vec![]` 但没有提供 `hashmap!{}`。让我们动手写一个：

```rust
macro_rules! hashmap {
    ( $( $key:expr => $value:expr ),* $(,)? ) => {
        {
            let mut map = std::collections::HashMap::new();
            $( map.insert($key, $value); )*
            map
        }
    };
}

fn main() {
    let scores = hashmap! {
        "Alice" => 95,
        "Bob" => 87,
        "Carol" => 92,  // 幸亏有了 $(,)?，尾随逗号也是可以接受的
    };
    println!("{scores:?}");
}
```

---

### 实战示例：诊断检查宏

一种在嵌入式/诊断代码中常见的模式 —— 检查某个条件并在不满足时返回错误：

```rust
use thiserror::Error;

#[derive(Error, Debug)]
enum DiagError {
    #[error("检查失败: {0}")]
    CheckFailed(String),
}

macro_rules! diag_check {
    ($cond:expr, $msg:expr) => {
        if !($cond) {
            return Err(DiagError::CheckFailed($msg.to_string()));
        }
    };
}

fn run_diagnostics(temp: f64, voltage: f64) -> Result<(), DiagError> {
    diag_check!(temp < 85.0, "GPU 温度过高");
    diag_check!(voltage > 0.8, "导轨电压过低");
    diag_check!(voltage < 1.5, "导轨电压过高");
    println!("所有检查均已通过");
    Ok(())
}
```

> **C/C++ 对比：**
> ```c
> // C 预处理器 —— 仅是文本替换，没有类型安全，没有卫生性
> #define DIAG_CHECK(cond, msg) \
>     do { if (!(cond)) { log_error(msg); return -1; } } while(0)
> ```
> Rust 版本返回一个正式的 `Result` 类型，不存在多次求值 (Double-evaluation) 的风险，且编译器会检查 `$cond` 是否确实是一个 `bool` 类型的表达式。

---

### 卫生性 (Hygiene)：为什么 Rust 宏是安全的

C/C++ 的宏 Bug 通常源于命名冲突：

```c
// C：非常危险 —— 宏内部的 `x` 可能会遮蔽调用者的 `x`
#define SQUARE(x) ((x) * (x))
int x = 5;
int result = SQUARE(x++);  // 未定义行为 (UB)：x 被递增了两次！
```

Rust 宏是**有卫生性 (Hygienic)** 的 —— 在宏内部创建的变量不会泄露到外部：

```rust
macro_rules! make_x {
    () => {
        let x = 42;  // 此处的 `x` 仅在宏展开的作用域内有效
    };
}

fn main() {
    let x = 10;
    make_x!();
    println!("{x}");  // 输出 10，而非 42 —— 卫生性防止了命名冲突
}
```

编译器会将宏内部的 `x` 与调用者的 `x` 视为不同的变量，即使它们的名称相同。**这在使用 C 预处理器时是不可能实现的。**

---

## 常用的标准库宏

自第 1 章开始你就一直在使用这些宏 —— 以下是它们的实际作用：

| 宏 | 作用 | 展开后（简化版） |
|-------|-------------|------------------------|
| `println!("{}", x)` | 格式化并打印至标准输出 + 换行 | `std::io::_print(format_args!(...))` |
| `eprintln!("{}", x)` | 打印至标准错误 + 换行 | 同上，但在标准错误中输出 |
| `format!("{}", x)` | 格式化并生成一个 `String` | 分配并返回一个 `String` |
| `vec![1, 2, 3]` | 创建一个包含指定元素的 `Vec` | `Vec::from([1, 2, 3])` (近似于此) |
| `todo!()` | 标记尚未完成的代码 | `panic!("尚未实现")` |
| `unimplemented!()` | 标记故意不予实现的代码 | `panic!("未实现")` |
| `unreachable!()` | 标记编译器无法证明不可达的代码 | `panic!("不可达")` |
| `assert!(cond)` | 当条件为 false 时触发 panic | `if !cond { panic!(...) }` |
| `assert_eq!(a, b)` | 当两者不相等时触发 panic | 在失败时显示这两个值 |
| `dbg!(expr)` | 将表达式及其值打印至 stderr 并返回该值 | `eprintln!("[文件:行号] 表达式 = {:#?}", &expr); expr` |
| `include_str!("file.txt")` | 在编译期将文件内容嵌入为 `&str` | 在编译期间读取该文件 |
| `include_bytes!("data.bin")` | 在编译期将文件内容嵌入为 `&[u8]` | 在编译期间读取该文件 |
| `cfg!(condition)` | 获取编译期的条件判定结果（作为 `bool` 值） | 根据目标环境返回 `true` 或 `false` |
| `env!("VAR")` | 在编译期读取环境变量 | 如果该变量未设置，则编译失败 |
| `concat!("a", "b")` | 在编译期拼接字面量 | `"ab"` |

### `dbg!` —— 你每天都会用到的调试宏

```rust
fn factorial(n: u32) -> u32 {
    if dbg!(n <= 1) {     // 输出：[src/main.rs:2] n <= 1 = false
        dbg!(1)           // 输出：[src/main.rs:3] 1 = 1
    } else {
        dbg!(n * factorial(n - 1))  // 输出中间计算过程的值
    }
}

fn main() {
    dbg!(factorial(4));   // 打印包含文件:行号在内的所有递归调用
}
```

`dbg!` 会返回它所包裹的值，因此你可以将其插入在任何位置而不影响程序原有的行为。它在标准错误 (stderr) 中输出（而非 stdout），因此不会干扰程序的输出结果。**在提交代码之前，请务必移除所有的 `dbg!` 调用。**

---

### 格式化字符串语法

由于 `println!`、`format!`、`eprintln!` 和 `write!` 都使用同一套格式化机制，这里提供一份快速参考指南：

```rust
let name = "传感器";
let value = 3.14159;
let count = 42;

println!("{name}");                    // 通过变量名引用 (Rust 1.58+)
println!("{}", name);                  // 位置参数
println!("{value:.2}");                // 保留两位小数："3.14"
println!("{count:>10}");               // 右对齐，宽度为 10："        42"
println!("{count:0>10}");              // 左侧补零："0000000042"
println!("{count:#06x}");              // 带前缀的十六进制："0x002a"
println!("{count:#010b}");             // 带前缀的二进制："0b00101010"
println!("{value:?}");                 // 调试格式 (Debug)
println!("{value:#?}");                // 易读的调试格式 (Pretty-printed Debug)
```

> **致 C 开发者：** 可以将其视为类型安全的 `printf` —— 编译器会检查 `{:.2}` 是否应用在了浮点数而非字符串上。绝不会出现 `%s` / `%d` 类型不匹配的 Bug。
>
> **致 C++ 开发者：** 这取代了 `std::cout << std::fixed << std::setprecision(2) << value` 这种写法，取而代之的是单一且易读的格式化字符串。

---

## 派生宏 (Derive Macros)

在本书的绝大多数结构体中你都能看到 `#[derive(...)]`：

```rust
#[derive(Debug, Clone, PartialEq)]
struct Point {
    x: f64,
    y: f64,
}
```

`#[derive(Debug)]` 是**派生宏** —— 一种特殊的过程宏，它能够自动生成 Trait 的实现代码。以下是它实际生成的代码（简化版）：

```rust
// #[derive(Debug)] 为 Point 结构体生成的代码：
impl std::fmt::Debug for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Point")
            .field("x", &self.x)
            .field("y", &self.y)
            .finish()
    }
}
```

如果没有 `#[derive(Debug)]`，你就必须为每一个结构体手动编写这样的 `impl` 块。

### 常用的派生 Trait

| 派生项 | 生成的代码作用 | 何时使用 |
|--------|-------------------|-------------|
| `Debug` | `{:?}` 格式化支持 | 几乎总是使用 —— 开启调试打印支持 |
| `Clone` | `.clone()` 方法 | 当你需要复制值时 |
| `Copy` | 赋值时的隐式复制 | 小型、仅限栈存储的类型（整数、`[f64; 3]`） |
| `PartialEq` / `Eq` | `==` 与 `!=` 运算符 | 当你需要进行相等性比较时 |
| `PartialOrd` / `Ord` | `<`, `>`, `<=`, `>=` 运算符 | 当你需要对值进行排序时 |
| `Hash` | 供 `HashMap`/`HashSet` 使用的哈希值 | 用作 Map Key 的类型 |
| `Default` | `Type::default()` 构造器 | 具有合理的零值或空值的类型 |
| `serde::Serialize` / `Deserialize` | JSON/TOML 等序列化支持 | 跨 API 边界传输的数据类型 |

### 派生决策树

```text
我应该通过派生来实现它吗？
  │
  ├── 我的类型是否仅包含实现了该 Trait 的子类型？
  │     ├── 是 → #[derive] 将能正常工作
  │     └── 否 → 手动编写 impl（或者跳过它）
  │
  └── 用户是否会理所当然地预期该类型具备这种行为？
        ├── 是 → 进行派生 (Debug, Clone, PartialEq 几乎总是合理的)
        └── 否 → 不要派生（例如，不要为一个包含文件句柄的类型派生 Copy）
```

> **C++ 对比：** `#[derive(Clone)]` 类似于自动生成一个正确的拷贝构造函数。`#[derive(PartialEq)]` 类似于自动生成一个对所有字段进行比较的 `operator==` —— 这在 C++20 中由 `= default` 的太空船运算符 (Spaceship Operator) 最终提供。

---

## 属性宏 (Attribute Macros)

属性宏会对它们所附加的项进行转换向。你已经使用过其中的好几个了：

```rust
#[test]                    // 将一个函数标记为测试函数
fn test_addition() {
    assert_eq!(2 + 2, 4);
}

#[cfg(target_os = "linux")] // 有条件地包含此函数
fn linux_only() { /* ... */ }

#[derive(Debug)]            // 自动生成 Debug 实现
struct MyType { /* ... */ }

#[allow(dead_code)]         // 抑制编译器警告
fn unused_helper() { /* ... */ }

#[must_use]                 // 如果返回值被丟弃，则发出警告
fn compute_checksum(data: &[u8]) -> u32 { /* ... */ }
```

常见的内置属性：

| 属性 | 用途 |
|-----------|---------|
| `#[test]` | 标记为测试函数 |
| `#[cfg(...)]` | 条件编译 |
| `#[derive(...)]` | 自动生成 Trait 实现 |
| `#[allow(...)]` / `#[deny(...)]` / `#[warn(...)]` | 控制 Lint 级别 |
| `#[must_use]` | 对未使用的返回值发出警告 |
| `#[inline]` / `#[inline(always)]` | 提示编译器内联该函数 |
| `#[repr(C)]` | 使用 C 兼容的内存布局 (用于 FFI) |
| `#[no_mangle]` | 不要混淆符号名称 (用于 FFI) |
| `#[deprecated]` | 标记为已弃用（可附带可选的消息） |

> **针对 C/C++ 开发者：** 属性宏取代了预处理指令 (`#pragma`、`__attribute__((...))`) 以及特定于编译器的扩展。它们是语言语法的一部分，而非强行挂载的扩展。

---

## 过程宏 (Procedural Macros) (概念概览)

过程宏 ("Proc macros") 是作为**独立的 Rust 程序**编写的。它们在编译期间运行并生成代码。它们比 `macro_rules!` 更加强大，但实现起来也更为复杂。

过程宏共有三种类型：

| 类型 | 语法形式 | 示例 | 作用 |
|------|--------|---------|-------------|
| **函数式宏** | `my_macro!(...)` | `sql!(SELECT * FROM users)` | 解析自定义语法，生成 Rust 代码 |
| **派生宏** | `#[derive(MyTrait)]` | `#[derive(Serialize)]` | 根据结构体定义生成 Trait 实现 |
| **属性宏** | `#[my_attr]` | `#[tokio::main]`, `#[instrument]` | 对所修饰的项进行转换 |

### 你已经使用过过程宏了

- 来自 `thiserror` 的 `#[derive(Error)]` —— 为错误枚举生成 `Display` 与 `From` 的实现。
- 来自 `serde` 的 `#[derive(Serialize, Deserialize)]` —— 生成序列化/反序列化代码。
- `#[tokio::main]` —— 将 `async fn main()` 转换为运行时的设置代码。
- `#[test]` —— 由测试框架注册的内置过程宏。

### 何时编写你自己的过程宏

在学习本课程期间，你可能并不需要编写过程宏。只有在下列场景中它们才会派上用场：
- 你需要在编译期检查结构体字段/枚举变体（派生宏）。
- 你正在构建一种领域特定语言 (DSL)（函数式宏）。
- 你需要转换函数的签名（属性宏）。

对于大部分代码来说，使用 `macro_rules!` 或普通函数就已经足够了。

> **C++ 对比：** 过程宏填补了 C++ 中代码生成器、模板元编程以及像 `protoc` 之类的外部工具所扮演的角色。不同之处在于，过程宏是 Cargo 构建流水线的一部分 —— 既不需要外部构建步骤，也不需要 CMake 的自定义命令。

---

## 何时该使用哪种方案：宏 vs 函数 vs 泛型

```text
需要生成代码吗？
  │
  ├── 否 → 使用普通函数或泛型函数
  │         (更简单，拥有更佳的错误提示与 IDE 支持)
  │
  └── 是 ─┬── 参数数量可变吗？
            │     └── 是 → 使用 macro_rules! (例如 println!、vec!)
            │
            ├── 需要为多个类型生成重复的 impl 块吗？
            │     └── 是 → 使用带有重复机制的 macro_rules!
            │
            ├── 需要检查结构体字段吗？
            │     └── 是 → 派生宏 (过程宏)
            │
            ├── 需要自定义语法 (DSL) 吗？
            │     └── 是 → 函数式过程宏
            │
            └── 需要转换一个函数/结构体吗？
                  └── 是 → 属性过程宏
```

**通用准则：** 如果普通函数或泛型能够胜任，就不要使用宏。宏的错误提示较差，且在宏体内部缺乏 IDE 的自动补全支持，调试起来也更为困难。

---

## 练习

### 🟢 练习 1：`min!` 宏

编写一个 `min!` 宏，要求：
- `min!(a, b)` 返回两个值中较小的一个。
- `min!(a, b, c)` 返回三个值中最小的一个。
- 适用于任何实现了 `PartialOrd` 的类型。

**提示：** 你需要在 `macro_rules!` 中编写两个匹配分支。

<details><summary>解决方案 (点击展开)</summary>

```rust
macro_rules! min {
    ($a:expr, $b:expr) => {
        if $a < $b { $a } else { $b }
    };
    ($a:expr, $b:expr, $c:expr) => {
        min!(min!($a, $b), $c)
    };
}

fn main() {
    println!("{}", min!(3, 7));        // 3
    println!("{}", min!(9, 2, 5));     // 2
    println!("{}", min!(1.5, 0.3));    // 0.3
}
```

**注意：** 对于生产环境的代码，应当优先使用 `std::cmp::min` 或 `a.min(b)`。此练习仅用于通过演示多分支宏的机制。

</details>

---

### 🟡 练习 2：从零开始编写 `hashmap!`

在不查看上方示例的前提下，尝试编写一个 `hashmap!` 宏，要求：
- 利用 `key => value` 键值对创建一个 `HashMap`。
- 支持尾随逗号。
- 适用于任何可哈希的键类型。

使用以下代码进行测试：
```rust
let m = hashmap! {
    "name" => "Alice",
    "role" => "工程师",
};
assert_eq!(m["name"], "Alice");
assert_eq!(m.len(), 2);
```

<details><summary>解决方案 (点击展开)</summary>

```rust
use std::collections::HashMap;

macro_rules! hashmap {
    ( $( $key:expr => $val:expr ),* $(,)? ) => {{
        let mut map = HashMap::new();
        $( map.insert($key, $val); )*
        map
    }};
}

fn main() {
    let m = hashmap! {
        "name" => "Alice",
        "role" => "工程师",
    };
    assert_eq!(m["name"], "Alice");
    assert_eq!(m.len(), 2);
    println!("测试通过！");
}
```

</details>

---

### 🟡 练习 3：用于浮点数比较的 `assert_approx_eq!`

编写一个 `assert_approx_eq!(a, b, epsilon)` 宏，如果 `|a - b| > epsilon` 则触发 panic。在精确相等判断失效的浮点数计算测试中，这个宏非常有用。

使用以下代码进行测试：
```rust
assert_approx_eq!(0.1 + 0.2, 0.3, 1e-10);        // 应该通过
assert_approx_eq!(3.14159, std::f64::consts::PI, 1e-4); // 应该通过
// assert_approx_eq!(1.0, 2.0, 0.5);              // 应该触发 panic
```

<details><summary>解决方案 (点击展开)</summary>

```rust
macro_rules! assert_approx_eq {
    ($a:expr, $b:expr, $eps:expr) => {
        let (a, b, eps) = ($a as f64, $b as f64, $eps as f64);
        let diff = (a - b).abs();
        if diff > eps {
            panic!(
                "断言失败：|{} - {}| = {} > {} (epsilon)",
                a, b, diff, eps
            );
        }
    };
}

fn main() {
    assert_approx_eq!(0.1 + 0.2, 0.3, 1e-10);
    assert_approx_eq!(3.14159, std::f64::consts::PI, 1e-4);
    println!("所有浮点数比较均已通过！");
}
```

</details>

---

### 🔴 练习 4：`impl_display_for_enum!`

编写一个宏，为简单的类 C 枚举生成 `Display` Trait 的实现。已知：

```rust
impl_display_for_enum! {
    enum Color {
        Red => "红色",
        Green => "绿色",
        Blue => "蓝色",
    }
}
```

该宏应当同时生成 `enum Color { Red, Green, Blue }` 的定义，以及将每个变体映射到其对应字符串的 `impl Display for Color` 实现。

**提示：** 你需要同时使用 `$( ... ),*` 重复机制和多个片段说明符。

<details><summary>解决方案 (点击展开)</summary>

```rust
use std::fmt;

macro_rules! impl_display_for_enum {
    (enum $name:ident { $( $variant:ident => $display:expr ),* $(,)? }) => {
        #[derive(Debug, Clone, Copy, PartialEq)]
        enum $name {
            $( $variant ),*
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                match self {
                    $( $name::$variant => write!(f, "{}", $display), )*
                }
            }
        }
    };
}

impl_display_for_enum! {
    enum Color {
        Red => "红色",
        Green => "绿色",
        Blue => "蓝色",
    }
}

fn main() {
    let c = Color::Green;
    println!("颜色: {c}");          // "颜色: 绿色"
    println!("调试打印: {c:?}");    // "调试打印: Green"
    assert_eq!(format!("{}", Color::Red), "红色");
    println!("所有测试已通过！");
}
```

</details>

---
