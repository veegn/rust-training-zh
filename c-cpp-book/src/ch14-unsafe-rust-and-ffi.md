### Unsafe Rust / Unsafe Rust
 
 > **What you'll learn / 你将学到：** When and how to use `unsafe` — raw pointer dereferencing, FFI (Foreign Function Interface) for calling C from Rust and vice versa, `CString`/`CStr` for string interop, and how to write safe wrappers around unsafe code.
 >
 > 何时以及如何使用 `unsafe` —— 裸指针解引用、用于 Rust 与 C 互调的 FFI（外部函数接口）、用于字符串互操作的 `CString` / `CStr`，以及如何编写由于 `unsafe` 代码构成的安全包装层。
 
 - ```unsafe``` unlocks access to features that are normally disallowed by the Rust compiler
+ - ```unsafe``` 解锁了通常被 Rust 编译器禁止的功能访：
-     - Dereferencing raw pointers
+     - 解引用裸指针
-     - Accessing *mutable* static variables
+     - 访问**可变**静态变量
-     - https://doc.rust-lang.org/book/ch19-01-unsafe-rust.html
+     - 相关文档：https://doc.rust-lang.org/book/ch19-01-unsafe-rust.html
- - With great power comes great responsibility
+ - 权力越大，责任越大：
-     - ```unsafe``` tells the compiler "I, the programmer, take responsibility for upholding the invariants that the compiler normally guarantees"
+     - ```unsafe``` 告诉编译器：“我，作为程序员，负责维护编译器通常保证的不变量。”
-     - Must guarantee no aliased mutable and immutable references, no dangling pointers, no invalid references, ...
+     - 必须确保没有同时存在的可变与不可变引用别名、没有悬挂指针、没有无效引用等等。
-     - The use of ```unsafe``` should be limited to the smallest possible scope
+     - ```unsafe``` 的使用应限制在尽可能小的范围内。
-     - All code using ```unsafe``` should have a "safety" comment describing the assumptions
+     - 所有使用 ```unsafe``` 的代码都应包含一个“safety”注释，说明其所依赖的假设。
 
- ### Unsafe Rust examples
+ ### Unsafe Rust examples / Unsafe Rust 示例
 ```rust
 unsafe fn harmless() {}
 fn main() {
-    // Safety: We are calling a harmless unsafe function
+    // Safety: We are calling a harmless unsafe function / 安全说明：我们正在调用一个无害的 unsafe 函数
     unsafe {
         harmless();
     }
     let a = 42u32;
     let p = &a as *const u32;
-    // Safety: p is a valid pointer to a variable that will remain in scope
+    // Safety: p is a valid pointer / 安全说明：p 是一个指向仍在作用域内的变量的有效指针
     unsafe {
         println!("{}", *p);
     }
-    // Safety: Not safe; for illustration purposes only
+    // Safety: Not safe / 安全说明：此处并不安全；仅用于演示
     let dangerous_buffer = 0xb8000 as *mut u32;
     unsafe {
         println!("About to go kaboom!!!");
-        *dangerous_buffer = 0; // This will SEGV on most modern machines
+        *dangerous_buffer = 0; // SEGV expected / 这在大多数现代机器上会导致段错误（SEGV）
     }
 }
 ```
 
- ### Simple FFI example (Rust library function consumed by C)
+ ### Simple FFI example (Rust library function consumed by C) / 简单 FFI 示例（C 语言调用 Rust 库函数）
 
- ## FFI Strings: CString and CStr
+ ## FFI Strings: CString and CStr / FFI 字符串：CString 与 CStr
 
- FFI stands for *Foreign Function Interface* — the mechanism Rust uses to call functions written in other languages (such as C) and vice versa.
+ FFI 代表**外部函数接口（Foreign Function Interface）** —— 这是 Rust 用于调用其他语言（如 C）编写的函数，以及反向调用的机制。
 
