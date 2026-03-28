# Dimensional Analysis ‚Ä?Making the Compiler Check Your Units üü¢

> **What you'll learn:** How newtype wrappers and the `uom` crate turn the compiler into a unit-checking engine, preventing the class of bug that destroyed a $328M spacecraft.
>
> **Cross-references:** [ch02](ch02-typed-command-interfaces-request-determi.md) (typed commands use these types), [ch07](ch07-validated-boundaries-parse-dont-validate.md) (validated boundaries), [ch10](ch10-putting-it-all-together-a-complete-diagn.md) (integration)

## The Mars Climate Orbiter

In 1999, NASA lost a $327.6M spacecraft because one team used pound-force seconds and another expected newton-seconds. Both were `double` at the code level, and the compiler couldn't distinguish them.

## Newtypes for Physical Quantities

The simplest fix: **wrap each unit in its own type**.

```rust
pub struct Celsius(pub f64);
pub struct Fahrenheit(pub f64);
pub struct Volts(pub f64);
pub struct Rpm(pub f64);
```

Now, comparing `Celsius` to `Volts` is a **compile error**:

```rust
fn check_limit(temp: Celsius, limit: Celsius) -> bool {
    temp > limit // ‚ú?same units
}

// temp > voltage // ‚ù?ERROR: mismatched types
```

## Macro-Generated Quantities

A macro can eliminate the boilerplate for comparisons, arithmetic, and display:

```rust
macro_rules! quantity {
    ($Name:ident, $unit:expr) => {
        #[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
        pub struct $Name(pub f64);
        // ... impl Display, Add, Sub ...
    };
}

quantity!(Celsius, "¬∞C");
quantity!(Volts, "V");
```

## The `uom` Crate

For advanced dimensional analysis (e.g., automatically knowing that `Watts = Volts √ó Amperes`), use the [`uom`](https://crates.io/crates/uom) (Units of Measurement) crate.

```rust
// uom handles complex derived units at zero runtime cost.
let power = voltage * current; // automatically type-checked
```

## When to Use Dimensional Types

| Scenario | Recommendation |
|----------|---------------|
| Sensor readings | ‚ú?Always |
| Thresholds | ‚ú?Always |
| API boundaries | ‚ú?Always |
| Internal helpers | ‚ö†Ô∏è Optional |

## Key Takeaways

1. **Newtypes prevent unit confusion** ‚Ä?`Celsius` and `Rpm` are distinct types.
2. **Zero runtime cost** ‚Ä?newtypes compile down to their inner values (e.g., `f64`).
3. **Macro automation** ‚Ä?quickly stamp out units with standard operations.
4. **`uom` for derived units** ‚Ä?use it for complex physics calculations.

***

