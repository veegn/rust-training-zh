# 7.2 智能指针与内部可变性 🟢

智能指针是某种数据结构，其行为类似于指针，但同时具有额外的原数据和功能（例如引用计数）。

### 1. 用于堆分配的 `Box<T>`
最直接的智能指针是 **Box**（盒子），它允许你将数据存储在堆 (Heap) 上而不是栈 (Stack) 上。

```rust
fn main() {
    let b = Box::new(5); // 5 存放在堆上
    println!("b = {b}");
} // b 在这里离开作用域，随后堆内存被释放。
```

在以下情况使用 `Box<T>`：
- 当你拥有一个在编译时无法确定其大小的类型（递归类型）。
- 当你想将大量数据的所有权转移而不进行拷贝。
- 当你想拥有一个实现特定 Trait 的值（Trait 对象）。

---

### 2. 用于共享所有权的 `Rc<T>`
在某些情况下，单个值可能具有多个所有者。`Rc<T>` (Reference Counted，引用计数型) 会跟踪指向该值的引用数量。

```rust
use std::rc::Rc;

fn main() {
    let a = Rc::new(String::from("hello"));
    let b = Rc::clone(&a); // 增加引用计数
    let c = Rc::clone(&a); // 再次增加引用计数

    println!("c 之后的计数：{}", Rc::strong_count(&a)); // 3
} // 所有引用均离开作用域，计数变为 0，内存随后被释放。
```

**注意**：`Rc<T>` 仅用于单线程场景。若在多线程中使用，请使用 `Arc<T>`。

---

### 3. 内部可变性：`Cell<T>` 与 `RefCell<T>`
有时即使你持有一个不可变引用，也需要修改其指向的值。这种模式被称作 **内部可变性 (Interior Mutability)**。

#### `Cell<T>`
适用于实现了 `Copy` 的类型。它允许你在不需要可变引用的情况下进行 get 和 set。
```rust
use std::cell::Cell;

struct User {
    id: u32,
    active: Cell<bool>,
}

let user = User { id: 1, active: Cell::new(true) };
user.active.set(false); // ✅ 即使 `user` 是不可变的，这也能正常工作
```

#### `RefCell<T>`
适用于任何类型。它将借用规则从编译时转移到了 **运行时 (Runtime)**。

```rust
use std::cell::RefCell;

let data = RefCell::new(vec![1, 2, 3]);

{
    let mut mutable_borrow = data.borrow_mut();
    mutable_borrow.push(4);
} // mutable_borrow 在这里离开作用域

println!("数据：{:?}", data.borrow());
```

**警告**：如果在运行时违反了借用规则（例如，在已有其他借用时调用 `borrow_mut()`），程序将会 **崩溃 (Panic)**。

---

### 4. 总结表

| 指针 | 用例 | 可变性 |
|---------|----------|------------|
| `Box<T>` | 堆上的单所有权 | 继承式（若变量为 `mut` 则可变） |
| `Rc<T>` | 多所有权（单线程） | 不可变 |
| `RefCell<T>` | 针对任何类型的内部可变性 | 可变（在运行时进行检查） |
| `Cell<T>` | 针对 `Copy` 类型的内部可变性 | 可变（通过 set/get） |

***
