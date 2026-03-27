## C++ → Rust Semantic Deep Dives / C++ → Rust 语义深入对比
 
 > **What you'll learn / 你将学到：** Detailed mappings for C++ concepts that don't have obvious Rust equivalents — the four named casts, SFINAE vs trait bounds, CRTP vs associated types, and other common friction points during translation.
 >
 > 针对没有明显 Rust 等效概念的 C++ 特性进行详细映射 —— 包括四种命名转换（Casts）、SFINAE 与 Trait 约束的对比、CRTP 与关联类型的对比，以及翻译过程中的其他常见摩擦点。
 
- The sections below map C++ concepts that don't have an obvious 1:1 Rust
+ 以下各节映射了那些没有明显 1:1 Rust 对应物的 C++ 概念。
- equivalent. These differences frequently trip up C++ programmers during
+ 这些差异经常会让正在进行翻译工作的 C++ 程序员感到困惑。
- translation work.
 
- ### Casting Hierarchy: Four C++ Casts → Rust Equivalents
+ ### Casting Hierarchy: Four C++ Casts → Rust Equivalents / 类型转换层次：四种 C++ Cast → Rust 等效方案
 
- C++ has four named casts. Rust replaces them with different, more explicit mechanisms:
+ C++ 有四种命名的类型转换。Rust 用不同且更显式的机制替代了它们：
 
 ```cpp
 // C++ casting hierarchy
- int i = static_cast<int>(3.14);            // 1. Numeric / up-cast
+ int i = static_cast<int>(3.14);            // 1. Numeric / 数值转换或向上转型
- Derived* d = dynamic_cast<Derived*>(base); // 2. Runtime downcasting
+ Derived* d = dynamic_cast<Derived*>(base); // 2. Runtime / 运行时向下转型
- int* p = const_cast<int*>(cp);              // 3. Cast away const
+ int* p = const_cast<int*>(cp);              // 3. Away const / 去除 const 属性
- auto* raw = reinterpret_cast<char*>(&obj); // 4. Bit-level reinterpretation
+ auto* raw = reinterpret_cast<char*>(&obj); // 4. Bit-level / 位级重解释
 ```
 
-| C++ Cast | Rust Equivalent | Safety | Notes |
+| **C++ Cast** | **Rust Equivalent / Rust 等效** | **Safety / 安全性** | **Notes / 说明** |
 |----------|----------------|--------|-------|
-| `static_cast` (numeric) | `as` keyword | Safe but can truncate/wrap | `let i = 3.14_f64 as i32;` — truncates to 3 |
-| `static_cast` (numeric) | `as` 关键字 | Safe / 但可能截断/回绕 | `3.14 as i32` -> 3 |
-| `static_cast` (numeric, checked) | `From`/`Into` | Safe, compile-time verified | `let i: i32 = 42_u8.into();` — only widens |
-| `Checked static_cast` | `From` / `Into` | Safe / 编译时验证 | `42_u8.into()` -> i32 (仅限无损增宽) |
-| `static_cast` (numeric, fallible) | `TryFrom`/`TryInto` | Safe, returns `Result` | `let i: u8 = 300_u16.try_into()?;` — returns Err |
-| `Fallible static_cast` | `TryFrom` / `TryInto` | Safe / 返回 `Result` | `300_u16.try_into()?` -> Err |
-| `dynamic_cast` (downcast) | `match` on enum / `Any::downcast_ref` | Safe | Pattern matching for enums; `Any` for trait objects |
-| `dynamic_cast` | `match` 枚举 / `Any` | Safe / 安全 | 枚举用模式匹配；Trait 对象用 `Any` |
-| `const_cast` | No equivalent | | Rust has no way to cast away `&` → `&mut` in safe code. Use `Cell`/`RefCell` for interior mutability |
-| `const_cast` | 无直接对应 | | 安全代码无法将 `&` 强制转为 `&mut`。请使用 `Cell`/`RefCell` 实现内部可变性 |
-| `reinterpret_cast` | `std::mem::transmute` | **`unsafe`** | Reinterprets bit pattern. Almost always wrong — prefer `from_le_bytes()` etc. |
-| `reinterpret_cast` | `transmute` | **`unsafe` / 不安全** | 重解释位模式。几乎总是错误的 —— 优先使用 `from_le_bytes()` 等 |
 
 ```rust
 // Rust equivalents:
+// Rust 等效代码示例：
 
- // 1. Numeric casts — prefer From/Into over `as`
+ // 1. Numeric casts / 数值转换 —— 优先使用 From/Into 而非 `as`
- let widened: u32 = 42_u8.into();             // Infallible widening — always prefer
+ let widened: u32 = 42_u8.into();             // Infallible / 绝不失败的增宽转换 —— 始终以此为首选
- let truncated = 300_u16 as u8;                // ⚠ Wraps to 44! Silent data loss
+ let truncated = 300_u16 as u8;                // ⚠ Wraps / 回绕至 44！存在静默数据丢失
- let checked: Result<u8, _> = 300_u16.try_into(); // Err — safe fallible conversion
+ let checked: Result<u8, _> = 300_u16.try_into(); // Err / 返回错误 —— 安全的易错转换
 
- // 2. Downcast: enum (preferred) or Any (when needed for type erasure)
+ // 2. Downcast / 向下转型：枚举（推荐）或 Any（用于类型擦除时）
 use std::any::Any;
 
 fn handle_any(val: &dyn Any) {
     if let Some(s) = val.downcast_ref::<String>() {
         println!("Got string: {s}");
     } else if let Some(n) = val.downcast_ref::<i32>() {
         println!("Got int: {n}");
     }
 }
 
- // 3. "const_cast" → interior mutability (no unsafe needed)
+ // 3. "const_cast" -> interior mutability / 内部可变性（无需使用 unsafe）
 use std::cell::Cell;
 struct Sensor {
-    read_count: Cell<u32>,  // Mutate through &self
+    read_count: Cell<u32>,  // Mutate / 通过 &self 进行修改
 }
 impl Sensor {
     fn read(&self) -> f64 {
-        self.read_count.set(self.read_count.get() + 1); // &self, not &mut self
+        self.read_count.set(self.read_count.get() + 1); // 是 &self，而非 &mut self
         42.0
     }
 }
 
- // 4. reinterpret_cast → transmute (almost never needed)
+ // 4. reinterpret_cast -> transmute / 几乎不需要使用
- // Prefer safe alternatives:
+ // 优先使用安全替代方案：
- let bytes: [u8; 4] = 0x12345678_u32.to_ne_bytes();  // ✅ Safe
+ let bytes: [u8; 4] = 0x12345678_u32.to_ne_bytes();  // ✅ Safe / 安全
- let val = u32::from_ne_bytes(bytes);                   // ✅ Safe
+ let val = u32::from_ne_bytes(bytes);                   // ✅ Safe / 安全
- // unsafe { std::mem::transmute::<u32, [u8; 4]>(val) } // ❌ Avoid
+ // unsafe { std::mem::transmute::<u32, [u8; 4]>(val) } // ❌ Avoid / 应当避免
 ```
 
- > **Guideline**: In idiomatic Rust, `as` should be rare (use `From`/`Into`
+ > **准则**：在地道的 Rust 代码中，`as` 应该很少见（增宽用 `From`/`Into`
- > for widening, `TryFrom`/`TryInto` for narrowing), `transmute` should be
+ > ，缩窄用 `TryFrom`/`TryInto`），`transmute` 应该是极个别的情况，而且不需要 `const_cast` 的等效方案，因为内部可变性类型已经让它变得不再必要。
- > exceptional, and `const_cast` has no equivalent because interior mutability
- > types make it unnecessary.
 
 ---
 
