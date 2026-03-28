# 测试类型层面的保证 🟡

> **你将学到：** 如何测试非法代码无法通过编译 (`trybuild`)、对已验证边界进行模糊测试 (`proptest`)、验证 RAII 不变量以及通过 `cargo-show-asm` 证明零成本抽象。
>
> **相关章节：** [ch03](ch03-single-use-types-cryptographic-guarantee.md)（Nonce）、[ch05](ch05-protocol-state-machines-type-state-for-r.md)（会话）、[ch07](ch07-validated-boundaries-parse-dont-validate.md) (proptest)

## 测试“不可能”的情况

“构造即正确”模式将 bug 转化为了编译错误。但是，我们如何确保这些不变量在项目重构后依然有效且在强制执行呢？

### 1. 使用 `trybuild` 进行编译失败测试
[`trybuild`](https://crates.io/crates/trybuild) 允许你断言某些非法的代码 **不应通过编译**。

```rust
// 在测试文件中：
let t = trybuild::TestCases::new();
t.compile_fail("tests/ui/nonce_reuse.rs");
```

如果 `nonce_reuse.rs` 中的代码（例如尝试两次使用单次使用令牌）竟然通过了编译，那么该测试就会失败。

### 2. 使用 `proptest` 进行基于属性的测试
已验证边界 (ch07) 只解析一次数据。使用 [`proptest`](https://crates.io/crates/proptest) 生成上千个随机输入，以确保你的 `TryFrom` 实现对于垃圾数据既不会恐慌 (panic)，也能正确地予以拒绝。

### 3. 使用 `cargo-show-asm` 证明零成本
为了证明 Newtype 和 Phantom Type 具有零运行时开销，可以使用 `cargo-show-asm` 来检视生成的汇编代码。你会看到 `Celsius(f64)` 与原始的 `f64` 产生的机器码是完全相同的。

## 正确性测试金字塔

1. **编译失败测试 (trybuild)**：“非法代码绝不能通过编译。”
2. **属性测试 (proptest)**：“合法输入绝不能引起恐慌。”
3. **单元测试 (#[test])**：“具体的逻辑按预期执行。”
4. **类型系统**：“整类 bug 在结构上是不可能存在的。”

## 关键收获

1. **`trybuild` 维护不变量** —— 捕获由于疏忽而在单次使用类型上实现 `Clone` 的行为。
2. **`proptest` 强化边界** —— 确保验证逻辑对于模糊测试具备鲁棒性。
3. **汇编代码不会撒谎** —— `cargo-show-asm` 证明了所有的类型层面标记都是零运行时成本的。
4. **测试不可能的情况** —— 如果一个状态被认为是“不可能的”，那么就编写一个尝试进入该状态并会在编译期报错的测试。

***
