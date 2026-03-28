[English Original](../en/ch11-serialization-zero-copy-and-binary-data.md)

# 11. 序列化、零拷贝与二进制数据 🟡

> **你将学到：**
> - `serde` 基础：派生宏、属性与枚举表示形式。
> - 用于高性能场景的零拷贝（Zero-copy）反序列化。
> - `serde` 格式生态系统（JSON、bincode 等）。
> - 使用 `repr(C)`、`zerocopy` 和 `bytes` 处理二进制数据。

## serde 基础知识

`serde` 将 **数据模型（Data Model）**（即你的结构体）与 **数据格式（Format）**（如 JSON、二进制）分离开来。

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

### 枚举表示形式

- **外部标记（默认）**：`{"Variant": { "field": "value" }}`
- **内部标记**：`{"type": "Variant", "field": "value"}` (通过 `#[serde(tag = "type")]`)
- **相邻标记**：`{"t": "Variant", "c": { ... }}` (通过 `#[serde(tag = "t", content = "c")]`)
- **无标记**：`{ "field": "value" }` (通过 `#[serde(untagged)]`)

---

## 零拷贝（Zero-Copy）反序列化

通过直接从输入缓冲区借用字符串来避免内存分配，从而实现高性能。

```rust
#[derive(Deserialize)]
struct Record<'a> {
    name: &'a str, // 直接从输入缓冲区借用
    id: u64,
}

let input = r#"{"name": "cpu", "id": 1}"#;
let r: Record = serde_json::from_str(input).unwrap();
```

> **要求**：输入缓冲区的生命周期必须超过反序列化出的结构体。

---

## 二进制数据与固定布局

对于硬件协议或文件格式，使用 `repr(C)` 来保证字段顺序与声明一致。

```rust
#[repr(C)]
struct Header {
    magic: u32,
    version: u16,
    flags: u16,
}
```

### 使用 `zerocopy` 进行安全零拷贝

使用 `zerocopy` 或 `bytemuck` 在不使用 `unsafe` 的情况下，安全地将原始字节转换为结构体引用。

```rust
#[derive(FromBytes, IntoBytes, Immutable)]
#[repr(C)]
struct Reading { id: u16, val: u32 }

let r = Reading::ref_from_bytes(raw_bytes)?;
```

---

## bytes::Bytes

`bytes` crate 提供了引用计数的字节缓冲区。克隆一个 `Bytes` 对象的开销是 O(1)，且不会拷贝底层数据。

```rust
let data = Bytes::from(vec![0; 1024]);
let sub = data.slice(0..10); // 零拷贝子切片
```

***
