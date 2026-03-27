# Rust memory management / Rust 内存管理
 
 > **What you'll learn / 你将学到：** Rust's ownership system — the single most important concept in the language. After this chapter you'll understand move semantics, borrowing rules, and the `Drop` trait. If you grasp this chapter, the rest of Rust follows naturally. If you're struggling, re-read it — ownership clicks on the second pass for most C/C++ developers.
 >
 > Rust 的所有权系统 —— 该语言中最重要的单一概念。学完本章后，你将理解移动语义、借用规则以及 `Drop` trait。如果你掌握了本章内容，Rust 的其余部分就会自然而然地理解。如果你感到困惑，请重读一遍 —— 对于大多数 C/C++ 开发者来说，所有权概念通常在读第二遍时才会真正“开悟”。
 
 - Memory management in C/C++ is a source of bugs / C/C++ 中的内存管理是 bug 的温床：
-     - In C: memory is allocated with `malloc()` and freed with `free()`. No checks against dangling pointers, use-after-free, or double-free
+     - In C: memory is allocated with `malloc()` and freed with `free()`. No checks against dangling pointers, use-after-free, or double-free / 在 C 中：使用 `malloc()` 分配内存，使用 `free()` 释放内存。没有针对悬垂指针、读取已释放内存（use-after-free）或二次释放（double-free）的检查
-     - In C++: RAII (Resource Acquisition Is Initialization) and smart pointers help, but `std::move(ptr)` compiles even after the move — use-after-move is UB
+     - In C++: RAII (Resource Acquisition Is Initialization) and smart pointers help, but `std::move(ptr)` compiles even after the move — use-after-move is UB / 在 C++ 中：RAII（资源获取即初始化）和智能指针有所帮助，但 `std::move(ptr)` 在移动后仍然可以编译 —— 移动后使用（use-after-move）是未定义行为（UB）
 - Rust makes RAII **foolproof** / Rust 让 RAII 变得**万无一失**：
-     - Move is **destructive** — the compiler refuses to let you touch the moved-from variable
+     - Move is **destructive** — the compiler refuses to let you touch the moved-from variable / 移动是**破坏性**的 —— 编译器拒绝让你触碰已经移出的变量
-     - No Rule of Five needed (no copy ctor, move ctor, copy assign, move assign, destructor)
+     - No Rule of Five needed (no copy ctor, move ctor, copy assign, move assign, destructor) / 不需要遵守 “Rule of Five”（不需要定义拷贝构造、移动构造、拷贝赋值、移动赋值、析构函数）
-     - Rust gives complete control of memory allocation, but enforces safety at **compile time**
+     - Rust gives complete control of memory allocation, but enforces safety at **compile time** / Rust 赋予了对内存分配的完全控制权，但在**编译时**强制执行安全性
-     - This is done by a combination of mechanisms including ownership, borrowing, mutability and lifetimes
+     - This is done by a combination of mechanisms including ownership, borrowing, mutability and lifetimes / 这是由包括所有权、借用、可变性和生命周期在内的多种机制结合实现的
-     - Rust runtime allocations can happen both on the stack and the heap
+     - Rust runtime allocations can happen both on the stack and the heap / Rust 运行时分配既可以发生在栈上，也可以发生在堆上
 
- > **For C++ developers — Smart Pointer Mapping:**
+ > **For C++ developers — Smart Pointer Mapping / C++ 开发者 —— 智能指针映射：**
 >
-> | **C++** | **Rust** | **Safety Improvement** |
+| **C++** | **Rust** | **Safety Improvement / 安全提升** |
 |---------|----------|----------------------|
-| `std::unique_ptr<T>` | `Box<T>` | No use-after-move possible |
+| `std::unique_ptr<T>` | `Box<T>` | No use-after-move possible / 不可能出现移动后使用 |
-| `std::shared_ptr<T>` | `Rc<T>` (single-thread) | No reference cycles by default |
+| `std::shared_ptr<T>` | `Rc<T>` (single-thread) | No reference cycles by default / 默认无引用循环 |
-| `std::shared_ptr<T>` (thread-safe) | `Arc<T>` | Explicit thread-safety |
+| `std::shared_ptr<T>` (thread-safe) | `Arc<T>` | Explicit thread-safety / 显式的线程安全 |
-| `std::weak_ptr<T>` | `Weak<T>` | Must check validity |
+| `std::weak_ptr<T>` | `Weak<T>` | Must check validity / 必须检查有效性 |
-| Raw pointer | `*const T` / `*mut T` | Only in `unsafe` blocks |
+| Raw pointer / 原始指针 | `*const T` / `*mut T` | Only in `unsafe` blocks / 仅限 `unsafe` 块中使用 |
 >
