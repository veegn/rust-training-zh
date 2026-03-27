# Rust `Box<T>` / Rust `Box<T>`
 
 > **What you'll learn / 你将学到：** Rust's smart pointer types — `Box<T>` for heap allocation, `Rc<T>` for shared ownership, and `Cell<T>`/`RefCell<T>` for interior mutability. These build on the ownership and lifetime concepts from the previous sections. You'll also see a brief introduction to `Weak<T>` for breaking reference cycles.
 >
 > Rust 的智能指针类型 —— 用于堆分配的 `Box<T>`、用于共享所有权的 `Rc<T>`，以及用于内部可变性的 `Cell<T>`/`RefCell<T>`。这些都建立在前面章节的所有权和生命周期概念之上。你还将看到通过 `Weak<T>` 打破引用循环的简要介绍。
 
- **Why `Box<T>`?** In C, you use `malloc`/`free` for heap allocation. In C++, `std::unique_ptr<T>` wraps `new`/`delete`. Rust's `Box<T>` is the equivalent — a heap-allocated, single-owner pointer that is automatically freed when it goes out of scope. Unlike `malloc`, there's no matching `free` to forget. Unlike `unique_ptr`, there's no use-after-move — the compiler prevents it entirely.
+ **Why `Box<T>`? / 为什么使用 `Box<T>`？** 在 C 中，你使用 `malloc`/`free` 进行堆分配。在 C++ 中，`std::unique_ptr<T>` 包装了 `new`/`delete`。Rust 的 `Box<T>` 是其等效物 —— 一个堆分配的、单一所有者的指针，当它离开作用域时会自动释放。与 `malloc` 不同，没有配套的 `free` 需要去操心。与 `unique_ptr` 不同，没有“移动后使用（use-after-move）”的问题 —— 编译器完全阻止了这种情况。
 
- **When to use `Box` vs stack allocation:**
+ **When to use `Box` vs stack allocation / 何时使用 `Box` 而不是栈分配：**
- - The contained type is large and you don't want to copy it on the stack
+ - The contained type is large and you don't want to copy it on the stack / 包含的类型很大，你不想在栈上拷贝它
- - You need a recursive type (e.g., a linked list node that contains itself)
+ - You need a recursive type (e.g., a linked list node that contains itself) / 你需要一个递归类型（例如，包含自身的链表节点）
- - You need trait objects (`Box<dyn Trait>`)
+ - You need trait objects (`Box<dyn Trait>`) / 你需要 trait 对象（`Box<dyn Trait>`）
 
- - ```Box<T>``` can be use to create a pointer to a heap allocated type. The pointer is always a fixed size regardless of the type of ```<T>```
+ - ```Box<T>``` can be use to create a pointer to a heap allocated type. The pointer is always a fixed size regardless of the type of ```<T>``` / ```Box<T>``` 可用于创建指向堆分配类型的指针。无论 ```<T>``` 是什么类型，指针的大小始终是固定的
 ```rust
 fn main() {
-    // Creates a pointer to an integer (with value 42) created on the heap
+    // Creates a pointer to an integer (with value 42) created on the heap / 创建指向堆上整数（值为 42）的指针
     let f = Box::new(42);
     println!("{} {}", *f, f);
-    // Cloning a box creates a new heap allocation
+    // Cloning a box creates a new heap allocation / 克隆 box 会创建一个新的堆分配
     let mut g = f.clone();
     *g = 43;
     println!("{f} {g}");
-    // g and f go out of scope here and are automatically deallocated
+    // g and f go out of scope here and are automatically deallocated / g 和 f 在这里离开作用域并自动释放
 }
 ```
 ```mermaid
 graph LR
-    subgraph "Stack"
+    subgraph "Stack / 栈"
-        F["f: Box&lt;i32&gt;"]
+        F["f: Box&lt;i32&gt; / 指针"]
-        G["g: Box&lt;i32&gt;"]
+        G["g: Box&lt;i32&gt; / 指针"]
     end
 
-    subgraph "Heap"
+    subgraph "Heap / 堆"
         HF["42"]
         HG["43"]
     end
 
-    F -->|"owns"| HF
+    F -->|"owns / 指向"| HF
-    G -->|"owns (cloned)"| HG
+    G -->|"owns (cloned) / 指向（克隆）"| HG
 
     style F fill:#51cf66,color:#000,stroke:#333
     style G fill:#51cf66,color:#000,stroke:#333
     style HF fill:#91e5a3,color:#000,stroke:#333
     style HG fill:#91e5a3,color:#000,stroke:#333
 ```
 
