## C++ → Rust Semantic Deep Dives / C++ → Rust 语义深入对比

> **What you'll learn / 你将学到：** Detailed mappings for C++ concepts that don't have obvious Rust equivalents — the four named casts, SFINAE vs trait bounds, CRTP vs associated types, and other common friction points during translation.
>
> 针对没有明显 Rust 等效概念的 C++ 特性进行详细映射 —— 包括四种命名转换（Casts）、SFINAE 与 Trait 约束的对比、CRTP 与关联类型的对比，以及翻译过程中的其他常见摩擦点。

*The sections below map C++ concepts that don't have an obvious 1:1 Rust equivalent. These differences frequently trip up C++ programmers during translation work.*

以下各节映射了那些没有明显 1:1 Rust 对应物的 C++ 概念。这些差异经常会让正在进行翻译工作的 C++ 程序员感到困惑。

---

### Casting Hierarchy / 类型转换层次：四种 C++ Cast → Rust 等效方案

*C++ has four named casts. Rust replaces them with different, more explicit mechanisms:*

C++ 有四种命名的类型转换。Rust 用不同且更显式的机制替代了它们：

| **C++ Cast** | **Rust Equivalent / Rust 等效** | **Safety / 安全性** | **Notes / 说明** |
|----------|----------------|--------|-------|
| `static_cast` (numeric) | `as` keyword | Safe / 但可能截断/回绕 | `3.14 as i32` -> 3 |
| `Checked static_cast` | `From` / `Into` | Safe / 编译时验证 | `42_u8.into()` -> i32 (无损增宽) |
| `Fallible static_cast` | `TryFrom` / `TryInto` | Safe / 返回 `Result` | `300_u16.try_into()?` -> Err |
| `dynamic_cast` | `match` / `Any` | Safe / 安全 | 枚举用模式匹配；Trait 对象用 `Any` |
| `const_cast` | No equivalent / 无直接对应 | — | 使用 `Cell`/`RefCell` 实现内部可变性 |
| `reinterpret_cast` | `transmute` | **`unsafe` / 不安全** | 重解释位模式。优先使用 `to_le_bytes` 等 |

```rust
// Rust equivalent examples:
let widened: u32 = 42_u8.into();             // Infallible / 始终以此为首选
let truncated = 300_u16 as u8;                // ⚠ Wraps / 回绕至 44！静默数据丢失
let checked: Result<u8, _> = 300_u16.try_into(); // Err / 安全的易错转换

// "const_cast" -> interior mutability (safe)
use std::cell::Cell;
struct Sensor { read_count: Cell<u32> }
impl Sensor {
    fn read(&self) -> f64 {
        self.read_count.set(self.read_count.get() + 1); // Mutate through &self
        42.0
    }
}
```

---

### Preprocessor → `cfg`, Feature Flags / 预处理器 → `cfg`、特性标志与宏

*C++ relies heavily on the preprocessor. Rust replaces these with first-class language features.*

C++ 严重依赖预处理器。Rust 则用语言的一等公民特性替代了它们。

| **C++ Preprocessor** | **Rust Equivalent / Rust 等效** | **Advantage / 优势** |
|-----------------|----------------|-----------|
| `#define PI 3.14` | `const PI: f64 = 3.14;` | 有类型、有作用域、调试器可见 |
| `#define MAX(a,b)` | `macro_rules!` or `fn max<T: Ord>` | 无重复求值漏洞 |
| `#ifdef DEBUG` | `#[cfg(debug_assertions)]` | 编译器检查，无拼写错误风险 |
| `#ifdef FEATURE_X` | `#[cfg(feature = "x")]` | Cargo 管理特性，支持依赖感知 |
| `#include "foo.h"` | `mod` + `use` | 无需头文件卫士，无循环导出 |

---

### Header Files → Modules and `use` / 头文件与 `#include` → 模块与 `use`

*In Rust, there are **no header files, no forward declarations, no include guards**:*

在 Rust 中，**没有头文件、没有前向声明、也没有头文件卫士**：

