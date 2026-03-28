# Typed Command Interfaces — Request Determines Response 🟡

> **What you'll learn:** How associated types on a command trait create a compile-time binding between request and response, eliminating mismatched parsing, unit confusion, and silent type coercion across IPMI, Redfish, and NVMe protocols.
>
> **Cross-references:** [ch01](ch01-the-philosophy-why-types-beat-tests.md) (philosophy), [ch06](ch06-dimensional-analysis-making-the-compiler.md) (dimensional types), [ch07](ch07-validated-boundaries-parse-dont-validate.md) (validated boundaries), [ch10](ch10-putting-it-all-together-a-complete-diagn.md) (integration)

## The Untyped Swamp

Most hardware management stacks start life as `raw bytes in → raw bytes out`. This leads to parsing bugs, scaling issues, and unit confusion that are often only found in production.

## The Typed Command Pattern

### Step 1 — Domain newtypes

```rust
pub struct Celsius(pub f64);
pub struct Rpm(pub u32);
pub struct Volts(pub f64);
```

### Step 2 — The command trait

The associated type `Response` binds each command to a specific return type.

```rust
pub trait IpmiCmd {
    type Response;
    fn net_fn(&self) -> u8;
    fn cmd_byte(&self) -> u8;
    fn payload(&self) -> Vec<u8>;
    fn parse_response(&self, raw: &[u8]) -> io::Result<Self::Response>;
}
```

### Step 3 — Implementation

Each command struct defines its own response type and parsing logic.

```rust
pub struct ReadTemp { pub sensor_id: u8 }
impl IpmiCmd for ReadTemp {
    type Response = Celsius;
    fn parse_response(&self, raw: &[u8]) -> io::Result<Celsius> {
        Ok(Celsius(raw[0] as f64))
    }
    // ...
}
```

### Step 4 — The executor

```rust
impl BmcConnection {
    pub fn execute<C: IpmiCmd>(&self, cmd: &C) -> io::Result<C::Response> {
        let raw = self.raw_send(cmd.net_fn(), cmd.cmd_byte(), &cmd.payload())?;
        cmd.parse_response(&raw)
    }
}
```

## Pattern Family

This pattern applies to nearly every hardware protocol:

| Protocol | Request Type | Response Type |
|----------|-------------|---------------|
| IPMI | `ReadTemp` | `Celsius` |
| Redfish | `GetThermal` | `ThermalResponse` |
| NVMe Admin | `Identify` | `IdentifyResponse` |
| PLDM | `GetFwParams` | `FwParamsResponse` |

## Key Takeaways

1. **Associated type = compile-time contract** — locks request to response.
2. **Encapsulated parsing** — logic stays with the command definition.
3. **Zero-cost dispatch** — monomorphized generic calls.
4. **Universal pattern** — fits IPMI, Redfish, NVMe, and more.

***
