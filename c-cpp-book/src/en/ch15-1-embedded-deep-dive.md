# 15.1 Embedded Rust Deep Dive 🟢

Embedded Rust takes `no_std` a step further by providing a set of tools and libraries specifically designed for microcontrollers and bare-metal hardware.

### 1. Peripheral Access Crates (PACs)
PACs are generated from SVD (System View Description) files provided by chip manufacturers. They provide a type-safe way to access hardware registers.

```rust
use stm32f4::stm32f401;

fn main() {
    let dp = stm32f401::Peripherals::take().unwrap();
    let rcc = &dp.RCC;

    // Enabling a clock in a type-safe way
    rcc.ahb1enr.modify(|_, w| w.gpioaen().enabled());
}
```

---

### 2. Hardware Abstraction Layers (HALs)
HALs sit on top of PACs and provide higher-level, more ergonomic APIs for interacting with peripherals like GPIO, UART, SPI, and I2C.

```rust
use stm32f4xx_hal::{pac, prelude::*};

fn main() {
    let dp = pac::Peripherals::take().unwrap();
    let gpioa = dp.GPIOA.split();
    let mut led = gpioa.pa5.into_push_pull_output();

    led.set_high(); // Turn on LED
}
```

---

### 3. The `embedded-hal` Ecosystem
`embedded-hal` is a set of traits that define common peripheral behaviors. This allows you to write generic drivers that work across different microcontrollers as long as they implement the `embedded-hal` traits.

```rust
use embedded_hal::digital::v2::OutputPin;

fn blink<P: OutputPin>(pin: &mut P) {
    pin.set_high().ok();
    // delay...
    pin.set_low().ok();
}
```

---

### 4. Real-Time Interrupt-driven Concurrency (RTIC)
RTIC is a framework for building real-time systems in Rust. It provides a structured way to handle interrupts and share data safely between tasks.

```rust
#[rtic::app(device = stm32f4::stm32f401)]
mod app {
    #[shared]
    struct Shared {}

    #[local]
    struct Local {}

    #[init]
    fn init(_: init::Context) -> (Shared, Local, init::Monotonics) {
        (Shared {}, Local {}, init::Monotonics())
    }

    #[task(binds = EXTI0, local = [led])]
    fn button_pressed(cx: button_pressed::Context) {
        // Toggle LED when button is pressed
    }
}
```

---

### Summary for C/C++ Developers
- **In C/C++**: You often use vendor-provided HALs (like STM32Cube) or write your own register-level code. Safety is entirely up to you (e.g., ensuring you don't access a peripheral while it's being used by another interrupt).
- **In Rust**: The type system and ownership model extend to the hardware level. Tools like RTIC and `embedded-hal` ensure that your concurrent hardware access is safe by construction. This significantly reduces the likelihood of hard-to-debug race conditions in your firmware.

***
