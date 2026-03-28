# 14. Unsafe Rust 与 FFI 🟢

Rust 有一个“安全”的核心，但也允许你通过使用 `unsafe` 关键字来停用某些安全保证。这对于底层系统编程、与其他语言交互或构建高性能数据结构是必要的。

### 1. `unsafe` 关键字
`unsafe` 关键字允许你执行五种在安全 Rust 中不被允许的操作：
- 解引用一个裸指针。
- 调用一个 unsafe 函数或方法。
- 访问或修改一个可变静态变量。
- 实现一个 unsafe Trait。
- 访问 `union` 的字段。

```rust
fn main() {
    let mut num = 5;

    let r1 = &num as *const i32; // 裸指针
    let r2 = &mut num as *mut i32;

    unsafe {
        println!("r1 是：{}", *r1); // 解引用裸指针
        println!("r2 是：{}", *r2);
    }
}
```

---

### 2. 从 Rust 调用 C 代码 (FFI)
Rust 提供了一个外部函数接口 (Foreign Function Interface, FFI)，用于调用 C 等其他语言编写的函数。

```rust
extern "C" {
    fn abs(input: i32) -> i32;
}

fn main() {
    unsafe {
        println!("在 C 看来 -3 的绝对值是：{}", abs(-3));
    }
}
```

---

### 3. 从 C 调用 Rust 代码
你还可以通过使用 `extern` 关键字和 `#[no_mangle]` 属性导出 Rust 函数，以便让 C 代码调用。

```rust
#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("正如你所见，刚好从 C 调用了一个 Rust 函数！");
}
```

---

### 4. 创建安全包装器 (Safe Wrappers)
Rust 中的一种常见模式是将 unsafe 操作包装在一个安全函数中，从而为用户提供更安全的 API。

```rust
use std::slice;

fn main() {
    let address = 0x01234usize;
    let r = address as *const i32;

    // 从裸指针创建切片是 unsafe 的
    let values: &[i32] = unsafe { slice::from_raw_parts(r, 10000) };
}
```

---

### 对 C/C++ 开发者的总结
- **在 C/C++ 中**：按照 Rust 的标准，一切都是“unsafe”的。你负责手动管理内存并确保不发生数据竞态。
- **在 Rust 中**：你通过使用 `unsafe` 明确指定在何处绕过了安全检查。这使得审计代码中潜在的内存安全问题变得更加容易。大多数 Rust 代码应该是安全的，`unsafe` 应该谨慎使用，并包装在安全抽象中。

***
