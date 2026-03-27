### Rust array type / Rust 数组类型
 
 > **What you'll learn / 你将学到：** Rust's core data structures — arrays, tuples, slices, strings, structs, `Vec`, and `HashMap`. This is a dense chapter; focus on understanding `String` vs `&str` and how structs work. You'll revisit references and borrowing in depth in chapter 7.
+ >
+ > Rust 的核心数据结构 —— 数组、元组、切片、字符串、结构体、`Vec` 和 `HashMap`。这是一个内容密集的章节；请重点理解 `String` 与 `&str` 的区别以及结构体的工作原理。你将在第 7 章深入学习引用和借用。
 
- - Arrays contain a fixed number of elements of the same type
+ - Arrays contain a fixed number of elements of the same type / 数组包含固定数量的同类型元素
-     - Like all other Rust types, arrays are immutable by default (unless mut is used)
+     - Like all other Rust types, arrays are immutable by default (unless mut is used) / 与所有其他 Rust 类型一样，数组默认是不可变的（除非使用了 mut）
-     - Arrays are indexed using [] and are bounds checked. The len() method can be used to obtain the length of the array
+     - Arrays are indexed using [] and are bounds checked. The len() method can be used to obtain the length of the array / 数组使用 [] 进行索引，并且会进行边界检查。可以使用 len() 方法获取数组长度
 ```rust
     fn get_index(y : usize) -> usize {
         y+1        
     }
     
     fn main() {
-        // Initializes an array of 3 elements and sets all to 42
+        // Initializes an array of 3 elements and sets all to 42 / 初始化一个 3 元素的数组，全部设为 42
         let a : [u8; 3] = [42; 3];
-        // Alternative syntax
+        // Alternative syntax / 替代语法
         // let a = [42u8, 42u8, 42u8];
         for x in a {
             println!("{x}");
         }
         let y = get_index(a.len());
-        // Commenting out the below will cause a panic
+        // Commenting out the below will cause a panic / 下面一行如果取消注释会触发 panic
         //println!("{}", a[y]);
     }
 ```
 
 ----
- ### Rust array type continued
+ ### Rust array type continued / Rust 数组类型（续）
- - Arrays can be nested
+ - Arrays can be nested / 数组可以嵌套
-     - Rust has several built-in formatters for printing. In the below, the ```:?``` is the ```debug``` print formatter. The ```:#?``` formatter can be used for ```pretty print```. These formatters can be customized per type (more on this later) 
+     - Rust has several built-in formatters for printing. In the below, the ```:?``` is the ```debug``` print formatter. The ```:#?``` formatter can be used for ```pretty print```. These formatters can be customized per type (more on this later) / Rust 有几种内置的格式化程序。在下面，```:?``` 是 ```debug``` 打印格式化程序。```:#?``` 格式化程序可用于 ```pretty print```（美化打印）。这些格式化程序可以按类型自定义（稍后会详细介绍）
 ```rust
     fn main() {
         let a = [
-            [40, 0], // Define a nested array
+            [40, 0], // Define a nested array / 定义嵌套数组
             [41, 0],
             [42, 1],
         ];
         for x in a {
             println!("{x:?}");
         }
     }
 ```
 ----
- ### Rust tuples
+ ### Rust tuples / Rust 元组
- - Tuples have a fixed size and can group arbitrary types into a single compound type
+ - Tuples have a fixed size and can group arbitrary types into a single compound type / 元组具有固定大小，可以将任意类型组合成单个复合类型
-     - The constituent types can be indexed by their relative location (.0, .1, .2, ...). An empty tuple, i.e., () is called the unit value and is the equivalent of a void return value
+     - The constituent types can be indexed by their relative location (.0, .1, .2, ...). An empty tuple, i.e., () is called the unit value and is the equivalent of a void return value / 组成类型可以通过其相对位置（.0, .1, .2, ...）进行索引。空元组即 () 被称为 unit（单元）值，相当于 void 返回值
-     - Rust supports tuple destructuring to make it easy to bind variables to individual elements
+     - Rust supports tuple destructuring to make it easy to bind variables to individual elements / Rust 支持元组解构，可以轻松地将变量绑定到各个元素
 ```rust
 fn get_tuple() -> (u32, bool) {
     (42, true)        
 }
 
 fn main() {
    let t : (u8, bool) = (42, true);
    let u : (u32, bool) = (43, false);
    println!("{}, {}", t.0, t.1);
    println!("{}, {}", u.0, u.1);
-    let (num, flag) = get_tuple(); // Tuple destructuring
+    let (num, flag) = get_tuple(); // Tuple destructuring / 元组解构
    println!("{num}, {flag}");
 }
 ```
 
- ### Rust references
+ ### Rust references / Rust 引用
- - References in Rust are roughly equivalent to pointers in C with some key differences
+ - References in Rust are roughly equivalent to pointers in C with some key differences / Rust 中的引用大致相当于 C 中的指针，但有一些关键区别
-     - It is legal to have any number of read-only (immutable) references to a variable at any point of time. A reference cannot outlive the variable scope (this is a key concept called **lifetime**; discussed in detail later)
+     - It is legal to have any number of read-only (immutable) references to a variable at any point of time. A reference cannot outlive the variable scope (this is a key concept called **lifetime**; discussed in detail later) / 在任何时间点拥有任意数量的只读（不可变）变量引用都是合法的。引用不能超出变量作用域的存活期（这是一个名为**生命周期**的关键概念；稍后会详细讨论）
-     - Only a single writable (mutable) reference to a mutable variable is permitted and it must not overlap with any other reference.
+     - Only a single writable (mutable) reference to a mutable variable is permitted and it must not overlap with any other reference. / 对于可变变量，只允许存在一个可写（可变）引用，且它不能与任何其他引用重叠。
 ```rust
 fn main() {
     let mut a = 42;
     {
         let b = &a;
         let c = b;
-        println!("{} {}", *b, *c); // The compiler automatically dereferences *c
+        println!("{} {}", *b, *c); // The compiler automatically dereferences *c / 编译器会自动对 *c 进行解引用
-        // Illegal because b and still are still in scope
+        // Illegal because b and still are still in scope / 非法，因为 b 仍然在作用域内
         // let d = &mut a;
     }
-    let d = &mut a; // Ok: b and c are not in scope
+    let d = &mut a; // Ok: b and c are not in scope / Ok：b 和 c 不在作用域内
     *d = 43;
 }
 ```
 
 ----
