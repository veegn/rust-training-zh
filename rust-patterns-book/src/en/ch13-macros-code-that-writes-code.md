# 13. Macros — Code That Writes Code 🟡

> **What you'll learn:**
> - Declarative macros (`macro_rules!`) with pattern matching.
> - When to use macros vs generics.
> - Procedural macros: Derive, Attribute, and Function-like.
> - Writing a custom derive macro with `syn` and `quote`.

## Declarative Macros (macro_rules!)

Macros match syntax patterns and expand to code at compile time.

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

### Fragment Types

| Fragment | Matches | Example |
|----------|---------|---------|
| `$x:ident` | Identifier | `my_var`, `Point` |
| `$x:expr` | Expression | `a + b`, `42` |
| `$x:ty` | Type | `i32`, `Vec<u8>` |
| `$x:tt` | Token tree | Anything (most flexible) |

---

## When to Use Macros

- **YES**: Reducing boilerplate that traits/generics can't handle (e.g., variadic arguments).
- **YES**: DSLs like `html!` or `sql!`.
- **NO**: When a normal function or generic would suffice. Macros are harder to debug and don't benefit from standard IDE autocomplete.

---

## Procedural Macros

Procedural macros are Rust functions that take a `TokenStream` and return a `TokenStream`.

1. **Derive Macros**: `#[derive(MyTrait)]` generates code based on struct/enum structure.
2. **Attribute Macros**: `#[my_attr]` transforms the item it is attached to.
3. **Function-like Macros**: `my_macro!(...)` custom syntax handling.

### syn and quote

- **`syn`**: Parses Rust source code into an AST (Abstract Syntax Tree).
- **`quote`**: Turns Rust-like templates back into tokens.

```rust
// Goal: Generate an impl from a struct definition
let input = parse_macro_input!(input as DeriveInput);
let name = &input.ident;

let expanded = quote! {
    impl MyTrait for #name {
        fn hello() { println!("Hello from #name"); }
    }
};
```

---

## Hygiene and $crate

- **Hygiene**: Macros ensure that local variables defined inside the macro don't accidentally collide with the caller's variables.
- **`$crate`**: In library macros, always use `$crate::path` to ensure the macro works even if the user renames your crate.

***