- ## Ownership and Borrowing Visualization
+ ## Ownership and Borrowing Visualization / 所有权与借用可视化
 
- ### C/C++ vs Rust: Pointer and Ownership Management
+ ### C/C++ vs Rust: Pointer and Ownership Management / C/C++ vs Rust：指针与所有权管理
 
 ```c
 // C - Manual memory management, potential issues
+// C - 手动内存管理，潜在问题
 void c_pointer_problems() {
     int* ptr1 = malloc(sizeof(int));
     *ptr1 = 42;
     
-    int* ptr2 = ptr1;  // Both point to same memory
+    int* ptr2 = ptr1;  // Both point to same memory / 两个指针指向同一块内存
-    int* ptr3 = ptr1;  // Three pointers to same memory
+    int* ptr3 = ptr1;  // Three pointers to same memory / 三个指针指向同一块内存
     
-    free(ptr1);        // Frees the memory
+    free(ptr1);        // Frees the memory / 释放内存
     
-    *ptr2 = 43;        // Use after free - undefined behavior!
+    *ptr2 = 43;        // Use after free / 释放后使用 —— 未定义行为！
-    *ptr3 = 44;        // Use after free - undefined behavior!
+    *ptr3 = 44;        // Use after free / 释放后使用 —— 未定义行为！
 }
 ```
 
