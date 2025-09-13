# Chaos Backend Service - Actor Core Makefile
# This Makefile provides development commands and CI integration

.PHONY: help build test bench clean lint format check security coverage docs examples install-tools
.DEFAULT_GOAL := help

# Configuration
CARGO := cargo
PACKAGE := actor-core
FEATURES_DEFAULT := 
FEATURES_ALL := --all-features
FEATURES_HEAVY := --features="heavy-deps"

# Colors for output
RED := \033[31m
GREEN := \033[32m
YELLOW := \033[33m
BLUE := \033[34m
RESET := \033[0m

help: ## Show this help message
	@echo "$(BLUE)Chaos Backend Service - Actor Core$(RESET)"
	@echo "$(BLUE)=====================================$(RESET)"
	@echo ""
	@echo "Available commands:"
	@awk 'BEGIN {FS = ":.*##"; printf "\n"} /^[a-zA-Z_-]+:.*?##/ { printf "  $(GREEN)%-20s$(RESET) %s\n", $$1, $$2 }' $(MAKEFILE_LIST)
	@echo ""
	@echo "$(YELLOW)Feature sets:$(RESET)"
	@echo "  default     - Core functionality only"
	@echo "  heavy-deps  - All heavy dependencies enabled"
	@echo "  all         - All features enabled"

# Installation and Setup
install-tools: ## Install required development tools
	@echo "$(BLUE)Installing development tools...$(RESET)"
	$(CARGO) install cargo-audit
	$(CARGO) install cargo-deny
	$(CARGO) install cargo-llvm-cov
	$(CARGO) install cargo-criterion
	$(CARGO) install cargo-expand
	$(CARGO) install cargo-machete
	@echo "$(GREEN)Tools installed successfully$(RESET)"

setup: install-tools ## Set up development environment
	@echo "$(BLUE)Setting up development environment...$(RESET)"
	rustup component add rustfmt clippy llvm-tools-preview
	@echo "$(GREEN)Development environment ready$(RESET)"

# Building
build: ## Build the project (default features)
	@echo "$(BLUE)Building actor-core (default features)...$(RESET)"
	$(CARGO) build -p $(PACKAGE)

build-all: ## Build with all features
	@echo "$(BLUE)Building actor-core (all features)...$(RESET)"
	$(CARGO) build -p $(PACKAGE) $(FEATURES_ALL)

build-release: ## Build release version
	@echo "$(BLUE)Building actor-core (release)...$(RESET)"
	$(CARGO) build -p $(PACKAGE) --release $(FEATURES_ALL)

build-feature-matrix: ## Build with all feature combinations
	@echo "$(BLUE)Testing feature matrix...$(RESET)"
	@for features in "" "moka-cache" "memory-mapped" "redis-cache" "mongodb-storage" "sqlx-database" "cli-tools" "heavy-deps"; do \
		echo "$(YELLOW)Building with features: $$features$(RESET)"; \
		if [ -z "$$features" ]; then \
			$(CARGO) build -p $(PACKAGE); \
		else \
			$(CARGO) build -p $(PACKAGE) --features="$$features"; \
		fi; \
	done
	@echo "$(GREEN)Feature matrix build complete$(RESET)"

# Testing
test: ## Run tests (default features)
	@echo "$(BLUE)Running tests (default features)...$(RESET)"
	$(CARGO) test -p $(PACKAGE) --lib

test-all: ## Run all tests with all features
	@echo "$(BLUE)Running all tests (all features)...$(RESET)"
	$(CARGO) test -p $(PACKAGE) --lib $(FEATURES_ALL)
	$(CARGO) test -p $(PACKAGE) --test '*' $(FEATURES_ALL)

test-integration: ## Run integration tests
	@echo "$(BLUE)Running integration tests...$(RESET)"
	$(CARGO) test -p $(PACKAGE) --test '*' $(FEATURES_ALL)

test-feature-matrix: ## Test all feature combinations
	@echo "$(BLUE)Testing feature matrix...$(RESET)"
	@for features in "" "moka-cache" "memory-mapped" "redis-cache" "mongodb-storage" "heavy-deps"; do \
		echo "$(YELLOW)Testing with features: $$features$(RESET)"; \
		if [ -z "$$features" ]; then \
			$(CARGO) test -p $(PACKAGE) --lib; \
		else \
			$(CARGO) test -p $(PACKAGE) --lib --features="$$features"; \
		fi; \
	done
	@echo "$(GREEN)Feature matrix testing complete$(RESET)"

test-performance: ## Run performance tests
	@echo "$(BLUE)Running performance tests...$(RESET)"
	$(CARGO) test -p $(PACKAGE) --release --test performance_tests $(FEATURES_ALL)

# Benchmarking
bench: ## Run benchmarks
	@echo "$(BLUE)Running benchmarks...$(RESET)"
	$(CARGO) bench -p $(PACKAGE) $(FEATURES_ALL)

bench-baseline: ## Create benchmark baseline
	@echo "$(BLUE)Creating benchmark baseline...$(RESET)"
	$(CARGO) bench -p $(PACKAGE) $(FEATURES_ALL) -- --save-baseline main

bench-compare: ## Compare benchmarks with baseline
	@echo "$(BLUE)Comparing benchmarks with baseline...$(RESET)"
	$(CARGO) bench -p $(PACKAGE) $(FEATURES_ALL) -- --baseline main

# Code Quality
lint: ## Run Clippy linter
	@echo "$(BLUE)Running Clippy...$(RESET)"
	$(CARGO) clippy -p $(PACKAGE) -- -D warnings
	$(CARGO) clippy -p $(PACKAGE) $(FEATURES_ALL) -- -D warnings

format: ## Format code
	@echo "$(BLUE)Formatting code...$(RESET)"
	$(CARGO) fmt --all

format-check: ## Check code formatting
	@echo "$(BLUE)Checking code formatting...$(RESET)"
	$(CARGO) fmt --all -- --check

check: ## Run cargo check
	@echo "$(BLUE)Running cargo check...$(RESET)"
	$(CARGO) check -p $(PACKAGE)
	$(CARGO) check -p $(PACKAGE) $(FEATURES_ALL)

# Security and Dependencies
security: ## Run security audit
	@echo "$(BLUE)Running security audit...$(RESET)"
	$(CARGO) audit
	$(CARGO) deny check -d crates/$(PACKAGE)/deny.toml

deps-check: ## Check for unused dependencies
	@echo "$(BLUE)Checking for unused dependencies...$(RESET)"
	$(CARGO) machete

deps-update: ## Update dependencies
	@echo "$(BLUE)Updating dependencies...$(RESET)"
	$(CARGO) update

# Documentation
docs: ## Build documentation
	@echo "$(BLUE)Building documentation...$(RESET)"
	$(CARGO) doc -p $(PACKAGE) $(FEATURES_ALL) --no-deps

docs-open: ## Build and open documentation
	@echo "$(BLUE)Building and opening documentation...$(RESET)"
	$(CARGO) doc -p $(PACKAGE) $(FEATURES_ALL) --no-deps --open

docs-test: ## Test documentation examples
	@echo "$(BLUE)Testing documentation examples...$(RESET)"
	$(CARGO) test -p $(PACKAGE) --doc $(FEATURES_ALL)

# Examples
examples: ## Run all examples
	@echo "$(BLUE)Running examples...$(RESET)"
	$(CARGO) run -p $(PACKAGE) --example basic_usage --features=cli-tools
	$(CARGO) run -p $(PACKAGE) --example configuration_example $(FEATURES_ALL)
	$(CARGO) run -p $(PACKAGE) --example resource_manager_example $(FEATURES_ALL)
	$(CARGO) run -p $(PACKAGE) --example subsystem_example --features=cli-tools
	$(CARGO) run -p $(PACKAGE) --example performance_workflow_example $(FEATURES_ALL)

examples-check: ## Check that examples compile
	@echo "$(BLUE)Checking examples compilation...$(RESET)"
	@for example in basic_usage configuration_example resource_manager_example subsystem_example performance_workflow_example; do \
		echo "$(YELLOW)Checking example: $$example$(RESET)"; \
		$(CARGO) check -p $(PACKAGE) --example $$example $(FEATURES_ALL); \
	done
	@echo "$(GREEN)All examples compile successfully$(RESET)"

# Coverage
coverage: ## Generate test coverage report
	@echo "$(BLUE)Generating coverage report...$(RESET)"
	$(CARGO) llvm-cov -p $(PACKAGE) $(FEATURES_ALL) --html --output-dir coverage

coverage-lcov: ## Generate LCOV coverage report
	@echo "$(BLUE)Generating LCOV coverage report...$(RESET)"
	$(CARGO) llvm-cov -p $(PACKAGE) $(FEATURES_ALL) --lcov --output-path lcov.info

# Configuration
validate-configs: ## Validate configuration files
	@echo "$(BLUE)Validating configuration files...$(RESET)"
	cd crates/$(PACKAGE) && python scripts/validate_configs.py

# Cleaning
clean: ## Clean build artifacts
	@echo "$(BLUE)Cleaning build artifacts...$(RESET)"
	$(CARGO) clean

clean-all: ## Clean all artifacts including coverage and benchmarks
	@echo "$(BLUE)Cleaning all artifacts...$(RESET)"
	$(CARGO) clean
	rm -rf coverage/
	rm -f lcov.info
	rm -rf target/criterion/

# CI Commands (used by GitHub Actions)
ci-check: format-check lint check ## Run CI checks
	@echo "$(GREEN)CI checks passed$(RESET)"

ci-test: test-all test-integration docs-test ## Run CI tests
	@echo "$(GREEN)CI tests passed$(RESET)"

ci-feature-matrix: build-feature-matrix test-feature-matrix ## Run CI feature matrix
	@echo "$(GREEN)CI feature matrix passed$(RESET)"

ci-security: security ## Run CI security checks
	@echo "$(GREEN)CI security checks passed$(RESET)"

ci-full: ci-check ci-test ci-feature-matrix ci-security coverage ## Run full CI pipeline
	@echo "$(GREEN)Full CI pipeline completed$(RESET)"

# Development workflow
dev-check: format lint test ## Quick development check
	@echo "$(GREEN)Development check completed$(RESET)"

dev-full: clean build-all test-all examples docs ## Full development build
	@echo "$(GREEN)Full development build completed$(RESET)"

# Release preparation
pre-release: ci-full bench validate-configs ## Prepare for release
	@echo "$(GREEN)Pre-release checks completed$(RESET)"

# Performance monitoring
perf-monitor: ## Run performance monitoring
	@echo "$(BLUE)Running performance monitoring...$(RESET)"
	$(CARGO) run -p $(PACKAGE) --example performance_workflow_example $(FEATURES_ALL)

# Debug utilities
expand: ## Expand macros for debugging
	@echo "$(BLUE)Expanding macros...$(RESET)"
	$(CARGO) expand -p $(PACKAGE) > expanded.rs

# Environment info
env-info: ## Show environment information
	@echo "$(BLUE)Environment Information$(RESET)"
	@echo "$(BLUE)======================$(RESET)"
	@echo "Rust version: $$(rustc --version)"
	@echo "Cargo version: $$(cargo --version)"
	@echo "OS: $$(uname -a)"
	@echo "CPU cores: $$(nproc)"
	@echo "Memory: $$(free -h | grep Mem | awk '{print $$2}')"
	@echo "Package: $(PACKAGE)"
	@echo "Features available: $$($(CARGO) metadata --format-version 1 --no-deps | jq -r '.packages[] | select(.name == "$(PACKAGE)") | .features | keys[]' | tr '\n' ' ')"
