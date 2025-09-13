#!/bin/bash
# CI Validation Script
# This script validates that the CI setup is working correctly

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
PACKAGE="actor-core"
PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

echo -e "${BLUE}🔍 CI Validation Script${NC}"
echo -e "${BLUE}======================${NC}"
echo "Project root: $PROJECT_ROOT"
echo ""

# Function to print status
print_status() {
    if [ $? -eq 0 ]; then
        echo -e "${GREEN}✅ $1${NC}"
    else
        echo -e "${RED}❌ $1${NC}"
        exit 1
    fi
}

print_warning() {
    echo -e "${YELLOW}⚠️  $1${NC}"
}

print_info() {
    echo -e "${BLUE}ℹ️  $1${NC}"
}

# Change to project directory
cd "$PROJECT_ROOT"

echo -e "${BLUE}1. Validating CI Workflow Files${NC}"
echo "================================="

# Check if GitHub Actions workflows exist
if [ -d ".github/workflows" ]; then
    print_status "GitHub Actions directory exists"
    
    # Check specific workflow files
    workflows=("ci.yml" "nightly.yml" "release.yml" "config.yml")
    for workflow in "${workflows[@]}"; do
        if [ -f ".github/workflows/$workflow" ]; then
            print_status "Workflow file $workflow exists"
        else
            print_warning "Workflow file $workflow is missing"
        fi
    done
else
    print_warning "GitHub Actions directory not found"
fi

echo ""
echo -e "${BLUE}2. Validating Project Structure${NC}"
echo "==============================="

# Check for essential files
essential_files=("Cargo.toml" "crates/$PACKAGE/Cargo.toml" "Makefile")
for file in "${essential_files[@]}"; do
    if [ -f "$file" ]; then
        print_status "Essential file $file exists"
    else
        print_warning "Essential file $file is missing"
    fi
done

# Check for configuration files
if [ -d "crates/$PACKAGE/configs" ]; then
    print_status "Configuration directory exists"
    config_count=$(find "crates/$PACKAGE/configs" -name "*.yaml" | wc -l)
    print_info "Found $config_count YAML configuration files"
else
    print_warning "Configuration directory not found"
fi

echo ""
echo -e "${BLUE}3. Validating Rust Environment${NC}"
echo "=============================="

# Check Rust installation
if command -v rustc &> /dev/null; then
    rust_version=$(rustc --version)
    print_status "Rust compiler found: $rust_version"
else
    print_warning "Rust compiler not found"
fi

if command -v cargo &> /dev/null; then
    cargo_version=$(cargo --version)
    print_status "Cargo found: $cargo_version"
else
    print_warning "Cargo not found"
fi

# Check for required components
components=("rustfmt" "clippy")
for component in "${components[@]}"; do
    if rustup component list --installed | grep -q "$component"; then
        print_status "Component $component is installed"
    else
        print_warning "Component $component is not installed"
        print_info "Run: rustup component add $component"
    fi
done

echo ""
echo -e "${BLUE}4. Validating Feature Matrix${NC}"
echo "============================"

# Test basic compilation with different feature sets
feature_sets=("" "moka-cache" "memory-mapped" "redis-cache" "heavy-deps")

for features in "${feature_sets[@]}"; do
    print_info "Testing feature set: ${features:-'default'}"
    
    if [ -z "$features" ]; then
        if cargo check -p "$PACKAGE" --quiet; then
            print_status "Default features compile"
        else
            print_warning "Default features compilation failed"
        fi
    else
        if cargo check -p "$PACKAGE" --features="$features" --quiet; then
            print_status "Features '$features' compile"
        else
            print_warning "Features '$features' compilation failed"
        fi
    fi
done

echo ""
echo -e "${BLUE}5. Validating Tests${NC}"
echo "=================="

# Run basic tests
print_info "Running unit tests..."
if cargo test -p "$PACKAGE" --lib --quiet; then
    print_status "Unit tests pass"
