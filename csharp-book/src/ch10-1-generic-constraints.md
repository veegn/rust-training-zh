## Generic Constraints: where vs trait bounds | 泛型约束：`where` 与 trait bound

> **What you'll learn:** Rust's trait bounds vs C#'s `where` constraints, the `where` clause syntax,
> conditional trait implementations, associated types, and higher-ranked trait bounds (HRTBs).
>
> **你将学到什么：** Rust 的 trait bound 与 C# `where` 约束的区别，`where` 子句语法，
> 条件 trait 实现、关联类型，以及高阶生命周期约束（HRTB）。
>
> **Difficulty:** Advanced
>
> **难度：** 高级

### C# Generic Constraints | C# 泛型约束
```csharp
// C# Generic constraints with where clause
public class Repository<T> where T : class, IEntity, new()
{
    public T Create()
    {
        return new T();  // new() constraint allows parameterless constructor
    }
    
    public void Save(T entity)
    {
        if (entity.Id == 0)  // IEntity constraint provides Id property
        {
            entity.Id = GenerateId();
        }
        // Save to database
    }
}

// Multiple type parameters with constraints
public class Converter<TInput, TOutput> 
    where TInput : IConvertible
    where TOutput : class, new()
{
    public TOutput Convert(TInput input)
    {
        var output = new TOutput();
        // Conversion logic using IConvertible
        return output;
    }
}

// Variance in generics
public interface IRepository<out T> where T : IEntity
{
    IEnumerable<T> GetAll();  // Covariant - can return more derived types
}

public interface IWriter<in T> where T : IEntity
{
    void Write(T entity);  // Contravariant - can accept more base types
}
```

### Rust Generic Constraints with Trait Bounds | Rust 中基于 Trait Bound 的泛型约束
```rust
use std::fmt::{Debug, Display};
use std::clone::Clone;

// Basic trait bounds
pub struct Repository<T> 
where 
    T: Clone + Debug + Default,
{
    items: Vec<T>,
}

impl<T> Repository<T> 
where 
    T: Clone + Debug + Default,
{
    pub fn new() -> Self {
        Repository { items: Vec::new() }
    }
    
    pub fn create(&self) -> T {
        T::default()  // Default trait provides default value
    }
    
    pub fn add(&mut self, item: T) {
        println!("Adding item: {:?}", item);  // Debug trait for printing
        self.items.push(item);
    }
    
    pub fn get_all(&self) -> Vec<T> {
        self.items.clone()  // Clone trait for duplication
    }
}

// Multiple trait bounds with different syntaxes
pub fn process_data<T, U>(input: T) -> U 
where 
    T: Display + Clone,
    U: From<T> + Debug,
{
    println!("Processing: {}", input);  // Display trait
    let cloned = input.clone();         // Clone trait
    let output = U::from(cloned);       // From trait for conversion
    println!("Result: {:?}", output);   // Debug trait
    output
}

// Associated types (similar to C# generic constraints)
pub trait Iterator {
    type Item;  // Associated type instead of generic parameter
    
    fn next(&mut self) -> Option<Self::Item>;
}

pub trait Collect<T> {
    fn collect<I: Iterator<Item = T>>(iter: I) -> Self;
}

// Higher-ranked trait bounds (advanced)
fn apply_to_all<F>(items: &[String], f: F) -> Vec<String>
where 
    F: for<'a> Fn(&'a str) -> String,  // Function works with any lifetime
{
    items.iter().map(|s| f(s)).collect()
}

// Conditional trait implementations
impl<T> PartialEq for Repository<T> 
where 
    T: PartialEq + Clone + Debug + Default,
{
    fn eq(&self, other: &Self) -> bool {
        self.items == other.items
    }
}
```

