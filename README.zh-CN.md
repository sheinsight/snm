# SNM

<div align="center">
  <h1>SNM - 智能 Node 管理器</h1>
  <p>强大的一体化 Node.js 版本与包管理工具</p>
</div>

[English](./README.md) | 简体中文

## ✨ 特性

SNM 完美结合了 [corepack](https://github.com/nodejs/corepack)、[fnm](https://github.com/Schniz/fnm) 和 [ni](https://github.com/antfu/ni) 的优秀特性：

- 📦 统一管理 Node.js、npm、pnpm 和 Yarn 版本
- 💡 基于项目配置智能切换包管理器
- ✅ 自动校验包管理器是否符合 `packageManager` 配置
- 🔄 根据 `.node-version` 文件自动切换 Node.js 版本
- 🌟 通过 CodeWhisperer (Fig) 集成提供增强的命令行体验
- 🚀 基于 Rust 实现的极致性能

![SNM CLI 演示](./assets/fig.png)

## 🚀 安装

### 快速安装（macOS/Linux）

```bash
curl -fsSL https://raw.githubusercontent.com/sheinsight/snm/main/install.sh | bash
```

### 安装选项

安装程序支持以下配置选项：

#### 自定义安装目录

```bash
curl -fsSL https://raw.githubusercontent.com/sheinsight/snm/main/install.sh | bash -s -- --install-dir "./.snm"
```

#### 跳过 Shell 配置

```bash
curl -fsSL https://raw.githubusercontent.com/sheinsight/snm/main/install.sh | bash -s -- --skip-shell
```

#### 安装指定版本

```bash
curl -fsSL https://raw.githubusercontent.com/sheinsight/snm/main/install.sh | bash -s -- --release "0.0.1-27"
```

你可以组合多个选项：

```bash
curl -fsSL https://raw.githubusercontent.com/sheinsight/snm/main/install.sh | bash -s -- --install-dir "./.snm" --skip-shell --release "0.0.1-27"
```

## ⚙️ 配置

SNM 可以通过环境变量进行自定义配置：

### 工作空间配置

| 变量名       | 默认值 | 描述         |
| ------------ | ------ | ------------ |
| SNM_HOME_DIR | ~      | 工作空间目录 |

### 远程资源配置

| 变量名                        | 默认值                            | 描述             |
| ----------------------------- | --------------------------------- | ---------------- |
| SNM_NPM_REGISTRY_HOST         | https://registry.npmjs.org        | npm 注册表 URL   |
| SNM_NODE_DIST_URL             | https://nodejs.org/dist           | Node.js 下载 URL |
| SNM_NODE_GITHUB_RESOURCE_HOST | https://raw.githubusercontent.com | GitHub 资源主机  |

### 行为设置

| 变量名     | 默认值 | 描述                       |
| ---------- | ------ | -------------------------- |
| SNM_STRICT | false  | 启用包管理器验证的严格模式 |

## 📖 文档

有关详细使用说明和高级配置选项，请访问我们的[文档](https://github.com/sheinsight/snm/wiki)。

## 🤝 贡献

我们欢迎各种形式的贡献！详情请参阅我们的[贡献指南](CONTRIBUTING.md)。

## 📄 许可证

MIT License © 2024 [SheinSight](https://github.com/sheinsight)
