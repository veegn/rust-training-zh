# 练习题

### 练习 1：类型安全的状态机 ★★ (~30 分钟)

使用类型状态 (Type-state) 模式构建一个红绿灯状态机。该灯必须按照 `红 → 绿 → 黄 → 红` 的顺序进行切换，且不允许出现其他顺序。

<details>
<summary>🔑 参考答案</summary>

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
        println!("🔴 红灯 — 停止");
        TrafficLight { _state: PhantomData }
    }

    fn go(self) -> TrafficLight<Green> {
        println!("🟢 绿灯 — 行驶");
        TrafficLight { _state: PhantomData }
    }
}

impl TrafficLight<Green> {
    fn caution(self) -> TrafficLight<Yellow> {
        println!("🟡 黄灯 — 注意");
        TrafficLight { _state: PhantomData }
    }
}

impl TrafficLight<Yellow> {
    fn stop(self) -> TrafficLight<Red> {
        println!("🔴 红灯 — 停止");
        TrafficLight { _state: PhantomData }
    }
}

fn main() {
    let light = TrafficLight::new(); // 红灯
    let light = light.go();          // 绿灯
    let light = light.caution();     // 黄灯
    let light = light.stop();        // 红灯

    // light.caution(); // ❌ 编译错误：红灯状态下没有 `caution` 方法
    // TrafficLight::new().stop(); // ❌ 编译错误：红灯状态下没有 `stop` 方法
}
```

**关键要点**：非法的状态切换会导致编译错误，而不是运行时 panic。

</details>

---

### 练习 2：使用 PhantomData 实现单位计量 ★★ (~30 分钟)

扩展第 4 章中的单位计量模式，以支持：
- `Meters` (米)、`Seconds` (秒)、`Kilograms` (千克)
- 相同单位的加法
- 乘法：`Meters * Meters = SquareMeters` (平方米)
- 除法：`Meters / Seconds = MetersPerSecond` (米/秒)

<details>
<summary>🔑 参考答案</summary>

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
    let area = width * height; // Qty<SquareMeters>
    println!("面积: {:.1} m²", area.value);

    let dist = Qty::<Meters>::new(100.0);
    let time = Qty::<Seconds>::new(9.58);
    let speed = dist / time;
    println!("速度: {:.2} m/s", speed.value);

    let sum = width + height; // 相同单位 ✅
    println!("总和: {:.1} m", sum.value);

    // let bad = width + time; // ❌ 编译错误：无法将“米”与“秒”相加
}
```

</details>

---

### 练习 3：基于通道的工作池 (Worker Pool) ★★★ (~45 分钟)

使用通道构建一个工作池，满足以下要求：
- 一个调度器 (Dispatcher) 通过通道发送 `Job` 结构体。
- N 个工作者 (Workers) 消耗这些任务并将结果发回。
- 使用 `std::sync::mpsc` 实现。

<details>
<summary>🔑 参考答案</summary>

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

    // 将接收器封装在 Arc<Mutex> 中以便在工作者之间共享
    let job_rx = std::sync::Arc::new(std::sync::Mutex::new(job_rx));

    // 生成工作者
    let mut handles = Vec::new();
    for worker_id in 0..num_workers {
        let job_rx = job_rx.clone();
        let result_tx = result_tx.clone();
        handles.push(thread::spawn(move || {
            loop {
                // 加锁、接收、解锁 —— 保持极短的临界区
                let job = {
                    let rx = job_rx.lock().unwrap();
                    rx.recv() // 阻塞直到接收到任务或通道关闭
                };
                match job {
                    Ok(job) => {
                        let output = format!("由工作者 {worker_id} 处理了任务 '{}'", job.data);
                        result_tx.send(JobResult {
                            job_id: job.id,
                            output,
                            worker_id,
                        }).unwrap();
                    }
                    Err(_) => break, // 通道已关闭 —— 退出
                }
            }
        }));
    }
    drop(result_tx); // 丢弃我们手中的副本，这样当所有工作者结束时，结果通道才会关闭

    // 调度任务
    let num_jobs = jobs.len();
    for job in jobs {
        job_tx.send(job).unwrap();
    }
    drop(job_tx); // 关闭任务通道 —— 工作者在排空任务后将退出

    // 收集结果
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
        println!("[工作者 {}] 任务 {}: {}", r.worker_id, r.job_id, r.output);
    }
}
```

</details>

---

### 练习 4：高阶组合子流水线 (Pipeline) ★★ (~25 分钟)

创建一个 `Pipeline` 结构体来链接一系列变换操作。它应该支持通过 `.pipe(f)` 添加变换，并通过 `.execute(input)` 运行完整的流水线。

<details>
<summary>🔑 参考答案</summary>

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

    // 数字流水线：
    let result = Pipeline::new()
        .pipe(|x: i32| x * 2)
        .pipe(|x| x + 10)
        .pipe(|x| x * x)
        .execute(5);

    println!("{result}"); // (5*2 + 10)^2 = 400
}
```

