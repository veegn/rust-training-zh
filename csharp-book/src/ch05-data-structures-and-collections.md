## Tuples and Destructuring / 元组与解构

> **What you'll learn / 你将学到：** Rust tuples vs C# `ValueTuple`, arrays and slices, structs vs classes, the newtype pattern for domain modeling with zero-cost type safety, and destructuring syntax.
>
> Rust 元组与 C# `ValueTuple` 的区别、数组与切片、结构体与类、如何用 newtype 模式为领域建模提供零成本类型安全，以及解构语法。
>
> **Difficulty / 难度：** 🟢 Beginner / 初级

C# has `ValueTuple` (since C# 7). Rust tuples are similar but more deeply integrated into the language.

C# 从 C# 7 起提供了 `ValueTuple`。Rust 元组在概念上类似，但它在语言中的集成程度更深。

### C# Tuples / C# 元组
```csharp
// C# ValueTuple (C# 7+)
var point = (10, 20);                         // (int, int)
var named = (X: 10, Y: 20);                   // Named elements
Console.WriteLine($"{named.X}, {named.Y}");

// Tuple as return type
public (int Quotient, int Remainder) Divide(int a, int b)
{
    return (a / b, a % b);
}

var (q, r) = Divide(10, 3);    // Deconstruction
Console.WriteLine($"{q} remainder {r}");

// Discards
var (_, remainder) = Divide(10, 3);  // Ignore quotient
```

### Rust Tuples / Rust 元组
```rust
// Rust tuples - immutable by default, no named elements
let point = (10, 20);                // (i32, i32)
let point3d: (f64, f64, f64) = (1.0, 2.0, 3.0);

// Access by index (0-based)
println!("x={}, y={}", point.0, point.1);

// Tuple as return type
fn divide(a: i32, b: i32) -> (i32, i32) {
    (a / b, a % b)
}

let (q, r) = divide(10, 3);       // Destructuring
println!("{q} remainder {r}");

// Discards with _
let (_, remainder) = divide(10, 3);

// Unit type () - the "empty tuple" (like C# void)
fn greet() {          // implicit return type is ()
    println!("hi");
}
```

### Key Differences / 关键差异

| Feature / 特性 | C# `ValueTuple` | Rust Tuple |
|---------|-----------------|------------|
| Named elements / 命名元素 | `(int X, int Y)` | Not supported - use structs / 不支持，需改用 struct |
| Max arity / 最大长度 | ~8 (nesting for more) / 约 8（更多时通过嵌套） | Unlimited (practical limit ~12) / 理论不限（实践中约 12 以内最常见） |
| Comparisons / 比较 | Automatic / 自动 | Automatic for tuples <= 12 elements / 对长度不超过 12 的元组通常自动支持 |
| Used as dict key / 作为字典键 | Yes / 可以 | Yes (if elements implement `Hash`) / 可以（前提是元素实现 `Hash`） |
| Return from functions / 作为返回值 | Common / 常见 | Common / 常见 |
| Mutable elements / 元素可变性 | Always mutable / 默认可变 | Only with `let mut` / 只有使用 `let mut` 才可变 |

### Tuple Structs (Newtypes) / 元组结构体（Newtype）
```rust
// When a plain tuple isn't descriptive enough, use a tuple struct:
struct Meters(f64);     // Single-field "newtype" wrapper
struct Celsius(f64);
struct Fahrenheit(f64);

// The compiler treats these as DIFFERENT types:
let distance = Meters(100.0);
let temp = Celsius(36.6);
// distance == temp;  // ERROR: can't compare Meters with Celsius

// Newtype pattern prevents unit-confusion bugs at compile time!
// In C# you'd need a full class/struct for the same safety.
```

```csharp
// C# equivalent requires more ceremony:
public readonly record struct Meters(double Value);
public readonly record struct Celsius(double Value);
// Not interchangeable, but records add overhead vs Rust's zero-cost newtypes
```

### The Newtype Pattern in Depth: Domain Modeling with Zero Cost / 深入理解 Newtype：零成本领域建模

Newtypes go far beyond preventing unit confusion. They're Rust's primary tool for **encoding business rules into the type system** - replacing the "guard clause" and "validation class" patterns common in C#.

Newtype 的作用远不止防止单位混淆。它是 Rust 中把**业务规则编码进类型系统**的核心工具，可以替代 C# 中常见的“守卫语句”和“验证类”模式。

