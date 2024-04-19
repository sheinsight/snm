# SNM

## 🤔 What is SNM ?

snm = corepack + fnm + ni .

- 📦 Node、Npm、Pnpm、Yarn Version Manager
- 💡 Use the right package manager
- ✅ Verify if package manager meets the 'packageManager' configuration
- 🌟 CodeWhisperer ( Fig ) Friendly

![](./assets/fig.png)

## ⚙️ How to install

```bash
curl -fsSL https://raw.githubusercontent.com/sheinsight/snm/main/install.sh | bash
```

### Parameters

`--install-dir`

Custom installation directory, default is `~/.snm`.

Example:

```bash
curl -fsSL https://raw.githubusercontent.com/sheinsight/snm/main/install.sh | bash -s -- --install-dir "./.snm"
```

`--skip-shell`

Skip automatic configuration of shell environment variables .

If you install directory to `/bin` directory, you may not need to configure shell environment variables.

Example:

```bash
curl -fsSL https://raw.githubusercontent.com/sheinsight/snm/main/install.sh | bash -s -- --skip-shell
```

`--force-install`

Forcing the use of shell scripts for installation


Example:

```bash
curl -fsSL https://raw.githubusercontent.com/sheinsight/snm/main/install.sh | bash -s -- --force-install
```

`--version`

Specify the installation version

Example:

```bash
curl -fsSL https://raw.githubusercontent.com/sheinsight/snm/main/install.sh | bash -s -- --version "0.0.1-27"
```


Of course, you can combine multiple parameters. Example:

```bash
curl -fsSL https://raw.githubusercontent.com/sheinsight/snm/main/install.sh | bash -s -- --install-dir "./.fnm" --skip-shell --version "0.0.1-27"
```


## Environment Variables

### Local directory configuration

|name|default|description|
|---|---|---|
|SNM_BASE_DIR|~/.snm|Installation directory|
|SNM_NODE_BIN_DIR|~/.snm/bin|Node binary directory|
|SNM_DOWNLOAD_DIR|~/.snm/download|Download directory|
|SNM_NODE_MODULES_DIR|~/.snm/node_modules|Node modules directory|

### Remote resource configuration


|name|default|description|
|---|---|---|
|SNM_NPM_REGISTRY_HOST|https://registry.npmjs.org|Npm host|
|SNM_YARN_REGISTRY_HOST_KEY|https://registry.yarnpkg.com|Yarn registry , Used by less 2.0.0|
|SNM_YARN_REPO_HOST_KEY|https://repo.yarnpkg.com|Yarn registry , Used by greater 2.0.0|
|SNM_NODEJS_HOST_KEY|https://nodejs.org|Nodejs Host|
|SNM_NODEJS_GITHUB_RESOURCE_HOST_KEY|https://raw.githubusercontent.com|Github resource host|

### Function configuration

|name|default|description|
|---|---|---|
|SNM_STRICT|false|strict mode|
|SNM_NODE_INSTALL_STRATEGY|ask|Install Strategy , Optional `ask`\|`panic`\|`auto`|
|SNM_PACKAGE_MANAGER_INSTALL_STRATEGY|ask|Install Strategy , Optional `ask`\|`panic`\|`auto`|

## Todo List


### Node Manager

- [x] snm node list
- [x] snm node list-remote
- [x] snm node install 20.11.1
- [x] snm node uninstall 20.11.1
- [x] snm node default 20.11.1
- [ ] snm node env

### Npm Manager

- [x] snm npm list
- [ ] snm npm list-remote
- [x] snm npm install 7.5.6
- [x] snm npm uninstall 7.5.6
- [x] snm npm default 7.5.6

### Pnpm Manager

- [x] snm pnpm list
- [ ] snm pnpm list-remote
- [x] snm pnpm install 6.7.5
- [x] snm pnpm uninstall 6.7.5
- [x] snm pnpm default 6.7.5

### Yarn Manager

- [x] snm yarn list
- [ ] snm yarn list-remote
- [x] snm yarn install 1.22.10
- [x] snm yarn uninstall 1.22.10
- [x] snm yarn default 1.22.10


### CodeWhisperer

- [x] snm fig-spec

### Use the right package manager

- [x] snm install
- [x] snm ci
- [x] snm add
- [ ] snm delete
- [x] snm run
- [ ] snm dlx
- [ ] snm exec


### Self Developed

- [ ] snm query
- [ ] snm bump
- [ ] snm outdated
- [ ] snm update
- [ ] snm dedupe



- [x] corepack auto download
- [x] check valid package manager
- [x] execute package manager command

 