- > **For C++ developers:** Smart pointers help, but don't prevent all issues:
+ > **For C++ developers / C++ 开发者注意：** 智能指针有所帮助，但不能防止所有问题：
 >
 > ```cpp
 // C++ - Smart pointers help, but don't prevent all issues
+// C++ - 智能指针有所帮助，但不能防止所有问题
 void cpp_pointer_issues() {
     auto ptr1 = std::make_unique<int>(42);
     
-    // auto ptr2 = ptr1;  // Compile error: unique_ptr not copyable
+    // auto ptr2 = ptr1;  // Compile error / 编译错误：unique_ptr 不可拷贝
-    auto ptr2 = std::move(ptr1);  // OK: ownership transferred
+    auto ptr2 = std::move(ptr1);  // OK: ownership transferred / OK：所有权转移
 >     
-    // But C++ still allows use-after-move:
+    // But C++ still allows use-after-move / 但 C++ 仍然允许移动后使用：
-    // std::cout << *ptr1;  // Compiles! But undefined behavior!
+    // std::cout << *ptr1;  // Compiles! / 能编译！但会导致未定义行为！
 >     
-    // shared_ptr aliasing:
+    // shared_ptr aliasing / shared_ptr 别名：
     auto shared1 = std::make_shared<int>(42);
-    auto shared2 = shared1;  // Both own the data
+    auto shared2 = shared1;  // Both own the data / 两者都拥有数据
-    // Who "really" owns it? Neither. Ref count overhead everywhere.
+    // Who "really" owns it? Neither. Ref count overhead everywhere. / 谁是“真正”的所有者？都不是。引用计数的开销到处都是。
 > }
 > ```
 
 ```rust
 // Rust - Ownership system prevents these issues
+// Rust - 所有权系统防止了这些问题
 fn rust_ownership_safety() {
-    let data = Box::new(42);  // data owns the heap allocation
+    let data = Box::new(42);  // data owns the heap allocation / data 拥有堆分配的所有权
     
-    let moved_data = data;    // Ownership transferred to moved_data
+    let moved_data = data;    // Ownership transferred / 所有权转移给 moved_data
-    // data is no longer accessible - compile error if used
+    // data is no longer accessible / data 不再可访问 —— 如果使用会导致编译错误
     
-    let borrowed = &moved_data;  // Immutable borrow
+    let borrowed = &moved_data;  // Immutable borrow / 不可变借用
-    println!("{}", borrowed);    // Safe to use
+    println!("{}", borrowed);    // Safe to use / 安全使用
     
-    // moved_data automatically freed when it goes out of scope
+    // moved_data automatically freed when it goes out of scope / moved_data 在离开作用域时自动释放
 }
 ```
 
 ```mermaid
 graph TD
-    subgraph "C/C++ Memory Management Issues"
+    subgraph "C/C++ Memory Management Issues / 内存管理问题"
-        CP1["int* ptr1"] --> CM["Heap Memory<br/>value: 42"]
+        CP1["int* ptr1 / 指针"] --> CM["Heap Memory / 堆内存<br/>value: 42"]
-        CP2["int* ptr2"] --> CM
+        CP2["int* ptr2 / 指针"] --> CM
-        CP3["int* ptr3"] --> CM
+        CP3["int* ptr3 / 指针"] --> CM
-        CF["free(ptr1)"] --> CM_F["[ERROR] Freed Memory"]
+        CF["free(ptr1) / 释放"] --> CM_F["[ERROR] Freed Memory / 已释放内存"]
-        CP2 -.->|"Use after free<br/>Undefined Behavior"| CM_F
+        CP2 -.->|"Use after free / 释放后使用<br/>Undefined Behavior / 未定义行为"| CM_F
-        CP3 -.->|"Use after free<br/>Undefined Behavior"| CM_F
+        CP3 -.->|"Use after free / 释放后使用<br/>Undefined Behavior / 未定义行为"| CM_F
     end
     
-    subgraph "Rust Ownership System"
+    subgraph "Rust Ownership System / 所有权系统"
-        RO1["data: Box<i32>"] --> RM["Heap Memory<br/>value: 42"]
+        RO1["data: Box<i32> / 所有者"] --> RM["Heap Memory / 堆内存<br/>value: 42"]
-        RO1 -.->|"Move ownership"| RO2["moved_data: Box<i32>"]
+        RO1 -.->|"Move ownership / 移动所有权"| RO2["moved_data: Box<i32> / 所有者"]
         RO2 --> RM
-        RO1_X["data: [WARNING] MOVED<br/>Cannot access"]
+        RO1_X["data: [WARNING] MOVED / 已移动<br/>Cannot access / 无法访问"]
-        RB["&moved_data<br/>Immutable borrow"] -.->|"Safe reference"| RM
+        RB["&moved_data / 借用<br/>Immutable borrow / 不可变"] -.->|"Safe reference / 安全引用"| RM
-        RD["Drop automatically<br/>when out of scope"] --> RM
+        RD["Drop automatically / 自动释放<br/>when out of scope / 离开作用域时"] --> RM
     end
     
     style CM_F fill:#ff6b6b,color:#000
     style CP2 fill:#ff6b6b,color:#000
     style CP3 fill:#ff6b6b,color:#000
     style RO1_X fill:#ffa07a,color:#000
     style RO2 fill:#51cf66,color:#000
     style RB fill:#91e5a3,color:#000
     style RD fill:#91e5a3,color:#000
 ```
 
