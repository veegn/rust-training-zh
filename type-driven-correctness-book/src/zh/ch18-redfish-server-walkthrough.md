# 综合实战 —— 类型安全的 Redfish 服务端 🟡

> **你将学到：** 如何将响应构造器 (Response Builder) Type-state、数据源可用性令牌 (Source-availability tokens)、量纲序列化、健康状态汇总 (Health rollup)、架构版本化以及类型化的动作分发组合成一个 Redfish 服务端，它 **绝对无法产生不符合 Schema 的响应** —— 这是 [ch17](ch17-redfish-applied-walkthrough.md) 中客户端实战的镜像。
>
> **相关章节：** [ch02](ch02-typed-command-interfaces-request-determi.md)（类型化命令）、[ch04](ch04-capability-tokens-zero-cost-proof-of-aut.md)（能力令牌）、[ch06](ch06-dimensional-analysis-making-the-compiler.md)（量纲分析）、[ch07](ch07-validated-boundaries-parse-dont-validate.md)（已验证边界）、[ch08](ch08-capability-mixins-compile-time-hardware-.md) (mixins)、[ch17](ch17-redfish-applied-walkthrough.md) (客户端对等章节)。

## 镜像问题：发送有效数据

[ch17](ch17-redfish-applied-walkthrough.md) 讨论的是“如何正确地 **消费** Redfish 数据”，而本章讨论的是其镜像问题：“如何正确地 **产生** Redfish 数据？”

在客户端，风险在于 **信任了** 错误的数据。在服务端，风险则在于 **发出了** 错误的数据 —— 且整个集群中的每一个客户端都会盲目信任你发送的内容。

一个单一的 `GET /redfish/v1/Systems/1` 响应必须融合来自多个源（SMBIOS, PCIe, IPMI, 传感器等）的数据，并整理成单一且符合 Schema 的 JSON。

## 第一节：响应构造器 (Type-State)

使用 Builder 模式来构建 Redfish 响应。将 `.build()` 方法的可用性锚定在是否已提供 Schema 要求的 **所有必选字段**（如 `Name`、`UUID`、`PowerState`）上。
- 如果缺少必选字段，代码将 **无法通过编译**。
- 这彻底消除了在基于 C 语言实现的 Redfish 服务端中常见的“遗漏某个字段”类 bug。

## 第二节：数据源可用性令牌

使用零大小的证明令牌（例如 `SmbiosReady`、`SensorsReady`）在查询子系统之前证明该子系统已成功初始化。
- 只有在持有 `SmbiosReady` 令牌时，从 SMBIOS 填充 Builder 字段的方法才变为可用。
- 这可以防止空指针引用或对尚未初始化的硬件进行无效查询。

## 第三节：量纲化的序列化

在响应结构体中使用量纲类型（如 `reading_celsius: Celsius`）定义字段。这可以防止在代码编写过程中，由于疏忽将 RPM 值序列化到摄氏度 (Celsius) JSON 字段中。

## 第四节：作为类型化折叠的健康汇总 (Health Rollup)

Redfish 的 `Status.Health` 必须汇总所有子组件中最差的健康状态。
- 使用具有 `Ord` (OK < Warning < Critical) 顺序的枚举。
- 在组件健康状态集合上进行简单的 `.max()` 折叠 (fold)，即可得到一个在逻辑上可证明正确的汇总结果。

## 关键收获

1. **构造即正确，而非序列化即正确** —— 让类型系统来保证你的 JSON 完全符合架构规范。
2. **可用性即证明** —— 证明令牌取代了运行时的“是否已初始化”检查。
3. **服务端是“真相之源”** —— 服务端的一个逻辑 bug 会波及所有客户端。在服务端进行类型层面的强制约束，是你能做的投入产出比最高的正确性投资。

***