- > For C developers: `Box<T>` replaces `malloc`/`free` pairs. `Rc<T>` replaces manual reference counting. Raw pointers exist but are confined to `unsafe` blocks.
+ > For C developers / 对于 C 开发者：`Box<T>` 取代了 `malloc`/`free` 配对。`Rc<T>` 取代了手动引用计数。原始指针存在，但被限制在 `unsafe` 块中。
 
- # Rust ownership, borrowing and lifetimes
+ # Rust ownership, borrowing and lifetimes / Rust 所有权、借用与生命周期
- - Recall that Rust only permits a single mutable reference to a variable and multiple read-only references
+ - Recall that Rust only permits a single mutable reference to a variable and multiple read-only references / 回想一下，Rust 只允许一个变量有一个可变引用或多个只读引用
-     - The initial declaration of the variable establishes ```ownership```
+     - The initial declaration of the variable establishes ```ownership``` / 变量的初始声明建立了```所有权（ownership）```
-     - Subsequent references ```borrow``` from the original owner. The rule is that the scope of the borrow can never exceed the owning scope. In other words, the ```lifetime``` of a borrow cannot exceed the owning lifetime
+     - Subsequent references ```borrow``` from the original owner. The rule is that the scope of the borrow can never exceed the owning scope. In other words, the ```lifetime``` of a borrow cannot exceed the owning lifetime / 随后的引用从原始所有者那里```借用（borrow）```。规则是借用的范围永远不能超过所有权范围。换句话说，借用的```生命周期（lifetime）```不能超过所有权的生命周期
 ```rust
 fn main() {
-    let a = 42; // Owner
+    let a = 42; // Owner / 所有者
-    let b = &a; // First borrow
+    let b = &a; // First borrow / 第一次借用
     {
         let aa = 42;
-        let c = &a; // Second borrow; a is still in scope
+        let c = &a; // Second borrow; a is still in scope / 第二次借用；a 仍在作用域内
-        // Ok: c goes out of scope here
+        // Ok: c goes out of scope here / OK：c 在这里离开作用域
-        // aa goes out of scope here
+        // aa goes out of scope here / aa 在这里离开作用域
     }
-    // let d = &aa; // Will not compile unless aa is moved to outside scope
+    // let d = &aa; // Will not compile / 无法编译，除非 aa 被移出到外部作用域
-    // b implicitly goes out of scope before a
+    // b implicitly goes out of scope before a / b 在 a 之前隐式离开作用域
-    // a goes out of scope last
+    // a goes out of scope last / a 最后离开作用域
 }
 ```
 
- - Rust can pass parameters to methods using several different mechanisms
+ - Rust can pass parameters to methods using several different mechanisms / Rust 可以使用几种不同的机制向方法传递参数
-     - By value (copy): Typically types that can be trivially copied (ex: u8, u32, i8, i32)
+     - By value (copy): Typically types that can be trivially copied (ex: u8, u32, i8, i32) / 按值传递（拷贝）：通常是那些可以被简单拷贝的类型（例如：u8, u32, i8, i32）
-     - By reference: This is the equivalent of passing a pointer to the actual value. This is also commonly known as borrowing, and the reference can be immutable (```&```), or mutable (```&mut```) 
+     - By reference: This is the equivalent of passing a pointer to the actual value. This is also commonly known as borrowing, and the reference can be immutable (```&```), or mutable (```&mut```) / 按引用传递：这相当于传递指向实际值的指针。这也通常被称为借用，引用可以是不可变的（```&```）或可变的（```&mut```）
-     - By moving: This transfers "ownership" of the value to the function. The caller can no longer reference the original value
+     - By moving: This transfers "ownership" of the value to the function. The caller can no longer reference the original value / 按移动传递：这会将值的“所有权”转移给函数。调用者不能再引用原始值
 ```rust
 fn foo(x: &u32) {
     println!("{x}");
 }
 fn bar(x: u32) {
     println!("{x}");
 }
 fn main() {
     let a = 42;
-    foo(&a);    // By reference
+    foo(&a);    // By reference / 按引用
-    bar(a);     // By value (copy)
+    bar(a);     // By value (copy) / 按值（拷贝）
 }
 ```
 
