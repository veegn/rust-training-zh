## Exercises / 练习
### Exercise 1: Type-Safe State Machine ★★ (~30 min) / 练习 1：类型安全的状态机

Build a traffic light state machine using the type-state pattern. The light must transition `Red → Green → Yellow → Red` and no other order should be possible.

使用类型状态（type-state）模式构建一个红绿灯状态机。该灯必须遵循 `红 → 绿 → 黄 → 红` 的切换顺序，且不应允许任何其他顺序。

<details>
<summary>🔑 Solution / 参考答案</summary>

```rust
use std::marker::PhantomData;

struct Red;
struct Green;
struct Yellow;

struct TrafficLight<State> {
    _state: PhantomData<State>,
}

impl TrafficLight<Red> {
    fn new() -> Self {
        println!("🔴 Red — STOP / 红灯 —— 停止");
        TrafficLight { _state: PhantomData }
    }

    fn go(self) -> TrafficLight<Green> {
        println!("🟢 Green — GO / 绿灯 —— 行进");
        TrafficLight { _state: PhantomData }
    }
}

impl TrafficLight<Green> {
    fn caution(self) -> TrafficLight<Yellow> {
        println!("🟡 Yellow — CAUTION / 黄灯 —— 注意");
        TrafficLight { _state: PhantomData }
    }
}

impl TrafficLight<Yellow> {
    fn stop(self) -> TrafficLight<Red> {
        println!("🔴 Red — STOP / 红灯 —— 停止");
        TrafficLight { _state: PhantomData }
    }
}

fn main() {
    let light = TrafficLight::new(); // Red / 红灯
    let light = light.go();          // Green / 绿灯
    let light = light.caution();     // Yellow / 黄灯
    let light = light.stop();        // Red / 红灯

    // light.caution(); // ❌ Compile error: no method `caution` on Red
                        // ❌ 编译错误：Red 类型没有 `caution` 方法
    // TrafficLight::new().stop(); // ❌ Compile error: no method `stop` on Red
                                   // ❌ 编译错误：Red 类型没有 `stop` 方法
}
```

**Key takeaway / 关键要点**：Invalid transitions are compile errors, not runtime panics. / 无效的转换会导致编译错误，而不是运行时 panic。

</details>

---

### Exercise 2: Unit-of-Measure with PhantomData ★★ (~30 min) / 练习 2：结合 PhantomData 的计量单位

Extend the unit-of-measure pattern from Ch 4 to support:

扩展第 4 章中的计量单位模式，以支持：

- `Meters`, `Seconds`, `Kilograms` / 米、秒、千克
- Addition of same units / 相同单位的加法
- Multiplication: `Meters * Meters = SquareMeters` / 乘法：`米 * 米 = 平方米`
- Division: `Meters / Seconds = MetersPerSecond` / 除法：`米 / 秒 = 米/秒`

<details>
<summary>🔑 Solution / 参考答案</summary>

```rust
use std::marker::PhantomData;
use std::ops::{Add, Mul, Div};

#[derive(Clone, Copy)]
struct Meters;
#[derive(Clone, Copy)]
struct Seconds;
#[derive(Clone, Copy)]
struct Kilograms;
#[derive(Clone, Copy)]
struct SquareMeters;
#[derive(Clone, Copy)]
struct MetersPerSecond;

#[derive(Debug, Clone, Copy)]
struct Qty<U> {
    value: f64,
    _unit: PhantomData<U>,
}

impl<U> Qty<U> {
    fn new(v: f64) -> Self { Qty { value: v, _unit: PhantomData } }
}

impl<U> Add for Qty<U> {
    type Output = Qty<U>;
    fn add(self, rhs: Self) -> Self::Output { Qty::new(self.value + rhs.value) }
}

impl Mul<Qty<Meters>> for Qty<Meters> {
    type Output = Qty<SquareMeters>;
    fn mul(self, rhs: Qty<Meters>) -> Qty<SquareMeters> {
        Qty::new(self.value * rhs.value)
    }
}

impl Div<Qty<Seconds>> for Qty<Meters> {
    type Output = Qty<MetersPerSecond>;
    fn div(self, rhs: Qty<Seconds>) -> Qty<MetersPerSecond> {
        Qty::new(self.value / rhs.value)
    }
}

fn main() {
    let width = Qty::<Meters>::new(5.0);
    let height = Qty::<Meters>::new(3.0);
    let area = width * height; // Qty<SquareMeters> / 平方米
    println!("Area: {:.1} m²", area.value);

    let dist = Qty::<Meters>::new(100.0);
    let time = Qty::<Seconds>::new(9.58);
    let speed = dist / time; // MetersPerSecond / 米每秒
    println!("Speed: {:.2} m/s", speed.value);

    let sum = width + height; // Same unit ✅ / 相同单位 ✅
    println!("Sum: {:.1} m", sum.value);

    // let bad = width + time; // ❌ Compile error: can't add Meters + Seconds
                               // ❌ 编译错误：无法将“米”与“秒”相加
```
</details>

