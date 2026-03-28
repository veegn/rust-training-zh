# 10.1 Generics 🟢

**Generics** are abstract stand-ins for concrete types or other properties. We can use generics to create definitions for items like function signatures or structs, which we can then use with many different concrete data types.

### 1. In Function Definitions
When defining a function that uses generics, we place the generics in the signature of the function where we would usually specify the data types of the parameters and return value.

```rust
fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}
```

---

### 2. In Struct Definitions
We can also define structs to use a generic type parameter in one or more fields using the `<>` syntax.

```rust
struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
}
```

---

### 3. In Method Definitions
We can implement methods on structs and enums and use generic types in their definitions too.

```rust
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}
```

---

### 4. Performance of Code Using Generics
Rust implements generics in such a way that your code doesn’t run any slower than it would with concrete types. Rust accomplishes this by performing **monomorphization** of the code that is using generics at compile time.

Monomorphization is the process of turning generic code into specific code by filling in the concrete types that are used when compiled.

---

### Summary for C/C++ Developers
- **In C++**: You use **Templates**. Templates are also monomorphized (instantiated), so the performance is identical.
- **In Rust**: Generics with **Trait Bounds** (like `<T: PartialOrd>`) are equivalent to **C++20 Concepts**. They provide much better error messages than traditional C++ templates because the requirements are checked at the declaration site, not just when the template is instantiated.

***
