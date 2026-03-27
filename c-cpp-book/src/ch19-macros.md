## Rust Macros: From Preprocessor to Metaprogramming / Rust 宏：从预处理器到元编程
 
 > **What you'll learn / 你将学到：** How Rust macros work, when to use them instead of functions or generics, and how they replace the C/C++ preprocessor. By the end of this chapter you can write your own `macro_rules!` macros and understand what `#[derive(Debug)]` does under the hood.
 >
 > Rust 宏的工作原理、何时使用它们而非函数或泛型，以及它们如何替代 C/C++ 预处理器。在本章结束时，你将能编写自己的 `macro_rules!` 宏，并理解 `#[derive(Debug)]` 底层的运作机制。
 
- Macros are one of the first things you encounter in Rust (`println!("hello")` on line one) but one of the last things most courses explain. This chapter fixes that.
+ 宏是你最早接触到的 Rust 特性之一（第一行代码通常就是 `println!("hello")`），但却是大多数课程最后才解释的内容。本章将填补这一空白。
 
- ### Why Macros Exist
+ ### Why Macros Exist / 为什么需要宏
 
- Functions and generics handle most code reuse in Rust. Macros fill the gaps where the type system can't reach:
+ 在 Rust 中，函数和泛型处理了大部分代码复用。宏则填补了类型系统无法触及的空白：
 
-| Need | Function/Generic? | Macro? | Why |
+| **Need / 需求** | **Function/Generic? / 函数/泛型？** | **Macro? / 宏？** | **Why / 原因** |
 |------|-------------------|--------|-----|
-| Compute a value | ✅ `fn max<T: Ord>(a: T, b: T) -> T` | — | Type system handles it |
-| Compute value / 计算值 | ✅ `fn max<T: Ord>(...)` | — | 类型系统足以处理 |
-| Accept variable number of arguments | ❌ Rust has no variadic functions | ✅ `println!("{} {}", a, b)` | Macros accept any number of tokens |
-| Variadic args / 变长参数 | ❌ Rust 无变长参数函数 | ✅ `println!` | 宏可以接收任意数量的 Token |
-| Generate repetitive `impl` blocks | ❌ No way with generics alone | ✅ `macro_rules!` | Macros generate code at compile time |
-| 重复的 `impl` 块 | ❌ 仅靠泛型无法实现 | ✅ `macro_rules!` | 宏在编译时生成代码 |
-| Run code at compile time | ❌ `const fn` is limited | ✅ Procedural macros | Full Rust code runs at compile time |
-| 编译时执行代码 | ❌ `const fn` 限制较多 | ✅ 过程宏 | 完整的 Rust 代码可在编译时运行 |
-| Conditionally include code | ❌ | ✅ `#[cfg(...)]` | Attribute macros control compilation |
-| 条件包含代码 | ❌ | ✅ `cfg` 属性 | 属性宏控制编译过程 |
 
- If you're coming from C/C++, think of macros as the *only correct replacement for the preprocessor* — except they operate on the syntax tree instead of raw text, so they're hygienic (no accidental name collisions) and type-aware.
+ 如果你来自 C/C++ 领域，可以将宏视为*预处理器的唯一正确替代方案* —— 不同之处在于它们操作的是语法树而非原始文本，因此它们是“卫生”的（不会发生意外的名称冲突）且能感知类型。
 
- > **For C developers:** Rust macros replace `#define` entirely. There is no textual preprocessor. See [ch18](ch18-cpp-rust-semantic-deep-dives.md) for the full preprocessor → Rust mapping.
+ > **致 C 开发者**：Rust 宏完全替代了 `#define`。Rust 中没有文本预处理器。有关预处理器到 Rust 的完整映射，请参阅第 [18](ch18-cpp-rust-semantic-deep-dives.md) 章。
 
 ---
 
- ## Declarative Macros with `macro_rules!`
+ ## Declarative Macros with `macro_rules!` / 使用 `macro_rules!` 的声明式宏
 
- Declarative macros (also called "macros by example") are Rust's most common macro form. They use pattern matching on syntax, similar to `match` on values.
+ 声明式宏（也称为“示例宏”）是 Rust 中最常见的宏形式。它们在语法上使用模式匹配，类似于对值进行的 `match` 操作。
 
- ### Basic syntax
+ ### Basic syntax / 基础语法
 
 ```rust
 macro_rules! say_hello {
     () => {
-        println!("Hello!");
+        println!("Hello!"); // 打印
     };
 }
 
 fn main() {
-    say_hello!();  // Expands to: println!("Hello!");
+    say_hello!();  // Expands / 展开为：println!("Hello!");
 }
 ```
 
- The `!` after the name is what tells you (and the compiler) this is a macro invocation.
+ 名称后的 `!` 告诉开发者（以及编译器）这是一个宏调用。
 