---

### Exercise 3: Channel-Based Worker Pool ★★★ (~45 min) / 练习 3：基于通道的线程池（Worker Pool）

Build a worker pool using channels where:

使用通道构建一个工作线程池，要求：

- A dispatcher sends `Job` structs through a channel / 调度器（Dispatcher）通过通道发送 `Job` 结构体
- N workers consume jobs and send results back / N 个工作线程（Worker）消费任务并将结果发回
- Use `crossbeam-channel` (or `std::sync::mpsc` if crossbeam is unavailable) / 使用 `crossbeam-channel`（如果不可用，请使用 `std::sync::mpsc`）

<details>
<summary>🔑 Solution / 参考答案 (Ex 3)</summary>

```rust
use std::sync::mpsc;
use std::thread;

struct Job {
    id: u64,
    data: String,
}

struct JobResult {
    job_id: u64,
    output: String,
    worker_id: usize,
}

fn worker_pool(jobs: Vec<Job>, num_workers: usize) -> Vec<JobResult> {
    let (job_tx, job_rx) = mpsc::channel::<Job>();
    let (result_tx, result_rx) = mpsc::channel::<JobResult>();

    // Wrap receiver in Arc<Mutex> for sharing among workers
    let job_rx = std::sync::Arc::new(std::sync::Mutex::new(job_rx));

    // Spawn workers
    let mut handles = Vec::new();
    for worker_id in 0..num_workers {
        let job_rx = job_rx.clone();
        let result_tx = result_tx.clone();
        handles.push(thread::spawn(move || {
            loop {
                // Lock, receive, unlock — short critical section
                // 加锁，接收，解锁 —— 极短的临界区
                let job = {
                    let rx = job_rx.lock().unwrap();
                    rx.recv() // Blocks until a job or channel closes / 阻塞直至获取任务或通道关闭
                };
                match job {
                    Ok(job) => {
                        let output = format!("processed '{}' by worker {worker_id}", job.data);
                        result_tx.send(JobResult {
                            job_id: job.id,
                            output,
                            worker_id,
                        }).unwrap();
                    }
                    Err(_) => break, // Channel closed — exit / 通道已关闭 —— 退出
                }
            }
        }));
    }
    drop(result_tx); // Drop our copy so result channel closes when workers finish
                     // 丢弃我们手中的副本，以便在所有工作线程完成时关闭结果通道

    // Dispatch jobs / 调度任务
    let num_jobs = jobs.len();
    for job in jobs {
        job_tx.send(job).unwrap();
    }
    drop(job_tx); // Close the job channel — workers will exit after draining
                  // 关闭任务通道 —— 工作线程在处理完剩余任务后将退出

    // Collect results / 收集结果
    let mut results = Vec::new();
    for result in result_rx {
        results.push(result);
    }
    assert_eq!(results.len(), num_jobs);

    for h in handles { h.join().unwrap(); }
    results
}

fn main() {
    let jobs: Vec<Job> = (0..20).map(|i| Job {
        id: i,
        data: format!("task-{i}"),
    }).collect();

    let results = worker_pool(jobs, 4);
    for r in &results {
        println!("[worker {}] job {}: {}", r.worker_id, r.job_id, r.output);
        println!("[工作线程 {}] 任务 {}: {}", r.worker_id, r.job_id, r.output);
    }
}
```

