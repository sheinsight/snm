# CI 与本地开发统一使用固定版本，避免工具或传递依赖升级导致构建结果随时间漂移。
cargo_binstall_version := "1.22.0"
dev_tools := "cargo-insta@1.48.0 taplo-cli@0.10.0 cargo-deny@0.20.2 watchexec-cli@2.2.1"

setup:
  # 同时固定安装脚本和安装器版本，防止脚本内容或默认下载版本发生非预期变化。
  curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/v{{cargo_binstall_version}}/install-from-binstall-release.sh | env BINSTALL_VERSION={{cargo_binstall_version}} bash
  # 固定工具版本并锁定源码依赖；即使预编译产物缺失，回退编译也与项目工具链兼容。
  cargo binstall {{dev_tools}} --locked -y --force
  @echo '✅ Setup complete!'

ready:
  just fmt
  # 就绪检查必须使用已提交的依赖图，禁止 CI 静默更新锁文件。
  cargo c --locked --verbose
  # t 包含了 e2e 测试，所以必须要 build 一下
  cargo b --locked --verbose
  cargo t --locked
  #just ci-e2e
  #just lint 
  @echo '✅ All passed!'

fmt:
    cargo fmt --all -- --emit=files
    taplo fmt 
    #pnpm format
    @echo '✅ Format complete!'

# lint:
#   cargo lint -- --deny warnings

build-release:
    echo "Building the project..."
    # 发布构建必须与 CI 使用相同的锁定依赖图。
    cargo b --locked --verbose --release

build:
    cargo b --verbose
    @echo '✅ Build debug complete!'

prerelease:
    echo "Building the project for release..."
    ./target/debug/tools


watch:
    echo "Running the project..."
    # cargo watch -x build
    watchexec -r -e rs cargo build

test:
    echo "Running tests..."
    cargo test --locked --workspace --exclude e2e -- --nocapture

e2e:
    echo "Running end-to-end tests..."
    cargo insta test -p e2e --review -- --nocapture 

e2e-watch:
    echo "Watching end-to-end tests..."
    # cargo watch -q -c -w crates -w e2e -x "insta test -p e2e --review -- --nocapture"
    watchexec -r -e rs cargo insta test -p e2e --review -- --nocapture

ci-e2e:
    echo "Running end-to-end tests..."
    cargo insta test -p e2e -- --nocapture
