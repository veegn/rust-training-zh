[English Original](../en/ch14-unsafe-rust-and-ffi.md)

## 不安全 Rust (Unsafe Rust)

> **你将学到：** `unsafe` 允许的操作（生指针、FFI、不检查的类型转换）；安全包装模式；用于调用本地代码的 C# P/Invoke 与 Rust FFI 对比；以及 `unsafe` 代码块的安全规范。
>
> **难度：** 🔴 高级

不安全 Rust 允许你执行借用检查器无法验证的操作。请谨慎使用，并附上清晰的文档说明。

> **高级内容扩展**：关于在不安全代码之上构建安全抽象的模式（如 Arena 分配器、无锁结构、自定义虚表），请参阅 [Rust 模式](../../rust-patterns-book/src/summary.md)。

### 何时需要 Unsafe
```rust
// 1. 解引用生指针 (Dereferencing raw pointers)
let mut value = 42;
let ptr = &mut value as *mut i32;
// 安全性说明：ptr 指向一个有效的、存活的局部变量。
unsafe {
    *ptr = 100; // 必须在 unsafe 块中进行
}

// 2. 调用不安全函数
unsafe fn dangerous() {
    // 内部实现需要调用者维护某些不变性 (Invariants)
}

// 安全性说明：此示例函数无需维护特定的不变性。
unsafe {
    dangerous(); // 调用者承担安全责任
}

// 3. 访问可变的静态变量
static mut COUNTER: u32 = 0;
// 安全性说明：处于单线程环境；没有对 COUNTER 的并发访问。
unsafe {
    COUNTER += 1; // 非线程安全 —— 调用者必须确保同步
}

// 4. 实现不安全特性 (Unsafe traits)
unsafe trait UnsafeTrait {
    fn do_something(&self);
}
```

### C# 对比：unsafe 关键字
```csharp
// C# unsafe - 概念相似，但范围不同
unsafe void UnsafeExample()
{
    int value = 42;
    int* ptr = &value;
    *ptr = 100;
    
    // C# 的 unsafe 主要涉及指针算术运算
    // Rust 的 unsafe 涉及所有权/借用规则的放宽
}

// C# fixed - 固定托管对象
unsafe void PinnedExample()
{
    byte[] buffer = new byte[100];
    fixed (byte* ptr = buffer)
    {
        // ptr 仅在此代码块内有效
    }
}
```

### 安全包装 (Safe Wrappers)
```rust
/// 核心模式：将不安全代码包装在安全的 API 中
pub struct SafeBuffer {
    data: Vec<u8>,
}

impl SafeBuffer {
    pub fn new(size: usize) -> Self {
        SafeBuffer { data: vec![0; size] }
    }
    
    /// 安全 API —— 带有边界检查的访问
    pub fn get(&self, index: usize) -> Option<u8> {
        self.data.get(index).copied()
    }
    
    /// 快速的、不检查边界的访问 —— 虽然使用了 unsafe，但通过边界检查进行了安全包装
    pub fn get_unchecked_safe(&self, index: usize) -> Option<u8> {
        if index < self.data.len() {
            // 安全性说明：我们刚刚检查过 index 处于边界内
            Some(unsafe { *self.data.get_unchecked(index) })
        } else {
            None
        }
    }
}
```

---

## 通过 FFI 与 C# 互操作

Rust 可以暴露符合 C 兼容性的函数，C# 可以通过 P/Invoke 进行调用。

```mermaid
graph LR
    subgraph "C# 进程"
        CS["C# 代码"] -->|"P/Invoke"| MI["封送处理层 (Marshal Layer)\nUTF-16 → UTF-8\n结构体布局适配"]
    end
    MI -->|"C ABI 调用"| FFI["FFI 边界"]
    subgraph "Rust cdylib (.so / .dll)"
        FFI --> RF["extern \"C\" fn\n#[no_mangle]"]
        RF --> Safe["安全 Rust\n内部逻辑"]
    end

    style FFI fill:#fff9c4,color:#000
    style MI fill:#bbdefb,color:#000
    style Safe fill:#c8e6c9,color:#000
```

### Rust 库 (编译为 cdylib)
```rust
// src/lib.rs
#[no_mangle]
pub extern "C" fn add_numbers(a: i32, b: i32) -> i32 {
    a + b
}

#[no_mangle]
pub extern "C" fn process_string(input: *const std::os::raw::c_char) -> i32 {
    // 安全性说明：input 非空（通过内部检查），且假定调用者传递的是以 null 结尾的字符串。
    let c_str = unsafe {
        if input.is_null() {
            return -1;
        }
        std::ffi::CStr::from_ptr(input)
    };
    
    match c_str.to_str() {
        Ok(s) => s.len() as i32,
        Err(_) => -1,
    }
}
```