- ### Pattern matching with arguments
+ ### Pattern matching with arguments / 带参数的模式匹配
 
- Macros match on *token trees* using fragment specifiers:
+ 宏使用片段指示符（Fragment Specifiers）对“Token 树”进行匹配：
 
 ```rust
 macro_rules! greet {
-    // Pattern 1: no arguments
+    // Pattern 1 / 模式 1：无参数
     () => {
         println!("Hello, world!");
     };
-    // Pattern 2: one expression argument
+    // Pattern 2 / 模式 2：一个表达式参数
     ($name:expr) => {
         println!("Hello, {}!", $name);
     };
 }
 
 fn main() {
     greet!();           // "Hello, world!"
     greet!("Rust");     // "Hello, Rust!"
 }
 ```
 
- #### Fragment specifiers reference
+ #### Fragment specifiers reference / 片段指示符参考表
 
-| Specifier | Matches | Example |
+| **Specifier / 指示符** | **Matches / 匹配项** | **Example / 示例** |
 |-----------|---------|---------|
-| `$x:expr` | Any expression | `42`, `a + b`, `foo()` |
-| `$x:expr` | 任意表达式 | `42`, `a + b` |
-| `$x:ty` | A type | `i32`, `Vec<String>`, `&str` |
-| `$x:ty` | 类型 | `i32`, `&str` |
-| `$x:ident` | An identifier | `foo`, `my_var` |
-| `$x:ident` | 标识符 | `foo`, `my_var` |
-| `$x:pat` | A pattern | `Some(x)`, `_`, `(a, b)` |
-| `$x:pat` | 模式 | `Some(x)`, `_` |
-| `$x:stmt` | A statement | `let x = 5;` |
-| `$x:stmt` | 语句 | `let x = 5;` |
-| `$x:block` | A block | `{ println!("hi"); 42 }` |
-| `$x:block` | 代码块 | `{ ... }` |
-| `$x:literal` | A literal | `42`, `"hello"`, `true` |
-| `$x:literal` | 字面量 | `42`, `"hello"` |
-| `$x:tt` | A single token tree | Anything — the wildcard |
-| `$x:tt` | 单个 Token 树 | 任意内容 —— 通配符 |
-| `$x:item` | An item (fn, struct, impl, etc.) | `fn foo() {}` |
-| `$x:item` | 项 (函数、结构体等) | `fn foo() {}` |
 
- ### Repetition — the killer feature
+ ### Repetition — the killer feature / 重复 —— 宏的核心杀手锏
 
- C/C++ macros can't loop. Rust macros can repeat patterns:
+ C/C++ 宏无法循环。Rust 宏则可以重复模式：
 
 ```rust
 macro_rules! make_vec {
-    // Match zero or more comma-separated expressions
+    // Match zero / 匹配零个或多个逗号分隔的表达式
     ( $( $element:expr ),* ) => {
         {
             let mut v = Vec::new();
-            $( v.push($element); )*  // Repeat for each matched element
+            $( v.push($element); )*  // Repeat / 为每个匹配的元素重复执行 push
             v
         }
     };
 }
 
 fn main() {
     let v = make_vec![1, 2, 3, 4, 5];
     println!("{v:?}");  // [1, 2, 3, 4, 5]
 }
 ```
 
- The `$( ... ),*` syntax means "match zero or more of this pattern, separated by commas." The `$( ... )*` in the expansion repeats the body once for each match.
+ `$( ... ),*` 语法表示“匹配零个或多个此模式，以逗号分隔”。展开过程中的 `$( ... )*` 则会为每个匹配项重复执行其内部的代码体。
 
- > **This is exactly how `vec![]` is implemented in the standard library.** The actual source is:
+ > **标准库中的 `vec![]` 正是以这种方式实现的**。其源代码大致如下：
- > ```rust
- > macro_rules! vec {
- >     () => { Vec::new() };
- >     ($elem:expr; $n:expr) => { vec::from_elem($elem, $n) };
- >     ($($x:expr),+ $(,)?) => { <[_]>::into_vec(Box::new([$($x),+])) };
- > }
- > ```
+ > ```rust
+ > macro_rules! vec {
+ >     () => { Vec::new() };
+ >     ($elem:expr; $n:expr) => { vec::from_elem($elem, $n) };
+ >     ($($x:expr),+ $(,)?) => { <[_]>::into_vec(Box::new([$($x),+])) };
+ > }
+ > ```
- > The `$(,)?` at the end allows an optional trailing comma.
+ > 末尾的 `$(,)?` 允许可选的尾随逗号。
 
- #### Repetition operators
+ #### Repetition operators / 重复操作符
 
-| Operator | Meaning | Example |
+| **Operator / 操作符** | **Meaning / 含义** | **Example / 示例** |
 |----------|---------|---------|