- When interfacing with C code, Rust's `String` and `&str` types (which are UTF-8 without null terminators) aren't directly compatible with C strings (which are null-terminated byte arrays). Rust provides `CString` (owned) and `CStr` (borrowed) from `std::ffi` for this purpose:
+ 在与 C 代码接口时，Rust 的 `String` 和 `&str` 类型（不带空终止符的 UTF-8 编码）与 C 字符串（以空字符终止的字节数组）不直接兼容。为此，Rust 在 `std::ffi` 中提供了 `CString`（拥有所有权）和 `CStr`（借用）：
 
-| Type | Analogous to | Use when |
+| **Type / 类型** | **Analogous to / 类似于** | **Use when / 使用场景** |
 |------|-------------|----------|
-| `CString` | `String` (owned) | Creating a C string from Rust data |
+| `CString` | `String` (owned) | Creating C string from Rust / 从 Rust 数据创建 C 字符串 |
-| `&CStr` | `&str` (borrowed) | Receiving a C string from foreign code |
+| `&CStr` | `&str` (borrowed) | Receiving C string from foreign / 从外部代码接收 C 字符串 |
 
 ```rust
 use std::ffi::{CString, CStr};
 use std::os::raw::c_char;
 
 fn demo_ffi_strings() {
-    // Creating a C-compatible string (adds null terminator)
+    // Creating a C-compatible string / 创建 C 兼容字符串（添加空终止符）
     let c_string = CString::new("Hello from Rust").expect("CString::new failed");
     let ptr: *const c_char = c_string.as_ptr();
 
-    // Converting a C string back to Rust (unsafe because we trust the pointer)
-    // Safety: ptr is valid and null-terminated (we just created it above)
+    // Converting back to Rust / 将 C 字符串转回 Rust（unsafe，因为我们信任该指针）
+    // Safety: ptr is valid / 安全说明：ptr 有效且以空字符结尾（我们刚刚创建了它）
     let back_to_rust: &CStr = unsafe { CStr::from_ptr(ptr) };
     let rust_str: &str = back_to_rust.to_str().expect("Invalid UTF-8");
     println!("{}", rust_str);
 }
 ```
 
- > **Warning**: `CString::new()` will return an error if the input contains interior null bytes (`\0`). Always handle the `Result`. You'll see `CStr` used extensively in the FFI examples below.
+ > **警告**：如果输入内容包含内部空字节（`\0`），`CString::new()` 将返回错误。请务必处理 `Result`。在下面的 FFI 示例中，你将看到 `CStr` 的大量应用。
 
- - ```FFI``` methods must be marked with ```#[no_mangle]``` to ensure that the compiler doesn't mangle the name
+ - ```FFI``` 方法必须标记为 ```#[no_mangle]```，以确保编译器不会对名称进行“混淆（mangle）”。
- - We'll compile the crate as a static library
+ - 我们将把该 crate 编译为一个静态库。
     ```rust
     #[no_mangle] 
     pub extern "C" fn add(left: u64, right: u64) -> u64 {
         left + right
     }
     ```
- - We'll compile the following C-code and link it against our static library.
+ - 我们将编译以下 C 代码，并将其与我们的静态库链接。
     ```c
     #include <stdio.h>
     #include <stdint.h>
     extern uint64_t add(uint64_t, uint64_t);
     int main() {
         printf("Add returned %llu\n", add(21, 21));
     }
     ``` 
 
- ### Complex FFI example
+ ### Complex FFI example / 复杂的 FFI 示例
- - In the following examples, we'll create a Rust logging interface and expose it to
- [PYTHON] and ```C```
+ - 在接下来的示例中，我们将创建一个 Rust 日志接口，并将其暴露给 **Python** 和 **C**：
-     - We'll see how the same interface can be used natively from Rust and C
+     - 我们将了解同一个接口如何被 Rust 和 C 原生使用。
-     - We will explore the use of tools like ```cbindgen``` to generate header files for ```C```
+     - 我们将探索使用 ```cbindgen``` 之类的工具为 ```C``` 生成头文件。
-     - We will see how ```unsafe``` wrappers can act as a bridge to safe Rust code
+     - 我们将了解如何通过 ```unsafe``` 包装器作为通往安全 Rust 代码的桥梁。
 
