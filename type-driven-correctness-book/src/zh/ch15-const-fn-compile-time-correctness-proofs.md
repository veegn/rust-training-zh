# Const Fn — 编译期正确性证明 🟠

> **你将学到：** `const fn` 和 `assert!` 如何将编译器转化为一个证明引擎 —— 在编译期验证 SRAM 内存映射、寄存器布局、位域掩码 (bitfield masks) 以及查找表，且运行时开销为零。
>
> **相关章节：** [ch04](ch04-capability-tokens-zero-cost-proof-of-aut.md)（令牌）、[ch06](ch06-dimensional-analysis-making-the-compiler.md)（量纲分析）、[ch09](ch09-phantom-types-for-resource-tracking.md) (phantom types)

## 问题：虚假的内存映射

在嵌入式和系统编程中，内存映射是系统的基石。它们定义了引导程序、固件、数据段和栈的空间。一旦定义错误，子系统间就会发生静默的数据损坏。在 C 语言中，这些映射通常只是没有任何结构关系的宏定义，编译器无法察觉其间的重叠或越界。

## 将 `const fn` 作为证明引擎

Rust 的 `const fn` 可以在编译时执行逻辑。当 `const fn` 中的 `assert!` 失败时，它会触发一个 **编译错误**。这使得编译器能够化身为一个“定理证明器”，去验证你的系统不变量。

```rust
pub struct Region {
    pub base: u32,
    pub size: u32,
}

impl Region {
    pub const fn new(base: u32, size: u32) -> Self {
        assert!(size > 0, "size 必须非零");
        assert!(base as u64 + size as u64 <= u32::MAX as u64, "地址空间溢出");
        Self { base, size }
    }

    pub const fn overlaps(&self, other: &Region) -> bool {
        self.base < (other.base + other.size) && other.base < (self.base + self.size)
    }
}
```

## 已验证的内存映射

你可以组合多个区域，并静态证明它们互不重叠：

```rust
const SRAM: SramMap = SramMap::verified(
    Region::new(0x2000_0000, 256 * 1024), // 总 SRAM
    Region::new(0x2000_0000,  16 * 1024), // 引导程序
    Region::new(0x2000_4000, 128 * 1024), // 固件
    // ...
);
```

如果区域间发生了重叠，代码 **将无法通过编译**。原本会在数月后导致系统崩溃的 bug 现已在开发瞬间被捕获。

## 更多应用场景

- **寄存器映射**：证明寄存器是自然对齐且不相交的。
- **位域布局**：证明一个寄存器内的各比特位域没有重叠。
- **时钟树配置**：验证 PLL 倍频/分频参数链是否处于硬件限制范围内。
- **编译期查找表**：在编译期计算 CRC 或三角函数表，存放在 `.rodata` 中以实现零启动开销。

## 关键收获

1. **`const fn` + `assert!` = 证明**：每一个断言都是编译器必须证明为真的定理。
2. **零运行时成本**：所有的检查都在编译阶段完成，产生的常量直接硬编码到二进制文件中。
3. **快速失败 (Fail-fast)**：硬件约束违规变成了编译错误，而非无法复现的神秘故障。
4. **提升系统健壮性**：将对文档的依赖转化为对代码不变量的硬性约束。

***