</details>

---

### Exercise 4: Higher-Order Combinator Pipeline ★★ (~25 min) / 练习 4：高阶组合器流水线

Create a `Pipeline` struct that chains transformations. It should support `.pipe(f)` to add a transformation and `.execute(input)` to run the full chain.

创建一个 `Pipeline` 结构体用于串联各种转换操作（transformations）。它应该支持 `.pipe(f)` 方法来添加转换步骤，以及 `.execute(input)` 方法来运行完整的流水线链。

<details>
<summary>🔑 Solution / 参考答案 (Ex 4)</summary>

```rust
struct Pipeline<T> {
    transforms: Vec<Box<dyn Fn(T) -> T>>,
}

impl<T: 'static> Pipeline<T> {
    fn new() -> Self {
        Pipeline { transforms: Vec::new() }
    }

    fn pipe(mut self, f: impl Fn(T) -> T + 'static) -> Self {
        self.transforms.push(Box::new(f));
        self
    }

    fn execute(self, input: T) -> T {
        self.transforms.into_iter().fold(input, |val, f| f(val))
    }
}

fn main() {
    let result = Pipeline::new()
        .pipe(|s: String| s.trim().to_string())
        .pipe(|s| s.to_uppercase())
        .pipe(|s| format!(">>> {s} <<<"))
        .execute("  hello world  ".to_string());

    println!("{result}"); // >>> HELLO WORLD <<<

    // Numeric pipeline:
    let result = Pipeline::new()
        .pipe(|x: i32| x * 2)
        .pipe(|x| x + 10)
        .pipe(|x| x * x)
        .execute(5);

    println!("{result}"); // (5*2 + 10)^2 = 400
}
```

**Bonus / 加分项**：Generic pipeline that changes type between stages would use a different design — each `.pipe()` returns a `Pipeline` with a different output type (this requires more advanced generic plumbing).

能够在各个阶段改变类型的泛型流水线需要不同的设计 —— 每一个 `.pipe()` 都返回一个具有不同输出类型的 `Pipeline`（这需要更高级的泛型技巧）。

</details>

---

### Exercise 5: Error Hierarchy with thiserror ★★ (~30 min) / 练习 5：使用 thiserror 构建错误层级

Design an error type hierarchy for a file-processing application that can fail during I/O, parsing (JSON and CSV), and validation. Use `thiserror` and demonstrate `?` propagation.

为一个文件处理应用程序设计一个错误类型层级。该程序可能会在 I/O、解析（JSON 和 CSV）以及校验阶段失败。请使用 `thiserror` 库并展示 `?` 操作符的传播。

<details>
<summary>🔑 Solution / 参考答案 (Ex 5)</summary>

```rust,ignore
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON parse error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("CSV error at line {line}: {message}")]
    Csv { line: usize, message: String },

    #[error("validation error: {field} — {reason}")]
    Validation { field: String, reason: String },
}

fn read_file(path: &str) -> Result<String, AppError> {
    Ok(std::fs::read_to_string(path)?) // io::Error → AppError::Io via #[from]
}

fn parse_json(content: &str) -> Result<serde_json::Value, AppError> {
    Ok(serde_json::from_str(content)?) // serde_json::Error → AppError::Json
}

fn validate_name(value: &serde_json::Value) -> Result<String, AppError> {
    let name = value.get("name")
        .and_then(|v| v.as_str())
        .ok_or_else(|| AppError::Validation {
            field: "name".into(),
            reason: "must be a non-null string".into(),
        })?;

    if name.is_empty() {
        return Err(AppError::Validation {
            field: "name".into(),
            reason: "must not be empty".into(),
        });
    }

    Ok(name.to_string())
}

fn process_file(path: &str) -> Result<String, AppError> {
    let content = read_file(path)?;
    let json = parse_json(&content)?;
    let name = validate_name(&json)?;
    Ok(name)
}

fn main() {
    match process_file("config.json") {
        Ok(name) => println!("Name: {name}"),
        Err(e) => eprintln!("Error: {e}"),
    }
}
```

