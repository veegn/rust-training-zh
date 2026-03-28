# 用于资源跟踪的 Phantom Type 🟡

> **你将学到：** `PhantomData` 标记如何在类型层面上对寄存器宽度、DMA 方向和文件描述符状态进行编码 —— 从而在零运行时开销的情况下预防一整类资源不匹配的 bug。
>
> **相关章节：** [ch05](ch05-protocol-state-machines-type-state-for-r.md) (type-state)、[ch06](ch06-dimensional-analysis-making-the-compiler.md)（量纲类型）、[ch08](ch08-capability-mixins-compile-time-hardware-.md) (mixin)、[ch10](ch10-putting-it-all-together-a-complete-diagn.md)（集成）

## 问题：资源混淆

在代码中，不同的硬件资源看起来可能完全一样，但它们并不是互换的：

- 一个 32 位寄存器和一个 16 位寄存器在 C 语言中可能都是简单的偏移量。
- 一个用于读取的 DMA 缓冲区和一个用于写入的 DMA 缓冲区看起来都是 `*mut u8`。
- 一个打开的文件描述符和一个已经关闭的文件描述符在运行时都是 `i32`。

这种资源的不匹配会导致静默的 bug 或崩溃。

## Phantom Type 参数

**Phantom Type (幻象类型)** 是一种在 struct 定义中出现、但不在任何字段中出现的类型参数。它纯粹是为了携带类型层面的信息。

```rust
pub struct Width16;
pub struct Width32;

pub struct Register<W> {
    base: usize,
    offset: usize,
    _width: PhantomData<W>, // 运行时 0 字节
}

impl Register<Width16> {
    pub fn read(&self) -> u16 { ... }
}

impl Register<Width32> {
    pub fn read(&self) -> u32 { ... }
}
```

现在，编译器可以防止你将 16 位寄存器误读为 `u32`。

## DMA 缓冲区访问控制

DMA 缓冲区具有特定的方向（从设备读取 vs. 写入设备）。Phantom Type 可以强制执行这种方向性：

```rust
pub struct ToDevice;
pub struct FromDevice;

pub struct DmaBuffer<Dir> {
    ptr: *mut u8,
    _dir: PhantomData<Dir>,
}

impl DmaBuffer<ToDevice> {
    pub fn write_data(&mut self, data: &[u8]) { ... }
}
```

尝试对 `DmaBuffer<FromDevice>` 调用 `write_data` 会导致 **编译错误**。

## 何时使用 Phantom Type

| 场景 | 是否使用 Phantom 参数？ |
|----------|:------:|
| 寄存器宽度 | ✅ 始终 |
| DMA 方向 | ✅ 始终 |
| 文件描述符状态 | ✅ 始终 |
| 访问权限 (R/W/X) | ✅ 始终 |
| 运行时变化的属性 | ❌ 使用枚举 |

## 关键收获

1. **`PhantomData` 标记开销为零字节** —— 它们仅作为编译期的一种标签。
2. **结构化地预防资源不匹配** —— 寄存器宽度和 DMA 方向受类型系统严格保护。
3. **与其他模式的协同** —— 可以与 ch06 的量纲类型结合，提供更强大的保证。
4. **仅限编译期** —— 对于那些在运行时动态变化的属性，建议使用枚举而非 Phantom Type。

***
