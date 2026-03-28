[English Original](../en/ch01-introduction-and-motivation.md)

# 引言与动机

> **你将学到：** 为什么 Rust 值得 C# 开发者学习：托管代码与原生代码之间的性能差距，Rust 如何在编译期消除 null 引用异常和隐藏控制流，以及 Rust 适合作为 C# 补充或替代的核心场景。
>
> **难度：** 🟢 初级

---

## 讲师介绍与整体方法

*   **Microsoft SCHIE 团队首席固件架构师**
    *   在安全、系统编程（固件、操作系统、虚拟机监控器）以及 C++ 系统方面经验丰富。
*   **2017 年开始使用 Rust**
    *   2017 年在 AWS EC2 开始使用 Rust，此后长期深度投入。

*本课程尽量采用高互动式教学。我们假设你熟悉 C# 和 .NET 开发，示例会刻意把 C# 概念映射到 Rust 对应概念。欢迎在任何时候提出澄清性问题。*

---

## Rust 对 C# 开发者的价值

### 没有运行时税的性能

C# 提供了极大的生产力，但也带来了来自垃圾回收（GC）的运行时开销。Rust 在提供同等表达能力的同时，实现了零运行时开销和确定性的性能表现。

```csharp
// C# - 高生产力，但有运行时开销
public class DataProcessor
{
    private List<int> data = new List<int>();
    
    public void ProcessLargeDataset()
    {
        for (int i = 0; i < 10_000_000; i++)
        {
            data.Add(i * 2); // GC 压力
        }
    }
}
```

```rust
// Rust - 零运行时开销
struct DataProcessor {
    data: Vec<i32>,
}

impl DataProcessor {
    fn process_large_dataset(&mut self) {
        for i in 0..10_000_000 {
            self.data.push(i * 2); // 无 GC 压力
        }
    }
}
```

### 没有运行时额外负担的内存安全

```csharp
// C# - 带开销的运行时安全
public class RuntimeCheckedOperations
{
    public string? ProcessArray(int[] array)
    {
        if (array.Length > 0)
        {
            return array[0].ToString(); 
        }
        return null;
    }
}
```

```rust
// Rust - 编译期安全，无运行时开销
struct SafeOperations;

impl SafeOperations {
    // 编译时空安全，无运行时检查
    fn process_array(array: &[i32]) -> Option<String> {
        array.first().map(|x| x.to_string())
    }
}
```

---

## Rust 能解决的常见 C# 痛点

### 1. 十亿美元错误：空引用
在 C# 中，空引用异常是运行时的“炸弹”。Rust 通过 `Option<T>` 类型和穷尽匹配彻底消除了这一问题。

### 2. 隐藏的异常与控制流
在 C# 中，异常可能从任何地方抛出。而在 Rust 中，所有潜在的错误都通过 `Result<T, E>` 在函数签名中显式列出。

### 3. 正确性：将类型系统当作证明引擎
Rust 的类型系统能在编译期捕获整类逻辑错误，而这些问题在 C# 中通常只能在运行时发现。
- **ADT vs 类变通方案**
- **默认不可变性**
- **组合优于继承**

---

## 何时选择 Rust 还是 C#

| **场景** | **建议** | **原因** |
| :--- | :--- | :--- |
| **性能关键** | **Rust** | 无 GC，原生性能 |
| **高正确性要求** | **Rust** | 类型系统证明 |
| **快速原型开发** | **C#** | 丰富生态，高层抽象 |

> **核心洞见：** 在 C# 中，正确性通常取决于*纪律*（约定、评审、测试）。而在 Rust 中，正确性是*代码本身的属性*，由编译器强制执行。
