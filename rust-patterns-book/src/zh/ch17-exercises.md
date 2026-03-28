# 17. 练习 🟢

通过以下动手挑战来巩固本书学到的各种模式。

---

### 练习 1：类型安全的状态机 ★★
使用 **类型状态（Type-State）模式** 构建一个红绿灯状态机。该灯必须遵循 `红 → 绿 → 黄 → 红` 的切换顺序，且在编译时应无法进行任何其他顺序的切换。

```rust
struct Red;
struct Green;
struct Yellow;

struct TrafficLight<S> { _s: std::marker::PhantomData<S> }

impl TrafficLight<Red> {
    fn go(self) -> TrafficLight<Green> { ... }
}
// 为 Green -> Yellow 和 Yellow -> Red 实现转换。
```

---

### 练习 2：结合 PhantomData 的计量单位 ★★
扩展 `Qty<Unit>` 模式以支持除法。如果你用 `Qty<Meters>` 除以 `Qty<Seconds>`，结果应当为 `Qty<MetersPerSecond>` 类型。

---

### 练习 3：基于通道的线程池 ★★★
构建一个线程安全的 worker pool：
1. 调度层通过通道发送 `Job` 结构体。
2. N 个工作线程并发消费任务。
3. 工作线程通过一个独立的通道将 `Result` 发回。

---

### 练习 4：自定义 serde 反序列化器 ★★★
创建一个 `HumanDuration` 结构体，使其能够从诸如 `"30s"`、`"5m"` 或 `"2h"` 的字符串反序列化为 `std::time::Duration` 类型。

---

### 练习 5：Unsafe 的安全封装 ★★★
实现 `FixedVec<T, const N: usize>`，一个具有固定容量且分配在栈上的向量。使用 `MaybeUninit<T>` 作为存储，并确保所有公共方法都是安全的。

---

### 练习 6：异步流水线 ★★★
使用 `tokio::sync::mpsc` 通道创建一个“生产者-转换器-消费者”流水线：
- **生产者**：发送数字 1..100。
- **转换器**：将各数字乘以 2。
- **消费者**：收集并打印结果。
通过使用有界通道来演示背压（back-pressure）机制。

***
