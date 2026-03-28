[English Original](../en/ch10-traits.md)

# 10. Trait (特征) 🟢

**Trait** 定义了特定类型所拥有并可以与其他类型共享的功能。我们可以使用 Trait 以抽象的方式定义共同的行为。

### 1. 定义一个 Trait
Trait 的定义是将方法签名组合在一起，以定义完成某种目的所需的一系列行为。

```rust
pub trait Summary {
    fn summarize(&self) -> String;
}
```

---

### 2. 为类型实现 Trait
每个实现了该 Trait 的类型都必须为方法的正文提供其自定义行为。

```rust
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}
```

---

### 3. 默认实现
有时为 Trait 中的部分或全部方法提供默认行为会很有用。

```rust
pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(阅读更多...)")
    }
}
```

---

### 4. Trait 作为参数
我们可以使用 Trait 来定义那些接受多种不同类型的函数。

```rust
pub fn notify(item: &impl Summary) {
    println!("突发新闻！{}", item.summarize());
}
```

---

### 5. 返回实现了 Trait 的类型
我们还可以在返回值位置使用 `impl Trait` 语法，以返回实现了该 Trait 的某种类型的值。

```rust
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("当然，正如你可能已经知道的，人们情有独钟"),
        reply: false,
        retweet: false,
    }
}
```

---

### 对 C/C++ 开发者的总结
- **在 C++ 中**：你使用带有虚函数的抽象基类来实现多态。
- **在 Rust 中**：你使用 **Trait**。不存在类继承。Trait 允许你在类型定义之后再为其添加行为（只要 Trait 或类型中有一个是属于当前 Crate 的）。这比 C++ 接口要灵活得多。

***
