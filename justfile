


setup:
    cargo install cargo-binstall
    # cargo binstall taplo-cli cargo-release cargo-insta cargo-deny cargo-watch -y --force
    cargo install taplo-cli cargo-release cargo-insta cargo-deny cargo-watch 
    @echo '✅ Setup complete!'

ready:
  just fmt    
  # just lint 
  @echo '✅ All passed!'

fmt:
    cargo fmt --all -- --emit=files
    taplo fmt 
    # pnpm format
    @echo '✅ Format complete!'

build:
    echo "Building the project..."
    cargo b --verbose --release 

build-debug:
    cargo b --verbose
    @echo '✅ Build debug complete!'

check:
    cargo check --verbose
    cargo b --verbose
    cargo t
    just e2e
    @echo '✅ Check complete!'

prerelease:
    echo "Building the project for release..."
    ./target/debug/tools

watch:
    echo "Running the project..."
    cargo watch -x build

qtest:
    echo "Running tests..."
    cargo test --workspace --exclude e2e -- --nocapture

test:
    echo "Running tests..."
    cargo test --workspace --exclude e2e -- --nocapture

e2e:
    echo "Running end-to-end tests..."
    # cargo test --package e2e -- --nocapture
    cargo insta test -p e2e --review    
