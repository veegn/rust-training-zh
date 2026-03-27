# Rust lifetime and borrowing / Rust 生命周期与借用
 
 > **What you'll learn / 你将学到：** How Rust's lifetime system ensures references never dangle — from implicit lifetimes through explicit annotations to the three elision rules that make most code annotation-free. Understanding lifetimes here is essential before moving on to smart pointers in the next section.
 >
 > Rust 的生命周期系统如何确保引用永远不会悬垂 —— 从隐式生命周期到显式标注，再到使大多数代码无需标注的三条省略规则。在进入下一节编写智能指针之前，理解生命周期至关重要。
 
 - Rust enforces a single mutable reference and any number of immutable references / Rust 强制执行单一可变引用和任意数量的不可变引用
-     - The lifetime of any reference must be at least as long as the original owning lifetime. These are implicit lifetimes and are inferred by the compiler (see https://doc.rust-lang.org/nomicon/lifetime-elision.html)
+     - The lifetime of any reference must be at least as long as the original owning lifetime. These are implicit lifetimes and are inferred by the compiler (see https://doc.rust-lang.org/nomicon/lifetime-elision.html) / 任何引用的生命周期必须至少与原始所有权的生存期一样长。这些是隐式生命周期，由编译器推导（参见 https://doc.rust-lang.org/nomicon/lifetime-elision.html）
 ```rust
 fn borrow_mut(x: &mut u32) {
     *x = 43;
 }
 fn main() {
     let mut x = 42;
     let y = &mut x;
     borrow_mut(y);
-    let _z = &x; // Permitted because the compiler knows y isn't subsequently used
+    let _z = &x; // Permitted / 允许，因为编译器知道 y 随后不再被使用
-    //println!("{y}"); // Will not compile if this is uncommented
+    //println!("{y}"); // Will not compile / 如果取消注释，将无法编译
-    borrow_mut(&mut x); // Permitted because _z isn't used 
+    borrow_mut(&mut x); // Permitted / 允许，因为 _z 没被使用 
-    let z = &x; // Ok -- mutable borrow of x ended after borrow_mut() returned
+    let z = &x; // Ok -- mutable borrow of x ended / Ok —— x 的可变借用在 borrow_mut() 返回后结束
     println!("{z}");
 }
 ```
 
- # Rust lifetime annotations
+ # Rust lifetime annotations / Rust 生命周期标注
- - Explicit lifetime annotations are needed when dealing with multiple lifetimes
+ - Explicit lifetime annotations are needed when dealing with multiple lifetimes / 在处理多个生命周期时，需要显式的生命周期标注
-     - Lifetimes are denoted with `'` and can be any identifier (`'a`, `'b`, `'static`, etc.)
+     - Lifetimes are denoted with `'` and can be any identifier (`'a`, `'b`, `'static`, etc.) / 生命周期用 `'` 表示，可以是任何标识符（`'a`、`'b`、`'static` 等）
-     - The compiler needs help when it can't figure out how long references should live
+     - The compiler needs help when it can't figure out how long references should live / 当编译器无法确定引用应该存活多久时，需要提供帮助
- - **Common scenario**: Function returns a reference, but which input does it come from?
+ - **Common scenario / 常见场景**：函数返回一个引用，但它来自哪个输入？
 ```rust
 #[derive(Debug)]
 struct Point {x: u32, y: u32}
 
- // Without lifetime annotation, this won't compile:
+ // Without lifetime annotation, this won't compile / 没有生命周期标注，以下代码无法编译：
 // fn left_or_right(pick_left: bool, left: &Point, right: &Point) -> &Point
 
- // With lifetime annotation - all references share the same lifetime 'a
+ // With lifetime annotation - all references share the same lifetime 'a / 带有显式标注 —— 所有引用共享相同的生命周期 'a
 fn left_or_right<'a>(pick_left: bool, left: &'a Point, right: &'a Point) -> &'a Point {
     if pick_left { left } else { right }
 }
 
- // More complex: different lifetimes for inputs
+ // More complex: different lifetimes for inputs / 更复杂的情况：输入具有不同的生命周期
 fn get_x_coordinate<'a, 'b>(p1: &'a Point, _p2: &'b Point) -> &'a u32 {
-    &p1.x  // Return value lifetime tied to p1, not p2
+    &p1.x  // Return value lifetime tied to p1, not p2 / 返回值的生命周期与 p1 绑定，而不是 p2
 }
 
 fn main() {
     let p1 = Point {x: 20, y: 30};
     let result;
     {
         let p2 = Point {x: 42, y: 50};
         result = left_or_right(true, &p1, &p2);
-        // This works because we use result before p2 goes out of scope
+        // This works because we use result before p2 goes out of scope / 这可以工作，因为我们在 p2 离开作用域之前使用了 result
         println!("Selected: {result:?}");
     }
-    // This would NOT work - result references p2 which is now gone:
+    // This would NOT work - result references p2 which is now gone / 这行不通 —— result 引用了现在已经消失的 p2：
     // println!("After scope: {result:?}");
 }
 ```
 
- # Rust lifetime annotations
+ # Rust lifetime annotations continued / Rust 生命周期标注（续）
- - Lifetime annotations are also needed for references in data structures
+ - Lifetime annotations are also needed for references in data structures / 数据结构中的引用也需要生命周期标注
 ```rust
 use std::collections::HashMap;
 #[derive(Debug)]
 struct Point {x: u32, y: u32}
 struct Lookup<'a> {
     map: HashMap<u32, &'a Point>,
 }
 fn main() {
     let p = Point{x: 42, y: 42};
     let p1 = Point{x: 50, y: 60};
     let mut m = Lookup {map : HashMap::new()};
     m.map.insert(0, &p);
     m.map.insert(1, &p1);
     {
         let p3 = Point{x: 60, y:70};
-        //m.map.insert(3, &p3); // Will not compile
+        //m.map.insert(3, &p3); // Will not compile / 无法编译
-        // p3 is dropped here, but m will outlive
+        // p3 is dropped here, but m will outlive / p3 在这里被 drop，但 m 存活更久
     }
     for (k, v) in m.map {
         println!("{v:?}");
     }
-    // m is dropped here
+    // m is dropped here / m 在这里被 drop
-    // p1 and p are dropped here in that order
+    // p1 and p are dropped here in that order / p1 和 p 依次在这里被 drop
 } 
 ```
 
- # Exercise: First word with lifetimes
+ # Exercise: First word with lifetimes / 练习：带有生命周期的第一个单词
 
- 🟢 **Starter** — practice lifetime elision in action
+ 🟢 **Starter / 入门级** —— 实践生命周期省略规则
 
- Write a function `fn first_word(s: &str) -> &str` that returns the first whitespace-delimited word from a string. Think about why this compiles without explicit lifetime annotations (hint: elision rule #1 and #2).
+ 编写一个函数 `fn first_word(s: &str) -> &str`，返回字符串中第一个由空格分隔的单词。思考为什么这段代码在没有显式生命周期标注的情况下也能编译（提示：省略规则 #1 和 #2）。
 
- <details><summary>Solution (click to expand)</summary>
+ <details><summary>Solution (click to expand) / 解决方案（点击展开）</summary>
 
 ```rust
 fn first_word(s: &str) -> &str {
     // The compiler applies elision rules:
-    // Rule 1: input &str gets lifetime 'a → fn first_word(s: &'a str) -> &str
+    // Rule 1: input &str gets lifetime 'a / 规则1：输入 &str 获得生命周期 'a
-    // Rule 2: single input lifetime → output gets same → fn first_word(s: &'a str) -> &'a str
+    // Rule 2: single input lifetime → output gets same / 规则2：单个输入生命周期 -> 输出获得相同的生命周期
     match s.find(' ') {
         Some(pos) => &s[..pos],
         None => s,
     }
 }
 
 fn main() {
     let text = "hello world foo";
     let word = first_word(text);
     println!("First word: {word}");  // "hello"
     
     let single = "onlyone";
     println!("First word: {}", first_word(single));  // "onlyone"
 }
 ```
 
 </details>
 
- # Exercise: Slice storage with lifetimes
+ # Exercise: Slice storage with lifetimes / 练习：带有生命周期的切片存储
 
- 🟡 **Intermediate** — your first encounter with lifetime annotations
+ 🟡 **Intermediate / 中级** —— 第一次接触生命周期标注
- - Create a structure that stores references to the slice of a ```&str```
+ - Create a structure that stores references to the slice of a ```&str``` / 创建一个存储 ```&str``` 切片引用的结构体
-     - Create a long ```&str``` and store references slices from it inside the structure
+     - Create a long ```&str``` and store references slices from it inside the structure / 创建一个长 ```&str``` 并在结构体内部存储其中的切片引用
-     - Write a function that accepts the structure and returns the contained slice
+     - Write a function that accepts the structure and returns the contained slice / 编写一个接收该结构体并返回包含切片的函数
 ```rust
- // TODO: Create a structure to store a reference to a slice
+ // TODO: Create a structure to store a reference to a slice / TODO：创建一个存储切片引用的结构体
 struct SliceStore {
 
 }
 fn main() {
     let s = "This is long string";
     let s1 = &s[0..];
     let s2 = &s[1..2];
     // let slice = struct SliceStore {...};
     // let slice2 = struct SliceStore {...};
 }
 ```
 
- <details><summary>Solution (click to expand)</summary>
+ <details><summary>Solution (click to expand) / 解决方案（点击展开）</summary>
 
 ```rust
 struct SliceStore<'a> {
     slice: &'a str,
 }
 
 impl<'a> SliceStore<'a> {
     fn new(slice: &'a str) -> Self {
         SliceStore { slice }
     }
 
     fn get_slice(&self) -> &'a str {
         self.slice
     }
 }
 
 fn main() {
     let s = "This is a long string";
     let store1 = SliceStore::new(&s[0..4]);   // "This"
     let store2 = SliceStore::new(&s[5..7]);   // "is"
     println!("store1: {}", store1.get_slice());
     println!("store2: {}", store2.get_slice());
 }
- // Output:
+ // Output / 输出：
 // store1: This
 // store2: is
 ```
 
 </details>
 
 ---
 
- ## Lifetime Elision Rules Deep Dive
+ ## Lifetime Elision Rules Deep Dive / 生命周期省略规则深钻
 
- C programmers often ask: "If lifetimes are so important, why don't most Rust functions
+ C 程序员经常问：“如果生命周期如此重要，为什么大多数 Rust 函数
- have `'a` annotations?" The answer is **lifetime elision** — the compiler applies three
+ 不需要 `'a` 标注？”答案是**生命周期省略（lifetime elision）** —— 编译器应用三条
- deterministic rules to infer lifetimes automatically.
+ 确定性规则来自动推导生命周期。
 
- ### The Three Elision Rules
+ ### The Three Elision Rules / 三条省略规则
 
- The Rust compiler applies these rules **in order** to function signatures. If all output
+ Rust 编译器**按顺序**对函数签名应用这些规则。如果应用规则后所有输出
- lifetimes are determined after applying the rules, no annotations are needed.
+ 的生命周期都已确定，则无需标注。
 
 ```mermaid
 flowchart TD
-    A["Function signature with references"] --> R1
+    A["Function signature with references<br/>带有引用的函数签名"] --> R1
-    R1["Rule 1: Each input reference<br/>gets its own lifetime<br/><br/>fn f(&str, &str)<br/>→ fn f<'a,'b>(&'a str, &'b str)"]
+    R1["Rule 1: Each input reference<br/>gets its own lifetime<br/><br/>规则 1：每个输入引用<br/>获得各自的生命周期"]
     R1 --> R2
-    R2["Rule 2: If exactly ONE input<br/>lifetime, assign it to ALL outputs<br/><br/>fn f(&str) → &str<br/>→ fn f<'a>(&'a str) → &'a str"]
+    R2["Rule 2: If exactly ONE input<br/>lifetime, assign it to ALL outputs<br/><br/>规则 2：如果只有一个输入生命周期，<br/>将其分配给所有输出"]
     R2 --> R3
-    R3["Rule 3: If one input is &self<br/>or &mut self, assign its lifetime<br/>to ALL outputs<br/><br/>fn f(&self, &str) → &str<br/>→ fn f<'a>(&'a self, &str) → &'a str"]
+    R3["Rule 3: If one input is &self<br/>or &mut self, assign its lifetime<br/>to ALL outputs<br/><br/>规则 3：如果输入中有 &self<br/>或 &mut self，将其生命周期分配给所有输出"]
-    R3 --> CHECK{{"All output lifetimes<br/>determined?"}}
+    R3 --> CHECK{{"All output lifetimes<br/>determined?<br/>所有输出生命周期都已确定？"}}
-    CHECK -->|"Yes"| OK["✅ No annotations needed"]
+    CHECK -->|"Yes / 是"| OK["✅ No annotations needed / 无需标注"]
-    CHECK -->|"No"| ERR["❌ Compile error:<br/>must annotate manually"]
+    CHECK -->|"No / 否"| ERR["❌ Compile error / 编译错误：<br/>必须手动标注"]
     
     style OK fill:#91e5a3,color:#000
     style ERR fill:#ff6b6b,color:#000
 ```
 
- ### Rule-by-Rule Examples
+ ### Rule-by-Rule Examples / 逐条规则示例
 
- **Rule 1** — each input reference gets its own lifetime parameter:
+ **规则 1** —— 每个输入引用获得自己的生命周期参数：
 ```rust
- // What you write:
+ // 你写的：
 fn first_word(s: &str) -> &str { ... }
 
- // What the compiler sees after Rule 1:
+ // 编译器在规则 1 之后看到的：
 fn first_word<'a>(s: &'a str) -> &str { ... }
- // Only one input lifetime → Rule 2 applies
+ // 只有一个输入生命周期 -> 应用规则 2
 ```
 
- **Rule 2** — single input lifetime propagates to all outputs:
+ **规则 2** —— 单一输入生命周期传播到所有输出：
 ```rust
- // After Rule 2:
+ // 规则 2 之后：
 fn first_word<'a>(s: &'a str) -> &'a str { ... }
- // ✅ All output lifetimes determined — no annotation needed!
+ // ✅ 所有输出生命周期已确定 —— 无需标注！
 ```
 
- **Rule 3** — `&self` lifetime propagates to outputs:
+ **规则 3** —— `&self` 生命周期传播到输出：
 ```rust
- // What you write:
+ // 你写的：
 impl SliceStore<'_> {
     fn get_slice(&self) -> &str { self.slice }
 }
 
- // What the compiler sees after Rules 1 + 3:
+ // 编译器在规则 1 + 3 之后看到的：
 impl SliceStore<'_> {
     fn get_slice<'a>(&'a self) -> &'a str { self.slice }
 }
- // ✅ No annotation needed — &self lifetime used for output
+ // ✅ 无需标注 —— 输出使用了 &self 的生命周期
 ```
 
- **When elision fails** — you must annotate:
+ **省略失败的情况** —— 你必须手动标注：
 ```rust
- // Two input references, no &self → Rules 2 and 3 don't apply
+ // 两个输入引用，且没有 &self -> 规则 2 和 3 不适用
 // fn longest(a: &str, b: &str) -> &str  ← WON'T COMPILE
 
- // Fix: tell the compiler which input the output borrows from
+ // 修复：告诉编译器输出借用了哪个输入
 fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
     if a.len() >= b.len() { a } else { b }
 }
 ```
 
- ### C Programmer Mental Model
+ ### C Programmer Mental Model / C 程序员的心理模型
 
- In C, every pointer is independent — the programmer mentally tracks which allocation
+ 在 C 中，每个指针都是独立的 —— 程序员需要在脑海中跟踪每个指针
- each pointer refers to, and the compiler trusts you completely. In Rust, lifetimes make
+ 指向哪个分配，而编译器完全信任你。在 Rust 中，生命周期使这种跟踪变为
- this tracking **explicit and compiler-verified**:
+ **显式的并经过编译器验证**：
 
-| C | Rust | What happens |
+| C | Rust | **What happens / 发生了什么** |
 |---|------|-------------|
-| `char* get_name(struct User* u)` | `fn get_name(&self) -> &str` | Rule 3 elides: output borrows from `self` |
+| `char* get_name(struct User* u)` | `fn get_name(&self) -> &str` | Rule 3 elides / 规则3省略：输出借用自 `self` |
-| `char* concat(char* a, char* b)` | `fn concat<'a>(a: &'a str, b: &'a str) -> &'a str` | Must annotate — two inputs |
+| `char* concat(char* a, char* b)` | `fn concat<'a>(a: &'a str, b: &'a str) -> &'a str` | Must annotate / 必须标注 —— 有两个输入 |
-| `void process(char* in, char* out)` | `fn process(input: &str, output: &mut String)` | No output reference — no lifetime needed |
+| `void process(char* in, char* out)` | `fn process(input: &str, output: &mut String)` | No output reference / 无输出引用 —— 无需生命周期 |
-| `char* buf; /* who owns this? */` | Compile error if lifetime is wrong | Compiler catches dangling pointers |
+| `char* buf; /* who owns this? */` | Compile error / 编译错误（如果生命周期错误） | Compiler catches dangling pointers / 编译器捕获悬垂指针 |
 
- ### The `'static` Lifetime
+ ### The `'static` Lifetime / `'static` 生命周期
 
- `'static` means the reference is valid for the **entire program duration**. It's the
+ `'static` 意味着引用在**整个程序运行期间**都有效。它相当于
- Rust equivalent of a C global or string literal:
+ Rust 中的 C 全局变量或字符串字面量：
 
 ```rust
- // String literals are always 'static — they live in the binary's read-only section
+ // String literals are always 'static / 字符串字面量始终是 'static —— 它们存在于二进制文件的只读段中
- let s: &'static str = "hello";  // Same as: static const char* s = "hello"; in C
+ let s: &'static str = "hello";  // 等同于 C 中的 static const char* s = "hello";
 
- // Constants are also 'static
+ // Constants are also 'static / 常量也是 'static
 static GREETING: &str = "hello";
 
- // Common in trait bounds for thread spawning:
+ // 在线程生成的 trait 约束中很常见：
 fn spawn<F: FnOnce() + Send + 'static>(f: F) { /* ... */ }
- // 'static here means: "the closure must not borrow any local variables"
+ // 这里的 'static 意味着：“闭包不得借用任何局部变量”
- // (either move them in, or use only 'static data)
+ // （要么把它们 move 进去，要么只使用 'static 数据）
 ```
 
- ### Exercise: Predict the Elision
+ ### Exercise: Predict the Elision / 练习：预测省略
 
- 🟡 **Intermediate**
+ 🟡 **Intermediate / 中级**
 
- For each function signature below, predict whether the compiler can elide lifetimes.
+ 对于下面每个函数签名，预测编译器是否可以省略生命周期。
- If not, add the necessary annotations:
+ 如果不能，请添加必要的标注：
 
 ```rust
- // 1. Can the compiler elide?
+ // 1. 编译器可以省略吗？
 fn trim_prefix(s: &str) -> &str { &s[1..] }
 
- // 2. Can the compiler elide?
+ // 2. 编译器可以省略吗？
 fn pick(flag: bool, a: &str, b: &str) -> &str {
     if flag { a } else { b }
 }
 
- // 3. Can the compiler elide?
+ // 3. 编译器可以省略吗？
 struct Parser { data: String }
 impl Parser {
     fn next_token(&self) -> &str { &self.data[..5] }
 }
 
- // 4. Can the compiler elide?
+ // 4. 编译器可以省略吗？
 fn split_at(s: &str, pos: usize) -> (&str, &str) {
     (&s[..pos], &s[pos..])
 }
 ```
 
- <details><summary>Solution (click to expand)</summary>
+ <details><summary>Solution (click to expand) / 解决方案（点击展开）</summary>
 
 ```rust,ignore
- // 1. YES — Rule 1 gives 'a to s, Rule 2 propagates to output
+ // 1. 是 —— 规则 1 给 s 分配 'a，规则 2 传播到输出
 fn trim_prefix(s: &str) -> &str { &s[1..] }
 
- // 2. NO — Two input references, no &self. Must annotate:
+ // 2. 否 —— 两个输入引用，且没有 &self。必须手动标注：
 fn pick<'a>(flag: bool, a: &'a str, b: &'a str) -> &'a str {
     if flag { a } else { b }
 }
 
- // 3. YES — Rule 1 gives 'a to &self, Rule 3 propagates to output
+ // 3. 是 —— 规则 1 给 &self 分配 'a，规则 3 传播到输出
 impl Parser {
     fn next_token(&self) -> &str { &self.data[..5] }
 }
 
- // 4. YES — Rule 1 gives 'a to s (only one input reference),
+ // 4. 是 —— 规则 1 给 s 分配 'a（只有一个输入引用），
- //    Rule 2 propagates to BOTH outputs. Both slices borrow from s.
+ //    规则 2 会传播到两个输出。两个切片都借用自 s。
 fn split_at(s: &str, pos: usize) -> (&str, &str) {
     (&s[..pos], &s[pos..])
 }
 ```
 
 </details>
