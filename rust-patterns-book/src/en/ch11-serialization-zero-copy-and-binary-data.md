# 11. Serialization, Zero-Copy, and Binary Data 🟡

> **What you'll learn:**
> - `serde` fundamentals: derive macros, attributes, and enum representations.
> - Zero-copy deserialization for high performance.
> - The `serde` ecosystem (JSON, bincode, etc.).
> - Binary data handling with `repr(C)`, `zerocopy`, and `bytes`.

## serde Fundamentals

`serde` separates the **data model** (your structs) from the **format** (JSON, binary).

```rust
#[derive(Serialize, Deserialize)]
struct Config {
    name: String,
    #[serde(default)]
    port: u16,
    #[serde(rename = "max_conn")]
    max_connections: usize,
}
```

### Enum Representations

- **External (Default)**: `{"Variant": { "field": "value" }}`
- **Internal**: `{"type": "Variant", "field": "value"}` (via `#[serde(tag = "type")]`)
- **Adjacent**: `{"t": "Variant", "c": { ... }}` (via `#[serde(tag = "t", content = "c")]`)
- **Untagged**: `{ "field": "value" }` (via `#[serde(untagged)]`)

---

## Zero-Copy Deserialization

Avoid allocations by borrowing strings directly from the input buffer.

```rust
#[derive(Deserialize)]
struct Record<'a> {
    name: &'a str, // Borrows from input buffer
    id: u64,
}

let input = r#"{"name": "cpu", "id": 1}"#;
let r: Record = serde_json::from_str(input).unwrap();
```

> **Requirement**: The input buffer must outlive the deserialized struct.

---

## Binary Data & Fixed Layouts

For hardware protocols or file formats, use `repr(C)` to guarantee field order.

```rust
#[repr(C)]
struct Header {
    magic: u32,
    version: u16,
    flags: u16,
}
```

### Safe Zero-Copy with `zerocopy`

Use `zerocopy` or `bytemuck` to safely cast bytes to structs without `unsafe`.

```rust
#[derive(FromBytes, IntoBytes, Immutable)]
#[repr(C)]
struct Reading { id: u16, val: u32 }

let r = Reading::ref_from_bytes(raw_bytes)?;
```

---

## bytes::Bytes

The `bytes` crate provides reference-counted byte buffers. Cloning a `Bytes` object is O(1) and does not copy the underlying data.

```rust
let data = Bytes::from(vec![0; 1024]);
let sub = data.slice(0..10); // Zero-copy sub-slice
```

***
