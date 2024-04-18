# snm



## Install

```bash
curl -fsSL https://raw.githubusercontent.com/sheinsight/snm/main/install.sh | bash
```

--install-dir

Set a custom directory for snm to be installed.

--skip-shell

Skip appending shell specific loader to shell config file

--force-install

force use shell installed

--version 

Install specified version

```
curl -fsSL https://raw.githubusercontent.com/sheinsight/snm/main/install.sh | bash -s -- --install-dir "./.fnm" --skip-shell
```

RUST_BACKTRACE=1 cargo run -p cli --bin main -- node default 21.4.0

RUST_BACKTRACE=1 cargo run -p cli --bin main -- node list

RUST_BACKTRACE=1 cargo run -p cli --bin main -- node install 20.11.1

export PATH=$HOME/GitRepository/snm/target/debug:$PATH 


- [x] snm node list
- [x] snm node list-remote
- [x] snm node install 20.11.1
- [x] snm node uninstall 20.11.1
- [x] snm node default 20.11.1
- [x] snm node env


- [x] corepack auto download
- [x] check valid package manager
- [x] execute package manager command


- [ ] snm install
- [ ] snm clean-install
- [ ] snm uninstall 
- [ ] snm update
- [ ] snm dedupe
- [ ] snm pack
- [ ] snm publish
- [ ] snm run
- [ ] snm exec [name]


SNM_NODE_BIN_DIR = '~/.snm/bin'
SNM_DOWNLOAD_DIR = '~/.snm/download'
SNM_NODE_MODULES_DIR = '~/.snm/node_modules'





snm     add     [package-spec]      (-O --save-optional)    (-D --save-dev)     (-P --save-prod)    (-E --save-exact)
npm     add     [package-spec]      (-O --save-optional)    (-D --save-dev)     (-P --save-prod)    (-E --save-exact)
pnpm    add     [package-spec]      (-O --save-optional)    (-D --save-dev)     (-P --save-prod)    (-E --save-exact)
yarn1   add     [package-spec]      (-O --optional)         (-D --dev)          (-P --peer)         (-E --exact)
yarn2   add     [package-spec]      (-O --optional)         (-D --dev)          (-P --peer)         (-E --exact) 


snm     install
npm     install     (rename npm ci if v > 7 has npm clean-install)
pnpm    install     (--frozen-lockfile)
yarn1   install     (--frozen-lockfile)
yarn2   install     (--immutable)

snm     run

snx
npm     npx
pnpm    pnpm dlx
yarn1   😭
yarn2   yarn dlx




snm     query -i 自研


snm     update(up)  -i  自研
