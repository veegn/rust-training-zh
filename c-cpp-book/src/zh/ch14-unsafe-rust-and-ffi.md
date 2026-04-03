[English Original](../en/ch14-unsafe-rust-and-ffi.md)

### 不安全 Rust (Unsafe Rust)

> **你将学到：** 何时以及如何使用 `unsafe` —— 解引用裸指针、用于 Rust 与 C 互调的 FFI（外部函数接口）、用于字符串交互的 `CString`/`CStr`，以及如何为不安全代码编写安全包装器。

- `unsafe` 关键字解锁了 Rust 编译器通常禁止访问的功能：
    - 解引用裸指针。
    - 访问*可变*静态变量。
    - 更多内容请参考：https://doc.rust-lang.org/book/ch19-01-unsafe-rust.html
- 能力越大，责任越大：
    - `unsafe` 告诉编译器：“我，程序员，负责维护编译器通常保证的各项不变性。”
    - 必须保证不存在重叠的可变和不可变引用、无悬垂指针、无无效引用等。
    - `unsafe` 的使用应限制在尽可能小的范围内。
    - 所有使用 `unsafe` 的代码都应附带“Safety”注释，描述其背后的假设。

---

### 不安全 Rust 示例
```rust
unsafe fn harmless() {}
fn main() {
    // 安全性：我们正在调用一个无害的不安全函数
    unsafe {
        harmless();
    }
    let a = 42u32;
    let p = &a as *const u32;
    // 安全性：p 是指向一个仍处于作用域内的变量的有效指针
    unsafe {
        println!("{}", *p);
    }
    // 安全性：不安全；此处仅用于演示
    let dangerous_buffer = 0xb8000 as *mut u32;
    unsafe {
        println!("即将发生崩溃！！！");
        *dangerous_buffer = 0; // 在大多数现代机器上这会导致段错误 (SEGV)
    }
}
```

---

# FFI 字符串：CString 与 CStr

FFI 代表*外部函数接口*（Foreign Function Interface）—— Rust 用于调用其他语言（如 C）编写的函数以及被其他语言调用的机制。

在与 C 代码交互时，Rust 的 `String` 和 `&str` 类型（采用不含空终止符的 UTF-8 编码）无法直接与 C 字符串（以空字符终止的字节数组）兼容。为此，Rust 在 `std::ffi` 中提供了 `CString`（所有权型）和 `CStr`（借用型）：

| 类型 | 类似于 | 使用场景 |
|------|-------------|----------|
| `CString` | `String` (所有权) | 从 Rust 数据创建 C 字符串 |
| `&CStr` | `&str` (借用) | 从外部代码接收 C 字符串 |

```rust
use std::ffi::{CString, CStr};
use std::os::raw::c_char;

fn demo_ffi_strings() {
    // 创建 C 兼容字符串（添加空终止符 \0）
    let c_string = CString::new("Hello from Rust").expect("CString::new 失败");
    let ptr: *const c_char = c_string.as_ptr();

    // 将 C 字符串转换回 Rust（因信任指针而具有潜在不安全性）
    // 安全性：ptr 有效且以空字符终止（我们在上文刚创建了它）
    let back_to_rust: &CStr = unsafe { CStr::from_ptr(ptr) };
    let rust_str: &str = back_to_rust.to_str().expect("无效的 UTF-8");
    println!("{}", rust_str);
}
```

> **注意**：如果输入中包含内部空字节（`\0`），`CString::new()` 将返回错误。请务必处理 `Result`。在下文的 FFI 示例中，你将看到 `CStr` 的广泛应用。

---

### 简单 FFI 示例（由 C 取用的 Rust 库函数）

- `FFI` 方法必须标记为 `#[no_mangle]`，以确保编译器不会混淆（mangle）其名称。
- 我们将该 crate 编译为一个静态库。
    ```rust
    #[no_mangle] 
    pub extern "C" fn add(left: u64, right: u64) -> u64 {
        left + right
    }
    ```
- 我们将编译如下 C 代码，并将其与我们的静态库链接。
    ```c
    #include <stdio.h>
    #include <stdint.h>
    extern uint64_t add(uint64_t, uint64_t);
    int main() {
        printf("Add 返回了 %llu\n", add(21, 21));
    }
    ``` 

---

### 复杂 FFI 示例
- 在接下来的示例中，我们将创建一个 Rust 日志接口并将其暴露给 [PYTHON] 和 `C`。
    - 我们将看到同一个接口如何被 Rust 和 C 原生地使用。
    - 我们将探索使用 `cbindgen` 等工具为 `C` 生成头文件。
    - 我们将看到 `unsafe` 包装器如何充当通往安全 Rust 代码的桥梁。