-| `$( ... )*` | Zero or more | `vec![]`, `vec![1]`, `vec![1, 2, 3]` |
-| `$( ... )*` | 零个或多个 | `vec![]`, `vec![1, 2]` |
-| `$( ... )+` | One or more | At least one element required |
-| `$( ... )+` | 一个或多个 | 至少需要一个元素 |
-| `$( ... )?` | Zero or one | Optional element |
-| `$( ... )?` | 零个或一个 | 可选元素 |
 
- ### Practical example: a `hashmap!` constructor
+ ### Practical example: a `hashmap!` constructor / 实用示例：`hashmap!` 构造器
 
- The standard library has `vec![]` but no `hashmap!{}`. Let's build one:
+ 标准库提供了 `vec![]` 但没有提供 `hashmap!{}`。让我们来构建一个：
 
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
-        "Carol" => 92,  // trailing comma OK thanks to $(,)?
+        "Carol" => 92,  // Trailing comma / 多亏了 $(,)?，尾随逗号也是允许的
     };
     println!("{scores:?}");
 }
 ```
 
- ### Practical example: diagnostic check macro
+ ### Practical example: diagnostic check macro / 实用示例：诊断检查宏
 
- A pattern common in embedded/diagnostic code — check a condition and return an error:
+ 嵌入式/诊断代码中的常见模式 —— 检查一个条件并返回错误：
 
 ```rust
 use thiserror::Error;
 
 #[derive(Error, Debug)]
 enum DiagError {
     #[error("Check failed: {0}")]
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
     diag_check!(temp < 85.0, "GPU too hot");
     diag_check!(voltage > 0.8, "Rail voltage too low");
     diag_check!(voltage < 1.5, "Rail voltage too high");
     println!("All checks passed");
     Ok(())
 }
 ```
 
- > **C/C++ comparison:**
+ > **C/C++ 对比：**
- > ```c
- > // C preprocessor — textual substitution, no type safety, no hygiene
- > #define DIAG_CHECK(cond, msg) \
- >     do { if (!(cond)) { log_error(msg); return -1; } } while(0)
- > ```
+ > ```c
+ > // C 预处理器 — 文本替换，无类型安全，不卫生
+ > #define DIAG_CHECK(cond, msg) \
+ >     do { if (!(cond)) { log_error(msg); return -1; } } while(0)
+ > ```
- > The Rust version returns a proper `Result` type, has no double-evaluation risk, and the compiler checks that `$cond` is actually a `bool` expression.
+ > Rust 版本返回正规的 `Result` 类型，没有重复求值的风险，而且编译器会检查 `$cond` 是否确实是 `bool` 表达式。
 
- ### Hygiene: why Rust macros are safe
+ ### Hygiene: why Rust macros are safe / 卫生性：为什么 Rust 宏是安全的
 
- C/C++ macro bugs often come from name collisions:
+ C/C++ 宏的 Bug 通常源于名称冲突：
 
 ```c
- // C: dangerous — `x` could shadow the caller's `x`
+ // C: 危险 — `x` 可能会遮蔽调用者的 `x`
 #define SQUARE(x) ((x) * (x))
 int x = 5;
