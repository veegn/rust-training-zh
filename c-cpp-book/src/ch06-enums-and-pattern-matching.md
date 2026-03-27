# 6. Rust enum types / 6. Rust 枚举类型
 
 > **What you'll learn / 你将学到：** Rust enums as discriminated unions (tagged unions done right), `match` for exhaustive pattern matching, and how enums replace C++ class hierarchies and C tagged unions with compiler-enforced safety.
 >
 > Rust 枚举作为判别式联合（正确的标签联合实现）、用于详尽模式匹配的 `match`，以及枚举如何通过编译器强制的安全机制取代 C++ 类层次结构和 C 标签联合。
 
 - Enum types are discriminated unions, i.e., they are a sum type of several possible different types with a tag that identifies the specific variant / 枚举类型是判别式联合（discriminated unions），即它们是几种可能不同类型的和类型（sum type），带有一个标识特定变体的标签
-     - For C developers: enums in Rust can carry data (tagged unions done right — the compiler tracks which variant is active)
+     - For C developers / C 开发者注意：Rust 中的枚举可以携带数据（正确的标签联合实现 —— 编译器会跟踪哪个变体处于活动状态）
-     - For C++ developers: Rust enums are like `std::variant` but with exhaustive pattern matching, no `std::get` exceptions, and no `std::visit` boilerplate
+     - For C++ developers / C++ 开发者注意：Rust 枚举类似于 `std::variant`，但具有详尽的模式匹配，没有 `std::get` 异常，也没有 `std::visit` 样板代码
-     - The size of the `enum` is that of the largest possible type. The individual variants are not related to one another and can have completely different types
+     - The size of the `enum` is that of the largest possible type. The individual variants are not related to one another and can have completely different types / `enum` 的大小取决于最大可能类型的大小。各个变体之间没有关系，可以具有完全不同的类型
-     - `enum` types are one of the most powerful features of the language — they replace entire class hierarchies in C++ (more on this in the Case Studies)
+     - `enum` types are one of the most powerful features of the language — they replace entire class hierarchies in C++ (more on this in the Case Studies) / `enum` 类型是该语言最强大的特性之一 —— 它们取代了 C++ 中的整个类层次结构（稍后在案例研究中会有更多介绍）
 ```rust
 fn main() {
     enum Numbers {
         Zero,
         SmallNumber(u8),
         BiggerNumber(u32),
         EvenBiggerNumber(u64),
     }
-    let a = Numbers::Zero;
+    let a = Numbers::Zero; // 零
-    let b = Numbers::SmallNumber(42);
+    let b = Numbers::SmallNumber(42); // 小数
-    let c : Numbers = a; // Ok -- the type of a is Numbers
+    let c : Numbers = a; // Ok -- the type of a is Numbers / OK —— a 的类型是 Numbers
-    let d : Numbers = b; // Ok -- the type of b is Numbers
+    let d : Numbers = b; // Ok -- the type of b is Numbers / OK —— b 的类型是 Numbers
 }
 ```
 ----
- # Rust match statement
+ # Rust match statement / Rust match 语句
- - The Rust ```match``` is the equivalent of the C "switch" on steroids
+ - The Rust ```match``` is the equivalent of the C "switch" on steroids / Rust 的 ```match``` 相当于加强版的 C “switch”
-     - ```match``` can be used for pattern matching on simple data types, ```struct```, ```enum```
+     - ```match``` can be used for pattern matching on simple data types, ```struct```, ```enum``` / ```match``` 可用于对简单数据类型、```struct```、```enum``` 进行模式匹配
-     - The ```match``` statement must be exhaustive, i.e., they must cover all possible cases for a given ```type```. The ```_``` can be used a wildcard for the "all else" case
+     - The ```match``` statement must be exhaustive, i.e., they must cover all possible cases for a given ```type```. The ```_``` can be used a wildcard for the "all else" case / ```match``` 语句必须是详尽的，即它们必须涵盖给定类型的所有可能情况。```_``` 可用作“所有其他情况”的通配符
-     - ```match``` can yield a value, but all arms (```=>```) of must return a value of the same type
+     - ```match``` can yield a value, but all arms (```=>```) of must return a value of the same type / ```match``` 可以产生一个值，但所有分支（```=>```）必须返回相同类型的值
 
 ```rust
 fn main() {
     let x = 42;
-    // In this case, the _ covers all numbers except the ones explicitly listed
+    // In this case, the _ covers all numbers except the ones explicitly listed / 在这种情况下，_ 涵盖了除明确列出的数字之外的所有数字
     let is_secret_of_life = match x {
-        42 => true, // return type is boolean value
+        42 => true, // return type is boolean value / 返回类型是布尔值
-        _ => false, // return type boolean value
+        _ => false, // return type boolean value / 返回类型布尔值
-        // This won't compile because return type isn't boolean
+        // This won't compile because return type isn't boolean / 这将无法编译，因为返回类型不是布尔值
         // _ => 0  
     };
     println!("{is_secret_of_life}");
 }
 ```
 
