
build:
    echo "Building the project..."
    cargo b

prerelease:
    echo "Building the project for release..."
    ./target/debug/tools

dev:
    echo "Running the project..."
    cargo watch -x build

qtest:
    echo "Running tests..."
    cargo qtest

test:
    echo "Running tests..."
    cargo t