- ### Borrowing Rules Visualization
+ ### Borrowing Rules Visualization / 借用规则可视化
 
 ```rust
 fn borrowing_rules_example() {
     let mut data = vec![1, 2, 3, 4, 5];
     
-    // Multiple immutable borrows - OK
+    // Multiple immutable borrows - OK / 多个不可变借用 —— OK
     let ref1 = &data;
     let ref2 = &data;
-    println!("{:?} {:?}", ref1, ref2);  // Both can be used
+    println!("{:?} {:?}", ref1, ref2);  // Both can be used / 两者都可以使用
     
-    // Mutable borrow - exclusive access
+    // Mutable borrow - exclusive access / 可变借用 —— 排他性访问
     let ref_mut = &mut data;
     ref_mut.push(6);
-    // ref1 and ref2 can't be used while ref_mut is active
+    // ref1 and ref2 can't be used while ref_mut is active / 当 ref_mut 处于活跃状态时，不能使用 ref1 和 ref2
     
-    // After ref_mut is done, immutable borrows work again
+    // After ref_mut is done, immutable borrows work again / ref_mut 结束后，不可变借用再次生效
     let ref3 = &data;
     println!("{:?}", ref3);
 }
 ```
 
 ```mermaid
 graph TD
-    subgraph "Rust Borrowing Rules"
+    subgraph "Rust Borrowing Rules / Rust 借用规则"
         D["mut data: Vec<i32>"]
         
-        subgraph "Phase 1: Multiple Immutable Borrows [OK]"
+        subgraph "Phase 1: Multiple Immutable Borrows [OK] / 阶段 1：多个不可变借用"
             IR1["&data (ref1)"]
             IR2["&data (ref2)"]
             D --> IR1
             D --> IR2
-            IR1 -.->|"Read-only access"| MEM1["Memory: [1,2,3,4,5]"]
+            IR1 -.->|"Read-only access / 只读访问"| MEM1["Memory / 内存: [1,2,3,4,5]"]
-            IR2 -.->|"Read-only access"| MEM1
+            IR2 -.->|"Read-only access / 只读访问"| MEM1
         end
         
-        subgraph "Phase 2: Exclusive Mutable Borrow [OK]"
+        subgraph "Phase 2: Exclusive Mutable Borrow [OK] / 阶段 2：排他性可变借用"
             MR["&mut data (ref_mut)"]
             D --> MR
-            MR -.->|"Exclusive read/write"| MEM2["Memory: [1,2,3,4,5,6]"]
+            MR -.->|"Exclusive read/write / 排他性读写"| MEM2["Memory / 内存: [1,2,3,4,5,6]"]
-            BLOCK["[ERROR] Other borrows blocked"]
+            BLOCK["[ERROR] Other borrows blocked / 其他借用被阻塞"]
         end
         
-        subgraph "Phase 3: Immutable Borrows Again [OK]"
+        subgraph "Phase 3: Immutable Borrows Again [OK] / 阶段 3：再次允许不可变借用"
             IR3["&data (ref3)"]
             D --> IR3
-            IR3 -.->|"Read-only access"| MEM3["Memory: [1,2,3,4,5,6]"]
+            IR3 -.->|"Read-only access / 只读访问"| MEM3["Memory / 内存: [1,2,3,4,5,6]"]
         end
     end
     
-    subgraph "What C/C++ Allows (Dangerous)"
+    subgraph "What C/C++ Allows (Dangerous) / C/C++ 允许的行为（危险）"
         CP["int* ptr"]
         CP2["int* ptr2"]
         CP3["int* ptr3"]
-        CP --> CMEM["Same Memory"]
+        CP --> CMEM["Same Memory / 同一内存"]
         CP2 --> CMEM
         CP3 --> CMEM
-        RACE["[ERROR] Data races possible<br/>[ERROR] Use after free possible"]
+        RACE["[ERROR] Data races possible / 存在数据竞争可能<br/>[ERROR] Use after free possible / 存在释放后使用可能"]
     end
     
     style MEM1 fill:#91e5a3,color:#000
     style MEM2 fill:#91e5a3,color:#000
     style MEM3 fill:#91e5a3,color:#000
     style BLOCK fill:#ffa07a,color:#000
     style RACE fill:#ff6b6b,color:#000
     style CMEM fill:#ff6b6b,color:#000
 ```
 
 ---
 
- ## Interior Mutability: `Cell<T>` and `RefCell<T>`
+ ## Interior Mutability: `Cell<T>` and `RefCell<T>` / 内部可变性：`Cell<T>` 与 `RefCell<T>`
 
- Recall that by default variables are immutable in Rust. Sometimes it's desirable to have most of a type read-only while permitting write access to a single field.
+ 回想一下，在 Rust 中，变量默认是不可变的。有时我们希望该类型的其余部分都是只读的，但允许修改单个字段。
 
 ```rust
 struct Employee {
-    employee_id : u64,   // This must be immutable
+    employee_id : u64,   // This must be immutable / 这里必须是不可变的
-    on_vacation: bool,   // What if we wanted to permit write-access to this field, but make employee_id immutable?
+    on_vacation: bool,   // 如果我们想允许修改此字段，但保持 employee_id 不可变呢？
 }
 ```
 
- - Recall that Rust permits a *single mutable* reference to a variable and any number of *immutable* references — enforced at *compile-time*
+ - 回想一下，Rust 允许一个变量有一个*可变*引用或任意数量的*不可变*引用 —— 这是在*编译时*强制执行的
- - What if we wanted to pass an *immutable* vector of employees, *but* allow the `on_vacation` field to be updated, while ensuring `employee_id` cannot be mutated?
+ - 如果我们想传递一个*不可变*的员工 vector，*但是*允许更新 `on_vacation` 字段，同时确保 `employee_id` 不能被修改呢？
 