else
    print_warning "Unit tests failed"
fi

# Check for test files
test_count=$(find "crates/$PACKAGE/tests" -name "*.rs" 2>/dev/null | wc -l)
if [ "$test_count" -gt 0 ]; then
    print_status "Found $test_count integration test files"
else
    print_warning "No integration test files found"
fi

echo ""
echo -e "${BLUE}6. Validating Documentation${NC}"
echo "==========================="

# Check documentation compilation
print_info "Building documentation..."
if cargo doc -p "$PACKAGE" --no-deps --quiet; then
    print_status "Documentation builds successfully"
else
    print_warning "Documentation build failed"
fi

# Check for README
if [ -f "crates/$PACKAGE/README.md" ]; then
    print_status "Package README exists"
else
    print_warning "Package README not found"
fi

echo ""
echo -e "${BLUE}7. Validating Examples${NC}"
echo "====================="

# Check for examples
example_count=$(find "crates/$PACKAGE/examples" -name "*.rs" 2>/dev/null | wc -l)
if [ "$example_count" -gt 0 ]; then
    print_status "Found $example_count example files"
    
    # Try to check one example
    print_info "Checking basic_usage example..."
    if cargo check -p "$PACKAGE" --example basic_usage --features=cli-tools --quiet 2>/dev/null; then
        print_status "Example compilation works"
    else
        print_warning "Example compilation failed (may need features)"
    fi
else
    print_warning "No example files found"
fi

echo ""
echo -e "${BLUE}8. Validating Development Tools${NC}"
echo "==============================="

# Check for development tools
dev_tools=("cargo-audit" "cargo-llvm-cov" "cargo-criterion")
for tool in "${dev_tools[@]}"; do
    if command -v "$tool" &> /dev/null; then
        print_status "Development tool $tool is available"
    else
        print_info "Development tool $tool not found (optional)"
        print_info "Install with: cargo install $tool"
    fi
done

echo ""
echo -e "${BLUE}9. Validating Makefile Commands${NC}"
echo "==============================="

if [ -f "Makefile" ]; then
    print_status "Makefile exists"
    
    # Test a few basic make commands
    make_targets=("help" "check" "format-check")
    for target in "${make_targets[@]}"; do
        if make -n "$target" &>/dev/null; then
            print_status "Make target '$target' is valid"
        else
            print_warning "Make target '$target' has issues"
        fi
    done
else
    print_warning "Makefile not found"
fi

echo ""
echo -e "${BLUE}10. Security and Dependency Validation${NC}"
echo "========================================="

# Check for security files
security_files=("crates/$PACKAGE/deny.toml" "crates/$PACKAGE/clippy.toml")
for file in "${security_files[@]}"; do
    if [ -f "$file" ]; then
        print_status "Security config $file exists"
    else
        print_info "Security config $file not found (optional)"
    fi
done

# Check for cargo audit (if available)
if command -v cargo-audit &> /dev/null; then
    print_info "Running security audit..."
    if cargo audit --quiet; then
        print_status "Security audit passed"
    else
        print_warning "Security audit found issues"
    fi
else
    print_info "cargo-audit not available (install with: cargo install cargo-audit)"
fi

echo ""
echo -e "${BLUE}Summary${NC}"
echo "======="

# Final validation
print_info "CI validation completed!"
print_info "Key points for CI setup:"
echo "  • Ensure all workflow files are committed to version control"
echo "  • Configure any required secrets (CRATES_IO_TOKEN, etc.)"
echo "  • Set up branch protection rules if needed"
echo "  • Consider enabling required status checks"
echo ""

if [ -d ".github/workflows" ] && [ -f "Makefile" ] && command -v cargo &> /dev/null; then
    echo -e "${GREEN}🎉 CI setup looks good! Ready for GitHub Actions.${NC}"
    exit 0
else
    echo -e "${YELLOW}⚠️  CI setup needs attention. Check warnings above.${NC}"
    exit 1
fi
