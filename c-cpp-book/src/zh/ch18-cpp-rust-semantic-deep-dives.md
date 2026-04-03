[English Original](../en/ch18-cpp-rust-semantic-deep-dives.md)

## C++ → Rust 语义深潜

> **你将学到：** 针对那些没有明显 Rust 等效项的 C++ 概念，提供详细的映射指南 —— 包括四种命名强制转换 (Named Casts)、SFINAE 与 Trait 限定、CRTP 与关联类型 (Associated Types)，以及翻译过程中的其他常见摩擦点。

下文各节映射了那些在 Rust 中没有明显 1:1 等效项的 C++ 概念。在翻译工作中，这些差异经常会让 C++ 程序员感到困惑。

### 类型转换层次结构：四种 C++ Cast → Rust 等效项

C++ 拥有四种命名的强制转换 (Named Casts)。Rust 则使用不同且更为明确的机制来替代它们：

```cpp
// C++ 类型转换层次结构
int i = static_cast<int>(3.14);            // 1. 数值转换 / 向上转换 (Up-cast)
Derived* d = dynamic_cast<Derived*>(base); // 2. 运行时向下转换 (Downcasting)
int* p = const_cast<int*>(cp);              // 3. 强制去掉 const 属性
auto* raw = reinterpret_cast<char*>(&obj); // 4. 位级别的重新解释
```

| C++ 类型转换 | Rust 等效项 | 安全性 | 说明 |
|----------|----------------|--------|-------|
| `static_cast` (数值) | `as` 关键字 | 安全，但可能发生截断/绕回 | `let i = 3.14_f64 as i32;` —— 截断为 3 |
| `static_cast` (数值, 已检查) | `From`/`Into` | 安全，编译期验证 | `let i: i32 = 42_u8.into();` —— 仅限拓宽转换 |
| `static_cast` (数值, 易错) | `TryFrom`/`TryInto` | 安全，返回 `Result` | `let i: u8 = 300_u16.try_into()?;` —— 返回 Err |
| `dynamic_cast` (向下转换) | 枚举上的 `match` / `Any::downcast_ref` | 安全 | 枚举使用模式匹配；Trait 对象使用 `Any` |
| `const_cast` | 无等效项 | | Rust 无法在安全代码中将 `&` 转换为 `&mut`。使用 `Cell`/`RefCell` 实现内部可变性 |
| `reinterpret_cast` | `std::mem::transmute` | **`unsafe`** | 重新解释位模式。几乎总是错误的方案 —— 优先使用 `from_le_bytes()` 等 |

---

```rust
// Rust 等效写法示例：

// 1. 数值转换 —— 相比于 `as`，优先选择 From/Into
let widened: u32 = 42_u8.into();             // 绝不会失败的拓宽转换 —— 应当优先使用
let truncated = 300_u16 as u8;                // ⚠ 绕回到 44！导致静默数据丢失
let checked: Result<u8, _> = 300_u16.try_into(); // Err —— 安全的可失败转换

// 2. 向下转换 (Downcast)：枚举 (首选) 或 Any (需要遮蔽类型时)
use std::any::Any;

fn handle_any(val: &dyn Any) {
    if let Some(s) = val.downcast_ref::<String>() {
        println!("获取到字符串: {s}");
    } else if let Some(n) = val.downcast_ref::<i32>() {
        println!("获取到整数: {n}");
    }
}

// 3. "const_cast" → 内部可变性 (无需 unsafe)
use std::cell::Cell;
struct Sensor {
    read_count: Cell<u32>,  // 即使在 &self 中也可进行修改
}
impl Sensor {
    fn read(&self) -> f64 {
        self.read_count.set(self.read_count.get() + 1); // 接收的是 &self，而非 &mut self
        42.0
    }
}

// 4. reinterpret_cast → transmute (几乎从不需要使用)
// 请优先选择安全的替代方案：
let bytes: [u8; 4] = 0x12345678_u32.to_ne_bytes();  // ✅ 安全
let val = u32::from_ne_bytes(bytes);                   // ✅ 安全
// unsafe { std::mem::transmute::<u32, [u8; 4]>(val) } // ❌ 应当避免
```

> **准则**：在地道的 Rust 中，`as` 应当极少出现（拓宽转换应使用 `From`/`Into`，收缩转换应使用 `TryFrom`/`TryInto`），`transmute` 应当是非常例外的需求，而 `const_cast` 没有等效项，因为内部可变性类型已经使其变得没有必要。

---

### 预处理器 → `cfg`、特性标志 (Feature Flags) 和 `macro_rules!`

