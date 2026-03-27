## Connecting enums to Option and Result / 将枚举与 Option 和 Result 联系起来
 
 > **What you'll learn / 你将学到：** How Rust replaces null pointers with `Option<T>` and exceptions with `Result<T, E>`, and how the `?` operator makes error propagation concise. This is Rust's most distinctive pattern — errors are values, not hidden control flow.
 >
 > Rust 如何用 `Option<T>` 取代空指针，用 `Result<T, E>` 取代异常，以及 `?` 运算符如何使错误传播变得简洁。这是 Rust 最鲜明的模式 —— 错误是值，而不是隐藏的控制流。
 
 - Remember the `enum` type we learned earlier? Rust's `Option` and `Result` are simply enums defined in the standard library:
+ 还记得我们之前学过的 `enum` 类型吗？Rust 的 `Option` 和 `Result` 只是标准库中定义的简单枚举：
 ```rust
-// This is literally how Option is defined in std:
+// This is literally how Option is defined in std / 这确实是 Option 在 std 中的定义方式：
 enum Option<T> {
-    Some(T),  // Contains a value
+    Some(T),  // Contains a value / 包含一个值
-    None,     // No value
+    None,     // No value / 无值
 }
 
-// And Result:
+// And Result / 以及 Result：
 enum Result<T, E> {
-    Ok(T),    // Success with value
+    Ok(T),    // Success with value / 成功并包含值
-    Err(E),   // Error with details
+    Err(E),   // Error with details / 错误及其详情
 }
 ```
- - This means everything you learned about pattern matching with `match` works directly with `Option` and `Result`
+ - 这意味着你学到的所有关于 `match` 模式匹配的知识都可以直接应用于 `Option` 和 `Result`
- - There is **no null pointer** in Rust -- `Option<T>` is the replacement, and the compiler forces you to handle the `None` case
+ - Rust 中**没有空指针** —— `Option<T>` 是其替代方案，且编译器会强制你处理 `None` 的情况
 
- ### C++ Comparison: Exceptions vs Result
+ ### C++ Comparison: Exceptions vs Result / C++ 对比：异常 vs Result
 
-| **C++ Pattern** | **Rust Equivalent** | **Advantage** |
+| **C++ Pattern / C++ 模式** | **Rust Equivalent / Rust 等价物** | **Advantage / 优势** |
 |----------------|--------------------|--------------|
-| `throw std::runtime_error(msg)` | `Err(MyError::Runtime(msg))` | Error in return type — can't forget to handle |
+| `throw std::runtime_error(msg)` | `Err(MyError::Runtime(msg))` | Error in return type / 错误包含在返回类型中 —— 不会忘记处理 |
-| `try { } catch (...) { }` | `match result { Ok(v) => ..., Err(e) => ... }` | No hidden control flow |
+| `try { } catch (...) { }` | `match result { ... }` | No hidden control flow / 无隐藏控制流 |
-| `std::optional<T>` | `Option<T>` | Exhaustive match required — can't forget None |
+| `std::optional<T>` | `Option<T>` | Exhaustive match required / 必须穷尽匹配 —— 不会遗忘 None |
-| `noexcept` annotation | Default — all Rust functions are "noexcept" | Exceptions don't exist |
+| `noexcept` annotation / 注解 | Default / 默认 —— 所有 Rust 函数都是 "noexcept" | Exceptions don't exist / 并不存在异常 |
-| `errno` / return codes | `Result<T, E>` | Type-safe, can't ignore |
+| `errno` / return codes / 返回码 | `Result<T, E>` | Type-safe, can't ignore / 类型安全，无法被忽略 |
 