```toml
# Cargo.toml
[lib]
crate-type = ["cdylib"]
```

### C# 调用方 (P/Invoke)
```csharp
using System.Runtime.InteropServices;

public static class RustInterop
{
    [DllImport("my_rust_lib", CallingConvention = CallingConvention.Cdecl)]
    public static extern int add_numbers(int a, int b);
    
    [DllImport("my_rust_lib", CallingConvention = CallingConvention.Cdecl)]
    public static extern int process_string(
        [MarshalAs(UnmanagedType.LPUTF8Str)] string input);
}

// 用法
int sum = RustInterop.add_numbers(5, 3);  // 8
int len = RustInterop.process_string("Hello from C#!");  // 15
```

### FFI 安全规范清单

在将 Rust 函数暴露给 C# 时，遵循这些规则可以防止最常见的 Bug：

1. **务必使用 `extern "C"`** —— 否则 Rust 会使用其自身（不稳定）的调用约定。C# P/Invoke 期望的是 C ABI。

2. **使用 `#[no_mangle]`** —— 防止 Rust 编译器混淆函数名。如果没有它，C# 将无法找到该符号。

3. **不要让 Panic 跨越 FFI 边界** —— Rust 的 Panic 回溯进入 C# 属于**未定义行为**。请在 FFI 入口处捕获 Panic：
    ```rust
    #[no_mangle]
    pub extern "C" fn safe_ffi_function() -> i32 {
        match std::panic::catch_unwind(|| {
            // 实际逻辑写在这里
            42
        }) {
            Ok(result) => result,
            Err(_) => -1,  // 返回错误代码，而不是向 C# 抛出 Panic
        }
    }
    ```

4. **不透明结构体 vs 透明结构体** —— 如果 C# 仅持有指针（不透明句柄），则不需要 `#[repr(C)]`。如果 C# 通过 `StructLayout` 读取结构体字段，则**必须**使用 `#[repr(C)]`：
    ```rust
    // 不透明 —— C# 仅持有 IntPtr。无需 #[repr(C)]。
    pub struct Connection { /* 仅限 Rust 的字段 */ }

    // 透明 —— C# 直接封送处理字段。必须使用 #[repr(C)]。
    #[repr(C)]
    pub struct Point { pub x: f64, pub y: f64 }
    ```

5. **空指针检查** —— 在解引用前务必验证指针。C# 可能会传递 `IntPtr.Zero`。

6. **字符串编码** —— C# 内部使用 UTF-16。`MarshalAs(UnmanagedType.LPUTF8Str)` 会将其转换为 UTF-8 供 Rust 的 `CStr` 使用。请在文档中明确注明此约定。

### 完整示例：带有生命周期管理的不透明句柄

这种模式在生产环境中非常常见：Rust 拥有对象所有权，C# 持有一个不透明句柄，通过显式的创建/销毁函数来管理其生命周期。

**Rust 端** (`src/lib.rs`):
```rust
use std::ffi::{c_char, CStr};

pub struct ImageProcessor {
    width: u32,
    height: u32,
    pixels: Vec<u8>,
}

/// 创建一个新的处理器。如果尺寸无效则返回 null。
#[no_mangle]
pub extern "C" fn processor_new(width: u32, height: u32) -> *mut ImageProcessor {
    if width == 0 || height == 0 {
        return std::ptr::null_mut();
    }
    let proc = ImageProcessor {
        width,
        height,
        pixels: vec![0u8; (width * height * 4) as usize],
    };
    Box::into_raw(Box::new(proc)) // 在堆上分配，返回生指针
}

/// 应用灰度滤镜。成功返回 0，空指针返回 -1。
#[no_mangle]
pub extern "C" fn processor_grayscale(ptr: *mut ImageProcessor) -> i32 {
    // 安全性说明：ptr 是由 Box::into_raw 创建的（非空），且依然有效。
    let proc = match unsafe { ptr.as_mut() } {
        Some(p) => p,
        None => return -1,
    };
    for chunk in proc.pixels.chunks_exact_mut(4) {
        let gray = (0.299 * chunk[0] as f64
                  + 0.587 * chunk[1] as f64
                  + 0.114 * chunk[2] as f64) as u8;
        chunk[0] = gray;
        chunk[1] = gray;
        chunk[2] = gray;
    }
    0
}

/// 销毁处理器。可以安全地传入 null。
#[no_mangle]
pub extern "C" fn processor_free(ptr: *mut ImageProcessor) {
    if !ptr.is_null() {
        // 安全性说明：ptr 是由 processor_new 通过 Box::into_raw 创建的
        unsafe { drop(Box::from_raw(ptr)); }
    }
}
```

