[English Original](../en/ch10-2-inheritance-vs-composition.md)

# 继承 vs 组合

> **你将学到什么：** 为什么 Rust 没有类继承，trait + struct 如何取代深层类层级结构，以及如何通过组合实现多态。
>
> **难度：** 中级

C# 的基因里流淌着“类继承（Class Inheritance）”。然而，Rust 对此采取了完全不同的策略：完全放弃继承，拥抱 **组合 (Composition)** 与 **Trait**。

---

## 为什么不设计继承？
继承往往由于由于“脆弱的基类”问题而导致代码紧耦合。Rust 的模型更加简洁明了：
1.  **结构体 (Structs)**：负责承载数据。
2.  **Trait**：负责定义行为。
3.  **组合 (Composition)**：负责将它们联结在一起。

---

## C# 方式 (继承)
```csharp
public abstract class Animal {
    public string Name { get; set; }
    public abstract void MakeSound();
}

public class Dog : Animal {
    public override void MakeSound() => Console.WriteLine("汪汪!");
}
```

## Rust 方式 (组合)
```rust
trait Animal {
    fn make_sound(&self);
}

struct Dog {
    name: String, // 数据由结构体直接持有
}

impl Animal for Dog {
    fn make_sound(&self) {
        println!("汪汪!");
    }
}
```

---

## 没有继承，如何共享行为？
在 C# 中，你可能会把公共逻辑写在基类的 `virtual` 方法中。而在 Rust 中，你可以使用 **Trait 默认方法**。

```rust
trait Animal {
    fn name(&self) -> &str;
    
    // 共享的“虚函数”行为
    fn sleep(&self) {
        println!("{} 正在睡觉...", self.name());
    }
}
```

---

## 通过 Trait Bound 实现多态
你不再需要去判断一个对象是不是某个“子类”，而是去判断它是否具备所需的“Trait 能力”。

```rust
fn perform_action<T: Animal + Flyable>(creature: &T) {
    creature.make_sound();
    creature.fly();
}
```
**关键理解：** 这种“按需组合”的方式更具灵活性。你可以同时给 `Bird` 结构体赋予 `Animal` 和 `Flyable` 两个 trait，而不需要像继承那样去设计复杂的 `FlyingAnimal` 基类。

---

## C# 开发者总结表
| **C# (面向对象)** | **Rust (数据导向)** |
| :--- | :--- |
| **基类** | Trait (定义行为) |
| **基类里的字段** | 必须由具体的每一个结构体分别持有 |
| **虚函数 (Virtual)** | 带默认实现的 Trait 方法 |
| **重写 (Override)** | 对 Trait 方法的一个普通实现 |
| **抽象类** | 不包含默认实现的 Trait |

---

## 练习：用组合代替继承
**挑战：** 将一个 `Shape -> Shape3D` 的继承层级替换为两个独立的 trait：`HasArea` 和 `HasVolume`。并为圆柱体 `Cylinder` 实现它们。

```rust
trait HasArea { fn area(&self) -> f64; }
trait HasVolume { fn volume(&self) -> f64; }

struct Cylinder { radius: f64, height: f64 }

impl HasArea for Cylinder { ... }
impl HasVolume for Cylinder { ... }

fn check_3d_shape(shape: impl HasArea + HasVolume) {
    println!("面积为 {}, 体积为 {}", shape.area(), shape.volume());
}
```
**关键理解：** 通过将注意力集中在一个类型**能做什么** (Traits) 而非它**是什么** (Inheritance) 上，你可以编写出更加模块化、更易于测试的代码。
