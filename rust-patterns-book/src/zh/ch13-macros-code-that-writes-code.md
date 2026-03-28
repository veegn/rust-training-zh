[English Original](../en/ch13-macros-code-that-writes-code.md)

# 13. 宏：生成代码的代码 🟡

> **你将学到：**
> - 带有模式匹配的声明式宏（`macro_rules!`）。
> - 何时使用宏，以及何时使用泛型。
> - 过程宏：派生宏（Derive）、属性宏（Attribute）和函数式宏。
> - 使用 `syn` 和 `quote` 编写自定义派生宏。

## 声明式宏 (macro_rules!)

宏在编译时根据语法模式进行匹配并展开。

```rust
macro_rules! hashmap {
    ( $( $key:expr => $value:expr ),* $(,)? ) => {
        {
            let mut map = std::collections::HashMap::new();
            $( map.insert($key, $value); )*
            map
        }
    };
}

let m = hashmap! { "A" => 1, "B" => 2 };
```

### 片段类型 (Fragment Types)

| 片段 | 匹配内容 | 示例 |
|----------|---------|---------|
| `$x:ident` | 标识符 | `my_var`, `Point` |
| `$x:expr` | 表达式 | `a + b`, `42` |
| `$x:ty` | 类型 | `i32`, `Vec<u8>` |
| `$x:tt` | 标记树 | 任何内容（最灵活） |

---

## 何时使用宏

- **是**：当需要减少 trait/泛型无法处理的样板代码时（如变长参数）。
- **是**：如 `html!` 或 `sql!` 这样的 DSL。
- **否**：当普通函数或泛型足以完成任务时。宏更难调试，且无法享受标准 IDE 提供的高级自动补全。

---

## 过程宏 (Procedural Macros)

过程宏是接收 `TokenStream` 并返回 `TokenStream` 的 Rust 函数。

1. **派生宏 (Derive Macros)**：`#[derive(MyTrait)]`，根据结构体/枚举的结构生成代码。
2. **属性宏 (Attribute Macros)**：`#[my_attr]`，转换它所标记的项。
3. **函数式宏 (Function-like Macros)**：`my_macro!(...)`，自定义语法处理。

### syn 和 quote

- **`syn`**：将 Rust 源码解析为 AST（抽象语法树）。
- **`quote`**：将类 Rust 模板变回标记流。

```rust
// 目标：根据结构体定义生成一个实现 (impl)
let input = parse_macro_input!(input as DeriveInput);
let name = &input.ident;

let expanded = quote! {
    impl MyTrait for #name {
        fn hello() { println!("来自 #name 的问候"); }
    }
};
```

---

## 卫生性 (Hygiene) 与 $crate

- **卫生性**：宏确保其内部定义的局部变量不会意外地与调用者的变量发生冲突。
- **`$crate`**：在库宏中，务必使用 `$crate::path` 来引用自身，以确保即使在用户重命名了你的 crate 时，宏仍能正常工作。

***
