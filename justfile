

    

setup:
  curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash
  cargo binstall cargo-insta taplo-cli cargo-deny watchexec-cli@2.2.1 -y --force
  @echo '✅ Setup complete!'

ready:
  just fmt    
  cargo c --verbose
  # t 包含了 e2e 测试，所以必须要 build 一下
  cargo b --verbose
  cargo t
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
    cargo b --verbose --release 

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
    cargo test --workspace --exclude e2e -- --nocapture

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
