# Rust 学习项目

通过独立、可运行的小示例系统学习 Rust：先阅读目标和代码，再运行、修改，最后用测试验证自己的理解。文档和注释使用中文，代码标识符使用英文。

## 环境与快速开始

项目使用 Rust 2024 edition，需要 Rust 1.85 或以上版本，建议使用 stable 工具链。首批示例仅使用标准库，无需额外依赖。

如果已经通过 rustup 安装 Rust，可以准备工具链和检查工具：

```bash
rustup update stable
rustup component add --toolchain stable rustfmt clippy
```

在项目根目录运行：

```bash
# 查看学习导航
cargo run

# 从第一章开始；修改示例后重新运行即可
cargo run --example ch01_basics
```

## 目录与章节

```text
hello_rust/
├── Cargo.toml                  # 单包配置，Rust 2024 edition
├── Cargo.lock                  # 保留锁文件
├── README.md                   # 学习导航与开发约定
├── src/
│   └── main.rs                 # 默认导航入口
└── examples/
    ├── ch01_basics.rs          # 基础语法
    ├── ch02_ownership.rs       # 所有权、借用与切片，含单元测试
    └── ch03_structs_enums.rs   # 结构体、方法、枚举与模式匹配
```

| 章节 | 学习内容 | 运行命令 |
| --- | --- | --- |
| 01 基础语法 | 变量、类型、函数、控制流 | `cargo run --example ch01_basics` |
| 02 所有权与借用 | 所有权转移、共享借用、可变借用、字符串切片 | `cargo run --example ch02_ownership` |
| 03 结构体与枚举 | 结构体、方法、枚举、`match`、`Option` | `cargo run --example ch03_structs_enums` |

每个示例都有自己的 `main()`，由 Cargo 自动发现，可以独立运行。根目录的 `cargo run` 只显示学习导航。

## 学习方法与后续路线

1. 阅读章节顶部的学习目标，先预测程序的输出。
2. 运行示例，对照注释理解结果，再修改输入或实现。
3. 完成章节顶部的扩展练习，尝试解释编译器给出的提示。
4. 为有明确输入输出的函数补充测试，检查正常输入与边界情况。

第 02 章的 `first_word` 返回输入中第一个由空白分隔的单词切片，无需复制字符串；输入为空或只有空白时返回空切片。其测试演示了空输入、前后空白、多个单词、中文和 Unicode 空白的处理。

已实现的第 01–03 章之后，按以下顺序扩展。下列章节目前均为**规划中，尚未实现**：

| 章节 | 主题 | 后续练习方向 |
| --- | --- | --- |
| 04 | 集合 | `Vec`、`String`、`HashMap` 与单词计数 |
| 05 | 错误处理 | `Result`、`?`、文件读取与错误传播 |
| 06 | 泛型与 trait | 用泛型和 trait 表达共同行为 |
| 07 | 生命周期 | 理解引用之间的生命周期关系 |
| 08 | 模块与测试 | 模块可见性、单元测试、集成测试与文档测试 |
| 09 | 智能指针 | `Box`、`Rc`、`RefCell` 与共享数据 |
| 10 | 并发与异步 | 线程、消息传递、`Arc`、`Mutex`，再学习 async/await |
| 11 | 小项目实践 | 综合运用以上知识编写命令行工具 |

## 测试与代码检查

```bash
# 运行所有目标中的测试，包括 examples 内的单元测试
cargo test --all-targets

# 仅运行第 02 章的测试
cargo test --example ch02_ownership

# 检查格式，不修改文件
cargo fmt --all -- --check

# 检查所有目标，并将警告视为失败
cargo clippy --all-targets -- -D warnings
```

需要应用默认格式时运行 `cargo fmt --all`。完整验证时还应分别运行三个章节，确认示例的实际输出；测试命令不会执行示例的 `main()`。

## 新增章节

1. 在 `examples/` 新增 `chNN_topic.rs`，例如 `ch04_collections.rs`，编号与学习路线一致。
2. 在文件顶部写明学习目标、运行命令和扩展练习建议，再提供可独立运行的 `fn main()`。章节以展示一个主题为主，优先使用标准库。
3. 对需要验证输入输出行为的函数，在同一文件中增加 `#[cfg(test)] mod tests`；可参考第 02 章。练习要求写在注释中，让默认示例保持可编译、可运行。
4. 更新本文件的目录、章节索引及实现状态，并在 `src/main.rs` 中补充导航命令。
5. 运行新示例和上述测试、格式及 Clippy 检查。直接放在 `examples/` 下的单文件示例无需在 `Cargo.toml` 中逐一登记。

构建产物 `target/` 和本地编辑器目录 `.vscode/` 已加入忽略规则；`Cargo.lock` 保留在项目中。
