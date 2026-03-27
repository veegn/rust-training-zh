# <span class="lang-en">Introduction and Motivation</span><span class="lang-zh">引言与动机</span>

> <div class="lang-en">**What you'll learn:** Why Rust matters for C# developers - the performance gap between managed and native code, how Rust eliminates null-reference exceptions and hidden control flow at compile time, and the key scenarios where Rust complements or replaces C#.</div>
> <div class="lang-zh">**你将学到：** 为什么 Rust 值得 C# 开发者学习：托管代码与原生代码之间的性能差距，Rust 如何在编译期消除 null 引用异常和隐藏控制流，以及 Rust 适合作为 C# 补充或替代的核心场景。</div>
>
> <span class="lang-en">**Difficulty:** 🟢 Beginner</span><span class="lang-zh">**难度：** 🟢 初级</span>

---

## <span class="lang-en">Speaker Intro and General Approach</span><span class="lang-zh">讲师介绍与整体方法</span>

<div class="lang-en">
*   **Principal Firmware Architect in Microsoft SCHIE team**
    *   Industry veteran with expertise in security, systems programming, and C++ systems.
*   **Started programming in Rust in 2017**
    *   Began at AWS EC2 and have been deeply involved with the language ever since.

*This course is intended to be as interactive as possible. We assume you know C# and .NET development, and examples deliberately map C# concepts to Rust equivalents.*
</div>

<div class="lang-zh">
*   **Microsoft SCHIE 团队首席固件架构师**
    *   在安全、系统编程（固件、操作系统、虚拟机监控器）以及 C++ 系统方面经验丰富。
*   **2017 年开始使用 Rust**
    *   2017 年在 AWS EC2 开始使用 Rust，此后长期深度投入。

*本课程尽量采用高互动式教学。我们假设你熟悉 C# 和 .NET 开发，示例会刻意把 C# 概念映射到 Rust 对应概念。*
</div>

---

## <span class="lang-en">Performance Without the Runtime Tax</span><span class="lang-zh">没有运行时税的性能</span>

<div class="lang-en">
C# offers great productivity but comes with runtime overhead from the Garbage Collector (GC). Rust provides the same expressiveness with zero runtime overhead and deterministic performance.
</div>

<div class="lang-zh">
C# 提供了极高的生产力，但也带来了来自垃圾回收（GC）的运行时开销。Rust 在提供同等表达能力的同时，实现了零运行时开销和确定性的性能表现。
</div>

```csharp
// C# - Great productivity, runtime overhead / 高生产力，但有运行时开销
public class DataProcessor
{
    private List<int> data = new List<int>();
    
    public void ProcessLargeDataset()
    {
        for (int i = 0; i < 10_000_000; i++)
        {
            data.Add(i * 2); // GC pressure / GC 压力
        }
        // Unpredictable GC pauses / 不可预测的 GC 停顿
    }
}
// Performance / 性能: Variable (50-200ms) | Memory / 内存: ~80MB
```

```rust
// Rust - Zero runtime overhead / 零运行时开销
struct DataProcessor {
    data: Vec<i32>,
}

impl DataProcessor {
    fn process_large_dataset(&mut self) {
        // Zero-cost abstractions / 零成本抽象
        for i in 0..10_000_000 {
            self.data.push(i * 2); // No GC pressure / 无 GC 压力
        }
        // Deterministic performance / 确定性的性能
    }
}
// Performance / 性能: Consistent (~30ms) | Memory / 内存: ~40MB
```

---

## <span class="lang-en">Memory Safety</span><span class="lang-zh">内存安全：没有运行时额外负担的保证</span>

<div class="lang-en">
Rust's type system prevents data races and null-pointer exceptions at compile time, eliminating the need for many runtime checks that C# relies on.
</div>

<div class="lang-zh">
Rust 的类型系统在编译期就能防止数据竞争和空指针异常，从而消除了 C# 所依赖的许多运行时检查。
</div>

