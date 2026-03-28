# 6. 并发、并行与线程 🟡

> **你将学到：**
> - 并发与并行之间的精确区别
> - OS 线程、作用域线程以及用于数据并行的 rayon
> - 共享状态原语：Arc、Mutex、RwLock、原子操作、Condvar
> - 使用 OnceLock/LazyLock 进行延迟初始化

## 术语：并发 ≠ 并行

这两个术语经常被混淆。以下是它们的精确区别：

| | 并发 (Concurrency) | 并行 (Parallelism) |
|---|---|---|
| **定义** | 管理多个可以取得进展的任务 | 同时执行多个任务 |
| **硬件要求** | 单核即可 | 需要多核 |
| **类比** | 一名厨师，多道菜（在它们之间切换） | 多名厨师，每人负责一道菜 |

## std::thread — OS 线程

Rust 线程与操作系统线程是一一对应的。

```rust
use std::thread;

fn main() {
    let handle = thread::spawn(|| {
        println!("新线程序号");
        42
    });

    println!("主线程序号");
    let result = handle.join().unwrap();
}
```

### 作用域线程 (std::thread::scope)

作用域线程允许你直接借用局部数据，而不必使用 `Arc`。编译器会确保所有子线程在作用域结束前都已经完成。

```rust
let data = vec![1, 2, 3];
thread::scope(|s| {
    s.spawn(|| println!("线程 1: {:?}", data));
    s.spawn(|| println!("线程 2: {:?}", data));
}); // 保证在此处全部汇合
```

---

## rayon — 数据并行

对于计算密集型任务，请使用 `rayon`。它能够自动将工作负载分配到整个线程池中。

```rust
use rayon::prelude::*;

let sum: u64 = data.par_iter() // 并行迭代器
    .map(|x| x * x)
    .sum();
```

---

## 共享状态原语

| 原语 | 使用场景 |
|-----------|----------|
| `Mutex<T>` | 排他性访问（加锁/解锁）。 |
| `RwLock<T>` | 多个读取者 或 一个写入者。 |
| `AtomicU64` | 硬件级的无锁计数器或标志位。 |
| `OnceLock<T>` | 一次性延迟初始化（适用于全局变量）。 |

### 延迟初始化

```rust
static CONFIG: LazyLock<Config> = LazyLock::new(|| load_config());

fn main() {
    let cfg = &*CONFIG; // 在首次访问时才会触发加载
}
```

***