#### C# Validation Approach: Runtime Guards / C# 的校验方式：运行时守卫
```csharp
// C# - validation happens at runtime, every time
public class UserService
{
    public User CreateUser(string email, int age)
    {
        if (string.IsNullOrWhiteSpace(email) || !email.Contains('@'))
            throw new ArgumentException("Invalid email");
        if (age < 0 || age > 150)
            throw new ArgumentException("Invalid age");

        return new User { Email = email, Age = age };
    }

    public void SendEmail(string email)
    {
        // Must re-validate - or trust the caller?
        if (!email.Contains('@')) throw new ArgumentException("Invalid email");
        // ...
    }
}
```

#### Rust Newtype Approach: Compile-Time Proof / Rust 的 Newtype 方式：编译期证明
```rust
/// A validated email address - the type itself IS the proof of validity.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Email(String);

impl Email {
    /// The ONLY way to create an Email - validation happens once at construction.
    pub fn new(raw: &str) -> Result<Self, &'static str> {
        if raw.contains('@') && raw.len() > 3 {
            Ok(Email(raw.to_lowercase()))
        } else {
            Err("invalid email format")
        }
    }

    /// Safe access to the inner value
    pub fn as_str(&self) -> &str { &self.0 }
}

/// A validated age - impossible to create an invalid one.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Age(u8);

impl Age {
    pub fn new(raw: u8) -> Result<Self, &'static str> {
        if raw <= 150 { Ok(Age(raw)) } else { Err("age out of range") }
    }
    pub fn value(&self) -> u8 { self.0 }
}

// Now functions take PROVEN types - no re-validation needed!
fn create_user(email: Email, age: Age) -> User {
    // email is GUARANTEED valid - it's a type invariant
    User { email, age }
}

fn send_email(to: &Email) {
    // No validation needed - Email type proves validity
    println!("Sending to: {}", to.as_str());
}
```

#### Common Newtype Uses for C# Developers / 面向 C# 开发者的常见 Newtype 用法

| C# Pattern | Rust Newtype | What It Prevents / 防止什么问题 |
|------------|-------------|------------------|
| `string` for UserId, Email, etc. | `struct UserId(Uuid)` | Passing wrong string to wrong parameter / 把错误的字符串传给错误参数 |
| `int` for Port, Count, Index | `struct Port(u16)` | Port and Count are not interchangeable / 避免 Port 和 Count 被混用 |
| Guard clauses everywhere | Constructor validation once | Re-validation, missed validation / 重复校验、漏校验 |
| `decimal` for USD, EUR | `struct Usd(Decimal)` | Adding USD to EUR by accident / 防止误把美元和欧元相加 |
| `TimeSpan` for different semantics | `struct Timeout(Duration)` | Passing connection timeout as request timeout / 把连接超时误传成请求超时 |

```rust
// Zero-cost: newtypes compile to the same assembly as the inner type.
// This Rust code:
struct UserId(u64);
fn lookup(id: UserId) -> Option<User> { /* ... */ }

// Generates the SAME machine code as:
fn lookup(id: u64) -> Option<User> { /* ... */ }
// But with full type safety at compile time!
```

***

## Arrays and Slices / 数组与切片

Understanding the difference between arrays, slices, and vectors is crucial.

理解数组、切片与向量之间的区别，是掌握 Rust 集合模型的关键。

### C# Arrays / C# 数组
```csharp
// C# arrays
int[] numbers = new int[5];         // Fixed size, heap allocated
int[] initialized = { 1, 2, 3, 4, 5 }; // Array literal

// Access
numbers[0] = 10;
int first = numbers[0];

// Length
int length = numbers.Length;

// Array as parameter (reference type)
void ProcessArray(int[] array)
{
    array[0] = 99;  // Modifies original
}
```

### Rust Arrays, Slices, and Vectors / Rust 中的数组、切片与向量
```rust
// 1. Arrays - Fixed size, stack allocated
let numbers: [i32; 5] = [1, 2, 3, 4, 5];  // Type: [i32; 5]
let zeros = [0; 10];                       // 10 zeros

// Access
let first = numbers[0];
// numbers[0] = 10;  // Error: arrays are immutable by default

let mut mut_array = [1, 2, 3, 4, 5];
mut_array[0] = 10;  // Works with mut

// 2. Slices - Views into arrays or vectors
let slice: &[i32] = &numbers[1..4];  // Elements 1, 2, 3
let all_slice: &[i32] = &numbers;    // Entire array as slice

// 3. Vectors - Dynamic size, heap allocated (covered earlier)
let mut vec = vec![1, 2, 3, 4, 5];
vec.push(6);  // Can grow
```