**加分项**：如果想要让流水线支持在各个阶段改变类型，则需要不同的设计 —— 每次 `.pipe()` 调用都返回一个具有不同输出类型的 `Pipeline`（这需要更进阶的泛型处理能力）。

</details>

---

### 练习 5：使用 thiserror 构建错误层级 ★★ (~30 分钟)

为一个文件处理应用设计一个错误类型层级，该应用可能会在 I/O、解析（JSON 和 CSV）以及校验阶段发生故障。使用 `thiserror` 并演示 `?` 操作符的传播。

<details>
<summary>🔑 参考答案</summary>

```rust,ignore
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("I/O 错误: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON 解析错误: {0}")]
    Json(#[from] serde_json::Error),

    #[error("第 {line} 行发生 CSV 错误: {message}")]
    Csv { line: usize, message: String },

    #[error("校验错误: {field} — {reason}")]
    Validation { field: String, reason: String },
}

fn read_file(path: &str) -> Result<String, AppError> {
    Ok(std::fs::read_to_string(path)?) // io::Error → 通过 #[from] 转换为 AppError::Io
}

fn parse_json(content: &str) -> Result<serde_json::Value, AppError> {
    Ok(serde_json::from_str(content)?) // serde_json::Error → AppError::Json
}

fn validate_name(value: &serde_json::Value) -> Result<String, AppError> {
    let name = value.get("name")
        .and_then(|v| v.as_str())
        .ok_or_else(|| AppError::Validation {
            field: "name".into(),
            reason: "必须是非空字符串".into(),
        })?;

    if name.is_empty() {
        return Err(AppError::Validation {
            field: "name".into(),
            reason: "不能为空".into(),
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
        Ok(name) => println!("名称: {name}"),
        Err(e) => eprintln!("错误: {e}"),
    }
}
```

</details>

---

### 练习 6：带有关联类型的泛型特性 ★★★ (~40 分钟)

设计一个 `Repository<T>` 特性，包含关联的 `Error` 和 `Id` 类型。为内存存储实现该特性，并演示编译时类型安全性。

<details>
<summary>🔑 参考答案</summary>

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

// 错误类型为 Infallible —— 内存操作绝不失败
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

// 针对 任何 仓库的通用函数：
fn create_and_fetch<R: Repository>(repo: &mut R, item: R::Item) -> Result<(), R::Error>
where
    R::Item: std::fmt::Debug,
    R::Id: std::fmt::Debug,
{
    let id = repo.insert(item)?;
    println!("插入成功，ID 为: {id:?}");
    let retrieved = repo.get(&id)?;
    println!("检索到: {retrieved:?}");
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

### 练习 7：环绕 Unsafe 的安全包装器 (对应第 12 章) ★★★ (~45 分钟)

编写一个 `FixedVec<T, const N: usize>` —— 一个固定容量、栈分配的向量。
要求：
- `push(&mut self, value: T) -> Result<(), T>` 当满时返回 `Err(value)`。
- `pop(&mut self) -> Option<T>` 返回并移除最后一个元素。
- `as_slice(&self) -> &[T]` 借用已初始化的元素。
- 所有公共方法必须是安全的；所有 `unsafe` 代码块必须附带 `SAFETY:` 注释。
- `Drop` 必须清理已初始化的元素。

**提示**：使用 `MaybeUninit<T>` 和 `[const { MaybeUninit::uninit() }; N]`。

<details>
<summary>🔑 参考答案</summary>

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
        // SAFETY: len < N，因此 data[len] 在范围内。
        self.data[self.len] = MaybeUninit::new(value);
        self.len += 1;
        Ok(())
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 { return None; }
        self.len -= 1;
        // SAFETY: data[len] 已初始化（递减前 len > 0）。
        Some(unsafe { self.data[self.len].assume_init_read() })
    }

    pub fn as_slice(&self) -> &[T] {
        // SAFETY: data[0..len] 均已初始化，且 MaybeUninit<T> 
        // 与 T 的内存布局相同。
        unsafe { std::slice::from_raw_parts(self.data.as_ptr() as *const T, self.len) }
    }

    pub fn len(&self) -> usize { self.len }
    pub fn is_empty(&self) -> bool { self.len == 0 }
}

