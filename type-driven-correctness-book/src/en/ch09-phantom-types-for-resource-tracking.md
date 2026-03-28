# Phantom Types for Resource Tracking üü°

> **What you'll learn:** How `PhantomData` markers encode register width, DMA direction, and file-descriptor state at the type level ‚Ä?preventing an entire class of resource-mismatch bugs at zero runtime cost.
>
> **Cross-references:** [ch05](ch05-protocol-state-machines-type-state-for-r.md) (type-state), [ch06](ch06-dimensional-analysis-making-the-compiler.md) (dimensional types), [ch08](ch08-capability-mixins-compile-time-hardware-.md) (mixins), [ch10](ch10-putting-it-all-together-a-complete-diagn.md) (integration)

## The Problem: Resource Mismatch

Hardware resources often look identical in code (e.g., both 32-bit and 16-bit registers might be represented as simple pointers or offsets). This leads to silent bugs when using the wrong function for a resource.

## Phantom Type Parameters

A **phantom type** is a type parameter that appears in the struct definition but not in any field. It exists purely to carry type-level information.

```rust
pub struct Width16;
pub struct Width32;

pub struct Register<W> {
    base: usize,
    offset: usize,
    _width: PhantomData<W>, // 0 bytes at runtime
}

impl Register<Width16> {
    pub fn read(&self) -> u16 { ... }
}

impl Register<Width32> {
    pub fn read(&self) -> u32 { ... }
}
```

Now, the compiler prevents you from reading a `Width16` register as a `u32`.

## DMA Buffer Access Control

DMA buffers have specific directions (read-only from device vs. write-only to device). Phantom types enforce this:

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

Attempting to `write_data` to a `DmaBuffer<FromDevice>` is a **compile error**.

## When to Use Phantom Types

| Scenario | Use phantom parameter? |
|----------|:------:|
| Register widths | ‚ú?Always |
| DMA direction | ‚ú?Always |
| File descriptor state | ‚ú?Always |
| Permissions (R/W/X) | ‚ú?Always |
| Runtime-variable state | ‚ù?Use enums |

## Key Takeaways

1. **`PhantomData` markers cost zero bytes** ‚Ä?they are compile-time-only labels.
2. **Structural prevention of mismatches** ‚Ä?register width and DMA direction are enforced by the type system.
3. **Synergy with other patterns** ‚Ä?combine with dimensional types (ch06) for even stronger guarantees.
4. **Compile-time only** ‚Ä?they do not work for attributes that change at runtime.

***

