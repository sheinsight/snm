# SNM

<div align="center">
  <h1>SNM - Smart Node Manager</h1>
  <p>A powerful all-in-one Node.js version and package manager</p>
</div>

## ✨ Features

SNM combines the best features of [corepack](https://github.com/nodejs/corepack), [fnm](https://github.com/Schniz/fnm), and [ni](https://github.com/antfu/ni) to provide:

- 📦 Unified management of Node.js, npm, pnpm, and Yarn versions
- 💡 Intelligent package manager auto-switching based on project configuration
- ✅ Automatic validation of package manager against `packageManager` field
- 🔄 Seamless Node.js version switching based on `.node-version` file
- 🌟 Enhanced CLI experience with CodeWhisperer (Fig) integration
- 🚀 Lightning-fast performance with Rust implementation

![SNM CLI Demo](./assets/fig.png)

## 🚀 Installation

### Quick Install (macOS/Linux)

```bash
curl -fsSL https://raw.githubusercontent.com/sheinsight/snm/main/install.sh | bash
```

### Installation Options

The installer supports several configuration options:

#### Custom Installation Directory

```bash
curl -fsSL https://raw.githubusercontent.com/sheinsight/snm/main/install.sh | bash -s -- --install-dir "./.snm"
```

#### Skip Shell Configuration

```bash
curl -fsSL https://raw.githubusercontent.com/sheinsight/snm/main/install.sh | bash -s -- --skip-shell
```

#### Install Specific Version

```bash
curl -fsSL https://raw.githubusercontent.com/sheinsight/snm/main/install.sh | bash -s -- --release "0.0.1-27"
```

You can combine multiple options:

```bash
curl -fsSL https://raw.githubusercontent.com/sheinsight/snm/main/install.sh | bash -s -- --install-dir "./.snm" --skip-shell --release "0.0.1-27"
```

## ⚙️ Configuration

SNM can be customized through environment variables:

### Workspace Configuration

| Variable     | Default | Description         |
| ------------ | ------- | ------------------- |
| SNM_HOME_DIR | ~/      | Workspace directory |

### Remote Resources

| Variable                      | Default                           | Description          |
| ----------------------------- | --------------------------------- | -------------------- |
| SNM_NPM_REGISTRY_HOST         | https://registry.npmjs.org        | npm registry URL     |
| SNM_NODE_DIST_URL             | https://nodejs.org/dist           | Node.js download URL |
| SNM_NODE_GITHUB_RESOURCE_HOST | https://raw.githubusercontent.com | GitHub resource host |

### Behavior Settings

| Variable   | Default | Description                                       |
| ---------- | ------- | ------------------------------------------------- |
| SNM_STRICT | false   | Enable strict mode for package manager validation |

## 📖 Documentation

For detailed usage instructions and advanced configuration options, please visit our [documentation](https://github.com/sheinsight/snm/wiki).

## 🤝 Contributing

We welcome contributions! Please see our [Contributing Guide](CONTRIBUTING.md) for details.

## 📄 License

MIT License © 2024 [SheinSight](https://github.com/sheinsight)
