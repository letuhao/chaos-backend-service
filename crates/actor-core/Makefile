# Actor Core Makefile
# Provides convenient commands for development and CI

.PHONY: help test test-configs validate-configs clean build check format clippy ci audit outdated docs bench bench-quick bench-comprehensive bench-actor bench-bucket bench-registry bench-extra bench-report

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
	@echo "  ci                - Run all CI checks"
	@echo "  audit             - Run security audit"
	@echo "  outdated          - Check for outdated dependencies"
	@echo "  docs              - Generate documentation"
	@echo "  bench             - Run all benchmarks"
	@echo "  bench-quick       - Run quick benchmarks"
	@echo "  bench-comprehensive - Run comprehensive benchmarks"
	@echo "  bench-actor       - Run actor benchmarks only"
	@echo "  bench-bucket      - Run bucket processor benchmarks only"
	@echo "  bench-registry    - Run registry loader benchmarks only"
	@echo "  bench-extra       - Run benchmarks with extra buckets feature"
	@echo "  bench-report      - Generate benchmark reports"

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

# Run security audit
audit:
	cargo audit

# Check for outdated dependencies
outdated:
	cargo outdated

# Generate documentation
docs:
	cargo doc --no-deps --all-features --open

# Run benchmarks
bench:
	cargo bench

# Run quick benchmarks
bench-quick:
	cargo bench --bench quick_benches

# Run comprehensive benchmarks
bench-comprehensive:
	cargo bench --bench comprehensive_benches

# Run actor benchmarks only
bench-actor:
	cargo bench --bench actor_benches

# Run bucket processor benchmarks only
bench-bucket:
	cargo bench --bench bucket_processor_benches

# Run registry loader benchmarks only
bench-registry:
	cargo bench --bench registry_loader_benches

# Run benchmarks with extra buckets feature
bench-extra:
	cargo bench --features extra_buckets

# Generate benchmark reports
bench-report:
	cargo bench -- --save-baseline main
	cargo bench -- --baseline main --plotting-backend gnuplot

# CI validation (runs all checks)
ci: validate-configs test-configs test clippy format check audit outdated docs
	@echo "✅ All CI checks passed!"