- # Rust slices
+ # Rust slices / Rust 切片
- - Rust references can be used to create subsets of arrays
+ - Rust references can be used to create subsets of arrays / Rust 引用可用于创建数组的子集
-     - Unlike arrays, which have a static fixed length determined at compile time, slices can be of arbitrary size. Internally, slices are implemented as a "fat-pointer" that contains the length of the slice and a pointer to the starting element in the original array
+     - Unlike arrays, which have a static fixed length determined at compile time, slices can be of arbitrary size. Internally, slices are implemented as a "fat-pointer" that contains the length of the slice and a pointer to the starting element in the original array / 与编译时确定静态固定长度的数组不同，切片可以是任意大小。在内部，切片被实现为一个“胖指针（fat-pointer）”，包含切片长度和指向原始数组起始元素的指针
 ```rust
 fn main() {
     let a = [40, 41, 42, 43];
-    let b = &a[1..a.len()]; // A slice starting with the second element in the original
+    let b = &a[1..a.len()]; // A slice starting with the second element in the original / 从原始数组第二个元素开始的切片
-    let c = &a[1..]; // Same as the above
+    let c = &a[1..]; // Same as the above / 同上
-    let d = &a[..]; // Same as &a[0..] or &a[0..a.len()]
+    let d = &a[..]; // Same as &a[0..] or &a[0..a.len()] / 等同于 &a[0..] 或 &a[0..a.len()]
     println!("{b:?} {c:?} {d:?}");
 }
 ```
 ----
- # Rust constants and statics
+ # Rust constants and statics / Rust 常量与静态变量
- - The ```const``` keyword can be used to define a constant value. Constant values are evaluated at **compile time** and are inlined into the program
+ - The ```const``` keyword can be used to define a constant value. Constant values are evaluated at **compile time** and are inlined into the program / ```const``` 关键字用于定义常量值。常量值在**编译时**求值，并被内联到程序中
- - The ```static``` keyword is used to define the equivalent of global variables in languages like C/C++ Static variables have an addressable memory location and are created once and last the entire lifetime of the program
+ - The ```static``` keyword is used to define the equivalent of global variables in languages like C/C++ Static variables have an addressable memory location and are created once and last the entire lifetime of the program / ```static``` 关键字用于定义相当于 C/C++ 中全局变量的变量。静态变量具有可寻址的内存位置，只创建一次，并持续整个程序生命周期
 ```rust
 const SECRET_OF_LIFE: u32 = 42;
 static GLOBAL_VARIABLE : u32 = 2;
 fn main() {
     println!("The secret of life is {}", SECRET_OF_LIFE);
     println!("Value of global variable is {GLOBAL_VARIABLE}")
 }
 ```
 
 ----
- # Rust strings: String vs &str
+ # Rust strings: String vs &str / Rust 字符串：String vs &str
 