- - Rust prohibits dangling references from methods
+ - Rust 禁止从方法返回悬垂引用（dangling references）
-     - References returned by methods must still be in scope
+     - References returned by methods must still be in scope / 方法返回的引用必须仍在作用域内
-     - Rust will automatically ```drop``` a reference when it goes out of scope. 
+     - Rust will automatically ```drop``` a reference when it goes out of scope. / 引用离开作用域时，Rust 会自动将其 ```drop```（释放）。
 ```rust
 fn no_dangling() -> &u32 {
-    // lifetime of a begins here
+    // lifetime of a begins here / a 的生命周期在这里开始
     let a = 42;
-    // Won't compile. lifetime of a ends here
+    // Won't compile. lifetime of a ends here / 无法编译。a 的生命周期在这里结束
     &a
 }
 
 fn ok_reference(a: &u32) -> &u32 {
-    // Ok because the lifetime of a always exceeds ok_reference()
+    // Ok because the lifetime of a always exceeds ok_reference() / OK，因为 a 的生命周期总是超过 ok_reference()
     a
 }
 fn main() {
-    let a = 42;     // lifetime of a begins here
+    let a = 42;     // lifetime of a begins here / a 的生命周期在这里开始
     let b = ok_reference(&a);
-    // lifetime of b ends here
+    // lifetime of b ends here / b 的生命周期在这里结束
-    // lifetime of a ends here
+    // lifetime of a ends here / a 的生命周期在这里结束
 }
 ```
 
- # Rust move semantics
+ # Rust move semantics / Rust 移动语义
- - By default, Rust assignment transfers ownership
+ - By default, Rust assignment transfers ownership / 默认情况下，Rust 的赋值操作会转移所有权
 ```rust
 fn main() {
-    let s = String::from("Rust");    // Allocate a string from the heap
+    let s = String::from("Rust");    // Allocate a string from the heap / 从堆上分配一个字符串
-    let s1 = s; // Transfer ownership to s1. s is invalid at this point
+    let s1 = s; // Transfer ownership to s1. s is invalid at this point / 将所有权转移给 s1。此时 s 已失效
     println!("{s1}");
-    // This will not compile
+    // This will not compile / 以下代码无法编译
     //println!("{s}");
-    // s1 goes out of scope here and the memory is deallocated
+    // s1 goes out of scope here and the memory is deallocated / s1 在这里离开作用域，内存被释放
-    // s goes out of scope here, but nothing happens because it doesn't own anything
+    // s goes out of scope here, but nothing happens because it doesn't own anything / s 在这里离开作用域，但没有任何反应，因为它已不拥有任何东西
 }
 ```
 ```mermaid
 graph LR
-    subgraph "Before: let s1 = s"
+    subgraph "Before: let s1 = s / 之前"
-        S["s (stack)<br/>ptr"] -->|"owns"| H1["Heap: R u s t"]
+        S["s (stack)<br/>ptr / 指子"] -->|"owns / 指向"| H1["Heap / 堆: R u s t"]
     end
 
-    subgraph "After: let s1 = s"
+    subgraph "After: let s1 = s / 之后"
-        S_MOVED["s (stack)<br/>⚠️ MOVED"] -.->|"invalid"| H2["Heap: R u s t"]
+        S_MOVED["s (stack)<br/>⚠️ MOVED / 已移动"] -.->|"invalid / 无效"| H2["Heap / 堆: R u s t"]
-        S1["s1 (stack)<br/>ptr"] -->|"now owns"| H2
+        S1["s1 (stack)<br/>ptr / 指子"] -->|"now owns / 接手指向"| H2
     end
 
     style S_MOVED fill:#ff6b6b,color:#000,stroke:#333
     style S1 fill:#51cf66,color:#000,stroke:#333
     style H2 fill:#91e5a3,color:#000,stroke:#333
 ```