## 日志助手函数
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

---

## 日志结构体 (Logger struct)
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

---

## 测试
- 使用 Rust 完成功能测试非常简单。
    - 测试方法采用 `#[test]` 进行装饰，且不会成为编译产物的一部分。
    - 出于测试目的创建模拟 (mock) 方法非常方便。
```rust
#[test]
fn testfunc() -> Result<(), String> {
    let mut logger = SimpleLogger::new("test.log", false, LogLevel::INFO)?;
    logger.log_message(LogLevel::TRACELEVEL1, "Hello world")?;
    logger.log_message(LogLevel::CRITICAL, "Critical message")?;
    Ok(()) // 编译器在此处自动 drop 掉 logger
}
```
```bash
cargo test
```

---

## (C)-Rust FFI
- `cbindgen` 是一个用于为导出的 Rust 函数生成头文件的极佳工具。
    - 可以使用 cargo 进行安装。
```bash
cargo install cbindgen
cbindgen 
```
- 函数和结构体可以使用 `#[no_mangle]` 和 `#[repr(C)]` 进行导出。
    - 既然要遵循通用的接口模式，我们需要将 `**` 传递给实际实现，成功返回 0，出错返回非 0 值。
    - **不透明（Opaque）与透明（Transparent）结构体**：我们的 `SimpleLogger` 是作为*不透明指针* (`*mut SimpleLogger`) 传递的 —— C 端永远不会访问其字段，因此**不需要** `#[repr(C)]`。只有当 C 代码需要直接读写结构体字段时，才使用 `#[repr(C)]`：

```rust
// 不透明 —— C 只持有指针，从不检查字段。不需要 #[repr(C)]。
struct SimpleLogger { /* 仅限 Rust 的字段 */ }

// 透明 —— C 直接读写字段。必须使用 #[repr(C)]。
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

---

- 注意我们需要进行大量的健全性检查。
- 我们必须显式地进行内存泄漏处理，以防止 Rust 自动释放内存。
```rust
#[no_mangle] 
pub extern "C" fn create_simple_logger(file_name: *const std::os::raw::c_char, out_logger: *mut *mut SimpleLogger) -> u32 {
    use std::ffi::CStr;
    // 确保指针不为 NULL
    if file_name.is_null() || out_logger.is_null() {
        return 1;
    }
    // 安全性：根据约定，传入的指针要么为 NULL，要么是以空字符结尾的
    let file_name = unsafe {
        CStr::from_ptr(file_name)
    };
    let file_name = file_name.to_str();
    // 确保 file_name 中不包含乱码
    if file_name.is_err() {
        return 1;
    }
    let file_name = file_name.unwrap();
    // 假定一些默认值；实际应用中我们会传入这些值
    let new_logger = SimpleLogger::new(file_name, false, LogLevel::CRITICAL);
    // 检查是否能够成功构造 logger
    if new_logger.is_err() {
        return 1;
    }
    let new_logger = Box::new(new_logger.unwrap());
    // 这可以防止 Box 在离开作用域时被释放
    let logger_ptr: *mut SimpleLogger = Box::leak(new_logger);
    // 安全性：logger 非空且 logger_ptr 有效
    unsafe {
        *out_logger = logger_ptr;
    }
    return 0;
}
```

---

- 我们在 `log_entry()` 中也有类似的错误检查。
```rust
#[no_mangle]
pub extern "C" fn log_entry(logger: *mut SimpleLogger, message: *const std::os::raw::c_char) -> u32 {
    use std::ffi::CStr;
    if message.is_null() || logger.is_null() {
        return 1;
    }
    // 安全性：message 非空
    let message = unsafe {
        CStr::from_ptr(message)
    };
    let message = message.to_str();
    // 确保 message 中不包含乱码
    if message.is_err() {
        return 1;
    }
    // 安全性：logger 是先前由 create_simple_logger() 构造的正确定向指针
    unsafe {
        (*logger).log_message(LogLevel::CRITICAL, message.unwrap()).is_err() as u32
    }
}