### Slices as Function Parameters / 以切片作为函数参数
```csharp
// C# - Method that works with arrays
public void ProcessNumbers(int[] numbers)
{
    for (int i = 0; i < numbers.Length; i++)
    {
        Console.WriteLine(numbers[i]);
    }
}

// Works with arrays only
ProcessNumbers(new int[] { 1, 2, 3 });
```

```rust
// Rust - Function that works with any sequence
fn process_numbers(numbers: &[i32]) {  // Slice parameter
    for (i, num) in numbers.iter().enumerate() {
        println!("Index {}: {}", i, num);
    }
}

fn main() {
    let array = [1, 2, 3, 4, 5];
    let vec = vec![1, 2, 3, 4, 5];
    
    // Same function works with both!
    process_numbers(&array);      // Array as slice
    process_numbers(&vec);        // Vector as slice
    process_numbers(&vec[1..4]);  // Partial slice
}
```

### String Slices (&str) Revisited / 再看字符串切片 `&str`
```rust
// String and &str relationship
fn string_slice_example() {
    let owned = String::from("Hello, World!");
    let slice: &str = &owned[0..5];      // "Hello"
    let slice2: &str = &owned[7..];      // "World!"
    
    println!("{}", slice);   // "Hello"
    println!("{}", slice2);  // "World!"
    
    // Function that accepts any string type
    print_string("String literal");      // &str
    print_string(&owned);               // String as &str
    print_string(slice);                // &str slice
}

fn print_string(s: &str) {
    println!("{}", s);
}
```

***

## Structs vs Classes / 结构体与类

Structs in Rust are similar to classes in C#, but with some key differences around ownership and methods.

Rust 中的 struct 在用途上与 C# 类似，但在所有权与方法组织方式上有明显差异。

```mermaid
graph TD
    subgraph "C# Class (Heap)"
        CObj["Object Header<br/>+ vtable ptr"] --> CFields["Name: string ref<br/>Age: int<br/>Hobbies: List ref"]
        CFields --> CHeap1["Alice on heap"]
        CFields --> CHeap2["List<string> on heap"]
    end
    subgraph "Rust Struct (Stack)"
        RFields["name: String<br/>ptr | len | cap<br/>age: i32<br/>hobbies: Vec<br/>ptr | len | cap"]
        RFields --> RHeap1["Alice heap buffer"]
        RFields --> RHeap2["Vec heap buffer"]
    end

    style CObj fill:#bbdefb,color:#000
    style RFields fill:#c8e6c9,color:#000
```

> **Key insight / 核心洞见：** C# classes always live on the heap behind a reference. Rust structs live on the stack by default - only dynamically-sized data (like `String` contents) goes to the heap. This eliminates GC overhead for small, frequently-created objects.
>
> C# 的类实例总是通过引用存在于堆上。Rust 的 struct 默认直接存放在栈上，只有像 `String` 这类动态大小的数据才会把内容放到堆上。这让很多小而频繁创建的对象不再承担 GC 开销。

### C# Class Definition / C# 类定义
```csharp
// C# class with properties and methods
public class Person
{
    public string Name { get; set; }
    public int Age { get; set; }
    public List<string> Hobbies { get; set; }
    
    public Person(string name, int age)
    {
        Name = name;
        Age = age;
        Hobbies = new List<string>();
    }
    
    public void AddHobby(string hobby)
    {
        Hobbies.Add(hobby);
    }
    
    public string GetInfo()
    {
        return $"{Name} is {Age} years old";
    }
}
```

### Rust Struct Definition / Rust 结构体定义
```rust
// Rust struct with associated functions and methods
#[derive(Debug)]  // Automatically implement Debug trait
pub struct Person {
    pub name: String,    // Public field
    pub age: u32,        // Public field
    hobbies: Vec<String>, // Private field (no pub)
}

impl Person {
    // Associated function (like static method)
    pub fn new(name: String, age: u32) -> Person {
        Person {
            name,
            age,
            hobbies: Vec::new(),
        }
    }
    
    // Method (takes &self, &mut self, or self)
    pub fn add_hobby(&mut self, hobby: String) {
        self.hobbies.push(hobby);
    }
    
    // Method that borrows immutably
    pub fn get_info(&self) -> String {
        format!("{} is {} years old", self.name, self.age)
    }
    
    // Getter for private field
    pub fn hobbies(&self) -> &Vec<String> {
        &self.hobbies
    }
}
```

