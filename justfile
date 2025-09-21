# Justfile for Chaos Backend Service Development

# Set environment variables to suppress warnings
set RUSTFLAGS := "-A unused_variables -A unused_imports -A dead_code -A unused_mut"

# Build without warnings
build:
    cargo build

# Check without warnings  
check:
    cargo check

# Test without warnings
test:
    cargo test

# Run examples without warnings
examples:
    cargo run --example new_architecture_demo
    cargo run --example add_new_function_demo

# Run benchmarks without warnings
bench:
    cargo bench

# Clean build artifacts
clean:
    cargo clean

# Format code
fmt:
    cargo fmt

# Lint code (with warnings suppressed for examples)
lint:
    cargo clippy -- -A unused_variables -A unused_imports -A dead_code -A unused_mut

# Development mode - suppress all warnings
dev:
    set RUSTFLAGS="-A unused_variables -A unused_imports -A dead_code -A unused_mut" && cargo check