- ### Preprocessor → `cfg`, Feature Flags, and `macro_rules!`
+ ### Preprocessor → `cfg`, Feature Flags, and `macro_rules!` / 预处理器 → `cfg`、特性标志与 `macro_rules!`
 
- C++ relies heavily on the preprocessor for conditional compilation, constants, and
+ C++ 严重依赖预处理器来进行条件编译、定义常量和生成代码。
- code generation. Rust replaces all of these with first-class language features.
+ Rust 则用语言的一等公民特性替代了所有这些功能。
 
- #### `#define` constants → `const` or `const fn`
+ #### `#define` constants → `const` or `const fn` / `#define` 常量 → `const` 或 `const fn`
 
 ```cpp
 // C++
 #define MAX_RETRIES 5
 #define BUFFER_SIZE (1024 * 64)
- #define SQUARE(x) ((x) * (x))  // Macro — textual substitution, no type safety
+ #define SQUARE(x) ((x) * (x))  // Macro / 宏 —— 文本替换，没有类型安全
 ```
 
 ```rust
 // Rust — type-safe, scoped, no textual substitution
+// Rust — 类型安全、有作用域、无文本替换
 const MAX_RETRIES: u32 = 5;
 const BUFFER_SIZE: usize = 1024 * 64;
- const fn square(x: u32) -> u32 { x * x }  // Evaluated at compile time
+ const fn square(x: u32) -> u32 { x * x }  // Evaluated / 在编译时求值
 
- // Can be used in const contexts:
+ // 可用于 const 上下文：
- const AREA: u32 = square(12);  // Computed at compile time
+ const AREA: u32 = square(12);  // Computed / 在编译时计算
 static BUFFER: [u8; BUFFER_SIZE] = [0; BUFFER_SIZE];
 ```
 
- #### `#ifdef` / `#if` → `#[cfg()]` and `cfg!()`
+ #### `#ifdef` / `#if` → `#[cfg()]` and `cfg!()` / `#ifdef` / `#if` → `#[cfg()]` 与 `cfg!()`
 
 ```cpp
 // C++
 #ifdef DEBUG
     log_verbose("Step 1 complete");
 #endif
 
 #if defined(LINUX) && !defined(ARM)
     use_x86_path();
 #else
     use_generic_path();
 #endif
 ```
 
 ```rust
 // Rust — attribute-based conditional compilation
+// Rust — 基于属性的条件编译
 #[cfg(debug_assertions)]
 fn log_verbose(msg: &str) { eprintln!("[VERBOSE] {msg}"); }
 
 #[cfg(not(debug_assertions))]
- fn log_verbose(_msg: &str) { /* compiled away in release */ }
+ fn log_verbose(_msg: &str) { /* compiled / 在 release 模式下会被删除 */ }
 
- // Combine conditions:
+ // 组合条件：
 #[cfg(all(target_os = "linux", target_arch = "x86_64"))]
 fn use_x86_path() { /* ... */ }
 
 #[cfg(not(all(target_os = "linux", target_arch = "x86_64")))]
 fn use_generic_path() { /* ... */ }
 
- // Runtime check (condition is still compile-time, but usable in expressions):
+ // Runtime check / 运行时检查（条件仍然是编译时的，但可用于表达式）：
 if cfg!(target_os = "windows") {
     println!("Running on Windows");
 }
 ```
 
- #### Feature flags in `Cargo.toml`
+ #### Feature flags in `Cargo.toml` / `Cargo.toml` 中的特性标志（Feature Flags）
 
 ```toml
- # Cargo.toml — replace #ifdef FEATURE_FOO
+ # Cargo.toml — 用于替代 #ifdef FEATURE_FOO
 [features]
 default = ["json"]
- json = ["dep:serde_json"]       # Optional dependency
+ json = ["dep:serde_json"]       # Optional / 可选依赖
- verbose-logging = []            # Flag with no extra dependency
+ verbose-logging = []            # Flag / 无额外依赖的标志
- gpu-support = ["dep:cuda-sys"]  # Optional GPU support
+ gpu-support = ["dep:cuda-sys"]  # Optional / 可选的 GPU 支持
 ```
 
 ```rust
- // Conditional code based on feature flags:
+ // 基于特性标志的条件代码：
 #[cfg(feature = "json")]
 pub fn parse_config(data: &str) -> Result<Config, Error> {
     serde_json::from_str(data).map_err(Error::from)
 }
 
 #[cfg(feature = "verbose-logging")]
 macro_rules! verbose {
     ($($arg:tt)*) => { eprintln!("[VERBOSE] {}", format!($($arg)*)); }
 }
 #[cfg(not(feature = "verbose-logging"))]
 macro_rules! verbose {
-    ($($arg:tt)*) => { }; // Compiles to nothing
+    ($($arg:tt)*) => { }; // Compiles / 编译后不产生任何内容
 }
 ```
 
- #### `#define MACRO(x)` → `macro_rules!`
+ #### `#define MACRO(x)` → `macro_rules!` / `#define MACRO(x)` → `macro_rules!`
 
 ```cpp
 // C++ — textual substitution, notoriously error-prone
+// C++ — 文本替换，众所周知的容易出错
 #define DIAG_CHECK(cond, msg) \
     do { if (!(cond)) { log_error(msg); return false; } } while(0)
 ```
 
 ```rust
 // Rust — hygienic, type-checked, operates on syntax tree
+// Rust — 卫生的（Hygienic）、类型检查、在语法树上操作
 macro_rules! diag_check {
     ($cond:expr, $msg:expr) => {
         if !($cond) {
             log_error($msg);
             return Err(DiagError::CheckFailed($msg.to_string()));
         }
     };
 }
 
 fn run_test() -> Result<(), DiagError> {
     diag_check!(temperature < 85.0, "GPU too hot");
     diag_check!(voltage > 0.8, "Rail voltage too low");
     Ok(())
 }
 ```
 
-| C++ Preprocessor | Rust Equivalent | Advantage |
+| **C++ Preprocessor / C++ 预处理器** | **Rust Equivalent / Rust 等效** | **Advantage / 优势** |
 |-----------------|----------------|-----------|
-| `#define PI 3.14` | `const PI: f64 = 3.14;` | Typed, scoped, visible to debugger |
-| `#define PI 3.14` | `const PI: f64 = 3.14;` | 有类型、有作用域、调试器可见 |
-| `#define MAX(a,b) ((a)>(b)?(a):(b))` | `macro_rules!` or generic `fn max<T: Ord>` | No double-evaluation bugs |
-| `#define MAX(a,b)` | `macro_rules!` 或泛型 `max` | 无重复求值漏洞 |
-| `#ifdef DEBUG` | `#[cfg(debug_assertions)]` | Checked by compiler, no typo risk |
-| `#ifdef DEBUG` | `#[cfg(debug_assertions)]` | 编译器检查，无拼写错误风险 |
-| `#ifdef FEATURE_X` | `#[cfg(feature = "x")]` | Cargo manages features; dependency-aware |
-| `#ifdef FEATURE_X` | `#[cfg(feature = "x")]` | Cargo 管理特性，感知依赖 |
-| `#include "header.h"` | `mod module;` + `use module::Item;` | No include guards, no circular includes |
-| `#include "header.h"` | `mod` + `use` | 无需头文件卫士，无循环导出 |
-| `#pragma once` | Not needed | Each `.rs` file is a module — included exactly once |
-| `#pragma once` | 不需要 | 每个 `.rs` 文件就是一个模块 —— 仅包含一次 |
 
 ---
 
