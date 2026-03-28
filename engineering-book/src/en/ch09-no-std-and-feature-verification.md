# `no_std` and Feature Verification 🔴

> **What you'll learn:**
> - Verifying feature combinations systematically with `cargo-hack`
> - The three layers of Rust: `core` vs `alloc` vs `std` and when to use each
> - Building `no_std` crates with custom panic handlers and allocators
> - Testing `no_std` code on host and with QEMU
>
> **Cross-references:** [Windows & Conditional Compilation](ch10-windows-and-conditional-compilation.md) — the platform half of this topic · [Cross-Compilation](ch02-cross-compilation-one-source-many-target.md) — cross-compiling to ARM and embedded targets · [Miri and Sanitizers](ch05-miri-valgrind-and-sanitizers-verifying-u.md) — verifying `unsafe` code in `no_std` environments · [Build Scripts](ch01-build-scripts-buildrs-in-depth.md) — `cfg` flags emitted by `build.rs`

Rust runs everywhere from 8-bit microcontrollers to cloud servers. This chapter
covers the foundation: stripping the standard library with `#![no_std]` and
verifying that your feature combinations actually compile.

### Verifying Feature Combinations with `cargo-hack`

[`cargo-hack`](https://github.com/taiki-e/cargo-hack) tests all feature
combinations systematically — essential for crates with `#[cfg(...)]` code:

```bash
# Install
cargo install cargo-hack

# Check that every feature compiles individually
cargo hack check --each-feature --workspace
```

### `no_std` — When and Why

`#![no_std]` tells the compiler: "don't link the standard library." Your
crate can only use `core` (and optionally `alloc`).

| Scenario | Why `no_std` |
|----------|-------------|
| Embedded firmware | No OS, no heap, no file system |
| UEFI diagnostics | Pre-boot environment, no OS APIs |
| Kernel modules | Kernel space can't use userspace `std` |
| WebAssembly | Minimize binary size, no OS dependencies |

### `core` vs `alloc` vs `std` — The Three Layers

1. **`core`**: Always available. Primitives, `Option`, `Result`, traits, atomics.
2. **`alloc`**: Available with an allocator. `Vec`, `String`, `Box`, `BTreeMap`.
3. **`std`**: Full standard library. File I/O, networking, threads, time.

### Building a `no_std` Library

```rust
// src/lib.rs
#![no_std]

pub struct Temperature {
    raw: u16,
}

impl Temperature {
    pub const fn from_raw(raw: u16) -> Self {
        Self { raw }
    }

    pub const fn millidegrees_c(&self) -> i32 {
        (self.raw as i32) * 625 / 10
    }
}
```

### Testing `no_std` Code

Tests run on the host machine, which has `std`. Your library is `no_std`, but
your test harness uses `std` automatically.

```bash
cargo test --lib
```

### 🏋️ Exercises

#### 🟡 Exercise 1: Feature Combination Verification

Install `cargo-hack` and run `cargo hack check --each-feature --workspace` on a project with multiple features. Does it find any broken combinations?

#### 🔴 Exercise 2: Build a `no_std` Library

Create a library crate that compiles with `#![no_std]`. Implement a simple stack-allocated ring buffer. Verify it compiles for `thumbv7em-none-eabihf` (ARM Cortex-M).

### Key Takeaways

- `cargo-hack --each-feature` is essential for any crate with conditional compilation.
- `core` → `alloc` → `std` are layered.
- Custom panic handlers and allocators are required for `no_std` binaries.
- Test `no_std` libraries on the host with `cargo test --lib`.

***
