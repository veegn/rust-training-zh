# 4. Control Flow / 4. 控制流
 
 > **What you'll learn / 你将学到：** Rust's control flow constructs — `if`/`else` as expressions, `loop`/`while`/`for`, `match`, and how they differ from C/C++ counterparts. The key insight: most Rust control flow returns values.
 >
 > Rust 的控制流结构 —— 作为表达式的 `if`/`else`、`loop`/`while`/`for`、`match`，以及它们与 C/C++ 对应部分的区别。核心见解：大多数 Rust 控制流都会返回值。
 
 - In Rust, ```if``` is actually an expression, i.e., it can be used to assign values, but it also behaves like a statement. [▶ Try it / 尝试运行](https://play.rust-lang.org/)
 
 ```rust
 fn main() {
     let x = 42;
     if x < 42 {
         println!("Smaller than the secret of life / 小于生命之秘");
     } else if x == 42 {
         println!("Is equal to the secret of life / 等于生命之秘");
     } else {
         println!("Larger than the secret of life / 大于生命之秘");
     }
     let is_secret_of_life = if x == 42 {true} else {false};
     println!("{}", is_secret_of_life);
 }
 ```
 
 # Rust loops using while and for / Rust 循环：while 与 for
 - The ```while``` keyword can be used to loop while an expression is true / ```while``` 关键字用于在表达式为真时进行循环
 ```rust
 fn main() {
     let mut x = 40;
     while x != 42 {
         x += 1;
     }
 }
 ```
 - The ```for``` keyword can be used to iterate over ranges / ```for``` 关键字可用于遍历范围
 ```rust
 fn main() {
     // Will not print 43; use 40..=43 to include last element
     // 不会打印 43；使用 40..=43 来包含最后一个元素
     for x in 40..43 {
         println!("{}", x);
     } 
 }
 ```
 
 # Rust loops using loop / Rust 循环：loop
 - The ```loop``` keyword creates an infinite loop until a ```break``` is encountered / ```loop``` 关键字创建一个无限循环，直到遇到 ```break```
 ```rust
 fn main() {
     let mut x = 40;
     // Change the below to 'here: loop to specify optional label for the loop
     // 将下面改为 'here: loop 可以为循环指定可选标签
     loop {
         if x == 42 {
             break; // Use break x; to return the value of x / 使用 break x; 可以返回 x 的值
         }
         x += 1;
     }
 }
 ```
 - The ```break``` statement can include an optional expression that can be used to assign the value of a ```loop``` expression / ```break``` 语句可以包含一个可选表达式，用于为 ```loop``` 表达式赋值
 - The ```continue``` keyword can be used to return to the top of the ```loop``` / ```continue``` 关键字可以用于返回 ```loop``` 的顶部
 - Loop labels can be used with ```break``` or ```continue``` and are useful when dealing with nested loops / 循环标签可以配合 ```break``` 或 ```continue``` 使用，在处理嵌套循环时非常有用
 
 # Rust expression blocks / Rust 表达式块
 - Rust expression blocks are simply a sequence of expressions enclosed in ```{}```. The evaluated value is simply the last expression in the block / Rust 表达式块只是用 ```{}``` 包裹的一系列表达式。其求值结果就是块中的最后一个表达式
 ```rust
 fn main() {
     let x = {
         let y = 40;
         y + 2 // Note: ; must be omitted / 注意：必须省略分号 ;
     };
     // Notice the Python style printing / 注意这种 Python 风格的打印方式
     println!("{x}");
 }
 ```
 - Rust style is to use this to omit the ```return``` keyword in functions / Rust 的风格是利用这一点在函数中省略 ```return``` 关键字
 ```rust
 fn is_secret_of_life(x: u32) -> bool {
     // Same as if x == 42 {true} else {false}
     // 等同于 if x == 42 {true} else {false}
     x == 42 // Note: ; must be omitted / 注意：必须省略分号 ;
 }
 fn main() {
     println!("{}", is_secret_of_life(42));
 }
 ```
