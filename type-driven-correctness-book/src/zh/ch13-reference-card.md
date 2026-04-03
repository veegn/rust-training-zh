[English Original](../en/ch13-reference-card.md)

# 速记卡 🟡

> **14+ 种“正确构建 (Correct-by-Construction)”模式的快速参考指南** —— 包含模式选择流程图、模式目录、组合规则、Crate 映射以及“类型即保证”速记表。
>
> **参考：** 涉及本书的每一章 —— 本页是全书的速览表。

## 快速参考：正确构建模式

### 模式选择指南

```text
遗漏了该 Bug 是否会造成灾难性后果？
├── 是 → 它能否通过类型编码来实现？
│         ├── 能 → 使用“正确构建”模式
│         └── 不能 → 运行时检查 + 详尽测试
└── 否 → 正常的运行时检查即可
```

### 模式目录

| # | 模式 | 核心特性/类型 | 防止的错误 | 运行时开销 | 章节 |
|---|---------|---------------|----------|:------:|---------|
| 1 | 类型化命令 | `trait IpmiCmd { type Response; }` | 错误的响应类型 | 零 | ch02 |
| 2 | 一次性类型 | `struct Nonce` (无 Clone/Copy) | Nonce/密钥被重复使用 | 零 | ch03 |
| 3 | 能力令牌 | `struct AdminToken { _private: () }` | 未经授权的访问 | 零 | ch04 |
| 4 | 类型状态 | `Session<Active>` | 违反协议约定 | 零 | ch05 |
| 5 | 维度类型 | `struct Celsius(f64)` | 单位混淆 | 零 | ch06 |
| 6 | 验证边界 | `struct ValidFru` (通过 TryFrom) | 使用未经验证的数据 | 仅一次解析 | ch07 |
| 7 | 能力混入 | `trait FanDiagMixin: HasSpi + HasI2c` | 缺失总线访问权限 | 零 | ch08 |
| 8 | 幽灵类型 | `Register<Width16>` | 位宽/方向不匹配 | 零 | ch09 |
| 9 | 哨兵 → Option | `Option<u8>` (而非 `0xFF`) | 将哨兵值当作普通值的 Bug | 零 | ch11 |
| 10 | 密封特性 | `trait Cmd: private::Sealed` | 不健壮的外部实现 | 零 | ch11 |
| 11 | 非穷尽枚举 | `#[non_exhaustive] enum Sku` | match 分支的静默漏接 | 零 | ch11 |
| 12 | 类型状态构建器 | `DerBuilder<Set, Missing>` | 对象构建不完整 | 零 | ch11 |
| 13 | FromStr 验证 | `impl FromStr for DiagLevel` | 未经验证的字符串输入 | 仅一次解析 | ch11 |
| 14 | 常量泛型大小 | `RegisterBank<const N: usize>` | 缓冲区大小不匹配 | 零 | ch11 |
| 15 | 安全的 `unsafe` 封装 | `MmioRegion::read_u32()` | 未检查的 MMIO/FFI | 零 | ch11 |
| 16 | 异步类型状态 | `AsyncSession<Active>` | 异步协议违规 | 零 | ch11 |
| 17 | 常量断言 | `SdrSensorId<const N: u8>` | 无效的编译时 ID | 零 | ch11 |
| 18 | 会话类型 | `Chan<SendRequest>` | 信道操作顺序错误 | 零 | ch11 |
| 19 | Pin 与自引用 | `Pin<Box<StreamParser>>` | 悬空的结构体内部指针 | 零 | ch11 |
| 20 | RAII / Drop | `impl Drop for Session` | 任何出口路径下的资源泄露 | 零 | ch11 |
| 21 | 错误类型层级 | `#[derive(Error)] enum DiagError` | 错误被静默吞掉 | 零 | ch11 |
| 22 | `#[must_use]` | `#[must_use] struct Token` | 数值被静默丢弃 | 零 | ch11 |

### 组合规则

```text
能力令牌 + 类型状态 = 已授权的状态转换
类型化命令 + 维度类型 = 具备物理含义类型的响应
验证边界 + 幽灵类型 = 针对已验证配置的类型化寄存器访问
能力混入 + 类型化命令 = 总线感知的类型化操作
一次性类型 + 类型状态 = 转换即消费的协议约定
密封特性 + 类型化命令 = 封闭且鲁棒的命令集
哨兵 → Option + 验证边界 = 清晰的一次性解析流水线
类型状态构建器 + 能力令牌 = 构建完备性的证明
FromStr + #[non_exhaustive] = 可演进、强制快速失败的枚举解析
常量泛型大小 + 验证边界 = 定长且已验证的协议缓冲区
安全的 unsafe 封装 + 幽灵类型 = 类型化且安全的 MMIO 访问
异步类型状态 + 能力令牌 = 已授权的异步转换
会话类型 + 类型化命令 = 完全类型化的“请求-响应”信道
Pin + 类型状态 = 无法移动的自引用状态机
RAII (Drop) + 类型状态 = 依赖状态的清理保证
错误层级 + 验证边界 = 具备详尽处理机制的类型化解析错误
#[must_use] + 一次性类型 = 难以忽略、难以重用的令牌
```