- - Rust has **two** string types that serve different purposes
+ - Rust has **two** string types that serve different purposes / Rust 有**两种**字符串类型，用于不同的目的
-     - `String` — owned, heap-allocated, growable (like C's `malloc`'d buffer, or C++'s `std::string`)
+     - `String` — owned, heap-allocated, growable (like C's `malloc`'d buffer, or C++'s `std::string`) / `String` —— 有所有权的、堆分配的、可增长的（类似于 C 的 `malloc` 缓冲区或 C++ 的 `std::string`）
-     - `&str` — borrowed, lightweight reference (like C's `const char*` with length, or C++'s `std::string_view` — but `&str` is **lifetime-checked** so it can never dangle)
+     - `&str` — borrowed, lightweight reference (like C's `const char*` with length, or C++'s `std::string_view` — but `&str` is **lifetime-checked** so it can never dangle) / `&str` —— 借用的、轻量级引用（类似于 C 的带长度的 `const char*` 或 C++ 的 `std::string_view` —— 但 `&str` 是**经过生命周期检查的**，所以永远不会悬垂）
-     - Unlike C's null-terminated strings, Rust strings track their length and are guaranteed valid UTF-8
+     - Unlike C's null-terminated strings, Rust strings track their length and are guaranteed valid UTF-8 / 与 C 的以 null 结尾的字符串不同，Rust 字符串会记录长度，并保证是有效的 UTF-8
 
- > **For C++ developers:** `String` ≈ `std::string`, `&str` ≈ `std::string_view`. Unlike `std::string_view`, a `&str` is guaranteed valid for its entire lifetime by the borrow checker.
+ > **For C++ developers / C++ 开发者注意：** `String` ≈ `std::string`，`&str` ≈ `std::string_view`。与 `std::string_view` 不同，借用检查器保证 `&str` 在其整个生命周期内都是有效的。
 
- ## String vs &str: Owned vs Borrowed
+ ## String vs &str: Owned vs Borrowed / String vs &str：有所有权 vs 借用
 
- > **Production patterns**: See [JSON handling: nlohmann::json → serde](ch17-2-avoiding-unchecked-indexing.md#json-handling-nlohmannjson--serde) for how string handling works with serde in production code.
+ > **Production patterns / 生产实践：** 关于生产代码中 serde 如何处理字符串，请参阅 [JSON handling: nlohmann::json → serde / JSON 处理：nlohmann::json → serde](ch17-2-avoiding-unchecked-indexing.md#json-handling-nlohmannjson--serde)。
 
-| **Aspect** | **C `char*`** | **C++ `std::string`** | **Rust `String`** | **Rust `&str`** |
+| **Aspect / 维度** | **C `char*`** | **C++ `std::string`** | **Rust `String`** | **Rust `&str`** |
 |------------|--------------|----------------------|-------------------|----------------|
-| **Memory** | Manual (`malloc`/`free`) | Heap-allocated, owns buffer | Heap-allocated, auto-freed | Borrowed reference (lifetime-checked) |
+| **Memory / 内存** | Manual (`malloc`/`free`) / 手动 | Heap-allocated, owns buffer / 堆分配，拥有缓冲区 | Heap-allocated, auto-freed / 堆分配，自动释放 | Borrowed reference (lifetime-checked) / 借用引用（生命周期检查） |
-| **Mutability** | Always mutable via pointer | Mutable | Mutable with `mut` | Always immutable |
+| **Mutability / 可变性** | Always mutable via pointer / 始终通过指针可变 | Mutable / 可变 | Mutable with `mut` / 使用 `mut` 可变 | Always immutable / 始终不可变 |
-| **Size info** | None (relies on `'\0'`) | Tracks length and capacity | Tracks length and capacity | Tracks length (fat pointer) |
+| **Size info / 长度信息** | None (relies on `'\0'`) / 无（依赖 `'\0'`） | Tracks length and capacity / 记录长度和容量 | Tracks length and capacity / 记录长度和容量 | Tracks length (fat pointer) / 记录长度（胖指针） |
-| **Encoding** | Unspecified (usually ASCII) | Unspecified (usually ASCII) | Guaranteed valid UTF-8 | Guaranteed valid UTF-8 |
+| **Encoding / 编码** | Unspecified / 未指定 (usually ASCII) | Unspecified / 未指定 (usually ASCII) | Guaranteed valid UTF-8 / 保证有效 UTF-8 | Guaranteed valid UTF-8 / 保证有效 UTF-8 |
-| **Null terminator** | Required | Required (`c_str()`) | Not used | Not used |
+| **Null terminator / 空结束符** | Required / 必须 | Required (`c_str()`) / 必须 | Not used / 不使用 | Not used / 不使用 |
 
 ```rust
 fn main() {
-    // &str - string slice (borrowed, immutable, usually a string literal)
+    // &str - string slice (borrowed, immutable, usually a string literal) / 字符串切片（借用的、不可变的，通常是字面量）
     let greeting: &str = "Hello";  // Points to read-only memory
 
-    // String - owned, heap-allocated, growable
+    // String - owned, heap-allocated, growable / String —— 有所有权的、堆分配的、可增长的
     let mut owned = String::from(greeting);  // Copies data to heap
-    owned.push_str(", World!");        // Grow the string
+    owned.push_str(", World!");        // Grow the string / 增长字符串
-    owned.push('!');                   // Append a single character
+    owned.push('!');                   // Append a single character / 追加单个字符
 
-    // Converting between String and &str
+    // Converting between String and &str / 在 String 和 &str 之间转换
-    let slice: &str = &owned;          // String -> &str (free, just a borrow)
+    let slice: &str = &owned;          // String -> &str (free, just a borrow) / String -> &str（开销极低，仅借用）
-    let owned2: String = slice.to_string();  // &str -> String (allocates)
+    let owned2: String = slice.to_string();  // &str -> String (allocates) / &str -> String（会分配空间）
-    let owned3: String = String::from(slice); // Same as above
+    let owned3: String = String::from(slice); // Same as above / 同上
 
-    // String concatenation (note: + consumes the left operand)
+    // String concatenation (note: + consumes the left operand) / 字符串拼接（注意：+ 会消耗左操作数）
     let hello = String::from("Hello");
     let world = String::from(", World!");
-    let combined = hello + &world;  // hello is moved (consumed), world is borrowed
+    let combined = hello + &world;  // hello is moved (consumed), world is borrowed / hello 被移动（消耗），world 被借用
-    // println!("{hello}");  // Won't compile: hello was moved
+    // println!("{hello}");  // Won't compile: hello was moved / 无法编译：hello 已被移动
 
-    // Use format! to avoid move issues
+    // Use format! to avoid move issues / 使用 format! 避免移动问题
     let a = String::from("Hello");
     let b = String::from("World");
-    let combined = format!("{a}, {b}!");  // Neither a nor b is consumed
+    let combined = format!("{a}, {b}!");  // Neither a nor b is consumed / a 和 b 都不会被消耗
 
     println!("{combined}");
 }
 ```
 
- ## Why You Cannot Index Strings with `[]`
+ ## Why You Cannot Index Strings with `[]` / 为什么你不能用 `[]` 索引字符串
 ```rust
 fn main() {
     let s = String::from("hello");
-    // let c = s[0];  // Won't compile! Rust strings are UTF-8, not byte arrays
+    // let c = s[0];  // Won't compile! Rust strings are UTF-8, not byte arrays / 无法编译！Rust 字符串是 UTF-8 编码，不是字节数组
 
-    // Safe alternatives:
+    // Safe alternatives / 安全替代方案：
-    let first_char = s.chars().next();           // Option<char>: Some('h')
+    let first_char = s.chars().next();           // Option<char>: Some('h') / 取得第一个字符
-    let as_bytes = s.as_bytes();                 // &[u8]: raw UTF-8 bytes
+    let as_bytes = s.as_bytes();                 // &[u8]: raw UTF-8 bytes / 取得原始 UTF-8 字节
-    let substring = &s[0..1];                    // &str: "h" (byte range, must be valid UTF-8 boundary)
+    let substring = &s[0..1];                    // &str: "h" (byte range, must be valid UTF-8 boundary) / 取得子串（字节范围，必须是有效的 UTF-8 边界）
 
     println!("First char: {:?}", first_char);
     println!("Bytes: {:?}", &as_bytes[..5]);
 }
 ```
 
- ## Exercise: String manipulation
+ ## Exercise: String manipulation / 练习：字符串操作
 
- 🟢 **Starter**
+ 🟢 **Starter / 入门级**
- - Write a function `fn count_words(text: &str) -> usize` that counts the number of whitespace-separated words in a string
+ - Write a function `fn count_words(text: &str) -> usize` that counts the number of whitespace-separated words in a string / 编写一个函数 `fn count_words(text: &str) -> usize` 来统计字符串中由空格分隔的单词数量
- - Write a function `fn longest_word(text: &str) -> &str` that returns the longest word (hint: you'll need to think about lifetimes -- why does the return type need to be `&str` and not `String`?)
+ - Write a function `fn longest_word(text: &str) -> &str` that returns the longest word (hint: you'll need to think about lifetimes -- why does the return type need to be `&str` and not `String`?) / 编写一个函数 `fn longest_word(text: &str) -> &str` 来返回最长的单词（提示：你需要思考生命周期 —— 为什么返回类型必须是 `&str` 而不是 `String`？）
 
- <details><summary>Solution (click to expand)</summary>
+ <details><summary>Solution (click to expand) / 解决方案（点击展开）</summary>
 
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
     println!("Word count: {}", count_words(text));       // 9
     println!("Longest word: {}", longest_word(text));     // "jumps"
 }
 ```
 
 </details>
 
- # Rust structs
+ # Rust structs / Rust 结构体
- - The ```struct``` keyword declares a user-defined struct type
+ - The ```struct``` keyword declares a user-defined struct type / ```struct``` 关键字声明用户定义的结构体类型
-     - ```struct``` members can either be named, or anonymous (tuple structs)
+     - ```struct``` members can either be named, or anonymous (tuple structs) / ```struct``` 成员既可以是命名的，也可以是隐名的（元组结构体）
- - Unlike languages like C++, there's no notion of "data inheritance" in Rust
+ - Unlike languages like C++, there's no notion of "data inheritance" in Rust / 与 C++ 等语言不同，Rust 并没有“数据继承”的概念
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
-    let z = MyStruct { num: x.num, ..x }; // The .. means copy remaining
+    let z = MyStruct { num: x.num, ..x }; // The .. means copy remaining / .. 表示复制剩余字段
     println!("{} {} {}", x.num, y.is_secret_of_life, z.num);
 }
 ```
 
- # Rust tuple structs
+ # Rust tuple structs / Rust 元组结构体
- - Rust tuple structs are similar to tuples and individual fields don't have names
+ - Rust tuple structs are similar to tuples and individual fields don't have names / Rust 元组结构体类似于元组，各个字段没有名称
-     - Like tuples, individual elements are accessed using .0, .1, .2, .... A common use case for tuple structs is to wrap primitive types to create custom types. **This can useful to avoid mixing differing values of the same type**
+     - Like tuples, individual elements are accessed using .0, .1, .2, .... A common use case for tuple structs is to wrap primitive types to create custom types. **This can useful to avoid mixing differing values of the same type** / 与元组一样，单个元素使用 .0, .1, .2, ... 访问。元组结构体的一个常见用法是包装原始类型以创建自定义类型。**这对于避免混淆相同类型的不同业务含义的值非常有用**
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
-    // let z : WeightInGrams = x;  // Won't compile: x was moved into to_weight_in_milligrams()
+    // let z : WeightInGrams = x;  // Won't compile: x was moved / 无法编译：x 已被移动
-    // let a : WeightInGrams = y;   // Won't compile: type mismatch (WeightInMilligrams vs WeightInGrams)
+    // let a : WeightInGrams = y;   // Won't compile: type mismatch / 无法编译：类型不匹配
 }
 ```
 
 
- **Note**: The `#[derive(...)]` attribute automatically generates common trait implementations for structs and enums. You'll see this used throughout the course:
+ **Note / 注意**：`#[derive(...)]` 属性会自动为结构体和枚举生成常见的 trait 实现。你会在本课程中经常看到它的使用：
 ```rust
 #[derive(Debug, Clone, PartialEq)]
 struct Point { x: i32, y: i32 }
 
 fn main() {
     let p = Point { x: 1, y: 2 };
-    println!("{:?}", p);           // Debug: works because of #[derive(Debug)]
+    println!("{:?}", p);           // Debug: works because of #[derive(Debug)] / Debug 打印：因 #[derive(Debug)] 而生效
-    let p2 = p.clone();           // Clone: works because of #[derive(Clone)]
+    let p2 = p.clone();           // Clone: works because of #[derive(Clone)] / 克隆：因 #[derive(Clone)] 而生效
-    assert_eq!(p, p2);            // PartialEq: works because of #[derive(PartialEq)]
+    assert_eq!(p, p2);            // PartialEq: works because of #[derive(PartialEq)] / 断言相等：因 #[derive(PartialEq)] 而生效
 }
 ```
- We'll cover the trait system in depth later, but `#[derive(Debug)]` is so useful that you should add it to nearly every `struct` and `enum` you create.
+ 我们稍后会深入讲解 trait 系统，但 `#[derive(Debug)]` 是如此有用，以至于你几乎应该在你创建的每个 `struct` 和 `enum` 上都加上它。
 
- # Rust Vec type
+ # Rust Vec type / Rust Vec 类型
- - The ```Vec<T>``` type implements a dynamic heap allocated buffer (similar to manually managed `malloc`/`realloc` arrays in C, or C++'s `std::vector`)
+ - The ```Vec<T>``` type implements a dynamic heap allocated buffer (similar to manually managed `malloc`/`realloc` arrays in C, or C++'s `std::vector`) / ```Vec<T>``` 类型实现了一个动态的堆分配缓冲区（类似于 C 中手动管理的 `malloc`/`realloc` 数组，或 C++ 的 `std::vector`）
-     - Unlike arrays with fixed size, `Vec` can grow and shrink at runtime
+     - Unlike arrays with fixed size, `Vec` can grow and shrink at runtime / 与固定大小的数组不同，`Vec` 可以在运行时增长和缩小
-     - `Vec` owns its data and automatically manages memory allocation/deallocation
+     - `Vec` owns its data and automatically manages memory allocation/deallocation / `Vec` 拥有其数据，并自动管理内存分配/释放
- - Common operations: `push()`, `pop()`, `insert()`, `remove()`, `len()`, `capacity()`
+ - Common operations: `push()`, `pop()`, `insert()`, `remove()`, `len()`, `capacity()` / 常见操作：`push()`、`pop()`、`insert()`、`remove()`、`len()`、`capacity()`
 ```rust
 fn main() {
-    let mut v = Vec::new();    // Empty vector, type inferred from usage
+    let mut v = Vec::new();    // Empty vector, type inferred from usage / 空 vector，类型通过用法推导
-    v.push(42);                // Add element to end - Vec<i32>
+    v.push(42);                // Add element to end - Vec<i32> / 在末尾添加元素
     v.push(43);                
     
-    // Safe iteration (preferred)
+    // Safe iteration (preferred) / 安全迭代（推荐）
-    for x in &v {              // Borrow elements, don't consume vector
+    for x in &v {              // Borrow elements, don't consume vector / 借用元素，不消耗 vector
         println!("{x}");
     }
     
-    // Initialization shortcuts
+    // Initialization shortcuts / 初始化快捷方式
-    let mut v2 = vec![1, 2, 3, 4, 5];           // Macro for initialization
+    let mut v2 = vec![1, 2, 3, 4, 5];           // Macro for initialization / 初始化宏
-    let v3 = vec![0; 10];                       // 10 zeros
+    let v3 = vec![0; 10];                       // 10 zeros / 10 个零
     
-    // Safe access methods (preferred over indexing)
+    // Safe access methods (preferred over indexing) / 安全访问方法（优于索引）
     match v2.get(0) {
-        Some(first) => println!("First: {first}"),
+        Some(first) => println!("First: {first}"), // 取得第一个
-        None => println!("Empty vector"),
+        None => println!("Empty vector"), // vector 为空
     }
     
-    // Useful methods
+    // Useful methods / 有用的方法
     println!("Length: {}, Capacity: {}", v2.len(), v2.capacity());
-    if let Some(last) = v2.pop() {             // Remove and return last element
+    if let Some(last) = v2.pop() {             // Remove and return last element / 弹出最后一个元素
         println!("Popped: {last}");
     }
     
-    // Dangerous: direct indexing (can panic!)
+    // Dangerous: direct indexing (can panic!) / 危险：直接索引（可能导致 panic！）
     // println!("{}", v2[100]);  // Would panic at runtime
 }
 ```
- > **Production patterns**: See [Avoiding unchecked indexing](ch17-2-avoiding-unchecked-indexing.md#avoiding-unchecked-indexing) for safe `.get()` patterns from production Rust code.
+ > **Production patterns / 生产实践：** 关于生产代码中安全的 `.get()` 模式，请参阅 [Avoiding unchecked indexing / 避免未检查的索引](ch17-2-avoiding-unchecked-indexing.md#avoiding-unchecked-indexing)。
 
- # Rust HashMap type
+ # Rust HashMap type / Rust HashMap 类型
- - ```HashMap``` implements generic ```key``` -> ```value``` lookups (a.k.a. ```dictionary``` or ```map```)
+ - ```HashMap``` implements generic ```key``` -> ```value``` lookups (a.k.a. ```dictionary``` or ```map```) / ```HashMap``` 实现了通用的 ```键（key）``` -> ```值（value）``` 查找（又称 ```字典``` 或 ```映射```）
 ```rust
 fn main() {
-    use std::collections::HashMap;  // Need explicit import, unlike Vec
+    use std::collections::HashMap;  // Need explicit import, unlike Vec / 需要显式导入，不像 Vec 那样自动导入
-    let mut map = HashMap::new();       // Allocate an empty HashMap
+    let mut map = HashMap::new();       // Allocate an empty HashMap / 分配一个空的 HashMap
-    map.insert(40, false);  // Type is inferred as int -> bool
+    map.insert(40, false);  // Type is inferred / 类型已被推导
     map.insert(41, false);
     map.insert(42, true);
     for (key, value) in map {
         println!("{key} {value}");
     }
     let map = HashMap::from([(40, false), (41, false), (42, true)]);
     if let Some(x) = map.get(&43) {
         println!("43 was mapped to {x:?}");
     } else {
         println!("No mapping was found for 43");
     }
-    let x = map.get(&43).or(Some(&false));  // Default value if key isn't found
+    let x = map.get(&43).or(Some(&false));  // Default value / 默认值
     println!("{x:?}"); 
 }
 ```
 
- # Exercise: Vec and HashMap
+ # Exercise: Vec and HashMap / 练习：Vec 与 HashMap
 
- 🟢 **Starter**
+ 🟢 **Starter / 入门级**
- - Create a ```HashMap<u32, bool>``` with a few entries (make sure that some values are ```true``` and others are ```false```). Loop over all elements in the hashmap and put the keys into one ```Vec``` and the values into another
+ - Create a ```HashMap<u32, bool>``` with a few entries (make sure that some values are ```true``` and others are ```false```). Loop over all elements in the hashmap and put the keys into one ```Vec``` and the values into another / 创建一个含有几个条目的 ```HashMap<u32, bool>```（确保有些值是 ```true```，有些是 ```false```）。遍历 hashmap 中的所有元素，将键放入一个 ```Vec```，将值放入另一个 ```Vec```
 
- <details><summary>Solution (click to expand)</summary>
+ <details><summary>Solution (click to expand) / 解决方案（点击展开）</summary>
 
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
     println!("Keys:   {keys:?}");
     println!("Values: {values:?}");
 
-    // Alternative: use iterators with unzip()
+    // Alternative: use iterators with unzip() / 替代方案：使用带有 unzip() 的迭代器
     let (keys2, values2): (Vec<u32>, Vec<bool>) = map.into_iter().unzip();
     println!("Keys (unzip):   {keys2:?}");
     println!("Values (unzip): {values2:?}");
 }
 ```
 
 </details>
 
 ---
 
- ## Deep Dive: C++ References vs Rust References
+ ## Deep Dive: C++ References vs Rust References / 深入探讨：C++ 引用 vs Rust 引用
 
- > **For C++ developers:** C++ programmers often assume Rust `&T` works like C++ `T&`. While superficially similar, there are fundamental differences that cause confusion. C developers can skip this section — Rust references are covered in [Ownership and Borrowing](ch07-ownership-and-borrowing.md).
+ > **For C++ developers / C++ 开发者注意：** C++ 程序员经常假设 Rust 的 `&T` 工作方式与 C++ 的 `T&` 相同。虽然表面相似，但存在导致混淆的根本差异。C 开发者可以跳过此部分 —— Rust 引用在 [Ownership and Borrowing / 所有权与借用](ch07-ownership-and-borrowing.md) 中涵盖。
 
- #### 1. No Rvalue References or Universal References
+ #### 1. No Rvalue References or Universal References / 没有右值引用或万能引用
 
- In C++, `&&` has two meanings depending on context:
+ 在 C++ 中，`&&` 根据上下文有两种含义：
 
 ```cpp
 // C++: && means different things:
 int&& rref = 42;           // Rvalue reference — binds to temporaries
 void process(Widget&& w);   // Rvalue reference — caller must std::move
 
 // Universal (forwarding) reference — deduced template context:
 template<typename T>
 void forward(T&& arg) {     // NOT an rvalue ref! Deduced as T& or T&&
     inner(std::forward<T>(arg));  // Perfect forwarding
 }
 ```
 
- **In Rust: none of this exists.** `&&` is simply the logical AND operator.
+ **在 Rust 中：这些都不存在。** `&&` 仅仅是逻辑与运算符。
 
 ```rust
 // Rust: && is just boolean AND
 let a = true && false; // false
 
 // Rust has NO rvalue references, no universal references, no perfect forwarding.
+// Rust 没有右值引用，没有万能引用，也没有完美转发。
 // Instead:
+// 相反地：
- //   - Move is the default for non-Copy types (no std::move needed)
+ //   - Move is the default for non-Copy types (no std::move needed) / 非 Copy 类型默认就是移动（不需要 std::move）
- //   - Generics + trait bounds replace universal references
+ //   - Generics + trait bounds replace universal references / 泛型 + trait 约束取代了万能引用
- //   - No temporary-binding distinction — values are values
+ //   - No temporary-binding distinction — values are values / 没有临时变量绑定的区分 —— 值就是值
 
- fn process(w: Widget) { }      // Takes ownership (like C++ value param + implicit move)
+ fn process(w: Widget) { }      // Takes ownership / 获取所有权
- fn process_ref(w: &Widget) { } // Borrows immutably (like C++ const T&)
+ fn process_ref(w: &Widget) { } // Borrows immutably / 不可变借用
- fn process_mut(w: &mut Widget) { } // Borrows mutably (like C++ T&, but exclusive)
+ fn process_mut(w: &mut Widget) { } // Borrows mutably / 可变借用
 ```
 
-| C++ Concept | Rust Equivalent | Notes |
+| **C++ Concept / C++ 概念** | **Rust Equivalent / Rust 等价物** | **Notes / 说明** |
 |-------------|-----------------|-------|
-| `T&` (lvalue ref) | `&T` or `&mut T` | Rust splits into shared vs exclusive |
+| `T&` (lvalue ref) | `&T` or `&mut T` | Rust splits into shared vs exclusive / Rust 将其拆分为共享 vs 排他 |
-| `T&&` (rvalue ref) | Just `T` | Take by value = take ownership |
+| `T&&` (rvalue ref) | Just `T` | Take by value = take ownership / 按值接收 = 获取所有权 |
-| `T&&` in template (universal ref) | `impl Trait` or `<T: Trait>` | Generics replace forwarding |
+| `T&&` in template (universal ref) | `impl Trait` or `<T: Trait>` | Generics replace forwarding / 泛型取代了转发 |
-| `std::move(x)` | `x` (just use it) | Move is the default |
+| `std::move(x)` | `x` (just use it) | Move is the default / 移动是默认行为 |
-| `std::forward<T>(x)` | No equivalent needed | No universal references to forward |
+| `std::forward<T>(x)` | No equivalent needed | No universal references to forward / 无需转发万能引用 |
 
- #### 2. Moves Are Bitwise — No Move Constructors
+ #### 2. Moves Are Bitwise — No Move Constructors / 移动是按位进行的 —— 没有移动构造函数
 
- In C++, moving is a *user-defined operation* (move constructor / move assignment). In Rust, moving is always a **bitwise memcpy** of the value, and the source is invalidated:
+ 在 C++ 中，移动是一种*用户定义的处理*（移动构造函数 / 移动赋值）。在 Rust 中，移动始终是值的**按位 memcpy**，并且源对象会失效：
 
 ```rust
 // Rust move = memcpy the bytes, mark source as invalid
+// Rust move = 字节拷贝，并标记源对象无效
 let s1 = String::from("hello");
- let s2 = s1; // Bytes of s1 are copied to s2's stack slot
+ let s2 = s1; // Bytes of s1 are copied to s2's stack slot / s1 的字节被拷贝到 s2 的栈槽位中
-               // s1 is now invalid — compiler enforces this
+               // s1 is now invalid — compiler enforces this / s1 现已无效 —— 编译器强制执行此规则
 // println!("{s1}"); // ❌ Compile error: value used after move
 ```
 
 ```cpp
 // C++ move = call the move constructor (user-defined!)
+// C++ move = 调用移动构造函数（用户定义的！）
 std::string s1 = "hello";
- std::string s2 = std::move(s1); // Calls string's move ctor
+ std::string s2 = std::move(s1); // Calls string's move ctor / 调用 string 的移动构造函数
- // s1 is now a "valid but unspecified state" zombie
+ // s1 is now a "valid but unspecified state" zombie / s1 处于“有效但未指定状态”的僵尸状态
- std::cout << s1; // Compiles! Prints... something (empty string, usually)
+ std::cout << s1; // Compiles! / 能编译！
 ```
 
- **Consequences**:
+ **结果 / Consequences**：
- - Rust has no Rule of Five (no copy ctor, move ctor, copy=, move=, destructor to define)
+ - Rust has no Rule of Five (no copy ctor, move ctor, copy=, move=, destructor to define) / Rust 没有 Rule of Five（不需要定义拷贝构造、移动构造、拷贝/移动赋值以及析构函数）
- - No moved-from "zombie" state — the compiler simply prevents access
+ - No moved-from "zombie" state — the compiler simply prevents access / 没有 move 后的“僵尸”状态 —— 编译器直接阻止访问
- - No `noexcept` considerations for moves — bitwise copy can't throw
+ - No `noexcept` considerations for moves — bitwise copy can't throw / 移动时无需考虑 `noexcept` —— 按位拷贝不会抛出异常
 
- #### 3. Auto-Deref: The Compiler Sees Through Indirection
+ #### 3. Auto-Deref: The Compiler Sees Through Indirection / 自动解引用：编译器能看穿间接层
 
- Rust automatically dereferences through multiple layers of pointers/wrappers via the `Deref` trait. This has no C++ equivalent:
+ Rust 通过 `Deref` trait 自动对多层指针/包装器进行解引用。这在 C++ 中没有等价物：
 
 ```rust
 use std::sync::{Arc, Mutex};
 
 // Nested wrapping: Arc<Mutex<Vec<String>>>
 let data = Arc::new(Mutex::new(vec!["hello".to_string()]));
 
- // In C++, you'd need explicit unlocking and manual dereferencing at each layer.
+ // In C++, you'd need explicit unlocking and manual dereferencing at each layer. / 在 C++ 中，你需要在每一层手动解锁和手动解引用。
- // In Rust, the compiler auto-derefs through Arc → Mutex → MutexGuard → Vec:
+ // In Rust, the compiler auto-derefs through Arc → Mutex → MutexGuard → Vec: / 在 Rust 中，编译器自动通过 Arc -> Mutex -> MutexGuard -> Vec 进行解引用：
- let guard = data.lock().unwrap(); // Arc auto-derefs to Mutex
+ let guard = data.lock().unwrap(); // Arc auto-derefs / Arc 自动解引用
- let first: &str = &guard[0];      // MutexGuard→Vec (Deref), Vec[0] (Index),
+ let first: &str = &guard[0];      // MutexGuard→Vec (Deref), Vec[0] (Index)
-                                    // &String→&str (Deref coercion)
+                                    // &String→&str (Deref coercion) / &String -> &str（Deref 强制转换）
 println!("First: {first}");
 
- // Method calls also auto-deref:
+ // Method calls also auto-deref: / 方法调用也会自动解引用：
 let boxed_string = Box::new(String::from("hello"));
- println!("Length: {}", boxed_string.len());  // Box→String, then String::len()
+ println!("Length: {}", boxed_string.len());  // Box→String / Box -> String
- // No need for (*boxed_string).len() or boxed_string->len()
+ // No need for (*boxed_string).len() or boxed_string->len() / 无需使用 (*boxed_string).len() 或 boxed_string->len()
 ```
 
- **Deref coercion** also applies to function arguments — the compiler inserts dereferences to make types match:
+ **Deref coercion（Deref 强制转换）** 也适用于函数参数 —— 编译器插入解引用操作以使类型匹配：
 
 ```rust
 fn greet(name: &str) {
     println!("Hello, {name}");
 }
 
 fn main() {
     let owned = String::from("Alice");
     let boxed = Box::new(String::from("Bob"));
     let arced = std::sync::Arc::new(String::from("Carol"));
 
-    greet(&owned);  // &String → &str  (1 deref coercion)
+    greet(&owned);  // &String → &str / &String -> &str
-    greet(&boxed);  // &Box<String> → &String → &str  (2 deref coercions)
+    greet(&boxed);  // &Box<String> → &String → &str / 两次强制转换
-    greet(&arced);  // &Arc<String> → &String → &str  (2 deref coercions)
+    greet(&arced);  // &Arc<String> → &String → &str / 两次强制转换
-    greet("Dave");  // &str already — no coercion needed
+    greet("Dave");  // &str already / 已经是 &str
 }
- // In C++ you'd need .c_str() or explicit conversions for each case.
+ // In C++ you'd need .c_str() or explicit conversions for each case. / 在 C++ 中，你需要在每种情况下使用 .c_str() 或进行显式转换。
 ```
 
- **The Deref chain**: When you call `x.method()`, Rust's method resolution
+ **The Deref chain / Deref 链**：当你调用 `x.method()` 时，Rust 的方法解析会依次尝试接收者类型 `T`、然后是 `&T`、接着是 `&mut T`。如果都不匹配，它会通过 `Deref` trait 进行解引用，并对目标类型重复此过程。
- tries the receiver type `T`, then `&T`, then `&mut T`. If no match, it
- dereferences via the `Deref` trait and repeats with the target type.
- This continues through multiple layers — which is why `Box<Vec<T>>`
- "just works" like a `Vec<T>`. Deref *coercion* (for function arguments)
- is a separate but related mechanism that automatically converts `&Box<String>`
- to `&str` by chaining `Deref` impls.
+ 这可以持续多个层级 —— 这就是为什么 `Box<Vec<T>>` 能够像 `Vec<T>` 一样“直接使用”。Deref *coercion*（用于函数参数）是一个独立但相关的机制，它通过链接 `Deref` 实现自动将 `&Box<String>` 转换为 `&str`。
 
- #### 4. No Null References, No Optional References
+ #### 4. No Null References, No Optional References / 没有空引用，没有可选引用
 
 ```cpp
 // C++: references can't be null, but pointers can, and the distinction is blurry
+// C++：引用不能为 null，但指针可以，且两者的区别很模糊
 Widget& ref = *ptr;  // If ptr is null → UB
- Widget* opt = nullptr;  // "optional" reference via pointer
+ Widget* opt = nullptr;  // "optional" reference / “可选”引用
 ```
 
 ```rust
 // Rust: references are ALWAYS valid — guaranteed by the borrow checker
+// Rust：引用始终有效 —— 由借用检查器保证
 // No way to create a null or dangling reference in safe code
+// 在安全代码中无法创建 null 或悬垂引用
 let r: &i32 = &42; // Always valid
 
- // "Optional reference" is explicit:
+ // "Optional reference" is explicit / “可选引用”是显式的：
- let opt: Option<&Widget> = None; // Clear intent, no null pointer
+ let opt: Option<&Widget> = None; // Clear intent / 意图很明确
 if let Some(w) = opt {
-    w.do_something(); // Only reachable when present
+    w.do_something(); // Only reachable when present / 仅在存在时才可到达
 }
 ```
 
- #### 5. References Cannot Be Reseated
+ #### 5. References Cannot Be Reseated / 引用无法重绑定（指向其他对象）
 
 ```cpp
 // C++: a reference is an alias — it can't be rebound
+// C++：引用是一个别名 —— 无法重绑定
 int a = 1, b = 2;
 int& r = a;
- r = b;  // This ASSIGNS b's value to a — it does NOT rebind r!
+ r = b;  // This ASSIGNS b's value to a / 这是将 b 的值赋给 a —— 它并没有重绑定 r！
- // a is now 2, r still refers to a
+ // a is now 2, r still refers to a / a 现在是 2，r 仍然引用 a
 ```
 
 ```rust
 // Rust: let bindings can shadow, but references follow different rules
+// Rust：let 绑定可以遮蔽，但引用遵循不同的规则
 let a = 1;
 let b = 2;
 let r = &a;
- // r = &b;   // ❌ Cannot assign to immutable variable
+ // r = &b;   // ❌ Cannot assign to immutable variable / ❌ 无法为不可变变量赋值
- let r = &b;  // ✅ But you can SHADOW r with a new binding
+ let r = &b;  // ✅ But you can SHADOW r / ✅ 但你可以遮蔽 r
-              // The old binding is gone, not reseated
+              // The old binding is gone / 旧的绑定消失了，而不是重绑定了
 
- // With mut:
+ // With mut / 使用 mut：
 let mut r = &a;
- r = &b;      // ✅ r now points to b — this IS rebinding (not assignment through)
+ r = &b;      // ✅ r now points to b / ✅ r 现在指向 b —— 这就是重绑定
 ```
 
- > **Mental model**: In C++, a reference is a permanent alias for one object.
+ > **Mental model / 心理模型**：在 C++ 中，引用是某个对象的永久别名。
- > In Rust, a reference is a value (a pointer with lifetime guarantees) that
- > follows normal variable binding rules — immutable by default, rebindable
- > only if declared `mut`.
+ > 在 Rust 中，引用是一个值（带有生命周期保证的指针），它遵循常规的变量绑定规则 —— 默认不可变，只有声明为 `mut` 才能重绑定。
