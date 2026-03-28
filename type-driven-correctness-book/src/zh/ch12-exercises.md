# 课后练习 🟡

> **你将学到：** 在真实的硬件场景中动手实践“构造即正确”模式 —— 包括 NVMe 管理命令、固件更新状态机、传感器流水线、PCIe Phantom Type、多协议健康检查以及会话类型的诊断协议。
>
> **相关章节：** [ch02](ch02-typed-command-interfaces-request-determi.md)（练习 1）、[ch05](ch05-protocol-state-machines-type-state-for-r.md)（练习 2）、[ch06](ch06-dimensional-analysis-making-the-compiler.md)（练习 3）、[ch09](ch09-phantom-types-for-resource-tracking.md)（练习 4）、[ch10](ch10-putting-it-all-together-a-complete-diagn.md)（练习 5）

## 实践题目

### 练习 1：NVMe 管理命令 (类型化命令)
为 NVMe 管理命令设计一个类型化接口：
- `Identify` → `IdentifyResponse`
- `GetLogPage` → `SmartLog`
- `GetFeature` → 各功能对应的响应

### 练习 2：固件更新状态机 (Type-State)
模拟 BMC 固件更新的生命周期：`Idle → Uploading → Verifying → Applying → Rebooting → Complete`。
- `apply()` 必须要求一个 `VerifiedImage` 证明令牌。
- `abort()` 应当在 `Uploading` / `Verifying` 期间可用，但在 `Applying` 期间不可用（不可逆转）。

### 练习 3：传感器读取流水线 (量纲分析)
构建一个完整的流水线：`ADC → 校准 → 阈值检查 → 结果`。
- 为 `Celsius` (摄氏度)、`Volts` (伏特)、`Watts` (瓦特) 创建 Newtype。
- 在类型系统中实现 `P = V × I` (功率 = 电压 × 电流) 的算术逻辑。

### 练习 4：PCIe 能力链遍历 (Phantom Type)
为 PCIe 的能力链 (Linked List) 建模。每种能力类型 (MSI, MSI-X, PCIe 等) 都应当拥有自己基于 Phantom Type 的寄存器布局。

### 练习 5：多协议健康检查 (能力混入)
创建一个健康检查框架，包含以下混入 (Mixin)：
- `ThermalHealthMixin`（需具备 `HasIpmi + HasGpio` 能力）。
- `StorageHealthMixin`（需具备 `HasNvmeCli` 能力）。

### 练习 6：会话类型的诊断协议
设计一个诊断会话，其 `start()` 方法会签发 `N` 个执行令牌。每个 `TestToken` 在测试运行时被消耗掉，以防止重复运行相同的测试。

## 关键收获

1. **在真实协议中练习** —— NVMe 和固件更新是应用这些模式的绝佳目标。
2. **模式是可组合的** —— 现实世界的练习通常需要结合 2 到 3 种模式。
3. **静态度量是最终目标** —— 如果你能将协议违规表示为编译错误，你就已经成功了。

***
