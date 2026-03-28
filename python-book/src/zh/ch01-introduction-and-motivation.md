## 讲师介绍与整体方法

- **讲师介绍**
    - Microsoft SCHIE (Silicon and Cloud Hardware Infrastructure Engineering) 团队首席固件架构师。
    - 在安全、系统编程（固件、操作系统、虚拟机监控器）、CPU 与平台架构以及 C++ 系统方面经验丰富。
    - 2017 年在 AWS EC2 开始使用 Rust，此后长期深度投入该语言。
- **课程风格**：尽量采用高互动式教学。
    - **前提**：假设你熟悉 Python 及其生态系统。
    - **示例**：会刻意把 Python 概念映射到 Rust 对应概念。
    - **互动**：欢迎随时提出任何澄清性问题。

---

## Rust 对 Python 开发者的价值

> **你将学到：** 为什么 Python 开发者开始采用 Rust、真实世界中的性能收益（Dropbox、Discord、Pydantic）、何时应选择 Rust 而不是继续使用 Python，以及这两门语言在核心设计理念上的差异。
>
> **难度：** 🟢 初级

### 性能：从分钟到毫秒

Python 在 CPU 密集型任务上出了名地慢。Rust 则在保留高级语言体验的同时，提供接近 C 的原生性能。

```python
# Python - 处理 1000 万次调用约需 2 秒
import time

def fibonacci(n: int) -> int:
    if n <= 1:
        return n
    a, b = 0, 1
    for _ in range(2, n + 1):
        a, b = b, a + b
    return b

start = time.perf_counter()
results = [fibonacci(n % 30) for n in range(10_000_000)]
print(f"耗时: {time.perf_counter() - start:.2f}s")
```

```rust
// Rust - 处理同样的 1000 万次调用约需 0.07 秒
use std::time::Instant;

fn fibonacci(n: u64) -> u64 {
    if n <= 1 { return n; }
    let (mut a, mut b) = (0, 1);
    for _ in 2..=n {
        let temp = b;
        b = a + b;
        a = temp;
    }
    b
}

fn main() {
    let start = Instant::now();
    let _results: Vec<u64> = (0..10_000_000).map(|n| fibonacci(n % 30)).collect();
    println!("耗时: {:.2?}", start.elapsed());
}
```

> **说明**：为了公平比较性能，Rust 应在发布模式下运行（`cargo run --release`）。
>
> **为什么差距这么大？** Python 的每一次加法都要经过字典查找、整数对象的解包以及类型检查。而 Rust 会将逻辑直接编译成高效的机器码，与 C 编译器生成的指令集几乎一致。

### 没有垃圾回收器的内存安全

Python 的引用计数 GC 存在循环引用、垃圾回收时机不可控以及内存碎片等问题。Rust 则在编译期通过所有权系统彻底解决了这些隐患。

```rust
// Rust - 所有权机制原生阻止了循环引用的发生
struct Node {
    value: String,
    children: Vec<Node>, // 子节点被“拥有” - 不可能出现环路
}
```

> **核心洞见**：在 Rust 中，如果你确实需要类似图结构的交叉引用，必须显式使用 `Rc<RefCell<T>>` 或索引。这种方式让代码的复杂性显性化，而不是隐藏在运行时的黑盒中。

***

## Rust 能解决的常见 Python 痛点

### 1. 运行时类型错误

Python 生产环境中最常见的 Bug 之一就是传错了类型。虽然有类型提示，但它们在运行时并无强制约束力。

```rust
// Rust - 编译器在程序运行前就会抓住这些错误
fn process_user(user_id: i64, name: &str) -> User { ... }

// 传入字符串作为 ID 或传入 None 都会导致编译失败
```

### 2. None：Python 版的“十亿美元错误”

在 Python 中，`None` 可以出现在任何地方，且常在运行时导致著名的 `AttributeError`。

```rust
// Rust - 除非显式声明 Option<T>，否则不可能出现 None
match find_user(999) {
    Some(user) => println!("找到用户: {}", user.name),
    None => println!("用户不存在"),
}
```

### 3. GIL：Python 并发能力的天花板

由于全局解释器锁（GIL）的存在，Python 线程无法真正并行执行。Rust 则支持真正的多线程并行，且无需处理序列化开销。

### 4. 部署与分发的“噩梦”

Python 部署涉及虚拟环境、版本冲突以及复杂的依赖项。Rust 则编译成单一的静态二进制文件，复制到服务器即可运行，无需预装运行时。

***

## 何时选择 Rust 还是 Python？

### 建议选择 Rust 的场景：
- **性能至上**：数据流水线、高并发服务、计算密集型任务。
- **安全性要求极高**：金融系统、协议实现、关键业务。
- **追求部署简便**：仅需分发一个二进制文件。
- **需要底层控制**：硬件交互、嵌入式开发。

### 建议保留 Python 的场景：
- **快速原型验证**：编写脚本、一次性分析工具。
- **AI/ML 开发**：利用成熟的 PyTorch/TensorFlow 生态。
- **胶水代码**：简单的 API 串接或数据转换。
- **上市时间优先**：开发效率压倒一切的初创项目。

***

## 真实世界的影响：大厂为什么选 Rust？

- **Dropbox**：存储引擎从 Python 换成 Rust 后，性能提升 10 倍，内存占用减半。
- **Discord**：为了规避 Go 开发中的 GC 停顿引起的掉帧，后端核心切换到了 Rust，获得了极其稳定的低延迟。
- **Pydantic V2**：核心改用 Rust 编写后，校验速度提升了 5 到 50 倍，而 Python API 保持不变。

***

## 总结：编程思维的转变

Python 追求**开发速度**和**代码简洁**，代价是运行时开销和潜在的类型隐忧。
Rust 追求**性能**和**绝对正确**，代价是初期较高的学习曲线和编译器的“严格把关”。

通过 PyO3，你可以将两者优点结合：在 Python 中编写业务编排逻辑，在 Rust 中处理性能瓶颈。

***
