[English Original](../en/ch14-unsafe-rust-and-ffi.md)

# Unsafe Rust 与 FFI：超越安全的边界

> **你将学到什么：** `unsafe` 允许做哪些事（原始指针、FFI、非检查转换），如何为不安全代码包一层安全封装，以及 C# 的 P/Invoke 与 Rust FFI 调用原生代码的方式。
>
> **难度：** 高级

Rust 固然以安全性闻名，但有时候，为了能够与硬件直接对话、调用 C 语言库或者构建极低层级的数据结构，你不得不暂时跳出借用检查器的规则约束。这就是 `unsafe` 的用武之地。

---

## 什么是 `unsafe`？
`unsafe` 关键字并不是关掉了借用检查器，它只是赋予了你五个额外的“超能力”：
1.  解引用一个 **原始指针 (Raw Pointer)** (`*const T`，`*mut T`)。
2.  调用一个 `unsafe` 函数。
3.  访问或修改一个 **可变的静态 (Mutable Static)** 变量。
4.  实现一个 **不安全的 Trait**。
5.  访问 `union`（联合体）中的字段。

### Rust 示例：原始指针
```rust
let mut num = 5;
let r1 = &num as *const i32;
let r2 = &mut num as *mut i32;

unsafe {
    println!("r1 指向的值是: {}", *r1);
    *r2 = 10;
}
```

---

## “安全封装”模式
Rust 社区的目标并不是要不惜一切代价地“避开” `unsafe`。相反，我们的目标是**封装**它。你只需要编写极小量的 `unsafe` 代码，然后为其包裹上一层 100% 安全的外部 API 即可。

```rust
pub fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    let ptr = values.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (
            std::slice::from_raw_parts_mut(ptr, mid),
            std::slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}
```

---

## 通过 FFI 与 C# 互操作
你可以通过 **P/Invoke** 在 C# 中调用 Rust 代码。为此，Rust 必须以 C 语言的调用约定导出函数。

### Rust 侧 (`lib.rs`)
```rust
#[no_mangle]
pub extern "C" fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

### C# 侧
```csharp
[DllImport("my_rust_lib")]
public static extern int add(int a, int b);
```

---

## C# 开发者总结表
| **概念** | **C# 对应物** | **Rust 现实** |
| :--- | :--- | :--- |
| **不安全代码** | `unsafe { ... }` | `unsafe { ... }` |
| **指针类型** | `int* ptr` | `*mut i32` |
| **P/Invoke** | `DllImport` | `extern "C"` |
| **内存固定 (Pinning)** | `fixed` 语句 | `Box::into_raw` / `Box::from_raw` |
| **内存泄漏** | GC 处理绝大部分事情 | `Drop` trait + `unsafe` 的手动清理 |

---

## 练习：编写一个不安全函数
**挑战：** 编写一个 `unsafe` 函数，利用原始指针交换两个整数的值。然后，再写一个安全的包装函数来调用它。

```rust
unsafe fn raw_swap(a: *mut i32, b: *mut i32) {
    let temp = *a;
    *a = *b;
    *b = temp;
}

fn safe_swap(a: &mut i32, b: &mut i32) {
    unsafe { raw_swap(a, b); }
}
```
**关键理解：** `unsafe` 是一种契约。编译器信任你已经验证了安全性不变量（Safety Invariants）。通过将 `unsafe` 逻辑包裹在安全函数内，你可以防止“不安全性”泄露到代码库的其他部分。