- ### Header Files and `#include` → Modules and `use`
+ ### Header Files and `#include` → Modules and `use` / 头文件与 `#include` → 模块与 `use`
 
- In C++, the compilation model revolves around textual inclusion:
+ 在 C++ 中，编译模型围绕着文本包含展开：
 
 ```cpp
 // widget.h — every translation unit that uses Widget includes this
+// widget.h — 每个使用 Widget 的编译单元都要包含此文件
 #pragma once
 #include <string>
 #include <vector>
 
 class Widget {
 public:
     Widget(std::string name);
     void activate();
 private:
     std::string name_;
     std::vector<int> data_;
 };
 ```
 
 ```cpp
 // widget.cpp — separate definition
+// widget.cpp — 分离的定义
 #include "widget.h"
 Widget::Widget(std::string name) : name_(std::move(name)) {}
 void Widget::activate() { /* ... */ }
 ```
 
- In Rust, there are **no header files, no forward declarations, no include guards**:
+ 在 Rust 中，**没有头文件、没有前向声明、也没有头文件卫士**：
 
 ```rust
 // src/widget.rs — declaration AND definition in one file
+// src/widget.rs — 声明和定义都在一个文件中
 pub struct Widget {
-    name: String,         // Private by default
+    name: String,         // Default / 默认是私有的
     data: Vec<i32>,
 }
 
 impl Widget {
     pub fn new(name: String) -> Self {
         Widget { name, data: Vec::new() }
     }
     pub fn activate(&self) { /* ... */ }
 }
 ```
 
 ```rust
 // src/main.rs — import by module path
+// src/main.rs — 通过模块路径导入
- mod widget;  // Tells compiler to include src/widget.rs
+ mod widget;  // Tells / 告知编译器包含 src/widget.rs
 use widget::Widget;
 
 fn main() {
     let w = Widget::new("sensor".to_string());
     w.activate();
 }
 ```
 
-| C++ | Rust | Why it's better |
+| **C++** | **Rust** | **Why it's better / 为什么更好** |
 |-----|------|-----------------|
-| `#include "foo.h"` | `mod foo;` in parent + `use foo::Item;` | No textual inclusion, no ODR violations |
-| `#include "foo.h"` | `mod foo;` + `use foo;` | 无文本包含，无 ODR 违反 |
-| `#pragma once` / include guards | Not needed | Each `.rs` file is a module — compiled once |
-| `#pragma once` | 不需要 | 每个 `.rs` 文件即模块，仅编译一次 |
-| Forward declarations | Not needed | Compiler sees entire crate; order doesn't matter |
-| Forward declarations | 不需要 | 编译器可见整个 crate；顺序无关紧要 |
-| `class Foo;` (incomplete type) | Not needed | No separate declaration/definition split |
-| `Incomplete type` | 不需要 | 无需声明与定义分离 |
-| `.h` + `.cpp` for each class | Single `.rs` file | No declaration/definition mismatch bugs |
-| `.h` + `.cpp` 模式 | 单个 `.rs` 文件 | 无声明/定义不一致的漏洞 |
-| `using namespace std;` | `use std::collections::HashMap;` | Always explicit — no global namespace pollution |
-| `using namespace` | `use std::...` | 始终显式 —— 无全局命名空间污染 |
-| Nested `namespace a::b` | Nested `mod a { mod b { } }` or `a/b.rs` | File system mirrors module tree |
-| 嵌套 `namespace` | 嵌套 `mod` 或子目录 | 文件系统镜像模块树 |
 
 ---
 
- ### `friend` and Access Control → Module Visibility
+ ### `friend` and Access Control → Module Visibility / `friend` 与访问控制 → 模块可见性
 
- C++ uses `friend` to grant specific classes or functions access to private members.
+ C++ 使用 `friend` 授予特定类或函数访问私有成员的权限。
- Rust has no `friend` keyword — instead, **privacy is module-scoped**:
+ Rust 没有 `friend` 关键字 —— 取而代之的是，**私有属性是以模块为作用域的**：
 
 ```cpp
 // C++
 class Engine {
-    friend class Car;   // Car can access private members
+    friend class Car;   // Car / Car 类可以访问私有成员
     int rpm_;
     void set_rpm(int r) { rpm_ = r; }
 public:
     int rpm() const { return rpm_; }
 };
 ```
 
 ```rust
 // Rust — items in the same module can access all fields, no `friend` needed
+// Rust — 同一模块中的项可以访问所有字段，不需要 `friend`
 mod vehicle {
     pub struct Engine {
-        rpm: u32,  // Private to the module (not to the struct!)
+        rpm: u32,  // Private / 模块内私有（而非结构体内私有！）
     }
 
     impl Engine {
         pub fn new() -> Self { Engine { rpm: 0 } }
         pub fn rpm(&self) -> u32 { self.rpm }
     }
 
     pub struct Car {
         engine: Engine,
     }
 
     impl Car {
         pub fn new() -> Self { Car { engine: Engine::new() } }
         pub fn accelerate(&mut self) {
-            self.engine.rpm = 3000; // ✅ Same module — direct field access
+            self.engine.rpm = 3000; // ✅ Same / 同一模块 —— 直接访问字段
         }
         pub fn rpm(&self) -> u32 {
-            self.engine.rpm  // ✅ Same module — can read private field
+            self.engine.rpm  // ✅ Same / 同一模块 —— 可以读取私有字段
         }
     }
 }
 
 fn main() {
     let mut car = vehicle::Car::new();
     car.accelerate();
-    // car.engine.rpm = 9000;  // ❌ Compile error: `engine` is private
+    // car.engine.rpm = 9000;  // ❌ Compile error / 编译错误：`engine` 是私有的
-    println!("RPM: {}", car.rpm()); // ✅ Public method on Car
+    println!("RPM: {}", car.rpm()); // ✅ Public / Car 上的公共方法
 }
 ```
 
-| C++ Access | Rust Equivalent | Scope |
+| **C++ Access / C++ 访问** | **Rust Equivalent / Rust 等效** | **Scope / 作用域** |
 |-----------|----------------|-------|
-| `private` | (default, no keyword) | Accessible within the same module only |
-| `private` | (默认，无关键字) | 仅在同一模块内可访问 |
-| `protected` | No direct equivalent | Use `pub(super)` for parent module access |
-| `protected` | 无直接对应 | 使用 `pub(super)` 供父模块访问 |
-| `public` | `pub` | Accessible everywhere |
-| `public` | `pub` | 随处可访问 |
-| `friend class Foo` | Put `Foo` in the same module | Module-level privacy replaces friend |
-| `friend class Foo` | 将 `Foo` 放在同模块 | 模块级私有性替代了 friend |
-| — | `pub(crate)` | Visible within the crate but not to external dependents |
-| — | `pub(crate)` | 对 crate 内可见，但对外部依赖不可见 |
-| — | `pub(super)` | Visible to the parent module only |
-| — | `pub(super)` | 仅对父模块可见 |
-| — | `pub(in crate::path)` | Visible within a specific module subtree |
-| — | `pub(in 指定路径)` | 在特定的模块子树内可见 |
 
- > **Key insight**: C++ privacy is per-class. Rust privacy is per-module.
+ > **关键见解**：C++ 的私有性是基于类的。Rust 的私有性是基于模块的。
- > This means you control access by choosing which types live in the same module —
- > colocated types have full access to each other's private fields.
+ > 这意味着你可以通过选择哪些类型位于同一模块来控制访问权限 —— 处于同一位置的类型可以完全访问对方的私有字段。
 
 ---
 
