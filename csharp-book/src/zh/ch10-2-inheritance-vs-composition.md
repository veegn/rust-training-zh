[English Original](../en/ch10-2-inheritance-vs-composition.md)

## 继承 vs 组合 (Composition)

> **你将学到：** 为什么 Rust 没有类继承；特性 (Traits) + 结构体 (Structs) 是如何替代深层类层级的；以及如何通过组合实现多态的实践模式。
>
> **难度：** 🟡 中级

### C# - 基于类的继承
```csharp
// C# - 基于类的继承
public abstract class Animal
{
    public string Name { get; protected set; }
    public abstract void MakeSound();
    
    public virtual void Sleep()
    {
        Console.WriteLine($"{Name} is sleeping");
    }
}

public class Dog : Animal
{
    public Dog(string name) { Name = name; }
    
    public override void MakeSound()
    {
        Console.WriteLine("Woof!");
    }
    
    public void Fetch()
    {
        Console.WriteLine($"{Name} is fetching");
    }
}

// 基于接口的约束
public interface IFlyable
{
    void Fly();
}

public class Bird : Animal, IFlyable
{
    public Bird(string name) { Name = name; }
    
    public override void MakeSound()
    {
        Console.WriteLine("Tweet!");
    }
    
    public void Fly()
    {
        Console.WriteLine($"{Name} is flying");
    }
}
```

### Rust 组合模型
```rust
// Rust - 通过特性实现组合优于继承
pub trait Animal {
    fn name(&self) -> &str;
    fn make_sound(&self);
    
    // 默认实现 (类似于 C# 的虚方法)
    fn sleep(&self) {
        println!("{} is sleeping", self.name());
    }
}

pub trait Flyable {
    fn fly(&self);
}

// 将数据与行为分离
#[derive(Debug)]
pub struct Dog {
    name: String,
}

#[derive(Debug)]
pub struct Bird {
    name: String,
    wingspan: f64,
}

// 为类型实现行为
impl Animal for Dog {
    fn name(&self) -> &str {
        &self.name
    }
    
    fn make_sound(&self) {
        println!("Woof!");
    }
}

impl Dog {
    pub fn new(name: String) -> Self {
        Dog { name }
    }
    
    pub fn fetch(&self) {
        println!("{} is fetching", self.name);
    }
}

impl Animal for Bird {
    fn name(&self) -> &str {
        &self.name
    }
    
    fn make_sound(&self) {
        println!("Tweet!");
    }
}

impl Flyable for Bird {
    fn fly(&self) {
        println!("{} is flying with {:.1}m wingspan", self.name, self.wingspan);
    }
}

// 多重特性约束 (类似于实现多个接口)
fn make_flying_animal_sound<T>(animal: &T) 
where 
    T: Animal + Flyable,
{
    animal.make_sound();
    animal.fly();
}
```

```mermaid
graph TD
    subgraph "C# 继承层级"
        CS_ANIMAL["Animal (抽象类)"]
        CS_DOG["Dog : Animal"]
        CS_BIRD["Bird : Animal, IFlyable"]
        CS_VTABLE["虚方法分发<br/>运行时开销"]
        CS_COUPLING["[错误] 紧耦合<br/>[错误] 菱形继承问题<br/>[错误] 深层层级结构"]
        
        CS_ANIMAL --> CS_DOG
        CS_ANIMAL --> CS_BIRD
        CS_DOG --> CS_VTABLE
        CS_BIRD --> CS_VTABLE
        CS_ANIMAL --> CS_COUPLING
    end
    
    subgraph "Rust 组合模型"
        RUST_ANIMAL["trait Animal"]
        RUST_FLYABLE["trait Flyable"]
        RUST_DOG["struct Dog"]
        RUST_BIRD["struct Bird"]
        RUST_IMPL1["impl Animal for Dog"]
        RUST_IMPL2["impl Animal for Bird"]
        RUST_IMPL3["impl Flyable for Bird"]
        RUST_STATIC["静态分发<br/>零成本"]
        RUST_FLEXIBLE["[OK] 灵活的组合<br/>[OK] 无层级限制<br/>[OK] 自由混入特性"]
        
        RUST_DOG --> RUST_IMPL1
        RUST_BIRD --> RUST_IMPL2
        RUST_BIRD --> RUST_IMPL3
        RUST_IMPL1 --> RUST_ANIMAL
        RUST_IMPL2 --> RUST_ANIMAL
        RUST_IMPL3 --> RUST_FLYABLE
        RUST_IMPL1 --> RUST_STATIC
        RUST_IMPL2 --> RUST_STATIC
        RUST_IMPL3 --> RUST_STATIC
        RUST_ANIMAL --> RUST_FLEXIBLE
        RUST_FLYABLE --> RUST_FLEXIBLE
    end
    
    style CS_COUPLING fill:#ffcdd2,color:#000
    style RUST_FLEXIBLE fill:#c8e6c9,color:#000
    style CS_VTABLE fill:#fff3e0,color:#000
    style RUST_STATIC fill:#c8e6c9,color:#000
```

---

## 练习

<details>
<summary><strong>🏋️ 练习：使用特性替换继承</strong> (点击展开)</summary>

以下 C# 代码使用了继承。请使用特性组合在 Rust 中重写它：

```csharp
public abstract class Shape { public abstract double Area(); }
public abstract class Shape3D : Shape { public abstract double Volume(); }
public class Cylinder : Shape3D
{
    public double Radius { get; }
    public double Height { get; }
    public Cylinder(double r, double h) { Radius = r; Height = h; }
    public override double Area() => 2.0 * Math.PI * Radius * (Radius + Height);
    public override double Volume() => Math.PI * Radius * Radius * Height;
}
```

要求：
1. 定义 `HasArea` 特性，包含 `fn area(&self) -> f64`。
2. 定义 `HasVolume` 特性，包含 `fn volume(&self) -> f64`。
3. 实现 `Cylinder` 结构体并实现上述两个特性。
4. 编写一个函数 `fn print_shape_info(shape: &(impl HasArea + HasVolume))` —— 注意多重特性约束的用法（无需复杂的继承）。

<details>
<summary>🔑 参考答案</summary>

```rust
use std::f64::consts::PI;

trait HasArea {
    fn area(&self) -> f64;
}

trait HasVolume {
    fn volume(&self) -> f64;
}

struct Cylinder {
    radius: f64,
    height: f64,
}

impl HasArea for Cylinder {
    fn area(&self) -> f64 {
        2.0 * PI * self.radius * (self.radius + self.height)
    }
}

impl HasVolume for Cylinder {
    fn volume(&self) -> f64 {
        PI * self.radius * self.radius * self.height
    }
}

fn print_shape_info(shape: &(impl HasArea + HasVolume)) {
    println!("面积：   {:.2}", shape.area());
    println!("体积：   {:.2}", shape.volume());
}

fn main() {
    let c = Cylinder { radius: 3.0, height: 5.0 };
    print_shape_info(&c);
}
```

**关键洞察**：C# 需要一个三层结构（Shape → Shape3D → Cylinder）。Rust 使用扁平化的特性组合 —— `impl HasArea + HasVolume` 组合了各项能力，无需建立深层的继承关系。

</details>
</details>