C++ 严重依赖预处理器进行条件编译、常量定义和代码生成。Rust 将所有这些功能替换为了一等公民的语言特性。

#### `#define` 常量 → `const` 或 `const fn`

```cpp
// C++
#define MAX_RETRIES 5
#define BUFFER_SIZE (1024 * 64)
#define SQUARE(x) ((x) * (x))  // 宏 —— 属于文本替换，没有类型安全性
```

```rust
// Rust —— 类型安全、有作用域、并非文本替换
const MAX_RETRIES: u32 = 5;
const BUFFER_SIZE: usize = 1024 * 64;
const fn square(x: u32) -> u32 { x * x }  // 在编译期进行求值

// 可在常量上下文中使用：
const AREA: u32 = square(12);  // 编译期计算
static BUFFER: [u8; BUFFER_SIZE] = [0; BUFFER_SIZE];
```

#### `#ifdef` / `#if` → `#[cfg()]` 和 `cfg!()`

```cpp
// C++
#ifdef DEBUG
    log_verbose("步骤 1 已完成");
#endif

#if defined(LINUX) && !defined(ARM)
    use_x86_path();
#else
    use_generic_path();
#endif
```

```rust
// Rust —— 基于属性的条件编译
#[cfg(debug_assertions)]
fn log_verbose(msg: &str) { eprintln!("[详细日志] {msg}"); }

#[cfg(not(debug_assertions))]
fn log_verbose(_msg: &str) { /* 在 release 模式下会被优化掉 */ }

// 组合条件：
#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
fn use_x86_path() { /* ... */ }

#[cfg(not(all(target_os = "linux", target_arch = "x86_64")))]
fn use_generic_path() { /* ... */ }

// 运行时检查（条件依然是编译期确定的，但可以像普通表达式一样使用）：
if cfg!(target_os = "windows") {
    println!("正在 Windows 上运行");
}
```

---

#### `Cargo.toml` 中的特性标志 (Feature Flags)

```toml
# Cargo.toml —— 用于替代 #ifdef FEATURE_FOO
[features]
default = ["json"]
json = ["dep:serde_json"]       # 可选依赖
verbose-logging = []            # 不带额外依赖的标志
gpu-support = ["dep:cuda-sys"]  # 可选的 GPU 支持
```

```rust
// 根据特性标志编写条件代码：
#[cfg(feature = "json")]
pub fn parse_config(data: &str) -> Result<Config, Error> {
    serde_json::from_str(data).map_err(Error::from)
}

#[cfg(feature = "verbose-logging")]
macro_rules! verbose {
    ($($arg:tt)*) => { eprintln!("[详细日志] {}", format!($($arg)*)); }
}
#[cfg(not(feature = "verbose-logging"))]
macro_rules! verbose {
    ($($arg:tt)*) => { }; // 编译为空
}
```

---

#### `#define MACRO(x)` → `macro_rules!`

```cpp
// C++ —— 属于文本替换，众所周知非常容易出错
#define DIAG_CHECK(cond, msg) \
    do { if (!(cond)) { log_error(msg); return false; } } while(0)
```

```rust
// Rust —— 遵循卫生性 (Hygienic)、经过类型检查、操作语法树
macro_rules! diag_check {
    ($cond:expr, $msg:expr) => {
        if !($cond) {
            log_error($msg);
            return Err(DiagError::CheckFailed($msg.to_string()));
        }
    };
}

fn run_test() -> Result<(), DiagError> {
    diag_check!(temperature < 85.0, "GPU 温度过高");
    diag_check!(voltage > 0.8, "导轨电压过低");
    Ok(())
}
```

| C++ 预处理器 | Rust 等效方案 | 优势 |
|-----------------|----------------|-----------|
| `#define PI 3.14` | `const PI: f64 = 3.14;` | 强类型、有作用域、对调试器可见 |
| `#define MAX(a,b) ((a)>(b)?(a):(b))` | `macro_rules!` 或泛型 `fn max<T: Ord>` | 不存在多次求值引发的 Bug |
| `#ifdef DEBUG` | `#[cfg(debug_assertions)]` | 由编译器检查，无拼写错误风险 |
| `#ifdef FEATURE_X` | `#[cfg(feature = "x")]` | 由 Cargo 管理特性；支持依赖关系感知 |
| `#include "header.h"` | `mod module;` + `use module::Item;` | 没有包含守卫 (Include Guards)，没有循环引用 |
| `#pragma once` | 不需要 | 每一个 `.rs` 文件都是一个模块 —— 仅被包含一次 |

---

### 头文件与 `#include` → 模块与 `use`