### Creating and Using Instances / 创建和使用实例
```csharp
// C# object creation and usage
var person = new Person("Alice", 30);
person.AddHobby("Reading");
person.AddHobby("Swimming");

Console.WriteLine(person.GetInfo());
Console.WriteLine($"Hobbies: {string.Join(", ", person.Hobbies)}");

// Modify properties directly
person.Age = 31;
```

```rust
// Rust struct creation and usage
let mut person = Person::new("Alice".to_string(), 30);
person.add_hobby("Reading".to_string());
person.add_hobby("Swimming".to_string());

println!("{}", person.get_info());
println!("Hobbies: {:?}", person.hobbies());

// Modify public fields directly
person.age = 31;

// Debug print the entire struct
println!("{:?}", person);
```

### Struct Initialization Patterns / 结构体初始化模式
```csharp
// C# object initialization
var person = new Person("Bob", 25)
{
    Hobbies = new List<string> { "Gaming", "Coding" }
};

// Anonymous types
var anonymous = new { Name = "Charlie", Age = 35 };
```

```rust
// Rust struct initialization
let person = Person {
    name: "Bob".to_string(),
    age: 25,
    hobbies: vec!["Gaming".to_string(), "Coding".to_string()],
};

// Struct update syntax (like object spread)
let older_person = Person {
    age: 26,
    ..person  // Use remaining fields from person (moves person!)
};

// Tuple structs (like anonymous types)
#[derive(Debug)]
struct Point(i32, i32);

let point = Point(10, 20);
println!("Point: ({}, {})", point.0, point.1);
```

***

## Methods and Associated Functions / 方法与关联函数

Understanding the difference between methods and associated functions is key.

理解方法与关联函数之间的差异，是掌握 Rust API 设计的关键之一。

### C# Method Types / C# 中的方法类型
```csharp
public class Calculator
{
    private int memory = 0;
    
    // Instance method
    public int Add(int a, int b)
    {
        return a + b;
    }
    
    // Instance method that uses state
    public void StoreInMemory(int value)
    {
        memory = value;
    }
    
    // Static method
    public static int Multiply(int a, int b)
    {
        return a * b;
    }
    
    // Static factory method
    public static Calculator CreateWithMemory(int initialMemory)
    {
        var calc = new Calculator();
        calc.memory = initialMemory;
        return calc;
    }
}
```

### Rust Method Types / Rust 中的方法类型
```rust
#[derive(Debug)]
pub struct Calculator {
    memory: i32,
}

impl Calculator {
    // Associated function (like static method) - no self parameter
    pub fn new() -> Calculator {
        Calculator { memory: 0 }
    }
    
    // Associated function with parameters
    pub fn with_memory(initial_memory: i32) -> Calculator {
        Calculator { memory: initial_memory }
    }
    
    // Method that borrows immutably (&self)
    pub fn add(&self, a: i32, b: i32) -> i32 {
        a + b
    }
    
    // Method that borrows mutably (&mut self)
    pub fn store_in_memory(&mut self, value: i32) {
        self.memory = value;
    }
    
    // Method that takes ownership (self)
    pub fn into_memory(self) -> i32 {
        self.memory  // Calculator is consumed
    }
    
    // Getter method
    pub fn memory(&self) -> i32 {
        self.memory
    }
}

fn main() {
    // Associated functions called with ::
    let mut calc = Calculator::new();
    let calc2 = Calculator::with_memory(42);
    
    // Methods called with .
    let result = calc.add(5, 3);
    calc.store_in_memory(result);
    
    println!("Memory: {}", calc.memory());
    
    // Consuming method
    let memory_value = calc.into_memory();  // calc is no longer usable
    println!("Final memory: {}", memory_value);
}
```

### Method Receiver Types Explained / 方法接收者类型说明
```rust
impl Person {
    // &self - Immutable borrow (most common)
    // Use when you only need to read the data
    pub fn get_name(&self) -> &str {
        &self.name
    }
    
    // &mut self - Mutable borrow
    // Use when you need to modify the data
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
    
    // self - Take ownership (less common)
    // Use when you want to consume the struct
    pub fn consume(self) -> String {
        self.name  // Person is moved, no longer accessible
    }
}

fn method_examples() {
    let mut person = Person::new("Alice".to_string(), 30);
    
    // Immutable borrow
    let name = person.get_name();  // person can still be used
    println!("Name: {}", name);
    
    // Mutable borrow
    person.set_name("Alice Smith".to_string());  // person can still be used
    
    // Taking ownership
    let final_name = person.consume();  // person is no longer usable
    println!("Final name: {}", final_name);
}
```

