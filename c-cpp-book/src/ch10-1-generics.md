# Rust generics / Rust 泛型
 
 > **What you'll learn / 你将学到：** Generic type parameters, monomorphization (zero-cost generics), trait bounds, and how Rust generics compare to C++ templates — with better error messages and no SFINAE.
 >
 > 泛型类型参数、单态化（零成本泛型）、trait 限定，以及 Rust 泛型与 C++ 模板的点对点对比 —— 拥有更好的错误消息且无需 SFINAE。
 
 - Generics allow the same algorithm or data structure to be reused across data types
+ - 泛型允许相同的算法或数据结构在不同的数据类型之间复用
-     - The generic parameter appears as an identifier within ```<>```, e.g.: ```<T>```. The parameter can have any legal identifier name, but is typically kept short for brevity
+     - 泛型参数以 ```<>``` 内的标识符形式出现，例如：```<T>```。该参数可以是任何合法的标识符名称，但通常为了简洁而保持简短。
-     - The compiler performs monomorphization at compile time, i.e., it generates a new type for every variation of ```T``` that is encountered
+     - 编译器在编译时执行**单态化（monomorphization）**，即为遇到的每种 ```T``` 的变体生成一个新的类型版本。
 ```rust
-// Returns a tuple of type <T> composed of left and right of type <T>
+// Returns a tuple of type <T> composed of left and right of type <T> / 返回一个由类型为 <T> 的 left 和 right 组成的元组
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
 
- # Rust generics
+ # Rust generics continued / Rust 泛型（续）
- - Generics can also be applied to data types and associated methods. It is possible to specialize the implementation for a specific ```<T>``` (example: ```f32``` vs. ```u32```)
+ - 泛型也可以应用于数据类型和关联方法。可以针对特定的 ```<T>``` 进行专有化实现（例如：```f32``` 与 ```u32``` 的区别）。
 ```rust
