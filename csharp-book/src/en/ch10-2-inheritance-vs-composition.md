# Inheritance vs Composition

> **What you'll learn:** Why Rust has no class inheritance, how traits + structs replace deep class hierarchies, and practical patterns for achieving polymorphism through composition.
>
> **Difficulty:** Intermediate

C# is built on class inheritance. Rust, however, completely avoids it in favor of **Composition** and **Traits**.

---

## Why No Inheritance?
Inheritance often leads to the "Fragile Base Class" problem and tight coupling. Rust's model is simpler:
1.  **Structs** hold the data.
2.  **Traits** define the behavior.
3.  **Composition** combines them.

---

## The C# Way (Inheritance)
```csharp
public abstract class Animal {
    public string Name { get; set; }
    public abstract void MakeSound();
}

public class Dog : Animal {
    public override void MakeSound() => Console.WriteLine("Woof!");
}
```

## The Rust Way (Composition)
```rust
trait Animal {
    fn make_sound(&self);
}

struct Dog {
    name: String, // Data is kept in the struct
}

impl Animal for Dog {
    fn make_sound(&self) {
        println!("Woof!");
    }
}
```

---

## Shared Behavior without Inheritance
In C#, you might put shared logic in a base class's `virtual` method. In Rust, you use **Default Trait Methods**.

```rust
trait Animal {
    fn name(&self) -> &str;
    
    // Shared "virtual" behavior
    fn sleep(&self) {
        println!("{} is sleeping...", self.name());
    }
}
```

---

## Polymorphism via Trait Bounds
Instead of checking if an object is a `SubClass`, you check if it has the required `Traits`.

```rust
fn perform_action<T: Animal + Flyable>(creature: &T) {
    creature.make_sound();
    creature.fly();
}
```
**Takeaway:** This "Mix-and-Match" approach is much more flexible. You can give a `Bird` both `Animal` and `Flyable` traits without needing a complex `FlyingAnimal` base class.

---

## Summary for C# Developers
| **C# (Object Oriented)** | **Rust (Data Oriented)** |
| :--- | :--- |
| **Base Class** | Trait (for behavior) |
| **Fields in Base Class** | Must be in each Struct |
| **Virtual Method** | Trait method with default impl |
| **Override** | Simple `impl` of trait method |
| **Abstract Class** | Trait with no default impls |

---

## Exercise: Composition over Inheritance
**Challenge:** Replace a `Shape -> Shape3D` hierarchy with two traits `HasArea` and `HasVolume`. Implement them for a `Cylinder`.

```rust
trait HasArea { fn area(&self) -> f64; }
trait HasVolume { fn volume(&self) -> f64; }

struct Cylinder { radius: f64, height: f64 }

impl HasArea for Cylinder { ... }
impl HasVolume for Cylinder { ... }

fn check_3d_shape(shape: impl HasArea + HasVolume) {
    println!("A: {}, V: {}", shape.area(), shape.volume());
}
```
**Takeaway:** By focusing on what a type **can do** (Traits) rather than what it **is** (Inheritance), you create more modular and testable code.
