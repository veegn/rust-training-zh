[English Original](../en/ch02-1-essential-keywords-reference.md)

## C# 开发者必备的 Rust 关键字速查

> **你将学到：** Rust 关键字与 C# 等效项的代码级映射 —— 包括可见性修饰符、所有权关键字、控制流、类型定义以及模式匹配语法。
>
> **难度：** 🟢 初级

理解 Rust 的关键字及其用途，有助于 C# 开发者更有效地掌握这门语言。

### 可见性与访问控制关键字

#### C# 访问修饰符
```csharp
public class Example
{
    public int PublicField;           // 随处可访问
    private int privateField;        // 仅在当前类中访问
    protected int protectedField;    // 当前类及其子类访问
    internal int internalField;      // 当前程序集 (assembly) 内访问
    protected internal int protectedInternalField; // 组合访问
}
```

#### Rust 可见性关键字
```rust
// pub - 使项变为公开 (类似于 C# 的 public)
pub struct PublicStruct {
    pub public_field: i32,           // 公开字段
    private_field: i32,              // 默认私有 (无需关键字)
}

pub mod my_module {
    pub(crate) fn crate_public() {}     // 在当前 crate 内公开 (类似于 internal)
    pub(super) fn parent_public() {}    // 对父级模块公开
    pub(self) fn self_public() {}       // 在当前模块内公开 (等同于私有)
    
    pub use super::PublicStruct;        // 重新导出 (类似于 using 别名)
}

// Rust 中没有直接对应 C# protected 的关键字 - 建议使用组合 (composition) 代替继承
```

### 内存与所有权关键字

#### C# 内存相关关键字
```csharp
// ref - 按引用传递
public void Method(ref int value) { value = 10; }

// out - 输出参数
public bool TryParse(string input, out int result) { /* ... */ }

// in - 只读引用 (C# 7.2+)
public void ReadOnly(in LargeStruct data) { /* 无法修改 data */ }
```

#### Rust 所有权关键字
```rust
// & - 不可变引用 (类似于 C# 的 in 参数)
fn read_only(data: &Vec<i32>) {
    println!("Length: {}", data.len()); // 可读，不可修改
}

// &mut - 可变引用 (类似于 C# 的 ref 参数)
fn modify(data: &mut Vec<i32>) {
    data.push(42); // 可修改
}

// move - 强制闭包捕获变量的所有权
let data = vec![1, 2, 3];
let closure = move || {
    println!("{:?}", data); // data 被移动 (move) 到了闭包中
};
// data 在此处不再可用
}

// Box - 堆分配 (类似于 C# 对引用类型使用 new)
let boxed_data = Box::new(42); // 在堆上分配内存
```

### 控制流关键字

#### C# 控制流
```csharp
// return - 退出函数并返回值
public int GetValue() { return 42; }

// yield return - 迭代器模式
public IEnumerable<int> GetNumbers()
{
    yield return 1;
    yield return 2;
}

// break/continue - 循环控制
foreach (var item in items)
{
    if (item == null) continue;
    if (item.Stop) break;
}
```

#### Rust 控制流关键字
```rust
// return - 显式返回 (通常不需要)
fn get_value() -> i32 {
    return 42; // 显式返回
    // 或者直接写: 42 (隐式返回最后一行表达式的值)
}

// break/continue - 循环控制，可选返回值
fn find_value() -> Option<i32> {
    loop {
        let value = get_next();
        if value < 0 { continue; }
        if value > 100 { break None; }      // 退出并返回 None
        if value == 42 { break Some(value); } // 退出并返回成功值
    }
}

// loop - 无限循环 (类似于 while(true))
loop {
    if condition { break; }
}

// while - 条件循环
while condition {
    // 代码块
}

// for - 迭代器循环
for item in collection {
    // 代码块
}
```

### 类型定义关键字

#### C# 类型关键字
```csharp
// class - 引用类型
public class MyClass { }

// struct - 值类型
public struct MyStruct { }

// interface - 契约定义
public interface IMyInterface { }

// enum - 枚举
public enum MyEnum { Value1, Value2 }

// delegate - 函数指针
public delegate void MyDelegate(int value);
```

#### Rust 类型关键字
```rust
// struct - 数据结构 (类似于 C# 类和结构体的组合)
struct MyStruct {
    field: i32,
}

// enum - 代数数据类型 (比 C# 枚举强大得多)
enum MyEnum {
    Variant1,
    Variant2(i32),              // 可以持有数据
    Variant3 { x: i32, y: i32 }, // 结构体风格的变体
}

// trait - 接口定义 (类似于 C# 接口，但更强大)
trait MyTrait {
    fn method(&self);
    
    // 默认实现 (类似于 C# 8+ 的默认接口方法)
    fn default_method(&self) {
        println!("Default implementation");
    }
}

// type - 类型别名 (类似于 C# 的 using 别名)
type UserId = u32;
type Result<T> = std::result::Result<T, MyError>;

// impl - 实现块 (C# 没有直接等效项 - 方法通常在类内部定义)
impl MyStruct {
    // 类似于静态工厂方法
    fn new() -> MyStruct {
        MyStruct { field: 0 }
    }
}

impl MyTrait for MyStruct {
    fn method(&self) {
        println!("Implementation");
    }
}
```

### 函数定义关键字

#### C# 函数关键字
```csharp
// static - 类属性/方法
public static void StaticMethod() { }

// virtual - 可被重写
public virtual void VirtualMethod() { }

// override - 重写基类方法
public override void VirtualMethod() { }

// abstract - 必须被实现
public abstract void AbstractMethod();

// async - 异步方法
public async Task<int> AsyncMethod() { return await SomeTask(); }
```