### 应避免的反模式

| 反模式 | 为什么它是错的 | 正确的替代方案 |
|-------------|---------------|-------------------|
| `fn read_sensor() -> f64` | 无单位 —— 可能是 °C, °F 或 RPM | `fn read_sensor() -> Celsius` |
| `fn encrypt(nonce: &[u8; 12])` | Nonce 可能会被重复使用（通过借用） | `fn encrypt(nonce: Nonce)` (通过移动) |
| `fn admin_op(is_admin: bool)` | 调用者可以撒谎 (传 `true`) | `fn admin_op(_: &AdminToken)` |
| `fn send(session: &Session)` | 无状态保证 | `fn send(session: &Session<Active>)` |
| `fn process(data: &[u8])` | 未经验证 | `fn process(data: &ValidFru)` |
| 对临时密钥派生 `Clone` | 破坏了一次性使用的保证 | 不要派生 (derive) Clone |
| `let vendor_id: u16 = 0xFFFF` | 哨兵值在内部传递 | `let vendor_id: Option<u16> = None` |
| 带默认回退逻辑的 `fn route(level: &str)` | 拼写错误会被静默忽略 | `let level: DiagLevel = s.parse()?` |
| 缺少字段也能 `Builder::new().finish()` | 构建出的对象不完整 | 类型状态构建器：`finish()` 挂钩在 `Set` 状态上 |
| 为定长硬件缓冲区使用 `let buf: Vec<u8>` | 大小仅在运行时检查 | `RegisterBank<4096>` (常量泛型) |
| 散落在处的原始 `unsafe { ptr::read(...) }` | 有未定义行为 (UB) 风险，无法审计 | `MmioRegion::read_u32()` 安全封装 |
| 使用 `async fn transition(&mut self)` | 可变借用无法强制达成状态变更 | `async fn transition(self) -> NextState` |
| 手动调用 `fn cleanup()` | 在提前返回或 panic 时会被遗忘 | `impl Drop` —— 编译器会自动插入调用 |
| `fn op() -> Result<T, String>` | 错误信息不透明，无法进行变体匹配 | `fn op() -> Result<T, DiagError>` 枚举 |

### 在诊断代码库中的映射

| 模块 | 适用的模式 |
|---------------------|----------------------|
| `protocol_lib` | 类型化命令、类型状态会话 |
| `thermal_diag` | 能力混入、维度类型 |
| `accel_diag` | 验证边界、幽灵寄存器 |
| `network_diag` | 类型状态 (链路训练)、能力令牌 |
| `pci_topology` | 幽灵类型 (寄存器位宽)、已验证配置、哨兵 → Option |
| `event_handler` | 一次性审计令牌、能力令牌、FromStr (Component) |
| `event_log` | 验证边界 (SEL 记录解析) |
| `compute_diag` | 维度类型 (温度、频率) |
| `memory_diag` | 验证边界 (SPD 数据)、维度类型 |
| `switch_diag` | 类型状态 (端口枚举)、幽灵类型 |
| `config_loader` | FromStr (DiagLevel, FaultStatus, DiagAction) |
| `log_analyzer` | 验证边界 (CompiledPatterns) |
| `diag_framework` | 类型状态构建器 (DerBuilder)、会话类型 (编排器 ↔ 工作线程) |
| `topology_lib` | 常量泛型寄存器组、安全 MMIO 封装 |

### 类型即保证 —— 快速映射

| 保证 | Rust 等效实现 | 示例 |
|-----------|----------------|---------|
| “该证明存在” | 一个类型 | `AdminToken` |
| “我持有该证明” | 该类型的一个数值 | `let tok = authenticate()?;` |
| “由 A 推导出 B” | 函数 `fn(A) -> B` | `fn activate(AdminToken) -> Session<Active>` |
| “A 且 B 同时成立” | 元组 `(A, B)` 或多参数函数 | `fn op(a: &AdminToken, b: &LinkTrained)` |
| “A 或 B 其中之一成立” | `enum { A(A), B(B) }` 或 `Result<A, B>` | `Result<Session<Active>, Error>` |
| “始终为真” | 单元类型 `()` (unit type) | 始终可构造 |
| “不可能发生” | never 类型 `!` 或 `enum Void {}` | 永远无法被构造 |

***