- *After `let s1 = s`, ownership transfers to `s1`. The heap data stays put — only the stack pointer moves. `s` is now invalid.*
+ *执行 `let s1 = s` 后，所有权转移到了 `s1`。堆上的数据不动 —— 仅仅是栈上的指针发生了移动。`s` 现已失效。*
 
 ----
- # Rust move semantics and borrowing
+ # Rust move semantics and borrowing / Rust 移动语义与借用
 ```rust
 fn foo(s : String) {
     println!("{s}");
-    // The heap memory pointed to by s will be deallocated here
+    // The heap memory pointed to by s will be deallocated here / s 指向的堆内存将在这里被释放
 }
 fn bar(s : &String) {
     println!("{s}");
-    // Nothing happens -- s is borrowed
+    // Nothing happens -- s is borrowed / 没发生什么 —— s 是借用的
 }
 fn main() {
-    let s = String::from("Rust string move example");    // Allocate a string from the heap
+    let s = String::from("Rust string move example");    // Allocate a string from the heap / 从堆上分配一个字符串
-    foo(s); // Transfers ownership; s is invalid now
+    foo(s); // Transfers ownership; s is invalid now / 转移所有权；s 现在已失效
-    // println!("{s}");  // will not compile
+    // println!("{s}");  // will not compile / 无法编译
-    let t = String::from("Rust string borrow example");
+    let t = String::from("Rust string borrow example"); // 借用示例
-    bar(&t);    // t continues to hold ownership
+    bar(&t);    // t continues to hold ownership / t 继续持有所有权
     println!("{t}"); 
 }
 ```
 
- # Rust move semantics and ownership
+ # Rust move semantics and ownership / Rust 移动语义与所有权
- - It is possible to transfer ownership by moving
+ - It is possible to transfer ownership by moving / 可以通过移动来转移所有权
-     - It is illegal to reference outstanding references after the move is completed
+     - It is illegal to reference outstanding references after the move is completed / 移动完成后，引用已失效的（外挂）引用是非法的
-     - Consider borrowing if a move is not desirable
+     - Consider borrowing if a move is not desirable / 如果不需要移动，请考虑借用
 ```rust
 struct Point {
     x: u32,
     y: u32,
 }
 fn consume_point(p: Point) {
     println!("{} {}", p.x, p.y);
 }
 fn borrow_point(p: &Point) {
     println!("{} {}", p.x, p.y);
 }
 fn main() {
     let p = Point {x: 10, y: 20};
-    // Try flipping the two lines
+    // Try flipping the two lines / 试着调整这两行的顺序
     borrow_point(&p);
     consume_point(p);
 }
 ```
 
- # Rust Clone
+ # Rust Clone / Rust 克隆
- - The ```clone()``` method can be used to copy the original memory. The original reference continues to be valid (the downside is that we have 2x the allocation)
+ - The ```clone()``` method can be used to copy the original memory. The original reference continues to be valid (the downside is that we have 2x the allocation) / ```clone()``` 方法可用于拷贝原始内存。原始引用继续有效（缺点是我们会有双倍的内存分配）
 ```rust
 fn main() {
-    let s = String::from("Rust");    // Allocate a string from the heap
+    let s = String::from("Rust");    // Allocate a string from the heap / 从堆上分配一个字符串
-    let s1 = s.clone(); // Copy the string; creates a new allocation on the heap
+    let s1 = s.clone(); // Copy the string; creates a new allocation on the heap / 拷贝字符串；在堆上创建一个新的分配
     println!("{s1}");  
     println!("{s}");
-    // s1 goes out of scope here and the memory is deallocated
+    // s1 goes out of scope here and the memory is deallocated / s1 在这里离开作用域，内存被释放
-    // s goes out of scope here, and the memory is deallocated
+    // s goes out of scope here, and the memory is deallocated / s 在这里离开作用域，内存被释放
 }
 ```
 ```mermaid
 graph LR
-    subgraph "After: let s1 = s.clone()"
+    subgraph "After: let s1 = s.clone() / 之后"
-        S["s (stack)<br/>ptr"] -->|"owns"| H1["Heap: R u s t"]
+        S["s (stack)<br/>ptr / 指针"] -->|"owns / 指向"| H1["Heap / 堆: R u s t"]
-        S1["s1 (stack)<br/>ptr"] -->|"owns (copy)"| H2["Heap: R u s t"]
+        S1["s1 (stack)<br/>ptr / 指针"] -->|"owns (copy) / 指向（拷贝）"| H2["Heap / 堆: R u s t"]
     end
 
     style S fill:#51cf66,color:#000,stroke:#333
     style S1 fill:#51cf66,color:#000,stroke:#333
     style H1 fill:#91e5a3,color:#000,stroke:#333
     style H2 fill:#91e5a3,color:#000,stroke:#333
 ```
