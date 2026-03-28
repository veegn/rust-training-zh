# 能力混入 — 编译期硬件契约 🟡

> **你将学到：** 组件化 trait（硬件总线能力）如何与混入 (mixin) trait 以及 blanket impl 结合，从而在消除诊断代码重复的同时，保证每个硬件依赖都在编译期得到满足。
>
> **相关章节：** [ch04](ch04-capability-tokens-zero-cost-proof-of-aut.md)（能力令牌）、[ch09](ch09-phantom-types-for-resource-tracking.md) (phantom type)、[ch10](ch10-putting-it-all-together-a-complete-diagn.md)（集成）

## 问题：诊断代码重复

不同的硬件子系统通常需要类似的诊断工作流（例如读取传感器、检查阈值），但它们操作的总线不同（SPI, I2C, GPIO）。这通常会导致复制粘贴式代码。

## 组件化 Trait（硬件能力）

我们将硬件能力（总线、控制器）定义为带有关联类型的 trait：

```rust
pub trait HasSpi {
    type Spi: SpiBus;
    fn spi(&self) -> &Self::Spi;
}

pub trait HasI2c {
    type I2c: I2cBus;
    fn i2c(&self) -> &Self::I2c;
}
```

## 混入 Trait（诊断行为）

Mixin 模式为任何具备所需“组件”（硬件能力）的类型 **自动提供** 行为（通过 blanket implementation 实现）。

```rust
pub trait FanDiagMixin: HasSpi + HasI2c {
    fn run_fan_diagnostic(&self) -> bool {
        // 使用 self.spi() 和 self.i2c() 来进行诊断
        true
    }
}

// 任何同时具备 SPI 和 I2C 能力的类型都会自动获得此 mixin。
impl<T: HasSpi + HasI2c> FanDiagMixin for T {}
```

## 混合与匹配

具体的控制器只需列举它当前拥有的总线，它就会自动继承所有匹配的诊断行为：

```rust
pub struct BaseBoardController {
    spi: LinuxSpi,
    i2c: LinuxI2c,
    // ...
}

impl HasSpi for BaseBoardController { ... }
impl HasI2c for BaseBoardController { ... }

// BaseBoardController 现在自动具有了 FanDiagMixin！
```

## 何时使用能力混入

| 场景 | 建议 |
|----------|:------:|
| 通用的诊断行为 | ✅ 始终（防止代码冗余） |
| 多总线控制器 | ✅ 始终 |
| 跨平台测试 | ✅ 始终 |
| 简单的单总线设备 | ⚠️ 可选 |

## 关键收获

1. **组件化 trait 声明能力** —— 如 `HasSpi`, `HasI2c`。
2. **Mixin 通过 blanket impl 提供行为** —— `impl<T: HasSpi + HasI2c> FanDiagMixin for T {}`。
3. **编译期依赖检查** —— 如果移除某个总线，对应的 mixin 方法会在编译时消失。
4. **平台无关的诊断逻辑** —— 逻辑仅需编写一次，便能在提供所需总线的每个平台上复用。

***