- # Rust Option type
+ # Rust Option type / Rust Option 类型
- - The Rust ```Option``` type is an ```enum``` with only two variants: ```Some<T>``` and ```None```
+ - Rust ```Option``` 类型是一个只有两个变体的 ```enum```：```Some<T>``` 和 ```None```
-     - The idea is that this represents a ```nullable``` type, i.e., it either contains a valid value of that type (```Some<T>```), or has no valid value (```None```)
+     - 其核心理念是代表一个“可空（nullable）”类型，即它要么包含一个该类型的有效值（```Some<T>```），要么没有有效值（```None```）
-     - The ```Option``` type is used in APIs result of an operation either succeeds and returns a valid value or it fails (but the specific error is irrelevant). For example, consider parsing a string for an integer value
+     - ```Option``` 类型常用于操作结果要么成功并返回有效值，要么失败（但具体错误细节无关紧要）的 API 中。例如，考虑在字符串中搜索某个字符的索引：
 ```rust
 fn main() {
-    // Returns Option<usize>
+    // Returns Option<usize> / 返回 Option<usize>
     let a = "1234".find("1");
     match a {
         Some(a) => println!("Found 1 at index {a}"),
         None => println!("Couldn't find 1")
     }
 }
 ```
 
- # Rust Option type
+ # Rust Option type continued / Rust Option 类型（续）
- - Rust ```Option``` can be processed in various ways
+ - Rust ```Option``` 可以通过多种方式处理
-     - ```unwrap()``` panics if the ```Option<T>``` is ```None``` and returns ```T``` otherwise and it is the least preferred approach 
+     - ```unwrap()``` 如果 ```Option<T>``` 是 ```None``` 则会发生 panic，否则返回 ```T```。这是最不推荐的方法。
-     - ```or()``` can be used to return an alternative value 
+     - ```or()``` 可用于返回一个替代值
-     ```if let``` lets us test for ```Some<T>```
+     - ```if let``` 让我们能够便捷地测试 ```Some<T>```
 
