# 13. 并发编程 🔴

> **你将学到：**
> - 为什么 Rust 没有 GIL (全局解释器锁)
> - 真正的并行 vs Python 的多线程限制
> - 通过 `Arc<Mutex<T>>` 实现线程安全
> - 异步编程 (Async/Await)：Python 的 `asyncio` vs Rust 的 `tokio`

## 没有 GIL：真正的并行

Python 的全局解释器锁 (GIL) 是其在 CPU 密集型任务中的最大瓶颈。由于同一时刻只有一个线程可以执行 Python 字节码，即使使用了多线程，Python 也无法真正提速。

**Rust 没有 GIL。** 多个线程可以实现真正的并行运行。此外，Rust 的类型系统在编译阶段就能通过所有权和借用规则彻底消除数据竞争 (Data Races)。

### Python: GIL 瓶颈
```python
import threading

def cpu_bound_task():
    sum(range(10_000_000))

# 即使开启 4 个线程，受限于 GIL，其执行速度并不会比单线程快多少。
threads = [threading.Thread(target=cpu_bound_task) for _ in range(4)]
```

### Rust: 无所畏惧的并行
```rust
use std::thread;

fn cpu_bound_task() {
    (0..10_000_000).sum::<u64>();
}

fn main() {
    let handles: Vec<_> = (0..4).map(|_| {
        thread::spawn(|| cpu_bound_task())
    }).collect();

    for h in handles { h.join().unwrap(); }
    // 这将真正地在 4 个不同的 CPU 核心上同时运行！的速度提升约 4 倍。
}
```

---

## 共享状态：Arc 与 Mutex

为了在多个线程之间共享数据，你需要一种方法来**保护数据**并实现**跨线程引用计数**。

- **`Mutex<T>` (互斥锁)**：确保同一时间只有一个线程可以访问该数据。
- **`Arc<T>` (原子引用计数)**：允许数据的引用被多个线程安全地“拥有”。

```rust
use std::sync::{Arc, Mutex};
use std::thread;

let counter = Arc::new(Mutex::new(0));
let mut handles = vec![];

for _ in 0..10 {
    let counter = Arc::clone(&counter); // 原子引用计数增加
    handles.push(thread::spawn(move || {
        let mut num = counter.lock().unwrap(); // 必须先获取锁才能访问数据！
        *num += 1;
    }));
}
```

---

## Async/Await：I/O 密集型任务

虽然两种语言都支持 `async/await` 语法，但它们的底层实现逻辑大相径庭：
- **Python 的 `asyncio`**：单线程事件循环。如果在里面跑一个 CPU 耗时任务，整个循环都会被阻塞。
- **Rust 的 `tokio`**：多线程异步运行时。`tokio::spawn` 可以在任何空闲的 CPU 核心上调度任务并行执行。

### 快速概念映射清单：
| Python | Rust (使用 Tokio 运行时) |
|--------|--------------|
| `asyncio.run(main())` | `#[tokio::main] async fn main()` |
| `asyncio.gather(*tasks)` | `futures::future::join_all(tasks).await` |
| `asyncio.sleep(1)` | `tokio::time::sleep(Duration::from_secs(1)).await` |

---

## 练习

<details>
<summary><strong>🏋️ 练习：并发遍历利器 Rayon</strong> (点击展开)</summary>

**挑战**：在 Python 中，你可能会用 `multiprocessing.Pool` 来并行处理任务。而在 Rust 中，只需引入 `rayon` Crate 即可实现。请尝试使用 Rayon 的 `.par_iter()` 方法，并行地计算一个数组中每个数的平方。

<details>
<summary>参考答案</summary>

```rust
use rayon::prelude::*; // 需要先在 Cargo.toml 中引入 rayon crate

fn main() {
    let nums = vec![1, 2, 3, 4, 5, 6];
    let squares: Vec<_> = nums.par_iter().map(|&x| x * x).collect();
    println!("{:?}", squares);
}
```
</details>
</details>

***
