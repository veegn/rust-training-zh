[English Original](../en/ch07-1-memory-safety-deep-dive.md)

# 内存安全：编译期 vs 运行期

> **你将学到什么：** Rust 的引用与 C# 的指针及 `unsafe` 上下文之间的区别、生命周期基础，以及为什么 Rust 在编译期做出的安全性证明，比 C# 在运行期依赖的检查机制（边界检查、空值保护）更强。
>
> **难度：** 中级

## 引用与指针
在 C# 中，你很少使用指针，除非你处理 `unsafe` 代码块。而在 Rust 中，引用（`&`）则是处处可见的，且它们**默认即安全**。

### C# Unsafe 指针
```csharp
unsafe {
    int value = 42;
    int* ptr = &value;
    *ptr = 100;
}
```

### Rust 安全引用
```rust
let mut value = 42;
let r = &mut value; // 不需要 'unsafe' 关键字
*r = 100;
```
**借用检查器（Borrow Checker）** 会确保 `r` 永远不会指向无效的内存。这让你在获得类似指针性能的同时，依旧享有高级语言引用的安全性。

---

## 生命周期基础
**生命周期** 是编译器用来确保左右所有借用（引用）在它们被使用时依然有效的一套机制。

### 悬垂引用问题
在 C# 中，返回一个局部变量的指针简直是灾难。而 Rust 的编译器会立即抓出这类错误。
```rust
fn invalid_reference() -> &String {
    let s = String::from("你好");
    &s // ❌ 报错：`s` 本身活得不够久
}
```
在这个例子中，`s` 在函数结束时就会被释放（Dropped），因此该引用会变成一个指向垃圾数据的“悬垂引用”。Rust 绝不允许此类代码编译通过。

---

## 运行时检查 vs 编译期证明

| **特性** | **C# (运行时)** | **Rust (编译期)** |
| :--- | :--- | :--- |
| **边界检查** | 抛出 `IndexOutOfRange` | 证明安全或 Panic |
| **空值访问** | 抛出 `NullReference` | 彻底杜绝 (无 Null) |
| **数据竞争** | 可能出现 (锁/Mutex) | **不可能** (由借用规则限制) |
| **内存泄漏** | 极少 (由 GC 处理) | 杜绝常见泄漏 (由所有权限制) |

### Use-After-Free (释放后使用)
在 C# 中，你可能会 Dispose 一个资源（如 `FileStream`），然后不小心再次尝试使用它，导致 `ObjectDisposedException`。而在 Rust 中，所有权系统会让此类错误成为编译期错误。

```rust
// C#
var stream = new FileStream(...);
stream.Dispose();
stream.Write(...); // ❌ 运行时异常

// Rust
let file = File::open(...)?;
drop(file);
// file.write(...); // ❌ 编译期报错：该值在移动后仍被使用
```

---

## 练习：找出安全问题
**挑战：** 识别为什么在迭代集合时修改它是危险的（在 C# 中），以及 Rust 如何在编译期防止这种错误。

```rust
fn filter_evens(numbers: &mut Vec<i32>) {
    for n in numbers.iter() {
        if n % 2 == 0 {
            // numbers.remove(...); // ❌ 编译报错
        }
    }
}
```
**关键理解：** 在 C# 中，在 `foreach` 循环里修改集合会触发运行时的 `InvalidOperationException`。而在 Rust 中，迭代器持有了该 Vec 的不可变借用，因此任何修改它的尝试（如 `remove`）都会导致编译失败。