- # Rust match statement
+ # Rust match statement continued / Rust match 语句（续）
- - ```match``` supports ranges, boolean filters, and ```if``` guard statements
+ - ```match``` supports ranges, boolean filters, and ```if``` guard statements / ```match``` 支持范围、布尔过滤器和 ```if``` 守卫语句
 ```rust
 fn main() {
     let x = 42;
     match x {
-        // Note that the =41 ensures the inclusive range
+        // Note that the =41 ensures the inclusive range / 注意 =41 确保了包含边界的范围
         0..=41 => println!("Less than the secret of life"),
         42 => println!("Secret of life"),
         _ => println!("More than the secret of life"),
     }
     let y = 100;
     match y {
-        100 if x == 43 => println!("y is 100% not secret of life"),
+        100 if x == 43 => println!("y is 100% not secret of life"), // 100% 不是生命之秘
-        100 if x == 42 => println!("y is 100% secret of life"),
+        100 if x == 42 => println!("y is 100% secret of life"), // 100% 是生命之秘
-        _ => (),    // Do nothing
+        _ => (),    // Do nothing / 什么也不做
     }
 }
 ```
 
- # Rust match statement
+ # Rust match statement continued / Rust match 语句（续）
- - ```match``` and ```enums``` are often combined together
+ - ```match``` and ```enums``` are often combined together / ```match``` 和 ```enum``` 经常结合在一起使用
-     - The match statement can "bind" the contained value to a variable. Use ```_``` if the value is a don't care
+     - The match statement can "bind" the contained value to a variable. Use ```_``` if the value is a don't care / match 语句可以将包含的值“绑定”到变量。如果不关心该值，请使用 ```_```
-     - The ```matches!``` macro can be used to match to specific variant
+     - The ```matches!``` macro can be used to match to specific variant / ```matches!``` 宏可用于匹配特定的变体
 ```rust
 fn main() {
     enum Numbers {
         Zero,
         SmallNumber(u8),
         BiggerNumber(u32),
         EvenBiggerNumber(u64),
     }
     let b = Numbers::SmallNumber(42);
     match b {
         Numbers::Zero => println!("Zero"),
         Numbers::SmallNumber(value) => println!("Small number {value}"),
-        Numbers::BiggerNumber(_) | Numbers::EvenBiggerNumber(_) => println!("Some BiggerNumber or EvenBiggerNumber"),
+        Numbers::BiggerNumber(_) | Numbers::EvenBiggerNumber(_) => println!("Some BiggerNumber or EvenBiggerNumber"), // 较大的数
     }
     
-    // Boolean test for specific variants
+    // Boolean test for specific variants / 针对特定变体的布尔测试
     if matches!(b, Numbers::Zero | Numbers::SmallNumber(_)) {
         println!("Matched Zero or small number");
     }
 }
 ```
 
