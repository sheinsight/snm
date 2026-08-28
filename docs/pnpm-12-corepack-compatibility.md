# pnpm 12 Corepack 兼容适配

## 背景

pnpm 12 的普通安装包将 `package.json#bin` 指向根目录下的原生命令入口。正常通过 npm 安装时，平台相关的 `@pnpm/exe.<platform>-<arch>` 可选依赖会提供原生二进制，`preinstall` 再将其链接或复制到这些入口。

snm 的包管理器安装模型只下载并解压主包，不安装可选依赖，也不执行生命周期脚本。因此，pnpm 12 根目录下的 `pnpm` 仍是占位文件；继续用 Node.js 执行该文件会产生语法错误。

## 方案选择

pnpm 为 Corepack 提供了稳定的兼容入口：

- `pnpm` 使用 `bin/pnpm.mjs`
- `pnpx` 使用 `bin/pnpx.mjs`

Corepack 同样只解压主包，不安装依赖，也不执行生命周期脚本，所以它与 snm 面临相同约束。pnpm 自带的 `.mjs` 引导层会完成以下工作：

1. 查找已安装或已经缓存的平台原生程序。
2. 缺失时根据操作系统、架构和 libc 选择对应的 `@pnpm/exe.*` 包。
3. 校验 registry 返回的完整性信息和签名。
4. 以原子方式缓存原生程序并处理并发下载竞争。
5. 通过子进程执行原生 pnpm，并透传退出状态和终端信号。

因此，snm 不再自行维护平台包映射、下载、校验和缓存逻辑，而是复用 pnpm 官方引导层。

## 执行流程

1. snm 按原有逻辑读取项目中的 `packageManager`。
2. snm 按原有逻辑下载并解压 `pnpm@<version>` 主包。
3. 当版本为 pnpm 12+ 且命令为 `pnpm` 或 `pnpx` 时，shim 选择对应的 `.mjs` 入口。
4. shim 使用当前项目选定的 Node.js 执行该入口。
5. pnpm 引导层在首次运行时下载并缓存 `pnpm-native`；后续运行直接复用缓存。

pnpm 11 及以下版本、npm、Yarn，以及项目中调用的其他命令，继续使用原有 `package.json#bin` 解析和执行流程。

## Registry 与安全配置

pnpm 引导层读取 `COREPACK_NPM_REGISTRY` 下载平台原生包。snm 遵循以下优先级：

1. 如果用户已经设置 `COREPACK_NPM_REGISTRY`，完整保留用户配置。
2. 如果用户没有设置，则仅在子进程中将 snm 的 `npm_registry` 映射为 `COREPACK_NPM_REGISTRY`。

该环境变量不会写入 snm 父进程的全局环境。`COREPACK_NPM_TOKEN`、`COREPACK_NPM_USERNAME`、`COREPACK_NPM_PASSWORD`、`COREPACK_INTEGRITY_KEYS` 和 `COREPACK_ENABLE_NETWORK` 等现有变量由子进程自然继承，继续由 pnpm 官方引导层处理。

## 改动边界

- `crates/shim/src/pm_shim.rs`：为 pnpm 12+ 选择官方 `.mjs` 入口，并传入 registry 子进程环境。
- `crates/utils/src/exec.rs`：增加只作用于子进程的环境变量覆盖能力；原 `exec_cli` 接口和行为保持不变。
- `crates/shim/Cargo.toml`：使用 `semver` 准确识别 pnpm 12+。

下载器、包管理器 resolver、平台配置及 npm/Yarn/pnpm 旧版本执行链路均未修改。

## 取舍

该兼容路径与 Corepack 一样，会先启动 Node.js 引导层，再由引导层启动原生 pnpm。相比 snm 自行直接执行平台二进制，这会多一个很小的启动步骤；换来的收益是复用 pnpm 官方维护的平台识别、签名校验、缓存和并发安全机制，避免 snm 与 pnpm 发布结构长期耦合。

## 验证清单

- pnpm 12 的 `pnpm`/`pnpx` 入口选择单元测试。
- pnpm 9 既有端到端测试，确认旧链路不变。
- 全工作区单元测试与端到端测试。
- 真实 pnpm 12 主包烟测：首次下载原生程序，第二次复用缓存并正常输出版本。
- 格式化、差异检查和受影响 crate 的 Clippy 检查。