**C# 端**:
```csharp
using System.Runtime.InteropServices;

public sealed class ImageProcessor : IDisposable
{
    [DllImport("image_rust", CallingConvention = CallingConvention.Cdecl)]
    private static extern IntPtr processor_new(uint width, uint height);

    [DllImport("image_rust", CallingConvention = CallingConvention.Cdecl)]
    private static extern int processor_grayscale(IntPtr ptr);

    [DllImport("image_rust", CallingConvention = CallingConvention.Cdecl)]
    private static extern void processor_free(IntPtr ptr);

    private IntPtr _handle;

    public ImageProcessor(uint width, uint height)
    {
        _handle = processor_new(width, height);
        if (_handle == IntPtr.Zero)
            throw new ArgumentException("尺寸无效");
    }

    public void Grayscale()
    {
        if (processor_grayscale(_handle) != 0)
            throw new InvalidOperationException("处理器句柄为空");
    }

    public void Dispose()
    {
        if (_handle != IntPtr.Zero)
        {
            processor_free(_handle);
            _handle = IntPtr.Zero;
        }
    }
}

// 用法 —— IDisposable 确保 Rust 内存得到释放
using var proc = new ImageProcessor(1920, 1080);
proc.Grayscale();
// proc.Dispose() 会被自动调用 → processor_free() → Rust 侧销毁 Vec
```

> **关键洞察**：这是 C# 中 `SafeHandle` 模式在 Rust 侧的等效实现。Rust 的 `Box::into_raw` / `Box::from_raw` 跨越 FFI 边界转移所有权，C# 的 `IDisposable` 包装器确保执行清理工作。

---

## 练习

<details>
<summary><strong>🏋️ 练习：为生指针编写安全包装</strong> (点击展开)</summary>

你从某个 C 库收到了一个生指针。请为其编写一个安全的 Rust 包装器：

```rust
// 模拟 C API
extern "C" {
    fn lib_create_buffer(size: usize) -> *mut u8;
    fn lib_free_buffer(ptr: *mut u8);
}
```

要求：
1. 创建一个包装生指针的 `SafeBuffer` 结构体。
2. 实现 `Drop` 特性以调用 `lib_free_buffer`。
3. 通过 `as_slice()` 提供一个安全的 `&[u8]` 视图。
4. 确保当指针为空时 `SafeBuffer::new()` 返回 `None`。

<details>
<summary>🔑 参考答案</summary>

```rust,ignore
struct SafeBuffer {
    ptr: *mut u8,
    len: usize,
}

impl SafeBuffer {
    fn new(size: usize) -> Option<Self> {
        // 安全性说明：lib_create_buffer 返回一个有效的指针或 null（在下方检查）。
        let ptr = unsafe { lib_create_buffer(size) };
        if ptr.is_null() {
            None
        } else {
            Some(SafeBuffer { ptr, len: size })
        }
    }

    fn as_slice(&self) -> &[u8] {
        // 安全性说明：ptr 非空（在 new() 中已检查），len 为已分配的大小，
        // 且我们拥有独占所有权。
        unsafe { std::slice::from_raw_parts(self.ptr, self.len) }
    }
}

impl Drop for SafeBuffer {
    fn drop(&mut self) {
        // 安全性说明：ptr 是由 lib_create_buffer 分配的
        unsafe { lib_free_buffer(self.ptr); }
    }
}

// 用法：所有的 unsafe 逻辑都包含在 SafeBuffer 内部
fn process(buf: &SafeBuffer) {
    let data = buf.as_slice(); // 完全安全的 API
    println!("第一个字节的数值：{}", data[0]);
}
```

**关键模式**：将 `unsafe` 逻辑封装在一个带有 `// SAFETY:` 注释的小模块中。对外暴露 100% 安全的公有 API。Rust 标准库就是这样工作的 —— `Vec`, `String`, `HashMap` 内部都包含 unsafe，但展现给用户的是安全的接口。

</details>
</details>
