# Rust crates and modules / Rust Crate 与模块
 
 > **What you'll learn / 你将学到：** How Rust organizes code into modules and crates — privacy-by-default visibility, `pub` modifiers, workspaces, and the `crates.io` ecosystem. Replaces C/C++ header files, `#include`, and CMake dependency management.
 >
 > Rust 如何将代码组织成模块和 crate —— 默认私有的可见性、`pub` 修饰符、工作空间以及 `crates.io` 生态系统。这些将取代 C/C++ 的头文件、`#include` 和 CMake 依赖管理。
 
 - Modules are the fundamental organizational unit of code within crates / 模块是 crate 内部基本的代码组织单元
-     - Each source file (.rs) is its own module, and can create nested modules using the ```mod``` keyword.
+     - Each source file (.rs) is its own module, and can create nested modules using the ```mod``` keyword. / 每个源文件 (.rs) 都是一个独立的模块，并可以使用 ```mod``` 关键字创建嵌套模块。
-     - All types in a (sub-) module are **private** by default, and aren't externally visible within the same crate unless they are explicitly marked as ```pub``` (public). The scope of ```pub``` can be further restricted to ```pub(crate)```, etc
+     - All types in a (sub-) module are **private** by default, and aren't externally visible within the same crate unless they are explicitly marked as ```pub``` (public). The scope of ```pub``` can be further restricted to ```pub(crate)```, etc / （子）模块中的所有类型默认都是**私有的**，除非显式标记为 ```pub``` (public)，否则在同一个 crate 内部也是外部不可见的。```pub``` 的范围可以进一步限制为 ```pub(crate)``` 等。
-     - Even if an type is public, it doesn't automatically become visible within the scope of another module unless it's imported using the ```use``` keyword. Child submodules can reference types in the parent scope using the ```use super::```
+     - Even if an type is public, it doesn't automatically become visible within the scope of another module unless it's imported using the ```use``` keyword. Child submodules can reference types in the parent scope using the ```use super::``` / 即使一个类型是公有的，它也不会自动在另一个模块的作用域内可见，除非使用 ```use``` 关键字导入。子模块可以使用 ```use super::``` 引用父级作用域中的类型。
-     - Source files (.rs) aren't automatically included in the crate **unless** they are explicitly listed in ```main.rs``` (executable) or ```lib.rs```
+     - Source files (.rs) aren't automatically included in the crate **unless** they are explicitly listed in ```main.rs``` (executable) or ```lib.rs``` / 源文件 (.rs) 不会自动包含在 crate 中，**除非**它们在 ```main.rs```（可执行文件）或 ```lib.rs```（库）中被显式列出。
 
- # Exercise: Modules and functions
+ # Exercise: Modules and functions / 练习：模块与函数
- - We'll take a look at modifying our [hello world](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=522d86dbb8c4af71ff2ec081fb76aee7) to call another function
+ - We'll take a look at modifying our [hello world](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=522d86dbb8c4af71ff2ec081fb76aee7) to call another function / 我们来看看如何修改 [hello world](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=522d86dbb8c4af71ff2ec081fb76aee7) 示例以调用另一个函数
-     - As previously mentioned, function are defined with the ```fn``` keyword. The ```->``` keyword declares that the function returns a value (the default is void) with the type ```u32``` (unsigned 32-bit integer)
+     - As previously mentioned, function are defined with the ```fn``` keyword. The ```->``` keyword declares that the function returns a value (the default is void) with the type ```u32``` (unsigned 32-bit integer) / 如前所述，函数使用 ```fn``` 关键字定义。```->``` 关键字声明函数返回一个值（默认为 void），类型为 ```u32```（32 位无符号整数）。
-     - Functions are scoped by module, i.e., two functions with exact same name in two modules won't have a name collision
+     - Functions are scoped by module, i.e., two functions with exact same name in two modules won't have a name collision / 函数受模块作用域限制，即两个不同模块中同名的两个函数不会发生命名冲突。
-         - The module scoping extends to all types (for example, a ```struct foo``` in ```mod a { struct foo; }``` is a distinct type (```a::foo```) from ```mod b { struct foo; }``` (```b::foo```))
+         - The module scoping extends to all types (for example, a ```struct foo``` in ```mod a { struct foo; }``` is a distinct type (```a::foo```) from ```mod b { struct foo; }``` (```b::foo```)) / 模块作用域适用于所有类型（例如，```mod a { struct foo; }``` 中的 ```struct foo``` 与 ```mod b { struct foo; }``` 中的 ```b::foo``` 是不同的类型）。
 
- **Starter code** — complete the functions:
+ **Starter code / 初始代码** —— 完成以下函数：
 ```rust
 mod math {
-    // TODO: implement pub fn add(a: u32, b: u32) -> u32
+    // TODO: implement pub fn add(a: u32, b: u32) -> u32 / TODO：实现 pub fn add(a: u32, b: u32) -> u32
 }
 
 fn greet(name: &str) -> String {
-    // TODO: return "Hello, <name>! The secret number is <math::add(21,21)>"
+    // TODO: return "Hello, <name>! The secret number is <math::add(21,21)>" / TODO：返回 "Hello, <name>! The secret number is <math::add(21,21)>"
     todo!()
 }
 
 fn main() {
     println!("{}", greet("Rustacean"));
 }
 ```
 
- <details><summary>Solution (click to expand)</summary>
+ <details><summary>Solution (click to expand) / 解决方案（点击展开）</summary>
 
 ```rust
 mod math {
     pub fn add(a: u32, b: u32) -> u32 {
         a + b
     }
 }
 
 fn greet(name: &str) -> String {
     format!("Hello, {}! The secret number is {}", name, math::add(21, 21))
 }
 
 fn main() {
     println!("{}", greet("Rustacean"));
 }
-// Output: Hello, Rustacean! The secret number is 42
+// Output / 输出：Hello, Rustacean! The secret number is 42
 ```
 
 </details>
 
