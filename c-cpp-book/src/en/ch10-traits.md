# 10. Traits 🟢

A **trait** defines functionality a particular type has and can share with other types. We can use traits to define shared behavior in an abstract way.

### 1. Defining a Trait
A trait definition is a way to group method signatures together to define a set of behaviors necessary to accomplish some purpose.

```rust
pub trait Summary {
    fn summarize(&self) -> String;
}
```

---

### 2. Implementing a Trait on a Type
Each type that implements the trait must provide its own custom behavior for the body of the methods.

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

### 3. Default Implementations
Sometimes it’s useful to have default behavior for some or all of the methods in a trait.

```rust
pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}
```

---

### 4. Traits as Parameters
We can use traits to define functions that accept many different types.

```rust
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
```

---

### 5. Returning Types that Implement Traits
We can also use the `impl Trait` syntax in the return position to return a value of some type that implements a trait.

```rust
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}
```

---

### Summary for C/C++ Developers
- **In C++**: You use abstract base classes with virtual functions for polymorphism.
- **In Rust**: You use **traits**. There is no class inheritance. Traits allow you to add behavior to types even after they are defined (as long as either the trait or the type is local to your crate). This is much more flexible than C++ interfaces.

***