</details>

---

### Exercise 6: Generic Trait with Associated Types ★★★ (~40 min) / 练习 6：带有关联类型的泛型 Trait

Design a `Repository<T>` trait with associated `Error` and `Id` types. Implement it for an in-memory store and demonstrate compile-time type safety.

设计一个 `Repository<T>` trait，包含关联类型 `Error` 和 `Id`。为一个内存存储（in-memory store）实现该 trait，并展示编译时类型安全性。

<details>
<summary>🔑 Solution / 参考答案 (Ex 6)</summary>

```rust
use std::collections::HashMap;

trait Repository {
    type Item;
    type Id;
    type Error;

    fn get(&self, id: &Self::Id) -> Result<Option<&Self::Item>, Self::Error>;
    fn insert(&mut self, item: Self::Item) -> Result<Self::Id, Self::Error>;
    fn delete(&mut self, id: &Self::Id) -> Result<bool, Self::Error>;
}

#[derive(Debug, Clone)]
struct User {
    name: String,
    email: String,
}

struct InMemoryUserRepo {
    data: HashMap<u64, User>,
    next_id: u64,
}

impl InMemoryUserRepo {
    fn new() -> Self {
        InMemoryUserRepo { data: HashMap::new(), next_id: 1 }
    }
}

// Error type is Infallible — in-memory ops never fail
// 错误类型为 Infallible —— 内存操作永远不会失败
impl Repository for InMemoryUserRepo {
    type Item = User;
    type Id = u64;
    type Error = std::convert::Infallible;

    fn get(&self, id: &u64) -> Result<Option<&User>, Self::Error> {
        Ok(self.data.get(id))
    }

    fn insert(&mut self, item: User) -> Result<u64, Self::Error> {
        let id = self.next_id;
        self.next_id += 1;
        self.data.insert(id, item);
        Ok(id)
    }

    fn delete(&mut self, id: &u64) -> Result<bool, Self::Error> {
        Ok(self.data.remove(id).is_some())
    }
}

// Generic function works with ANY repository:
// 泛型函数适用于任何 Repository：
fn create_and_fetch<R: Repository>(repo: &mut R, item: R::Item) -> Result<(), R::Error>
where
    R::Item: std::fmt::Debug,
    R::Id: std::fmt::Debug,
{
    let id = repo.insert(item)?;
    println!("Inserted with id: {id:?}");
    let retrieved = repo.get(&id)?;
    println!("Retrieved: {retrieved:?}");
    Ok(())
}

fn main() {
    let mut repo = InMemoryUserRepo::new();
    create_and_fetch(&mut repo, User {
        name: "Alice".into(),
        email: "alice@example.com".into(),
    }).unwrap();
}
```

</details>

---

### Exercise 7: Safe Wrapper around Unsafe (Ch 12) ★★★ (~45 min) / 练习 7：Unsafe 的安全封装（第 12 章）

Write a `FixedVec<T, const N: usize>` — a fixed-capacity, stack-allocated vector. Requirements:

编写一个 `FixedVec<T, const N: usize>` —— 一个固定容量且分配在栈上的向量。要求如下：

- `push(&mut self, value: T) -> Result<(), T>` returns `Err(value)` when full / 满时返回 `Err(value)`
- `pop(&mut self) -> Option<T>` returns and removes the last element / 返回并移除最后一个元素
- `as_slice(&self) -> &[T]` borrows initialized elements / 借用已初始化的元素
- All public methods must be safe; all unsafe must be encapsulated with `SAFETY:` comments / 所有公共方法必须安全；所有 unsafe 必须带有 `SAFETY:` 注释封装
- `Drop` must clean up initialized elements / `Drop` 必须清理已初始化的元素

