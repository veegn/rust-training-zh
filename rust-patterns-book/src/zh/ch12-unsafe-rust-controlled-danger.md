[English Original](../en/ch12-unsafe-rust-controlled-danger.md)

# 12. Unsafe Rust：受控的危险 🔶

> **你将学到：**
> - 五种 Unsafe “超能力”及其适用场景。
> - 编写可靠的抽象：安全 API 与 Unsafe 内部实现。
> - FFI 模式：在 Rust 中调用 C。
> - 自定义分配器：Arena 与 Slab 模式。

## 五种 Unsafe 超能力

`unsafe` 允许你执行以下五项操作：
1. 解引用裸指针（`*const T`，`*mut T`）。
2. 调用 `unsafe` 函数或方法。
3. 访问或修改可变静态变量。
4. 实现 `unsafe` trait。
5. 访问 `union` 的字段。

> **核心规则**：`unsafe` 并没有关闭借用检查器。它仅仅开启了这五种特定的操作能力。

---

## 编写可靠的抽象

`unsafe` 的最终目标是将危险的低级操作封装在 **安全 API** 之中。

```rust
pub struct MyBuffer<T, const N: usize> {
    data: [MaybeUninit<T>; N],
    len: usize,
}

impl<T, const N: usize> MyBuffer<T, N> {
    pub fn get(&self, index: usize) -> Option<&T> {
        if index < self.len {
            // SAFETY: 索引已检查，确保在已初始化的范围内。
            Some(unsafe { self.data[index].assume_init_ref() })
        } else {
            None
        }
    }
}
```

---

## FFI：调用 C 代码

使用 `extern "C"` 和 `#[repr(C)]` 与 C 语言库进行交互。

```rust
extern "C" {
    fn strlen(s: *const c_char) -> usize;
}

pub fn safe_strlen(s: &str) -> usize {
    let c_str = CString::new(s).unwrap();
    // SAFETY: c_str 是一个以 null 结尾的有效 C 字符串。
    unsafe { strlen(c_str.as_ptr()) }
}
```

---

## 自定义分配器

- **Arena (Bump) 分配器**：通过向前移动指针来分配内存。分配极快（约 2ns），且在 arena 离开作用域时一次性释放所有内存。非常适合处理请求作用域的数据。
- **Slab 分配器**：预分配固定大小的插槽池。支持 O(1) 复杂度的分配和释放，且能有效防止内存碎片。

| 模式 | C 语言对应项 | Rust 工具 |
|---------|--------------|-----------|
| Arena | `obstack` | `bumpalo` |
| Slab | `kmem_cache` | `slab` |
| 局部池 | `alloca` | `FixedVec` (自定义) |

***
