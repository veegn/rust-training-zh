# Reference Card

> **Quick-reference for all 14+ correct-by-construction patterns** with selection flowchart, pattern catalogue, composition rules, crate mapping, and types-as-guarantees cheat sheet.
>
> **Cross-references:** Every chapter â€?this is the lookup table for the entire book.

## Quick Reference Table

| # | Pattern | Prevents | Chapter |
|---|---------|----------|---------|
| 1 | Typed Commands | Wrong response type | ch02 |
| 2 | Single-Use Types | Nonce/key reuse | ch03 |
| 3 | Capability Tokens | Unauthorized access | ch04 |
| 4 | Type-State | Protocol violations | ch05 |
| 5 | Dimensional Types | Unit confusion | ch06 |
| 6 | Validated Boundaries | Unvalidated data use | ch07 |
| 7 | Capability Mixins | Missing bus access | ch08 |
| 8 | Phantom Types | Width/direction mismatch | ch09 |
| 9 | Sentinel â†?Option | Sentinel-as-value bugs | ch11 |
| 10| Sealed Traits | Unsound external impls | ch11 |

## Composition Rules (Examples)

- **Capability Token + Type-State**: Authorized state transitions.
- **Typed Command + Dimensional Type**: Physically-typed responses.
- **Validated Boundary + Phantom Type**: Typed register access on validated config mappings.

## Anti-Patterns and Refactors

| Anti-Pattern | Correct Alternative |
|--------------|-------------------|
| `fn read() -> f64` | `fn read() -> Celsius` |
| `fn op(is_admin: bool)` | `fn op(_: &AdminToken)` |
| `fn send(session: &Session)` | `fn send(session: &Session<Active>)` |

## Types as Guarantees Mapper

- **"This proof exists"**: A Type.
- **"I have the proof"**: A Value of that type.
- **"A implies B"**: Function `fn(A) -> B`.
- **"Both A and B"**: Tuple `(A, B)`.
- **"Either A or B"**: `enum { A, B }` or `Result<A, B>`.

***