在 C++ 中，编译模型围绕着文本包含 (Textual Inclusion) 展开：

```cpp
// widget.h —— 每一个使用 Widget 的翻译单元都需要包含此文件
#pragma once
#include <string>
#include <vector>

class Widget {
public:
    Widget(std::string name);
    void activate();
private:
    std::string name_;
    std::vector<int> data_;
};
```

```cpp
// widget.cpp —— 独立的定义部分
#include "widget.h"
Widget::Widget(std::string name) : name_(std::move(name)) {}
void Widget::activate() { /* ... */ }
```

而在 Rust 中，**不存在头文件，没有前置声明 (Forward Declarations)，也没有包含守卫**：

```rust
// src/widget.rs —— 声明与定义均在同一个文件中
pub struct Widget {
    name: String,         // 默认是私有的
    data: Vec<i32>,
}

impl Widget {
    pub fn new(name: String) -> Self {
        Widget { name, data: Vec::new() }
    }
    pub fn activate(&self) { /* ... */ }
}
```

```rust
// src/main.rs —— 通过模块路径导入
mod widget;  // 告诉编译器需要包含 src/widget.rs
use widget::Widget;

fn main() {
    let w = Widget::new("传感器".to_string());
    w.activate();
}
```

---

| C++ | Rust | 为什么 Rust 更好 |
|-----|------|-----------------|
| `#include "foo.h"` | 父级模块中的 `mod foo;` + `use foo::Item;` | 没有文本包含，没有违反 ODR (单一定义原则) 的风险 |
| `#pragma once` / 包含守卫 | 不需要 | 每一个 `.rs` 文件都是一个模块 —— 仅会被编译一次 |
| 前置声明 (Forward declarations) | 不需要 | 编译器可以看见整个 Crate；其定义顺序并不重要 |
| `class Foo;` (不完整类型) | 不需要 | 不存在声明与定义分离的情况 |
| 每个类对应 `.h` + `.cpp` | 单个 `.rs` 文件 | 没有因声明与定义不匹配而导致的 Bug |
| `using namespace std;` | `use std::collections::HashMap;` | 始终保持明确性 —— 不会造成全局命名空间污染 |
| 嵌套的 `namespace a::b` | 嵌套的 `mod a { mod b { } }` 或 `a/b.rs` | 文件系统与模块树镜像对应 |

---

### `friend` 与 访问控制 → 模块可见性

C++ 使用 `friend` 来授予特定的类或函数访问私有成员的权限。Rust 中没有 `friend` 关键字 —— 相反，**私有属性是以模块为作用域 of 的**：

```cpp
// C++
class Engine {
    friend class Car;   // Car 可以访问私有成员
    int rpm_;
    void set_rpm(int r) { rpm_ = r; }
public:
    int rpm() const { return rpm_; }
};
```

```rust
// Rust —— 位于同一模块中的项可以访问所有字段，无需 `friend`
mod vehicle {
    pub struct Engine {
        rpm: u32,  // 对该模块内部可见（而非仅对该结构体可见！）
    }

    impl Engine {
        pub fn new() -> Self { Engine { rpm: 0 } }
        pub fn rpm(&self) -> u32 { self.rpm }
    }

    pub struct Car {
        engine: Engine,
    }

    impl Car {
        pub fn new() -> Self { Car { engine: Engine::new() } }
        pub fn accelerate(&mut self) {
            self.engine.rpm = 3000; // ✅ 位于同一模块 —— 可直接访问字段
        }
        pub fn rpm(&self) -> u32 {
            self.engine.rpm  // ✅ 位于同一模块 —— 可读取私有字段
        }
    }
}

fn main() {
    let mut car = vehicle::Car::new();
    car.accelerate();
    // car.engine.rpm = 9000;  // ❌ 编译错误：`engine` 字段是私有的
    println!("RPM: {}", car.rpm()); // ✅ 调用 Car 的公开方法
}
```

---

| C++ 访问级别 | Rust 等效方案 | 作用域 |
|-----------|----------------|-------|
| `private` | (默认，无关键字) | 仅限在同一模块内部访问 |
| `protected` | 没有直接等效项 | 使用 `pub(super)` 供父级模块访问 |
| `public` | `pub` | 在所有位置均可访问 |
| `friend class Foo` | 将 `Foo` 放在同一个模块中 | 模块级私有权取代了 `friend` |
| — | `pub(crate)` | 在整个 Crate 内部可见，但对外部依赖项不可见 |
| — | `pub(super)` | 仅对父级模块可见 |
| — | `pub(in crate::path)` | 在特定的子模块树内部可见 |

