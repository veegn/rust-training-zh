[English Original](../en/ch13-reference-card.md)

# 速查卡 🟡

> **你将学到什么？** 如何利用速查表快速查找并在项目中应用六个核心模式（Newtype、Type-state、Capability Tokens、Validated Boundaries、Phantom Types、Mixins）；如何决定在特定场景下该使用哪种模式；以及为什么提供一个清晰、简洁的决策流程图能够加速你的工程决策。

## 范围：核心模式概览

下表总结了本指南中的核心模式及其核心应用场景。

### 1. 核心模式汇总

| 模式 | 核心目的 | 关键字 |
| :--- | :--- | :--- |
| **Newtype** | 解决单位混淆、逻辑类型 | `struct MyType(u64)` |
| **Type-state** | 强制执行状态机、禁止无效调用 | `Device<Ready>`, `S: State` |
| **Capability Token** | 零成本权限证明、受控访问 | `&Token`, `ZST` |
| **Validated Boundary** | 解析，而不是事后校验 | `TryFrom`, `Authenticated` |
| **Phantom Type** | 追踪资源元数据、不占空 | `PhantomData<W>` |
| **Mixin** | 硬件能力的 Trait 约束 | `impl SupportsDMA` |

### 2. 工程决策流程图

在处理一个新问题（如 I/O 操作、内存页管理或 Redfish 客户端）时：
- 您是否需要防止单位混淆？ -> **Newtype**
- 对象是否存在互斥的生命周期状态？ -> **Type-state**
- 此操作是否需要显式授权？ -> **Capability Token**
- 此数据是否来自不受信的外部接口？ -> **Validated Boundary**
- 您是否要在类型中标记额外的硬件元数据？ -> **Phantom Type**
- 硬件型号是否具备可选功能？ -> **Mixin**

## 为什么速查卡及参考资料至关重要

1.  **快速检索 (Quick Lookup)**：在工程实践中，能够迅速找到合适的模式并应用。
2.  **减少认知开销 (Reduced Cognitive Overhead)**：将复杂的类型体系简化为具体的决策链路。
3.  **标准化实现 (Standardize Implementation)**：在团队项目中，提供一份标准的参考指南。

在为具有多种访问级别或不同物理规格的复杂系统编写统一的维护框架时，这些参考内容可以显著提升代码的鲁棒性。

***