```mermaid
graph TD
    subgraph "C# Generic Constraints"
        CS_WHERE["where T : class, IInterface, new()"]
        CS_RUNTIME["[ERROR] Some runtime type checking<br/>Virtual method dispatch"]
        CS_VARIANCE["[OK] Covariance/Contravariance<br/>in/out keywords"]
        CS_REFLECTION["[ERROR] Runtime reflection possible<br/>typeof(T), is, as operators"]
        CS_BOXING["[ERROR] Value type boxing<br/>for interface constraints"]
        
        CS_WHERE --> CS_RUNTIME
        CS_WHERE --> CS_VARIANCE
        CS_WHERE --> CS_REFLECTION
        CS_WHERE --> CS_BOXING
    end
    
    subgraph "Rust Trait Bounds"
        RUST_WHERE["where T: Trait + Clone + Debug"]
        RUST_COMPILE["[OK] Compile-time resolution<br/>Monomorphization"]
        RUST_ZERO["[OK] Zero-cost abstractions<br/>No runtime overhead"]
        RUST_ASSOCIATED["[OK] Associated types<br/>More flexible than generics"]
        RUST_HKT["[OK] Higher-ranked trait bounds<br/>Advanced type relationships"]
        
        RUST_WHERE --> RUST_COMPILE
        RUST_WHERE --> RUST_ZERO
        RUST_WHERE --> RUST_ASSOCIATED
        RUST_WHERE --> RUST_HKT
    end
    
    subgraph "Flexibility Comparison"
        CS_FLEX["C# Flexibility<br/>[OK] Variance<br/>[OK] Runtime type info<br/>[ERROR] Performance cost"]
        RUST_FLEX["Rust Flexibility<br/>[OK] Zero cost<br/>[OK] Compile-time safety<br/>[ERROR] No variance (yet)"]
    end
    
    style CS_RUNTIME fill:#fff3e0,color:#000
    style CS_BOXING fill:#ffcdd2,color:#000
    style RUST_COMPILE fill:#c8e6c9,color:#000
    style RUST_ZERO fill:#c8e6c9,color:#000
    style CS_FLEX fill:#e3f2fd,color:#000
    style RUST_FLEX fill:#c8e6c9,color:#000
```

```text
Rust 的泛型约束核心思想是：把“某个类型必须具备哪些能力”写成 trait bound，而不是写成运行时反射或继承层级判断。
```

---

## Exercises | 练习

<details>
<summary><strong>Exercise: Generic Repository | 练习：泛型仓储</strong> (click to expand / 点击展开)</summary>

Translate this C# generic repository interface to Rust traits:

把下面这个 C# 泛型仓储接口翻译成 Rust trait：

```csharp
public interface IRepository<T> where T : IEntity, new()
{
    T GetById(int id);
    IEnumerable<T> Find(Func<T, bool> predicate);
    void Save(T entity);
}
```

Requirements:
1. Define an `Entity` trait with `fn id(&self) -> u64`
2. Define a `Repository<T>` trait where `T: Entity + Clone`
3. Implement a `InMemoryRepository<T>` that stores items in a `Vec<T>`
4. The `find` method should accept `impl Fn(&T) -> bool`

要求：
1. 定义一个 `Entity` trait，包含 `fn id(&self) -> u64`
2. 定义 `Repository<T>` trait，其中 `T: Entity + Clone`
3. 实现一个 `InMemoryRepository<T>`，内部用 `Vec<T>` 存储数据
4. `find` 方法应接收 `impl Fn(&T) -> bool`

<details>
<summary>Solution | 参考答案</summary>

```rust
trait Entity: Clone {
    fn id(&self) -> u64;
}

trait Repository<T: Entity> {
    fn get_by_id(&self, id: u64) -> Option<&T>;
    fn find(&self, predicate: impl Fn(&T) -> bool) -> Vec<&T>;
    fn save(&mut self, entity: T);
}

struct InMemoryRepository<T> {
    items: Vec<T>,
}

impl<T: Entity> InMemoryRepository<T> {
    fn new() -> Self { Self { items: Vec::new() } }
}

impl<T: Entity> Repository<T> for InMemoryRepository<T> {
    fn get_by_id(&self, id: u64) -> Option<&T> {
        self.items.iter().find(|item| item.id() == id)
    }
    fn find(&self, predicate: impl Fn(&T) -> bool) -> Vec<&T> {
        self.items.iter().filter(|item| predicate(item)).collect()
    }
    fn save(&mut self, entity: T) {
        if let Some(pos) = self.items.iter().position(|e| e.id() == entity.id()) {
            self.items[pos] = entity;
        } else {
            self.items.push(entity);
        }
    }
}

#[derive(Clone, Debug)]
struct User { user_id: u64, name: String }

impl Entity for User {
    fn id(&self) -> u64 { self.user_id }
}

fn main() {
    let mut repo = InMemoryRepository::new();
    repo.save(User { user_id: 1, name: "Alice".into() });
    repo.save(User { user_id: 2, name: "Bob".into() });

    let found = repo.find(|u| u.name.starts_with('A'));
    assert_eq!(found.len(), 1);
}
```

**Key differences from C#**: No `new()` constraint (use `Default` trait instead). `Fn(&T) -> bool` replaces `Func<T, bool>`. Return `Option` instead of throwing.

**与 C# 的关键差异：** Rust 没有直接对应的 `new()` 约束，通常改用 `Default` trait。`Fn(&T) -> bool` 对应 `Func<T, bool>`。查找失败时更常返回 `Option`，而不是抛异常。

</details>
</details>

***