- > **Production patterns**: See [Safe value extraction with unwrap_or](ch17-2-avoiding-unchecked-indexing.md#safe-value-extraction-with-unwrap_or) and [Functional transforms: map, map_err, find_map](ch17-2-avoiding-unchecked-indexing.md#functional-transforms-map-map_err-find_map) for real-world examples from production Rust code.
+ > **Production patterns / 生产环境模式**：有关生产环境 Rust 代码的真实示例，请参阅 [Safe value extraction with unwrap_or / 使用 unwrap_or 安全提取值](ch17-2-avoiding-unchecked-indexing.md#safe-value-extraction-with-unwrap_or) 和 [Functional transforms: map, map_err, find_map / 函数式转换：map, map_err, find_map](ch17-2-avoiding-unchecked-indexing.md#functional-transforms-map-map_err-find_map)。
 ```rust
 fn main() {
-  // This return an Option<usize>
+  // This return an Option<usize> / 这返回一个 Option<usize>
   let a = "1234".find("1");
   println!("{a:?} {}", a.unwrap());
   let a = "1234".find("5").or(Some(42));
   println!("{a:?}");
   if let Some(a) = "1234".find("1") {
       println!("{a}");
   } else {
-    println!("Not found in string");
+    println!("Not found in string / 字符串中未找到");
   }
-  // This will panic
+  // This will panic / 这会发生 panic
   // "1234".find("5").unwrap();
 }
 ```
 
- # Rust Result type
+ # Rust Result type / Rust Result 类型
- - Result is an ```enum``` type similar to ```Option``` with two variants: ```Ok<T>``` or ```Err<E>```
+ - Result 是一种类似于 ```Option``` 的 ```enum``` 类型，具有两个变体：```Ok<T>``` 或 ```Err<E>```
-     - ```Result``` is used extensively in Rust APIs that can fail. The idea is that on success, functions will return a ```Ok<T>```, or they will return a specific error ```Err<T>```
+     - ```Result``` 在可能失败的 Rust API 中被广泛使用。其核心理念是：成功时函数返回 ```Ok<T>```，失败时返回具体的错误 ```Err<E>```。
 ```rust
   use std::num::ParseIntError;
   fn main() {
   let a : Result<i32, ParseIntError>  = "1234z".parse();
   match a {
       Ok(n) => println!("Parsed {n}"),
       Err(e) => println!("Parsing failed {e:?}"),
   }
   let a : Result<i32, ParseIntError>  = "1234z".parse().or(Ok(-1));
   println!("{a:?}");
   if let Ok(a) = "1234".parse::<i32>() {
     println!("Let OK {a}");  
   }
-  // This will panic
+  // This will panic / 这会发生 panic
   //"1234z".parse().unwrap();
 }
 ```
 
- ## Option and Result: Two Sides of the Same Coin
+ ## Option and Result: Two Sides of the Same Coin / Option 与 Result：一枚硬币的两面
 
- `Option` and `Result` are deeply related — `Option<T>` is essentially `Result<T, ()>` (a result where the error carries no information):
+ `Option` 与 `Result` 深度契合 —— `Option<T>` 本质上是 `Result<T, ()>`（一种错误信息不携带任何内容的 Result）：
 
-| `Option<T>` | `Result<T, E>` | Meaning |
+| **`Option<T>`** | **`Result<T, E>`** | **Meaning / 含义** |
 |-------------|---------------|---------|
-| `Some(value)` | `Ok(value)` | Success — value is present |
+| `Some(value)` | `Ok(value)` | Success / 成功 —— 值存在 |
-| `None` | `Err(error)` | Failure — no value (Option) or error details (Result) |
+| `None` | `Err(error)` | Failure / 失败 —— 无值 (Option) 或错误详情 (Result) |
 
- **Converting between them:**
+ **Converting between them / 两者之间的转换：**
 
 ```rust
 fn main() {
     let opt: Option<i32> = Some(42);
-    let res: Result<i32, &str> = opt.ok_or("value was None");  // Option → Result
+    let res: Result<i32, &str> = opt.ok_or("value was None");  // Option → Result / 将 Option 转换为 Result
     
     let res: Result<i32, &str> = Ok(42);
-    let opt: Option<i32> = res.ok();  // Result → Option (discards error)
+    let opt: Option<i32> = res.ok();  // Result → Option / 将 Result 转换为 Option（丢弃错误）
     
-    // They share many of the same methods:
+    // They share many of the same methods / 它们共享许多相同的方法：
     // .map(), .and_then(), .unwrap_or(), .unwrap_or_else(), .is_some()/is_ok()
 }
 ```
 
- > **Rule of thumb**: Use `Option` when absence is normal (e.g., looking up a key). Use `Result` when failure needs explanation (e.g., file I/O, parsing).
+ > **经验法则**：当“缺失”是预期内的正常情况时（例如：查找键值对），使用 `Option`。当“失败”需要解释时（例如：文件 I/O、转换），使用 `Result`。
 
- # Exercise: log() function implementation with Option
+ # Exercise: log() function implementation with Option / 练习：使用 Option 实现 log() 函数
 
- 🟢 **Starter**
+ 🟢 **Starter / 入门级**
 
- - Implement a ```log()``` function that accepts an ```Option<&str>``` parameter. If the parameter is ```None```, it should print a default string
+ - 实现一个接受 ```Option<&str>``` 参数的 ```log()``` 函数。如果参数为 ```None```，则应打印默认字符串
- - The function should return a ```Result``` with ```()``` for both success and error (in this case we'll never have an error)
+ - 函数应返回一个 ```Result```，成功和错误对应的类型都是 ```()```（本例中我们永远不会出错）
 
- <details><summary>Solution (click to expand)</summary>
+ <details><summary>Solution (click to expand) / 解决方案（点击展开）</summary>
 
 ```rust
 fn log(message: Option<&str>) -> Result<(), ()> {
     match message {
         Some(msg) => println!("LOG: {msg}"),
         None => println!("LOG: (no message provided)"),
     }
     Ok(())
 }
 
 fn main() {
     let _ = log(Some("System initialized"));
     let _ = log(None);
     
-    // Alternative using unwrap_or:
+    // Alternative using unwrap_or / 使用 unwrap_or 的替代方案：
     let msg: Option<&str> = None;
     println!("LOG: {}", msg.unwrap_or("(default message)"));
 }
- // Output:
+ // Output / 输出：
 // LOG: System initialized
 // LOG: (no message provided)
 // LOG: (default message)
 ```
 
 </details>
 
 ----
- # Rust error handling
+ # Rust error handling / Rust 错误处理
-  - Rust errors can be irrecoverable (fatal) or recoverable. Fatal errors result in a ``panic```
+  - Rust 错误可以是不可恢复的（致命的）或可恢复的。致命错误会导致 ``panic```
-     - In general, situation that result in ```panics``` should be avoided. ```panics``` are caused by bugs in the program, including exceeding index bounds, calling ```unwrap()``` on an ```Option<None>```, etc.
+     - 通常应避免会导致 ```panic``` 的情况。```panic``` 是由程序中的 bug 引起的，包括超出索引范围、对 ```Option<None>``` 调用 ```unwrap()``` 等。
-     - It is OK to have explicit ```panics``` for conditions that should be impossible. The ```panic!``` or ```assert!``` macros can be used for sanity checks
+     - 对于本该不可能发生的条件，可以使用显式的 ```panic```。```panic!``` 或 ```assert!``` 宏可用于完整性检查（sanity checks）。
 ```rust
 fn main() {
    let x : Option<u32> = None;
-   // println!("{x}", x.unwrap()); // Will panic
+   // println!("{x}", x.unwrap()); // Will panic / 会发生 panic
-   println!("{}", x.unwrap_or(0));  // OK -- prints 0
+   println!("{}", x.unwrap_or(0));  // OK -- prints 0 / OK —— 打印 0
    let x = 41;
-   //assert!(x == 42); // Will panic
+   //assert!(x == 42); // Will panic / 会发生 panic
-   //panic!("Something went wrong"); // Unconditional panic
+   //panic!("Something went wrong"); // Unconditional panic / 无条件 panic
    let _a = vec![0, 1];
-   // println!("{}", a[2]); // Out of bounds panic; use a.get(2) which will return Option<T>
+   // println!("{}", a[2]); // Out of bounds panic / 越界 panic；使用 a.get(2) 会返回 Option<T>
 }
 ```
 
- ## Error Handling: C++ vs Rust
+ ## Error Handling: C++ vs Rust / 错误处理：C++ vs Rust
 
- ### C++ Exception-Based Error Handling Problems
+ ### C++ Exception-Based Error Handling Problems / C++ 基于异常的错误处理问题
 
 ```cpp
 // C++ error handling - exceptions create hidden control flow
+// C++ 错误处理 —— 异常创建了隐藏的控制流
 #include <fstream>
 #include <stdexcept>
 
 std::string read_config(const std::string& path) {
     std::ifstream file(path);
     if (!file.is_open()) {
         throw std::runtime_error("Cannot open: " + path);
     }
     std::string content;
-    // What if getline throws? Is file properly closed?
+    // What if getline throws? Is file properly closed? / 如果 getline 抛出异常怎么办？文件是否被正确关闭？
-    // With RAII yes, but what about other resources?
+    // With RAII yes, but what about other resources? / 有了 RAII 是的，但其他资源呢？
     std::getline(file, content);
-    return content;  // What if caller doesn't try/catch?
+    return content;  // What if caller doesn't try/catch? / 如果调用者没有进行 try/catch 怎么办？
 }
 
 int main() {
-    // ERROR: Forgot to wrap in try/catch!
+    // ERROR: Forgot to wrap in try/catch! / 错误：忘记在 try/catch 中包装！
     auto config = read_config("nonexistent.txt");
-    // Exception propagates silently, program crashes
+    // Exception propagates silently, program crashes / 异常静默传播，程序崩溃
-    // Nothing in the function signature warned us
+    // Nothing in the function signature warned us / 函数签名中没有任何告警
     return 0;
 }
 ```
 
 ```mermaid
 graph TD
-    subgraph "C++ Error Handling Issues"
+    subgraph "C++ Error Handling Issues / C++ 错误处理问题"
-        CF["Function Call"]
+        CF["Function Call / 函数调用"]
-        CR["throw exception<br/>or return code"]
+        CR["throw exception / 抛出异常<br/>or return code / 或返回码"]
-        CIGNORE["[ERROR] Exception not caught<br/>or return code ignored"]
+        CIGNORE["[ERROR] Exception not caught / 异常未捕捉<br/>or return code ignored / 或返回码被忽略"]
-        CCHECK["try/catch or check"]
+        CCHECK["try/catch or check / 捕捉或检查"]
-        CERROR["Hidden control flow<br/>throws not in signature"]
+        CERROR["Hidden control flow / 隐藏控制流<br/>throws not in signature / 签名无异常声明"]
-        CERRNO["No compile-time<br/>enforcement"]
+        CERRNO["No compile-time / 无编译时<br/>enforcement / 强制约束"]
         
         CF --> CR
         CR --> CIGNORE
         CR --> CCHECK
         CCHECK --> CERROR
         CERROR --> CERRNO
         
-        CPROBLEMS["[ERROR] Exceptions invisible in types<br/>[ERROR] Hidden control flow<br/>[ERROR] Easy to forget try/catch<br/>[ERROR] Exception safety is hard<br/>[ERROR] noexcept is opt-in"]
+        CPROBLEMS["[ERROR] Exceptions invisible in types / 类型中不可见<br/>[ERROR] Hidden control flow / 隐藏控制流<br/>[ERROR] Easy to forget try/catch / 易遗忘<br/>[ERROR] Exception safety is hard / 异常安全困难<br/>[ERROR] noexcept is opt-in / noexcept 需要主动选择"]
     end
     
-    subgraph "Rust Result<T, E> System"
+    subgraph "Rust Result<T, E> System / Result 系统"
-        RF["Function Call"]
+        RF["Function Call / 函数调用"]
-        RR["Result<T, E><br/>Ok(value) | Err(error)"]
+        RR["Result<T, E><br/>Ok(value) | Err(error)"]
-        RMUST["[OK] Must handle<br/>Compile error if ignored"]
+        RMUST["[OK] Must handle / 必须处理<br/>Compile error if ignored / 忽略则报错"]
-        RMATCH["Pattern matching<br/>match, if let, ?"]
+        RMATCH["Pattern matching / 模式匹配<br/>match, if let, ?"]
-        RDETAIL["Detailed error info<br/>Custom error types"]
+        RDETAIL["Detailed error info / 详尽错误信息<br/>Custom error types / 自定义错误类型"]
-        RSAFE["Type-safe<br/>No global state"]
+        RSAFE["Type-safe / 类型安全<br/>No global state / 无全局状态"]
         
         RF --> RR
         RR --> RMUST
         RMUST --> RMATCH
         RMATCH --> RDETAIL
         RDETAIL --> RSAFE
         
-        RBENEFITS["[OK] Forced error handling<br/>[OK] Type-safe errors<br/>[OK] Detailed error info<br/>[OK] Composable with ?<br/>[OK] Zero runtime cost"]
+        RBENEFITS["[OK] Forced error handling / 强制错误处理<br/>[OK] Type-safe errors / 类型安全错误<br/>[OK] Detailed error info / 详尽信息<br/>[OK] Composable with ? / 可用 ? 组合<br/>[OK] Zero runtime cost / 零运行时成本"]
     end
     
     style CPROBLEMS fill:#ff6b6b,color:#000
     style RBENEFITS fill:#91e5a3,color:#000
     style CIGNORE fill:#ff6b6b,color:#000
     style RMUST fill:#91e5a3,color:#000
 ```
 
- ### `Result<T, E>` Visualization
+ ### `Result<T, E>` Visualization / 可视化
 
 ```rust
 // Rust error handling - comprehensive and forced
+// Rust 错误处理 —— 全面且强制
 use std::fs::File;
 use std::io::Read;
 
 fn read_file_content(filename: &str) -> Result<String, std::io::Error> {
-    let mut file = File::open(filename)?;  // ? automatically propagates errors
+    let mut file = File::open(filename)?;  // ? automatically propagates errors / ? 自动传播错误
     let mut contents = String::new();
     file.read_to_string(&mut contents)?;
-    Ok(contents)  // Success case
+    Ok(contents)  // Success case / 成功情况
 }
 
 fn main() {
     match read_file_content("example.txt") {
         Ok(content) => println!("File content: {}", content),
         Err(error) => println!("Failed to read file: {}", error),
-        // Compiler forces us to handle both cases!
+        // Compiler forces us to handle both cases! / 编译器强制我们处理这两种情况！
     }
 }
 ```
 
 ```mermaid
 graph TD
-    subgraph "Result<T, E> Flow"
+    subgraph "Result<T, E> Flow / 流转"
-        START["Function starts"]
+        START["Function starts / 函数开始"]
-        OP1["File::open()"]
+        OP1["File::open() / 打开文件"]
-        CHECK1{{"Result check"}}
+        CHECK1{{"Result check / 结果检查"}}
-        OP2["file.read_to_string()"]
+        OP2["file.read_to_string() / 读取文件"]
-        CHECK2{{"Result check"}}
+        CHECK2{{"Result check / 结果检查"}}
-        SUCCESS["Ok(contents)"]
+        SUCCESS["Ok(contents) / 成功"]
-        ERROR1["Err(io::Error)"]
+        ERROR1["Err(io::Error) / 错误"]
-        ERROR2["Err(io::Error)"]
+        ERROR2["Err(io::Error) / 错误"]
         
         START --> OP1
         OP1 --> CHECK1
-        CHECK1 -->|"Ok(file)"| OP2
+        CHECK1 -->|"Ok(file) / 成功"| OP2
-        CHECK1 -->|"Err(e)"| ERROR1
+        CHECK1 -->|"Err(e) / 失败"| ERROR1
         OP2 --> CHECK2
-        CHECK2 -->|"Ok(())"| SUCCESS
+        CHECK2 -->|"Ok(()) / 成功"| SUCCESS
-        CHECK2 -->|"Err(e)"| ERROR2
+        CHECK2 -->|"Err(e) / 失败"| ERROR2
         
-        ERROR1 --> PROPAGATE["? operator<br/>propagates error"]
+        ERROR1 --> PROPAGATE["? operator / 运算符<br/>propagates error / 传播错误"]
         ERROR2 --> PROPAGATE
-        PROPAGATE --> CALLER["Caller must<br/>handle error"]
+        PROPAGATE --> CALLER["Caller must / 调用者必须<br/>handle error / 处理错误"]
     end
     
-    subgraph "Pattern Matching Options"
+    subgraph "Pattern Matching Options / 模式匹配选项"
-        MATCH["match result"]
+        MATCH["match result / 匹配结果"]
-        IFLET["if let Ok(val) = result"]
+        IFLET["if let Ok(val) = result / 条件赋值"]
-        UNWRAP["result.unwrap()<br/>[WARNING] Panics on error"]
+        UNWRAP["result.unwrap()<br/>[WARNING] Panics on error / 报错会引发 Panic"]
-        EXPECT["result.expect(msg)<br/>[WARNING] Panics with message"]
+        EXPECT["result.expect(msg)<br/>[WARNING] Panics with message / 报错并带自定义信息"]
-        UNWRAP_OR["result.unwrap_or(default)<br/>[OK] Safe fallback"]
+        UNWRAP_OR["result.unwrap_or(default)<br/>[OK] Safe fallback / 安全备选项"]
-        QUESTION["result?<br/>[OK] Early return"]
+        QUESTION["result?<br/>[OK] Early return / 提早返回"]
         
-        MATCH --> SAFE1["[OK] Handles both cases"]
+        MATCH --> SAFE1["[OK] Handles both cases / 处理两种情况"]
-        IFLET --> SAFE2["[OK] Handles error case"]
+        IFLET --> SAFE2["[OK] Handles error case / 处理错误情况"]
-        UNWRAP_OR --> SAFE3["[OK] Always returns value"]
+        UNWRAP_OR --> SAFE3["[OK] Always returns value / 始终返回值"]
-        QUESTION --> SAFE4["[OK] Propagates to caller"]
+        QUESTION --> SAFE4["[OK] Propagates to caller / 传播给调用者"]
-        UNWRAP --> UNSAFE1["[ERROR] Can panic"]
+        UNWRAP --> UNSAFE1["[ERROR] Can panic / 可能 Panic"]
-        EXPECT --> UNSAFE2["[ERROR] Can panic"]
+        EXPECT --> UNSAFE2["[ERROR] Can panic / 可能 Panic"]
     end
     
     style SUCCESS fill:#91e5a3,color:#000
     style ERROR1 fill:#ffa07a,color:#000
     style ERROR2 fill:#ffa07a,color:#000
     style SAFE1 fill:#91e5a3,color:#000
     style SAFE2 fill:#91e5a3,color:#000
     style SAFE3 fill:#91e5a3,color:#000
     style SAFE4 fill:#91e5a3,color:#000
     style UNSAFE1 fill:#ff6b6b,color:#000
     style UNSAFE2 fill:#ff6b6b,color:#000
 ```
 
- # Rust error handling
+ # Rust error handling continued / Rust 错误处理（续）
- - Rust uses the ```enum Result<T, E>``` enum for recoverable error handling
+ - Rust 使用 ```enum Result<T, E>``` 枚举进行可恢复的错误处理
-     - The ```Ok<T>``` variant contains the result in case of success and ```Err<E>``` contains the error
+     - ```Ok<T>``` 变体包含成功时的结果，而 ```Err<E>``` 包含错误。
 ```rust
 fn main() {
     let x = "1234x".parse::<u32>();
     match x {
         Ok(x) => println!("Parsed number {x}"),
         Err(e) => println!("Parsing error {e:?}"),
     }
     let x  = "1234".parse::<u32>();
-    // Same as above, but with valid number
+    // Same as above, but with valid number / 与上面相同，但使用了有效数字
     if let Ok(x) = &x {
         println!("Parsed number {x}")
     } else if let Err(e) = &x {
         println!("Error: {e:?}");
     }
 }
 ```
 
- # Rust error handling
+ # Rust error handling continued / Rust 错误处理（续）
- - The try-operator ```?``` is a convenient short hand for the ```match``` ```Ok``` / ```Err``` pattern
+ - 问号操作符 ```?``` 是 ```match``` ```Ok``` / ```Err``` 模式的便捷简写
-     - Note the method must return ```Result<T, E>``` to enable use of ```?```
+     - 注意：方法必须返回 ```Result<T, E>``` 才能启用 ```?``` 的使用。
-     - The type for ```Result<T, E>``` can be changed. In the example below, we return the same error type (```std::num::ParseIntError```) returned by ```str::parse()``` 
+     - ```Result<T, E>``` 的类型可以更改。在下面的示例中，我们返回由 ```str::parse()``` 返回的相同错误类型（```std::num::ParseIntError```）。
 ```rust
 fn double_string_number(s : &str) -> Result<u32, std::num::ParseIntError> {
-   let x = s.parse::<u32>()?; // Returns immediately in case of an error
+   let x = s.parse::<u32>()?; // Returns immediately in case of an error / 如遇错误立即返回
    Ok(x*2)
 }
 fn main() {
     let result = double_string_number("1234");
     println!("{result:?}");
     let result = double_string_number("1234x");
     println!("{result:?}");
 }
 ```
 
- # Rust error handling
+ # Rust error handling continued / Rust 错误处理（续）
- - Errors can be mapped to other types, or to default values (https://doc.rust-lang.org/std/result/enum.Result.html#method.unwrap_or_default)
+ - 错误可以映射到其他类型，或映射到默认值（https://doc.rust-lang.org/std/result/enum.Result.html#method.unwrap_or_default）
 ```rust
-// Changes the error type to () in case of error
+// Changes the error type to () in case of error / 如果出错，将错误类型更改为 ()
 fn double_string_number(s : &str) -> Result<u32, ()> {
-   let x = s.parse::<u32>().map_err(|_|())?; // Returns immediately in case of an error
+   let x = s.parse::<u32>().map_err(|_|())?; // Returns immediately in case of an error / 如果出错立即返回
    Ok(x*2)
 }
 ```
 ```rust
 fn double_string_number(s : &str) -> Result<u32, ()> {
-   let x = s.parse::<u32>().unwrap_or_default(); // Defaults to 0 in case of parse error
+   let x = s.parse::<u32>().unwrap_or_default(); // Defaults to 0 / 在解析错误时默认为 0
    Ok(x*2)
 }
 ```
 ```rust
 fn double_optional_number(x : Option<u32>) -> Result<u32, ()> {
-    // ok_or converts Option<None> to Result<u32, ()> in the below
+    // ok_or converts Option<None> to Result<u32, ()> / 下面 ok_or 将 Option<None> 转换为 Result<u32, ()>
-    x.ok_or(()).map(|x|x*2) // .map() is applied only on Ok(u32)
+    x.ok_or(()).map(|x|x*2) // .map() is applied only on Ok(u32) / .map() 仅应用于 Ok(u32)
 }
 ```
 
- # Exercise: error handling
+ # Exercise: error handling / 练习：错误处理
 
- 🟡 **Intermediate**
+ 🟡 **Intermediate / 中级**
- - Implement a ```log()``` function with a single u32 parameter. If the parameter is not 42, return an error. The ```Result<>``` for success and error type is ```()```
+ - 实现一个带有单个 u32 参数的 ```log()``` 函数。如果参数不是 42，则返回错误。成功和错误的 ```Result<>``` 类型均为 ```()```。
- - Invoke ```log()``` function that exits with the same ```Result<>``` type if ```log()``` return an error. Otherwise print a message saying that log was successfully called
+ - 调用 ```log()``` 函数，如果 ```log()``` 返回错误，则以相同的 ```Result<>``` 类型退出。否则打印一条消息，说明 log 调用成功。
 
 ```rust
 fn log(x: u32) -> ?? {
 
 }
 
 fn call_log(x: u32) -> ?? {
-    // Call log(x), then exit immediately if it return an error
+    // Call log(x), then exit immediately if it return an error / 调用 log(x)，如果它返回错误则立即退出
-    println!("log was successfully called");
+    println!("log was successfully called / log 调用成功");
 }
 
 fn main() {
     call_log(42);
     call_log(43);
 }
 ``` 
 
- <details><summary>Solution (click to expand)</summary>
+ <details><summary>Solution (click to expand) / 解决方案（点击展开）</summary>
 
 ```rust
 fn log(x: u32) -> Result<(), ()> {
     if x == 42 {
         Ok(())
     } else {
         Err(())
     }
 }
 
 fn call_log(x: u32) -> Result<(), ()> {
-    log(x)?;  // Exit immediately if log() returns an error
+    log(x)?;  // Exit immediately if log() returns an error / 如果 log() 返回错误则立即退出
-    println!("log was successfully called with {x}");
+    println!("log was successfully called with {x} / 已成功调用 log，值为 {x}");
     Ok(())
 }
 
 fn main() {
-    let _ = call_log(42);  // Prints: log was successfully called with 42
+    let _ = call_log(42);  // Prints: log was successfully called with 42 / 打印成功信息
-    let _ = call_log(43);  // Returns Err(()), nothing printed
+    let _ = call_log(43);  // Returns Err(()), nothing printed / 返回 Err(())，没有任何打印
 }
- // Output:
+ // Output / 输出：
 // log was successfully called with 42
 ```
 
 </details>
