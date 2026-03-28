# Protocol State Machines — Type-State for Real Hardware 🔴

> **What you'll learn:** How type-state encoding makes protocol violations (wrong-order commands, use-after-close) into compile errors, applied to IPMI session lifecycles and PCIe link training.
>
> **Cross-references:** [ch01](ch01-the-philosophy-why-types-beat-tests.md) (level 2 — state correctness), [ch04](ch04-capability-tokens-zero-cost-proof-of-aut.md) (tokens), [ch09](ch09-phantom-types-for-resource-tracking.md) (phantom types), [ch11](ch11-fourteen-tricks-from-the-trenches.md) (trick 4 — typestate builder, trick 8 — async type-state)

## The Problem: Protocol Violations

Hardware protocols like IPMI or PCIe link training have strict state machines. Sending a command in the wrong state (e.g., sending data before authentication) can corrupt sessions or hang buses.

## Type-State Pattern

In Rust, we represent each state as a **distinct type**. State transitions are methods that consume one state and return another. This makes it impossible to call a method in the wrong state because that method doesn't exist on that type.

```rust
pub struct IpmiSession<State> {
    _state: PhantomData<State>,
}

impl IpmiSession<Idle> {
    pub fn authenticate(self) -> Result<IpmiSession<Authenticated>, Error> { ... }
}

impl IpmiSession<Active> {
    pub fn send_command(&mut self) { ... }
    pub fn close(self) -> IpmiSession<Closed> { ... }
}
```

The compiler enforces:
- Authentication before activation.
- Activation before sending commands.
- No commands after close.

## PCIe Link Training (LTSSM)

Link training goes through `Detect → Polling → Configuration → L0`. Type-state ensures `send_tlp()` is only available in the `L0` state.

```rust
impl PcieLink<L0> {
    pub fn send_tlp(&mut self, tlp: &[u8]) { ... }
}
```

## Composition: State + Capability

You can combine type-state with capability tokens (from ch04) to require both an active session and administrator privileges:

```rust
pub fn firmware_update(
    session: &mut IpmiSession<Active>,
    _admin: &AdminToken,
    image: &[u8],
) { ... }
```

## When to Use Type-State

| Protocol | Worthwhile? |
|----------|:----:|
| IPMI/USB/TLS Handshake | ✅ Yes |
| PCIe LTSSM | ✅ Yes |
| Firmware Update Lifecycle | ✅ Yes |
| 2-state Request/Response | ⚠️ Maybe not |

## Key Takeaways

1. **Wrong-order calls are impossible** — methods only exist on valid states.
2. **Transitions consume `self`** — prevents using stale states.
3. **Composable with tokens** — enforce both state and privilege.
4. **Scalable pattern** — works for simple sessions and complex firmware lifecycles.

***