- ### `Cell<T>` — interior mutability for Copy types
+ ### `Cell<T>` — interior mutability for Copy types / `Cell<T>` —— Copy 类型的内部可变性
 
- - `Cell<T>` provides **interior mutability**, i.e., write access to specific elements of references that are otherwise read-only
+ - `Cell<T>` 提供**内部可变性（interior mutability）**，即允许修改原本只读的引用的特定元素
- - Works by copying values in and out (requires `T: Copy` for `.get()`)
+ - 通过拷入和拷出值来工作（对于 `.get()`，需要 `T: Copy`）
 
- ### `RefCell<T>` — interior mutability with runtime borrow checking
+ ### `RefCell<T>` — interior mutability with runtime borrow checking / `RefCell<T>` —— 带有运行时借用检查的内部可变性
 
- - `RefCell<T>` provides a variation that works with references
+ - `RefCell<T>` 提供了一种适用于引用的变体
-     - Enforces Rust borrow-checks at **runtime** instead of compile-time
+     - 在**运行时**而非编译时强制执行 Rust 的借用检查
-     - Allows a single *mutable* borrow, but **panics** if there are any other references outstanding
+     - 允许单个*可变*借用，但如果有任何其他引用存在，则会 **panic**
-     - Use `.borrow()` for immutable access and `.borrow_mut()` for mutable access
+     - 使用 `.borrow()` 进行不可变访问，使用 `.borrow_mut()` 进行可变访问
 
- ### When to Choose `Cell` vs `RefCell`
+ ### When to Choose `Cell` vs `RefCell` / 如何选择 `Cell` 或 `RefCell`
 
-| Criterion | `Cell<T>` | `RefCell<T>` |
+| **Criterion / 标准** | **`Cell<T>`** | **`RefCell<T>`** |
 |-----------|-----------|-------------|
-| Works with | `Copy` types (integers, bools, floats) | Any type (`String`, `Vec`, structs) |
+| **Works with / 适用场景** | `Copy` types (integers, bools, floats) / `Copy` 类型 | Any type (`String`, `Vec`, structs) / 任何类型 |
-| Access pattern | Copies values in/out (`.get()`, `.set()`) | Borrows in place (`.borrow()`, `.borrow_mut()`) |
+| **Access pattern / 访问模式** | Copies values in/out / 拷入/拷出值 | Borrows in place / 现场借用 |
-| Failure mode | Cannot fail — no runtime checks | **Panics** if you borrow mutably while another borrow is active |
+| **Failure mode / 失败模式** | Cannot fail / 不会失败 (no runtime checks) | **Panics** / 会 Panic（如果已有其他借用时再次尝试可变借用） |
-| Overhead | Zero — just copies bytes | Small — tracks borrow state at runtime |
+| **Overhead / 开销** | Zero / 零 (just copies bytes) | Small / 微小 (tracks borrow state at runtime) |
-| Use when | You need a mutable flag, counter, or small value inside an immutable struct | You need to mutate a `String`, `Vec`, or complex type inside an immutable struct |
+| **Use when / 何时使用** | You need a mutable flag, counter... / 需要不可变结构体内的可变标志或计数器 | You need to mutate complex types... / 需要修改不可变结构体内的复杂类型 |
 
 ---
 
- ## Shared Ownership: `Rc<T>`
+ ## Shared Ownership: `Rc<T>` / 共享所有权：`Rc<T>`
 
- `Rc<T>` allows reference-counted shared ownership of *immutable* data. What if we wanted to store the same `Employee` in multiple places without copying?
+ `Rc<T>` 允许对*不可变*数据进行引用计数共享所有权。如果我们想在多个地方存储同一个 `Employee` 而不进行拷贝呢？
 
 ```rust
 #[derive(Debug)]
 struct Employee {
     employee_id: u64,
 }
 fn main() {
     let mut us_employees = vec![];
     let mut all_global_employees = Vec::<Employee>::new();
     let employee = Employee { employee_id: 42 };
     us_employees.push(employee);
-    // Won't compile — employee was already moved
+    // Won't compile — employee was already moved / 无法编译 —— employee 已被移动
     //all_global_employees.push(employee);
 }
 ```
 
