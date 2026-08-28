# CI 工具链可复现性修复

## 故障现象

Release workflow 的测试任务在执行 `just setup` 时失败。`cargo-binstall` 没有找到 `taplo-cli 0.10.0` 的预编译产物，因而回退到 `cargo install`；原命令没有使用 crate 随包发布的 `Cargo.lock`，解析到了要求 Rust 1.88 的新依赖，而项目工具链固定为 Rust 1.85.0。

该问题与 pnpm 12 兼容代码无关，属于 CI 开发工具安装过程随上游依赖漂移导致的环境故障。

## 修复方案

`just setup` 现在同时固定安装器、安装脚本和开发工具版本：

- `cargo-binstall 1.22.0`
- `cargo-insta 1.48.0`
- `taplo-cli 0.10.0`
- `cargo-deny 0.18.3`
- `watchexec-cli 2.2.1`

安装命令统一传入 `--locked`。当某个工具缺少当前平台的预编译产物时，`cargo-binstall` 会依据该 crate 发布时携带的 `Cargo.lock` 回退编译，不再重新解析可能提高最低 Rust 版本的依赖。

`cargo-deny` 选择 0.18.3，是因为该版本声明的最低 Rust 版本为 1.85.0，与项目工具链一致；后续升级开发工具时，应将工具版本、锁文件和 `rust-toolchain.toml` 作为一个整体验证。

## 设计边界

本次修复只调整开发工具的安装方式，不升级项目 Rust 工具链，不修改业务代码，也不改变 pnpm 12 的执行链路。固定版本集中保留在 `justfile`，本地开发和 GitHub Actions 继续复用同一个 `setup` 入口。

## 验证方式

1. 检查 `justfile` 能被 Just 正常解析。
2. 在隔离的 `CARGO_HOME` 中执行固定版本的安装脚本。
3. 使用 Rust 1.85.0 和 `--locked` 安装全部固定版本工具。
4. 执行格式化检查和差异检查。