- *`clone()` creates a **separate** heap allocation. Both `s` and `s1` are valid — each owns its own copy.*
+ *`clone()` 创建了一个**独立的**堆分配。`s` 和 `s1` 都有效 —— 它们各自拥有自己的拷贝。*
 
- # Rust Copy trait
+ # Rust Copy trait / Rust Copy Trait
- - Rust implements copy semantics for built-in types using the ```Copy``` trait
+ - Rust implements copy semantics for built-in types using the ```Copy``` trait / Rust 使用 ```Copy``` trait 为内置类型实现拷贝语义
-     - Examples include u8, u32, i8, i32, etc. Copy semantics use "pass by value"
+     - Examples include u8, u32, i8, i32, etc. Copy semantics use "pass by value" / 示例包括 u8, u32, i8, i32 等。拷贝语义使用“按值传递”
-     - User defined data types can optionally opt into ```copy``` semantics using the ```derive``` macro with to automatically implement the ```Copy``` trait
+     - User defined data types can optionally opt into ```copy``` semantics using the ```derive``` macro with to automatically implement the ```Copy``` trait / 用户定义的数据类型可以可选地通过使用 ```derive``` 宏自动实现 ```Copy``` trait 来加入 ```copy``` 语义
-     - The compiler will allocate space for the copy following a new assignment
+     - The compiler will allocate space for the copy following a new assignment / 在新的赋值操作后，编译器将为拷贝分配空间
 ```rust
- // Try commenting this out to see the change in let p1 = p; belw
+ // Try commenting this out to see the change in let p1 = p; below
+ // 试着注释掉这一行，看看下面 let p1 = p; 的变化
 #[derive(Copy, Clone, Debug)]   // We'll discuss this more later
 struct Point{x: u32, y:u32}
 fn main() {
     let p = Point {x: 42, y: 40};
-    let p1 = p;     // This will perform a copy now instead of move
+    let p1 = p;     // This will perform a copy now instead of move / 现在这将执行拷贝而不是移动
     println!("p: {p:?}");
     println!("p1: {p:?}");
-    let p2 = p1.clone();    // Semantically the same as copy
+    let p2 = p1.clone();    // Semantically the same as copy / 在语义上与拷贝相同
 }
 ```
 
- # Rust Drop trait
+ # Rust Drop trait / Rust Drop Trait
 
- - Rust automatically calls the `drop()` method at the end of scope
+ - Rust automatically calls the `drop()` method at the end of scope / Rust 在作用域结束时自动调用 `drop()` 方法
-     - `drop` is part of a generic trait called `Drop`. The compiler provides a blanket NOP implementation for all types, but types can override it. For example, the `String` type overrides it to release heap-allocated memory
+     - `drop` is part of a generic trait called `Drop`. The compiler provides a blanket NOP implementation for all types, but types can override it. For example, the `String` type overrides it to release heap-allocated memory / `drop` 是名为 `Drop` 的通用 trait 的一部分。编译器为所有类型提供了默认的 NOP（空操作）实现，但各类型可以覆盖它。例如，`String` 类型覆盖了它以释放堆分配的内存
-     - For C developers: this replaces the need for manual `free()` calls — resources are automatically released when they go out of scope (RAII)
+     - For C developers / 对于 C 开发者：这取代了手动调用 `free()` 的需要 —— 资源在离开作用域时会自动释放（RAII）
- - **Key safety:** You cannot call `.drop()` directly (the compiler forbids it). Instead, use `drop(obj)` which moves the value into the function, runs its destructor, and prevents any further use — eliminating double-free bugs
+ - **Key safety / 关键安全特性：** 你不能直接调用 `.drop()`（编译器禁止这样做）。相反，请使用 `drop(obj)`，这会将值移动到函数中，运行其析构函数，并阻止进一步使用 —— 从而消除了二次释放 bug
 
