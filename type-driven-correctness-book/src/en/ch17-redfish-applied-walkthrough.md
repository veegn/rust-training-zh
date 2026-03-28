# Applied Walkthrough â€?Type-Safe Redfish Client ðŸŸ¡

> **What you'll learn:** How to compose type-state sessions, capability tokens, phantom-typed resource navigation, dimensional analysis, validated boundaries, and builder type-state into a complete Redfish client.
>
> **Cross-references:** [ch02](ch02-typed-command-interfaces-request-determi.md) (commands), [ch04](ch04-capability-tokens-zero-cost-proof-of-aut.md) (tokens), [ch05](ch05-protocol-state-machines-type-state-for-r.md) (type-state), [ch06](ch06-dimensional-analysis-making-the-compiler.md) (dimensional types), [ch07](ch07-validated-boundaries-parse-dont-validate.md) (validated boundaries), [ch09](ch09-phantom-types-for-resource-tracking.md) (phantom types).

## The Complexity of Redfish

Redfish is a RESTful API for hardware management. While common, it's riddled with correctness hazards: malformed URIs, missing privilege checks, and unit confusion in JSON telemetry.

## Section 1: Session Lifecycle (Type-State)

Encode the connection lifecycle into the type system: `Disconnected â†?Connected â†?Authenticated â†?Closed`.
- Requests can *only* be sent on an `Authenticated` session.
- `logout()` consumes the session, preventing re-use.

## Section 2: Privilege Tokens (Capability Tokens)

Use zero-sized proof tokens for Redfish privileges: `LoginToken`, `ConfigureComponentsToken`, `ConfigureManagerToken`.
- `login()` returns tokens based on the user's role.
- `set_boot_order()` requires a `ConfigureComponentsToken` as an argument.

## Section 3: Typed Resource Navigation (Phantom Types)

Represent the Redfish resource tree as types: `ServiceRoot â†?ChassisCollection â†?ChassisInstance â†?Thermal`.
- Navigation methods (e.g., `.chassis()`) return paths tagged with the resource type.
- This prevents constructing invalid URIs like `.../Chassis/1/Bios` (BIOS is under Systems, not Chassis).

## Section 4: Typed Telemetry (Dimensional Analysis)

Parse Redfish JSON into validated structs with dimensional types like `Celsius` and `Watts`. The compiler will reject comparing a temperature reading to an RPM value.

## Key Takeaways

1. **Composition is Power** â€?combining 5+ patterns creates a "hardened" client where protocol violations are compile errors.
2. **Zero Overhead** â€?all these checks (tokens, state, phantom types) are erased at compile time.
3. **Formalizing Assumptions** â€?REST APIs are often loosely defined; types force you to formalize exactly what "Authenticated" or "Valid Thermal Data" means.

***

