# Exercises ðŸŸ¡

> **What you'll learn:** Hands-on practice applying correct-by-construction patterns to realistic hardware scenarios â€?NVMe admin commands, firmware update state machines, sensor pipelines, PCIe phantom types, multi-protocol health checks, and session-typed diagnostic protocols.
>
> **Cross-references:** [ch02](ch02-typed-command-interfaces-request-determi.md) (ex 1), [ch05](ch05-protocol-state-machines-type-state-for-r.md) (ex 2), [ch06](ch06-dimensional-analysis-making-the-compiler.md) (ex 3), [ch09](ch09-phantom-types-for-resource-tracking.md) (ex 4), [ch10](ch10-putting-it-all-together-a-complete-diagn.md) (ex 5)

## Practice Problems

### Exercise 1: NVMe Admin Command (Typed Commands)
Design a typed command interface for NVMe admin commands:
- `Identify` â†?`IdentifyResponse`
- `GetLogPage` â†?`SmartLog`
- `GetFeature` â†?feature-specific response

### Exercise 2: Firmware Update State Machine (Type-State)
Model a BMC firmware update lifecycle: `Idle â†?Uploading â†?Verifying â†?Applying â†?Rebooting â†?Complete`.
- `apply()` must require a `VerifiedImage` proof token.
- `abort()` should be available during uploading/verifying but NOT applying.

### Exercise 3: Sensor Reading Pipeline (Dimensional Analysis)
Build a complete pipeline: `ADC â†?calibration â†?threshold check â†?result`.
- Use newtypes for `Celsius`, `Volts`, `Watts`.
- Implement `P = V Ã— I` arithmetic in the type system.

### Exercise 4: PCIe Capability Walk (Phantom Types)
Model the PCIe capability linked list. Each capability type (MSI, MSI-X, PCIe, etc.) should have its own phantom-typed register layout.

### Exercise 5: Multi-Protocol Health Check (Capability Mixins)
Create a health-check framework with mixins like `ThermalHealthMixin` (requires `HasIpmi + HasGpio`) and `StorageHealthMixin` (requires `HasNvmeCli`).

### Exercise 6: Session-Typed Diagnostic Protocol
Design a diagnostic session where `start()` issues `N` execution tokens. Each `TestToken` is consumed when a test runs, preventing duplicate runs.

## Key Takeaways

1. **Practice with realistic protocols** â€?NVMe and firmware updates are perfect targets for these patterns.
2. **Patterns are composable** â€?real-world exercises often require combining 2-3 patterns.
3. **Static enforcement is the goal** â€?if you can represent a protocol violation as a compile error, you've succeeded.

***