**Hint / 提示**：Use `MaybeUninit<T>` and `[const { MaybeUninit::uninit() }; N]`.

<details>
<summary>🔑 Solution / 参考答案 (Ex 7)</summary>

```rust
use std::mem::MaybeUninit;

pub struct FixedVec<T, const N: usize> {
    data: [MaybeUninit<T>; N],
    len: usize,
}

impl<T, const N: usize> FixedVec<T, N> {
    pub fn new() -> Self {
        FixedVec {
            data: [const { MaybeUninit::uninit() }; N],
            len: 0,
        }
    }

    pub fn push(&mut self, value: T) -> Result<(), T> {
        if self.len >= N { return Err(value); }
        // SAFETY: len < N, so data[len] is within bounds.
        self.data[self.len] = MaybeUninit::new(value);
        self.len += 1;
        Ok(())
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 { return None; }
        self.len -= 1;
        // SAFETY: data[len] was initialized (len was > 0 before decrement).
        Some(unsafe { self.data[self.len].assume_init_read() })
    }

    pub fn as_slice(&self) -> &[T] {
        // SAFETY: data[0..len] are all initialized, and MaybeUninit<T>
        // has the same layout as T.
        unsafe { std::slice::from_raw_parts(self.data.as_ptr() as *const T, self.len) }
    }

    pub fn len(&self) -> usize { self.len }
    pub fn is_empty(&self) -> bool { self.len == 0 }
}

impl<T, const N: usize> Drop for FixedVec<T, N> {
    fn drop(&mut self) {
        // SAFETY: data[0..len] are initialized — drop each one.
        for i in 0..self.len {
            unsafe { self.data[i].assume_init_drop(); }
        }
    }
}

fn main() {
    let mut v = FixedVec::<String, 4>::new();
    v.push("hello".into()).unwrap();
    v.push("world".into()).unwrap();
    assert_eq!(v.as_slice(), &["hello", "world"]);
    assert_eq!(v.pop(), Some("world".into()));
    assert_eq!(v.len(), 1);
    // Drop cleans up remaining "hello"
}
```

</details>

---

### Exercise 8: Declarative Macro — `map!` (Ch 13) ★ (~15 min) / 练习 8：声明式宏 —— `map!`（第 13 章）

Write a `map!` macro that creates a `HashMap` from key-value pairs, similar to `vec![]`:

编写一个 `map!` 宏，用于从键值对创建 `HashMap`，类似于 `vec![]`：

```rust
let m = map! {
    "host" => "localhost",
    "port" => "8080",
};
assert_eq!(m.get("host"), Some(&"localhost"));
assert_eq!(m.len(), 2);
```

Requirements / 要求：
- Support trailing comma / 支持尾随逗号
- Support empty invocation `map!{}` / 支持空调用 `map!{}`
- Work with any types that implement `Into<K>` and `Into<V>` for maximum flexibility / 为了最大灵活性，使其支持实现了 `Into<K>` 和 `Into<V>` 的任何类型

<details>
<summary>🔑 Solution / 参考答案 (Ex 8)</summary>

```rust
macro_rules! map {
    // Empty case
    () => {
        std::collections::HashMap::new()
    };
    // One or more key => value pairs (trailing comma optional)
    ( $( $key:expr => $val:expr ),+ $(,)? ) => {{
        let mut m = std::collections::HashMap::new();
        $( m.insert($key, $val); )+
        m
    }};
}

fn main() {
    // Basic usage:
    let config = map! {
        "host" => "localhost",
        "port" => "8080",
        "timeout" => "30",
    };
    assert_eq!(config.len(), 3);
    assert_eq!(config["host"], "localhost");

    // Empty map:
    let empty: std::collections::HashMap<String, String> = map!();
    assert!(empty.is_empty());

    // Different types:
    let scores = map! {
        1 => 100,
        2 => 200,
    };
    assert_eq!(scores[&1], 100);
}
```