> **核心洞察**：C++ 的私有权是基于类 (Class) 的。Rust 的私有权是基于模块 (Module) 的。这意味着你可以通过选择哪些类型属于同一个模块来控制访问权限 —— 放在一起的类型具有访问彼此私有字段的完整权限。

---

### `volatile` → 原子 (Atomics) 以及 `read_volatile`/`write_volatile`

在 C++ 中，`volatile` 告诉编译器不要将读/写操作优化掉 —— 这通常用于内存映射 (memory-mapped) 的硬件寄存器。**Rust 中没有 `volatile` 关键字。**

```cpp
// C++: 用于硬件寄存器的 volatile
volatile uint32_t* const GPIO_REG = reinterpret_cast<volatile uint32_t*>(0x4002'0000);
*GPIO_REG = 0x01;              // 此写入操作不会被优化掉
uint32_t val = *GPIO_REG;     // 此读取操作不会被优化掉
```

```rust
// Rust: 明确的 volatile 操作 —— 仅限在 unsafe 代码中使用
use std::ptr;

const GPIO_REG: *mut u32 = 0x4002_0000 as *mut u32;

// 安全性：GPIO_REG 是一个有效的内存映射 I/O 地址
unsafe {
    ptr::write_volatile(GPIO_REG, 0x01);   // 写入操作不会被优化掉
    let val = ptr::read_volatile(GPIO_REG); // 读取操作不会被优化掉
}
```

针对**并发共享状态**（这是 `volatile` 在 C++ 中的另一个常见用法），Rust 使用原子：

```cpp
// C++: volatile 对于线程安全来说是不足够的（这是常见的错误！）
volatile bool stop_flag = false;  // ❌ 存在数据竞争 —— 在 C++11 之后属于未定义行为 (UB)

// 正确的 C++ 写法：
std::atomic<bool> stop_flag{false};
```

```rust
// Rust: 原子是跨线程共享可变状态的唯一方案
use std::sync::atomic::{AtomicBool, Ordering};

static STOP_FLAG: AtomicBool = AtomicBool::new(false);

// 在另一个线程中：
STOP_FLAG.store(true, Ordering::Release);

// 检查：
if STOP_FLAG.load(Ordering::Acquire) {
    println!("正在停止");
}
```

| C++ 用法 | Rust 等效方案 | 说明 |
|-----------|----------------|-------|
| 针对硬件寄存器的 `volatile` | `ptr::read_volatile` / `ptr::write_volatile` | 需要 `unsafe` —— 适用于 MMIO |
| 针对线程信号的 `volatile` | `AtomicBool` / `AtomicU32` 等 | C++ 在这种场景下使用 `volatile` 也是错误的！ |
| `std::atomic<T>` | `std::sync::atomic::AtomicT` | 相同的语义，相同的内存顺序 (Orderings) |
| `std::atomic<T>::load(memory_order_acquire)` | `AtomicT::load(Ordering::Acquire)` | 1:1 映射 |

---

### `static` 变量 → `static`、`const`、`LazyLock`、`OnceLock`

#### 基础的 `static` 与 `const`

```cpp
// C++
const int MAX_RETRIES = 5;                    // 编译期常量
static std::string CONFIG_PATH = "/etc/app";  // 静态初始化 —— 初始化顺序未定义！
```

```rust
// Rust
const MAX_RETRIES: u32 = 5;                   // 编译期常量，会被内联
static CONFIG_PATH: &str = "/etc/app";         // 'static 生命周期，固定地址
```

#### 静态初始化顺序困境 (Static Initialization Order Fiasco)

C++ 存在一个众所周知的问题：不同编译单元中的全局构造函数执行顺序是**未指定的**。Rust 完全避免了这一问题 —— `static` 值必须是编译期常量（没有构造函数）。

对于运行时初始化的全局变量，请使用 `LazyLock` (Rust 1.80+) 或 `OnceLock`：

```rust
use std::sync::LazyLock;

// 等效于 C++ 的 `static std::regex` —— 在首次访问时初始化，且是线程安全的
static CONFIG_REGEX: LazyLock<regex::Regex> = LazyLock::new(|| {
    regex::Regex::new(r"^[a-z]+_diag$").expect("非法的正则表达式")
});

fn is_valid_diag(name: &str) -> bool {
    CONFIG_REGEX.is_match(name)  // 首次调用时初始化；后续调用速度较快
}
```

