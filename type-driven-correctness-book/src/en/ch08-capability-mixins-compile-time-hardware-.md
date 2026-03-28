# Capability Mixins — Compile-Time Hardware Contracts 🟡

> **What you'll learn:** How ingredient traits combined with mixin traits and blanket impls eliminate diagnostic code duplication while guaranteeing every hardware dependency is satisfied at compile time.
>
> **Cross-references:** [ch04](ch04-capability-tokens-zero-cost-proof-of-aut.md) (tokens), [ch09](ch09-phantom-types-for-resource-tracking.md) (phantom types), [ch10](ch10-putting-it-all-together-a-complete-diagn.md) (integration)

## The Problem: Diagnostic Code Duplication

Different hardware subsystems often require similar diagnostic workflows (e.g., reading sensors, checking thresholds) but operate over different buses (SPI, I2C, GPIO). This usually leads to copy-pasted code.

## Ingredient Traits (Hardware Capabilities)

We define hardware capabilities (buses, controllers) as traits with associated types:

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

## Mixin Traits (Diagnostic Behaviors)

A mixin provides behavior **automatically** using a blanket implementation for any type that possesses the required "ingredients" (hardware capabilities).

```rust
pub trait FanDiagMixin: HasSpi + HasI2c {
    fn run_fan_diagnostic(&self) -> bool {
        // Uses self.spi() and self.i2c() to perform diagnostics
        true
    }
}

// Any type that has both SPI and I2C gets this mixin for free.
impl<T: HasSpi + HasI2c> FanDiagMixin for T {}
```

## Mixing and Matching

A concrete controller simply lists the buses it has, and it automatically inherits all matching diagnostic behaviors:

```rust
pub struct BaseBoardController {
    spi: LinuxSpi,
    i2c: LinuxI2c,
    // ...
}

impl HasSpi for BaseBoardController { ... }
impl HasI2c for BaseBoardController { ... }

// BaseBoardController now automatically has FanDiagMixin!
```

## When to Use Capability Mixins

| Scenario | Recommendation |
|----------|:------:|
| Common diagnostic behaviors | ✅ Always |
| Multi-bus controllers | ✅ Always |
| Platform-specific testing | ✅ Always |
| Simple single-bus devices | ⚠️ Optional |

## Key Takeaways

1. **Ingredient traits declare capabilities** — e.g., `HasSpi`, `HasI2c`.
2. **Mixins provide behavior via blanket impls** — `impl<T: HasSpi + HasI2c> FanDiagMixin for T {}`.
3. **Compile-time dependency check** — if a bus is removed, the associated mixin methods vanish at compile time.
4. **Platform-agnostic diagnostic logic** — write the logic once; reuse it on every platform that provides the required buses.

***