- ### `volatile` → Atomics and `read_volatile`/`write_volatile`
+ ### `volatile` → Atomics and `read_volatile`/`write_volatile` / `volatile` → 原子操作与 `read_volatile`/`write_volatile`
 
- In C++, `volatile` tells the compiler not to optimize away reads/writes — typically
+ 在 C++ 中，`volatile` 告知编译器不要优化掉读/写操作 —— 通常
- used for memory-mapped hardware registers. **Rust has no `volatile` keyword.**
+ 用于内存映射的硬件寄存器。**Rust 没有 `volatile` 关键字。**
 
 ```cpp
 // C++: volatile for hardware registers
+// C++: 用于硬件寄存器的 volatile
 volatile uint32_t* const GPIO_REG = reinterpret_cast<volatile uint32_t*>(0x4002'0000);
- *GPIO_REG = 0x01;              // Write not optimized away
+ *GPIO_REG = 0x01;              // Write / 写入不会被优化掉
- uint32_t val = *GPIO_REG;     // Read not optimized away
+ uint32_t val = *GPIO_REG;     // Read / 读取不会被优化掉
 ```
 
 ```rust
 // Rust: explicit volatile operations — only in unsafe code
+// Rust: 显式的 volatile 操作 —— 仅限 unsafe 代码
 use std::ptr;
 
 const GPIO_REG: *mut u32 = 0x4002_0000 as *mut u32;
 
 // SAFETY: GPIO_REG is a valid memory-mapped I/O address.
+// 安全性：GPIO_REG 是一个有效的内存映射 I/O 地址。
 unsafe {
-    ptr::write_volatile(GPIO_REG, 0x01);   // Write not optimized away
+    ptr::write_volatile(GPIO_REG, 0x01);   // Write / 写入不会被优化掉
-    let val = ptr::read_volatile(GPIO_REG); // Read not optimized away
+    let val = ptr::read_volatile(GPIO_REG); // Read / 读取不会被优化掉
 }
 ```
 
- For **concurrent shared state** (the other common C++ `volatile` use), Rust uses atomics:
+ 对于**并发共享状态**（C++ `volatile` 的另一种常见用途），Rust 使用原子操作：
 
 ```cpp
 // C++: volatile is NOT sufficient for thread safety (common mistake!)
+// C++: volatile 对线程安全是不够的（常见错误！）
- volatile bool stop_flag = false;  // ❌ Data race — UB in C++11+
+ volatile bool stop_flag = false;  // ❌ Data race / 数据竞争 —— C++11+ 中的未定义行为
 
- // Correct C++:
+ // 正确的 C++：
 std::atomic<bool> stop_flag{false};
 ```
 
 ```rust
 // Rust: atomics are the only way to share mutable state across threads
+// Rust: 原子操作是跨线程共享可变状态的唯一方式
 use std::sync::atomic::{AtomicBool, Ordering};
 
 static STOP_FLAG: AtomicBool = AtomicBool::new(false);
 
- // From another thread:
+ // 来自另一个线程：
 STOP_FLAG.store(true, Ordering::Release);
 
- // Check:
+ // 检查：
 if STOP_FLAG.load(Ordering::Acquire) {
     println!("Stopping");
 }
 ```
 
-| C++ Usage | Rust Equivalent | Notes |
+| **C++ Usage / C++ 用法** | **Rust Equivalent / Rust 等效** | **Notes / 说明** |
 |-----------|----------------|-------|
-| `volatile` for hardware registers | `ptr::read_volatile` / `ptr::write_volatile` | Requires `unsafe` — correct for MMIO |
-| `volatile` (hardware) | `read_volatile` / `write_volatile` | 需要 `unsafe` —— 适用于 MMIO |
-| `volatile` for thread signaling | `AtomicBool` / `AtomicU32` etc. | C++ `volatile` is wrong for this too! |
-| `volatile` (thread) | 原子类型 (AtomicBool 等) | C++ volatile 用于此场景也是错的！ |
-| `std::atomic<T>` | `std::sync::atomic::AtomicT` | Same semantics, same orderings |
-| `std::atomic<T>` | `std::sync::atomic::AtomicT` | 语义相同，内存次序相同 |
-| `std::atomic<T>::load(memory_order_acquire)` | `AtomicT::load(Ordering::Acquire)` | 1:1 mapping |
-| `load(memory_order_acquire)` | `load(Ordering::Acquire)` | 1:1 映射 |
 
 ---
 
- ### `static` Variables → `static`, `const`, `LazyLock`, `OnceLock`
+ ### `static` Variables → `static`, `const`, `LazyLock`, `OnceLock` / `static` 变量 → `static`、`const`、`LazyLock`、`OnceLock`
 
- #### Basic `static` and `const`
+ #### Basic `static` and `const` / 基础 `static` 与 `const`
 
 ```cpp
 // C++
- const int MAX_RETRIES = 5;                    // Compile-time constant
+ const int MAX_RETRIES = 5;                    // Constant / 编译时常量
- static std::string CONFIG_PATH = "/etc/app";  // Static init — order undefined!
+ static std::string CONFIG_PATH = "/etc/app";  // Static / 静态初始化 —— 顺序未定义！
 ```
 
 ```rust
 // Rust
- const MAX_RETRIES: u32 = 5;                   // Compile-time constant, inlined
+ const MAX_RETRIES: u32 = 5;                   // Constant / 编译时常量，内联
- static CONFIG_PATH: &str = "/etc/app";         // 'static lifetime, fixed address
+ static CONFIG_PATH: &str = "/etc/app";         // 'static / 'static 生命周期，固定地址
 ```
 
- #### The static initialization order fiasco
+ #### The static initialization order fiasco / 静态初始化顺序困境
 
- C++ has a well-known problem: global constructors in different translation units
+ C++ 有一个众所周知的问题：不同编译单元中的全局构造函数执行
- execute in **unspecified order**. Rust avoids this entirely — `static` values must
+ **顺序是不确定的**。Rust 完全避免了这一点 —— `static` 值必须是
- be compile-time constants (no constructors).
+ 编译时常量（无构造函数）。
 
- For runtime-initialized globals, use `LazyLock` (Rust 1.80+) or `OnceLock`:
+ 对于运行时初始化的全局变量，请使用 `LazyLock`（Rust 1.80+）或 `OnceLock`：
 
 ```rust
 use std::sync::LazyLock;
 
 // Equivalent to C++ `static std::regex` — initialized on first access, thread-safe
+// 等效于 C++ `static std::regex` —— 在首次访问时初始化，线程安全
 static CONFIG_REGEX: LazyLock<regex::Regex> = LazyLock::new(|| {
     regex::Regex::new(r"^[a-z]+_diag$").expect("invalid regex")
 });
 
 fn is_valid_diag(name: &str) -> bool {
-    CONFIG_REGEX.is_match(name)  // First call initializes; subsequent calls are fast
+    CONFIG_REGEX.is_match(name)  // First / 首次调用时初始化；后续调用非常快
 }
 ```
 
 ```rust
 use std::sync::OnceLock;
 
 // OnceLock: initialized once, can be set from runtime data
+// OnceLock：仅初始化一次，可根据运行时数据进行设置
 static DB_CONN: OnceLock<String> = OnceLock::new();
 
 fn init_db(connection_string: &str) {
     DB_CONN.set(connection_string.to_string())
         .expect("DB_CONN already initialized");
 }
 
 fn get_db() -> &'static str {
     DB_CONN.get().expect("DB not initialized")
 }
 ```
 
-| C++ | Rust | Notes |
+| **C++** | **Rust** | **Notes / 说明** |
 |-----|------|-------|
