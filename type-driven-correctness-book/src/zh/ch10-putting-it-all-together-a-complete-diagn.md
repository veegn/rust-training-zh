# 综合实战 — 构建完整的诊断平台 🟡

> **你将学到：** 全部七种核心模式（ch02–ch09）如何组合成一个单一的诊断工作流 —— 包含身份验证、会话管理、类型化命令、审计令牌、量纲化结果、已验证数据以及 Phantom Type 寄存器 —— 且总的运行时开销为零。
>
> **相关章节：** 每个核心模式章节 (ch02–ch09)，[ch14](ch14-testing-type-level-guarantees.md)（测试这些保证）

## 目标：组合工作流

我们将第 2 到第 9 章中的 **七种模式** 结合到一个真实且具有综合意义的诊断流中。我们将构建一个服务器健康检查程序，它具备：

1. **身份验证**（能力令牌 — ch04）
2. **开启 IPMI 会话**（Type-State — ch05）
3. **发送类型化命令**（类型化命令 — ch02）
4. **使用审计令牌**（单次使用类型 — ch03）
5. **返回量纲化结果**（量纲分析 — ch06）
6. **验证 FRU 数据**（已验证边界 — ch07）
7. **读取类型化寄存器**（Phantom Type — ch09）

## 综合实现

```rust
fn full_diagnostic() -> Result<(), String> {
    // 1. 身份验证 → 获得能力令牌 AdminToken
    let admin = authenticate("admin", "secret")?;

    // 2. 连接并激活会话 (Idle → Active)
    let session = Session::connect("192.168.1.100");
    let mut session = session.activate(&admin)?; // 需要 AdminToken

    // 3. 发送类型化命令 → 获得 Celsius (摄氏度)
    let temp: Celsius = session.execute(&ReadTemp { sensor_id: 0 })?;

    // 4. 读取 Phantom Type 的 u16 寄存器
    let vid: u16 = pcie.vendor_id.read(); // 保证返回 u16

    // 5. 在边界处验证 FRU 数据
    let fru = ValidFru::parse(&raw_fru)?;

    // 6. 签发单次使用的审计令牌
    let audit = AuditToken::issue(1001);

    // 7. 生成并记录审计报告，随后消耗令牌
    audit.log("诊断完成");
    // audit.log("oops"); // ❌ 编译报错：使用了已移动的值
    
    Ok(())
}
```

## 编译器能证明什么？

| Bug 类别 | 对应模式 |
|--------------|---------|
| 未经授权的访问 | 能力令牌 |
| 错误的会话状态命令 | Type-State |
| 单位混淆 (摄氏度 vs RPM) | 量纲分析 |
| 错误的响应类型 | 类型化命令 |
| 寄存器宽度不匹配 | Phantom Type |
| 处理未经验证的数据 | 已验证边界 |
| 重复的审计日志 | 单次使用类型 |

**总的运行时开销：零。**

## 关键收获

1. **七种模式无缝衔接** —— 身份验证、状态机、物理单位、命令等可以完美协同工作。
2. **零运行时开销** —— 生成的汇编代码与那些没有进行过任何检查的 C 代码一样高效。
3. **渐进式采用** —— 你不一定需要全部七种，可以按需逐步引入。
4. **设计模板** —— 将此综合工作流作为你构建属于自己的类型化诊断流的蓝图。

***
