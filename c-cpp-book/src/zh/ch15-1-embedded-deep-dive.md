[English Original](../en/ch15-1-embedded-deep-dive.md)

# 15.1 嵌入式 Rust 深度探索 🟢

嵌入式 Rust 在 `no_std` 的基础上更进一步，提供了一套专门为微控制器和裸机硬件设计的工具和库。

### 1. 外设访问 Crate (PAC)
PAC 是根据芯片制造商提供的 SVD (System View Description) 文件生成的。它们提供了一种类型安全的方式来访问硬件寄存器。

```rust
use stm32f4::stm32f401;

fn main() {
    let dp = stm32f401::Peripherals::take().unwrap();
    let rcc = &dp.RCC;

    // 以类型安全的方式启用时钟
    rcc.ahb1enr.modify(|_, w| w.gpioaen().enabled());
}
```

---

### 2. 硬件抽象层 (HAL)
HAL 位于 PAC 之上，为与 GPIO、UART、SPI 和 I2C 等外设交互提供了更高级、更符合人体工程学的 API。

```rust
use stm32f4xx_hal::{pac, prelude::*};

fn main() {
    let dp = pac::Peripherals::take().unwrap();
    let gpioa = dp.GPIOA.split();
    let mut led = gpioa.pa5.into_push_pull_output();

    led.set_high(); // 点亮 LED
}
```

---

### 3. `embedded-hal` 生态系统
`embedded-hal` 是一套定义常见外设行为的 Trait。只要不同的微控制器实现了 `embedded-hal` Trait，这套机制就允许你编写跨微控制器运行的通用驱动程序。

```rust
use embedded_hal::digital::v2::OutputPin;

fn blink<P: OutputPin>(pin: &mut P) {
    pin.set_high().ok();
    // 延迟...
    pin.set_low().ok();
}
```

---

### 4. 实时中断驱动并发 (RTIC)
RTIC 是一个在 Rust 中构建实时系统的框架。它为处理中断和在任务之间安全共享数据提供了一种结构化的方式。

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
        // 在按下按钮时切换 LED 状态
    }
}
```

---

### 对于 C/C++ 开发者的总结
- **在 C/C++ 中**：你通常使用厂商提供的 HAL（如 STM32Cube）或编写自己的寄存器级代码。安全性完全由你负责（例如，确保当外设被另一个中断使用时你不会去访问它）。
- **在 Rust 中**：类型系统和所有权模型延伸到了硬件层面。像 RTIC 和 `embedded-hal` 这样的工具确保了你的并发硬件访问在结构上是安全的。这显著降低了固件中难以调试的竞态条件发生的可能性。

***
