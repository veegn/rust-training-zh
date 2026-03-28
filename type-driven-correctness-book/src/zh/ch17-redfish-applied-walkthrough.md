# 综合实战 —— 类型安全的 Redfish 客户端 🟡

> **你将学到：** 如何将 Type-state 会话、能力令牌、基于 Phantom Type 的资源导航、量纲分析、已验证边界、Builder Type-state 以及单次使用类型组合成一个完整的、零开销的 Redfish 客户端 —— 在这里，任何协议违规都会导致编译错误。
>
> **相关章节：** [ch02](ch02-typed-command-interfaces-request-determi.md)（类型化命令）、[ch04](ch04-capability-tokens-zero-cost-proof-of-aut.md)（能力令牌）、[ch05](ch05-protocol-state-machines-type-state-for-r.md) (type-state)、[ch06](ch06-dimensional-analysis-making-the-compiler.md)（量纲分析）、[ch07](ch07-validated-boundaries-parse-dont-validate.md)（已验证边界）、[ch09](ch09-phantom-types-for-resource-tracking.md) (phantom types)。

## 为什么选择 Redfish 做实战

Redfish 是一套用于硬件管理的 RESTful API。虽然它应用广泛，但充满了正确性隐患：错误的 URI 拼接、缺失权限检查、以及 JSON 格式遥测数据中的单位混淆。

## 第一节：会话生命周期 (Type-State)

将连接的生命周期编码进类型系统：`Disconnected → Connected → Authenticated → Closed`。
- 请求 **只能** 在 `Authenticated` (已认证) 状态的会话上发送。
- `logout()` 方法会消耗会话对象，防止被后续错误重用。

## 第二节：权限令牌 (能力令牌)

使用零大小的证明令牌来管理 Redfish 权限：`LoginToken`、`ConfigureComponentsToken`、`ConfigureManagerToken`。
- `login()` 方法根据用户的角色选择性地签发相应令牌。
- `set_boot_order()` 方法要求必须传入 `ConfigureComponentsToken` 作为参数。

## 第三节：类型化的资源导航 (Phantom Type)

将 Redfish 资源树（如 `ServiceRoot → ChassisCollection → ChassisInstance → Thermal`）表示为类型。
- 导航方法（如 `.chassis()`）会返回由资源标记修饰的路径对象。
- 这样可以防止构造出非法的 URI，比如 `.../Chassis/1/Bios`（因为 BIOS 资源位于 Systems 下，而非 Chassis 下）。

## 第四节：类型化的遥测数据 (量纲分析)

将 Redfish JSON 解析为包含 `Celsius` (摄氏度) 或 `Watts` (瓦特) 等量纲类型的已验证结构体。编译器将拒绝把温度读数与 RPM 值进行比较。

## 关键收获

1. **组合即力量** —— 将 5 种以上的模式组合在一起，可以创建一个极度“坚固”的客户端。
2. **零成本抽象** —— 所有的检查（令牌、状态、phantom type）在编译期均会被抹除。
3. **将假设形式化** —— REST API 往往定义松散；类型系统强制你明确定义什么是“已认证”或“有效的遥测读数”。

***
