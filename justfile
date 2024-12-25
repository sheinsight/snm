
build:
    echo "Building the project..."
    cargo b --verbose --release 

prerelease:
    echo "Building the project for release..."
    ./target/debug/tools

dev:
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