- ## Logger helper functions
+ ## Logger helper functions / Logger 辅助函数
 ```rust
 fn create_or_open_log_file(log_file: &str, overwrite: bool) -> Result<File, String> {
     if overwrite {
         File::create(log_file).map_err(|e| e.to_string())
     } else {
         OpenOptions::new()
             .write(true)
             .append(true)
             .open(log_file)
             .map_err(|e| e.to_string())
     }
 }
 
 fn log_to_file(file_handle: &mut File, message: &str) -> Result<(), String> {
     file_handle
         .write_all(message.as_bytes())
         .map_err(|e| e.to_string())
 }
 ```
 
- ## Logger struct
+ ## Logger struct / Logger 结构体
 ```rust
 struct SimpleLogger {
     log_level: LogLevel,
     file_handle: File,
 }
 
 impl SimpleLogger {
     fn new(log_file: &str, overwrite: bool, log_level: LogLevel) -> Result<Self, String> {
         let file_handle = create_or_open_log_file(log_file, overwrite)?;
         Ok(Self {
             file_handle,
             log_level,
         })
     }
 
     fn log_message(&mut self, log_level: LogLevel, message: &str) -> Result<(), String> {
         if log_level as u32 <= self.log_level as u32 {
             let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
             let message = format!("Simple: {timestamp} {log_level} {message}\n");
             log_to_file(&mut self.file_handle, &message)
         } else {
             Ok(())
         }
     }
 }
 ```
 
- ## Testing
+ ## Testing / 测试
- - Testing functionality with Rust is trivial
+ - 使用 Rust 测试功能非常简单：
-     - Test methods are decorated with ```#[test]```, and aren't part of the compiled binary 
+     - 测试方法使用 ```#[test]``` 装饰，不属于编译后的二进制文件。
-     - It's easy to create mock methods for testing purposes
+     - 为测试目的创建 mock（模拟）方法非常容易。
 ```rust
 #[test]
 fn testfunc() -> Result<(), String> {
     let mut logger = SimpleLogger::new("test.log", false, LogLevel::INFO)?;
     logger.log_message(LogLevel::TRACELEVEL1, "Hello world")?;
     logger.log_message(LogLevel::CRITICAL, "Critical message")?;
-    Ok(()) // The compiler automatically drops logger here
+    Ok(()) // Automatically drops / 编译器在此处自动释放 logger
 }
 ```
 ```bash
 cargo test
 ```
 
- ## (C)-Rust FFI
+ ## (C)-Rust FFI / (C)-Rust FFI
- - cbindgen is a great tool for generating header files for exported Rust functions
+ - **cbindgen** 是为导出的 Rust 函数生成头文件的绝佳工具。
-     - Can be installed using cargo
+     - 可以使用 cargo 安装：
 ```bash
 cargo install cbindgen
 cbindgen 
 ```
