# 量纲分析 — 让编译器检查你的单位 🟢

> **你将学到：** Newtype 包装器和 `uom` crate 如何将编译器变为一个单位检查引擎，从而预防曾摧毁价值 3.28 亿美元航天器的同类 bug。
>
> **相关章节：** [ch02](ch02-typed-command-interfaces-request-determi.md)（类型化命令）、[ch07](ch07-validated-boundaries-parse-dont-validate.md)（已验证边界）、[ch10](ch10-putting-it-all-together-a-complete-diagn.md)（集成）

## 火星气候探测者号 (Mars Climate Orbiter)

1999 年，NASA 的火星气候探测者号失踪了，原因是其中一个团队发送的推力数据单位是 **磅力秒**，而导航团队期望的单位是 **牛顿秒**。这两组数据在代码中都是 `double` 类型，编译器无法区分它们。最终，航天器进入大气层时的高度与预期相差巨大并解体。
损失：3.276 亿美元。

## 物理量的 Newtype

最简单的解决方案：**将每个单位包装在自己的类型中**。

```rust
pub struct Celsius(pub f64);
pub struct Fahrenheit(pub f64);
pub struct Volts(pub f64);
pub struct Rpm(pub f64);
```

现在，将 `Celsius` 与 `Volts` 进行对比会导致 **编译错误**：

```rust
fn check_limit(temp: Celsius, limit: Celsius) -> bool {
    temp > limit // ✅ 同一单位
}

// temp > voltage // ❌ 编译报错：类型不匹配
```

## 宏生成的量纲类型

使用宏可以消除为每个单位编写比较、算术和显示逻辑的冗余：

```rust
macro_rules! quantity {
    ($Name:ident, $unit:expr) => {
        #[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
        pub struct $Name(pub f64);
        // ... impl Display, Add, Sub ...
    };
}

quantity!(Celsius, "°C");
quantity!(Volts, "V");
```

## `uom` Crate

对于高级量纲分析（例如自动处理 `Watts = Volts × Amperes`），可以使用 [`uom`](https://crates.io/crates/uom) (Units of Measurement) crate。

```rust
// uom 在零运行时开销下处理复杂的衍生单位。
let power = voltage * current; // 自动类型检查
```

## 何时使用量纲类型

| 场景 | 建议 |
|----------|---------------|
| 传感器读数 | ✅ 始终（防止单位混淆） |
| 阈值比较 | ✅ 始终 |
| API 边界 | ✅ 始终 |
| 内部辅助函数 | ⚠️ 可选 |

## 关键收获

1. **Newtype 防止单位混淆** — `Celsius` 和 `Rpm` 是彼此独立的类型。
2. **零运行时开销** — Newtype 在编译后会被还原为其内部的原始值（例如 `f64`）。
3. **宏驱动的自动化** — 快速为各单位生成标准操作逻辑。
4. **`uom` 用于衍生单位** — 适用于涉及复杂物理计算的场景。

***
