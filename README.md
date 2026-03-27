<div style="background-color: #d9d9d9; padding: 16px; border-radius: 6px; color: #000000;">

**License / 许可证** This project is dual-licensed under the [MIT License](LICENSE) and [Creative Commons Attribution 4.0 International (CC-BY-4.0)](LICENSE-DOCS).

本项目采用双重许可：[MIT License](LICENSE) 与 [Creative Commons Attribution 4.0 International (CC-BY-4.0)](LICENSE-DOCS)。

</div>

<div style="background-color: #d9d9d9; padding: 16px; border-radius: 6px; color: #000000;">

**Trademarks / 商标** This project may contain trademarks or logos for projects, products, or services. Authorized use of Microsoft trademarks or logos is subject to and must follow [Microsoft's Trademark & Brand Guidelines](https://www.microsoft.com/en-us/legal/intellectualproperty/trademarks/usage/general). Use of Microsoft trademarks or logos in modified versions of this project must not cause confusion or imply Microsoft sponsorship. Any use of third-party trademarks or logos are subject to those third-party's policies.

本项目可能包含与项目、产品或服务相关的商标或标识。经授权使用 Microsoft 商标或标识时，必须遵循 [Microsoft 商标与品牌指南](https://www.microsoft.com/en-us/legal/intellectualproperty/trademarks/usage/general)。在本项目修改版本中使用 Microsoft 商标或标识时，不得造成混淆，也不得暗示 Microsoft 提供背书。任何第三方商标或标识的使用均受其各自政策约束。

</div>

# Rust Training Books / Rust 培训书籍

Seven training courses covering Rust from different programming backgrounds, plus deep-dives on async, advanced patterns, and engineering practices.

本仓库包含七套 Rust 培训课程，面向不同编程背景的读者，同时覆盖 async、进阶模式以及工程实践等深入主题。

This material combines original content with ideas and examples inspired by some of the best resources in the Rust ecosystem. The goal is to present an in-depth, technically accurate curriculum that weaves together knowledge scattered across books, blogs, conference talks, and video series into a cohesive, pedagogically structured experience.

这些内容结合了原创讲解以及来自 Rust 生态优质资料的启发和示例。目标是构建一套深入、技术准确、教学结构完整的课程，把分散在书籍、博客、会议演讲和视频系列中的知识整合成连贯的学习体验。

> **Disclaimer / 免责声明:** These books are training material, not an authoritative reference. While we strive for accuracy, always verify critical details against the [official Rust documentation](https://doc.rust-lang.org/) and the [Rust Reference](https://doc.rust-lang.org/reference/).
>
> 这些书籍是培训材料，并非权威参考文档。尽管我们尽力保证准确性，但对于关键细节，仍建议始终对照 [Rust 官方文档](https://doc.rust-lang.org/) 和 [Rust Reference](https://doc.rust-lang.org/reference/) 进行核实。

### Inspirations & Acknowledgments / 灵感来源与致谢

- [**The Rust Programming Language**](https://doc.rust-lang.org/book/) — the foundation everything builds on / 一切内容的重要基础
- [**Jon Gjengset**](https://www.youtube.com/c/JonGjengset) — deep-dive streams on advanced Rust internals, `Crust of Rust` series / 高级 Rust 内部机制深度直播与 `Crust of Rust` 系列
- [**withoutboats**](https://without.boats/blog/) — async design, `Pin`, and the futures model / async 设计、`Pin` 与 futures 模型
- [**fasterthanlime (Amos)**](https://fasterthanli.me/) — systems programming from first principles, engaging long-form explorations / 从第一性原理讲系统编程的长篇深度内容
- [**Mara Bos**](https://marabos.nl/) — *Rust Atomics and Locks*, concurrency primitives / *Rust Atomics and Locks* 以及并发原语相关内容
- [**Aleksey Kladov (matklad)**](https://matklad.github.io/) — Rust analyzer insights, API design, error handling patterns / Rust Analyzer 洞见、API 设计与错误处理模式
- [**Niko Matsakis**](https://smallcultfollowing.com/babysteps/) — language design, borrow checker internals, Polonius / 语言设计、借用检查器内部机制与 Polonius
- [**Rust by Example**](https://doc.rust-lang.org/rust-by-example/) and [**Rustonomicon**](https://doc.rust-lang.org/nomicon/) — practical patterns and unsafe deep-dives / 实用模式与 unsafe 深入讲解
- [**This Week in Rust**](https://this-week-in-rust.org/) — community discoveries that shaped many examples / 社区动态启发了大量示例
- …and many others in the **Rust community at large** whose blog posts, conference talks, RFCs, and forum discussions have informed this material — too numerous to list individually, but deeply appreciated
- ……以及更多 **Rust 社区**中的贡献者，他们的博客、演讲、RFC 和论坛讨论共同塑造了本材料，无法一一列举，在此一并致谢

## Start Reading / 开始阅读

Pick the book that matches your background. Books are grouped by complexity so you can chart a learning path:

选择与你背景最匹配的书籍。各书按复杂度分组，方便你规划学习路径：

| Level / 级别 | Description / 说明 |
|-------|-------------|
| 🟢 **Bridge / 桥接** | Learn Rust coming from another language — start here / 从其他语言迁移到 Rust，建议从这里开始 |
| 🔵 **Deep Dive / 深潜** | Focused exploration of a major Rust subsystem / 聚焦 Rust 关键子系统的专题讲解 |
| 🟡 **Advanced / 进阶** | Patterns and techniques for experienced Rustaceans / 面向有经验 Rust 开发者的模式与技巧 |
| 🟣 **Expert / 专家** | Cutting-edge type-level and correctness techniques / 前沿的类型层与正确性技术 |
| 🟤 **Practices / 实践** | Engineering, tooling, and production readiness / 工程化、工具链与生产准备 |

| Book / 书籍 | Level / 级别 | Who it's for / 适合人群 |
|------|-------|-------------|
| [**Rust for C/C++ Programmers**](c-cpp-book/src/SUMMARY.md) | 🟢 Bridge / 桥接 | Move semantics, RAII, FFI, embedded, no_std / 移动语义、RAII、FFI、嵌入式、`no_std` |
| [**Rust for C# Programmers**](csharp-book/src/SUMMARY.md) | 🟢 Bridge / 桥接 | Swift / C# / Java → ownership & type system / 从 Swift、C#、Java 过渡到所有权与类型系统 |
| [**Rust for Python Programmers**](python-book/src/SUMMARY.md) | 🟢 Bridge / 桥接 | Dynamic → static typing, GIL-free concurrency / 从动态类型到静态类型、无 GIL 并发 |
| [**Async Rust**](async-book/src/SUMMARY.md) | 🔵 Deep Dive / 深潜 | Tokio, streams, cancellation safety / Tokio、流、取消安全 |
| [**Rust Patterns**](rust-patterns-book/src/SUMMARY.md) | 🟡 Advanced / 进阶 | Pin, allocators, lock-free structures, unsafe / `Pin`、分配器、无锁结构、unsafe |
| [**Type-Driven Correctness**](type-driven-correctness-book/src/SUMMARY.md) | 🟣 Expert / 专家 | Type-state, phantom types, capability tokens / Type-State、Phantom Type、能力令牌 |
| [**Rust Engineering Practices**](engineering-book/src/SUMMARY.md) | 🟤 Practices / 实践 | Build scripts, cross-compilation, CI/CD, Miri / 构建脚本、交叉编译、CI/CD、Miri |

Each book has 15–16 chapters with Mermaid diagrams, editable Rust playgrounds, exercises, and full-text search.

每本书包含 15 到 16 章，配有 Mermaid 图示、可编辑 Rust Playground、练习题以及全文搜索能力。

> **Tip / 提示:** You can read the markdown source directly on GitHub, or browse the rendered site with sidebar navigation and search at the [GitHub Pages site](https://microsoft.github.io/RustTraining/).
>
> 你可以直接在 GitHub 上阅读 Markdown 源文件，也可以在 [GitHub Pages 站点](https://microsoft.github.io/RustTraining/) 上使用侧边栏导航和搜索浏览渲染后的内容。
>
> **Local serving / 本地预览:** For the best reading experience (keyboard navigation between chapters, instant search, offline access), clone the repo and run:
>
> 为获得最佳阅读体验（章节间键盘导航、即时搜索、离线访问），请克隆仓库并执行：
> ```
> # Install Rust via rustup if you don't have it yet:
> # 如果尚未安装 Rust，请先通过 rustup 安装：
> # https://rustup.rs/
>
> cargo install mdbook@0.4.52 mdbook-mermaid@0.14.0
> cargo xtask serve          # builds all books and opens a local server / 构建所有书籍并启动本地服务
> ```

---

## For Maintainers / 维护者说明

<details>
<summary>Building, serving, and editing the books locally / 本地构建、预览和编辑书籍</summary>

### Prerequisites / 前置条件

Install [Rust via **rustup**](https://rustup.rs/) if you haven't already, then:

如果尚未安装，请先通过 [**rustup** 安装 Rust](https://rustup.rs/)，然后执行：

```bash
cargo install mdbook@0.4.52 mdbook-mermaid@0.14.0
```

### Build & serve / 构建与预览

```bash
cargo xtask build               # Build all books into site/ (local preview) / 构建所有书到 site/（本地预览）
cargo xtask serve               # Build and serve at http://localhost:3000 / 构建并在 http://localhost:3000 提供服务
cargo xtask deploy              # Build all books into docs/ (for GitHub Pages) / 构建所有书到 docs/（用于 GitHub Pages）
cargo xtask clean               # Remove site/ and docs/ / 删除 site/ 和 docs/
```

To build or serve a single book:

若只想构建或预览单本书：

```bash
cd c-cpp-book && mdbook serve --open    # http://localhost:3000
```

### Deployment / 部署

The site auto-deploys to GitHub Pages on push to `master` via `.github/workflows/pages.yml`. No manual steps needed.

站点会在代码推送到 `master` 后通过 `.github/workflows/pages.yml` 自动部署到 GitHub Pages，无需手动操作。

</details>