- int result = SQUARE(x++);  // UB: x incremented twice!
+ int result = SQUARE(x++);  // UB / 未定义行为：x 被递增了两次！
 ```
 
- Rust macros are **hygienic** — variables created inside a macro don't leak out:
+ Rust 宏是**卫生（Hygienic）**的 —— 在宏内部创建的变量不会泄露到外部：
 
 ```rust
 macro_rules! make_x {
     () => {
-        let x = 42;  // This `x` is scoped to the macro expansion
+        let x = 42;  // 此 `x` 的作用域仅限于宏展开内部
     };
 }
 
 fn main() {
     let x = 10;
     make_x!();
-    println!("{x}");  // Prints 10, not 42 — hygiene prevents collision
+    println!("{x}");  // Prints / 打印 10，而非 42 —— 卫生性防止了冲突
 }
 ```
 
- The macro's `x` and the caller's `x` are treated as different variables by the compiler, even though they have the same name. **This is impossible with the C preprocessor.**
+ 编译器将宏内部的 `x` 和调用者的 `x` 视为不同的变量，即使它们同名。**这在 C 预处理器中是不可能实现的。**
 
 ---
 
- ## Common Standard Library Macros
+ ## Common Standard Library Macros / 常用标准库宏
 
- You've been using these since chapter 1 — here's what they actually do:
+ 自第 1 章起你就一直在使用这些宏 —— 以下是它们的实际作用：
 
-| Macro | What it does | Expands to (simplified) |
+| **Macro / 宏** | **What it does / 作用** | **Expands to / 展开为 (简化版)** |
 |-------|-------------|------------------------|
-| `println!("{}", x)` | Format and print to stdout + newline | `std::io::_print(format_args!(...))` |
-| `println!` | 格式化并打印到 stdout | `io::_print(...)` |
-| `eprintln!("{}", x)` | Print to stderr + newline | Same but to stderr |
-| `eprintln!` | 打印到 stderr | 打印到 stderr |
-| `format!("{}", x)` | Format into a `String` | Allocates and returns a `String` |
-| `format!` | 格式化为 `String` | 分配并返回 `String` |
-| `vec![1, 2, 3]` | Create a `Vec` with elements | `Vec::from([1, 2, 3])` (approximately) |
-| `vec!` | 创建带初始值的 `Vec` | `Vec::from(...)` |
-| `todo!()` | Mark unfinished code | `panic!("not yet implemented")` |
-| `todo!` | 标记未完成的代码 | `panic!` 带有待办消息 |
-| `unimplemented!()` | Mark deliberately unimplemented code | `panic!("not implemented")` |
-| `unreachable!()` | Mark code the compiler can't prove unreachable | `panic!("unreachable")` |
-| `assert!(cond)` | Panic if condition is false | `if !cond { panic!(...) }` |
-| `assert_eq!(a, b)` | Panic if values aren't equal | Shows both values on failure |
-| `dbg!(expr)` | Print expression + value to stderr, return value | `eprintln!("[file:line] expr = {:#?}", &expr); expr` |
-| `dbg!` | 打印表达式及值，并返回该值 | `eprintln!` + 值的转发 |
-| `include_str!("file.txt")` | Embed file contents as `&str` at compile time | Reads file during compilation |
-| `include_str!` | 编译时以 `&str` 嵌入文件内容 | 编译阶段读取文件 |
-| `include_bytes!("data.bin")` | Embed file contents as `&[u8]` at compile time | Reads file during compilation |
-| `cfg!(condition)` | Compile-time condition as a `bool` | `true` or `false` based on target |
-| `env!("VAR")` | Read environment variable at compile time | Fails compilation if not set |
-| `env!` | 编译时读取环境变量 | 失败则导致编译报错 |
-| `concat!("a", "b")` | Concatenate literals at compile time | `"ab"` |
 
- ### `dbg!` — the debugging macro you'll use daily
+ ### `dbg!` — the debugging macro you'll use daily / `dbg!` —— 你每天都会用到的调试宏
 
 ```rust
 fn factorial(n: u32) -> u32 {
-    if dbg!(n <= 1) {     // Prints: [src/main.rs:2] n <= 1 = false
+    if dbg!(n <= 1) {     // Prints / 打印：[src/main.rs:2] n <= 1 = false
-        dbg!(1)           // Prints: [src/main.rs:3] 1 = 1
+        dbg!(1)           // Prints / 打印：[src/main.rs:3] 1 = 1
     } else {
-        dbg!(n * factorial(n - 1))  // Prints intermediate values
+        dbg!(n * factorial(n - 1))  // Prints / 打印中间值
     }
 }
 
 fn main() {
-    dbg!(factorial(4));   // Prints all recursive calls with file:line
+    dbg!(factorial(4));   // Prints / 打印所有带文件名和行号的递归调用
 }
 ```
 
- `dbg!` returns the value it wraps, so you can insert it anywhere without changing program behavior. It prints to stderr (not stdout), so it doesn't interfere with program output. **Remove all `dbg!` calls before committing code.**
+ `dbg!` 会返回它包裹的值，因此你可以将其插入任何地方而不影响程序行为。它打印到 stderr（而非 stdout），因此不会干扰程序输出。**在提交代码前，请删除所有的 `dbg!` 调用。**
 
- ### Format string syntax
+ ### Format string syntax / 格式化字符串语法
 
- Since `println!`, `format!`, `eprintln!`, and `write!` all use the same format machinery, here's the quick reference:
+ 由于 `println!`、`format!`、`eprintln!` 和 `write!` 通用一套格式化机制，以下是快速参考：
 
 ```rust
 let name = "sensor";
 let value = 3.14159;
 let count = 42;
 
- println!("{name}");                    // Variable by name (Rust 1.58+)
+ println!("{name}");                    // Variable / 按名称引用变量 (Rust 1.58+)
- println!("{}", name);                  // Positional
+ println!("{}", name);                  // Positional / 位置匹配
- println!("{value:.2}");                // 2 decimal places: "3.14"
+ println!("{value:.2}");                // 2 dec / 2 位小数："3.14"
- println!("{count:>10}");               // Right-aligned, width 10: "        42"
+ println!("{count:>10}");               // Align / 右对齐，宽度 10："        42"
- println!("{count:0>10}");              // Zero-padded: "0000000042"
+ println!("{count:0>10}");              // Padding / 零填充："0000000042"
- println!("{count:#06x}");              // Hex with prefix: "0x002a"
+ println!("{count:#06x}");              // Hex / 带前缀的十六进制："0x002a"
- println!("{count:#010b}");             // Binary with prefix: "0b00101010"
+ println!("{count:#010b}");             // Bin / 带前缀的二进制："0b00101010"
- println!("{value:?}");                 // Debug format
+ println!("{value:?}");                 // Debug / Debug 格式化
- println!("{value:#?}");                // Pretty-printed Debug format
+ println!("{value:#?}");                // Pretty / 美化后的 Debug 格式化
 ```
 
- > **For C developers:** Think of this as a type-safe `printf` — the compiler checks that `{:.2}` is applied to a float, not a string. No `%s`/`%d` format mismatch bugs.
+ > **致 C 开发者**：将其视为类型安全的 `printf` —— 编译器会检查 `{:.2}` 是否应用于浮点数而非字符串。不会再有 `%s`/`%d` 格式不匹配的 Bug。
- >
- > **For C++ developers:** This replaces `std::cout << std::fixed << std::setprecision(2) << value` with a single readable format string.
+ >
+ > **致 C++ 开发者**：这用单个可读的格式化字符串替代了繁杂的 `std::cout << std::fixed << ... << value` 调用。
 
 ---
 
- ## Derive Macros
+ ## Derive Macros / 派生宏 (Derive Macros)
 
- You've seen `#[derive(...)]` on nearly every struct in this book:
+ 在本书的几乎每个结构体上你都见过 `#[derive(...)]`：
 
 ```rust
 #[derive(Debug, Clone, PartialEq)]
 struct Point {
     x: f64,
     y: f64,
 }
 ```
 
- `#[derive(Debug)]` is a **derive macro** — a special kind of procedural macro that generates trait implementations automatically. Here's what it produces (simplified):
+ `#[derive(Debug)]` 是一个**派生宏** —— 一种特殊的过程宏，可以自动生成 Trait 实现。以下是它生成的代码（简化版）：
 
 ```rust
- // What #[derive(Debug)] generates for Point:
+ // #[derive(Debug)] 为 Point 自动生成的代码：
 impl std::fmt::Debug for Point {
     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
         f.debug_struct("Point")
             .field("x", &self.x)
             .field("y", &self.y)
             .finish()
     }
 }
 ```
 
- Without `#[derive(Debug)]`, you'd have to write that `impl` block by hand for every struct.
+ 如果没有 `#[derive(Debug)]`，你就必须为每个结构体手动编写这个 `impl` 块。
 
- ### Commonly derived traits
+ ### Commonly derived traits / 常用的派生 Trait
 
-| Derive | What it generates | When to use |
+| **Derive / 派生** | **What it generates / 生成内容** | **When to use / 场景** |
 |--------|-------------------|-------------|
-| `Debug` | `{:?}` formatting | Almost always — enables printing for debugging |
-| `Debug` | `{:?}` 格式化 | 几乎所有场景 —— 启用调试打印 |
-| `Clone` | `.clone()` method | When you need to duplicate values |
-| `Clone` | `.clone()` 方法 | 需要复制值时 |
-| `Copy` | Implicit copy on assignment | Small, stack-only types (integers, `[f64; 3]`) |
-| `Copy` | 隐式赋值拷贝 | 小型、仅栈类型 (整数、`[f64; 3]`) |
-| `PartialEq` / `Eq` | `==` and `!=` operators | When you need equality comparison |
-| `Eq` | `==` 和 `!=` 运算符 | 需要相等性比较时 |
-| `PartialOrd` / `Ord` | `<`, `>`, `<=`, `>=` operators | When you need ordering |
-| `Ord` | 比较运算符 | 需要排序时 |
-| `Hash` | Hashing for `HashMap`/`HashSet` keys | Types used as map keys |
-| `Hash` | 哈希计算 | 用作 Map 的键时 |
-| `Default` | `Type::default()` constructor | Types with sensible zero/empty values |
-| `Default` | 默认值构造器 | 具有合理的“零值”或空值的类型 |
-| `serde::Serialize` / `Deserialize` | JSON/TOML/etc. serialization | Data types that cross API boundaries |
-| `serde` | 序列化/反序列化 | 涉及 API 边界的数据类型 |
 
- ### The derive decision tree
+ ### The derive decision tree / 派生决策树
 
 ```text
- Should I derive it?
+ 我该派生它吗？
   │
-   ├── Does my type contain only types that implement the trait?
+   ├── 所有的成员类型是否都实现了该 Trait？
-   │     ├── Yes → #[derive] will work
+   │     ├── 是 → #[derive] 可以正常工作
-   │     └── No  → Write a manual impl (or skip it)
+   │     └── 否 → 需要手动编写 impl（或者放弃实现）
   │
-   └── Will users of my type reasonably expect this behavior?
+   └── 用户是否会理所当然地期望这种行为？
-         ├── Yes → Derive it (Debug, Clone, PartialEq are almost always reasonable)
+         ├── 是 → 派生它（Debug, Clone, PartialEq 几乎总是合理的）
-         └── No  → Don't derive (e.g., don't derive Copy for a type with a file handle)
+         └── 否 → 不要派生（例如，不要为包含文件句柄的类型派生 Copy）
 ```
 
- > **C++ comparison:** `#[derive(Clone)]` is like auto-generating a correct copy constructor. `#[derive(PartialEq)]` is like auto-generating `operator==` that compares each field — something C++20's `= default` spaceship operator finally provides.
+ > **C++ 对比**：`#[derive(Clone)]` 类似于自动生成正确的拷贝构造函数。`#[derive(PartialEq)]` 类似于自动生成按字段比较的 `operator==` —— 这正是 C++20 的 `= default` 飞船操作符最终提供的功能。
 
 ---
 
- ## Attribute Macros
+ ## Attribute Macros / 属性宏
 
- Attribute macros transform the item they're attached to. You've already used several:
+ 属性宏会转换它们所附加的项。你已经使用过好几个了：
 
 ```rust
- #[test]                    // Marks a function as a test
+ #[test]                    // Marks / 将函数标记为测试
 fn test_addition() {
     assert_eq!(2 + 2, 4);
 }
 
- #[cfg(target_os = "linux")] // Conditionally includes this function
+ #[cfg(target_os = "linux")] // Conditional / 条件包含该函数
 fn linux_only() { /* ... */ }
 
- #[derive(Debug)]            // Generates Debug implementation
+ #[derive(Debug)]            // Generates / 生成 Debug 实现
 struct MyType { /* ... */ }
 
- #[allow(dead_code)]         // Suppresses a compiler warning
+ #[allow(dead_code)]         // Suppress / 抑制编译器警告
 fn unused_helper() { /* ... */ }
 
- #[must_use]                 // Warn if return value is discarded
+ #[must_use]                 // Warn / 如果忽略返回值则发出警告
 fn compute_checksum(data: &[u8]) -> u32 { /* ... */ }
 ```
 
- Common built-in attributes:
+ 常见的内置属性：
 
-| Attribute | Purpose |
+| **Attribute / 属性** | **Purpose / 用途** |
 |-----------|---------|
-| `#[test]` | Mark as test function |
-| `#[test]` | 标记为测试函数 |
-| `#[cfg(...)]` | Conditional compilation |
-| `#[cfg(...)]` | 条件编译 |
-| `#[derive(...)]` | Auto-generate trait impls |
-| `#[derive(...)]` | 自动生成 Trait 实现 |
-| `#[allow(...)]` / `#[deny(...)]` / `#[warn(...)]` | Control lint levels |
-| `Lint control` | 控制代码检查级别 |
-| `#[must_use]` | Warn on unused return values |
-| `#[must_use]` | 警告未使用的返回值 |
-| `#[inline]` / `#[inline(always)]` | Hint to inline the function |
-| `#[inline]` | 提示函数内联 |
-| `#[repr(C)]` | Use C-compatible memory layout (for FFI) |
-| `#[repr(C)]` | 使用 C 兼容的内存布局 (用于 FFI) |
-| `#[no_mangle]` | Don't mangle the symbol name (for FFI) |
-| `#[no_mangle]` | 禁止符号修饰 (用于 FFI) |
-| `#[deprecated]` | Mark as deprecated with optional message |
-| `#[deprecated]` | 标记为已废弃 |
 
- > **For C/C++ developers:** Attributes replace a mix of preprocessor directives (`#pragma`, `__attribute__((...))`), and compiler-specific extensions. They're part of the language grammar, not bolted-on extensions.
+ > **致 C/C++ 开发者**：属性替代了预处理器指令（`#pragma`, `__attribute__((...))`）和编译器特定扩展。它们是语言语法的一部分，而不是额外的修补插件。
 
 ---
 
- ## Procedural Macros (Conceptual Overview)
+ ## Procedural Macros (Conceptual Overview) / 过程宏（概念概览）
 
- Procedural macros ("proc macros") are macros written as *separate Rust programs* that run at compile time and generate code. They're more powerful than `macro_rules!` but also more complex.
+ 过程宏（"Proc Macros"）是作为*独立的 Rust 程序*编写的宏，它们在编译时运行并生成代码。它们比 `macro_rules!` 更强大，但也更复杂。
 
- There are three kinds:
+ 共有三种类型：
 
-| Kind | Syntax | Example | What it does |
+| **Kind / 类型** | **Syntax / 语法** | **Example / 示例** | **What it does / 作用** |
 |------|--------|---------|-------------|
-| **Function-like** | `my_macro!(...)` | `sql!(SELECT * FROM users)` | Parses custom syntax, generates Rust code |
-| **Function-like** | `my_macro!(...)` | `sql!(...)` | 解析自定义语法，生成 Rust 代码 |
-| **Derive** | `#[derive(MyTrait)]` | `#[derive(Serialize)]` | Generates trait impl from struct definition |
-| **Derive** | `#[derive(...)]` | `#[derive(Serialize)]` | 从结构体定义生成 Trait 实现 |
-| **Attribute** | `#[my_attr]` | `#[tokio::main]`, `#[instrument]` | Transforms the annotated item |
-| **Attribute** | `#[my_attr]` | `#[tokio::main]` | 转换被标记的项目 |
 
- ### You've already used proc macros
+ ### You've already used proc macros / 你已经使用过过程宏了
- - `#[derive(Error)]` from `thiserror` — generates `Display` and `From` impls for error enums
+ - 来自 `thiserror` 的 `#[derive(Error)]` —— 为错误枚举生成 `Display` 和 `From` 实现。
- - `#[derive(Serialize, Deserialize)]` from `serde` — generates serialization code
+ - 来自 `serde` 的 `#[derive(Serialize, Deserialize)]` —— 生成序列化代码。
- - `#[tokio::main]` — transforms `async fn main()` into a runtime setup + block_on
+ - `#[tokio::main]` —— 将 `async fn main()` 转换为运行时设置及 `block_on` 调用。
- - `#[test]` — registered by the test harness (built-in proc macro)
+ - `#[test]` —— 由测试框架注册（内置的过程宏）。
 
- ### When to write your own proc macro
+ ### When to write your own proc macro / 何时编写自己的过程宏
- You likely won't need to write proc macros during this course. They're useful when:
+ 在本课程中你可能不需要编写过程宏。它们在以下场景中非常有用：
- - You need to inspect struct fields/enum variants at compile time (derive macros)
+ - 你需要在编译时检查结构体字段/枚举变体（派生宏）。
- - You're building a domain-specific language (function-like macros)
+ - 你正在构建领域特定语言（函数式宏）。
- - You need to transform function signatures (attribute macros)
+ - 你需要转换函数签名（属性宏）。
 
- For most code, `macro_rules!` or plain functions are sufficient.
+ 对于大多数代码，`macro_rules!` 或普通的函数就足够了。
 
- > **C++ comparison:** Procedural macros fill the role that code generators, template metaprogramming, and external tools like `protoc` fill in C++. The difference is that proc macros are part of the cargo build pipeline — no external build steps, no CMake custom commands.
+ > **C++ 对比**：过程宏填补了 C++ 中代码生成器、模板元编程以及像 `protoc` 这样的外部工具所扮演的角色。不同之处在于过程宏是 Cargo 构建流水线的一部分 —— 无需外部构建步骤，也无需 CMake 自定义命令。
 
 ---
 
- ## When to Use What: Macros vs Functions vs Generics
+ ## When to Use What: Macros vs Functions vs Generics / 何时用什么：宏 vs 函数 vs 泛型
 
 ```text
- Need to generate code?
+ 是否需要生成代码？
   │
-   ├── No → Use a function or generic function
+   ├── 否 → 使用函数或泛型函数
-   │         (simpler, better error messages, IDE support)
+   │         （更简单、错误消息更好、IDE 支持更佳）
   │
-   └── Yes ─┬── Variable number of arguments?
+   └── 是 ─┬── 是否需要变长参数？
-             │     └── Yes → macro_rules! (e.g., println!, vec!)
+             │     └── 是 → macro_rules! (例如：println!, vec!)
-             │
-             ├── Repetitive impl blocks for many types?
+             │
+             ├── 是否需要为多种类型生成重复的 impl 块？
-             │     └── Yes → macro_rules! with repetition
+             │     └── 是 → 带重复模式的 macro_rules!
-             │
-             ├── Need to inspect struct fields?
+             │
+             ├── 是否需要检查结构体字段？
-             │     └── Yes → Derive macro (proc macro)
+             │     └── 是 → 派生宏 (Derive macro)
-             │
-             ├── Need custom syntax (DSL)?
+             │
+             ├── 是否需要自定义语法 (DSL)？
-             │     └── Yes → Function-like proc macro
+             │     └── 是 → 函数式过程宏
-             │
-             └── Need to transform a function/struct?
+             │
+             └── 是否需要转换函数或结构体？
-                   └── Yes → Attribute proc macro
+                   └── 是 → 属性过程宏
 ```
 
- **General guideline:** If a function or generic can do it, don't use a macro. Macros have worse error messages, no IDE auto-complete inside the macro body, and are harder to debug.
+ **通用准则**：如果函数或泛型能做到，就不要用宏。宏的错误消息更糟，在宏体内没有 IDE 自动补全，且更难调试。
 
 ---
 
- ## Exercises
+ ## Exercises / 练习
 
- ### 🟢 Exercise 1: `min!` macro
+ ### 🟢 Exercise 1: `min!` macro / 练习 1：`min!` 宏
 
- Write a `min!` macro that:
+ 编写一个 `min!` 宏，要求：
- - `min!(a, b)` returns the smaller of two values
+ - `min!(a, b)` 返回两个值中的较小者。
- - `min!(a, b, c)` returns the smallest of three values
+ - `min!(a, b, c)` 返回三个值中的最小值。
- - Works with any type that implements `PartialOrd`
+ - 适用于任何实现了 `PartialOrd` 的类型。
 
- **Hint:** You'll need two match arms in your `macro_rules!`.
+ **提示**：你需要在 `macro_rules!` 中编写两个匹配分支。
 
- <details><summary>Solution (click to expand)</summary>
+ <details><summary>Solution / 解决方案（点击展开）</summary>
 
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
 
- **Note:** For production code, prefer `std::cmp::min` or `a.min(b)`. This exercise demonstrates the mechanics of multi-arm macros.
+ **注**：在生产代码中，请优先使用 `std::cmp::min` 或 `a.min(b)`。此练习仅用于演示多分支宏的机制。
 
 </details>
 
- ### 🟡 Exercise 2: `hashmap!` from scratch
+ ### 🟡 Exercise 2: `hashmap!` from scratch / 练习 2：从零开始编写 `hashmap!`
 
- Without looking at the example above, write a `hashmap!` macro that:
+ 不看上面的例子，编写一个 `hashmap!` 宏，要求：
- - Creates a `HashMap` from `key => value` pairs
+ - 从 `key => value` 对创建 `HashMap`。
- - Supports trailing commas
+ - 支持尾随逗号。
- - Works with any hashable key type
+ - 适用于任何可哈希的键类型。
 
- <details><summary>Solution (click to expand)</summary>
+ <details><summary>Solution / 解决方案（点击展开）</summary>
 
 ```rust
 use std::collections::HashMap;
@@ -517,14 +513,14 @@
 
 </details>
 
- ### 🟡 Exercise 3: `assert_approx_eq!` for floating-point comparison
+ ### 🟡 Exercise 3: `assert_approx_eq!` for floating-point comparison / 练习 3：用于浮点比较的 `assert_approx_eq!`
 
- Write a macro `assert_approx_eq!(a, b, epsilon)` that panics if `|a - b| > epsilon`. This is useful for testing floating-point calculations where exact equality fails.
+ 编写一个宏 `assert_approx_eq!(a, b, epsilon)`，如果 `|a - b| > epsilon` 则触发 panic。这在测试浮点运算（精确相等往往失效）时非常有用。
 
- <details><summary>Solution (click to expand)</summary>
+ <details><summary>Solution / 解决方案（点击展开）</summary>
 
 ```rust
 macro_rules! assert_approx_eq {
     ($a:expr, $b:expr, $eps:expr) => {
         let (a, b, eps) = ($a as f64, $b as f64, $eps as f64);
@@ -553,24 +549,24 @@
 
 </details>
 
- ### 🔴 Exercise 4: `impl_display_for_enum!`
+ ### 🔴 Exercise 4: `impl_display_for_enum!` / 练习 4：`impl_display_for_enum!`
 
- Write a macro that generates a `Display` implementation for simple C-like enums. Given:
+ 编写一个宏，为简单的 C 风格枚举生成 `Display` 实现。给定：
 
 ```rust
 impl_display_for_enum! {
     enum Color {
         Red => "red",
         Green => "green",
         Blue => "blue",
     }
 }
 ```
 
- It should generate both the `enum Color { Red, Green, Blue }` definition AND the `impl Display for Color` that maps each variant to its string.
+ 它应该同时生成 `enum Color { Red, Green, Blue }` 定义以及将每个变体映射到其字符串的 `impl Display for Color` 实现。
 
- **Hint:** You'll need both `$( ... ),*` repetition and multiple fragment specifiers.
+ **提示**：你将同时需要 `$( ... ),*` 重复模式以及多个片段指示符。
 
- <details><summary>Solution (click to expand)</summary>
+ <details><summary>Solution / 解决方案（点击展开）</summary>
 
 ```rust
 use std::fmt;
@@ -608,8 +604,8 @@
     assert_eq!(format!("{}", Color::Red), "red");
     println!("All tests passed!");
 }
 ```
 
 </details>
