# 1. Introduction and Motivation 🟢

### Course Approach
This course is designed to be interactive and practical. We assume you are already familiar with C, C++, or both, and our goal is to map those familiar concepts into Rust equivalents.

- **Interactive**: Feel free to ask questions as we go.
- **Relatable**: We use C/C++ analogies to explain Rust's unique features.
- **Hands-on**: Every chapter ends with exercises to build muscle memory.

---

### The Case for Rust
For C and C++ developers, the primary motivation for adopting Rust is **Safety without sacrificing Performance**.

- Over **70% of vulnerabilities** (CVEs) are memory safety issues (buffer overflows, use-after-free, etc.).
- While Modern C++ (smart pointers, RAII) improves safety, it remains a **"layer on top"**—it doesn't eliminate the fundamental risks of the language.
- Rust provides the same low-level control as C/C++ but moves safety checks from the **runtime to the compiler**.

---

### How Rust Addresses C/C++ Pain Points

#### 1. Buffer Overflows
Rust strings and arrays are bounds-checked. Any out-of-bounds access results in a predictable **panic** (runtime crash), never Undefined Behavior (UB).

#### 2. Dangling Pointers
Rust's **Ownership and Borrow Checker** ensures that references never outlive the data they point to, eliminating use-after-free and dangling pointers at compile time.

#### 3. Data Races
The `Send` and `Sync` traits allow the compiler to guarantee thread safety, making data races impossible in safe Rust.

#### 4. Memory Leaks & Resource Management
Rust's `Drop` trait implements RAII more strictly than C++. Combined with the ownership system, it prevents resource leaks and common "Rule of Five" complexities.

---

### Rust vs C/C++ Quick Comparison

| Feature | C / C++ | Rust |
|---------|---------|------|
| **Memory** | Manual or Smart Pointers | Ownership & Borrowing |
| **Safety** | Developer Responsibility | Compiler Guaranteed |
| **Error Handling** | Codes or Exceptions | `Result<T, E>` / `Option<T>` |
| **Threads** | Manual Sync | Safe Concurrency (`Send`/`Sync`) |
| **Build System** | Make / CMake | Cargo (Integrated) |

***