```rust
use std::sync::OnceLock;

// OnceLock：仅初始化一次，可使用运行时数据进行设置
static DB_CONN: OnceLock<String> = OnceLock::new();

fn init_db(connection_string: &str) {
    DB_CONN.set(connection_string.to_string())
        .expect("DB_CONN 已经完成初始化");
}

fn get_db() -> &'static str {
    DB_CONN.get().expect("数据库尚未初始化")
}
```

| C++ | Rust | 说明 |
|-----|------|-------|
| `const int X = 5;` | `const X: i32 = 5;` | 均在编译期确定。Rust 要求显式类型标注 |
| `constexpr int X = 5;` | `const X: i32 = 5;` | Rust 的 `const` 始终属于 constexpr |
| 文件作用域的 `static int count = 0;` | `static COUNT: AtomicI32 = AtomicI32::new(0);` | 可变的 static 变量需要 `unsafe` 或原子操作 |
| `static std::string s = "hi";` | `static S: &str = "hi";` 或 `LazyLock<String>` | 简单场景下不存在运行时构造函数 |
| `static MyObj obj;` (复杂初始化) | `static OBJ: LazyLock<MyObj> = LazyLock::new(|| { ... });` | 线程安全、惰性求值、没有初始化顺序问题 |
| `thread_local` | `thread_local! { static X: Cell<u32> = Cell::new(0); }` | 语义相同 |

---

### `constexpr` → `const fn`

C++ 的 `constexpr` 用于标记可以在编译期求值的函数和变量。Rust 处于相同的目的，使用了 `const fn` 和 `const`：

```cpp
// C++
constexpr int factorial(int n) {
    return n <= 1 ? 1 : n * factorial(n - 1);
}
constexpr int val = factorial(5);  // 编译期计算结果为 120
```

```rust
// Rust
const fn factorial(n: u32) -> u32 {
    if n <= 1 { 1 } else { n * factorial(n - 1) }
}
const VAL: u32 = factorial(5);  // 编译期计算结果为 120

// 同样适用于数组长度和 match 模式：
const LOOKUP: [u32; 5] = [factorial(1), factorial(2), factorial(3),
                           factorial(4), factorial(5)];
```

| C++ | Rust | 说明 |
|-----|------|-------|
| `constexpr int f()` | `const fn f() -> i32` | 同样的意图 —— 可在编译期求值 |
| `constexpr` 变量 | `const` 变量 | Rust 的 `const` 始终属于编译期常量 |
| `consteval` (C++20) | 无直接等效项 | `const fn` 也可以在运行时运行 |
| `if constexpr` (C++17) | 无直接等效项（使用 `cfg!` 或泛型） | Trait 特化 (Specialization) 覆盖了部分用例 |
| `constinit` (C++20) | 使用 const 初始化器的 `static` 变量 | Rust 的 `static` 变量默认必须进行 const 初始化 |

> **`const fn` 当前的局限性**（截至 Rust 1.82 已稳定）：
> - 不支持 Trait 方法（无法在常量上下文中对 `Vec` 调用 `.len()`）
> - 不支持堆分配（`Box::new`、`Vec::new` 均不是 const）
> - ~~不支持浮点运算~~ —— **已在 Rust 1.82 中稳定**
> - 无法使用 `for` 循环（请使用递归，或者配合手动索引使用 `while`）

---

### SFINAE 与 `enable_if` → Trait 限定（Trait Bounds）与 `where` 子句

在 C++ 中，SFINAE (Substitution Failure Is Not An Error，替换失败并非错误) 是条件化泛型编程背后的核心机制。虽然它功能强大，但其可读性差也是众所周知的。Rust 完全使用 **Trait 限定 (Trait Bounds)** 替代了这一机制：

```cpp
// C++：基于 SFINAE 的条件函数 (C++20 之前)
template<typename T,
         std::enable_if_t<std::is_integral_v<T>, int> = 0>
T double_it(T val) { return val * 2; }

template<typename T,
         std::enable_if_t<std::is_floating_point_v<T>, int> = 0>
T double_it(T val) { return val * 2.0; }

// C++20 概念 (Concepts) —— 更加整洁，但依然比较冗长：
template<std::integral T>
T double_it(T val) { return val * 2; }
```

```rust
// Rust：Trait 限定 —— 可读性好、可组合，且拥有出色的错误提示
use std::ops::Mul;

fn double_it<T: Mul<Output = T> + From<u8>>(val: T) -> T {
    val * T::from(2)
}

// 或者针对复杂的限定使用 where 子句：
fn process<T>(val: T) -> String
where
    T: std::fmt::Display + Clone + Send,
{
    format!("正在处理：{}", val)
}

// 通过不同的 impl 实现条件行为（取代了 SFINAE 重载）：
trait Describable {
    fn describe(&self) -> String;
}

impl Describable for u32 {
    fn describe(&self) -> String { format!("整数: {self}") }
}

impl Describable for f64 {
    fn describe(&self) -> String { format!("浮点数: {self:.2}") }
}
```