- - Function and structures can be exported using ```#[no_mangle]``` and ```#[repr(C)]```
+ - 函数和结构体可以使用 ```#[no_mangle]``` 和 ```#[repr(C)]``` 导出。
-     - We'll assume the common interface pattern passing in a `**` to the actual implementation and returning 0 on success and non-zero on error
+     - 我们将采用通用的接口模式：向实际实现传递一个指向指针的指针（`**`），成功时返回 0，出错时返回非零值。
-     - **Opaque vs transparent structs**: Our `SimpleLogger` is passed as an *opaque pointer* (`*mut SimpleLogger`) — the C side never accesses its fields, so `#[repr(C)]` is **not** needed. Use `#[repr(C)]` when C code needs to read/write struct fields directly:
+     - **不透明（Opaque）与透明结构体**：我们的 `SimpleLogger` 是作为**不透明指针**（`*mut SimpleLogger`）传递的 —— C 侧永远不会访问其字段，因此**不需要** `#[repr(C)]`。只有当 C 代码需要直接读写结构体字段时，才必须使用 `#[repr(C)]`：
 
 ```rust
-// Opaque — C only holds a pointer, never inspects fields. No #[repr(C)] needed.
+// Opaque — C holds pointer only / 不透明 —— C 仅持有指针，不检查字段。无需 #[repr(C)]。
 struct SimpleLogger { /* Rust-only fields */ }
 
-// Transparent — C reads/writes fields directly. MUST use #[repr(C)].
+// Transparent — C accesses fields / 透明 —— C 直接读写字段。必须使用 #[repr(C)]。
 #[repr(C)]
 pub struct Point {
     pub x: f64,
     pub y: f64,
 }
 ```
 ```c
 typedef struct SimpleLogger SimpleLogger;
 uint32_t create_simple_logger(const char *file_name, struct SimpleLogger **out_logger);
 uint32_t log_entry(struct SimpleLogger *logger, const char *message);
 uint32_t drop_logger(struct SimpleLogger *logger);
 ```
 
- - Note that we need to a lot of sanity checks
+ - 请注意，我们需要进行大量的健全性检查。
- - We have to explicitly leak memory to prevent Rust from automatically deallocating
+ - 我们必须显式地进行内存“泄漏（leak）”，以防止 Rust 自动释放内存。
 ```rust
 #[no_mangle] 
 pub extern "C" fn create_simple_logger(file_name: *const std::os::raw::c_char, out_logger: *mut *mut SimpleLogger) -> u32 {
     use std::ffi::CStr;
-    // Make sure pointer isn't NULL
+    // Make sure pointer is not NULL / 确保指针不是 NULL
     if file_name.is_null() || out_logger.is_null() {
         return 1;
     }
-    // Safety: The passed in pointer is either NULL or 0-terminated by contract
+    // Safety / 安全说明：根据契约，传入的指针必然以空字符结尾
     let file_name = unsafe {
         CStr::from_ptr(file_name)
     };
     let file_name = file_name.to_str();
-    // Make sure that file_name doesn't have garbage characters
+    // Check for garbage / 确保文件名中没有乱码
     if file_name.is_err() {
         return 1;
     }
     let file_name = file_name.unwrap();
-    // Assume some defaults; we'll pass them in in real life
+    // Use defaults / 假定一些默认值；现实中我们会把它们传进来
     let new_logger = SimpleLogger::new(file_name, false, LogLevel::CRITICAL);
-    // Check that we were able to construct the logger
+    // Check construction / 检查是否能成功构造 logger
     if new_logger.is_err() {
         return 1;
     }
     let new_logger = Box::new(new_logger.unwrap());
-    // This prevents the Box from being dropped when if goes out of scope
+    // Leak the Box / 防止 Box 在超出作用域时被释放
     let logger_ptr: *mut SimpleLogger = Box::leak(new_logger);
-    // Safety: logger is non-null and logger_ptr is valid
+    // Safety / 安全说明：logger 非空且 logger_ptr 有效
     unsafe {
         *out_logger = logger_ptr;
     }
     return 0;
 }
 ```
 