-| `const int X = 5;` | `const X: i32 = 5;` | Both compile-time. Rust requires type annotation |
-| `const int X = 5;` | `const X: i32 = 5;` | 均为编译时。Rust 必须标注类型 |
-| `constexpr int X = 5;` | `const X: i32 = 5;` | Rust `const` is always constexpr |
-| `constexpr` | `const` | Rust `const` 始终是 constexpr |
-| `static int count = 0;` (file scope) | `static COUNT: AtomicI32 = AtomicI32::new(0);` | Mutable statics require `unsafe` or atomics |
-| `static` (文件作用域) | `static COUNT: AtomicI32` | 可变静态变量需要 `unsafe` 或原子操作 |
-| `static std::string s = "hi";` | `static S: &str = "hi";` or `LazyLock<String>` | No runtime constructor for simple cases |
-| `static string` | `static S: &str` 或 `LazyLock` | 简单场景无需运行时构造函数 |
-| `static MyObj obj;` (complex init) | `static OBJ: LazyLock<MyObj> = LazyLock::new(\|\| { ... });` | Thread-safe, lazy, no init order issues |
-| `static MyObj` (复杂初始化) | `static OBJ: LazyLock` | 线程安全、延迟加载、无初始化顺序问题 |
-| `thread_local` | `thread_local! { static X: Cell<u32> = Cell::new(0); }` | Same semantics |
-| `thread_local` | `thread_local!` 宏 | 语义相同 |
 
 ---
 
- ### `constexpr` → `const fn`
+ ### `constexpr` → `const fn` / `constexpr` → `const fn`
 
- C++ `constexpr` marks functions and variables for compile-time evaluation. Rust
+ C++ `constexpr` 标记函数和变量以进行编译时求值。
- uses `const fn` and `const` for the same purpose:
+ Rust 使用 `const fn` 和 `const` 来达到同样的目的：
 
 ```cpp
 // C++
 constexpr int factorial(int n) {
     return n <= 1 ? 1 : n * factorial(n - 1);
 }
- constexpr int val = factorial(5);  // Computed at compile time → 120
+ constexpr int val = factorial(5);  // Computed / 在编译时计算 → 120
 ```
 
 ```rust
 // Rust
 const fn factorial(n: u32) -> u32 {
     if n <= 1 { 1 } else { n * factorial(n - 1) }
 }
- const VAL: u32 = factorial(5);  // Computed at compile time → 120
+ const VAL: u32 = factorial(5);  // Computed / 在编译时计算 → 120
 
- // Also works in array sizes and match patterns:
+ // 也适用于数组大小和匹配模式：
 const LOOKUP: [u32; 5] = [factorial(1), factorial(2), factorial(3),
                            factorial(4), factorial(5)];
 ```
 
-| C++ | Rust | Notes |
+| **C++** | **Rust** | **Notes / 说明** |
 |-----|------|-------|
-| `constexpr int f()` | `const fn f() -> i32` | Same intent — compile-time evaluable |
-| `constexpr f()` | `const fn f()` | 意图相同 —— 编译时可求值 |
-| `constexpr` variable | `const` variable | Rust `const` is always compile-time |
-| `constexpr` 变量 | `const` 变量 | Rust `const` 始终是编译时的 |
-| `consteval` (C++20) | No equivalent | `const fn` can also run at runtime |
-| `consteval` (C++20) | 无直接对应 | `const fn` 也可以在运行时运行 |
-| `if constexpr` (C++17) | No equivalent (use `cfg!` or generics) | Trait specialization fills some use cases |
-| `if constexpr` (C++17) | 无直接对应 (用 `cfg!` 或泛型) | Trait 特化可覆盖部分用例 |
-| `constinit` (C++20) | `static` with const initializer | Rust `static` must be const-initialized by default |
-| `constinit` (C++20) | 带 const 初始值的 `static` | Rust `static` 默认必须用 const 初始化 |
 
