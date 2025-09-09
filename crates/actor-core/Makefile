# Actor Core Makefile
# Provides convenient commands for development and CI

.PHONY: help test test-configs validate-configs clean build check format clippy

# Default target
help:
	@echo "Actor Core Development Commands:"
	@echo "  test              - Run all tests"
	@echo "  test-configs      - Run configuration validation tests"
	@echo "  validate-configs  - Validate YAML configuration files"
	@echo "  build             - Build the project"
	@echo "  check             - Check code without building"
	@echo "  format            - Format code with rustfmt"
	@echo "  clippy            - Run clippy lints"
	@echo "  clean             - Clean build artifacts"

# Run all tests
test:
	cargo test

# Run configuration validation tests
test-configs:
	cargo test --test config_validation_tests --verbose
	cargo test --test config_validation_tests --features extra_buckets --verbose

# Validate configuration files
validate-configs:
	python3 scripts/validate_configs.py

# Build the project
build:
	cargo build

# Check code without building
check:
	cargo check

# Format code
format:
	cargo fmt

# Run clippy
clippy:
	cargo clippy --all-targets --all-features -D warnings

# Clean build artifacts
clean:
	cargo clean

# CI validation (runs all checks)
ci: validate-configs test-configs test clippy
	@echo "✅ All CI checks passed!"
