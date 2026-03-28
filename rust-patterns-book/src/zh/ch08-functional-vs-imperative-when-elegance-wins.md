[English Original](../en/ch08-functional-vs-imperative-when-elegance-wins.md)

# 8. 函数式与命令式：优雅何时胜出 🟡

> **你将学到：**
> - 核心原则：转换流水线 vs 状态管理。
> - `Option` 和 `Result` 组合器家族。
> - 迭代器链与 for 循环：决策框架。
> - 作用域内可变性：内部可变，外部不可变。

## 核心原则

- **函数式风格** 在通过流水线转换数据时大放异彩。
- **命令式风格** 在管理复杂的控制流和副作用时更胜一筹。

地道的 Rust 代码通常会结合这两者。

---

## Option 和 Result 组合器

与其使用嵌套的 `if let` 块，不如使用组合器：

| 使用... | 代替... |
|---|---|
| `opt.unwrap_or(default)` | `if let Some(x) = opt { x } else { default }` |
| `opt.map(f)` | `match opt { Some(x) => Some(f(x)), None => None }` |
| `opt.and_then(f)` | `match opt { Some(x) => f(x), None => None }` |
| `res.map_err(f)` | `match res { Ok(x) => Ok(x), Err(e) => Err(f(e)) }` |

---

## 迭代器 vs 循环

### 何时使用迭代器：
- 每个步骤都是简单的转换（`filter`, `map`）。
- 正在计算单个聚合值（`sum`, `fold`）。
- 流水线具有良好的可读性（少于 4-5 步）。

```rust
let results: Vec<_> = data.iter()
    .filter(|item| item.active)
    .map(|item| item.score)
    .collect();
```

### 何时使用循环：
- 需要同时构建多个输出。
- 具有复杂的副作用（如多分支日志记录/警报）。
- 需要实现具有提前退出的状态机。

---

## 作用域内可变性

你可以将可变性限制在构建阶段，以获得更好的安全性：

```rust
let samples = {
    let mut buf = Vec::new();
    // 包含循环、提前退出和可变操作的复杂逻辑
    buf.push(1);
    buf.push(2);
    buf
}; // buf 被移出并变成了不可变的 'samples'
```

这种模式确保了 `samples` 在函数稍后的部分不会被意外修改。

---

## 性能：零成本抽象

在 Rust 中，**迭代器链编译出的机器码与手写的循环是一样的。** 唯一的性能开销是如果你在流水线中间使用了不必要的 `.collect()` 调用。

***
