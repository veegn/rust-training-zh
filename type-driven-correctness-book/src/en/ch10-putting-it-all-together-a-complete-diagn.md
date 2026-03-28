# Putting It All Together — A Complete Diagnostic Platform 🟡

> **What you'll learn:** How all seven core patterns (ch02–ch09) compose into a single diagnostic workflow — authentication, sessions, typed commands, audit tokens, dimensional results, validated data, and phantom-typed registers — with zero total runtime overhead.
>
> **Cross-references:** Every core pattern chapter (ch02–ch09), [ch14](ch14-testing-type-level-guarantees.md) (testing these guarantees)

## Goal: Composite Workflow

We'll combine seven patterns from chapters 2–9 into a single health-check workflow:
1. **Authenticate** (Capability Token — ch04)
2. **Open IPMI session** (Type-State — ch05)
3. **Send typed commands** (Typed Commands — ch02)
4. **Use audit tokens** (Single-Use Types — ch03)
5. **Return dimensional results** (Dimensional Analysis — ch06)
6. **Validate FRU data** (Validated Boundary — ch07)
7. **Read typed registers** (Phantom Types — ch09)

## Composite Implementation

```rust
fn full_diagnostic() -> Result<(), String> {
    // 1. Authenticate → get AdminToken
    let admin = authenticate("admin", "secret")?;

    // 2. Connect and activate (Idle → Active)
    let session = Session::connect("192.168.1.100");
    let mut session = session.activate(&admin)?; 

    // 3. Send typed command → get Celsius
    let temp: Celsius = session.execute(&ReadTemp { sensor_id: 0 })?;

    // 4. Read phantom-typed u16 register
    let vid: u16 = pcie.vendor_id.read(); 

    // 5. Validate FRU at boundary
    let fru = ValidFru::parse(&raw_fru)?;

    // 6. Issue single-use audit token
    let audit = AuditToken::issue(1001);

    // 7. Log and consume token
    audit.log("Diagnostic complete");
    // audit.log("oops"); // ❌ compile error: use of moved value

    Ok(())
}
```

## What the Compiler Proves

| Bug Category | Pattern |
|--------------|---------|
| Unauthenticated access | Capability token |
| Commands in wrong state | Type-state |
| Unit confusion (°C vs RPM) | Dimensional types |
| Wrong response type | Typed commands |
| Register width mismatch | Phantom types |
| Unvalidated data processing | Validated boundary |
| Duplicate audit entries | Single-use type |

**Total runtime overhead: zero.**

## Key Takeaways

1. **Seven patterns compose seamlessly** — authentication, state, units, commands, and more work together.
2. **Zero runtime overhead** — the generated assembly is as efficient as unchecked C code.
3. **Incremental adoption** — you can use one pattern or all seven as needed.
4. **Design template** — use this composite workflow as a blueprint for your own systems.

***