impl<T, const N: usize> Drop for FixedVec<T, N> {
    fn drop(&mut self) {
        // SAFETY: data[0..len] 已初始化 —— 对每一个进行 drop。
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
    // Drop 时会清理剩余的 "hello"
}
```

</details>

---

### 练习 8：声明式宏 —— `map!` (对应第 13 章) ★ (~15 分钟)

编写一个 `map!` 宏，用于从键值对创建 `HashMap`，类似于 `vec![]`：

```rust
let m = map! {
    "host" => "localhost",
    "port" => "8080",
};
assert_eq!(m.get("host"), Some(&"localhost"));
assert_eq!(m.len(), 2);
```

要求：
- 支持尾随逗号。
- 支持空调用 `map!{}`。
- 为了最大的灵活性，应适用于任何实现了 `Into<K>` 和 `Into<V>` 的类型。

<details>
<summary>🔑 参考答案</summary>

```rust
macro_rules! map {
    // 空案例
    () => {
        std::collections::HashMap::new()
    };
    // 一个或多个 key => value 键值对（尾随逗号可选）
    ( $( $key:expr => $val:expr ),+ $(,)? ) => {{
        let mut m = std::collections::HashMap::new();
        $( m.insert($key, $val); )+
        m
    }};
}

fn main() {
    // 基础用法：
    let config = map! {
        "host" => "localhost",
        "port" => "8080",
        "timeout" => "30",
    };
    assert_eq!(config.len(), 3);
    assert_eq!(config["host"], "localhost");

    // 空 map：
    let empty: std::collections::HashMap<String, String> = map!();
    assert!(empty.is_empty());

    // 不同类型：
    let scores = map! {
        1 => 100,
        2 => 200,
    };
    assert_eq!(scores[&1], 100);
}
```

</details>

---

### 练习 9：自定义 serde 反序列化 (对应第 11 章) ★★★ (~45 分钟)

设计一个 `Duration` 包装器，使用自定义 serde 反序列化器从 `"30s"`、`"5m"`、`"2h"` 等人类可读的字符串中进行反序列化。该结构体还应该能够序列化回相同的格式。

<details>
<summary>🔑 参考答案</summary>

```rust,ignore
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
struct HumanDuration(std::time::Duration);

impl HumanDuration {
    fn from_str(s: &str) -> Result<Self, String> {
        let s = s.trim();
        if s.is_empty() { return Err("时长字符串不能为空".into()); }

        let (num_str, suffix) = s.split_at(
            s.find(|c: char| !c.is_ascii_digit()).unwrap_or(s.len())
        );
        let value: u64 = num_str.parse()
            .map_err(|_| format!("无效数字: {num_str}"))?;

        let duration = match suffix {
            "s" | "sec"  => std::time::Duration::from_secs(value),
            "m" | "min"  => std::time::Duration::from_secs(value * 60),
            "h" | "hr"   => std::time::Duration::from_secs(value * 3600),
            "ms"         => std::time::Duration::from_millis(value),
            other        => return Err(format!("未知后缀: {other}")),
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

    // 往返转换正确：
    let serialized = serde_json::to_string(&config).unwrap();
    assert!(serialized.contains("30s"));
    assert!(serialized.contains("5m"));
    println!("配置: {serialized}");
}
```

</details>

---

### 练习 10 —— 带超时的并发获取器 (对应第 16 章) ★★ (~25 分钟)

编写一个异步函数 `fetch_all`，生成三个 `tokio::spawn` 任务，每个任务都使用 `tokio::time::sleep` 模拟网络调用。使用 `tokio::try_join!` 将三者组合在一起，并将其封装在 `tokio::time::timeout(Duration::from_secs(5), ...)` 中。返回 `Result<Vec<String>, ...>`，或者在任何任务失败或截止时间到期时返回错误。

**学习目标**：`tokio::spawn`、`try_join!`、`timeout` 以及跨任务边界的错误传播。

<details>
<summary>提示</summary>

每个生成的任务都返回 `Result<String, _>`。`try_join!` 会解包这三个结果。将整个 `try_join!` 封装在 `timeout()` 中 —— `Elapsed` 错误意味着达到了截止时间。

</details>

<details>
<summary>参考答案</summary>

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
    .await??; // 第一个 ? 为 timeout，第二个 ? 为 join

    Ok(vec![a?, b?, c?]) // 解包内部的 Result
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

---

### 练习 11 —— 异步通道流水线 (Async Channel Pipeline) ★★★ (~40 分钟)

使用 `tokio::sync::mpsc` 构建一个“生产者 → 转换器 → 消费者”流水线：

1. **生产者 (Producer)**：将整数 1..=20 发送到通道 A（容量为 4）。
2. **转换器 (Transformer)**：从通道 A 读取，对每个值求平方，然后发送到通道 B。
3. **消费者 (Consumer)**：从通道 B 读取，收集到 `Vec<u64>` 中并返回。

这三个阶段都作为并发的 `tokio::spawn` 任务运行。使用有界通道 (Bounded channels) 来演示背压 (Back-pressure)。断言最终的向量等于 `[1, 4, 9, ..., 400]`。

**学习目标**：`mpsc::channel`、有界背压、带 move 闭包的 `tokio::spawn`、通过通道关闭实现的优雅停机。

<details>
<summary>参考答案</summary>

```rust,ignore
use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    let (tx_a, mut rx_a) = mpsc::channel::<u64>(4); // 有界 —— 提供背压
    let (tx_b, mut rx_b) = mpsc::channel::<u64>(4);

    // 生产者
    let producer = tokio::spawn(async move {
        for i in 1..=20u64 {
            tx_a.send(i).await.unwrap();
        }
        // tx_a 在此处被丢弃 → 通道 A 关闭
    });

    // 转换器
    let transformer = tokio::spawn(async move {
        while let Some(val) = rx_a.recv().await {
            tx_b.send(val * val).await.unwrap();
        }
        // tx_b 在此处被丢弃 → 通道 B 关闭
    });

    // 消费者
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
    println!("流水线已完成: {results:?}");
}
```

</details>

***
