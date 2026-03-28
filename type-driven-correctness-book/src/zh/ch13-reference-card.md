# 模式速查表 🟢

> **所有 14+ 种“构造即正确”模式的快速参考指南**，包含选择流程图、模式列表、组合规则、模块映射以及类型作为保证 (Types-as-Guarantees) 的对照表。
>
> **相关章节：** 每一章 —— 本页是整本书的查阅表。

## 快速参考表

| # | 模式 | 预防的 Bug 类目 | 章节 |
|---|---------|----------|---------|
| 1 | 类型化命令 (Typed Commands) | 错误的响应类型 | ch02 |
| 2 | 单次使用类型 (Single-Use Types) | Nonce/密钥重复使用 | ch03 |
| 3 | 能力令牌 (Capability Tokens) | 未经授权的访问 | ch04 |
| 4 | 状态机 (Type-State) | 协议顺序违规 | ch05 |
| 5 | 量纲类型 (Dimensional Types) | 物理单位混淆 | ch06 |
| 6 | 已验证边界 (Validated Boundaries) | 使用未经验证的数据 | ch07 |
| 7 | 能力混入 (Capability Mixins) | 缺失总线访问权限 | ch08 |
| 8 | Phantom Type | 宽度/方向不匹配 | ch09 |
| 9 | 哨兵值 → Option | 哨兵值误当数据处理 | ch11 |
| 10| 密封 trait (Sealed Traits) | 不安全的外部实现 | ch11 |

## 模式组合示例

- **能力令牌 + Type-State**：授信的状态转换。
- **类型化命令 + 量纲类型**：具有物理含义的响应。
- **已验证边界 + Phantom Type**：对已验证配置映射的类型化寄存器访问。

## 反模式与重构

| 反模式 | 正确的替代方案 |
|--------------|-------------------|
| `fn read() -> f64` | `fn read() -> Celsius` (量纲类型) |
| `fn op(is_admin: bool)` | `fn op(_: &AdminToken)` (能力令牌) |
| `fn send(session: &Session)` | `fn send(session: &Session<Active>)` (状态机) |

## 类型作为保证的映射关系

- **“证明存在”**：一个类型。
- **“我持有证明”**：该类型的一个值。
- **“A 意味着 B”**：函数 `fn(A) -> B`。
- **“A 和 B 同时成立”**：元组 `(A, B)`。
- **“A 或 B 其中之一”**：枚举 `enum { A, B }` 或 `Result<A, B>`。

***