| **C++** | **Rust** | **Why it's better / 为什么更好** |
|-----|------|-----------------|
| `#include "foo.h"` | `mod foo;` + `use foo;` | 无文本包含，无 ODR 违反 |
| `#pragma once` | Not needed | 每个 `.rs` 文件即模块，仅编译一次 |
| Forward declarations | Not needed | 编译器可见整个 crate；顺序无关紧要 |
| `.h` + `.cpp` split | Single `.rs` file | 无声明/定义不一致的漏洞 |

---

### `friend` and Access Control / `friend` 与访问控制 → 模块可见性

*C++ uses `friend`. Rust has no `friend` keyword — instead, **privacy is module-scoped**:*

C++ 使用 `friend`。Rust 没有 `friend` 关键字 —— 取而代之的是，**私有属性是以模块为作用域的**：

```rust
mod vehicle {
    pub struct Engine { rpm: u32 } // Private to module (not to struct!)
    pub struct Car { engine: Engine }
    impl Car {
        pub fn accelerate(&mut self) {
            self.engine.rpm = 3000; // ✅ Same module — direct field access
        }
    }
}
```
*Key insight: C++ privacy is per-class. Rust privacy is per-module. Put types in the same module if they need "friend" access.*

---

### `volatile` and Atomics / `volatile` → 原子操作与显式 Volatile

| **C++ Usage / C++ 用法** | **Rust Equivalent / Rust 等效** | **Notes / 说明** |
|-----------|----------------|-------|
| `volatile` (hardware) | `read_volatile` / `write_volatile` | 需要 `unsafe` —— 适用于 MMIO |
| `volatile` (thread) | `AtomicBool` / `AtomicU32` etc. | C++ volatile 用于此场景也是错的！ |
| `std::atomic<T>` | `std::sync::atomic::AtomicT` | 语义相同，内存次序一致 |

---

### Container Mapping / 容器映射：C++ STL → Rust `std::collections`

| **C++ STL Container** | **Rust Equivalent / Rust 等效** | **Notes / 说明** |
|------------------|----------------|-------|
| `std::vector<T>` | `Vec<T>` | 几乎一致的 API。Rust 默认检查边界 |
| `std::array<T, N>` | `[T; N]` | 栈分配的固定大小数组 |
| `std::unordered_map`| `HashMap<K, V>` | 默认使用 `SipHash`（抗 DoS） |
| `std::map<K, V>` | `BTreeMap<K, V>` | B 树；键有序；要求 `K: Ord` |
| `std::string` | `String` | 保证 UTF-8，不以 null 结尾 |
| `std::string_view` | `&str` | 借用的 UTF-8 切片 |
| `std::span<T>` | `&[T]` / `&mut [T]` | Rust 切片是语言一等公民 |

---

### Exception Safety → Panic Safety / 异常安全性 → Panic 安全性

| **C++ Level / C++ 级别** | **Meaning / 含义** | **Rust Equivalent / Rust 等效** |
|----------|---------|----------------|
| **No-throw** | 绝不抛出异常 | 绝不发生 panic (返回 `Result`) |
| **Strong** | 提交或回滚 | 所有权模型使其变得自然 —— `?` 提前返回会自动 Drop |
| **Basic** | 保持逻辑一致 | Rust 默认行为 —— `Drop` 执行，无泄漏 |

*In Rust, `panic!` is for bugs (like `assert!` failures), while `Result<T, E>` is for routine errors. The ownership system handles cleanup automatically.*

---

### Quick Reference: C++ → Rust Idiom Map / 快速参考：C++ → Rust 惯用法映射

| **C++ Pattern / C++ 模式** | **Rust Idiom / Rust 惯用法** | **Notes / 说明** |
|----------------|---------------|----------|
| Polymorphism (Inh) | `enum Variant { ... }` | 封闭集合首选枚举 |
| Factory Pattern | `enum` + `match` | 穷尽性检查，无运行时失败 |
| `unique_ptr` | `Box<T>` | Owned values are the default |
| `shared_ptr<T>` | `Rc<T>` / `Arc<T>` | Explicit shared ownership |
| `optional<T>` | `Option<T>` | Forced matching, safer defaults |
| `const string&` | `&str` | Accepts both `String` and `&str` |
| `std::move` | `let x = obj;` | Move is the default in Rust |
| CMake / Build | `cargo` | One tool for everything |
