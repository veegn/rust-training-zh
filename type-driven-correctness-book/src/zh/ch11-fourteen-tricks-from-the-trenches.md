# 战壕里的十四个小技巧 🟡

> **你将学到：** 十四个较小的“构造即正确”技术 —— 从消除哨兵值、密封 trait 到会话类型、`Pin`、RAII 函数以及 `#[must_use]` —— 每一个技巧都能以极低的成本消除特定类别的 bug。
>
> **相关章节：** [ch02](ch02-typed-command-interfaces-request-determi.md)（密封 trait）、[ch05](ch05-protocol-state-machines-type-state-for-r.md)（类型化 builder）、[ch07](ch07-validated-boundaries-parse-dont-validate.md) (FromStr)

## 高价值技巧荟萃

虽然前八章的核心模式涵盖了主要的架构技术，但这十四个较小但在生产环境中频繁出现的技巧，同样能够有效地消除特定的 bug 类别。

### 1. 哨兵值 → 边界处的 `Option`
在最原始的解析边界处，将硬件协议中的哨兵值（如表示“不存在”的 `0xFF`）转换为 `Option::None`。这会强制所有下游消费者必须处理“值缺失”的情况。

### 2. 密封 Trait (Sealed Traits)
通过要求一个私有的超级 trait (supertrait)，防止外部 crate 实现你内部定义的 trait（如 `IpmiCmd`）。

```rust
mod private { pub trait Sealed {} }
pub trait IpmiCmd: private::Sealed { ... }
```

### 3. 用于演进中枚举的 `#[non_exhaustive]`
强制枚举的外部使用者在 `match` 语句中必须包含通配符 (`_`)。这可以确保当你的库增加新成员（如新的硬件 SKU）时，不会破坏下游的构建。

### 4. 类型化 Builder (Typestate Builder)
通过使用类型参数来跟踪状态，确保只有当所有必需字段都已设置后，Builder 的 `build()` 方法才变为可用。

### 5. 将 `FromStr` 作为验证边界
使用 `FromStr` 在数据进入系统的一刻就对字符串形式的输入（配置文件、命令行参数）进行验证。

### 6. 用于编译期大小验证的 Const Generic
在类型系统中对固定的硬件缓冲区大小（如 4096 字节的 NVMe 缓冲区）进行编码，防止误传大小错误的缓冲区。

### 7. 对 `unsafe` 的安全封装
将 `unsafe` 块限制在一个小而可审计的模块内部，并仅向应用程序的其他部分暴露安全的方法。

### 8. 异步 Type-State
将类型化状态机（type-state）扩展到 `async` 工作流中。确保状态切换方法消耗 `self`，以便在 `.await` 挂起点保持所有权。

### 10. 用于通道的会话类型 (Session Types)
在通道类型本身中对通信协议（请求 -> 响应 -> 完成）进行编码，从而防止消息顺序错误。

... 此外还包括 RAII、`#[must_use]`、自定义错误枚举等更多技巧。

## 关键收获

1. **小投入，大产出** —— 大多数技巧只需几行代码即可实现。
2. **定点消除 Bug 类目** —— 从“忘记检查哨兵值”到“使用了已关闭的会话”。
3. **渐进式改进** —— 可以在现有的代码库中根据需要随时采用这些技巧。

***