- - We have similar error checks in ```log_entry()```
+ - 我们在 ```log_entry()``` 中也有类似的错误检查。
 ```rust
 #[no_mangle]
 pub extern "C" fn log_entry(logger: *mut SimpleLogger, message: *const std::os::raw::c_char) -> u32 {
     use std::ffi::CStr;
     if message.is_null() || logger.is_null() {
         return 1;
     }
-    // Safety: message is non-null
+    // Safety / 安全说明：message 非空
     let message = unsafe {
         CStr::from_ptr(message)
     };
     let message = message.to_str();
-    // Make sure that file_name doesn't have garbage characters
+    // Check for garbage / 确保消息中没有乱码
     if message.is_err() {
         return 1;
     }
-    // Safety: logger is valid pointer previously constructed by create_simple_logger()
+    // Safety: previously constructed / 安全说明：logger 是由 create_simple_logger() 构造的有效指针
     unsafe {
         (*logger).log_message(LogLevel::CRITICAL, message.unwrap()).is_err() as u32
     }
 }
 
 #[no_mangle]
 pub extern "C" fn drop_logger(logger: *mut SimpleLogger) -> u32 {
     if logger.is_null() {
         return 1;
     }
-    // Safety: logger is valid pointer previously constructed by create_simple_logger()
+    // Safety: previously constructed / 安全说明：logger 是由 create_simple_logger() 构造的有效指针
     unsafe {
-        // This constructs a Box<SimpleLogger>, which is dropped when it goes out of scope
+        // Re-construct the Box / 重新构造 Box<SimpleLogger>，在其超出作用域时会被释放
         let _ = Box::from_raw(logger);
     }
     0
 }
 ```
 
- - We can test our (C)-FFI using Rust, or by writing a (C)-program
+ - 我们可以使用 Rust 或编写 C 程序来测试我们的 (C)-FFI。
 ```rust
 #[test]
 fn test_c_logger() {
-    // The c".." creates a NULL terminated string
+    // c".." creates NULL terminated / c".." 会创建一个空终止的字符串
     let file_name = c"test.log".as_ptr() as *const std::os::raw::c_char;
     let mut c_logger: *mut SimpleLogger = std::ptr::null_mut();
     assert_eq!(create_simple_logger(file_name, &mut c_logger), 0);
-    // This is the manual way to create c"..." strings
+    // Manual way for null termination / 带有空终止符的手动方式
     let message = b"message from C\0".as_ptr() as *const std::os::raw::c_char;
     assert_eq!(log_entry(c_logger, message), 0);
     drop_logger(c_logger);
 }
 ```
 ```c
 #include "logger.h"
 // ...
 int main() {
     SimpleLogger *logger = NULL;
     if (create_simple_logger("test.log", &logger) == 0) {
         log_entry(logger, "Hello from C");
-        drop_logger(logger); /*Needed to close handle, etc.*/
+        drop_logger(logger); /* Close handle / 需要关闭句柄等 */
     } 
-    ...
+    // ...
 }
 ```
 
- ## Ensuring correctness of unsafe code
+ ## Ensuring correctness of unsafe code / 确保 Unsafe 代码的正确性
 
- The TL;DR version is that using ```unsafe``` requires deliberate thought
+ 简而言之，使用 ```unsafe``` 需要深思熟虑：
-     - Always document the safety assumptions made by the code and review it with experts
+     - 始终记录代码所做的安全假设，并请专家进行审查。
-     - Use tools like cbindgen, Miri, Valgrind that can help verify correctness
+     - 使用 cbindgen、Miri、Valgrind 等有助于验证正确性的工具。
-     - **Never let a panic unwind across an FFI boundary** — this is UB. Use `std::panic::catch_unwind` at FFI entry points, or configure `panic = "abort"` in your profile
+     - **绝不允许 panic 跨越 FFI 边界传播** —— 这是未定义行为。在 FFI 入口点使用 `std::panic::catch_unwind`，或者在配置文件中配置 `panic = "abort"`。
-     - If a struct is shared across FFI, mark it `#[repr(C)]` to guarantee C-compatible memory layout
+     - 如果结构体是在 FFI 间共享的，请将其标记为 `#[repr(C)]` 以保证 C 兼容的内存布局。
-     - Consult https://doc.rust-lang.org/nomicon/intro.html (the "Rustonomicon" — the dark arts of unsafe Rust)
+     - 参考 https://doc.rust-lang.org/nomicon/intro.html（《Rustonomicon》 —— 深入 unsafe Rust 的黑魔法）。
-     - Seek help of internal experts
+     - 寻求内部专家的帮助。
 
- ### Verification tools: Miri vs Valgrind
+ ### Verification tools: Miri vs Valgrind / 验证工具：Miri vs Valgrind
 
