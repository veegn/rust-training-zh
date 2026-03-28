<div style="background-color: #d9d9d9; padding: 16px; border-radius: 6px; color: #000000;">

**许可证** 本项目采用双重许可：[MIT License](LICENSE) 与 [Creative Commons Attribution 4.0 International (CC-BY-4.0)](LICENSE-DOCS)。

</div>

<div style="background-color: #d9d9d9; padding: 16px; border-radius: 6px; color: #000000;">

**商标** 本项目可能包含与项目、产品或服务相关的商标或标识。经授权使用 Microsoft 商标或标识时，必须遵循 [Microsoft 商标与品牌指南](https://www.microsoft.com/en-us/legal/intellectualproperty/trademarks/usage/general)。在本项目修改版本中使用 Microsoft 商标或标识时，不得造成混淆，也不得暗示 Microsoft 提供背书。任何第三方商标或标识的使用均受其各自政策约束。

</div>

# Rust 培训书籍

本项目包含七套 Rust 培训课程，面向不同编程背景的读者，同时覆盖 async、进阶模式以及工程实践等深入主题。

这些内容结合了原创讲解以及来自 Rust 生态优质资料的启发和示例。目标是构建一套深入、技术准确、教学结构完整的课程，把分散在书籍、博客、会议演讲和视频系列中的知识整合成连贯的学习体验。

> **免责声明:** 这些书籍是培训材料，并非权威参考文档。尽管我们尽力保证准确性，但对于关键细节，仍建议始终对照 [Rust 官方文档](https://doc.rust-lang.org/) 和 [Rust Reference](https://doc.rust-lang.org/reference/) 进行核实。

### 灵感来源与致谢

- [**The Rust Programming Language**](https://doc.rust-lang.org/book/) — 一切内容的重要基础
- [**Jon Gjengset**](https://www.youtube.com/c/JonGjengset) — 高级 Rust 内部机制深度直播与 `Crust of Rust` 系列
- [**withoutboats**](https://without.boats/blog/) — async 设计、`Pin` 与 futures 模型
- [**fasterthanlime (Amos)**](https://fasterthanli.me/) — 从第一性原理讲系统编程的长篇深度内容
- [**Mara Bos**](https://marabos.nl/) — *Rust Atomics and Locks* 以及并发原语相关内容
- [**Aleksey Kladov (matklad)**](https://matklad.github.io/) — Rust Analyzer 洞见、API 设计与错误处理模式
- [**Niko Matsakis**](https://smallcultfollowing.com/babysteps/) — 语言设计、借用检查器内部机制与 Polonius
- [**Rust by Example**](https://doc.rust-lang.org/rust-by-example/) 和 [**Rustonomicon**](https://doc.rust-lang.org/nomicon/) — 实用模式与 unsafe 深入讲解
- [**This Week in Rust**](https://this-week-in-rust.org/) — 社区动态启发了大量示例
- ……以及更多 **Rust 社区**中的贡献者，他们的博客、演讲、RFC 和论坛讨论共同塑造了本材料，无法一一列举，在此一并致谢

## 开始阅读

选择与你背景最匹配的书籍。各书按复杂度分组，方便你规划学习路径：

| 级别 | 说明 |
|-------|-------------|
| 🟢 **桥接** | 从其他语言迁移到 Rust，建议从这里开始 |
| 🔵 **深潜** | 聚焦 Rust 关键子系统的专题讲解 |
| 🟡 **进阶** | 面向有经验 Rust 开发者的模式与技巧 |
| 🟣 **专家** | 前沿的类型层与正确性技术 |
| 🟤 **实践** | 工程化、工具链与生产准备 |

| 书籍 | 级别 | 适合人群 |
|------|-------|-------------|
| [**Rust for C/C++ Programmers**](c-cpp-book/src/SUMMARY.md) | 🟢 桥接 | 移动语义、RAII、FFI、嵌入式、`no_std` |
| [**Rust for C# Programmers**](csharp-book/src/SUMMARY.md) | 🟢 桥接 | 从 Swift、C#、Java 过渡到所有权与类型系统 |
| [**Rust for Python Programmers**](python-book/src/SUMMARY.md) | 🟢 桥接 | 从动态类型到静态类型、无 GIL 并发 |
| [**Async Rust**](async-book/src/SUMMARY.md) | 🔵 深潜 | Tokio、流、取消安全 |
| [**Rust Patterns**](rust-patterns-book/src/SUMMARY.md) | 🟡 进阶 | `Pin`、分配器、无锁结构、unsafe |
| [**Type-Driven Correctness**](type-driven-correctness-book/src/SUMMARY.md) | 🟣 专家 | Type-State、Phantom Type、能力令牌 |
| [**Rust Engineering Practices**](engineering-book/src/SUMMARY.md) | 🟤 实践 | 构建脚本、交叉编译、CI/CD、Miri |

每本书包含 15 到 16 章，配有 Mermaid 图示、可编辑 Rust Playground、练习题以及全文搜索能力。

> **提示:** 你可以直接在 GitHub 上阅读 Markdown 源文件，也可以在 [GitHub Pages 站点](https://microsoft.github.io/RustTraining/) 上使用侧边栏导航和搜索浏览渲染后的内容。
>
> **本地预览:** 为获得最佳阅读体验（章节间键盘导航、即时搜索、离线访问），请克隆仓库并执行：
> ```
> # 如果尚未安装 Rust，请先通过 rustup 安装：
> # https://rustup.rs/
>
> cargo install mdbook@0.4.52 mdbook-mermaid@0.14.0
> cargo xtask serve          # 构建所有书籍并启动本地服务
> ```

---

## 维护者说明

<details>
<summary>本地构建、预览和编辑书籍</summary>

### 前置条件

如果尚未安装，请先通过 [**rustup** 安装 Rust](https://rustup.rs/)，然后执行：

```bash
cargo install mdbook@0.4.52 mdbook-mermaid@0.14.0
```

### 构建与预览

```bash
cargo xtask build               # 构建所有书到 site/（本地预览）
cargo xtask serve               # 构建并在 http://localhost:3000 提供服务
cargo xtask deploy              # 构建所有书到 docs/（用于 GitHub Pages）
cargo xtask clean               # 删除 site/ 和 docs/
```

若只想构建或预览单本书：

```bash
cd c-cpp-book && mdbook serve --open    # http://localhost:3000
```

### 部署

站点会在代码推送到 `main` 后通过 `.github/workflows/pages.yml` 自动部署到 GitHub Pages，无需手动操作。

</details>