- > **For C++ developers:** `Drop` maps directly to C++ destructors (`~ClassName()`):
+ > **For C++ developers — Drop Mapping / C++ 开发者 —— Drop 映射：** `Drop` 直接映射到 C++ 析构函数（`~ClassName()`）：
 >
-| | **C++ destructor** | **Rust `Drop`** |
+| | **C++ destructor / C++ 析构函数** | **Rust `Drop`** |
 |---|---|---|
-| **Syntax** | `~MyClass() { ... }` | `impl Drop for MyType { fn drop(&mut self) { ... } }` |
+| **Syntax / 语法** | `~MyClass() { ... }` | `impl Drop for MyType { fn drop(&mut self) { ... } }` |
-| **When called** | End of scope (RAII) | End of scope (same) |
+| **When called / 何时调用** | End of scope (RAII) / 作用域结束 | End of scope (same) / 同上 |
-| **Called on move** | Source left in "valid but unspecified" state — destructor still runs on the moved-from object | Source is **gone** — no destructor call on moved-from value |
+| **Called on move / 在移动时调用** | Source left in "valid but unspecified" state — destructor still runs on the moved-from object / 源对象处于“有效但未指定”状态 —— 析构函数仍会在已移出的对象上运行 | Source is **gone** — no destructor call on moved-from value / 源对象**已消失** —— 不会在已移出的值上调用析构函数 |
-| **Manual call** | `obj.~MyClass()` (dangerous, rarely used) | `drop(obj)` (safe — takes ownership, calls `drop`, prevents further use) |
+| **Manual call / 手动调用** | `obj.~MyClass()` (dangerous, rarely used) / 危险且罕见 | `drop(obj)` (safe — takes ownership, calls `drop`, prevents further use) / 安全 —— 获取所有权，调用 `drop`，阻止后续使用 |
-| **Order** | Reverse declaration order | Reverse declaration order (same) |
+| **Order / 顺序** | Reverse declaration order / 与声明顺序相反 | Reverse declaration order (same) / 同上 |
-| **Rule of Five** | Must manage copy ctor, move ctor, copy assign, move assign, destructor | Only `Drop` — compiler handles move semantics, and `Clone` is opt-in |
+| **Rule of Five** | Must manage copy ctor, move ctor, copy assign, move assign, destructor / 必须管理拷贝/移动构造、拷贝/移动赋值、析构函数 | Only `Drop` — compiler handles move semantics, and `Clone` is opt-in / 只有 `Drop` —— 编译器处理移动语义，`Clone` 是可选加入的 |
-| **Virtual dtor needed?** | Yes, if deleting through base pointer | No — no inheritance, so no slicing problem |
+| **Virtual dtor needed? / 需要虚析构函数？** | Yes / 是，如果通过基类指针删除 | No / 否 —— 没有继承，所以没有对象切割问题 |
 
 ```rust
 struct Point {x: u32, y:u32}
 
- // Equivalent to: ~Point() { printf("Goodbye point x:%u, y:%u\n", x, y); }
+ // Equivalent to / 等同于：~Point() { printf("Goodbye point x:%u, y:%u\n", x, y); }
 impl Drop for Point {
     fn drop(&mut self) {
         println!("Goodbye point x:{}, y:{}", self.x, self.y);
     }
 }
 fn main() {
     let p = Point{x: 42, y: 42};
     {
         let p1 = Point{x:43, y: 43};
         println!("Exiting inner block");
-        // p1.drop() called here — like C++ end-of-scope destructor
+        // p1.drop() called here — like C++ end-of-scope destructor / p1.drop() 在这里被调用 —— 类似于 C++ 的作用域结束析构函数
     }
     println!("Exiting main");
-    // p.drop() called here
+    // p.drop() called here / p.drop() 在这里被调用
 }
 ```
 
- # Exercise: Move, Copy and Drop
+ # Exercise: Move, Copy and Drop / 练习：移动、拷贝与释放
 