---

## Exercises / 练习

<details>
<summary><strong>Exercise: Slice Window Average / 练习：切片窗口平均值</strong> (click to expand / 点击展开)</summary>

**Challenge / 挑战：** Write a function that takes a slice of `f64` values and a window size, and returns a `Vec<f64>` of rolling averages. For example, `[1.0, 2.0, 3.0, 4.0, 5.0]` with window 3 -> `[2.0, 3.0, 4.0]`.

编写一个函数，接收 `f64` 切片和窗口大小，返回一个包含滚动平均值的 `Vec<f64>`。例如 `[1.0, 2.0, 3.0, 4.0, 5.0]` 配合窗口大小 3，应得到 `[2.0, 3.0, 4.0]`。

```rust
fn rolling_average(data: &[f64], window: usize) -> Vec<f64> {
    // Your implementation here
    todo!()
}

fn main() {
    let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    let avgs = rolling_average(&data, 3);
    println!("{avgs:?}"); // [2.0, 3.0, 4.0]
}
```

<details>
<summary>Solution / 参考答案</summary>

```rust
fn rolling_average(data: &[f64], window: usize) -> Vec<f64> {
    data.windows(window)
        .map(|w| w.iter().sum::<f64>() / w.len() as f64)
        .collect()
}

fn main() {
    let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    let avgs = rolling_average(&data, 3);
    assert_eq!(avgs, vec![2.0, 3.0, 4.0]);
    println!("{avgs:?}");
}
```

**Key takeaway / 关键要点：** Slices have powerful built-in methods like `.windows()`, `.chunks()`, and `.split()` that replace manual index arithmetic. In C#, you'd use `Enumerable.Range` or LINQ `.Skip().Take()`.

切片自带很多强大的方法，例如 `.windows()`、`.chunks()` 和 `.split()`，可以替代手写索引逻辑。在 C# 里，这类场景通常会用 `Enumerable.Range` 或 LINQ 的 `.Skip().Take()` 来表达。

</details>
</details>

<details>
<summary><strong>Exercise: Mini Address Book / 练习：迷你通讯录</strong> (click to expand / 点击展开)</summary>

Build a small address book using structs, enums, and methods:

使用 struct、enum 和方法构建一个小型通讯录：

1. Define an enum `PhoneType { Mobile, Home, Work }`  
   定义一个枚举 `PhoneType { Mobile, Home, Work }`
2. Define a struct `Contact` with `name: String` and `phones: Vec<(PhoneType, String)>`  
   定义一个结构体 `Contact`，包含 `name: String` 和 `phones: Vec<(PhoneType, String)>`
3. Implement `Contact::new(name: impl Into<String>) -> Self`  
   实现 `Contact::new(name: impl Into<String>) -> Self`
4. Implement `Contact::add_phone(&mut self, kind: PhoneType, number: impl Into<String>)`  
   实现 `Contact::add_phone(&mut self, kind: PhoneType, number: impl Into<String>)`
5. Implement `Contact::mobile_numbers(&self) -> Vec<&str>` that returns only mobile numbers  
   实现 `Contact::mobile_numbers(&self) -> Vec<&str>`，只返回手机号
6. In `main`, create a contact, add two phones, and print the mobile numbers  
   在 `main` 中创建一个联系人，添加两个电话号码，并打印手机号

<details>
<summary>Solution / 参考答案</summary>

```rust
#[derive(Debug, PartialEq)]
enum PhoneType { Mobile, Home, Work }

#[derive(Debug)]
struct Contact {
    name: String,
    phones: Vec<(PhoneType, String)>,
}

impl Contact {
    fn new(name: impl Into<String>) -> Self {
        Contact { name: name.into(), phones: Vec::new() }
    }

    fn add_phone(&mut self, kind: PhoneType, number: impl Into<String>) {
        self.phones.push((kind, number.into()));
    }

    fn mobile_numbers(&self) -> Vec<&str> {
        self.phones
            .iter()
            .filter(|(kind, _)| *kind == PhoneType::Mobile)
            .map(|(_, num)| num.as_str())
            .collect()
    }
}

fn main() {
    let mut alice = Contact::new("Alice");
    alice.add_phone(PhoneType::Mobile, "+1-555-0100");
    alice.add_phone(PhoneType::Work, "+1-555-0200");
    alice.add_phone(PhoneType::Mobile, "+1-555-0101");

    println!("{}'s mobile numbers: {:?}", alice.name, alice.mobile_numbers());
}
```

</details>
</details>

***
