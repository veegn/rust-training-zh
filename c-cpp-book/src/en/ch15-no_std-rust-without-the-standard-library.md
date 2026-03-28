# 15. `no_std` Rust 🟢

By default, Rust programs link to the standard library (`std`), which provides high-level abstractions like `Vec`, `String`, and networking. However, in embedded systems or OS kernels, you might not have an operating system or a heap allocator. Rust allows you to opt-out of the standard library using the `#![no_std]` attribute.

### 1. The `#![no_std]` Attribute
When you use `#![no_std]`, your program links only to the `core` library, which contains the most fundamental parts of Rust that don't depend on an OS.

```rust
#![no_std]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
```

---

### 2. `core` vs `alloc` vs `std`
- **`core`**: Always available. Includes basics like `Option`, `Result`, and basic math. No heap allocation.
- **`alloc`**: Available if you have a heap allocator. Includes `Vec`, `String`, `Box`, etc.
- **`std`**: Includes `core` and `alloc`, plus OS-specific features like file I/O, networking, and threads.

---

### 3. Implementing a Panic Handler
In a `#![no_std]` environment, you must define what happens when the program panics, as there is no OS to print an error message and terminate the process.

```rust
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    // For embedded, you might blink an LED or reset the CPU here
    loop {}
}
```

---

### 4. Embedded Rust Ecosystem
Rust has a vibrant ecosystem for embedded development:
- **`embedded-hal`**: A set of traits for hardware abstraction.
- **`cortex-m`**: Support for ARM Cortex-M microcontrollers.
- **`riscv`**: Support for RISC-V microcontrollers.

---

### Summary for C/C++ Developers
- **In C/C++**: You often work in a "freestanding" environment by not linking against the standard C library (`libc`). This is similar to `#![no_std]` in Rust.
- **In Rust**: Working without `std` is a first-class feature. Many libraries (crates) are designed to be "no_std" compatible, meaning they can be used in both hosted and bare-metal environments. This makes it much easier to share code between your embedded firmware and your desktop tools.

***
