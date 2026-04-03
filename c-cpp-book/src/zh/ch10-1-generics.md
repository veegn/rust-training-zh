[English Original](../en/ch10-1-generics.md)

# Rust 泛型 (Generics)

> **你将学到：** 泛型类型参数、单态化（零开销泛型）、特性结合，以及 Rust 泛型与 C++ 模板的对比 —— 具有更友好的错误消息且无需 SFINAE。

- 泛型允许相同的算法或数据结构跨多种数据类型进行复用。
    - 泛型参数以标识符的形式出现在 `<>` 中，例如：`<T>`。该参数可以使用任何合法的标识符名称，但为了简洁，通常保持较短。
    - 编译器在编译时执行“单态化 (Monomorphization)”，即为遇到的每一个 `T` 的变体生成一个新的类型。
```rust
// 返回一个由 T 类型的 left 和 right 组成的 T 类型元组
fn pick<T>(x: u32, left: T, right: T) -> (T, T) {
   if x == 42 {
    (left, right) 
   } else {
    (right, left)
   }
}
fn main() {
    let a = pick(42, true, false);
    let b = pick(42, "hello", "world");
    println!("{a:?}, {b:?}");
}
```

---

# Rust 泛型

- 泛型也可以应用于数据类型及其关联方法。还可以为特定的 `<T>`（例如：`f32` vs. `u32`）实现特化。
```rust
#[derive(Debug)] // 我们稍后会讨论这个
struct Point<T> {
    x : T,
    y : T,
}

impl<T> Point<T> {
    fn new(x: T, y: T) -> Self {
        Point {x, y}
    }
    fn set_x(&mut self, x: T) {
         self.x = x;       
    }
    fn set_y(&mut self, y: T) {
         self.y = y;       
    }
}

impl Point<f32> {
    fn is_secret(&self) -> bool {
        self.x == 42.0
    }    
}

fn main() {
    let mut p = Point::new(2, 4); // 推导为 i32
    let q = Point::new(2.0, 4.0); // 推导为 f32
    p.set_x(42);
    p.set_y(43);
    println!("{p:?} {q:?} {}", q.is_secret());
}
```

---

# 练习：泛型

平衡性 **入门**
- 修改 `Point` 类型，使其对 `x` 和 `y` 使用两个不同的泛型类型（`T` 和 `U`）。

<details><summary>参考答案 (点击展开)</summary>

```rust
#[derive(Debug)]
struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn new(x: T, y: U) -> Self {
        Point { x, y }
    }
}

fn main() {
    let p1 = Point::new(42, 3.14);        // Point<i32, f64>
    let p2 = Point::new("你好", true);     // Point<&str, bool>
    let p3 = Point::new(1u8, 1000u64);    // Point<u8, u64>
    println!("{p1:?}");
    println!("{p2:?}");
    println!("{p3:?}");
}
```
**输出示例：**
```text
Point { x: 42, y: 3.14 }
Point { x: "你好", y: true }
Point { x: 1, y: 1000 }
```

</details>

---

