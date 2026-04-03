[English Original](../en/ch14-testing-type-level-guarantees.md)

# 测试类型层级的保证 🟡

> **你将学到：**
> - 如何测试无效代码 *无法通过编译* (`trybuild`)、如何对验证边界进行模糊测试 (`proptest`)、如何验证 RAII 不变式，以及如何通过 `cargo-show-asm` 证明零成本抽象。
>
> **参考：** [第 3 章](ch03-single-use-types-cryptographic-guarantee.md)（Nonce 的编译失败测试）、[第 7 章](ch07-validated-boundaries-parse-dont-validate.md)（边界的 proptest 测试）、[第 5 章](ch05-protocol-state-machines-type-state-for-r.md)（会话的 RAII 验证）。

## 测试类型层级的保证

“正确构建 (Correct-by-Construction)”模式将 Bug 从运行时转移到了编译时。但是，你该如何 **测试** 无效的代码确实无法通过编译呢？又该如何确保验证边界在模糊测试下依然稳健？本章将介绍与类型层级正确性相辅相成的各种测试工具。

### 使用 `trybuild` 进行编译失败测试

[`trybuild`](https://crates.io/crates/trybuild) crate 允许你断言某些代码 **不应通过编译**。这对于在重构过程中维持类型层级的不变式至关重要 —— 如果有人不小心给你的一次性 `Nonce` 增加了 `Clone` 实现，编译失败测试就能捕获到它。

**设置：**

```toml
# Cargo.toml
[dev-dependencies]
trybuild = "1"
```

**测试文件 (`tests/compile_fail.rs`)：**

```rust,ignore
#[test]
fn type_safety_tests() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/*.rs");
}
```

**测试用例：Nonce 的重复使用必须不能编译 (`tests/ui/nonce_reuse.rs`)：**

```rust,ignore
// tests/ui/nonce_reuse.rs
use my_crate::Nonce;

fn main() {
    let nonce = Nonce::new();
    encrypt(nonce);
    encrypt(nonce); // 应当失败：此处使用了已被移动的值 (use of moved value)
}

fn encrypt(_n: Nonce) {}
```

**预期的错误信息 (`tests/ui/nonce_reuse.stderr`)：**

```text
error[E0382]: use of moved value: `nonce`
 --> tests/ui/nonce_reuse.rs:6:13
  |
4 |     let nonce = Nonce::new();
  |         ----- move occurs because `nonce` has type `Nonce`, which does not implement the `Copy` trait
5 |     encrypt(nonce);
  |             ----- value moved here
6 |     encrypt(nonce); // should fail: use of moved value
  |             ^^^^^ value used here after move
```

**针对不同章节的更多编译失败测试用例：**

| 模式 (章节) | 测试断言 | 文件 |
|-------------------|---------------|------|
| 一次性 Nonce (第 3 章) | 无法使用同一个 Nonce 两次 | `nonce_reuse.rs` |
| 能力令牌 (第 4 章) | 无令牌则无法调用 `admin_op()` | `missing_token.rs` |
| 类型状态 (第 5 章) | 无权在 `Session<Idle>` 上调用 `send_command()` | `wrong_state.rs` |
| 维度类型 (第 6 章) | 无法将 `Celsius + Rpm` 进行加和 | `unit_mismatch.rs` |
| 密封特性 (技巧 2) | 外部 crate 无法实现密封特性 | `unseal_attempt.rs` |
| 非穷尽枚举 (技巧 3) | 外部 match 如果没有通配符则报错 | `missing_wildcard.rs` |

**CI 集成：**

```yaml
# .github/workflows/ci.yml
- name: Run compile-fail tests
  run: cargo test --test compile_fail
```

### 验证边界的基于属性的测试

验证边界 (第 7 章) 仅在解析阶段对数据进行一次校验，之后便拒绝任何非法输入。但是，你如何知道你的验证逻辑捕捉到了 **所有** 的非法输入呢？使用 [`proptest`](https://crates.io/crates/proptest) 来进行基于属性的测试，它可以生成数千个随机输入，对边界进行压力测试：

```toml
# Cargo.toml
[dev-dependencies]
proptest = "1"
```

```rust,ignore
use proptest::prelude::*;

/// 选自第 7 章：ValidFru 封装了符合规范的 FRU 负载。
/// 这些测试使用了带 board_area()、product_area() 
/// 以及 format_version() 方法的完整第 7 章 ValidFru 实现。
/// 注意：第 7 章定义了 TryFrom<RawFruData>，因此我们首先需要封装原始字节。

proptest! {
    /// 任何通过了验证的字节序列都必须能被安全使用，且不会发生 panic。
    #[test]
    fn valid_fru_never_panics(data in proptest::collection::vec(any::<u8>(), 0..1024)) {
        if let Ok(fru) = ValidFru::try_from(RawFruData(data)) {
            // 在已验证的 FRU 上，这些方法绝不能产生 panic
            // (来自第 7 章 ValidFru 实现的方法)：
            let _ = fru.format_version();
            let _ = fru.board_area();
            let _ = fru.product_area();
        }
    }

    /// 往返测试 (Round-trip)：重新解析后 format_version 保持不变。
    #[test]
    fn fru_round_trip(data in valid_fru_strategy()) {
        let raw = RawFruData(data.clone());
        let fru = ValidFru::try_from(raw).unwrap();
        let version = fru.format_version();
        // 重新解析相同的字节 —— 版本号必须一致
        let reparsed = ValidFru::try_from(RawFruData(data)).unwrap();
        prop_assert_eq!(version, reparsed.format_version());
    }
}

/// 自定义策略：生成满足 FRU 规范头部的字节向量。
/// 头部格式与第 7 章的 `TryFrom<RawFruData>` 验证逻辑一致：
///   - 字节 0: 版本 = 0x01
///   - 字节 1-6: 区域偏移量 (乘以 8 = 实际字节偏移)
///   - 字节 7: 校验和 (字节 0-7 的总和对 256 取模结果为 0)
/// Body 是随机生成的，但长度足够以保证偏移量在合法范围内。
fn valid_fru_strategy() -> impl Strategy<Value = Vec<u8>> {
    let header = vec![0x01, 0x00, 0x01, 0x02, 0x00, 0x00, 0x00];
    proptest::collection::vec(any::<u8>(), 64..256)
        .prop_map(move |body| {
            let mut fru = header.clone();
            let sum: u8 = fru.iter().fold(0u8, |a, &b| a.wrapping_add(b));
            fru.push(0u8.wrapping_sub(sum));
            fru.extend_from_slice(&body);
            fru
        })
}
```

**针对“正确构建”代码的测试金字塔：**

```text
┌───────────────────────────────────┐
│     编译失败测试 (trybuild)       │ ← “无效代码绝不能通过编译”
├───────────────────────────────────┤
│   属性测试 (proptest/quickcheck)  │ ← “合法输入绝不能产生 panic”
├───────────────────────────────────┤
│     单元测试 (#[test])            │ ← “特定输入产生预期输出”
├───────────────────────────────────┤
│     类型系统 (第 2–13 章模式)     │ ← “整类 Bug 根本无法存在”
└───────────────────────────────────┘
```

### RAII 验证

RAII (技巧 12) 保证了清理工作的执行。要对此进行测试，只需验证 `Drop` 实现确实被触发了即可：

```rust,ignore
use std::sync::atomic::{AtomicBool, Ordering};

// 注意：这些测试使用了全局 AtomicBool，因此不能并行运行。
// 请使用 `#[serial_test::serial]` 或通过 `cargo test -- --test-threads=1` 运行。
// 另一种更好的替代方案是使用闭包内传递的每个测试专属的 `Arc<AtomicBool>`，从而完全避免全局变量。
static DROPPED: AtomicBool = AtomicBool::new(false);

struct TestSession;
impl Drop for TestSession {
    fn drop(&mut self) {
        DROPPED.store(true, Ordering::SeqCst);
    }
}

#[test]
fn session_drops_on_early_return() {
    DROPPED.store(false, Ordering::SeqCst);
    let result: Result<(), &str> = (|| {
        let _session = TestSession;
        Err("模拟失败")?;
        Ok(())
    })();
    assert!(result.is_err());
    assert!(DROPPED.load(Ordering::SeqCst), "在提前返回时必须触发 Drop");
}

#[test]
fn session_drops_on_panic() {
    DROPPED.store(false, Ordering::SeqCst);
    let result = std::panic::catch_unwind(|| {
        let _session = TestSession;
        panic!("模拟 panic");
    });
    assert!(result.is_err());
    assert!(DROPPED.load(Ordering::SeqCst), "在 panic 时必须触发 Drop");
}
```

### 在你的代码库中的运用

以下是向工作区中添加类型层级测试的优先级计划：

| Crate | 测试类型 | 测试内容 |
|-------|-----------|-------------|
| `protocol_lib` | 编译失败 | `Session<Idle>` 无法调用 `send_command()` |
| `protocol_lib` | 属性测试 | 任意字节序列 → `TryFrom` 要么成功，要么返回 Err (绝不 panic) |
| `thermal_diag` | 编译失败 | 如果没有 `HasSpi` 混入，则无法构造 `FanReading` |
| `accel_diag` | 属性测试 | GPU 传感器解析：随机字节 → 要么被通过验证，要么被拒绝 |
| `config_loader` | 属性测试 | 随机字符串 → `DiagLevel` 的 `FromStr` 实现绝不 panic |
| `pci_topology` | 编译失败 | 在需要 `Width32` 的地方无法传入 `Register<Width16>` |
| `event_handler` | 编译失败 | 审计令牌无法由外部克隆 (Clone) |
| `diag_framework` | 编译失败 | `DerBuilder<Missing, _>` 无法调用 `finish()` |

### 零成本抽象：通过汇编代码进行证明

一个常见的担忧是：“新类型 (Newtypes) 和幽灵类型 (Phantom Types) 会增加运行时开销吗？”
答案是 **不会** —— 它们编译后的汇编代码与原始基元 (Raw Primitives) 完全一致。以下是验证方法：

**设置：**

```bash
cargo install cargo-show-asm
```

**示例：新类型 vs 原始 u32：**

```rust,ignore
// src/lib.rs
#[derive(Clone, Copy)]
pub struct Rpm(pub u32);

#[derive(Clone, Copy)]
pub struct Celsius(pub f64);

// 使用新类型进行算术运算
#[inline(never)]
pub fn add_rpm(a: Rpm, b: Rpm) -> Rpm {
    Rpm(a.0 + b.0)
}

// 使用原始类型进行算术运算 (用于对比)
#[inline(never)]
pub fn add_raw(a: u32, b: u32) -> u32 {
    a + b
}
```

**运行：**

```bash
cargo asm my_crate::add_rpm
cargo asm my_crate::add_raw
```

**结果 —— 汇编代码完全一致：**

```asm
; add_rpm (新类型)              ; add_raw (原始 u32)
my_crate::add_rpm:            my_crate::add_raw:
  lea eax, [rdi + rsi]         lea eax, [rdi + rsi]
  ret                          ret
```

`Rpm` 包装器在编译时被完全擦除了。同样的结论也适用于 `PhantomData<S>` (零字节)、`ZST` 令牌 (零字节) 以及本指南中用到的所有其他类型层级的标记。

**针对你自己的类型进行验证：**

```bash
# 显示特定函数的汇编代码
cargo asm --lib ipmi_lib::session::execute

# 证明 PhantomData 占用了零个字节
cargo asm --lib --rust ipmi_lib::session::IpmiSession
```

> **关键要点：** 本指南中的每一个模式都有 **零运行时开销**。类型系统完成了所有的工作，并在编译过程中被彻底擦除。你既拥有了 Haskell 的安全性，又获得了 C 语言的性能。

## 关键要点

1. **trybuild 测试无效代码无法编译成功** —— 这是在重构过程中维持类型层级不变式的核心手段。
2. **proptest 对验证边界进行模糊测试** —— 生成数千个随机输入以对 `TryFrom` 实现进行压力测试。
3. **RAII 验证测试 Drop 是否运行** —— 原子计数器或 mock 标记可以证明清理工作确已执行。
4. **cargo-show-asm 证明零成本特性** —— 幽灵类型、ZST 和新类型产生的汇编代码与原始 C 代码无异。
5. **为每一个“不可能”的状态增加编译失败测试** —— 如果有人不小心给一次性类型派生了 `Clone`，测试就能及时发现它。

---

*《Rust 类型驱动的正确性》全书完*