- `Rc<T>` solves the problem by allowing shared *immutable* access:
+ `Rc<T>` 通过允许共享的*不可变*访问来解决此问题：
- - The contained type is automatically dereferenced
+ - 包含的类型会被自动解引用
- - The type is dropped when the reference count goes to 0
+ - 当引用计数归零时，该类型会被释放（drop）
 
 ```rust
 use std::rc::Rc;
 #[derive(Debug)]
 struct Employee {employee_id: u64}
 fn main() {
     let mut us_employees = vec![];
     let mut all_global_employees = vec![];
     let employee = Employee { employee_id: 42 };
     let employee_rc = Rc::new(employee);
-    us_employees.push(employee_rc.clone());
+    us_employees.push(employee_rc.clone()); // 克隆 Rc 句柄
-    all_global_employees.push(employee_rc.clone());
+    all_global_employees.push(employee_rc.clone()); // 克隆 Rc 句柄
-    let employee_one = all_global_employees.get(0); // Shared immutable reference
+    let employee_one = all_global_employees.get(0); // Shared immutable reference / 共享的不可变引用
     for e in us_employees {
-        println!("{}", e.employee_id);  // Shared immutable reference
+        println!("{}", e.employee_id);  // Shared immutable reference / 共享的不可变引用
     }
     println!("{employee_one:?}");
 }
 ```
 
- > **For C++ developers: Smart Pointer Mapping**
+ > **For C++ developers: Smart Pointer Mapping / C++ 开发者：智能指针映射**
 >
-| C++ Smart Pointer | Rust Equivalent | Key Difference |
+| **C++ Smart Pointer / C++ 智能指针** | **Rust Equivalent / Rust 等等价物** | **Key Difference / 关键区别** |
 |---|---|---|
-| `std::unique_ptr<T>` | `Box<T>` | Rust's version is the default — move is language-level, not opt-in |
+| `std::unique_ptr<T>` | `Box<T>` | Rust's version is the default / Rust 的版本是默认行为 —— 移动是语言层面的 |
-| `std::shared_ptr<T>` | `Rc<T>` (single-thread) / `Arc<T>` (multi-thread) | No atomic overhead for `Rc`; use `Arc` only when sharing across threads |
+| `std::shared_ptr<T>` | `Rc<T>` (single-thread) / `Arc<T>` (multi-thread) | No atomic overhead for `Rc` / `Rc` 没有任何原子操作开销；仅在跨线程共享时才使用 `Arc` |
-| `std::weak_ptr<T>` | `Weak<T>` (from `Rc::downgrade()` or `Arc::downgrade()`) | Same purpose: break reference cycles |
+| `std::weak_ptr<T>` | `Weak<T>` | Same purpose: break reference cycles / 目的相同：打破引用循环 |
 >
- > **Key distinction**: In C++, you *choose* to use smart pointers. In Rust, owned values (`T`) and borrowing (`&T`) cover most use cases — reach for `Box`/`Rc`/`Arc` only when you need heap allocation or shared ownership.
+ > **Key distinction / 关键区别**：在 C++ 中，你*选择*去使用智能指针。在 Rust 中，拥有所有权的值（`T`）和借用（`&T`）涵盖了大多数用例 —— 只有当你确实需要堆分配或共享所有权时，才会求助于 `Box`/`Rc`/`Arc`。
 
- ### Breaking Reference Cycles with `Weak<T>`
+ ### Breaking Reference Cycles with `Weak<T>` / 使用 `Weak<T>` 打破引用循环
 
