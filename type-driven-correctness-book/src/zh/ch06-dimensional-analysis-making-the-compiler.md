[English Original](../en/ch06-dimensional-analysis-making-the-compiler.md)

# 量纲分析：让编译器检查单位 🟢

> **你将学到什么？** 如何利用 Rust 的 `Newtype` 模式实现轻量级的量纲分析；如何利用类型系统防止物理单位混淆（如摄氏度 vs 华氏度、毫秒 vs 微秒，或者伏特 vs 毫伏）；以及为什么这相比于 `f64` 这种模糊的基础类型，能够将致命事故消除在编译期。

## 引言：单位混淆带来的巨大成本

在工程和科学计算中，最常见的 Bug 之一就是传错了单位。著名的 **火星气候轨道器 (Mars Climate Orbiter)** 事故就是因为一个模块使用了英制单位（磅·秒），而另一个模块使用了公制单位（牛顿·秒）。

传统的代码通常依赖参数命名来暗示单位：

```rust
fn adjust_power(voltage_v: f64) {
    // 如果传入了毫伏 (mV) 怎么办？
}
```

这种方案的问题在于：变量名是无关痛痒的注释；编译器只看到 `f64` 这种无语义的数字。

## Rust 方案：具有语义的 Newtype

我们可以利用 Newtype 模式为物理量创建独立的、具有语义的类型。

### 1. 定义单位类型

```rust
pub struct Volts(pub f64);
pub struct Millivolts(pub f64);
```

### 2. 强类型函数签名

```rust
fn adjust_power(voltage: Volts) {
    // 逻辑
}
```

现在，如果你尝试这样调用：

```rust
let mv = Millivolts(100.0);
// adjust_power(mv); // 错误：期待 Volts，发现 Millivolts
```

## 类型化的单位转换

你可以为这些类型实现 `From` 特征，或者添加显式的转换函数：

```rust
impl From<Millivolts> for Volts {
    fn from(mv: Millivolts) -> Self {
        Volts(mv.0 / 1000.0)
    }
}
```

## 为什么这种模式至关重要

1.  **无法混淆 (No Unit Confusion)**：物理单位的关联逻辑在编译期就被彻底锁死了。
2.  **强制显式转换 (Explicit Conversion)**：所有跨单位的操作都必须经过显式转换，不再存在隐式、难以追踪的数字转换 Bug。
3.  **零成本抽象 (Zero-Cost)**：在编译后生成的机器码中，所有的 Newtype 包装器都会消失，程序依然是高效地操作 `f64`。

在嵌入式编程或处理复杂的传感器套件时，这种模式可以有效防御各种关于电流、电压、压力和温度的常见逻辑错误。

***