```csharp
// C# - Runtime safety with overhead / 带开销的运行时安全
public class RuntimeCheckedOperations
{
    public string? ProcessArray(int[] array)
    {
        // Runtime bounds checking / 运行时边界检查
        if (array.Length > 0)
        {
            return array[0].ToString(); 
        }
        return null;
    }
    
    public void ProcessConcurrently()
    {
        var list = new List<int>();
        // Data races possible, requires locking / 可能存在数据竞争，需要加锁
        Parallel.For(0, 1000, i => {
            lock (list) { list.Add(i); }
        });
    }
}
```

```rust
// Rust - Compile-time safety / 编译时安全
struct SafeOperations;

impl SafeOperations {
    // Compile-time null safety / 编译时空安全
    fn process_array(array: &[i32]) -> Option<String> {
        array.first().map(|x| x.to_string())
    }
    
    fn process_concurrently() {
        use std::sync::{Arc, Mutex};
        let data = Arc::new(Mutex::new(Vec::new()));
        
        // Data races prevented at compile time / 编译时防止数据竞争
        let handles: Vec<_> = (0..1000).map(|i| {
            let data = Arc::clone(&data);
            std::thread::spawn(move || {
                data.lock().unwrap().push(i);
            })
        }).collect();
        // ...
    }
}
```

---

## <span class="lang-en">Common C# Pain Points</span><span class="lang-zh">Rust 能解决的常见 C# 痛点</span>

<div class="lang-en">
### 1. Null References: The Billion Dollar Mistake
In C#, null reference exceptions are runtime "bombs." Rust eliminates them entirely through the `Option<T>` type and exhaustive matching.

### 2. Hidden Control Flow
In C#, exceptions can be thrown from almost anywhere, and the caller often doesn't know what to expect. In Rust, all potential errors are explicit in the function signature via `Result<T, E>`.

### 3. Correctness as a Proof Engine
Rust's type system catches logic bugs that C# only catches at runtime (or not at all).
</div>

<div class="lang-zh">
### 1. 空引用：十亿美元错误
在 C# 中，空引用异常是运行时的“炸弹”。Rust 通过 `Option<T>` 类型和穷尽匹配彻底消除了这一问题。

### 2. 隐藏的异常与控制流
在 C# 中，异常可能从任何地方抛出，调用者通常不知道会发生什么。在 Rust 中，所有潜在的错误都通过 `Result<T, E>` 在函数签名中显式列出。

### 3. 正确性：类型系统即证明引擎
Rust 的类型系统能捕获 C# 只能在运行时（或根本无法）捕获的逻辑 Bug。
</div>

---

## <span class="lang-en">When to Choose Rust Over C#</span><span class="lang-zh">何时选择 Rust 还是 C#</span>

| <span class="lang-en">**Scenario**</span><span class="lang-zh">**场景**</span> | <span class="lang-en">**Recommendation**</span><span class="lang-zh">**建议**</span> | <span class="lang-en">**Reason**</span><span class="lang-zh">**原因**</span> |
| :--- | :--- | :--- |
| **Performance-Critical** | **Rust** | <span class="lang-en">Zero GC, native performance</span><span class="lang-zh">无 GC，原生性能</span> |
| **High Correctness** | **Rust** | <span class="lang-en">Type-system proofs</span><span class="lang-zh">类型系统证明</span> |
| **Rapid Prototyping** | **C#** | <span class="lang-en">Rich ecosystem, high-level abstractions</span><span class="lang-zh">丰富生态，高层抽象</span> |
| **Enterprise Integration** | **C#** | <span class="lang-en">Deep Azure/Windows support</span><span class="lang-zh">深度集成的企业支持</span> |
| **Memory Constrained** | **Rust** | <span class="lang-en">Fine-grained memory control</span><span class="lang-zh">细粒度的内存控制</span> |

> <div class="lang-en">**Key Insight:** In C#, correctness is often a matter of *discipline* (convention, review, tests). In Rust, correctness is a *property of the code* itself, enforced by the compiler.</div>
> <div class="lang-zh">**核心洞见：** 在 C# 中，正确性通常取决于*纪律*（约定、评审、测试）。而在 Rust 中，正确性是*代码本身的属性*，由编译器强制执行。</div>
