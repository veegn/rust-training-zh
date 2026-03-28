[English Original](../en/ch19-macros.md)

# 19. 宏 (Macros) 🟢

Rust 中的宏是一项强大的特性，允许你编写“生成其他代码的代码”，也就是所谓的元编程 (Metaprogramming)。Rust 有两种主要的宏类型：**声明式宏**（由 `macro_rules!` 定义）和 **过程式宏**。

### 1. 使用 `macro_rules!` 的声明式宏
声明式宏是 Rust 中最常见的宏类型。它们使用模式匹配将一段代码转换成另一段代码。

```rust
macro_rules! say_hello {
    () => {
        println!("你好！");
    };
}

fn main() {
    say_hello!(); // 将在编译时被替换为 println!("你好！")
}
```

---

### 2. 带参数的宏
你可以定义带参数的宏，从而创建更灵活和可重用的模式。

```rust
macro_rules! create_vec {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

fn main() {
    let v = create_vec![1, 2, 3];
    println!("{:?}", v);
}
```

---

### 3. 过程式宏 (Procedural Macros)
过程式宏更加复杂且行为更像函数。它们将一段 Token 流作为输入，对流进行操作，并产生一段 Token 流作为输出。过程式宏有三种类型：
- **自定义 `#[derive]` 宏**：指定通过 `derive` 属性添加的代码。
- **属性式 (Attribute-like) 宏**：定义可在任何项上使用的自定义属性。
- **函数式 (Function-like) 宏**：看起来像函数调用，但操作的是 Token 而不是值。

---

### 4. 宏 vs. 函数
- 对于不需要操作代码结构本身的逻辑，请使用 **函数**。
- 对于函数无法执行的任务，请使用 **宏**，例如：
    - 创建变长参数接口（如 `println!`）。
    - 减少样板代码 (Boilerplate)（如 `#[derive(Debug)]`）。
    - 定义领域特定语言 (DSL)。

---

### 对于 C/C++ 开发者的总结
- **在 C/C++ 中**：宏由预处理器处理，本质上是简单的文本替换。这会导致许多细微的 Bug（例如，宏定义中遗漏了括号）。
- **在 Rust 中**：宏是编译器的一部分，并作用于抽象语法树 (AST) 或 Token 流。这使得它们比 C/C++ 预处理器宏更安全、更强大。Rust 宏是“卫生的 (Hygienic)”，这意味着它们不会意外地从调用它们的作用域中捕获变量。

***
