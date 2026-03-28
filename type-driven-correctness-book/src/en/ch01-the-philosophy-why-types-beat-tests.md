# The Philosophy ‚Ä?Why Types Beat Tests üü¢

> **What you'll learn:** The three levels of compile-time correctness (value, state, protocol), how generic function signatures act as compiler-checked guarantees, and when correct-by-construction patterns are ‚Ä?and aren't ‚Ä?worth the investment.
>
> **Cross-references:** [ch02](ch02-typed-command-interfaces-request-determi.md) (typed commands), [ch05](ch05-protocol-state-machines-type-state-for-r.md) (type-state), [ch13](ch13-reference-card.md) (reference card)

## The Cost of Runtime Checking

Consider a typical runtime guard in a diagnostics codebase:

```rust
fn read_sensor(sensor_type: &str, raw: &[u8]) -> f64 {
    match sensor_type {
        "temperature" => raw[0] as i8 as f64,          // signed byte
        "fan_speed"   => u16::from_le_bytes([raw[0], raw[1]]) as f64,
        "voltage"     => u16::from_le_bytes([raw[0], raw[1]]) as f64 / 1000.0,
        _             => panic!("unknown sensor type: {sensor_type}"),
    }
}
```

This function has **four failure modes** the compiler cannot catch: typos, wrong `raw` length, logic bugs (mixing units), and missing update for new types.

## Three Levels of Correctness

### Level 1 ‚Ä?Value Correctness
**Make invalid values unrepresentable.**

```rust
pub struct Port(u16);  // private field

impl TryFrom<u16> for Port {
    type Error = &'static str;
    fn try_from(v: u16) -> Result<Self, Self::Error> {
        if v > 0 { Ok(Port(v)) } else { Err("port must be > 0") }
    }
}
```

### Level 2 ‚Ä?State Correctness
**Make invalid transitions unrepresentable.**

```rust
struct Socket<State> {
    fd: i32,
    _state: PhantomData<State>,
}

impl Socket<Disconnected> {
    fn connect(self) -> Socket<Connected> { ... }
}

impl Socket<Connected> {
    fn send(&mut self, data: &[u8]) { ... }
}
```

### Level 3 ‚Ä?Protocol Correctness
**Make invalid interactions unrepresentable.**

```rust
trait IpmiCmd {
    type Response;
    fn parse_response(&self, raw: &[u8]) -> io::Result<Self::Response>;
}
```

## When NOT to Use These Patterns

| Situation | Recommendation |
|-----------|---------------|
| Safety-critical boundary | ‚ú?Always |
| Cross-module public API | ‚ú?Usually |
| State machine with 3+ states | ‚ú?Usually |
| Internal helper (50 lines) | ‚ù?Overkill |
| Prototyping | ‚ù?Raw types first |

## Key Takeaways

1. **Three levels of correctness** ‚Ä?value, state, protocol.
2. **Types as guarantees** ‚Ä?every generic signature is a compiler-checked contract.
3. **The cost question** ‚Ä?"if this bug ships, how bad is it?"
4. **Types complement tests** ‚Ä?they eliminate categories; tests cover values.

***

