# snm

[English](./README.md) | 简体中文

snm = [fnm](https://github.com/Schniz/fnm) + [corepack](https://github.com/nodejs/corepack) + [ni](https://github.com/antfu-collective/ni) .

## ✨ 特性

- 📦 node、npm、pnpm、yarn 版本管理工具
- 💡 使用正确的包管理器
- 😄 根据当前工作目录下 .node-version 文件内声明的 node 版本自动切换
- ✅ 验证包管理器是否符合'packageManager'配置
- 🌟 CodeWhisperer 友好

## 🚀 安装

### 使用脚本 (macOS/Linux)

对于 bash 、zsh 和 fish shells ，有一个自动安装脚本。

首先确保 curl 和 unzip 已经安装在您的操作系统上。然后执行：

```bash

curl -fsSL https://raw.githubusercontent.com/sheinsight/snm/main/install.sh | bash

```

#### 升级

在 macOS 上，只需 brew upgrade snm 即可。

在其他操作系统上，升级 snm 几乎与安装它相同。为了防止在您的 shell 配置文件中出现重复，请在安装命令中添加 --skip-shell。

#### 参数

--skip-shell

跳过 shell 环境变量的自动配置。一般用于升级 snm 避免重复注入 shell 。

| 类型 | 配置目录                           |
| ---- | ---------------------------------- |
| Bash | $HOME/.bashrc                      |
| Zsh  | $HOME/.zshrc                       |
| Fish | $HOME/.config/fish/conf.d/snm.fish |

### 手动安装

#### 使用 Homebrew (macOS/Linux)

```sh
brew install snm
```

#### 使用发布二进制文件 (Linux/macOS/Windows)

- 下载适用于您系统的[最新版本二进制文件](https://github.com/sheinsight/snm/releases)
- 在全局范围内将其放置在`PATH`环境变量中。
- [Set up your shell for snm](#shell-setup)

### 删除

要删除 snm , 只需要删除您的主目录中的`.snm`文件夹。您还应该编辑您的 shell 配置文件以删除任何对 snm 的引用（即阅读 [设置](#设置) ，并执行相反操作）。

## ⚙️ 设置

在 shell 中我们提供了以下的配置项目：

| 配置项                               | 默认值                            | 功能描述                        |
| ------------------------------------ | --------------------------------- | ------------------------------- |
| SNM_STRICT                           | false                             | 严格模式开关                    |
| SNM_NODE_BIN_DIR                     | node_bin                          | node 的二进制存储目录           |
| SNM_DOWNLOAD_DIR                     | downloads                         | 文件的下载目录                  |
| SNM_NODE_MODULES_DIR                 | node_modules                      | npm 、pnpm、yarn 的模块存储目录 |
| SNM_NODE_DIST_URL                    | https://nodejs.org/dist           | nodejs 元数据的获取地址         |
| SNM_DOWNLOAD_TIMEOUT_SECS            | 60                                | 下载超时时间 ( 单位为 `秒` )    |
| SNM_NODE_GITHUB_RESOURCE_HOST        | https://raw.githubusercontent.com | GITHUB_RESOURCE 地址            |
| SNM_NODE_INSTALL_STRATEGY            | auto                              | node 的安装策略                 |
| SNM_PACKAGE_MANAGER_INSTALL_STRATEGY | auto                              | 包管理器的安装策略              |

### SNM_STRICT

严格模式开关，当开启时，snm 将会进行如下的限制：

- 1
- 2
- 3
- 4