</details>

---

### Exercise 9: Custom serde Deserialization (Ch 11) ★★★ (~45 min) / 练习 9：自定义 serde 反序列化（第 11 章）

Design a `Duration` wrapper that deserializes from human-readable strings like `"30s"`, `"5m"`, `"2h"` using a custom serde deserializer. The struct should also serialize back to the same format.

设计一个 `Duration` 包装器，使用自定义 serde 反序列化器从人类可读的字符串（如 `"30s"`、`"5m"`、`"2h"`）中进行反序列化。该结构体还应当能序列化回相同的格式。

<details>
<summary>🔑 Solution / 参考答案 (Ex 9)</summary>

```rust,ignore
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
struct HumanDuration(std::time::Duration);

impl HumanDuration {
    fn from_str(s: &str) -> Result<Self, String> {
        let s = s.trim();
        if s.is_empty() { return Err("empty duration string".into()); }

        let (num_str, suffix) = s.split_at(
            s.find(|c: char| !c.is_ascii_digit()).unwrap_or(s.len())
        );
        let value: u64 = num_str.parse()
            .map_err(|_| format!("invalid number: {num_str}"))?;

        let duration = match suffix {
            "s" | "sec"  => std::time::Duration::from_secs(value),
            "m" | "min"  => std::time::Duration::from_secs(value * 60),
            "h" | "hr"   => std::time::Duration::from_secs(value * 3600),
            "ms"         => std::time::Duration::from_millis(value),
            other        => return Err(format!("unknown suffix: {other}")),
        };
        Ok(HumanDuration(duration))
    }
}

impl fmt::Display for HumanDuration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let secs = self.0.as_secs();
        if secs == 0 {
            write!(f, "{}ms", self.0.as_millis())
        } else if secs % 3600 == 0 {
            write!(f, "{}h", secs / 3600)
        } else if secs % 60 == 0 {
            write!(f, "{}m", secs / 60)
        } else {
            write!(f, "{}s", secs)
        }
    }
}

impl Serialize for HumanDuration {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for HumanDuration {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        HumanDuration::from_str(&s).map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct Config {
    timeout: HumanDuration,
    retry_interval: HumanDuration,
}

fn main() {
    let json = r#"{ "timeout": "30s", "retry_interval": "5m" }"#;
    let config: Config = serde_json::from_str(json).unwrap();

    assert_eq!(config.timeout.0, std::time::Duration::from_secs(30));
    assert_eq!(config.retry_interval.0, std::time::Duration::from_secs(300));

    // Round-trips correctly:
    let serialized = serde_json::to_string(&config).unwrap();
    assert!(serialized.contains("30s"));
    assert!(serialized.contains("5m"));
    println!("Config: {serialized}");
}
```

</details>

### Exercise 10 — Concurrent Fetcher with Timeout ★★ (~25 min) / 练习 10 —— 带有超时的并发获取器

Write an async function `fetch_all` that spawns three `tokio::spawn` tasks, each simulating a network call with `tokio::time::sleep`. Join all three with `tokio::try_join!` wrapped in `tokio::time::timeout(Duration::from_secs(5), ...)`. Return `Result<Vec<String>, ...>` or an error if any task fails or the deadline expires.

编写一个异步函数 `fetch_all`，生成三个 `tokio::spawn` 任务，每个任务使用 `tokio::time::sleep` 模拟网络调用。使用 `tokio::try_join!` 将三者结合，并包装在 `tokio::time::timeout(Duration::from_secs(5), ...)` 中。如果任何任务失败或截止时间到期，则返回错误，否则返回 `Result<Vec<String>, ...>`。

**Learning goals / 学习目标**：`tokio::spawn`, `try_join!`, `timeout`, error propagation across task boundaries. / `tokio::spawn`、`try_join!`、`timeout` 以及跨任务边界的错误传播。

<details>
<summary>Hint / 提示</summary>

Each spawned task returns `Result<String, _>`. `try_join!` unwraps all three. Wrap the whole `try_join!` in `timeout()` — the `Elapsed` error means you hit the deadline.