- 🟡 **Intermediate** — experiment freely; the compiler will guide you
+ 🟡 **Intermediate / 中级** —— 自由实验；编译器会引导你
- - Create your own experiments with ```Point``` with and without ```Copy``` in ```#[derive(Debug)]``` in the below make sure you understand the differences. The idea is to get a solid understanding of how move vs. copy works, so make sure to ask
+ - Create your own experiments with ```Point``` with and without ```Copy``` in ```#[derive(Debug)]``` in the below make sure you understand the differences. The idea is to get a solid understanding of how move vs. copy works, so make sure to ask / 用带有和不带有 ```Copy``` 的 ```Point``` 进行你自己的实验，确保你理解其中的差异。其目的是让你牢固理解移动与拷贝的工作原理。
- - Implement a custom ```Drop``` for ```Point``` that sets x and y to 0 in ```drop```. This is a pattern that's useful for releasing locks and other resources for example
+ - Implement a custom ```Drop``` for ```Point``` that sets x and y to 0 in ```drop```. This is a pattern that's useful for releasing locks and other resources for example / 为 ```Point``` 实现一个自定义的 ```Drop```，在 ```drop``` 中将 x 和 y 设置为 0。例如，这是一种对于释放锁和其他资源非常有用的模式。
 ```rust
 struct Point{x: u32, y: u32}
 fn main() {
     // Create Point, assign it to a different variable, create a new scope,
     // pass point to a function, etc.
+    // 创建 Point，将其赋值给不同的变量，创建一个新作用域，
+    // 将 point 传递给函数等。
 }
 ```
 
- <details><summary>Solution (click to expand)</summary>
+ <details><summary>Solution (click to expand) / 解决方案（点击展开）</summary>
 
 ```rust
 #[derive(Debug)]
 struct Point { x: u32, y: u32 }
 
 impl Drop for Point {
     fn drop(&mut self) {
         println!("Dropping Point({}, {})", self.x, self.y);
         self.x = 0;
         self.y = 0;
-        // Note: setting to 0 in drop demonstrates the pattern,
+        // Note: setting to 0 in drop demonstrates the pattern, / 注意：在 drop 中设置为 0 只是为了演示该模式
-        // but you can't observe these values after drop completes
+        // but you can't observe these values after drop completes / 但在 drop 完成后你无法观察到这些值
     }
 }
 
 fn consume(p: Point) {
     println!("Consuming: {:?}", p);
-    // p is dropped here
+    // p is dropped here / p 在这里被 drop
 }
 
 fn main() {
     let p1 = Point { x: 10, y: 20 };
-    let p2 = p1;  // Move — p1 is no longer valid
+    let p2 = p1;  // Move — p1 is no longer valid / 移动 —— p1 现已失效
-    // println!("{:?}", p1);  // Won't compile: p1 was moved
+    // println!("{:?}", p1);  // Won't compile: p1 was moved / 无法编译：p1 已被移动
 
     {
         let p3 = Point { x: 30, y: 40 };
         println!("p3 in inner scope: {:?}", p3);
-        // p3 is dropped here (end of scope)
+        // p3 is dropped here (end of scope) / p3 在这里被 drop（作用域结束）
     }
 
-    consume(p2);  // p2 is moved into consume and dropped there
+    consume(p2);  // p2 is moved into consume and dropped there / p2 被移动到 consume 函数中并在那里被 drop
-    // println!("{:?}", p2);  // Won't compile: p2 was moved
+    // println!("{:?}", p2);  // Won't compile: p2 was moved / 无法编译：p2 已被移动
 
-    // Now try: add #[derive(Copy, Clone)] to Point (and remove the Drop impl)
+    // Now try: add #[derive(Copy, Clone)] to Point (and remove the Drop impl) / 现在尝试：给 Point 添加 #[derive(Copy, Clone)]（并移除 Drop 实现）
-    // and observe how p1 remains valid after let p2 = p1;
+    // and observe how p1 remains valid after let p2 = p1; / 然后观察在执行 let p2 = p1; 后 p1 如何保持有效
 }
- // Output:
+ // Output / 输出：
 // p3 in inner scope: Point { x: 30, y: 40 }
 // Dropping Point(30, 40)
 // Consuming: Point { x: 10, y: 20 }
 // Dropping Point(10, 20)
 ```
 
 </details>