| C++ 模板元编程 | Rust 等效方案 | 可读性 |
|-----------------------------|----------------|-------------|
| `std::enable_if_t<cond>` | `where T: Trait` | 🟢 语义清晰 |
| `std::is_integral_v<T>` | 数值 Trait 限定或特定类型限定 | 🟢 没有 `_v` / `_t` 后缀 |
| SFINAE 重载集合 | 独立的 `impl Trait for ConcreteType` 块 | 🟢 每个 impl 相互独立 |
| `if constexpr (std::is_same_v<T, int>)` | 通过 Trait impl 实现特化 | 🟢 编译期分派 |
| C++20 `concept` | `trait` | 🟢 意图几乎完全一致 |
| `requires` 子句 | `where` 子句 | 🟢 位置相同，语法相似 |
| 模板内部深处触发编译失败 | 在调用处因 Trait 不匹配触发编译失败 | 🟢 不会产生长达 200 行的错误级联 |

> **核心洞察**：C++ 概念 (Concepts, C++20) 是与 Rust Trait 最为接近的概念。如果你熟悉 C++20 的概念，可以将 Rust Trait 看作是自 1.0 版本起就已经作为一等公民存在的、拥有一致实现模型（Trait impls）而非鸭子类型 (Duck Typing) 的“概念”。

---

### `std::function` → 函数指针、`impl Fn` 以及 `Box<dyn Fn>`

C++ 的 `std::function<R(Args...)>` 是一种类型擦除 (Type-erased) 的可调用对象。Rust 提供了三种方案，每种方案都有其优缺点：

```cpp
// C++：通用方案（堆分配、类型擦除）
#include <functional>
std::function<int(int)> make_adder(int n) {
    return [n](int x) { return x + n; };
}
```

```rust
// Rust 方案 1：函数指针 —— 简单、无捕获、无分配
fn add_one(x: i32) -> i32 { x + 1 }
let f: fn(i32) -> i32 = add_one;
println!("{}", f(5)); // 输出 6

// Rust 方案 2：impl Fn —— 单态化、零开销、可捕获
fn apply(val: i32, f: impl Fn(i32) -> i32) -> i32 { f(val) }
let n = 10;
let result = apply(5, |x| x + n);  // 闭包捕获了变量 `n`

// Rust 方案 3：Box<dyn Fn> —— 类型擦除、堆分配（类似于 std::function）
fn make_adder(n: i32) -> Box<dyn Fn(i32) -> i32> {
    Box::new(move |x| x + n)
}
let adder = make_adder(10);
println!("{}", adder(5));  // 输出 15

// 存储异构的可调用对象（类似于 vector<function<int(int)>>）：
let callbacks: Vec<Box<dyn Fn(i32) -> i32>> = vec![
    Box::new(|x| x + 1),
    Box::new(|x| x * 2),
    Box::new(make_adder(100)),
];
for cb in &callbacks {
    println!("{}", cb(5));  // 分别输出 6, 10, 105
}
```

| 使用场景 | C++ 等效方案 | Rust 选型 |
|------------|---------------|-------------|
| 顶级函数，无捕获 | 函数指针 | `fn(Args) -> Ret` |
| 接受可调用对象的泛型函数 | 模板参数 | `impl Fn(Args) -> Ret` (静态分派) |
| 泛型中的 Trait 限定 | `template<typename F>` | `F: Fn(Args) -> Ret` |
| 存储可调用对象，类型擦除 | `std::function<R(Args)>` | `Box<dyn Fn(Args) -> Ret>` |
| 会修改状态的回调函数 | 带有可变 lambda 的 `std::function` | `Box<dyn FnMut(Args) -> Ret>` |
| 仅限调用一次的回调 (一次性消耗) | 被移动的 `std::function` | `Box<dyn FnOnce(Args) -> Ret>` |

> **性能提示**：`impl Fn` 具有零开销（单态化，类似于 C++ 模板）。而 `Box<dyn Fn>` 拥有与 `std::function` 相同的开销（虚函数表 + 堆分配）。除非你需要存储异构的可调用对象，否则请优先使用 `impl Fn`。

---

### 容器映射：C++ STL → Rust `std::collections`