- #[derive(Debug)] // We will discuss this later
+ #[derive(Debug)] // We will discuss this later / 我们稍后会讨论这个
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
- impl Point<f32> {
+ impl Point<f32> { // 为 f32 类型专门实现的方法
     fn is_secret(&self) -> bool {
         self.x == 42.0
     }    
 }
 fn main() {
-    let mut p = Point::new(2, 4); // i32
+    let mut p = Point::new(2, 4); // i32 类型
-    let q = Point::new(2.0, 4.0); // f32 类型
+    let q = Point::new(2.0, 4.0); // f32 类型
     p.set_x(42);
     p.set_y(43);
     println!("{p:?} {q:?} {}", q.is_secret());
 }
 ```
 
- # Exercise: Generics
+ # Exercise: Generics / 练习：泛型
 
- 🟢 **Starter**
+ 🟢 **Starter / 入门级**
- - Modify the ```Point``` type to use two different types (```T``` and ```U```) for x and y
+ - 修改 ```Point``` 类型，使其在 x 和 y 上使用两种不同的类型（```T``` 和 ```U```）。
 
- <details><summary>Solution (click to expand)</summary>
+ <details><summary>Solution (click to expand) / 解决方案（点击展开）</summary>
 
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
-    let p1 = Point::new(42, 3.14);        // Point<i32, f64>
+    let p1 = Point::new(42, 3.14);        // 类型为 Point<i32, f64>
-    let p2 = Point::new("hello", true);   // Point<&str, bool>
+    let p2 = Point::new("hello", true);   // 类型为 Point<&str, bool>
-    let p3 = Point::new(1u8, 1000u64);    // Point<u8, u64>
+    let p3 = Point::new(1u8, 1000u64);    // 类型为 Point<u8, u64>
     println!("{p1:?}");
     println!("{p2:?}");
     println!("{p3:?}");
 }
- // Output:
+ // Output / 输出：
 // Point { x: 42, y: 3.14 }
 // Point { x: "hello", y: true }
 // Point { x: 1, y: 1000 }
 ```
 
 </details>
 
- ### Combining Rust traits and generics
+ ### Combining Rust traits and generics / 结合 Rust Trait 与泛型
- - Traits can be used to place restrictions on generic types (constraints)
+ - Trait 可用于对泛型类型施加限制（约束/限定）
- - The constraint can be specified using a ```:``` after the generic type parameter, or using ```where```. The following defines a generic function ```get_area``` that takes any type ```T``` as long as it implements the ```ComputeArea``` ```trait```
+ - 约束可以使用泛型类型参数后的 ```:``` 来指定，也可以使用 ```where``` 子句。下面定义了一个泛型函数 ```get_area```，它接受任何实现了 ```ComputeArea``` ```trait``` 的类型 ```T```：
 ```rust
     trait ComputeArea {
         fn area(&self) -> u64;
     }
-    fn get_area<T: ComputeArea>(t: &T) -> u64 {
+    fn get_area<T: ComputeArea>(t: &T) -> u64 { // 使用冒号指定约束
         t.area()
     }
 ```
- - [▶ Try it in the Rust Playground](https://play.rust-lang.org/)
+ - [▶ 在 Rust Playground 中尝试](https://play.rust-lang.org/)
 
- ### Combining Rust traits and generics
+ ### Combining Rust traits and generics continued / 结合 Rust Trait 与泛型（续）
- - It is possible to have multiple trait constraints
+ - 可以有多个 trait 约束
 ```rust
 trait Fish {}
 trait Mammal {}
 struct Shark;
 struct Whale;
 impl Fish for Shark {}
 impl Fish for Whale {}
 impl Mammal for Whale {}
- fn only_fish_and_mammals<T: Fish + Mammal>(_t: &T) {}
+ fn only_fish_and_mammals<T: Fish + Mammal>(_t: &T) {} // 必须同时实现 Fish 和 Mammal
 fn main() {
     let w = Whale {};
-    only_fish_and_mammals(&w);
+    only_fish_and_mammals(&w); // 成功
     let _s = Shark {};
-    // Won't compile
+    // Won't compile / 无法编译
     only_fish_and_mammals(&_s);
 }
 ```
 
- ### Rust traits constraints in data types
+ ### Rust traits constraints in data types / 数据类型中的 Trait 约束
- - Trait constraints can be combined with generics in data types
+ - Trait 约束可以与数据类型中的泛型相结合。
- - In the following example, we define the ```PrintDescription``` ```trait``` and a generic ```struct``` ```Shape``` with a member constrained by the trait
+ - 在下面的示例中，我们定义了 ```PrintDescription``` ```trait``` 和一个带有受该 trait 约束的成员的泛型 ```struct``` ```Shape```：
 ```rust
 trait PrintDescription {
     fn print_description(&self);
 }
 struct Shape<S: PrintDescription> {
     shape: S,
 }
- // Generic Shape implementation for any type that implements PrintDescription
+ // Generic Shape implementation / 针对任何实现 PrintDescription 的类型的泛型 Shape 实现
 impl<S: PrintDescription> Shape<S> {
     fn print(&self) {
         self.shape.print_description();
     }
 }
 ```
- - [▶ Try it in the Rust Playground](https://play.rust-lang.org/)
+ - [▶ 在 Rust Playground 中尝试](https://play.rust-lang.org/)
 
- # Exercise: Trait constraints and generics
+ # Exercise: Trait constraints and generics / 练习：Trait 约束与泛型
 
- 🟡 **Intermediate**
+ 🟡 **Intermediate / 中级**
- - Implement a ```struct``` with a generic member ```cipher``` that implements ```CipherText```
+ - 实现一个带有一个实现了 ```CipherText``` 的泛型成员 ```cipher``` 的 ```struct```。
 ```rust
 trait CipherText {
     fn encrypt(&self);
 }
- // TO DO
+ // TO DO / 待办
 //struct Cipher<>
 
 ```
- - Next, implement a method called ```encrypt``` on the ```struct``` ```impl``` that invokes ```encrypt``` on ```cipher```
+ - 接着，在 ```struct``` 的 ```impl``` 中实现一个名为 ```encrypt``` 的方法，该方法在 ```cipher``` 上调用 ```encrypt```。
 ```rust
- // TO DO
+ // TO DO / 待办
 impl for Cipher<> {}
 ```
- - Next, implement ```CipherText``` on two structs called ```CipherOne``` and ```CipherTwo``` (just ```println()``` is fine). Create ```CipherOne``` and ```CipherTwo```, and use ```Cipher``` to invoke them
+ - 接着，在名为 ```CipherOne``` 和 ```CipherTwo``` 的两个结构体上实现 ```CipherText```（仅使用 ```println()``` 即可）。创建 ```CipherOne``` 和 ```CipherTwo``` 实例，并使用 ```Cipher``` 来调用它们。
 
- <details><summary>Solution (click to expand)</summary>
+ <details><summary>Solution (click to expand) / 解决方案（点击展开）</summary>
 
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
         println!("CipherOne encryption applied");
     }
 }
 
 impl CipherText for CipherTwo {
     fn encrypt(&self) {
         println!("CipherTwo encryption applied");
     }
 }
 
 fn main() {
     let c1 = Cipher { cipher: CipherOne };
     let c2 = Cipher { cipher: CipherTwo };
     c1.encrypt();
     c2.encrypt();
 }
- // Output:
+ // Output / 输出：
 // CipherOne encryption applied
 // CipherTwo encryption applied
 ```
 
 </details>
 
- ### Rust type state pattern and generics
+ ### Rust type state pattern and generics / Rust 类型状态（Type State）模式与泛型
- - Rust types can be used to enforce state machine transitions at *compile* time
+ - Rust 类型可用于在**编译时**强制执行状态机转换。
-     - Consider a ```Drone``` with say two states: ```Idle``` and ```Flying```. In the ```Idle``` state, the only permitted method is ```takeoff()```. In the ```Flying``` state, we permit ```land()```
+     - 考虑一个具有两种状态（比如 ```Idle``` 和 ```Flying```）的 ```Drone```（无人机）。在 ```Idle``` 状态下，唯一允许的方法是 ```takeoff()```（起飞）。在 ```Flying``` 状态下，我们允许 ```land()```（降落）。
-     
- - One approach is to model the state machine using something like the following
+ - 一种方法是使用类似以下的方式为状态机建模：
 ```rust
 enum DroneState {
     Idle,
     Flying
 }
- struct Drone {x: u64, y: u64, z: u64, state: DroneState}  // x, y, z are coordinates
+ struct Drone {x: u64, y: u64, z: u64, state: DroneState}  // x, y, z 为坐标
 ```
- - This requires a lot of runtime checks to enforce the state machine semantics — [▶ try it](https://play.rust-lang.org/) to see why
+ - 这需要大量的运行时检查来强制执行状态机语义 —— [▶ 尝试一下](https://play.rust-lang.org/) 看看为什么。
 
- ### Rust type state pattern generics
+ ### Rust type state pattern generics continued / Rust 类型状态模式泛型（续）
- - Generics allows us to enforce the state machine at *compile time*. This requires using a special generic called ```PhantomData<T>```
+ - 泛型允许我们在**编译时**强制执行状态机。这需要使用一个名为 ```PhantomData<T>``` 的特殊泛型。
- - The ```PhantomData<T>``` is a ```zero-sized``` marker data type. In this case, we use it to represent the ```Idle``` and ```Flying``` states, but it has ```zero``` runtime size
+ - ```PhantomData<T>``` 是一个**零大小（zero-sized）**的标记数据类型。在本例中，我们使用它来表示 ```Idle``` 和 ```Flying``` 状态，但它的运行时大小为**零**。
- - Notice that the ```takeoff``` and ```land``` methods take ```self``` as a parameter. This is referred to as ```consuming``` (contrast with ```&self``` which uses borrowing). Basically, once we call the ```takeoff()``` on ```Drone<Idle>```, we can only get back a ```Drone<Flying>``` and viceversa
+ - 请注意，```takeoff``` 和 ```land``` 方法将 ```self``` 作为参数。这被称为**消耗（consuming）**（与使用借用的 ```&self``` 相对）。基本上，一旦我们在 ```Drone<Idle>``` 上调用了 ```takeoff()```，我们只能得到一个 ```Drone<Flying>```，反之亦然。
 ```rust
 struct Drone<T> {x: u64, y: u64, z: u64, state: PhantomData<T> }
 impl Drone<Idle> {
     fn takeoff(self) -> Drone<Flying> {...}
 }
 impl Drone<Flying> {
     fn land(self) -> Drone<Idle> { ...}
 }
 ```
-     - [▶ Try it in the Rust Playground](https://play.rust-lang.org/)
+     - [▶ 在 Rust Playground 中尝试](https://play.rust-lang.org/)
 
- ### Rust type state pattern generics
+ ### Rust type state pattern generics key takeaways / Rust 类型状态模式泛型要点
- - Key takeaways:
-     - States can be represented using structs (zero-size)
+ - 状态可以使用结构体表示（零大小）。
-     - We can combine the state ```T``` with ```PhantomData<T>``` (zero-size)
+ - 我们可以将状态 ```T``` 与 ```PhantomData<T>``` 结合（零大小）。
-     - Implementing the methods for a particular stage of the state machine is now just a matter of ```impl State<T>```
+ - 为状态机的特定阶段实现方法现在只需 ```impl State<T>```。
-     - Use a method that consumes ```self``` to transition from one state to another
+ - 使用消耗 ```self``` 的方法实现从一个状态到另一个状态的转换。
-     - This gives us ```zero cost``` abstractions. The compiler can enforce the state machine at compile time and it's impossible to call methods unless the state is right
+ - 这为我们提供了**零成本（zero cost）**抽象。编译器可以在编译时强制执行状态机，并且除非状态正确，否则不可能调用某些方法。
 
- ### Rust builder pattern
+ ### Rust builder pattern / Rust 构建器模式
- - The consume ```self``` can be useful for builder patterns
+ - 消耗 ```self``` 在构建器模式中非常有用。
- - Consider a GPIO configuration with several dozen pins. The pins can be configured to high or low (default is low)
+ - 考虑一个具有几十个引脚的 GPIO 配置。引脚可以配置为高电平或低电平（默认为低）。
 ```rust
 #[derive(default)]
 enum PinState {
     #[default]
     Low,
     High,
 } 
 #[derive(default)]
 struct GPIOConfig {
     pin0: PinState,
     pin1: PinState
-    ... 
+    // ... 
 }
 ```
- - The builder pattern can be used to construct a GPIO configuration by chaining — [▶ Try it](https://play.rust-lang.org/)
+ - 构建器模式可以通过链式调用来构造 GPIO 配置 —— [▶ 尝试一下](https://play.rust-lang.org/)。
