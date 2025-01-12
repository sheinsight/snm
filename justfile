


setup:
  cargo install --locked cargo-binstall
  cargo binstall taplo-cli cargo-release cargo-insta cargo-deny cargo-watch -y 
  # cargo install taplo-cli cargo-release cargo-insta cargo-deny cargo-watch 
  @echo '✅ Setup complete!'

ready:
  just fmt    
  cargo c --verbose
  cargo b --verbose
  cargo t
  #just ci-e2e
  # just lint 
  @echo '✅ All passed!'

fmt:
    cargo fmt --all -- --emit=files
    taplo fmt 
    # pnpm format
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
    cargo watch -x build

test:
    echo "Running tests..."
    cargo test --workspace --exclude e2e -- --nocapture

e2e:
    echo "Running end-to-end tests..."
    cargo insta test -p e2e --review -- --nocapture


ci-e2e:
    echo "Running end-to-end tests..."
    cargo insta test -p e2e --update -- --nocapture
