[English Original](../en/ch12-exercises.md)

# 练习 🟡

> **你将学到什么？** 如何在实际场景中应用所学的六个关键模式（Newtype、Type-state、Capability Tokens、Validated Boundaries、Phantom Types、Mixins）；如何通过动手实践加深对类型驱动开发的理解；以及为什么这些练习能够帮助你掌握在编译期捕捉 Bug 的能力。

## 练习范围：综合实战

我们将完成六个具有实际背景的练习，每个练习都侧重于一个或多个关键概念。

### 1. 动手练习 1：Newtype 与量纲分析

定义一个 `Temperature` 类型，支持 `Celsius` 和 `Fahrenheit`：
- 禁止直接将摄氏度与华氏度相加；
- 实现 `From` 转换特征使之能互相转换。

### 2. 动手练习 2：Type-state 与连接管理器

实现一个 `Connection<S>` 状态机，包含三个阶段：`Disconnected`、`Handshaking`、`Established`：
- `send_data` 只能在 `Established` 状态下被调用；
- `disconnect` 应该消耗连接对象并返回一个新的 `Disconnected` 状态。

### 3. 动手练习 3：Capability Tokens 与硬件访问

定义一个 ZST（零大小类型）`ResetPrivilege`：
- 只有通过 `authenticate_admin` 获取到该令牌后，才能调用 `hard_reset` 函数。

### 4. 动手练习 4：Validated Boundaries 与 IP 解析

实现一个 `IpAddress` 结构体：
- 提供一个 `parse_ipv4(raw: &str) -> Result<IpAddress, Error>` 函数。
- 只有在解析成功的 `IpAddress` 对象上，才能执行 `connect_to_server`。

### 5. 动手练习 5：Phantom Types 与读写锁

利用 `PhantomData` 定义一个 `Lock<Mode>`：
- `ReadOnly` 与 `ReadWrite` 型的 `Lock` 拥有不同的功能集合。

### 6. 动手练习 6：综合练习：一个简化的 PCIe 驱动

综合使用上述所有模式，设计一个 `PCIeDevice`：
- 包含宽度、状态标识、访问权限。
- 所有的错误配置（如在 16bit 模式下写入 32bit 数据）应在编译期报错。

## 为什么进行这些练习至关重要

1.  **内化模式 (Internalize Patterns)**：通过手动编写代码，将理论知识转化为实际的编码肌肉记忆。
2.  **强制遵守规范 (Enforced Policy)**：在尝试编码时遇到编译器报错，是学习“如何将运行时检查推向编译期”的最佳途径。
3.  **高性能 (High Performance)**：你会注意到所有的抽象在生成目标代码后，都没有产生任何逻辑上的额外开销。

在完成这些练习并查看参考答案后，你将对如何在自己的项目中建立强一致性的类型驱动防护有深刻的理解。

***