#[no_mangle]
pub extern "C" fn drop_logger(logger: *mut SimpleLogger) -> u32 {
    if logger.is_null() {
        return 1;
    }
    // 安全性：logger 是先前由 create_simple_logger() 构造的正确定向指针
    unsafe {
        // 这将构造一个 Box<SimpleLogger>，它在离开作用域时会被释放
        let _ = Box::from_raw(logger);
    }
    0
}
```

---

- 我们可以使用 Rust 或编写 C 程序来测试我们的 (C)-FFI。
```rust
#[test]
fn test_c_logger() {
    // c".." 创建一个以空字符结尾的字符串
    let file_name = c"test.log".as_ptr() as *const std::os::raw::c_char;
    let mut c_logger: *mut SimpleLogger = std::ptr::null_mut();
    assert_eq!(create_simple_logger(file_name, &mut c_logger), 0);
    // 这是手动创建 c"..." 字符串的方法
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
        drop_logger(logger); /* 需要关闭句柄等操作 */
    } 
    // ...
}
```

---

## 确保不安全代码的正确性
- 简而言之，使用 `unsafe` 需要经过深思熟虑。
    - 始终记录代码的安全假设，并组织专家进行评审。
    - 使用 `cbindgen`、`Miri`、`Valgrind` 等工具来辅助验证正确性。
    - **绝不让 panic 跨越 FFI 边界传播** —— 这是未定义行为。请在 FFI 入口点使用 `std::panic::catch_unwind`，或者在你的 profile 中配置 `panic = "abort"`。
    - 如果结构体在 FFI 之间共享，请标记为 `#[repr(C)]` 以保证其内存布局与 C 兼容。
    - 请查阅 https://doc.rust-lang.org/nomicon/intro.html（《Rustonomicon》—— 介绍不安全 Rust 的“黑魔法”）。
    - 寻求团队内部专家的帮助。

### 验证工具：Miri 对比 Valgrind

C++ 开发者熟悉 Valgrind 和各类 sanitizer。Rust 除了提供这些工具，还拥有针对 Rust 特有未定义行为（UB）更为精准的 **Miri**。

| | **Miri** | **Valgrind** | **C++ sanitizers (ASan/MSan/UBSan)** |
|---|---------|-------------|--------------------------------------|
| **捕捉内容** | Rust 特有的 UB：Stacked Borrows、无效枚举判别值、未初始化读取、别名违反 | 内存泄漏、释放后使用、无效读/写、未初始化内存 | 缓冲区溢出、释放后使用、数据竞争、UB |
| **工作原理** | 解释 MIR（Rust 的中级 IR）—— 非原生执行 | 在运行时对编译后的二进制进行插桩 | 编译时插桩 |
| **FFI 支持** | ❌ 无法跨越 FFI 边界（跳过 C 调用） | ✅ 适用于任何编译后的二进制，包括 FFI | ✅ 只要 C 代码也使用了 sanitizer 编译即可 |
| **运行速度** | 比原生慢约 100 倍 | 慢约 10-50 倍 | 慢约 2-5 倍 |
| **使用时机** | 纯 Rust `unsafe` 代码、数据结构不变性验证 | FFI 代码、完整二进制的集成测试 | FFI 的 C/C++ 端、性能敏感型测试 |
| **捕捉别名 Bug** | ✅ Stacked Borrows 模型 | ❌ | 部分（TSan 可捕捉数据竞争） |

---

**建议**：两者结合使用 —— 针对纯 Rust 的不安全代码使用 Miri，针对 FFI 集成使用 Valgrind：

- **Miri** —— 捕捉 Valgrind 看不到的 Rust 特有 UB（如别名违反、无效枚举值、stacked borrows 等）：
    ```bash
    rustup +nightly component add miri
    cargo +nightly miri test                    # 在 Miri 下运行所有测试
    cargo +nightly miri test -- test_name       # 运行特定测试
    ```
    > ⚠️ Miri 需要使用 nightly 版本且无法执行 FFI 调用。请将不安全的 Rust 逻辑隔离为可独立测试的单元。

- **Valgrind** —— 你已经熟悉的工具，适用于包括 FFI 在内的编译后的二进制：
    ```bash
    sudo apt install valgrind
    cargo install cargo-valgrind
    cargo valgrind test                         # 在 Valgrind 下运行所有测试
    ```
    > 它可以捕捉 FFI 代码中常见的 `Box::leak` / `Box::from_raw` 模式导致的内存泄漏。

- **cargo-careful** —— 开启额外的运行时检查来运行测试（介于常规测试与 Miri 之间）：
    ```bash
    cargo install cargo-careful
    cargo +nightly careful test
    ```

## 不安全 Rust 小结
- `cbindgen` 是用于 Rust (C) FFI 的绝佳工具。
    - 另一个方向的 FFI 接口请使用 `bindgen`（请查阅其详尽的文档）。
- **不要理所当然地认为你的不安全代码是正确的，或者认为它可以安全地在安全 Rust 中使用。由于一些微妙的原因，即使看起来运行正确的代码也可能是错误的。**
    - 使用相关工具验证正确性。
    - 若仍有疑问，请咨询专家。