#### Rust 函数关键字
```rust
// fn - 函数定义 (类似于 C# 方法，但可以独立存在)
fn regular_function() {
    println!("Hello");
}

// const fn - 编译时函数 (类似于 C# const，但用于函数)
const fn compile_time_function() -> i32 {
    42 // 可以在编译时进行求值
}

// async fn - 异步函数 (类似于 C# async)
async fn async_function() -> i32 {
    some_async_operation().await
}

// unsafe fn - 可能违反内存安全的函数
unsafe fn unsafe_function() {
    // 可以执行不安全的操作
}

// extern fn - 外部函数接口 (FFI)
extern "C" fn c_compatible_function() {
    // 可以被 C 语言调用
}
```

### 变量声明关键字

#### C# 变量关键字
```csharp
// var - 类型推断
var name = "John"; // 推断为 string

// const - 编译时常量
const int MaxSize = 100;

// readonly - 运行时常量 (仅用于字段，不用于局部变量)
// readonly DateTime createdAt = DateTime.Now;

// static - 类级别变量
static int instanceCount = 0;
```

#### Rust 变量关键字
```rust
// let - 变量绑定 (类似于 C# var)
let name = "John"; // 默认是不可变的 (immutable)

// let mut - 可变变量绑定
let mut count = 0; // 可以被修改
count += 1;

// const - 编译时常量 (类似于 C# const)
const MAX_SIZE: usize = 100;

// static - 全局变量 (类似于 C# static 字段)
static INSTANCE_COUNT: std::sync::atomic::AtomicUsize = 
    std::sync::atomic::AtomicUsize::new(0);
```

### 模式匹配关键字

#### C# 模式匹配 (C# 8+)
```csharp
// switch 表达式
string result = value switch
{
    1 => "One",
    2 => "Two",
    _ => "Other"
};

// is 模式
if (obj is string str)
{
    Console.WriteLine(str.Length);
}
```

#### Rust 模式匹配关键字
```rust
// match - 模式匹配 (类似于 C# switch，但强大得多)
let result = match value {
    1 => "One",
    2 => "Two",
    3..=10 => "Between 3 and 10", // 范围模式
    _ => "Other", // 通配符 (类似于 C# 的 _)
};

// if let - 条件模式匹配
if let Some(value) = optional {
    println!("Got value: {}", value);
}

// while let - 带有模式匹配的循环
while let Some(item) = iterator.next() {
    println!("Item: {}", item);
}

// 带有模式的 let - 解构
let (x, y) = point; // 解构元组
let Some(value) = optional else {
    return; // 如果模式不匹配，则尽早返回 (Early return)
};
```

### 内存安全关键字

#### C# 内存关键字
```csharp
// unsafe - 禁用安全检查
unsafe
{
    int* ptr = &variable;
    *ptr = 42;
}

// fixed - 固定托管内存 (防止 GC 移动对象)
unsafe
{
    fixed (byte* ptr = array)
    {
        // 使用指针 ptr
    }
}
```

#### Rust 安全关键字
```rust
// unsafe - 禁用借用检查器 (请谨慎使用！)
unsafe {
    let ptr = &variable as *const i32;
    let value = *ptr; // 解引用原始指针 (raw pointer)
}

// 原始指针类型 (C# 无直接等效项 - 通常不需要)
let ptr: *const i32 = &42;  // 不可变原始指针
let ptr: *mut i32 = &mut 42; // 可变原始指针
```

### C# 中没有的常见 Rust 关键字

```rust
// where - 泛型约束 (比 C# 的 where 更灵活)
fn generic_function<T>() 
where 
    T: Clone + Send + Sync,
{
    // T 必须实现 Clone, Send, 和 Sync Trait
}

// dyn - 动态 Trait 对象 (类似于 C# object，但是类型安全的)
let drawable: Box<dyn Draw> = Box::new(Circle::new());

// Self - 指代实现类型 (类似于 C# 的 this，但用于类型声明)
impl MyStruct {
    fn new() -> Self { // Self = MyStruct
        Self { field: 0 }
    }
}

// self - 方法接收者
impl MyStruct {
    fn method(&self) { }        // 不可变借用 (Immutable borrow)
    fn method_mut(&mut self) { } // 可变借用 (Mutable borrow)  
    fn consume(self) { }        // 获取所有权 (Take ownership)
}

// crate - 指代当前 crate 的根
use crate::models::User; // 从 crate 根开始的绝对路径

// super - 指代父级模块
use super::utils; // 从父级模块导入
```

### C# 开发者关键字总结

| 重点用途 | C# | Rust | 关键差异 |
|---------|----|----|----------------|
| 可见性 | `public`, `private`, `internal` | `pub`, 默认私有 | 使用 `pub(crate)` 等提供更细粒度控制 |
| 变量声明 | `var`, `readonly`, `const` | `let`, `let mut`, `const` | 变量默认是不可变的 |
| 函数定义 | `method()` | `fn` | 支持独立于类的全局函数 |
| 类型定义 | `class`, `struct`, `interface` | `struct`, `enum`, `trait` | Rust 枚举是功能强大的代数数据类型 |
| 泛型 | `<T> where T : IFoo` | `<T> where T: Foo` | 约束更灵活，支持多种组合 |
| 引用/参数 | `ref`, `out`, `in` | `&`, `&mut` | 编译期强制执行借用检查 |
| 模式匹配 | `switch`, `is` | `match`, `if let` | 在 Rust 中通常要求穷尽匹配 (Exhaustive matching) |

***
