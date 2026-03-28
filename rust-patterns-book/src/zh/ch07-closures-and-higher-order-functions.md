[English Original](../en/ch07-closures-and-higher-order-functions.md)

# 7. 闭包与高阶函数 🟢

> **你将学到：**
> - 三种闭包 Trait (`Fn`, `FnMut`, `FnOnce`) 以及捕获的工作原理
> - 将闭包作为参数传递，并从函数中返回闭包
> - 组合器链与迭代器适配器 (`map`, `filter`, `fold`)
> - `with` 模式用于括号式资源访问

## Fn, FnMut, FnOnce — 闭包 Trait

Rust 中的每个闭包都会根据它捕获变量的方式，实现三个 Trait 中的一个或多个：

```rust
// FnOnce — 消耗所捕获的值（只能被调用一次）
let name = String::from("Alice");
let greet = move || { drop(name); };

// FnMut — 以可变方式借用捕获的值（可以被调用多次）
let mut count = 0;
let mut inc = || { count += 1; };

// Fn — 以不可变方式借用捕获的值（可以并行被多次调用）
let prefix = "ID";
let display = |x| println!("{prefix}: {x}");
```

**等阶体系**：`Fn` : `FnMut` : `FnOnce`。每个 `Fn` 都实现了 `FnMut`，而每个 `FnMut` 都实现了 `FnOnce`。

> **API 设计建议**：默认情况下请将 `FnMut` 作为参数的 Trait Bound —— 它对调用者来说最具灵活性。

---

## 组合器链

高阶函数在迭代器中大放异彩。它们是延迟计算（Lazy）的，并由 LLVM 优化为与传统循环一样高效。

```rust
let result: Vec<i32> = data.iter()
    .filter(|&&x| x % 2 == 0) // 保留偶数
    .map(|&x| x * x)         // 对其进行平方
    .collect();              // 收集到 Vec 中
```

---

## `with` 模式（括号式访问）

与其直接暴露资源并寄希望于用户在之后进行清理，不如 **通过闭包将其借出**。

```rust
impl GpioController {
    pub fn with_pin_output<R>(&self, pin: u8, mut f: impl FnMut(&GpioPin) -> R) -> R {
        self.set_direction(pin, Direction::Out); // 设置
        let result = f(&GpioPin { pin });         // 执行
        self.set_direction(pin, Direction::In);  // 清理
        result
    }
}

// 使用方式：
gpio.with_pin_output(4, |pin| {
    pin.write(true);
}); // 即使闭包发生恐慌（Panic），引脚方向也会被自动恢复
```

当资源 **严禁逃逸** 出操作作用域时，这种模式比传统的 RAII/Drop 更为安全。

***
