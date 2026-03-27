# Rust From and Into traits / Rust From 与 Into Trait
 
 > **What you'll learn / 你将学到：** Rust's type conversion traits — `From<T>` and `Into<T>` for infallible conversions, `TryFrom` and `TryInto` for fallible ones. Implement `From` and get `Into` for free. Replaces C++ conversion operators and constructors.
 >
 > Rust 的类型转换 trait —— 用于无误转换的 `From<T>` 和 `Into<T>`，以及用于可能失败转换的 `TryFrom` 和 `TryInto`。实现 `From` 即可免费获得 `Into`。取代了 C++ 的转换运算符和构造函数。
 
 - ```From``` and ```Into``` are complementary traits to facilitate type conversion
+ - ```From``` 和 ```Into``` 是互补的 trait，旨在简化类型转换。
- - Types normally implement on the ```From``` trait. the ```String::from()``` converts from "&str" to ```String```, and compiler can automatically derive ```&str.into```
+ - 类型通常实现 ```From``` trait。比如 ```String::from()``` 将 "&str" 转换为 ```String```，编译器可以自动根据此推导出 ```&str.into``` 的实现。
 ```rust
 struct Point {x: u32, y: u32}
-// Construct a Point from a tuple
+// Construct a Point from a tuple / 从元组构造 Point
 impl From<(u32, u32)> for Point {
     fn from(xy : (u32, u32)) -> Self {
-        Point {x : xy.0, y: xy.1}       // Construct Point using the tuple elements
+        Point {x : xy.0, y: xy.1}       // Using elements / 使用元组元素构造 Point
     }
 }
 fn main() {
     let s = String::from("Rust");
     let x = u32::from(true);
     let p = Point::from((40, 42));
-    // let p : Point = (40.42)::into(); // Alternate form of the above
+    // let p : Point = (40.42)::into(); // Alternate form / 上述形式的另一种写法
     println!("s: {s} x:{x} p.x:{} p.y {}", p.x, p.y);   
 }
 ```
 
- # Exercise: From and Into
+ # Exercise: From and Into / 练习：From 与 Into
- - Implement a ```From``` trait for ```Point``` to convert into a type called ```TransposePoint```. ```TransposePoint``` swaps the ```x``` and ```y``` elements of ```Point```
+ - 为 ```Point``` 实现 ```From``` trait，将其转换为名为 ```TransposePoint``` 的类型。```TransposePoint``` 会交换 ```Point``` 的 ```x``` 和 ```y``` 元素。
 
- <details><summary>Solution (click to expand)</summary>
+ <details><summary>Solution (click to expand) / 解决方案（点击展开）</summary>
 
 ```rust
 struct Point { x: u32, y: u32 }
 struct TransposePoint { x: u32, y: u32 }
 
 impl From<Point> for TransposePoint {
     fn from(p: Point) -> Self {
         TransposePoint { x: p.y, y: p.x }
     }
 }
 
 fn main() {
     let p = Point { x: 10, y: 20 };
     let tp = TransposePoint::from(p);
-    println!("Transposed: x={}, y={}", tp.x, tp.y);  // x=20, y=10
+    println!("Transposed: x={}, y={}", tp.x, tp.y);  // 结果：x=20, y=10
 
-    // Using .into() — works automatically when From is implemented
+    // Using .into() — works automatically / 使用 .into() —— 在实现 From 后自动起效
     let p2 = Point { x: 3, y: 7 };
     let tp2: TransposePoint = p2.into();
-    println!("Transposed: x={}, y={}", tp2.x, tp2.y);  // x=7, y=3
+    println!("Transposed: x={}, y={}", tp2.x, tp2.y);  // 结果：x=7, y=3
 }
- // Output:
+ // Output / 输出：
 // Transposed: x=20, y=10
 // Transposed: x=7, y=3
 ```
 
 </details>
 
