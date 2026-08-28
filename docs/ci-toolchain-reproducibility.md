# CI 与 Rust 工具链可复现性

## 故障演进

第一次 Release workflow 失败发生在 `just setup`。`cargo-binstall` 没有找到 `taplo-cli 0.10.0` 的预编译产物，因而回退到 `cargo install`；原命令没有使用 crate 随包发布的 `Cargo.lock`，解析到了要求 Rust 1.88 的新依赖，而项目工具链仍为 Rust 1.85.0。

锁定开发工具的源码依赖后，`just setup` 恢复正常。随后构建项目本身时再次失败：仓库忽略了根 `Cargo.lock`，CI 每次都会重新解析工作区依赖，并获取了要求 Rust 1.88 的 ICU 2.3 系列依赖。所有失败任务最终都指向同一问题，即 Rust 1.85.0 已低于当前依赖图的最低版本要求。

该问题与 pnpm 12 兼容逻辑无关，属于项目工具链版本和依赖图未统一管理导致的环境故障。

## 修复方案

1. 将项目 Rust 工具链统一升级并固定为 1.97.1。
2. 将根 `Cargo.lock` 纳入版本控制。snm 是包含多个可执行程序的应用工作区，提交锁文件可以保证本地、CI 和发布构建使用同一依赖图；CI 检查和发布构建显式传入 `--locked`，清单与锁文件不一致时立即失败。
3. 继续固定安装器、安装脚本和开发工具版本：

- `cargo-binstall 1.22.0`
- `cargo-insta 1.48.0`
- `taplo-cli 0.10.0`
- `cargo-deny 0.20.2`
- `watchexec-cli 2.2.1`

4. 开发工具安装统一传入 `--locked`。当某个工具缺少当前平台的预编译产物时，`cargo-binstall` 会依据该 crate 发布时携带的 `Cargo.lock` 回退编译。
5. 清理 Rust 1.97.1 严格警告检查发现的无效自赋值和未使用导入，继续保持 CI 的 `-D warnings` 策略。

升级 Rust 后，`cargo-deny` 恢复为 0.20.2；该版本要求 Rust 1.88，与项目的 Rust 1.97.1 兼容。

## 设计边界

本次修复统一 Rust 工具链、工作区依赖锁和开发工具版本，不修改 pnpm 12 的执行链路或其他业务行为。固定的开发工具集中保留在 `justfile`，本地开发和 GitHub Actions 继续复用同一个 `setup` 入口。

后续升级 Rust 时，应同时检查 `rust-toolchain.toml`、根 `Cargo.lock`、开发工具的最低 Rust 版本，并在 `-D warnings` 下完成全工作区验证。

## 验证方式

1. 检查 `justfile` 能被 Just 正常解析。
2. 在隔离的 `CARGO_HOME` 中执行固定版本的安装脚本。
3. 使用 Rust 1.97.1 和 `--locked` 安装全部固定版本工具。
4. 使用根 `Cargo.lock` 对全工作区执行严格检查、测试和发布构建。
5. 执行格式化检查和差异检查。