- ## Workspaces and crates (packages)
+ ## Workspaces and crates (packages) / 工作空间与 Crate（包）
 
- - Any significant Rust project should use workspaces to organize component crates
+ - Any significant Rust project should use workspaces to organize component crates / 任何具有一定规模的 Rust 项目都应使用工作空间来组织各组件 crate
-     - A workspace is simply a collection of local crates that will be used to build the target binaries. The `Cargo.toml` at the workspace root should have a pointer to the constituent packages (crates)
+     - A workspace is simply a collection of local crates that will be used to build the target binaries. The `Cargo.toml` at the workspace root should have a pointer to the constituent packages (crates) / 工作空间仅仅是用于构建目标二进制文件的本地 crate 集合。工作空间根目录下的 `Cargo.toml` 应包含指向各组成包（crate）的指针。
 
 ```toml
 [workspace]
 resolver = "2"
 members = ["package1", "package2"]
 ```
 
 ```text
- workspace_root/
+ workspace_root/ # 工作空间根目录
- |-- Cargo.toml      # Workspace configuration
+ |-- Cargo.toml      # Workspace configuration / 工作空间配置
 |-- package1/
- |   |-- Cargo.toml  # Package 1 configuration
+ |   |-- Cargo.toml  # Package 1 configuration / 包 1 配置
 |   `-- src/
- |       `-- lib.rs  # Package 1 source code
+ |       `-- lib.rs  # Package 1 source code / 包 1 源码
 |-- package2/
- |   |-- Cargo.toml  # Package 2 configuration
+ |   |-- Cargo.toml  # Package 2 configuration / 包 2 配置
 |   `-- src/
- |       `-- main.rs # Package 2 source code
+ |       `-- main.rs # Package 2 source code / 包 2 源码
 ```
 
 ---
- ## Exercise: Using workspaces and package dependencies
+ ## Exercise: Using workspaces and package dependencies / 练习：使用工作空间与包依赖
- - We'll create a simple package and use it from our ```hello world``` program`
+ - We'll create a simple package and use it from our ```hello world``` program / 我们将创建一个简单的包，并在我们的 ```hello world``` 程序中使用它
- - Create the workspace directory
+ - Create the workspace directory / 创建工作空间目录
 ```bash
 mkdir workspace
 cd workspace
 ```
- - Create a file called Cargo.toml and add the following to it. This creates an empty workspace
+ - Create a file called Cargo.toml and add the following to it. This creates an empty workspace / 创建一个名为 Cargo.toml 的文件并添加以下内容。这将创建一个空的工作空间
 ```toml
 [workspace]
 resolver = "2"
 members = []
 ```