- C++ developers are familiar with Valgrind and sanitizers. Rust has those **plus** Miri, which is far more precise for Rust-specific UB:
+ C++ 开发者对 Valgrind 和 sanitizers 很熟悉。Rust 不仅有这些，还有 **Miri**，它对于 Rust 特有的未定义行为更加精确：
 
-| | **Miri** | **Valgrind** | **C++ sanitizers (ASan/MSan/UBSan)** |
+| **Feature / 特性** | **Miri** | **Valgrind** | **C++ sanitizers** |
 |---|---------|-------------|----------------------------|
-| **What it catches** | Rust-specific UB: stacked borrows, invalid `enum` discriminants, uninitialized reads, aliasing violations | Memory leaks, use-after-free, invalid reads/writes, uninitialized memory | Buffer overflow, use-after-free, data races, UB |
+| **What it catches / 捕获内容** | Rust-specific UB / Rust 特有 UB | Memory leaks / 内存泄漏等 | Buffer overflow / 缓冲区溢出等 |
-| **How it works** | Interprets MIR (Rust's mid-level IR) — no native execution | Instruments compiled binary at runtime | Compile-time instrumentation |
+| **Mechanism / 机制** | Interprets MIR / 解释 MIR | Runtime instr. / 运行时插桩 | Compile instr. / 编译时插桩 |
-| **FFI support** | ❌ Cannot cross FFI boundary (skips C calls) | ✅ Works on any compiled binary, including FFI | ✅ Works if C code also compiled with sanitizers |
+| **FFI support / FFI 支持** | ❌ NO / 否 | ✅ YES / 是 | ✅ YES / 是 |
-| **Speed** | ~100x slower than native | ~10-50x slower | ~2-5x slower |
+| **Speed / 速度** | ~100x slower | ~10-50x slower | ~2-5x slower |
-| **When to use** | Pure Rust `unsafe` code, data structure invariants | FFI code, full binary integration tests | C/C++ side of FFI, performance-sensitive testing |
+| **When to use / 适用场景** | Pure Rust / 纯 Rust unsafe | FFI / 全局集成测试 | C/C++ 端的 FFI 检测 |
-| **Catches aliasing bugs** | ✅ Stacked Borrows model | ❌ | Partially (TSan for data races) |
+| **Aliasing bugs / 别名缺陷** | ✅ YES / 是 | ❌ NO / 否 | Partially / 部分 |
 
- **Recommendation**: Use **both** — Miri for pure Rust unsafe, Valgrind for FFI integration:
+ **建议**：双管齐下 —— 对纯 Rust unsafe 代码使用 Miri，对 FFI 集成使用 Valgrind：
 
- - **Miri** — catches Rust-specific UB that Valgrind cannot see (aliasing violations, invalid enum values, stacked borrows):
+ - **Miri** —— 捕获 Valgrind 无法识别的 Rust 特有 UB（别名冲突、无效枚举值、stacked borrows）：
     ```bash
     rustup +nightly component add miri
     cargo +nightly miri test                    # Run all tests / 运行所有测试
     cargo +nightly miri test -- test_name       # Run specific / 运行特定测试
     ```
-     > ⚠️ Miri requires nightly and cannot execute FFI calls. Isolate unsafe Rust logic into testable units.
+     > ⚠️ Miri 需要 nightly 版本且无法执行 FFI 调用。请将 unsafe Rust 逻辑隔离为可测试的单元。
 
- - **Valgrind** — the tool you already know, works on the compiled binary including FFI:
+ - **Valgrind** —— 你熟悉的工具，作用于包括 FFI 在内的已编译二进制文件：
     ```bash
     sudo apt install valgrind
     cargo install cargo-valgrind
     cargo valgrind test                         # Run all / 运行所有测试
     ```
-     > Catches leaks in `Box::leak` / `Box::from_raw` patterns common in FFI code.
+     > 捕获 FFI 代码中常见的 `Box::leak` / `Box::from_raw` 模式下的泄露。
 
- - **cargo-careful** — runs tests with extra runtime checks enabled (between regular tests and Miri):
+ - **cargo-careful** —— 运行时执行额外的安全检查（由于普通测试和 Miri 之间）：
     ```bash
     cargo install cargo-careful
     cargo +nightly careful test
     ```
 
- ## Unsafe Rust summary
+ ## Unsafe Rust summary / Unsafe Rust 总结
- - ```cbindgen``` is a great tool for (C) FFI to Rust
+ - **cbindgen** 是实现 (C) FFI 到 Rust 的利器。
-     - Use ```bindgen``` for FFI-interfaces in the other direction (consult the extensive documentation)
+     - 另一个方向的 FFI 接口请使用 **bindgen**（详见其丰富文档）。
- - **Do not assume that your unsafe code is correct, or that it's fine to use from safe Rust. It's really easy to make mistakes, and even code that seemingly works correctly can be wrong for subtle reasons**
+ - **不要假定你的 unsafe 代码是正确的，或者它可以直接从安全 Rust 中调用。犯错非常容易，即使是表面上运行正常的代码也可能因为细微原因而存在错误。**
-     - Use tools to verify correctness
+     - 使用工具验证正确性。
-     - If still in doubt, reach out for expert advice
+     - 如有疑虑，请寻求专家建议。
- - Make sure that your ```unsafe``` code has comments with an explicit documentation about assumptions and why it's correct
+ - 确保你的 ```unsafe``` 代码带有明确记录假设及正确性理由的注释。
-     - Callers of ```unsafe``` code should have corresponding comments on safety as well, and observe restrictions
+ - ```unsafe``` 代码的调用者也应当有相应的安全注释，并遵守相关限制。
 
- # Exercise: Writing a safe FFI wrapper
+ # Exercise: Writing a safe FFI wrapper / 练习：编写安全 FFI 包装器
 
- 🔴 **Challenge** — requires understanding unsafe blocks, raw pointers, and safe API design
+ 🔴 **Challenge / 挑战题** —— 需要理解 unsafe 块、裸指针和安全 API 设计
 
- - Write a safe Rust wrapper around an `unsafe` FFI-style function. The exercise simulates calling a C function that writes a formatted string into a caller-provided buffer.
+ - 为一个 `unsafe` 的 FFI 风格函数编写一个安全的 Rust 包装器。本练习模拟调用一个 C 函数，该函数将格式化字符串写入调用者提供的缓冲区。
- - **Step 1**: Implement the unsafe function `unsafe_greet` that writes a greeting into a raw `*mut u8` buffer
+ - **步骤 1**：实现 unsafe 函数 `unsafe_greet`，它将问候语写入一个裸 `*mut u8` 缓冲区。
- - **Step 2**: Write a safe wrapper `safe_greet` that allocates a `Vec<u8>`, calls the unsafe function, and returns a `String`
+ - **步骤 2**：编写一个安全包装器 `safe_greet`，它分配一个 `Vec<u8>`，调用该 unsafe 函数，并返回一个 `String`。
- - **Step 3**: Add proper `// Safety:` comments to every unsafe block
+ - **步骤 3**：为每个 unsafe 块添加妥当的 `// Safety:` 注释。
 
- **Starter code:**
+ **Starter code / 入门代码：**
 ```rust
 use std::fmt::Write as _;
 
- /// Simulates a C function: writes "Hello, <name>!" into buffer.
+ /// Simulates a C function / 模拟 C 函数：将 "Hello, <name>!" 写入缓冲区
 /// Returns the number of bytes written (excluding null terminator).
 /// # Safety
 /// - `buf` must point to at least `buf_len` writable bytes
 /// - `name` must be a valid pointer to a null-terminated C string
 unsafe fn unsafe_greet(buf: *mut u8, buf_len: usize, name: *const u8) -> isize {
-    // TODO: Build greeting, copy bytes into buf, return length
-    // Hint: use std::ffi::CStr::from_ptr or iterate bytes manually
+    // TODO: Build greeting / 待办：构建问候语，将字节拷贝到 buf，返回长度
+    // Hint / 提示：使用 std::ffi::CStr::from_ptr 或手动遍历字节
     todo!()
 }
 
- /// Safe wrapper — no unsafe in the public API
+ /// Safe wrapper / 安全包装器 —— 公开 API 中没有 unsafe
 fn safe_greet(name: &str) -> Result<String, String> {
-    // TODO: Allocate a Vec<u8> buffer, create a null-terminated name,
-    // call unsafe_greet inside an unsafe block with Safety comment,
-    // convert the result back to a String
+    // TODO / 待办：分配 Vec<u8> 缓冲区，创建一个以空字符结尾的名称，
+    // 在带有 Safety 注释的 unsafe 块中调用 unsafe_greet，
+    // 将结果转回 String
     todo!()
 }
 
 fn main() {
     match safe_greet("Rustacean") {
         Ok(msg) => println!("{msg}"),
         Err(e) => eprintln!("Error: {e}"),
     }
-    // Expected output: Hello, Rustacean!
+    // Expected / 预期输出：Hello, Rustacean!
 }
 ```
 
- <details><summary>Solution (click to expand)</summary>
+ <details><summary>Solution (click to expand) / 解决方案（点击展开）</summary>
 
 ```rust
 use std::ffi::CStr;
 
- /// Simulates a C function: writes "Hello, <name>!" into buffer.
- /// Returns the number of bytes written, or -1 if buffer too small.
+ /// Simulates a C function / 模拟 C 函数：写入问候语，返回长度或 -1
 /// # Safety
 /// - `buf` must point to at least `buf_len` writable bytes
 /// - `name` must be a valid pointer to a null-terminated C string
 unsafe fn unsafe_greet(buf: *mut u8, buf_len: usize, name: *const u8) -> isize {
-    // Safety: caller guarantees name is a valid null-terminated string
+    // Safety / 安全说明：调用者保证 name 是有效的空终止字符串
     let name_cstr = unsafe { CStr::from_ptr(name as *const std::os::raw::c_char) };
     let name_str = match name_cstr.to_str() {
         Ok(s) => s,
         Err(_) => return -1,
     };
     let greeting = format!("Hello, {}!", name_str);
     if greeting.len() > buf_len {
         return -1;
     }
-    // Safety: buf points to at least buf_len writable bytes (caller guarantee)
+    // Safety / 安全说明：buf 指向至少 buf_len 个可写字节
     unsafe {
         std::ptr::copy_nonoverlapping(greeting.as_ptr(), buf, greeting.len());
     }
     greeting.len() as isize
 }
 
- /// Safe wrapper — no unsafe in the public API
+ /// Safe wrapper / 安全包装器
 fn safe_greet(name: &str) -> Result<String, String> {
     let mut buffer = vec![0u8; 256];
-    // Create a null-terminated version of name for the C API
+    // Create null-terminated / 为 C API 创建空终止的名称
     let name_with_null: Vec<u8> = name.bytes().chain(std::iter::once(0)).collect();
 
-    // Safety: buffer has 256 writable bytes, name_with_null is null-terminated
+    // Safety / 安全说明：buffer 有 256 个可写字节，name_with_null 已空终止
     let bytes_written = unsafe {
         unsafe_greet(buffer.as_mut_ptr(), buffer.len(), name_with_null.as_ptr())
     };
 
     if bytes_written < 0 {
         return Err("Buffer too small or invalid name".to_string());
     }
 
     String::from_utf8(buffer[..bytes_written as usize].to_vec())
         .map_err(|e| format!("Invalid UTF-8: {e}"))
 }
 
 fn main() {
     match safe_greet("Rustacean") {
         Ok(msg) => println!("{msg}"),
         Err(e) => eprintln!("Error: {e}"),
     }
 }
-// Output:
+// Output / 输出：
 // Hello, Rustacean!
 ```
 
 </details>