### 特性与泛型的结合
- 特性可用于对泛型类型施加限制（约束）。
- 可以通过在泛型类型参数后面使用 `:` 或使用 `where` 子句来指定约束。下文定义了一个泛型函数 `get_area`，它接受任何实现了 `ComputeArea` 特性的类型 `T`。
```rust
trait ComputeArea {
    fn area(&self) -> u64;
}
fn get_area<T: ComputeArea>(t: &T) -> u64 {
    t.area()
}
```
- [▶ 在 Rust Playground 中尝试](https://play.rust-lang.org/)

### 特性与泛型的结合
- 也可以设置多个特性约束。
```rust
trait Fish {}
trait Mammal {}
struct Shark;
struct Whale;
impl Fish for Shark {}
impl Fish for Whale {}
impl Mammal for Whale {}
fn only_fish_and_mammals<T: Fish + Mammal>(_t: &T) {}
fn main() {
    let w = Whale {};
    only_fish_and_mammals(&w);
    let _s = Shark {};
    // 下行将无法编译
    // only_fish_and_mammals(&_s);
}
```

---

### 数据类型中的 Rust 特性约束
- 特性约束可以与数据类型中的泛型相结合。
- 在下例中，我们定义了 `PrintDescription` 特性以及一个名为 `Shape` 的泛型结构体，其成员受到该特性的约束。
```rust
trait PrintDescription {
    fn print_description(&self);
}
struct Shape<S: PrintDescription> {
    shape: S,
}
// 为任何实现了 PrintDescription 的类型 S 实现泛型结构体 Shape
impl<S: PrintDescription> Shape<S> {
    fn print(&self) {
        self.shape.print_description();
    }
}
```
- [▶ 在 Rust Playground 中尝试](https://play.rust-lang.org/)

---

# 练习：特性约束与泛型

🟡 **中级**

- 实现一个包含泛型成员 `cipher` 的 `struct`，且该成员需实现 `CipherText` 特性。
```rust
trait CipherText {
    fn encrypt(&self);
}
// 待完成
//struct Cipher<>
```
- 接着，在结构体的 `impl` 块中实现一个名为 `encrypt` 的方法，该方法调用 `cipher` 的 `encrypt` 方法。
```rust
// 待完成
impl for Cipher<> {}
```
- 最后，为名为 `CipherOne` 和 `CipherTwo` 的两个结构体实现 `CipherText` 特性（仅打印 `println()` 即可）。创建 `CipherOne` 和 `CipherTwo` 的实例，并使用 `Cipher` 调用它们。

<details><summary>参考答案 (点击展开)</summary>

```rust
trait CipherText {
    fn encrypt(&self);
}

struct Cipher<T: CipherText> {
    cipher: T,
}

impl<T: CipherText> Cipher<T> {
    fn encrypt(&self) {
        self.cipher.encrypt();
    }
}

struct CipherOne;
struct CipherTwo;

impl CipherText for CipherOne {
    fn encrypt(&self) {
        println!("已应用 CipherOne 加密");
    }
}

impl CipherText for CipherTwo {
    fn encrypt(&self) {
        println!("已应用 CipherTwo 加密");
    }
}

fn main() {
    let c1 = Cipher { cipher: CipherOne };
    let c2 = Cipher { cipher: CipherTwo };
    c1.encrypt();
    c2.encrypt();
}
```
**输出示例：**
```text
已应用 CipherOne 加密
已应用 CipherTwo 加密
```

</details>

---

### Rust 类型状态模式 (Type State Pattern) 与泛型
- Rust 类型可用于在**编译时**强制执行状态机转换。
    - 想象一架具有两种状态的“无人机”：`Idle`（空闲）和 `Flying`（飞行）。在 `Idle` 状态下，唯一允许的方法是 `takeoff()`。在 `Flying` 状态下，我们允许 `land()`。
    
- 一种方法是使用类似以下代码的方式对状态机建模：
```rust
enum DroneState {
    Idle,
    Flying
}
struct Drone {x: u64, y: u64, z: u64, state: DroneState}  // x, y, z 为坐标
```
- 这需要大量的运行时检查来强制执行状态机语义 —— [▶ 尝试一下](https://play.rust-lang.org/)以了解其原因。

### 使用泛型的类型状态模式
- 泛型允许我们在**编译时**强制执行状态机。这需要使用一种特殊的泛型，即 `PhantomData<T>`。
- `PhantomData<T>` 是一种**零大小 (Zero-sized)** 的标记数据类型。在本例中，我们用它来表示 `Idle` 和 `Flying` 状态，但它在运行时的占用空间为**计为零**。
- 请注意，`takeoff` 和 `land` 方法接收 `self` 作为参数。这被称为“**消耗 (Consuming)**”（与之相对的是使用借用的 `&self`）。基本上，一旦我们调用了 `Drone<Idle>` 的 `takeoff()` 方法，我们就只能取回一个 `Drone<Flying>`，反之亦然。
```rust
struct Drone<T> {x: u64, y: u64, z: u64, state: PhantomData<T> }
impl Drone<Idle> {
    fn takeoff(self) -> Drone<Flying> {...}
}
impl Drone<Flying> {
    fn land(self) -> Drone<Idle> { ...}
}
```
    - [▶ 在 Rust Playground 中尝试](https://play.rust-lang.org/)

### 使用泛型的类型状态模式
- 核心要点：
    - 状态可以使用结构体表示（零大小）。
    - 我们可以将状态 `T` 与 `PhantomData<T>` 相结合（零大小）。
    - 为状态机的特定阶段实现方法，现在只需编写 `impl State<T>`。
    - 使用消耗 `self` 的方法从一个状态转换到另一个状态。
    - 这为我们提供了**零成本 (Zero cost)** 抽象。编译器可以在编译时强制执行状态机规则，除非状态匹配，否则根本无法通过编译调用方法。

---

### Rust 构建器 (Builder) 模式
- 消耗 `self` 的模式对于构建器模式也非常有用。
- 考虑一个具有几十个引脚的 GPIO 配置。引脚可以配置为高电平或低电平（默认为低电平）。
```rust
#[derive(Default)]
enum PinState {
    #[default]
    Low,
    High,
} 
#[derive(Default)]
struct GPIOConfig {
    pin0: PinState,
    pin1: PinState,
    // ... 
}
```
- 构建器模式可以通过链式调用来构造 GPIO 配置 —— [▶ 尝试一下](https://play.rust-lang.org/)。

---