- `Rc<T>` uses reference counting — if two `Rc` values point to each other, neither will ever be dropped (a cycle). `Weak<T>` solves this:
+ `Rc<T>` 使用引用计数 —— 如果两个 `Rc` 值互相指向对方，它们都永远不会被释放（产生循环）。`Weak<T>` 解决了这个问题：
 
 ```rust
 use std::rc::{Rc, Weak};
 
 struct Node {
     value: i32,
-    parent: Option<Weak<Node>>,  // Weak reference — doesn't prevent drop
+    parent: Option<Weak<Node>>,  // Weak reference / 弱引用 —— 不会阻止被释放
 }
 
 fn main() {
     let parent = Rc::new(Node { value: 1, parent: None });
     let child = Rc::new(Node {
         value: 2,
-        parent: Some(Rc::downgrade(&parent)),  // Weak ref to parent
+        parent: Some(Rc::downgrade(&parent)),  // Weak ref to parent / 对父节点的弱引用
     });
 
-    // To use a Weak, try to upgrade it — returns Option<Rc<T>>
+    // To use a Weak, try to upgrade it — returns Option<Rc<T>> / 要使用 Weak，尝试通过 upgrade 提升它 —— 返回 Option<Rc<T>>
     if let Some(parent_rc) = child.parent.as_ref().unwrap().upgrade() {
         println!("Parent value: {}", parent_rc.value);
     }
-    println!("Parent strong count: {}", Rc::strong_count(&parent)); // 1, not 2
+    println!("Parent strong count: {}", Rc::strong_count(&parent)); // 1, not 2 / 强引用计数为 1，而不是 2
 }
 ```
 
- > `Weak<T>` is covered in more depth in [Avoiding Excessive clone()](ch17-1-avoiding-excessive-clone.md). For now, the key takeaway: **use `Weak` for "back-references" in tree/graph structures to avoid memory leaks.**
+ > 我们在 [Avoiding Excessive clone() / 避免过度的克隆](ch17-1-avoiding-excessive-clone.md) 中更深入地讨论了 `Weak<T>`。目前你只需记住：**在树/图结构中使用 `Weak` 作为“回溯引用”，以避免内存泄漏。**
 
 ---
 
- ## Combining `Rc` with Interior Mutability
+ ## Combining `Rc` with Interior Mutability / 结合 `Rc` 与内部可变性
 
- The real power emerges when you combine `Rc<T>` (shared ownership) with `Cell<T>` or `RefCell<T>` (interior mutability). This lets multiple owners **read and modify** shared data:
+ 当你将 `Rc<T>`（共享所有权）与 `Cell<T>` 或 `RefCell<T>`（内部可变性）结合使用时，真正的威力就显现出来了。这允许多个所有者**读取和修改**共享数据：
 
-| Pattern | Use case |
+| **Pattern / 模式** | **Use case / 用例** |
 |---------|----------|
-| `Rc<RefCell<T>>` | Shared, mutable data (single-threaded) |
+| `Rc<RefCell<T>>` | Shared, mutable data / 共享、可变数据（单线程） |
-| `Arc<Mutex<T>>` | Shared, mutable data (multi-threaded — see [ch13](ch13-concurrency.md)) |
+| `Arc<Mutex<T>>` | Shared, mutable data / 共享、可变数据（多线程 —— 见 [第13章](ch13-concurrency.md)） |
-| `Rc<Cell<T>>` | Shared, mutable Copy types (simple flags, counters) |
+| `Rc<Cell<T>>` | Shared, mutable Copy types / 共享、可变 Copy 类型（简单标志、计数器） |
 
 ---
 
- # Exercise: Shared ownership and interior mutability
+ # Exercise: Shared ownership and interior mutability / 练习：共享所有权与内部可变性
 
- 🟡 **Intermediate**
+ 🟡 **Intermediate / 中级**
 
- - **Part 1 (Rc)**: Create an `Employee` struct with `employee_id: u64` and `name: String`. Place it in an `Rc<Employee>` and clone it into two separate `Vec`s (`us_employees` and `global_employees`). Print from both vectors to show they share the same data.
+ - **Part 1 (Rc)**: Create an `Employee` struct with `employee_id: u64` and `name: String`. Place it in an `Rc<Employee>` and clone it into two separate `Vec`s (`us_employees` and `global_employees`). Print from both vectors to show they share the same data. / **第一部分 (Rc)**：创建一个包含 `employee_id: u64` 和 `name: String` 的 `Employee` 结构体。将其放入 `Rc<Employee>` 中并克隆到两个不同的 `Vec` 中（`us_employees` 和 `global_employees`）。从两个向量中打印数据，以证明它们共享相同的数据。
- - **Part 2 (Cell)**: Add an `on_vacation: Cell<bool>` field to `Employee`. Pass an immutable `&Employee` reference to a function and toggle `on_vacation` from inside that function — without making the reference mutable.
+ - **Part 2 (Cell)**: Add an `on_vacation: Cell<bool>` field to `Employee`. Pass an immutable `&Employee` reference to a function and toggle `on_vacation` from inside that function — without making the reference mutable. / **第二部分 (Cell)**：为 `Employee` 添加一个 `on_vacation: Cell<bool>` 字段。将不可变的 `&Employee` 引用传递给一个函数，并在函数内部切换 `on_vacation` 状态 —— 而无需将引用变为可变的。
- - **Part 3 (RefCell)**: Replace `name: String` with `name: RefCell<String>` and write a function that appends a suffix to the employee's name through an `&Employee` (immutable reference).
+ - **Part 3 (RefCell)**: Replace `name: String` with `name: RefCell<String>` and write a function that appends a suffix to the employee's name through an `&Employee` (immutable reference). / **第三部分 (RefCell)**：将 `name: String` 替换为 `name: RefCell<String>`，并编写一个函数，通过 `&Employee`（不可变引用）在员工姓名后追加后缀。
 