- - Add the packages (```cargo new --lib``` specifies a library instead of an executable`)
+ - Add the packages (```cargo new --lib``` specifies a library instead of an executable) / 添加包（```cargo new --lib``` 指定创建一个库而不是可执行文件）
 ```bash
 cargo new hello
 cargo new --lib hellolib
 ```
 
- ## Exercise: Using workspaces and package dependencies
+ ## Exercise: Using workspaces and package dependencies continued / 练习：使用工作空间与包依赖（续）
- - Take a look at the generated Cargo.toml in ```hello``` and ```hellolib```. Notice that both of them have been to the upper level ```Cargo.toml```
+ - Take a look at the generated Cargo.toml in ```hello``` and ```hellolib```. Notice that both of them have been to the upper level ```Cargo.toml``` / 查看 ```hello``` 和 ```hellolib``` 中生成的 Cargo.toml。注意它们都已被添加到上一级的 ```Cargo.toml``` 中。
- - The presence of ```lib.rs``` in ```hellolib``` implies a library package (see https://doc.rust-lang.org/cargo/reference/cargo-targets.html for customization options)
+ - The presence of ```lib.rs``` in ```hellolib``` implies a library package (see https://doc.rust-lang.org/cargo/reference/cargo-targets.html for customization options) / ```hellolib``` 中 ```lib.rs``` 的存在意味着这是一个库包（有关自定义选项，请参阅 https://doc.rust-lang.org/cargo/reference/cargo-targets.html）。
- - Adding a dependency on ```hellolib``` in ```Cargo.toml``` for ```hello```
+ - Adding a dependency on ```hellolib``` in ```Cargo.toml``` for ```hello``` / 在 ```hello``` 的 ```Cargo.toml``` 中添加对 ```hellolib``` 的依赖：
 ```toml
 [dependencies]
 hellolib = {path = "../hellolib"}
 ```
- - Using ```add()``` from ```hellolib```
+ - Using ```add()``` from ```hellolib``` / 从 ```hellolib``` 中使用 ```add()```：
 ```rust
 fn main() {
     println!("Hello, world! {}", hellolib::add(21, 21));
 }
 ```
 
- <details><summary>Solution (click to expand)</summary>
+ <details><summary>Solution (click to expand) / 解决方案（点击展开）</summary>
 
- The complete workspace setup:
+ 完整的工作空间设置：
 
 ```bash
- # Terminal commands
+ # Terminal commands / 终端命令
 mkdir workspace && cd workspace
 
- # Create workspace Cargo.toml
+ # Create workspace Cargo.toml / 创建工作空间 Cargo.toml
 cat > Cargo.toml << 'EOF'
 [workspace]
 resolver = "2"
 members = ["hello", "hellolib"]
 EOF
 
 cargo new hello
 cargo new --lib hellolib
 ```
 
 ```toml
- # hello/Cargo.toml — add dependency
+ # hello/Cargo.toml — add dependency / 添加依赖
 [dependencies]
 hellolib = {path = "../hellolib"}
 ```
 
 ```rust
- // hellolib/src/lib.rs — already has add() from cargo new --lib
+ // hellolib/src/lib.rs — already has add() / 已经带有 add()
 pub fn add(left: u64, right: u64) -> u64 {
     left + right
 }
 ```
 
 ```rust,ignore
 // hello/src/main.rs
 fn main() {
     println!("Hello, world! {}", hellolib::add(21, 21));
 }
-// Output: Hello, world! 42
+// Output / 输出：Hello, world! 42
 ```
 
 </details>
 
- # Using community crates from crates.io
+ # Using community crates from crates.io / 使用来自 crates.io 的社区 Crate
- - Rust has a vibrant ecosystem of community crates (see https://crates.io/)
+ - Rust has a vibrant ecosystem of community crates (see https://crates.io/) / Rust 拥有充满活力的社区 crate 生态系统（见 https://crates.io/）
-     - The Rust philosophy is to keep the standard library compact and outsource functionality to community crates
+     - The Rust philosophy is to keep the standard library compact and outsource functionality to community crates / Rust 的哲学是保持标准库精简，并将功能外包给社区 crate。
-     - There is no hard and fast rule about using community crates, but the rule of thumb should be ensure that the crate has a decent maturity level (indicated by the version number), and that it's being actively maintained. Reach out to internal sources if in doubt about a crate
+     - There is no hard and fast rule about using community crates, but the rule of thumb should be ensure that the crate has a decent maturity level (indicated by the version number), and that it's being actively maintained. Reach out to internal sources if in doubt about a crate / 关于使用社区 crate 没有一成不变的规则，但经验法则是应确保该 crate 具有适当的成熟度（由版本号指示），并且正在积极维护。如果对某个 crate 有疑问，请咨询内部资源。
- - Every crate published on ```crates.io``` has a major and minor version
+ - Every crate published on ```crates.io``` has a major and minor version / 在 ```crates.io``` 上发布的每个 crate 都包含主版本号和次版本号。
-     - Crates are expected to observe the major and minor ```SemVer``` guidelines defined here: https://doc.rust-lang.org/cargo/reference/semver.html
+     - Crates are expected to observe the major and minor ```SemVer``` guidelines defined here: https://doc.rust-lang.org/cargo/reference/semver.html / Crate 应遵守此处定义的 ```SemVer```（语义化版本控制）指南：https://doc.rust-lang.org/cargo/reference/semver.html。
-     - The TL;DR version is that there should be no breaking changes for the same minor version. For example, v0.11 must be compatible with v0.15 (but v0.20 may have breaking changes)
+     - The TL;DR version is that there should be no breaking changes for the same minor version. For example, v0.11 must be compatible with v0.15 (but v0.20 may have breaking changes) / 简单来说，在同一个次版本内不应有破坏性更改。例如，v0.11 必须与 v0.15 兼容（但 v0.20 可能会有破坏性更改）。
 
- # Crates dependencies and SemVer
+ # Crates dependencies and SemVer / Crate 依赖与语义化版本控制
- - Crates can define dependencies on a specific versions of a crate, specific minor or major version, or don't care. The following examples show the ```Cargo.toml``` entries for declaring a dependency on the ```rand``` crate
+ - Crates can define dependencies on a specific versions of a crate, specific minor or major version, or don't care. The following examples show the ```Cargo.toml``` entries for declaring a dependency on the ```rand``` crate / Crate 可以定义对特定版本、特定次版本、主版本或任意版本的依赖。以下示例显示了在 ```Cargo.toml``` 中声明对 ```rand``` crate 依赖的条目。
- - At least ```0.10.0```, but anything ```< 0.11.0``` is fine
+ - At least ```0.10.0```, but anything ```< 0.11.0``` is fine / 至少 ```0.10.0```，但任何 ```< 0.11.0``` 的版本都可以
 ```toml
 [dependencies]
 rand = { version = "0.10.0"}
 ```
- - Only ```0.10.0```, and nothing else
+ - Only ```0.10.0```, and nothing else / 仅限 ```0.10.0```，别无他选
 ```toml
 [dependencies]
 rand = { version = "=0.10.0"}
 ```
- - Don't care; ```cargo``` will select the latest version
+ - Don't care; ```cargo``` will select the latest version / 无所谓；```cargo``` 将选择最新版本
 ```toml
 [dependencies]
 rand = { version = "*"}
 ```
- - Reference: https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html
+ - Reference / 参考：https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html
 ----
- # Exercise: Using the rand crate
+ # Exercise: Using the rand crate / 练习：使用 rand crate
- - Modify the ```helloworld``` example to print a random number
+ - Modify the ```helloworld``` example to print a random number / 修改 ```helloworld``` 示例以打印一个随机数
- - Use ```cargo add rand``` to add a dependency
+ - Use ```cargo add rand``` to add a dependency / 使用 ```cargo add rand``` 添加依赖
- - Use ```https://docs.rs/rand/latest/rand/``` as a reference for the API
+ - Use ```https://docs.rs/rand/latest/rand/``` as a reference for the API / 以 ```https://docs.rs/rand/latest/rand/``` 作为 API 参考
 
- **Starter code** — add this to `main.rs` after running `cargo add rand`:
+ **Starter code / 初始代码** —— 在运行 `cargo add rand` 后，将以下内容添加到 `main.rs`：
 ```rust,ignore
 use rand::RngExt;
 
 fn main() {
     let mut rng = rand::rng();
-    // TODO: Generate and print a random u32 in 1..=100
+    // TODO: Generate and print a random u32 in 1..=100 / TODO：生成并打印 1..=100 之间的随机 u32
-    // TODO: Generate and print a random bool
+    // TODO: Generate and print a random bool / TODO：生成并打印随机布尔值
-    // TODO: Generate and print a random f64
+    // TODO: Generate and print a random f64 / TODO：生成并打印随机 f64
 }
 ```
 
- <details><summary>Solution (click to expand)</summary>
+ <details><summary>Solution (click to expand) / 解决方案（点击展开）</summary>
 
 ```rust
 use rand::RngExt;
 
 fn main() {
     let mut rng = rand::rng();
     let n: u32 = rng.random_range(1..=100);
     println!("Random number (1-100): {n}");
 
-    // Generate a random boolean
+    // Generate a random boolean / 生成随机布尔值
     let b: bool = rng.random();
     println!("Random bool: {b}");
 
-    // Generate a random float between 0.0 and 1.0
+    // Generate a random float between 0.0 and 1.0 / 生成 0.0 到 1.0 之间的随机浮点数
     let f: f64 = rng.random();
     println!("Random float: {f:.4}");
 }
 ```
 
 </details>
 
- # Cargo.toml and Cargo.lock
+ # Cargo.toml and Cargo.lock / Cargo.toml 与 Cargo.lock
- - As mentioned previously, Cargo.lock is automatically generated from Cargo.toml
+ - As mentioned previously, Cargo.lock is automatically generated from Cargo.toml / 如前所述，Cargo.lock 是根据 Cargo.toml 自动生成的。
-     - The main idea behind Cargo.lock is to ensure reproducible builds. For example, if ```Cargo.toml``` had specified a version of ```0.10.0```, cargo is free to choose any version that is ```< 0.11.0```
+     - The main idea behind Cargo.lock is to ensure reproducible builds. For example, if ```Cargo.toml``` had specified a version of ```0.10.0```, cargo is free to choose any version that is ```< 0.11.0``` / Cargo.lock 的核心思想是确保构建的可复现性。例如，如果 ```Cargo.toml``` 指定了 ```0.10.0``` 版本，cargo 可以自由选择任何 ```< 0.11.0``` 的版本。
-     - Cargo.lock contains the *specific* version of the rand crate that was used during the build.
+     - Cargo.lock contains the *specific* version of the rand crate that was used during the build. / Cargo.lock 包含了构建期间使用的 rand crate 的*具体*版本。
-     - The recommendation is to include ```Cargo.lock``` in the git repo to ensure reproducible builds
+     - The recommendation is to include ```Cargo.lock``` in the git repo to ensure reproducible builds / 建议将 ```Cargo.lock``` 包含在 git 仓库中，以确保构建的可复现性。
 
- ## Cargo test feature
+ ## Cargo test feature / Cargo 测试功能
- - Rust unit tests reside in the same source file (by convention), and are usually grouped into separate module
+ - Rust unit tests reside in the same source file (by convention), and are usually grouped into separate module / （按照惯例）Rust 单元测试位于同一个源文件中，通常被分在独立的模块内。
-     - The test code is never included in the actual binary. This is made possible by the ```cfg``` (configuration) feature. Configurations are useful for creating platform specific code (```Linux``` vs. ```Windows```) for example
+     - The test code is never included in the actual binary. This is made possible by the ```cfg``` (configuration) feature. Configurations are useful for creating platform specific code (```Linux``` vs. ```Windows```) for example / 测试代码永远不会包含在实际的二进制文件中。这是通过 ```cfg```（配置）特性实现的。例如，配置对于创建特定于平台的代码（如 ```Linux``` vs ```Windows```）非常有用。
-     - Tests can be executed with ```cargo test```. Reference: https://doc.rust-lang.org/reference/conditional-compilation.html
+     - Tests can be executed with ```cargo test```. Reference / 参考：https://doc.rust-lang.org/reference/conditional-compilation.html。
 
 ```rust
 pub fn add(left: u64, right: u64) -> u64 {
     left + right
 }
- // Will be included only during testing
+ // Will be included only during testing / 仅在测试期间包含
 #[cfg(test)]
 mod tests {
-    use super::*; // This makes all types in the parent scope visible
+    use super::*; // This makes all types in the parent scope visible / 使父作用域中的所有类型可见
     #[test]
     fn it_works() {
-        let result = add(2, 2); // Alternatively, super::add(2, 2);
+        let result = add(2, 2); // Alternatively / 或者：super::add(2, 2);
         assert_eq!(result, 4);
     }
 }
 ```
 
- # Other Cargo features
+ # Other Cargo features / 其他 Cargo 功能
- - ```cargo``` has several other useful features including:
+ - ```cargo``` 还有其他几个有用的功能，包括：
-     - ```cargo clippy``` is a great way of linting Rust code. In general, warnings should be fixed (or rarely suppressed if really warranted)
+     - ```cargo clippy``` 是对 Rust 代码进行 lint（静态检查）的绝佳方式。通常，应修复所有警告（或者在确实必要且有根据的情况下极少数地进行抑制）。
-     - ```cargo format``` executes the ```rustfmt``` tool to format source code. Using the tool ensures standard formatting of checked-in code and puts an end to debates about style
+     - ```cargo format``` 执行 ```rustfmt``` 工具来格式化源代码。使用此工具可确保提交的代码符合标准格式，并终结关于代码风格的争论。
-     - ```cargo doc``` can be used to generate documentation from the ```///``` style comments. The documentation for all crates on ```crates.io``` was generated using this method
+     - ```cargo doc``` 可用于从 ```///``` 风格的注释生成文档。```crates.io``` 上所有 crate 的文档都是使用此方法生成的。
 
- ### Build Profiles: Controlling Optimization
+ ### Build Profiles: Controlling Optimization / 构建配置：控制优化
 
- In C, you pass `-O0`, `-O2`, `-Os`, `-flto` to `gcc`/`clang`. In Rust, you configure
+ 在 C 中，你将 `-O0`、`-O2`、`-Os`、`-flto` 传递给 `gcc`/`clang`。在 Rust 中，你在 `Cargo.toml` 中配置构建配置（profiles）：
- build profiles in `Cargo.toml`:
 
 ```toml
- # Cargo.toml — build profile configuration
+ # Cargo.toml — build profile configuration / 构建配置
 
 [profile.dev]
- opt-level = 0          # No optimization (fast compile, like -O0)
+ opt-level = 0          # No optimization / 无优化 (fast compile, like -O0)
- debug = true           # Full debug symbols (like -g)
+ debug = true           # Full debug symbols / 完整调试符号 (like -g)
 
 [profile.release]
- opt-level = 3          # Maximum optimization (like -O3)
+ opt-level = 3          # Maximum optimization / 最大优化 (like -O3)
- lto = "fat"            # Link-Time Optimization (like -flto)
+ lto = "fat"            # Link-Time Optimization / 链接时优化 (like -flto)
- strip = true           # Strip symbols (like the strip command)
+ strip = true           # Strip symbols / 剥离符号 (like the strip command)
- codegen-units = 1      # Single codegen unit — slower compile, better optimization
+ codegen-units = 1      # Single codegen unit / 单独的代码生成单元 —— 编译较慢，优化更好
- panic = "abort"        # No unwind tables (smaller binary)
+ panic = "abort"        # No unwind tables / 无展开表 (smaller binary)
 ```
 
-| C/GCC Flag | Cargo.toml Key | Values |
+| **C/GCC Flag** | **Cargo.toml Key** | **Values / 值** |
 |------------|---------------|--------|
-| `-O0` / `-O2` / `-O3` | `opt-level` | `0`, `1`, `2`, `3`, `"s"`, `"z"` |
+| `-O0` / `-O2` / `-O3` | `opt-level` | `0`, `1`, `2`, `3`, `"s"（优化大小）`, `"z"（极致优化大小）` |
-| `-flto` | `lto` | `false`, `"thin"`, `"fat"` |
+| `-flto` | `lto` | `false`, `"thin"`, `"fat"` |
-| `-g` / no `-g` | `debug` | `true`, `false`, `"line-tables-only"` |
+| `-g` / no `-g` | `debug` | `true`, `false`, `"line-tables-only"` |
-| `strip` command | `strip` | `"none"`, `"debuginfo"`, `"symbols"`, `true`/`false` |
+| `strip` command | `strip` | `"none"`, `"debuginfo"`, `"symbols"`, `true`/`false` |
-| — | `codegen-units` | `1` = best opt, slowest compile |
+| — | `codegen-units` | `1` = best opt, slowest compile / 最好优化，最慢编译 |
 
 ```bash
- cargo build              # Uses [profile.dev]
+ cargo build              # Uses [profile.dev] / 使用开发配置
- cargo build --release    # Uses [profile.release]
+ cargo build --release    # Uses [profile.release] / 使用发布配置
 ```
 
- ### Build Scripts (`build.rs`): Linking C Libraries
+ ### Build Scripts (`build.rs`): Linking C Libraries / 构建脚本 (build.rs)：链接 C 库
 
- In C, you use Makefiles or CMake to link libraries and run code generation.
+ 在 C 中，你使用 Makefiles 或 CMake 来链接库并运行代码生成。
- Rust uses a `build.rs` file at the crate root:
+ Rust 在 crate 根目录下使用 `build.rs` 文件：
 
 ```rust
- // build.rs — runs before compiling the crate
+ // build.rs — runs before compiling the crate / 在编译 crate 之前运行
 
 fn main() {
-    // Link a system C library (like -lbmc_ipmi in gcc)
+    // Link a system C library (like -lbmc_ipmi in gcc) / 链接系统 C 库（类似于 gcc 中的 -lbmc_ipmi）
     println!("cargo::rustc-link-lib=bmc_ipmi");
 
-    // Where to find the library (like -L/usr/lib/bmc)
+    // Where to find the library (like -L/usr/lib/bmc) / 到哪里寻找库（类似于 -L/usr/lib/bmc）
     println!("cargo::rustc-link-search=/usr/lib/bmc");
 
-    // Re-run if the C header changes
+    // Re-run if the C header changes / 如果 C 头文件发生变化则重新运行
     println!("cargo::rerun-if-changed=wrapper.h");
 }
 ```
 
- You can even compile C source files directly from a Rust crate:
+ 你甚至可以直接从 Rust crate 编译 C 源文件：
 
 ```toml
 # Cargo.toml
 [build-dependencies]
- cc = "1"  # C compiler integration
+ cc = "1"  # C compiler integration / C 编译器集成
 ```
 
 ```rust
 // build.rs
 fn main() {
     cc::Build::new()
         .file("src/c_helpers/ipmi_raw.c")
         .include("/usr/include/bmc")
-        .compile("ipmi_raw");   // Produces libipmi_raw.a, linked automatically
+        .compile("ipmi_raw");   // Produces libipmi_raw.a / 产生 libipmi_raw.a，自动链接
     println!("cargo::rerun-if-changed=src/c_helpers/ipmi_raw.c");
 }
 ```
 
-| C / Make / CMake | Rust `build.rs` |
+| **C / Make / CMake** | **Rust `build.rs`** |
 |-----------------|-----------------|
-| `-lfoo` | `println!("cargo::rustc-link-lib=foo")` |
+| `-lfoo` | `println!("cargo::rustc-link-lib=foo")` |
-| `-L/path` | `println!("cargo::rustc-link-search=/path")` |
+| `-L/path` | `println!("cargo::rustc-link-search=/path")` |
-| Compile C source | `cc::Build::new().file("foo.c").compile("foo")` |
+| Compile C source / 编译 C 源码 | `cc::Build::new().file("foo.c").compile("foo")` |
-| Generate code | Write files to `$OUT_DIR`, then `include!()` |
+| Generate code / 生成代码 | Write files to `$OUT_DIR`, then `include!()` / 将文件写入 `$OUT_DIR`，然后使用 `include!()` |
 
- ### Cross-Compilation
+ ### Cross-Compilation / 交叉编译
 
- In C, cross-compilation requires installing a separate toolchain (`arm-linux-gnueabihf-gcc`)
+ 在 C 中，交叉编译需要安装一个独立的工具链（`arm-linux-gnueabihf-gcc`）
- and configuring Make/CMake. In Rust:
+ 并配置 Make/CMake。在 Rust 中：
 
 ```bash
- # Install a cross-compilation target
+ # Install a cross-compilation target / 安装交叉编译目标
 rustup target add aarch64-unknown-linux-gnu
 
- # Cross-compile
+ # Cross-compile / 交叉编译
 cargo build --target aarch64-unknown-linux-gnu --release
 ```
 
- Specify the linker in `.cargo/config.toml`:
+ 在 `.cargo/config.toml` 中指定链接器：
 
 ```toml
 [target.aarch64-unknown-linux-gnu]
 linker = "aarch64-linux-gnu-gcc"
 ```
 
-| C Cross-Compile | Rust Equivalent |
+| **C Cross-Compile / C 交叉编译** | **Rust Equivalent / Rust 等价物** |
 |-----------------|-----------------|
-| `apt install gcc-aarch64-linux-gnu` | `rustup target add aarch64-unknown-linux-gnu` + install linker |
+| `apt install gcc-aarch64-linux-gnu` | `rustup target add aarch64-unknown-linux-gnu` + install linker / 安装链接器 |
-| `CC=aarch64-linux-gnu-gcc make` | `.cargo/config.toml` `[target.X] linker = "..."` |
+| `CC=aarch64-linux-gnu-gcc make` | `.cargo/config.toml` `[target.X] linker = "..."` |
-| `#ifdef __aarch64__` | `#[cfg(target_arch = "aarch64")]` |
+| `#ifdef __aarch64__` | `#[cfg(target_arch = "aarch64")]` |
-| Separate Makefile targets | `cargo build --target ...` |
+| Separate Makefile targets / 独立的 Makefile 目标 | `cargo build --target ...` |
 
- ### Feature Flags: Conditional Compilation
+ ### Feature Flags: Conditional Compilation / Feature 标志：条件编译
 
- C uses `#ifdef` and `-DFOO` for conditional compilation. Rust uses feature flags
+ C 使用 `#ifdef` 和 `-DFOO` 进行条件编译。Rust 使用在 `Cargo.toml` 中定义的 feature 标志：
- defined in `Cargo.toml`:
 
 ```toml
 # Cargo.toml
 [features]
- default = ["json"]         # Enabled by default
+ default = ["json"]         # Enabled by default / 默认启用
- json = ["dep:serde_json"]  # Optional dependency
+ json = ["dep:serde_json"]  # Optional dependency / 可选依赖
- verbose = []               # Flag with no dependency
+ verbose = []               # Flag with no dependency / 无依赖的标志
- gpu = ["dep:cuda-sys"]     # Optional GPU support
+ gpu = ["dep:cuda-sys"]     # Optional GPU support / 可选的 GPU 支持
 ```
 
 ```rust
- // Code gated on features:
+ // Code gated on features / 受 feature 控制的代码：
 #[cfg(feature = "json")]
 pub fn parse_config(data: &str) -> Result<Config, Error> {
     serde_json::from_str(data).map_err(Error::from)
 }
 
 #[cfg(feature = "verbose")]
 macro_rules! verbose {
     ($($arg:tt)*) => { eprintln!("[VERBOSE] {}", format!($($arg)*)); }
 }
 #[cfg(not(feature = "verbose"))]
 macro_rules! verbose {
-    ($($arg:tt)*) => {}; // Compiles to nothing
+    ($($arg:tt)*) => {}; // Compiles to nothing / 编译为空
 }
 ```
 
-| C Preprocessor | Rust Feature Flags |
+| **C Preprocessor / C 预处理器** | **Rust Feature Flags / Rust Feature 标志** |
 |---------------|-------------------|
-| `gcc -DDEBUG` | `cargo build --features verbose` |
+| `gcc -DDEBUG` | `cargo build --features verbose` |
-| `#ifdef DEBUG` | `#[cfg(feature = "verbose")]` |
+| `#ifdef DEBUG` | `#[cfg(feature = "verbose")]` |
-| `#define MAX 100` | `const MAX: u32 = 100;` |
+| `#define MAX 100` | `const MAX: u32 = 100;` |
-| `#ifdef __linux__` | `#[cfg(target_os = "linux")]` |
+| `#ifdef __linux__` | `#[cfg(target_os = "linux")]` |
 
- ### Integration Tests vs Unit Tests
+ ### Integration Tests vs Unit Tests / 集成测试 vs 单元测试
 
- Unit tests live next to the code with `#[cfg(test)]`. **Integration tests** live in
+ 单元测试使用 `#[cfg(test)]` 与代码共存。**集成测试**位于
- `tests/` and test your crate's **public API only**:
+ `tests/` 目录下，并且**仅测试**你的 crate 的公有 API：
 
 ```rust
- // tests/smoke_test.rs — no #[cfg(test)] needed
+ // tests/smoke_test.rs — no #[cfg(test)] needed / 不需要 #[cfg(test)]
 use my_crate::parse_config;
 
 #[test]
 fn parse_valid_config() {
     let config = parse_config("test_data/valid.json").unwrap();
     assert_eq!(config.max_retries, 5);
 }
 ```
 
-| Aspect | Unit Tests (`#[cfg(test)]`) | Integration Tests (`tests/`) |
+| **Aspect / 维度** | **Unit Tests (`#[cfg(test)]`) / 单元测试** | **Integration Tests (`tests/`) / 集成测试** |
 |--------|----------------------------|------------------------------|
-| **Location** | Same file as code | Separate `tests/` directory |
+| **Location / 位置** | Same file as code / 与代码在同一个文件 | Separate `tests/` directory / 独立的 `tests/` 目录 |
-| **Access** | Private + public items | **Public API only** |
+| **Access / 访问权限** | Private + public items / 私有 + 公有项 | **Public API only / 仅限公有 API** |
-| **Run command** | `cargo test` | `cargo test --test smoke_test` |
+| **Run command / 运行命令** | `cargo test` | `cargo test --test smoke_test` |
 
 
- ### Testing Patterns and Strategies
+ ### Testing Patterns and Strategies / 测试模式与策略
 
- C firmware teams typically write tests in CUnit, CMocka, or custom frameworks with a
+ C 固件团队通常使用 CUnit、CMocka 或带有大量样板代码的自定义框架来编写测试。
- lot of boilerplate. Rust's built-in test harness is far more capable. This section
+ Rust 内置的测试工具功能更为强大。本节介绍生产代码中需要的模式。
- covers patterns you'll need for production code.
 
- #### `#[should_panic]` — Testing Expected Failures
+ #### `#[should_panic]` — Testing Expected Failures / 测试预期的失败
 
 ```rust
- // Test that certain conditions cause panics (like C's assert failures)
+ // Test that certain conditions cause panics / 测试某些条件是否会导致 panic（类似于 C 的 assert 失败）
 #[test]
 #[should_panic(expected = "index out of bounds")]
 fn test_bounds_check() {
     let v = vec![1, 2, 3];
-    let _ = v[10];  // Should panic
+    let _ = v[10];  // Should panic / 应该 panic
 }
 
 #[test]
 #[should_panic(expected = "temperature exceeds safe limit")]
 fn test_thermal_shutdown() {
     fn check_temperature(celsius: f64) {
         if celsius > 105.0 {
             panic!("temperature exceeds safe limit: {celsius}°C");
         }
     }
-    check_temperature(110.0);
+    check_temperature(110.0); // 出发 panic
 }
 ```
 
- #### `#[ignore]` — Slow or Hardware-Dependent Tests
+ #### `#[ignore]` — Slow or Hardware-Dependent Tests / 忽略：慢速或依赖硬件的测试
 
 ```rust
- // Mark tests that require special conditions (like C's #ifdef HARDWARE_TEST)
+ // Mark tests that require special conditions / 标记需要特殊条件的测试（类似于 C 的 #ifdef HARDWARE_TEST）
 #[test]
 #[ignore = "requires GPU hardware"]
 fn test_gpu_ecc_scrub() {
-    // This test only runs on machines with GPUs
+    // This test only runs on machines with GPUs / 此测试仅在带有 GPU 的机器上运行
     // Run with: cargo test -- --ignored
     // Run with: cargo test -- --include-ignored  (runs ALL tests)
 }
 ```
 
- #### Result-Returning Tests (replacing `unwrap` chains)
+ #### Result-Returning Tests / 返回 Result 的测试（取代 `unwrap` 链）
 
 ```rust
- // Instead of many unwrap() calls that hide the actual failure:
+ // Instead of many unwrap() / 与其使用会隐藏实际故障的多次 unwrap() 调用：
 #[test]
 fn test_config_parsing() -> Result<(), Box<dyn std::error::Error>> {
     let json = r#"{"hostname": "node-01", "port": 8080}"#;
-    let config: ServerConfig = serde_json::from_str(json)?;  // ? instead of unwrap()
+    let config: ServerConfig = serde_json::from_str(json)?;  // ? / 使用 ? 代替 unwrap()
     assert_eq!(config.hostname, "node-01");
     assert_eq!(config.port, 8080);
-    Ok(())  // Test passes if we reach here without error
+    Ok(())  // Test passes / 如果到达这里且没有错误，测试评估为通过
 }
 ```
 
- #### Test Fixtures with Builder Functions
+ #### Test Fixtures with Builder Functions / 带有构建器函数的测试固件
 
- C uses `setUp()`/`tearDown()` functions. Rust uses helper functions and `Drop`:
+ C 使用 `setUp()`/`tearDown()` 函数。Rust 使用辅助函数和 `Drop` trait：
 
 ```rust
 struct TestFixture {
     temp_dir: std::path::PathBuf,
     config: Config,
 }
 
 impl TestFixture {
     fn new() -> Self {
         let temp_dir = std::env::temp_dir().join(format!("test_{}", std::process::id()));
         std::fs::create_dir_all(&temp_dir).unwrap();
         let config = Config {
             log_dir: temp_dir.clone(),
             max_retries: 3,
             ..Default::default()
         };
         Self { temp_dir, config }
     }
 }
 
 impl Drop for TestFixture {
     fn drop(&mut self) {
-        // Automatic cleanup — like C's tearDown() but can't be forgotten
+        // Automatic cleanup / 自动清理 —— 类似于 C 的 tearDown()，但不会被遗忘
         let _ = std::fs::remove_dir_all(&self.temp_dir);
     }
 }
 
 #[test]
 fn test_with_fixture() {
     let fixture = TestFixture::new();
-    // Use fixture.config, fixture.temp_dir...
+    // Use fixture / 使用 fixture.config, fixture.temp_dir...
     assert!(fixture.temp_dir.exists());
-    // fixture is automatically dropped here → cleanup runs
+    // fixture is automatically dropped / fixture 在这里被自动 drop -> 运行清理
 }
 ```
 
- #### Mocking Traits for Hardware Interfaces
+ #### Mocking Traits for Hardware Interfaces / 为硬件接口模拟 Trait
 
- In C, mocking hardware requires preprocessor tricks or function pointer swapping.
+ 在 C 中，模拟硬件需要预处理器技巧或函数指针交换。
- In Rust, traits make this natural:
+ 在 Rust 中，使用 trait 让这一切变得很自然：
 
 ```rust
- // Production trait for IPMI communication
+ // Production trait / 用于 IPMI 通信的生产级 trait
 trait IpmiTransport {
     fn send_command(&self, cmd: u8, data: &[u8]) -> Result<Vec<u8>, String>;
 }
 
- // Real implementation (used in production)
+ // Real implementation / 真实实现（用于生产）
 struct RealIpmi { /* BMC connection details / BMC 连接详情 */ }
 impl IpmiTransport for RealIpmi {
     fn send_command(&self, cmd: u8, data: &[u8]) -> Result<Vec<u8>, String> {
-        // Actually talks to BMC hardware
+        // Actually talks to BMC hardware / 实际与 BMC 硬件通信
         todo!("Real IPMI call")
     }
 }
 
- // Mock implementation (used in tests)
+ // Mock implementation / 模拟实现（用于测试）
 struct MockIpmi {
     responses: std::collections::HashMap<u8, Vec<u8>>,
 }
 impl IpmiTransport for MockIpmi {
     fn send_command(&self, cmd: u8, _data: &[u8]) -> Result<Vec<u8>, String> {
         self.responses.get(&cmd)
             .cloned()
             .ok_or_else(|| format!("No mock response for cmd 0x{cmd:02x}"))
     }
 }
 
- // Generic function that works with both real and mock
+ // Generic function / 同时适用于真实和模拟的泛型函数
 fn read_sensor_temperature(transport: &dyn IpmiTransport) -> Result<f64, String> {
     let response = transport.send_command(0x2D, &[])?;
     if response.len() < 2 {
         return Err("Response too short".into());
     }
     Ok(response[0] as f64 + (response[1] as f64 / 256.0))
 }
 
 #[cfg(test)]
 mod tests {
     use super::*;
 
     #[test]
     fn test_temperature_reading() {
         let mut mock = MockIpmi { responses: std::collections::HashMap::new() };
-        mock.responses.insert(0x2D, vec![72, 128]); // 72.5°C
+        mock.responses.insert(0x2D, vec![72, 128]); // 模拟 72.5°C
 
         let temp = read_sensor_temperature(&mock).unwrap();
         assert!((temp - 72.5).abs() < 0.01);
     }
 
     #[test]
     fn test_short_response() {
         let mock = MockIpmi { responses: std::collections::HashMap::new() };
-        // No response configured → error
+        // No response configured / 未配置响应 -> 报错
         assert!(read_sensor_temperature(&mock).is_err());
     }
 }
 ```
 
- #### Property-Based Testing with `proptest`
+ #### Property-Based Testing with `proptest` / 使用 `proptest` 进行基于属性的测试
 
- Instead of testing specific values, test **properties** that must always hold:
+ 与其测试特定数值，不如测试必须始终成立的**属性（properties）**：
 
 ```rust
- // Cargo.toml: [dev-dependencies] proptest = "1"
+ // Cargo.toml: [dev-dependencies] proptest = "1" / 开发依赖
 use proptest::prelude::*;
 
 fn parse_sensor_id(s: &str) -> Option<u32> {
     s.strip_prefix("sensor_")?.parse().ok()
 }
 
 fn format_sensor_id(id: u32) -> String {
     format!("sensor_{id}")
 }
 
 proptest! {
     #[test]
     fn roundtrip_sensor_id(id in 0u32..10000) {
-        // Property: format then parse should give back the original
+        // Property: format then parse should give back the original / 属性：格式化后再解析应返回原始值
         let formatted = format_sensor_id(id);
         let parsed = parse_sensor_id(&formatted);
         prop_assert_eq!(parsed, Some(id));
     }
 
     #[test]
     fn parse_rejects_garbage(s in "[^s].*") {
-        // Property: strings not starting with 's' should never parse
+        // Property: strings not starting with 's' should never parse / 属性：不以 's' 开头的字符串永远不应被解析成功
         let result = parse_sensor_id(&s);
         prop_assert!(result.is_none());
     }
 }
 ```
 
- #### C vs Rust Testing Comparison
+ #### C vs Rust Testing Comparison / C vs Rust 测试对比
 
-| C Testing | Rust Equivalent |
+| **C Testing / C 测试** | **Rust Equivalent / Rust 等价物** |
 |-----------|----------------|
-| `CUnit`, `CMocka`, custom framework | Built-in `#[test]` + `cargo test` |
+| `CUnit`, `CMocka`, custom framework / 自定义框架 | Built-in `#[test]` + `cargo test` / 内置支持 |
-| `setUp()` / `tearDown()` | Builder function + `Drop` trait |
+| `setUp()` / `tearDown()` | Builder function + `Drop` trait / 构建器函数 + Drop |
-| `#ifdef TEST` mock functions | Trait-based dependency injection |
+| `#ifdef TEST` mock functions | Trait-based dependency injection / 基于 Trait 的依赖注入 |
-| `assert(x == y)` | `assert_eq!(x, y)` with auto diff output |
+| `assert(x == y)` | `assert_eq!(x, y)` (带自动 diff 输出) |
-| Separate test executable | Same binary, conditional compilation with `#[cfg(test)]` |
+| Separate test executable / 独立的测试可执行文件 | Same binary, conditional compilation / 同一二进制文件，条件编译 |
-| `valgrind --leak-check=full ./test` | `cargo test` (memory safe by default) + `cargo miri test` |
+| `valgrind --leak-check=full ./test` | `cargo test` (默认内存安全) + `cargo miri test` |
-| Code coverage: `gcov` / `lcov` | `cargo tarpaulin` or `cargo llvm-cov` |
+| Code coverage / 代码覆盖率: `gcov` / `lcov` | `cargo tarpaulin` or `cargo llvm-cov` |
-| Test discovery: manual registration | Automatic — any `#[test]` fn is discovered |
+| Test discovery / 测试发现: 手动注册 | Automatic / 自动 —— 任何 `#[test]` 函数都会被发现 |