- # Rust match statement
+ # Rust match statement continued / Rust match 语句（续）
- - ```match``` can also perform matches using destructuring and slices
+ - ```match``` can also perform matches using destructuring and slices / ```match``` 还可以使用解构和切片进行匹配
 ```rust
 fn main() {
     struct Foo {
         x: (u32, bool),
         y: u32
     }
     let f = Foo {x: (42, true), y: 100};
     match f {
-        // Capture the value of x into a variable called tuple
+        // Capture the value of x into a variable called tuple / 将 x 的值捕获到名为 tuple 的变量中
         Foo{y: 100, x : tuple} => println!("Matched x: {tuple:?}"),
         _ => ()
     }
     let a = [40, 41, 42];
     match a {
-        // Last element of slice must be 42. @ is used to bind the match
+        // Last element of slice must be 42. @ is used to bind the match / 切片的最后一个元素必须是 42。使用 @ 绑定匹配项
         [rest @ .., 42] => println!("{rest:?}"),
-        // First element of the slice must be 42. @ is used to bind the match
+        // First element of the slice must be 42. @ is used to bind the match / 切片的第一个元素必须是 42。使用 @ 绑定匹配项
         [42, rest @ ..] => println!("{rest:?}"),
         _ => (),
     }
 }
 ```
 
- # Exercise: Implement add and subtract using match and enum
+ # Exercise: Implement add and subtract using match and enum / 练习：使用 match 和 enum 实现加减法
 
- 🟢 **Starter**
+ 🟢 **Starter / 入门级**
 
- - Write a function that implements arithmetic operations on unsigned 64-bit numbers
+ - Write a function that implements arithmetic operations on unsigned 64-bit numbers / 编写一个对 64 位无符号整数执行算术运算的函数
- - **Step 1**: Define an enum for operations:
+ - **Step 1**: Define an enum for operations / 第一步：定义操作枚举：
 ```rust
 enum Operation {
     Add(u64, u64),
     Subtract(u64, u64),
 }
 ```
- - **Step 2**: Define a result enum:
+ - **Step 2**: Define a result enum / 第二步：定义结果枚举：
 ```rust
 enum CalcResult {
-    Ok(u64),                    // Successful result
+    Ok(u64),                    // Successful result / 成功结果
-    Invalid(String),            // Error message for invalid operations
+    Invalid(String),            // Error message for invalid operations / 无效操作的错误消息
 }
 ```
- - **Step 3**: Implement `calculate(op: Operation) -> CalcResult`
+ - **Step 3**: Implement `calculate(op: Operation) -> CalcResult` / 第三步：实现 `calculate(op: Operation) -> CalcResult`
-     - For Add: return Ok(sum)
+     - For Add: return Ok(sum) / 对于加法：返回 Ok(sum)
-     - For Subtract: return Ok(difference) if first >= second, otherwise Invalid("Underflow")
+     - For Subtract: return Ok(difference) if first >= second, otherwise Invalid("Underflow") / 对于减法：如果第一个数 >= 第二个数则返回 Ok(difference)，否则返回 Invalid("Underflow")
- - **Hint**: Use pattern matching in your function:
+ - **Hint / 提示**：在你的函数中使用模式匹配：
 ```rust
 match op {
-    Operation::Add(a, b) => { /* your code */ },
+    Operation::Add(a, b) => { /* your code / 你的代码 */ },
-    Operation::Subtract(a, b) => { /* your code */ },
+    Operation::Subtract(a, b) => { /* your code / 你的代码 */ },
 }
 ```
 
- <details><summary>Solution (click to expand)</summary>
+ <details><summary>Solution (click to expand) / 解决方案（点击展开）</summary>
 
 ```rust
 enum Operation {
     Add(u64, u64),
     Subtract(u64, u64),
 }
 
 enum CalcResult {
     Ok(u64),
     Invalid(String),
 }
 
 fn calculate(op: Operation) -> CalcResult {
     match op {
         Operation::Add(a, b) => CalcResult::Ok(a + b),
         Operation::Subtract(a, b) => {
             if a >= b {
                 CalcResult::Ok(a - b)
             } else {
                 CalcResult::Invalid("Underflow".to_string())
             }
         }
     }
 }
 
 fn main() {
     match calculate(Operation::Add(10, 20)) {
         CalcResult::Ok(result) => println!("10 + 20 = {result}"),
         CalcResult::Invalid(msg) => println!("Error: {msg}"),
     }
     match calculate(Operation::Subtract(5, 10)) {
         CalcResult::Ok(result) => println!("5 - 10 = {result}"),
         CalcResult::Invalid(msg) => println!("Error: {msg}"),
     }
 }
 // Output / 输出：
 // 10 + 20 = 30
 // Error: Underflow
 ```
 
 </details>
 
