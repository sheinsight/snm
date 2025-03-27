setup:
  cargo install --locked cargo-binstall
  cargo binstall taplo-cli cargo-insta cargo-deny -y --force
  cargo binstall watchexec-cli --version 2.2.0 -y --force
  @echo '✅ Setup complete!'

ready:
  just fmt    
  cargo c --verbose
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