- **Starter code:**
+ **Starter code / 初始代码：**
 ```rust
 use std::cell::{Cell, RefCell};
 use std::rc::Rc;
 
 #[derive(Debug)]
 struct Employee {
     employee_id: u64,
     name: RefCell<String>,
     on_vacation: Cell<bool>,
 }
 
 fn toggle_vacation(emp: &Employee) {
-    // TODO: Flip on_vacation using Cell::set()
+    // TODO: Flip on_vacation using Cell::set() / TODO：使用 Cell::set() 翻转 on_vacation
 }
 
 fn append_title(emp: &Employee, title: &str) {
-    // TODO: Borrow name mutably via RefCell and push_str the title
+    // TODO: Borrow name mutably via RefCell and push_str the title / TODO：通过 RefCell 可变借用姓名并追加标题
 }
 
 fn main() {
-    // TODO: Create an employee, wrap in Rc, clone into two Vecs,
+    // TODO: Create an employee, wrap in Rc, clone into two Vecs, / TODO：创建一个员工，包装在 Rc 中，克隆到两个 Vec 中，
-    // call toggle_vacation and append_title, print results
+    // 调用 toggle_vacation 和 append_title，打印结果
 }
 ```
 
- <details><summary>Solution (click to expand)</summary>
+ <details><summary>Solution (click to expand) / 解决方案（点击展开）</summary>
 
 ```rust
 use std::cell::{Cell, RefCell};
 use std::rc::Rc;
 
 #[derive(Debug)]
 struct Employee {
     employee_id: u64,
     name: RefCell<String>,
     on_vacation: Cell<bool>,
 }
 
 fn toggle_vacation(emp: &Employee) {
     emp.on_vacation.set(!emp.on_vacation.get());
 }
 
 fn append_title(emp: &Employee, title: &str) {
     emp.name.borrow_mut().push_str(title);
 }
 
 fn main() {
     let emp = Rc::new(Employee {
         employee_id: 42,
         name: RefCell::new("Alice".to_string()),
         on_vacation: Cell::new(false),
     });
 
     let mut us_employees = vec![];
     let mut global_employees = vec![];
     us_employees.push(Rc::clone(&emp));
     global_employees.push(Rc::clone(&emp));
 
-    // Toggle vacation through an immutable reference
+    // Toggle vacation through an immutable reference / 通过不可变引用切换休假状态
     toggle_vacation(&emp);
     println!("On vacation: {}", emp.on_vacation.get()); // true
 
-    // Append title through an immutable reference
+    // Append title through an immutable reference / 通过不可变引用追加标题
     append_title(&emp, ", Sr. Engineer");
     println!("Name: {}", emp.name.borrow()); // "Alice, Sr. Engineer"
 
-    // Both Vecs see the same data (Rc shares ownership)
+    // Both Vecs see the same data (Rc shares ownership) / 两个 Vec 都看到相同的数据（Rc 共享所有权）
     println!("US: {:?}", us_employees[0].name.borrow());
     println!("Global: {:?}", global_employees[0].name.borrow());
     println!("Rc strong count: {}", Rc::strong_count(&emp));
 }
- // Output:
+ // Output / 输出：
 // On vacation: true
 // Name: Alice, Sr. Engineer
 // US: "Alice, Sr. Engineer"
 // Global: "Alice, Sr. Engineer"
 // Rc strong count: 3
 ```
 
 </details>