- # Rust Default trait
+ # Rust Default trait / Rust Default Trait
- - ```Default``` can be used to implement default values for a type
+ - ```Default``` 可用于为类型实现默认值。
-     - Types can use the ```Derive``` macro with ```Default``` or provide a custom implementation
+     - 类型可以使用 ```Derive``` 宏自动派生 ```Default```，也可以提供自定义实现。
 ```rust
 #[derive(Default, Debug)]
 struct Point {x: u32, y: u32}
 #[derive(Debug)]
 struct CustomPoint {x: u32, y: u32}
 impl Default for CustomPoint {
     fn default() -> Self {
         CustomPoint {x: 42, y: 42}
     }
 }
 fn main() {
-    let x = Point::default();   // Creates a Point{0, 0}
+    let x = Point::default();   // Creates / 创建一个 Point{0, 0}
     println!("{x:?}");
     let y = CustomPoint::default();
     println!("{y:?}");
 }
 ```
 
- ### Rust Default trait
+ ### Rust Default trait continued / Rust Default Trait（续）
- - ```Default``` trait has several use cases including
+ - ```Default``` trait 具有多种用例，包括：
-     - Performing a partial copy and using default initialization for rest
+     - 执行部分复制，并对其余部分使用默认初始化。
-     - Default alternative for ```Option``` types in methods like ```unwrap_or_default()```
+     - 作为 ```Option``` 类型的默认备选项，例如在 ```unwrap_or_default()``` 方法中。
 ```rust
 #[derive(Debug)]
 struct CustomPoint {x: u32, y: u32}
 impl Default for CustomPoint {
     fn default() -> Self {
         CustomPoint {x: 42, y: 42}
     }
 }
 fn main() {
     let x = CustomPoint::default();
-    // Override y, but leave rest of elements as the default
+    // Override y, but leave rest default / 覆盖 y，但其余元素保留为默认值
     let y = CustomPoint {y: 43, ..CustomPoint::default()};
     println!("{x:?} {y:?}");
     let z : Option<CustomPoint> = None;
-    // Try changing the unwrap_or_default() to unwrap()
+    // Try changing unwrap_or_default() to unwrap() / 尝试将 unwrap_or_default() 更改为 unwrap()
     println!("{:?}", z.unwrap_or_default());
 }
 ```
 
- ### Other Rust type conversions
+ ### Other Rust type conversions / 其他 Rust 类型转换
- - Rust doesn't support implicit type conversions and ```as``` can be used for ```explicit``` conversions
+ - Rust 不支持隐式类型转换，可以使用 ```as``` 进行**显式**转换。
- - ```as``` should be sparingly used because it's subject to loss of data by narrowing and so forth. In general, it's preferable to use ```into()``` or ```from()``` where possible
+ - 应当谨慎使用 ```as```，因为它可能会导致由于范围收窄（narrowing）等原因产生的数据丢失。通常情况下，尽可能使用 ```into()``` 或 ```from()``` 更好。
 ```rust
 fn main() {
     let f = 42u8;
-    // let g : u32 = f;    // Will not compile
+    // let g : u32 = f;    // Will not compile / 无法编译
-    let g = f as u32;      // Ok, but not preferred. Subject to rules around narrowing
+    let g = f as u32;      // Ok, but not preferred / 可以使用但不推荐。受收窄规则限制
-    let g : u32 = f.into(); // Most preferred form; infallible and checked by the compiler
+    let g : u32 = f.into(); // Most preferred / 最推荐的形式；无误且由编译器检查
-    //let k : u8 = f.into();  // Fails to compile; narrowing can result in loss of data
+    //let k : u8 = f.into();  // Fails to compile / 编译失败；收窄可能导致数据丢失
     
-    // Attempting a narrowing operation requires use of try_into
+    // Narrowing requires try_into / 尝试收窄操作需要使用 try_into
     if let Ok(k) = TryInto::<u8>::try_into(g) {
         println!("{k}");
     }
 }
 ```
