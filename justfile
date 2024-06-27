
build:
    echo "Building the project..."
    cargo b

dev:
    echo "Running the project..."
    cargo watch -x build

test:
    echo "Running tests..."
    cargo t

