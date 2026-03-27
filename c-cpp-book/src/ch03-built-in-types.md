# 3. Built-in Rust types / 3. Rust 内建类型
 
 > **What you'll learn / 你将学到：** Rust's fundamental types (`i32`, `u64`, `f64`, `bool`, `char`), type inference, explicit type annotations, and how they compare to C/C++ primitive types. No implicit conversions — Rust requires explicit casts.
 >
 > Rust 的基本类型（`i32`、`u64`、`f64`、`bool`、`char`）、类型推导、显式类型标注，以及它们与 C/C++ 原生类型的对比。Rust 没有隐式转换 —— 所有转换都必须显式使用 cast。
 
 - Rust has type inference, but also allows explicit specification of the type / Rust 拥有类型推导能力，但也允许显式指定类型
 
 | **Description / 描述** | **Type / 类型** | **Example / 示例** |
 |:-----------------:|:------------------------------:|:-----------------------------:|
 | Signed integers / 有符号整数 | i8, i16, i32, i64, i128, isize | -1, 42, 1_00_000, 1_00_000i64 |
 | Unsigned integers / 无符号整数 | u8, u16, u32, u64, u128, usize | 0, 42, 42u32, 42u64 |
 | Floating point / 浮点数 | f32, f64 | 0.0, 0.42 |
 | Unicode / Unicode 字符 | char | 'a', '$' |
 | Boolean / 布尔值 | bool | true, false |
 
 - Rust permits arbitrarily use of ```_``` between numbers for ease of reading / Rust 允许在数字之间任意使用 ```_``` 以提高可读性
 ----
 ### Rust type specification and assignment / Rust 类型指定与赋值
 - Rust uses the ```let``` keyword to assign values to variables. The type of the variable can be optionally specified after a ```:``` / Rust 使用 ```let``` 关键字为变量赋值。类型可以可选地写在 ```:``` 之后
 ```rust
 fn main() {
     let x : i32 = 42;
     // These two assignments are logically equivalent / 这两个赋值在逻辑上是等价的
     let y : u32 = 42;
     let z = 42u32;
 }
 ``` 
 - Function parameters and return values (if any) require an explicit type. The following takes an u8 parameter and returns u32 / 函数参数和返回值（如果有）需要显式指定类型。以下函数接收一个 u8 参数并返回 u32
 ```rust
 fn foo(x : u8) -> u32
 {
     return x as u32 * x as u32;
 }
 ```
 - Unused variables are prefixed with ```_``` to avoid compiler warnings / 未使用的变量以前缀 ```_``` 开头，以避免编译器警告
 ----
 # Rust type specification and inference / Rust 类型指定与推导
 - Rust can automatically infer the type of the variable based on the context. / Rust 可以根据上下文自动推导变量的类型。
 - [▶ Try it in the Rust Playground / 在 Rust Playground 中尝试](https://play.rust-lang.org/)
 ```rust
 fn secret_of_life_u32(x : u32) {
     println!("The u32 secret_of_life is {}", x);
 }
 
 fn secret_of_life_u8(x : u8) {
     println!("The u8 secret_of_life is {}", x);
 }
 
 fn main() {
     let a = 42; // The let keyword assigns a value; type of a is u32 / let 关键字赋值；a 的类型被推导为 u32
     let b = 42; // The let keyword assigns a value; inferred type of b is u8 / let 关键字赋值；b 的推导类型为 u8
     secret_of_life_u32(a);
     secret_of_life_u8(b);
 }
 ```
 
 # Rust variables and mutability / Rust 变量与可变性
 - Rust variables are **immutable** by default unless the ```mut``` keyword is used to denote that a variable is mutable. For example, the following code will not compile unless the ```let a = 42``` is changed to ```let mut a = 42``` / Rust 变量默认是**不可变**的，除非使用 ```mut``` 关键字声明变量可变。例如，除非将 ```let a = 42``` 改为 ```let mut a = 42```，否则以下代码将无法编译
 ```rust
 fn main() {
     let a = 42; // Must be changed to let mut a = 42 to permit the assignment below / 必须改为 let mut a = 42 才能允许下面的赋值
     a = 43;  // Will not compile unless the above is changed / 除非修改上面一行，否则无法编译
 }
 ```
 - Rust permits the reuse of the variable names (shadowing) / Rust 允许变量名复用（变量遮蔽，Shadowing）
 ```rust
 fn main() {
     let a = 42;
     {
         let a = 43; //OK: Different variable with the same name / OK：同名但不同的变量
     }
     // a = 43; // Not permitted / 不允许
     let a = 43; // Ok: New variable and assignment / Ok：新变量及赋值
 }
 ```
