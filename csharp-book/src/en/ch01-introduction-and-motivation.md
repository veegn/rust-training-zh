# Introduction and Motivation

> **What you'll learn:** Why Rust matters for C# developers - the performance gap between managed and native code, how Rust eliminates null-reference exceptions and hidden control flow at compile time, and the key scenarios where Rust complements or replaces C#.
>
> **Difficulty:** 🟢 Beginner

---

## Speaker Intro and General Approach

*   **Principal Firmware Architect in Microsoft SCHIE team**
    *   Industry veteran with expertise in security, systems programming, and C++ systems.
*   **Started programming in Rust in 2017**
    *   Began at AWS EC2 and have been deeply involved with the language ever since.

*This course is intended to be as interactive as possible. We assume you know C# and .NET development, and examples deliberately map C# concepts to Rust equivalents. Please feel free to ask clarifying questions at any point of time.*

---

## The Case for Rust for C# Developers

### Performance Without the Runtime Tax

C# offers great productivity but comes with runtime overhead from the Garbage Collector (GC). Rust provides the same expressiveness with zero runtime overhead and deterministic performance.

```csharp
// C# - Great productivity, runtime overhead
public class DataProcessor
{
    private List<int> data = new List<int>();
    
    public void ProcessLargeDataset()
    {
        for (int i = 0; i < 10_000_000; i++)
        {
            data.Add(i * 2); // GC pressure
        }
    }
}
```

```rust
// Rust - Same expressiveness, zero runtime overhead
struct DataProcessor {
    data: Vec<i32>,
}

impl DataProcessor {
    fn process_large_dataset(&mut self) {
        for i in 0..10_000_000 {
            self.data.push(i * 2); // No GC pressure
        }
    }
}
```

### Memory Safety Without Runtime Checks

```csharp
// C# - Runtime safety with overhead
public class RuntimeCheckedOperations
{
    public string? ProcessArray(int[] array)
    {
        if (array.Length > 0)
        {
            return array[0].ToString(); 
        }
        return null;
    }
}
```

```rust
// Rust - Compile-time safety with zero runtime cost
struct SafeOperations;

impl SafeOperations {
    fn process_array(array: &[i32]) -> Option<String> {
        array.first().map(|x| x.to_string())
    }
}
```

---

## Common C# Pain Points That Rust Addresses

### 1. The Billion Dollar Mistake: Null References
In C#, null reference exceptions are runtime "bombs." Rust eliminates them entirely through the `Option<T>` type and exhaustive matching.

### 2. Hidden Exceptions and Control Flow
In C#, exceptions can be thrown from almost anywhere. In Rust, all potential errors are explicit in function signatures via `Result<T, E>`.

### 3. Correctness: The Type System as a Proof Engine
Rust's type system catches entire categories of logic bugs at compile time that C# can only catch at runtime.
- **ADTs vs Class Workarounds**
- **Immutability by Default**
- **Composition over Inheritance**

---

## When to Choose Rust Over C#

| **Scenario** | **Recommendation** | **Reason** |
| :--- | :--- | :--- |
| **Performance-Critical** | **Rust** | Zero GC, native performance |
| **High Correctness** | **Rust** | Type-system proofs |
| **Rapid Prototyping** | **C#** | Rich ecosystem, high-level abstractions |

> **Key Insight:** In C#, correctness is often a matter of *discipline*. In Rust, correctness is a *property of the code* enforced by the compiler.
