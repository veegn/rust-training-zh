# Validated Boundaries — Parse, Don't Validate 🟡

> **What you'll learn:** How to validate data exactly once at the system boundary, carry the proof of validity in a dedicated type, and never re-check — applied to IPMI FRU records, Redfish JSON, and IPMI SEL records.
>
> **Cross-references:** [ch02](ch02-typed-command-interfaces-request-determi.md) (typed commands), [ch06](ch06-dimensional-analysis-making-the-compiler.md) (dimensional types), [ch11](ch11-fourteen-tricks-from-the-trenches.md) (tricks 2, 3, 5), [ch14](ch14-testing-type-level-guarantees.md) (proptest)

## The Problem: Shotgun Validation

In many codebases, validation is scattered across every function that receives data. This results in code redundancy and potential gaps where a check is forgotten, leading to bugs.

## Parse, Don't Validate

The correct-by-construction approach is to **validate once at the boundary** and then carry the proof of validity in the type itself.

### IPMI FRU Data Case Study

```rust
pub struct ValidFru {
    format_version: u8,
    internal_area_offset: u8,
    chassis_area_offset: u8,
    board_area_offset: u8,
    product_area_offset: u8,
    data: Vec<u8>,
}

impl TryFrom<RawFruData> for ValidFru {
    type Error = FruError;

    fn try_from(raw: RawFruData) -> Result<Self, FruError> {
        let data = raw.0;
        // 1. Length check
        // 2. Format version check
        // 3. Checksum verification
        // 4. Offset validation
        // ...
        Ok(ValidFru { ... })
    }
}
```

Once you have a `ValidFru`, all downstream functions know the data is well-formed without further checking.

## Validated Redfish JSON

Parsing Redfish responses into typed structures ensures all required fields are present and within valid ranges.

```rust
pub struct ValidThermalResponse {
    pub temperatures: Vec<ValidTemperatureReading>,
    pub fans: Vec<ValidFanReading>,
}
```

## Polymorphic Validation: IPMI SEL Records

IPMI System Event Log (SEL) records are 16-byte fixed-size but polymorphic. The meaning changes based on "Record Type", "Event Type", and "Sensor Type".

We handle this by nesting enums that mirror the spec's dispatch hierarchy:

```rust
pub enum ValidSelRecord {
    SystemEvent(SystemEventRecord),
    OemTimestamped(OemTimestampedRecord),
    OemNonTimestamped(OemNonTimestampedRecord),
}

pub enum TypedEvent {
    Threshold(ThresholdEvent),
    SensorSpecific(SensorSpecificEvent),
    Discrete { offset: u8, event_data: [u8; 3] },
}
```

## Advantages of Boundary Validation

1. **Redundancy removal** — checks happen once.
2. **Exhaustive handling** — enums force you to handle all possible variants (e.g., all 42 IPMI sensor types).
3. **Internal safety** — functions taking validated types can use `unwrap()` or direct indexing safely.

## Key Takeaways

1. **Boundary is the filter** — data entrance is the single point for validation.
2. **Types as proofs** — witnessing a `ValidFru` is proof that it was checked for checksums and bounds.
3. **Enums handle polymorphism** — mirror spec hierarchies in your type system to avoid silent logic errors.
4. **Internal code is cleaner** — no more defensive `if` statements everywhere.

***
