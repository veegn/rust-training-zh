# Const Fn â€?Compile-Time Correctness Proofs ðŸŸ 

> **What you'll learn:** How `const fn` and `assert!` turn the compiler into a proof engine â€?verifying SRAM memory maps, register layouts, bitfield masks, and lookup tables at compile time with zero runtime cost.
>
> **Cross-references:** [ch04](ch04-capability-tokens-zero-cost-proof-of-aut.md) (tokens), [ch06](ch06-dimensional-analysis-making-the-compiler.md) (units), [ch09](ch09-phantom-types-for-resource-tracking.md) (phantom types)

## The Problem: Memory Map Corruption

In systems programming, memory maps (RAM/Flash layouts) are often defined as loose constants. A misalignment or an overlap of two regions can lead to silent data corruption or stack overflows.

## Const Fn as a Proof Engine

Rust's `const fn` can evaluate logic at compile time. When combined with `assert!`, it forces the compiler to verify invariants before the program even runs.

```rust
pub struct Region {
    pub base: u32,
    pub size: u32,
}

impl Region {
    pub const fn new(base: u32, size: u32) -> Self {
        assert!(size > 0, "size must be non-zero");
        assert!(base as u64 + size as u64 <= u32::MAX as u64, "overflow");
        Self { base, size }
    }

    pub const fn overlaps(&self, other: &Region) -> bool {
        self.base < (other.base + other.size) && other.base < (self.base + self.size)
    }
}
```

## Verified Memory Maps

Compose multiple regions and prove they don't overlap:

```rust
const SRAM: SramMap = SramMap::verified(
    Region::new(0x2000_0000, 256 * 1024), // Total
    Region::new(0x2000_0000,  16 * 1024), // Bootloader
    Region::new(0x2000_4000, 128 * 1024), // Firmware
    // ...
);
```

If the regions overlap, the code **fails to compile**. The bug is caught instantly by the developer.

## Beyond Memory Maps

- **Register Maps**: Prove registers are aligned and disjoint.
- **Bitfield Layouts**: Prove bits within a register don't overlap.
- **Clock Trees**: Verify PLL multipliers/dividers stay within hardware limits.
- **Lookup Tables**: Compute CRC tables or trig tables at compile time.

## Key Takeaways

1. **`const fn` + `assert!` = Proof**. Each assertion is a theorem the compiler must prove true.
2. **Zero runtime cost** â€?all checks result in constants or are erased during compilation.
3. **Fail-fast** â€?hardware constraint violations become compile errors, not field failures.
4. **Compile-time lookup tables** â€?precompute complex tables with zero startup overhead.

***

