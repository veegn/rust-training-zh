# Fourteen Tricks from the Trenches ðŸŸ¡

> **What you'll learn:** Fourteen smaller correct-by-construction techniques â€?from sentinel elimination and sealed traits to session types, `Pin`, RAII, and `#[must_use]` â€?each eliminating a specific bug class for near-zero effort.
>
> **Cross-references:** [ch02](ch02-typed-command-interfaces-request-determi.md) (sealed traits), [ch05](ch05-protocol-state-machines-type-state-for-r.md) (typestate builder), [ch07](ch07-validated-boundaries-parse-dont-validate.md) (FromStr)

## High-Value Tricks

While the eight core patterns cover major architectures, these fourteen smaller tricks are frequently used in production to eliminate specific bug categories.

### 1. Sentinel â†?`Option` at Boundary
Convert hardware-sentinel values (like `0xFF` for "not present") into `Option::None` at the very first parsing boundary. This forces all downstream consumers to handle the "missing" case.

### 2. Sealed Traits
Prevent external crates from implementing your internal traits (like `IpmiCmd`) by requiring a private supertrait.

```rust
mod private { pub trait Sealed {} }
pub trait IpmiCmd: private::Sealed { ... }
```

### 3. `#[non_exhaustive]` for Evolving Enums
Force external consumers of an enum to include a wildcard (`_`) in `match` statements. This ensures your library can add new variants (e.g., a new hardware Sku) without breaking downstream builds.

### 4. Typestate Builder
Ensure that a builder's `build()` method can only be called once all required fields are set by using type parameters to track state.

### 5. `FromStr` as Validation Boundary
Use `FromStr` to validate string-based inputs (configs, CLI args) immediately upon entry into the system.

### 6. Const Generics for Size Validation
Encode fixed hardware sizes (e.g., 4096-byte NVMe buffers) in the type system to prevent passing wrongly-sized buffers.

### 7. Safe Wrappers Around `unsafe`
Contain `unsafe` blocks within a small, auditable module and expose only safe methods to the rest of the application.

### 8. Async Type-State
Extend type-state to `async` workflows. Ensure transitions take `self` to maintain ownership across `.await` points.

### 9. Refinement Types via Const Assertions
Reject invalid hardware IDs (e.g., sensor ID must be 0x01..0xFE) at compile time using `const` assertions.

### 10. Session Types for Channels
Encode communication protocols (Request -> Response -> Done) in the channel types themselves to prevent out-of-order messages.

... and more (RAII, `#[must_use]`, custom error enums, etc.).

## Key Takeaways

1. **Small effort, high impact** â€?most of these tricks take only a few lines to implement.
2. **Eliminate specific bug classes** â€?from "forgot to check sentinel" to "used closed session."
3. **Incremental improvement** â€?apply these tricks to your existing codebase as you find them useful.

***

