# Testing Type-Level Guarantees 🟡

> **What you'll learn:** How to test that invalid code *fails to compile* (trybuild), fuzz validated boundaries (proptest), verify RAII invariants, and prove zero-cost abstraction via `cargo-show-asm`.
>
> **Cross-references:** [ch03](ch03-single-use-types-cryptographic-guarantee.md) (nonces), [ch05](ch05-protocol-state-machines-type-state-for-r.md) (sessions), [ch07](ch07-validated-boundaries-parse-dont-validate.md) (proptest)

## Testing the "Impossible"

Correct-by-construction patterns shift bugs to compile time. But how do we ensure those invariants stay enforced after project refactors?

### 1. Compile-Fail Tests with `trybuild`
The [`trybuild`](https://crates.io/crates/trybuild) crate lets you assert that certain invalid code **should not compile**.

```rust
// In a test file:
let t = trybuild::TestCases::new();
t.compile_fail("tests/ui/nonce_reuse.rs");
```

If the code in `nonce_reuse.rs` (e.g., using a single-use token twice) ever compiles, the test fails.

### 2. Property-Based Testing with `proptest`
Validated boundaries (ch07) parse data once. Use [`proptest`](https://crates.io/crates/proptest) to generate thousands of random inputs to ensure your `TryFrom` implementation never panics and correctly rejects garbage.

### 3. Proof of Zero-Cost with `cargo-show-asm`
To prove that newtypes and phantom types have zero runtime overhead, use `cargo-show-asm` to inspect the generated assembly. You'll see that `Celsius(f64)` and `f64` produce identical machine code.

## The Testing Pyramid

1. **Compile-Fail (trybuild)**: "Invalid code must not compile."
2. **Property Tests (proptest)**: "Valid inputs never panic."
3. **Unit Tests (#[test])**: "Specific logic behaves as expected."
4. **Type System**: "Entire classes of bugs are structurally impossible."

## Key Takeaways

1. **`trybuild` maintains invariants** — catches accidental `impl Clone` on single-use types.
2. **`proptest` stresses boundaries** — ensures validation logic is robust against fuzzing.
3. **Assembly doesn't lie** — `cargo-show-asm` confirms zero runtime cost for all type-level markers.
4. **Test the impossible** — if a state is supposed to be impossible, write a test that tries to reach it and fails to compile.

***
