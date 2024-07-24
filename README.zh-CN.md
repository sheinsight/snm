# snm

[English](./README.md) | 简体中文

snm = [fnm](https://github.com/Schniz/fnm) + [corepack](https://github.com/nodejs/corepack) + [ni](https://github.com/antfu-collective/ni) .

## ✨ 特性

- 📦 node、npm、pnpm、yarn 版本管理工具
- 🤡 根据你配置的 `packageManager` 自动的切换对应的包管理器
- ✅ 检查你使用的命令是否符合 `packageManager` 的约定
- 😄 根据当前工作目录下 .node-version 文件内声明的 node 版本自动切换
- 🌟 CodeWhisperer 友好

  ![](./assets/fig.png)

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

| 配置项                        | 默认值                            | 功能描述                        |
| ----------------------------- | --------------------------------- | ------------------------------- |
| SNM_NODE_BIN_DIR              | node_bin                          | node 的二进制存储目录           |
| SNM_DOWNLOAD_DIR              | downloads                         | 文件的下载目录                  |
| SNM_NODE_MODULES_DIR          | node_modules                      | npm 、pnpm、yarn 的模块存储目录 |
| SNM_NODE_DIST_URL             | https://nodejs.org/dist           | nodejs 元数据的获取地址         |
| SNM_DOWNLOAD_TIMEOUT_SECS     | 60                                | 下载超时时间 ( 单位为 `秒` )    |
| SNM_NODE_GITHUB_RESOURCE_HOST | https://raw.githubusercontent.com | GITHUB_RESOURCE 地址            |
| SNM_NODE_INSTALL_STRATEGY     | auto                              | node 的安装策略                 |
| SNM_NODE_WHITE_LIST           | ""                                | node 白名单版本                 |

### SNM_NODE_BIN_DIR

这是 snm 安装 node 的存储路径，路径规则 `$HOME/.snm/$SNM_NODE_BIN_DIR`

### SNM_DOWNLOAD_DIR

这是 snm 的下载 node、npm、pnpm、yarn 等压缩包的存储路径，路径规则 `$HOME/.snm/$SNM_DOWNLOAD_DIR` , 一般在我们正确解压缩之后会删除下载文件。

### SNM_NODE_MODULES_DIR

这是 snm 存放 npm、pnpm、yarn 的目录，路径规则：`$HOME/.snm/$SNM_NODE_MODULES_DIR`

### SNM_NODE_DIST_URL

snm 获取 node 最新版本信息的地址 , 并且我们的下载前缀也会使用这个。 如果你试图搭建代理站，请保证站点结构与官方一致。

### SNM_DOWNLOAD_TIMEOUT_SECS

下载超时时间，单位为 秒

### SNM_NODE_GITHUB_RESOURCE_HOST

主要用于获取 node schedule 信息，如果你搭建私有代理站点，

请注意满足路径 `https://raw.githubusercontent.com/nodejs/Release/main/schedule.json`

### SNM_NODE_INSTALL_STRATEGY

node 的安装策略，可选值范围如下：

- ask ( 询问用户是否需要安装，这也是默认值 )
- auto （ 静默安装 ）

### SNM_NODE_WHITE_LIST

可执行的 node 白名单列表
