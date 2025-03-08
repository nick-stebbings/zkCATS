# List available recipes
default:
    @just --list

# Complete development environment: init DB, run server in background, then client
dev:
    #!/usr/bin/env bash
    # Initialize the database first
    echo "Initializing database..."
    ./scripts/init_db.sh

    # Start the web server in the background
    echo "Starting web server..."
    cargo run -p web_server &
    SERVER_PID=$!

    # Give the server a moment to start up
    sleep 1

    # Run the client example
    echo "Starting client..."
    cargo run --example client -p web_server

    # Clean up the server when the client exits
    kill $SERVER_PID

# Development with file watching
dev-watch:
    #!/usr/bin/env bash
    # Initialize the database first
    echo "Initializing database..."
    ./scripts/init_db.sh

    # Start web server with watching
    echo "Starting web server with watching..."
    # Use multiple terminals or a terminal multiplexer like tmux for this
    # This particular command should be run in a separate terminal
    cargo watch -q -c -w crates/lib/services/web_server/src/ -x "run -p web_server"


# Initialize the database using Docker
init-db:
    ./scripts/init_db.sh

# Run the web server specifically
run-ws:
    cargo run -p web_server --bin web_server

# Run the web server with watch mode for development (requires cargo-watch)
watch-ws:
    cargo watch -q -c -w crates/lib/services/web_server/src/ -x "run -p web_server"

# Run with watch mode for development (requires cargo-watch)
watch:
    cargo watch -x run

# Build the project
build:
    cargo build

# Build for production (optimized)
build-release:
    cargo build --release

# Check for errors
check:
    cargo check

# Run tests
test:
    cargo test

# Run clippy for linting
lint:
    cargo clippy -- -D warnings

# Format code
format:
    cargo fmt --all

# Clean the project
clean:
    cargo clean

# Generate documentation
docs:
    cargo doc --open

# Audit dependencies for security vulnerabilities
audit:
    cargo audit

# Update dependencies
update-deps:
    cargo update

# Database migrations (if using diesel)
migrate:
    diesel migration run

# Revert last migration (if using diesel)
migrate-revert:
    diesel migration revert

# Combined quality checks before committing
pre-commit: format lint test

# Deploy to production (customize as needed)
deploy: build-release
    @echo "Deploying application..."
    # Add your deployment commands here