- # Rust associated methods
+ # Rust associated methods / Rust 关联方法
- - ```impl``` can define methods associated for types like ```struct```, ```enum```, etc
+ - ```impl``` can define methods associated for types like ```struct```, ```enum```, etc / ```impl``` 可以为 ```struct```、```enum``` 等类型定义关联方法
-     - The methods may optionally take ```self``` as a parameter. ```self``` is conceptually similar to passing a pointer to the struct as the first parameter in C, or ```this``` in C++
+     - The methods may optionally take ```self``` as a parameter. ```self``` is conceptually similar to passing a pointer to the struct as the first parameter in C, or ```this``` in C++ / 方法可以可选地接收 ```self``` 作为参数。从概念上讲，```self``` 类似于 C 中将指向结构体的指针作为第一个参数传递，或者是 C++ 中的 ```this```
-     - The reference to ```self``` can be immutable (default: ```&self```), mutable (```&mut self```), or ```self``` (transferring ownership)
+     - The reference to ```self``` can be immutable (default: ```&self```), mutable (```&mut self```), or ```self``` (transferring ownership) / 对 ```self``` 的引用可以是不可变的（默认：```&self```）、可变的（```&mut self```）或 ```self```（转移所有权）
-     - The ```Self``` keyword can be used a shortcut to imply the type
+     - The ```Self``` keyword can be used a shortcut to imply the type / ```Self``` 关键字可用作指代该类型的快捷方式
 ```rust
 struct Point {x: u32, y: u32}
 impl Point {
     fn new(x: u32, y: u32) -> Self {
         Point {x, y}
     }
     fn increment_x(&mut self) {
         self.x += 1;
     }
 }
 fn main() {
     let mut p = Point::new(10, 20);
     p.increment_x();
 }
 ```
 
- # Exercise: Point add and transform
+ # Exercise: Point add and transform / 练习：Point 的加法与变换
 
- 🟡 **Intermediate** — requires understanding move vs borrow from method signatures
+ 🟡 **Intermediate / 中级** —— 需要从方法签名中理解移动 vs 借用
- - Implement the following associated methods for ```Point```
+ - Implement the following associated methods for ```Point``` / 为 ```Point``` 实现以下关联方法：
-     - ```add()``` will take another ```Point``` and will increment the x and y values in place (hint: use ```&mut self```)
+     - ```add()``` will take another ```Point``` and will increment the x and y values in place (hint: use ```&mut self```) / ```add()``` 将接收另一个 ```Point``` 并就地增加 x 和 y 值（提示：使用 ```&mut self```）
-     - ```transform()``` will consume an existing ```Point``` (hint: use ```self```) and return a new ```Point``` by squaring the x and y
+     - ```transform()``` will consume an existing ```Point``` (hint: use ```self```) and return a new ```Point``` by squaring the x and y / ```transform()``` 将消耗现有的 ```Point```（提示：使用 ```self```）并通过对 x 和 y 求平方返回一个新的 ```Point```
 
- <details><summary>Solution (click to expand)</summary>
+ <details><summary>Solution (click to expand) / 解决方案（点击展开）</summary>
 
 ```rust
 struct Point { x: u32, y: u32 }
 
 impl Point {
     fn new(x: u32, y: u32) -> Self {
         Point { x, y }
     }
     fn add(&mut self, other: &Point) {
         self.x += other.x;
         self.y += other.y;
     }
     fn transform(self) -> Point {
         Point { x: self.x * self.x, y: self.y * self.y }
     }
 }
 
 fn main() {
     let mut p1 = Point::new(2, 3);
     let p2 = Point::new(10, 20);
     p1.add(&p2);
     println!("After add: x={}, y={}", p1.x, p1.y);           // x=12, y=23
     let p3 = p1.transform();
     println!("After transform: x={}, y={}", p3.x, p3.y);     // x=144, y=529
-    // p1 is no longer accessible — transform() consumed it
+    // p1 is no longer accessible — transform() consumed it / p1 不再可访问 —— transform() 消耗了它
 }
 ```
 
 </details>
