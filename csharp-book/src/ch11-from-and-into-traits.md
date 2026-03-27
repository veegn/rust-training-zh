## Type Conversions in Rust | Rust 中的类型转换

> **What you'll learn:** `From`/`Into` traits vs C#'s implicit/explicit operators, `TryFrom`/`TryInto`
> for fallible conversions, `FromStr` for parsing, and idiomatic string conversion patterns.
>
> **你将学到什么：** `From`/`Into` trait 与 C# 隐式/显式转换运算符的对比，`TryFrom`/`TryInto`
> 如何表示可能失败的转换，`FromStr` 如何用于解析，以及惯用的字符串转换模式。
>
> **Difficulty:** Intermediate
>
> **难度：** 中级

C# uses implicit/explicit conversions and casting operators. Rust uses the `From` and `Into` traits for safe, explicit conversions.

C# 主要通过隐式/显式转换和强制类型转换运算符完成类型变换。Rust 则通过 `From` 和 `Into` trait 提供安全、显式的转换能力。

### C# Conversion Patterns | C# 转换模式
```csharp
// C# implicit/explicit conversions
public class Temperature
{
    public double Celsius { get; }
    
    public Temperature(double celsius) { Celsius = celsius; }
    
    // Implicit conversion
    public static implicit operator double(Temperature t) => t.Celsius;
    
    // Explicit conversion
    public static explicit operator Temperature(double d) => new Temperature(d);
}

double temp = new Temperature(100.0);  // implicit
Temperature t = (Temperature)37.5;     // explicit
```

### Rust From and Into | Rust 的 `From` 与 `Into`
```rust
#[derive(Debug)]
struct Temperature {
    celsius: f64,
}

impl From<f64> for Temperature {
    fn from(celsius: f64) -> Self {
        Temperature { celsius }
    }
}

impl From<Temperature> for f64 {
    fn from(temp: Temperature) -> f64 {
        temp.celsius
    }
}

fn main() {
    // From
    let temp = Temperature::from(100.0);
    
    // Into (automatically available when From is implemented)
    let temp2: Temperature = 37.5.into();
    
    // Works in function arguments too
    fn process_temp(temp: impl Into<Temperature>) {
        let t: Temperature = temp.into();
        println!("Temperature: {:.1}degC", t.celsius);
    }
    
    process_temp(98.6);
    process_temp(Temperature { celsius: 0.0 });
}
```

```mermaid
graph LR
    A["impl From&lt;f64&gt; for Temperature"] -->|"auto-generates"| B["impl Into&lt;Temperature&gt; for f64"]
    C["Temperature::from(37.5)"] -->|"explicit"| D["Temperature"]
    E["37.5.into()"] -->|"implicit via Into"| D
    F["fn process(t: impl Into&lt;Temperature&gt;)"] -->|"accepts both"| D

    style A fill:#c8e6c9,color:#000
    style B fill:#bbdefb,color:#000
```

> **Rule of thumb**: Implement `From`, and you get `Into` for free. Callers can use whichever reads better.
>
> **经验法则：** 实现 `From` 之后，`Into` 会自动可用。调用方可以选择更顺手的写法。

### TryFrom for Fallible Conversions | `TryFrom`：可能失败的转换
```rust
use std::convert::TryFrom;

impl TryFrom<i32> for Temperature {
    type Error = String;
    
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value < -273 {
            Err(format!("Temperature {}degC is below absolute zero", value))
        } else {
            Ok(Temperature { celsius: value as f64 })
        }
    }
}

fn main() {
    match Temperature::try_from(-300) {
        Ok(t) => println!("Valid: {:?}", t),
        Err(e) => println!("Error: {}", e),
    }
}
```

### String Conversions | 字符串转换
```rust
// ToString via Display trait
impl std::fmt::Display for Temperature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:.1}degC", self.celsius)
    }
}

// Now .to_string() works automatically
let s = Temperature::from(100.0).to_string(); // "100.0degC"

// FromStr for parsing
use std::str::FromStr;

impl FromStr for Temperature {
    type Err = String;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim_end_matches("degC").trim();
        let celsius: f64 = s.parse().map_err(|e| format!("Invalid temp: {}", e))?;
        Ok(Temperature { celsius })
    }
}

let t: Temperature = "100.0degC".parse().unwrap();
```

```text
Rust 转换体系的重点不是“隐式更方便”，而是“转换规则由 trait 明确表达，可组合、可检查、可失败”。
```

---

## Exercises | 练习

<details>
<summary><strong>Exercise: Currency Converter | 练习：货币转换器</strong> (click to expand / 点击展开)</summary>

Create a `Money` struct that demonstrates the full conversion ecosystem:

创建一个 `Money` 结构体，完整展示 Rust 的转换体系：

1. `Money { cents: i64 }` (stores value in cents to avoid floating-point issues)
1. `Money { cents: i64 }`（以分为单位存储，避免浮点误差）
2. Implement `From<i64>` (treats input as whole dollars -> `cents = dollars * 100`)
2. 实现 `From<i64>`（把输入视为整美元，转换为 `cents = dollars * 100`）
3. Implement `TryFrom<f64>` - reject negative amounts, round to nearest cent
3. 实现 `TryFrom<f64>`，拒绝负数金额，并四舍五入到最近的分
4. Implement `Display` to show `"$1.50"` format
4. 实现 `Display`，输出 `"$1.50"` 这样的格式
5. Implement `FromStr` to parse `"$1.50"` or `"1.50"` back into `Money`
5. 实现 `FromStr`，支持把 `"$1.50"` 或 `"1.50"` 解析为 `Money`
6. Write a function `fn total(items: &[impl Into<Money> + Copy]) -> Money` that sums values
6. 编写函数 `fn total(items: &[impl Into<Money> + Copy]) -> Money`，对多个值求和

<details>
<summary>Solution | 参考答案</summary>

```rust
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
struct Money { cents: i64 }

impl From<i64> for Money {
    fn from(dollars: i64) -> Self {
        Money { cents: dollars * 100 }
    }
}

impl TryFrom<f64> for Money {
    type Error = String;
    fn try_from(value: f64) -> Result<Self, Self::Error> {
        if value < 0.0 {
            Err(format!("negative amount: {value}"))
        } else {
            Ok(Money { cents: (value * 100.0).round() as i64 })
        }
    }
}

impl fmt::Display for Money {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "${}.{:02}", self.cents / 100, self.cents.abs() % 100)
    }
}

impl FromStr for Money {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim_start_matches('$');
        let val: f64 = s.parse().map_err(|e| format!("{e}"))?;
        Money::try_from(val)
    }
}

fn main() {
    let a = Money::from(10);                         // $10.00
    let b = Money::try_from(3.50).unwrap();         // $3.50
    let c: Money = "$7.25".parse().unwrap();        // $7.25
    println!("{a} + {b} + {c}");
}
```

</details>
</details>

***