- > **Current limitations of `const fn`** (stabilized as of Rust 1.82):
+ > **`const fn` 的当前限制**（截至 Rust 1.82 已稳定）：
- > - No trait methods (can't call `.len()` on a `Vec` in const context)
+ > - 无 Trait 方法（不能在 const 上下文中对 `Vec` 调用 `.len()`）
- > - No heap allocation (`Box::new`, `Vec::new` not const)
+ > - 无堆分配（`Box::new`、`Vec::new` 均不是 const）
- > - ~~No floating-point arithmetic~~ — **stabilized in Rust 1.82**
+ > - ~~无浮点运算~~ —— **Rust 1.82 已稳定**
- > - Can't use `for` loops (use recursion or `while` with manual index)
+ > - 不能使用 `for` 循环（使用递归或带有手动索引的 `while`）
 
 ---
 
- ### SFINAE and `enable_if` → Trait Bounds and `where` Clauses
+ ### SFINAE and `enable_if` → Trait Bounds and `where` Clauses / SFINAE 与 `enable_if` → Trait 约束与 `where` 子句
 
- In C++, SFINAE (Substitution Failure Is Not An Error) is the mechanism behind
+ 在 C++ 中，SFINAE（替换失败并非错误）是
- conditional generic programming. It is powerful but notoriously unreadable. Rust
+ 条件泛型编程背后的机制。它很强大，但众所周知的难以阅读。
- replaces it entirely with **trait bounds**:
+ Rust 完全用 **Trait 约束** 替代了它：
 
 ```cpp
 // C++: SFINAE-based conditional function (pre-C++20)
+// C++: 基于 SFINAE 的条件函数 (C++20 之前)
 template<typename T,
          std::enable_if_t<std::is_integral_v<T>, int> = 0>
 T double_it(T val) { return val * 2; }
 
 template<typename T,
          std::enable_if_t<std::is_floating_point_v<T>, int> = 0>
 T double_it(T val) { return val * 2.0; }
 
- // C++20 concepts — cleaner but still verbose:
+ // C++20 concepts — 更加清晰但依然繁琐：
 template<std::integral T>
 T double_it(T val) { return val * 2; }
 ```
 
 ```rust
 // Rust: trait bounds — readable, composable, excellent error messages
+// Rust: Trait 约束 —— 易读、可组合、错误消息极佳
 use std::ops::Mul;
 
 fn double_it<T: Mul<Output = T> + From<u8>>(val: T) -> T {
     val * T::from(2)
 }
 
- // Or with where clause for complex bounds:
+ // 或者针对复杂约束使用 where 子句：
 fn process<T>(val: T) -> String
 where
     T: std::fmt::Display + Clone + Send,
 {
     format!("Processing: {}", val)
 }
 
- // Conditional behavior via separate impls (replaces SFINAE overloads):
+ // 通过分离的 impl 实现条件行为（替换 SFINAE 重载）：
 trait Describable {
     fn describe(&self) -> String;
 }
 
 impl Describable for u32 {
     fn describe(&self) -> String { format!("integer: {self}") }
 }
 
 impl Describable for f64 {
     fn describe(&self) -> String { format!("float: {self:.2}") }
 }
 ```
 
-| C++ Template Metaprogramming | Rust Equivalent | Readability |
+| **C++ Template Metaprogramming / C++ 模板元编程** | **Rust Equivalent / Rust 等效** | **Readability / 易读性** |
 |-----------------------------|----------------|-------------|
-| `std::enable_if_t<cond>` | `where T: Trait` | 🟢 Clear English |
-| `enable_if_t` | `where T: Trait` | 🟢 清晰的类自然语言 |
-| `std::is_integral_v<T>` | Bound on a numeric trait or specific types | 🟢 No `_v` / `_t` suffixes |
-| `is_integral_v` | 针对数值 Trait 或特定类型的约束 | 🟢 无 `_v` / `_t` 后缀 |
-| SFINAE overload sets | Separate `impl Trait for ConcreteType` blocks | 🟢 Each impl stands alone |
-| SFINAE 重载集合 | 独立的 `impl Trait for 类型` 块 | 🟢 每个实现各司其职 |
-| `if constexpr (std::is_same_v<T, int>)` | Specialization via trait impls | 🟢 Compile-time dispatched |
-| `if constexpr` 类型判断 | 通过 Trait 实现进行特化 | 🟢 编译时分发 |
-| C++20 `concept` | `trait` | 🟢 Nearly identical intent |
-| C++20 `concept` | `trait` | 🟢 意图几乎一致 |
-| `requires` clause | `where` clause | 🟢 Same position, similar syntax |
-| `requires` 子句 | `where` 子句 | 🟢 相同位置，语法相似 |
-| Compilation fails deep inside template | Compilation fails at the call site with trait mismatch | 🟢 No 200-line error cascades |
-| 模板深处编译失败 | 在调用处因 Trait 不匹配而失败 | 🟢 无 200 行错误连锁 |
 
- > **Key insight**: C++ concepts (C++20) are the closest thing to Rust traits.
+ > **关键见解**：C++ 概念（C++20）是与 Rust Trait 最接近的东西。
- > If you're familiar with C++20 concepts, think of Rust traits as concepts
- > that have been a first-class language feature since 1.0, with a coherent
- > implementation model (trait impls) instead of duck typing.
+ > 如果你熟悉 C++20 概念，可以将 Rust Trait 视作为自 1.0 以来就是一等公民特性的概念，它具有连贯的实现模型（Trait 实现）而非鸭子类型。
 
 ---
 
- ### `std::function` → Function Pointers, `impl Fn`, and `Box<dyn Fn>`
+ ### `std::function` → Function Pointers, `impl Fn`, and `Box<dyn Fn>` / `std::function` → 函数指针、`impl Fn` 与 `Box<dyn Fn>`
 
- C++ `std::function<R(Args...)>` is a type-erased callable. Rust has three options,
+ C++ `std::function<R(Args...)>` 是一个类型擦除的可调用对象。Rust 有三种选择，
- each with different trade-offs:
+ 每种都有不同的权衡：
 
 ```cpp
 // C++: one-size-fits-all (heap-allocated, type-erased)
+// C++: 一站式方案（堆分配、类型擦除）
 #include <functional>
 std::function<int(int)> make_adder(int n) {
     return [n](int x) { return x + n; };
 }
 ```
 
 ```rust
 // Rust Option 1: fn pointer — simple, no captures, no allocation
+// Rust 选项 1：fn 指针 —— 简单、无捕获、无分配
 fn add_one(x: i32) -> i32 { x + 1 }
 let f: fn(i32) -> i32 = add_one;
 println!("{}", f(5)); // 6
 
 // Rust Option 2: impl Fn — monomorphized, zero overhead, can capture
+// Rust 选项 2：impl Fn —— 单态化、零开销、可以捕获
 fn apply(val: i32, f: impl Fn(i32) -> i32) -> i32 { f(val) }
 let n = 10;
- let result = apply(5, |x| x + n);  // Closure captures `n`
+ let result = apply(5, |x| x + n);  // Capture / 闭包捕获了 `n`
 
 // Rust Option 3: Box<dyn Fn> — type-erased, heap-allocated (like std::function)
+// Rust 选项 3：Box<dyn Fn> —— 类型擦除、堆分配（类似于 std::function）
 fn make_adder(n: i32) -> Box<dyn Fn(i32) -> i32> {
     Box::new(move |x| x + n)
 }
 let adder = make_adder(10);
 println!("{}", adder(5));  // 15
 
 // Storing heterogeneous callables (like vector<function<int(int)>>):
+// 存储异构可调用对象（如 vector<function<int(int)>>）：
 let callbacks: Vec<Box<dyn Fn(i32) -> i32>> = vec![
     Box::new(|x| x + 1),
     Box::new(|x| x * 2),
     Box::new(make_adder(100)),
 ];
 for cb in &callbacks {
     println!("{}", cb(5));  // 6, 10, 105
 }
 ```
 
-| When to use | C++ Equivalent | Rust Choice |
+| **When to use / 场景** | **C++ Equivalent / C++ 等效** | **Rust Choice / Rust 选择** |
 |------------|---------------|-------------|
-| Top-level function, no captures | Function pointer | `fn(Args) -> Ret` |
-| Top-level / 无捕获的顶层函数 | Function pointer | `fn(Args) -> Ret` |
-| Generic function accepting callables | Template parameter | `impl Fn(Args) -> Ret` (static dispatch) |
-| Generic / 接收可调用对象的泛型函数 | Template parameter | `impl Fn(Args) -> Ret` (静态分发) |
-| Trait bound in generics | `template<typename F>` | `F: Fn(Args) -> Ret` |
-| Trait / 泛型中的 Trait 约束 | `template<typename F>` | `F: Fn(Args) -> Ret` |
-| Stored callable, type-erased | `std::function<R(Args)>` | `Box<dyn Fn(Args) -> Ret>` |
-| Stored / 类型擦除的存储对象 | `std::function` | `Box<dyn Fn(Args) -> Ret>` |
-| Callback that mutates state | `std::function` with mutable lambda | `Box<dyn FnMut(Args) -> Ret>` |
-| Mutates / 会修改状态的回调 | mutable lambda | `Box<dyn FnMut(Args) -> Ret>` |
-| One-shot callback (consumed) | `std::function` (moved) | `Box<dyn FnOnce(Args) -> Ret>` |
-| One-shot / 一次性回调 (被消耗) | moved function | `Box<dyn FnOnce(Args) -> Ret>` |
 
- > **Performance note**: `impl Fn` has zero overhead (monomorphized, like a C++ template).
+ > **性能说明**：`impl Fn` 具有零开销（单态化，类似于 C++ 模板）。
- > `Box<dyn Fn>` has the same overhead as `std::function` (vtable + heap allocation).
+ > `Box<dyn Fn>` 具有与 `std::function` 相同的开销（虚函数表 + 堆分配）。
- > Prefer `impl Fn` unless you need to store heterogeneous callables.
+ > 除非你需要存储异构的可调用对象，否则请优先选用 `impl Fn`。
 
 ---
 
- ### Container Mapping: C++ STL → Rust `std::collections`
+ ### Container Mapping: C++ STL → Rust `std::collections` / 容器映射：C++ STL → Rust `std::collections`
 
-| C++ STL Container | Rust Equivalent | Notes |
+| **C++ STL Container / C++ 容器** | **Rust Equivalent / Rust 等效** | **Notes / 说明** |
 |------------------|----------------|-------|
-| `std::vector<T>` | `Vec<T>` | Nearly identical API. Rust checks bounds by default |
-| `vector<T>` | `Vec<T>` | 几乎一致的 API。Rust 默认检查边界 |
-| `std::array<T, N>` | `[T; N]` | Stack-allocated fixed-size array |
-| `array<T, N>` | `[T; N]` | 栈分配的固定大小数组 |
-| `std::deque<T>` | `std::collections::VecDeque<T>` | Ring buffer. Efficient push/pop at both ends |
-| `deque<T>` | `VecDeque<T>` | 环形缓冲区。两端高效 push/pop |
-| `std::list<T>` | `std::collections::LinkedList<T>` | Rarely used in Rust — `Vec` is almost always faster |
-| `list<T>` | `LinkedList<T>` | Rust 中极少使用 —— `Vec` 几乎总是更快 |
-| `std::forward_list<T>` | No equivalent | Use `Vec` or `VecDeque` |
-| `forward_list<T>` | 无直接对应 | 使用 `Vec` 或 `VecDeque` |
-| `std::unordered_map<K, V>` | `std::collections::HashMap<K, V>` | Uses `SipHash` by default (DoS-resistant) |
-| `unordered_map<K, V>` | `HashMap<K, V>` | 默认使用 `SipHash`（抗 DoS 攻击） |
-| `std::map<K, V>` | `std::collections::BTreeMap<K, V>` | B-tree; keys sorted; `K: Ord` required |
-| `map<K, V>` | `BTreeMap<K, V>` | B 树；键有序；要求 `K: Ord` |
-| `std::unordered_set<T>` | `std::collections::HashSet<T>` | `T: Hash + Eq` required |
-| `unordered_set<T>` | `HashSet<T>` | 要求 `T: Hash + Eq` |
-| `std::set<T>` | `std::collections::BTreeSet<T>` | Sorted set; `T: Ord` required |
-| `set<T>` | `BTreeSet<T>` | 有序集合；要求 `T: Ord` |
-| `std::priority_queue<T>` | `std::collections::BinaryHeap<T>` | Max-heap by default (same as C++) |
-| `priority_queue<T>` | `BinaryHeap<T>` | 默认大顶堆（与 C++ 一致） |
-| `std::stack<T>` | `Vec<T>` with `.push()` / `.pop()` | No separate stack type needed |
-| `stack<T>` | `Vec<T>` | 无需单独的栈类型 |
-| `std::queue<T>` | `VecDeque<T>` with `.push_back()` / `.pop_front()` | No separate queue type needed |
-| `queue<T>` | `VecDeque<T>` | 无需单独的队列类型 |
-| `std::string` | `String` | UTF-8 guaranteed, not null-terminated |
-| `string` | `String` | 保证 UTF-8，不以 null 结尾 |
-| `std::string_view` | `&str` | Borrowed UTF-8 slice |
-| `string_view` | `&str` | 借用的 UTF-8 切片 |
-| `std::span<T>` (C++20) | `&[T]` / `&mut [T]` | Rust slices have been a first-class type since 1.0 |
-| `span<T>` (C++20) | `&[T]` / `&mut [T]` | Rust 切片自 1.0 起就是一等公民类型 |
-| `std::tuple<A, B, C>` | `(A, B, C)` | First-class syntax, destructurable |
-| `tuple<A, B, C>` | `(A, B, C)` | 一等公民语法，支持解构 |
-| `std::pair<A, B>` | `(A, B)` | Just a 2-element tuple |
-| `pair` | 二元元组 | 仅为两个元素的元组 |
-| `std::bitset<N>` | No std equivalent | Use the `bitvec` crate or `[u8; N/8]` |
-| `bitset<N>` | 标准库无对应 | 使用 `bitvec` crate 或 `[u8; N/8]` |
 
- **Key differences**:
+ **主要差异**：
- - Rust's `HashMap`/`HashSet` require `K: Hash + Eq` — the compiler enforces this at the type level, unlike C++ where using an unhashable key gives a template error deep in the STL
+ - Rust 的 `HashMap`/`HashSet` 要求 `K: Hash + Eq` —— 编译器在类型层面强制执行此要求，而不像 C++ 那样在 STL 深处报错。
- - `Vec` indexing (`v[i]`) panics on out-of-bounds by default. Use `.get(i)` for `Option<&T>` or iterators to avoid bounds checks entirely
+ - `Vec` 索引（`v[i]`）默认在越界时触发 panic。使用 `.get(i)` 获取 `Option<&T>` 或使用迭代器以完全避免边界检查。
- - No `std::multimap` or `std::multiset` — use `HashMap<K, Vec<V>>` or `BTreeMap<K, Vec<V>>`
+ - 没有 `std::multimap` 或 `std::multiset` —— 请使用 `HashMap<K, Vec<V>>` 或 `BTreeMap<K, Vec<V>>`。
 
 ---
 
- ### Exception Safety → Panic Safety
+ ### Exception Safety → Panic Safety / 异常安全性 → Panic 安全性
 
- C++ defines three levels of exception safety (Abrahams guarantees):
+ C++ 定义了三个级别的异常安全性（Abrahams 保证）：
 
-| C++ Level | Meaning | Rust Equivalent |
+| **C++ Level / C++ 级别** | **Meaning / 含义** | **Rust Equivalent / Rust 等效** |
 |----------|---------|----------------|
-| **No-throw** | Function never throws | Function never panics (returns `Result`) |
-| **No-throw** | 绝不抛出异常 | 绝不发生 panic (返回 `Result`) |
-| **Strong** (commit-or-rollback) | If it throws, state is unchanged | Ownership model makes this natural — if `?` returns early, partially built values are dropped |
-| **Strong** (提交或回滚) | 发生异常时状态不变 | 所有权模型使其变得自然 —— `?` 提前返回时部分构建的值会被 Drop |
-| **Basic** | If it throws, invariants are preserved | Rust's default — `Drop` runs, no leaks |
-| **Basic** | 发生异常时保持逻辑一致 | Rust 默认行为 —— `Drop` 执行，无泄漏 |
 
- #### How Rust's ownership model helps
+ #### How Rust's ownership model helps / Rust 所有权模型如何提供帮助
 
 ```rust
 // Strong guarantee for free — if file.write() fails, config is unchanged
+// 免费获得强力保证 —— 如果 file.write() 失败，config 保持不变
 fn update_config(config: &mut Config, path: &str) -> Result<(), Error> {
-    let new_data = fetch_from_network()?; // Err → early return, config untouched
+    let new_data = fetch_from_network()?; // Err -> early / 错误则提前返回，config 未受影响
-    let validated = validate(new_data)?;   // Err → early return, config untouched
+    let validated = validate(new_data)?;   // Err -> early / 错误则提前返回，config 未受影响
-    *config = validated;                   // Only reached on success (commit)
+    *config = validated;                   // Commit / 仅在成功时到达此处（提交）
     Ok(())
 }
 ```
 
- In C++, achieving the strong guarantee requires manual rollback or the copy-and-swap
+ 在 C++ 中，实现强力保证需要手动回滚或使用 copy-and-swap
- idiom. In Rust, `?` propagation gives you the strong guarantee by default for most code.
+ 惯用法。在 Rust 中，通过 `?` 的传播，大多数代码默认就能获得强力保证。
 
- #### `catch_unwind` — Rust's equivalent of `catch(...)`
+ #### `catch_unwind` — Rust's equivalent of `catch(...)` / `catch_unwind` —— Rust 版的 `catch(...)`
 
 ```rust
 use std::panic;
 
 // Catch a panic (like catch(...) in C++) — rarely needed
+// 捕获一个 panic（类似于 C++ 中的 catch(...)）—— 极少需要
 let result = panic::catch_unwind(|| {
-    // Code that might panic
+    // Might panic / 可能引发 panic 的代码
     let v = vec![1, 2, 3];
-    v[10]  // Panics! (index out of bounds)
+    v[10]  // Panics! / 触发 Panic！（越界索引）
 });
 
 match result {
     Ok(val) => println!("Got: {val}"),
-    Err(_) => eprintln!("Caught a panic — cleaned up"),
+    Err(_) => eprintln!("Caught / 捕获了 panic —— 已清理"),
 }
 ```
 
- #### `UnwindSafe` — marking types as panic-safe
+ #### `UnwindSafe` — marking types as panic-safe / `UnwindSafe` —— 将类型标记为 Panic 安全的
 
 ```rust
 use std::panic::UnwindSafe;
 
 // Types behind &mut are NOT UnwindSafe by default — the panic may have
+// 默认情况下 &mut 背后的类型不是 UnwindSafe 的 —— 因为 panic 可能
 // left them in a partially-modified state
+// 导致它们处于被部分修改的状态
 fn safe_execute<F: FnOnce() + UnwindSafe>(f: F) {
     let _ = std::panic::catch_unwind(f);
 }
 
 // Use AssertUnwindSafe to override when you've audited the code:
+// 当你审核过代码后，使用 AssertUnwindSafe 进行覆盖：
 use std::panic::AssertUnwindSafe;
 let mut data = vec![1, 2, 3];
 let _ = std::panic::catch_unwind(AssertUnwindSafe(|| {
     data.push(4);
 }));
 ```
 
-| C++ Exception Pattern | Rust Equivalent |
+| **C++ Exception Pattern / C++ 异常模式** | **Rust Equivalent / Rust 等效** |
 |-----------------------|-----------------|
-| `throw MyException()` | `return Err(MyError::...)` (preferred) or `panic!("...")` |
-| `throw` | `return Err` (推荐) 或 `panic!` |
-| `try { } catch (const E& e)` | `match result { Ok(v) => ..., Err(e) => ... }` or `?` |
-| `try / catch` | `match result` 或 `?` |
-| `catch (...)` | `std::panic::catch_unwind(...)` |
-| `catch (...)` | `std::panic::catch_unwind(...)` |
-| `noexcept` | `-> Result<T, E>` (errors are values, not exceptions) |
-| `noexcept` | `-> Result<T, E>` (错误是值，而非异常) |
-| RAII cleanup in stack unwinding | `Drop::drop()` runs during panic unwinding |
-| 栈展开中的 RAII 清理 | panic 展开期间执行 `Drop::drop()` |
-| `std::uncaught_exceptions()` | `std::thread::panicking()` |
-| `uncaught_exceptions()` | `std::thread::panicking()` |
-| `-fno-exceptions` compile flag | `panic = "abort"` in `Cargo.toml` [profile] |
-| `-fno-exceptions` 标志 | `Cargo.toml` 配置 `panic = "abort"` |
 
- > **Bottom line**: In Rust, most code uses `Result<T, E>` instead of exceptions,
+ > **底线**：在 Rust 中，大多数代码使用 `Result<T, E>` 而非异常，
- > making error paths explicit and composable. `panic!` is reserved for bugs
+ > 这使得错误路径变得显式且可组合。`panic!` 仅保留给程序 Bug
- > (like `assert!` failures), not routine errors. This means "exception safety"
+ > （如 `assert!` 失败），而非常规错误。这意味着“异常安全性”
- > is largely a non-issue — the ownership system handles cleanup automatically.
+ > 在很大程度上不再是一个问题 —— 所有权系统会自动处理清理工作。
 
 ---
 
- ## C++ to Rust Migration Patterns
+ ## C++ to Rust Migration Patterns / C++ 到 Rust 迁移模式
 
- ### Quick Reference: C++ → Rust Idiom Map
+ ### Quick Reference: C++ → Rust Idiom Map / 快速参考：C++ → Rust 惯用法映射
 
-| **C++ Pattern** | **Rust Idiom** | **Notes** |
+| **C++ Pattern / C++ 模式** | **Rust Idiom / Rust 惯用法** | **Notes / 说明** |
 |----------------|---------------|----------|
-| `class Derived : public Base` | `enum Variant { A {...}, B {...} }` | Prefer enums for closed sets |
-| 继承体系 | `enum Variant { ... }` | 封闭集合首选枚举 |
-| `virtual void method() = 0` | `trait MyTrait { fn method(&self); }` | Use for open/extensible interfaces |
-| 纯虚函数 | `trait MyTrait { ... }` | 用于开放/可扩展接口 |
-| `dynamic_cast<Derived*>(ptr)` | `match value { Variant::A(data) => ..., }` | Exhaustive, no runtime failure |
-| `dynamic_cast` | `match` / 模式匹配 | 穷尽性检查，无运行时失败 |
-| `vector<unique_ptr<Base>>` | `Vec<Box<dyn Trait>>` | Only when genuinely polymorphic |
-| `unique_ptr` 容器 | `Vec<Box<dyn Trait>>` | 仅在确实需要多态时使用 |
-| `shared_ptr<T>` | `Rc<T>` or `Arc<T>` | Prefer `Box<T>` or owned values first |
-| `shared_ptr<T>` | `Rc<T>` / `Arc<T>` | 优先使用 `Box` 或拥有所有权的值 |
-| `enable_shared_from_this<T>` | Arena pattern (`Vec<T>` + indices) | Eliminates reference cycles entirely |
-| `shared_from_this` | Arena 模式 / 索引 | 完全消除引用循环 |
-| `Base* m_pFramework` in every class | `fn execute(&mut self, ctx: &mut Context)` | Pass context, don't store pointers |
-| 持有父级指针 | 传递 Context 借用 | 传递上下文，不要存储指针 |
-| `try { } catch (...) { }` | `match result { Ok(v) => ..., Err(e) => ... }` | Or use `?` for propagation |
-| `try / catch` | `match result` | 或使用 `?` 进行传播 |
-| `std::optional<T>` | `Option<T>` | `match` required, can't forget None |
-| `optional<T>` | `Option<T>` | 强制匹配，不会忘记 None 场景 |
-| `const std::string&` parameter | `&str` parameter | Accepts both `String` and `&str` |
-| `const string&` 参数 | `&str` 参数 | 同时支持 `String` 和 `&str` |
-| `enum class Foo { A, B, C }` | `enum Foo { A, B, C }` | Rust enums can also carry data |
-| `enum class` | `enum` | Rust 枚举还可以携带数据 |
-| `auto x = std::move(obj)` | `let x = obj;` | Move is the default, no `std::move` needed |
-| `std::move` | `let x = obj;` | 默认即移动，无需 `std::move` |
-| CMake + make + lint | `cargo build / test / clippy / fmt` | One tool for everything |
+| CMake + lint | `cargo` 工具链 | 全能的一站式工具 |
 
- ### Migration Strategy
+ ### Migration Strategy / 迁移策略
- 1. **Start with data types**: Translate structs and enums first — this forces you to think about ownership
+ 1. **从数据类型开始**：首先翻译结构体和枚举 —— 这会强制你思考所有权问题。
- 2. **Convert factories to enums**: If a factory creates different derived types, it should probably be `enum` + `match`
+ 2. **将工厂转为枚举**：如果一个工厂创建不同的派生类型，它通常应该是一个 `enum` + `match` 结构。
- 3. **Convert god objects to composed structs**: Group related fields into focused structs
+ 3. **将上帝对象转为组合结构体**：将相关的字段分组到专门的结构体中。
- 4. **Replace pointers with borrows**: Convert `Base*` stored pointers to `&'a T` lifetime-bounded borrows
+ 4. **用借用替换指针**：将存储的 `Base*` 指针转换为受生命周期限制的 `&'a T` 借用。
- 5. **Use `Box<dyn Trait>` sparingly**: Only for plugin systems and test mocking
+ 5. **慎用 `Box<dyn Trait>`**：仅将其用于插件系统和测试 Mock。
- 6. **Let the compiler guide you**: Rust's error messages are excellent — read them carefully
+ 6. **让编译器引导你**：Rust 的错误消息非常出色 —— 请仔细阅读它们。