| C++ STL 容器 | Rust 等效容器 | 说明 |
|------------------|----------------|-------|
| `std::vector<T>` | `Vec<T>` | 几乎完全一致的 API。Rust 默认会检查索引是否越界 |
| `std::array<T, N>` | `[T; N]` | 栈分配的固定大小数组 |
| `std::deque<T>` | `std::collections::VecDeque<T>` | 环形缓冲区。在两端进行 push/pop 均非常高效 |
| `std::list<T>` | `std::collections::LinkedList<T>` | 在 Rust 中极少使用 —— `Vec` 几乎总是在性能上胜出 |
| `std::forward_list<T>` | 无直接等效项 | 请使用 `Vec` 或 `VecDeque` |
| `std::unordered_map<K, V>` | `std::collections::HashMap<K, V>` | 默认使用 `SipHash`（具备抗 DoS 攻击能力） |
| `std::map<K, V>` | `std::collections::BTreeMap<K, V>` | B-树；Key 是有序的；要求 `K: Ord` |
| `std::unordered_set<T>` | `std::collections::HashSet<T>` | 要求 `T: Hash + Eq` |
| `std::set<T>` | `std::collections::BTreeSet<T>` | 有序集合；要求 `T: Ord` |
| `std::priority_queue<T>` | `std::collections::BinaryHeap<T>` | 默认为最大堆（与 C++ 一致） |
| `std::stack<T>` | 使用 `.push()` / `.pop()` 的 `Vec<T>` | 无需独立的栈类型 |
| `std::queue<T>` | 使用 `.push_back()` / `.pop_front()` 的 `VecDeque<T>` | 无需独立的队列类型 |
| `std::string` | `String` | 保证是 UTF-8 编码，非 null 结尾 |
| `std::string_view` | `&str` | 借用的 UTF-8 字符串切片 |
| `std::span<T>` (C++20) | `&[T]` / `&mut [T]` | Rust 切片自 1.0 起就是一等公民 |
| `std::tuple<A, B, C>` | `(A, B, C)` | 一等公民语法，支持解构 |
| `std::pair<A, B>` | `(A, B)` | 仅包含两个元素的元组 |
| `std::bitset<N>` | 标准库无等效项 | 请使用 `bitvec` crate，或者 `[u8; N/8]` |

**关键差异点**：
- Rust 的 `HashMap`/`HashSet` 要求 `K: Hash + Eq` —— 编译器会在类型层面强制执行此要求，而不像 C++ 那样会在使用不可哈希的 Key 时在 STL 内部抛出深层的模板错误。
- `Vec` 索引 (`v[i]`) 默认在越界时会触发 panic。建议使用 `.get(i)` 返回 `Option<&T>`，或者利用迭代器来完全避免越界检查。
- 不存在 `std::multimap` 或 `std::multiset` —— 请使用 `HashMap<K, Vec<V>>` 或 `BTreeMap<K, Vec<V>>`。

---

### 异常安全性 → Panic 安全性

C++ 定义了三个级别的异常安全性（Abraham 保证）：

| C++ 级别 | 含义 | Rust 等效概念 |
|----------|---------|----------------|
| **无抛出 (No-throw)** | 函数绝不会抛出异常 | 函数绝不会触发 panic（返回 `Result`） |
| **强保证 (Strong)** | 如果抛出异常，状态保持不变 | 所有权模型使这一点变得非常自然 —— 如果 `?` 提前返回，部分构建的值会被销毁 |
| **基本保证 (Basic)** | 如果抛出异常，不变性依然维持 | Rust 的默认行为 —— `Drop` 会运行，不产生泄漏 |

#### Rust 所有权模型如何提供帮助

```rust
// 免费获得的强保证 —— 如果 file.write() 失败，config 保持不变
fn update_config(config: &mut Config, path: &str) -> Result<(), Error> {
    let new_data = fetch_from_network()?; // Err → 提前返回，config 未受影响
    let validated = validate(new_data)?;   // Err → 提前返回，config 未受影响
    *config = validated;                   // 仅在成功时到达此处（提交修改）
    Ok(())
}
```

在 C++ 中，为了实现强保证，需要手动进行回滚或使用 "copy-and-swap" 原语。在 Rust 中，`?` 的传播机制使得大多数代码默认就具备了强保证。

#### `catch_unwind` —— Rust 版的 `catch(...)`

```rust
use std::panic;

// 捕获 panic（类似于 C++ 中的 catch(...)）—— 极少使用
let result = panic::catch_unwind(|| {
    // 可能会触发 panic 的代码
    let v = vec![1, 2, 3];
    v[10]  // Panic！(索引越界)
});

match result {
    Ok(val) => println!("获取到: {val}"),
    Err(_) => eprintln!("捕获到一次 panic —— 已完成清理"),
}
```