- 确保你的 `unsafe` 代码包含详尽的注释，记录其背后的假设以及正确性的原因。
    - `unsafe` 代码的调用者也应在安全性方面附带相应的注释，并遵守相关限制。

---

# 练习：编写安全 FFI 包装器

🔴 **挑战** —— 需要理解不安全块、裸指针和安全 API 设计

- 实现一个围绕 `unsafe` FFI 风格函数的安全 Rust 包装器。本练习模拟调用一个 C 函数，该函数向调用方提供的缓冲区中写入一个格式化的字符串。
- **步骤 1**：实现不安全函数 `unsafe_greet`，它将向裸 `*mut u8` 缓冲区中写入问候语。
- **步骤 2**：编写安全包装器 `safe_greet`，它分配一个 `Vec<u8>`，调用该不安全函数，并返回一个 `String`。
- **步骤 3**：为每个不安全块添加恰当的 `// Safety:` 注释。

**初始代码：**
```rust
use std::fmt::Write as _;

/// 模拟 C 函数：向缓冲区写入 "Hello, <name>!"。
/// 返回写入的字节数（不包括空终止符 \0）。
/// # Safety
/// - `buf` 必须指向至少 `buf_len` 个可写字节。
/// - `name` 必须是向有效且以此空字符结尾的 C 字符串的指针。
unsafe fn unsafe_greet(buf: *mut u8, buf_len: usize, name: *const u8) -> isize {
    // 待办：构建问候语，将字节复制到 buf 中，返回长度
    // 提示：使用 std::ffi::CStr::from_ptr 或手动遍历字节
    todo!()
}

/// 安全包装器 —— 公共 API 中没有不安全部分
fn safe_greet(name: &str) -> Result<String, String> {
    // 待办：分配一个 Vec<u8> 缓冲区，创建一个带空终止符的名称，
    // 在带有安全注释的 unsafe 块中调用 unsafe_greet，
    // 将结果转换回 String
    todo!()
}

fn main() {
    match safe_greet("Rustacean") {
        Ok(msg) => println!("{msg}"),
        Err(e) => eprintln!("错误：{e}"),
    }
    // 预期输出：Hello, Rustacean!
}
```

---

<details><summary>参考答案 (点击展开)</summary>

```rust
use std::ffi::CStr;

/// 模拟 C 函数：向缓冲区写入 "Hello, <name>!"。
/// 返回写入的字节数，如果缓冲区太小则返回 -1。
/// # Safety
/// - `buf` 必须指向至少 `buf_len` 个可写字节.
/// - `name` 必须是向有效且以此空字符结尾的 C 字符串的指针.
unsafe fn unsafe_greet(buf: *mut u8, buf_len: usize, name: *const u8) -> isize {
    // 安全性：调用方保证 name 是一个有效的以空字符结尾的字符串
    let name_cstr = unsafe { CStr::from_ptr(name as *const std::os::raw::c_char) };
    let name_str = match name_cstr.to_str() {
        Ok(s) => s,
        Err(_) => return -1,
    };
    let greeting = format!("Hello, {}!", name_str);
    if greeting.len() > buf_len {
        return -1;
    }
    // 安全性：调用方保证 buf 指向至少 buf_len 个可写字节
    unsafe {
        std::ptr::copy_nonoverlapping(greeting.as_ptr(), buf, greeting.len());
    }
    greeting.len() as isize
}

/// 安全包装器 —— 公共 API 中没有不安全部分
fn safe_greet(name: &str) -> Result<String, String> {
    let mut buffer = vec![0u8; 256];
    // 为 C API 创建一个带空终止符的 name 版本
    let name_with_null: Vec<u8> = name.bytes().chain(std::iter::once(0)).collect();

    // 安全性：buffer 拥有 256 个可写字节，name_with_null 已添加空终止符
    let bytes_written = unsafe {
        unsafe_greet(buffer.as_mut_ptr(), buffer.len(), name_with_null.as_ptr())
    };

    if bytes_written < 0 {
        return Err("缓冲区太小或名称无效".to_string());
    }

    String::from_utf8(buffer[..bytes_written as usize].to_vec())
        .map_err(|e| format!("无效的 UTF-8 编码：{e}"))
}

fn main() {
    match safe_greet("Rustacean") {
        Ok(msg) => println!("{msg}"),
        Err(e) => eprintln!("错误：{e}"),
    }
}
// 输出：
// Hello, Rustacean!
```

</details>

---
