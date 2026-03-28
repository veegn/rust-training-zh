# 10. 特征与泛型 🟡

> **你将学到：**
> - 特征 (Traits)：Rust 版本的显式“鸭子类型”
> - `Protocol` (PEP 544) 与 Rust Traits 的概念对比
> - 泛型约束 (T: Trait)
> - 标准库常用特征清单 (Display, Debug, Clone 等)

## 特征 vs 鸭子类型

Python 遵循 **鸭子类型 (Duck Typing)**：“如果它走起路来像鸭子，那它就是鸭子。”而 Rust 遵循的是 **特征契约 (Trait Contracts)**：“我会明确告诉你我需要什么样的行为，编译器会帮我核实。”

### Python: 隐式的鸭子类型
```python
def total_area(shapes):
    return sum(s.area() for s in shapes)

# 如果传入的对象没有 .area() 方法，程序会在【运行时】崩溃
```

### Rust: 显式的特征契约
```rust
trait HasArea {
    fn area(&self) -> f64;
}

// 编译器会确保只有实现了 HasArea 契约的类型才能传入！
fn total_area(shapes: &[&dyn HasArea]) -> f64 {
    shapes.iter().map(|s| s.area()).sum()
}
```

---

## 标准库常用特征

这些特征相当于 Rust 版的 **“魔术方法” (dunder methods)**，如 `__str__`、`__repr__` 等。

| Rust 特征 | Python 魔术方法 | 用途 |
|------------|--------------|---------|
| `Display` | `__str__` | 人类理解的字符串 (用户看) |
| `Debug` | `__repr__` | 开发者使用的调试串 (`{:?}`) |
| `Clone` | `copy.deepcopy` | 显式深拷贝 |
| `PartialEq` | `__eq__` | 等值比较 (`==`) |
| `Add` | `__add__` | 运算符重载 (`+`) |
| `Iterator` | `__iter__` / `__next__` | 可迭代对象 (循环) |

### 派生特征 (Deriving)
在 Rust 中，你通常不需要手动编写这些方法，只需要加上派生标签即可：
```rust
#[derive(Debug, PartialEq, Clone)]
struct User {
    id: i32,
    username: String,
}
```

---

## 泛型约束

泛型允许你编写适用于多种类型 `T` 的代码，只要 `T` 遵循特定的约束规则。

```rust
// T 可以是任何类型，只要它实现了 Display 特征
fn print_it<T: std::fmt::Display>(item: T) {
    println!("它的值是: {item}");
}

// 多重约束：T 必须同时实现 Display 和 Debug
fn verbose_print<T>(item: T) 
where T: std::fmt::Display + std::fmt::Debug 
{
    println!("{item} (调试模式: {item:?})");
}
```

---

## 练习

<details>
<summary><strong>🏋️ 练习：摘要特征</strong> (点击展开)</summary>

**挑战**：定义一个名为 `Summary` 的特征，其中包含一个 `summarize(&self) -> String` 方法。分别为 `NewsArticle { headline: String }` 和 `Tweet { content: String }` 结构体实现它。最后写一个接收 `&impl Summary` 参数的 `notify` 函数并调用。

<details>
<summary>参考答案</summary>

```rust
trait Summary {
    fn summarize(&self) -> String;
}

struct NewsArticle { headline: String }
struct Tweet { content: String }

impl Summary for NewsArticle {
    fn summarize(&self) -> String { format!("头条新闻: {}", self.headline) }
}

impl Summary for Tweet {
    fn summarize(&self) -> String { format!("推文内容: {}", self.content) }
}

fn notify(item: &impl Summary) {
    println!("收到简报: {}", item.summarize());
}

fn main() {
    let t = Tweet { content: "Rust 真不错".to_string() };
    notify(&t);
}
```
</details>
</details>

***