#### `UnwindSafe` —— 将类型标记为 panic 安全

```rust
use std::panic::UnwindSafe;

// 位于 &mut 之后的类型默认不是 UnwindSafe 的 —— 因为 panic 可能会
// 使其处于某种被部分修改的状态
fn safe_execute<F: FnOnce() + UnwindSafe>(f: F) {
    let _ = std::panic::catch_unwind(f);
}

// 当你已经对代码进行了审计，可以使用 AssertUnwindSafe 来覆盖默认行为：
use std::panic::AssertUnwindSafe;
let mut data = vec![1, 2, 3];
let _ = std::panic::catch_unwind(AssertUnwindSafe(|| {
    data.push(4);
}));
```

| C++ 异常模式 | Rust 等效方案 |
|-----------------------|-----------------|
| `throw MyException()` | `return Err(MyError::...)` (推荐) 或 `panic!("...")` |
| `try { } catch (const E& e)` | `match result { Ok(v) => ..., Err(e) => ... }` 或 `?` |
| `catch (...)` | `std::panic::catch_unwind(...)` |
| `noexcept` | `-> Result<T, E>`（错误是值，而非异常） |
| 栈展开过程中的 RAII 清理 | 在 panic 展开期间会运行 `Drop::drop()` |
| `std::uncaught_exceptions()` | `std::thread::panicking()` |
| `-fno-exceptions` 编译标志 | 在 `Cargo.toml` 的 [profile] 中设置 `panic = "abort"` |

> **底线**：在 Rust 中，大多数代码使用 `Result<T, E>` 而非异常，这使得错误路径变得明确且可组合。`panic!` 仅保留给 Bug（例如 `assert!` 失败），而不用于处理常规错误。这意味着“异常安全性”在很大程度上不再是一个难题 —— 所有权系统会自动处理清理工作。

---

## C++ 到 Rust 的迁移模式

### 快速参考：C++ → Rust 惯用写法映射表

| **C++ 模式** | **Rust 惯用写法** | **说明** |
|----------------|---------------|----------|
| `class Derived : public Base` | `enum Variant { A {...}, B {...} }` | 针对封闭集合，优先选择枚举 |
| `virtual void method() = 0` | `trait MyTrait { fn method(&self); }` | 用于开放/可扩展的接口 |
| `dynamic_cast<Derived*>(ptr)` | `match value { Variant::A(data) => ..., }` | 穷尽性检查，无运行时失败风险 |
| `vector<unique_ptr<Base>>` | `Vec<Box<dyn Trait>>` | 仅当确实需要多态性时使用 |
| `shared_ptr<T>` | `Rc<T>` 或 `Arc<T>` | 优先考虑 `Box<T>` 或所有权值 |
| `enable_shared_from_this<T>` | Arena 模式（`Vec<T>` + 索引） | 从根本上消除引用循环 |
| 每个类中都有 `Base* m_pFramework` | `fn execute(&mut self, ctx: &mut Context)` | 传递上下文，不要存储指针 |
| `try { } catch (...) { }` | `match result { Ok(v) => ..., Err(e) => ... }` | 或是使用 `?` 进行错误传播 |
| `std::optional<T>` | `Option<T>` | 强制要求 `match`，不会遗忘 None 的情况 |
| `const std::string&` 参数 | `&str` 参数 | 同时兼容 `String` 与 `&str` |
| `enum class Foo { A, B, C }` | `enum Foo { A, B, C }` | Rust 枚举还可以携带数据 |
| `auto x = std::move(obj)` | `let x = obj;` | 移动是默认行为，无需 `std::move` |
| CMake + make + lint | `cargo build / test / clippy / fmt` | 一个工具搞定所有事情 |

### 迁移策略
1. **从数据类型开始**：首先翻译结构体和枚举 —— 这将迫使你思考所有权问题。
2. **将工厂模式转换为枚举**：如果一个工厂类会创建不同的派生类型，它可能应该被替换为 `enum` + `match`。
3. **将上帝对象 (God Objects) 拆分为组合结构体**：将相关的字段分组到聚焦的结构体中。
4. **用借用替代指针**：将存储的 `Base*` 指针转换为带有生命周期限定的 `&'a T` 借用。
5. **谨慎使用 `Box<dyn Trait>`**：仅将其用于插件系统和测试中的 Mock 模拟。
6. **让编译器引导你**：Rust 的错误提示非常出色 —— 请务必仔细阅读它们。

---