每个生成的任务都返回 `Result<String, _>`。`try_join!` 会解包这三个任务。将整个 `try_join!` 包装在 `timeout()` 中 —— `Elapsed` 错误意味着你触发了截止时间。

</details>

<details>
<summary>Solution / 参考答案 (Ex 10)</summary>

```rust,ignore
use tokio::time::{sleep, timeout, Duration};

async fn fake_fetch(name: &'static str, delay_ms: u64) -> Result<String, String> {
    sleep(Duration::from_millis(delay_ms)).await;
    Ok(format!("{name}: OK"))
}

async fn fetch_all() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let deadline = Duration::from_secs(5);

    let (a, b, c) = timeout(deadline, async {
        let h1 = tokio::spawn(fake_fetch("svc-a", 100));
        let h2 = tokio::spawn(fake_fetch("svc-b", 200));
        let h3 = tokio::spawn(fake_fetch("svc-c", 150));
        tokio::try_join!(h1, h2, h3)
    })
    .await??; // first ? = timeout, second ? = join

    Ok(vec![a?, b?, c?]) // unwrap inner Results
}

#[tokio::main]
async fn main() {
    let results = fetch_all().await.unwrap();
    for r in &results {
        println!("{r}");
    }
}
```

</details>

### Exercise 11 — Async Channel Pipeline ★★★ (~40 min) / 练习 11 —— 异步通道流水线

Build a producer → transformer → consumer pipeline using `tokio::sync::mpsc`:

使用 `tokio::sync::mpsc` 构建一个“生产者 → 转换器 → 消费者”流水线：

1. **Producer / 生产者**：sends integers 1..=20 into channel A (capacity 4). / 将整数 1..=20 发送到通道 A（容量为 4）。
2. **Transformer / 转换器**：reads from channel A, squares each value, sends into channel B. / 从通道 A 读取，计算各值的平方，然后发送到通道 B。
3. **Consumer / 消费者**：reads from channel B, collects into a `Vec<u64>`, returns it. / 从通道 B 读取，收集到 `Vec<u64>` 中并返回。

All three stages run as concurrent `tokio::spawn` tasks. Use bounded channels to demonstrate back-pressure. Assert the final vec equals `[1, 4, 9, ..., 400]`.

这三个阶段都作为并发的 `tokio::spawn` 任务运行。使用有界通道来演示背压（back-pressure）。断言最后的向量等于 `[1, 4, 9, ..., 400]`。

**Learning goals / 学习目标**：`mpsc::channel`, bounded back-pressure, `tokio::spawn` with move closures, graceful shutdown via channel close. / `mpsc::channel`、有界背压、带 move 闭包的 `tokio::spawn`、通过通道关闭实现优雅停机。

<details>
<summary>Solution / 参考答案 (Ex 11)</summary>

```rust,ignore
use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    let (tx_a, mut rx_a) = mpsc::channel::<u64>(4); // bounded — back-pressure
    let (tx_b, mut rx_b) = mpsc::channel::<u64>(4);

    // Producer
    let producer = tokio::spawn(async move {
        for i in 1..=20u64 {
            tx_a.send(i).await.unwrap();
        }
        // tx_a dropped here → channel A closes
    });

    // Transformer
    let transformer = tokio::spawn(async move {
        while let Some(val) = rx_a.recv().await {
            tx_b.send(val * val).await.unwrap();
        }
        // tx_b dropped here → channel B closes
    });

    // Consumer
    let consumer = tokio::spawn(async move {
        let mut results = Vec::new();
        while let Some(val) = rx_b.recv().await {
            results.push(val);
        }
        results
    });

    producer.await.unwrap();
    transformer.await.unwrap();
    let results = consumer.await.unwrap();

    let expected: Vec<u64> = (1..=20).map(|x: u64| x * x).collect();
    assert_eq!(results, expected);
    println!("Pipeline complete: {results:?}");
}
```

</details>

***

