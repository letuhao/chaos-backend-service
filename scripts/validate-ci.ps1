# CI Validation Script (PowerShell)
# This script validates that the CI setup is working correctly

param(
    [switch]$Detailed = $false
)

# Configuration
$PACKAGE = "actor-core"
$PROJECT_ROOT = Split-Path -Parent $PSScriptRoot

Write-Host "🔍 CI Validation Script" -ForegroundColor Blue
Write-Host "=======================" -ForegroundColor Blue
Write-Host "Project root: $PROJECT_ROOT"
Write-Host ""

# Function to print status
function Write-Status {
    param([string]$Message, [bool]$Success)
    
    if ($Success) {
        Write-Host "✅ $Message" -ForegroundColor Green
    } else {
        Write-Host "❌ $Message" -ForegroundColor Red
        if (-not $Detailed) {
            exit 1
        }
    }
}

function Write-Warning {
    param([string]$Message)
    Write-Host "⚠️  $Message" -ForegroundColor Yellow
}

function Write-Info {
    param([string]$Message)
    Write-Host "ℹ️  $Message" -ForegroundColor Blue
}

# Change to project directory
Set-Location $PROJECT_ROOT

Write-Host "1. Validating CI Workflow Files" -ForegroundColor Blue
Write-Host "================================="

# Check if GitHub Actions workflows exist
if (Test-Path ".github/workflows") {
    Write-Status "GitHub Actions directory exists" $true
    
    # Check specific workflow files
    $workflows = @("ci.yml", "nightly.yml", "release.yml", "config.yml")
    foreach ($workflow in $workflows) {
        $path = ".github/workflows/$workflow"
        if (Test-Path $path) {
            Write-Status "Workflow file $workflow exists" $true
        } else {
            Write-Warning "Workflow file $workflow is missing"
        }
    }
} else {
    Write-Warning "GitHub Actions directory not found"
}

Write-Host ""
Write-Host "2. Validating Project Structure" -ForegroundColor Blue
Write-Host "==============================="

# Check for essential files
$essentialFiles = @("Cargo.toml", "crates/$PACKAGE/Cargo.toml", "Makefile")
foreach ($file in $essentialFiles) {
    if (Test-Path $file) {
        Write-Status "Essential file $file exists" $true
    } else {
        Write-Warning "Essential file $file is missing"
    }
}

# Check for configuration files
$configDir = "crates/$PACKAGE/configs"
if (Test-Path $configDir) {
    Write-Status "Configuration directory exists" $true
    $configCount = (Get-ChildItem "$configDir/*.yaml" -ErrorAction SilentlyContinue).Count
    Write-Info "Found $configCount YAML configuration files"
} else {
    Write-Warning "Configuration directory not found"
}

Write-Host ""
Write-Host "3. Validating Rust Environment" -ForegroundColor Blue
Write-Host "=============================="

# Check Rust installation
try {
    $rustVersion = & rustc --version 2>$null
    Write-Status "Rust compiler found: $rustVersion" $true
} catch {
    Write-Warning "Rust compiler not found"
}

try {
    $cargoVersion = & cargo --version 2>$null
    Write-Status "Cargo found: $cargoVersion" $true
} catch {
    Write-Warning "Cargo not found"
}

# Check for required components
$components = @("rustfmt", "clippy")
foreach ($component in $components) {
    try {
        $installed = & rustup component list --installed 2>$null | Select-String $component
        if ($installed) {
            Write-Status "Component $component is installed" $true
        } else {
            Write-Warning "Component $component is not installed"
            Write-Info "Run: rustup component add $component"
        }
    } catch {
        Write-Warning "Unable to check component $component"
    }
}

Write-Host ""
Write-Host "4. Validating Feature Matrix" -ForegroundColor Blue
Write-Host "============================"

# Test basic compilation with different feature sets
$featureSets = @("", "moka-cache", "memory-mapped", "redis-cache", "heavy-deps")

foreach ($features in $featureSets) {
    $featureName = if ($features) { $features } else { "default" }
    Write-Info "Testing feature set: $featureName"
    
    try {
        if ($features) {
            $result = & cargo check -p $PACKAGE --features="$features" --quiet 2>$null
        } else {
            $result = & cargo check -p $PACKAGE --quiet 2>$null
        }
        
        if ($LASTEXITCODE -eq 0) {
            Write-Status "Features '$featureName' compile" $true
        } else {
            Write-Warning "Features '$featureName' compilation failed"
        }
    } catch {
        Write-Warning "Error testing features '$featureName'"
    }
}

Write-Host ""
Write-Host "5. Validating Tests" -ForegroundColor Blue
Write-Host "=================="

# Run basic tests
Write-Info "Running unit tests..."
try {
    $result = & cargo test -p $PACKAGE --lib --quiet 2>$null
    if ($LASTEXITCODE -eq 0) {
        Write-Status "Unit tests pass" $true
    } else {
        Write-Warning "Unit tests failed"
    }
} catch {
    Write-Warning "Error running unit tests"
}

# Check for test files
$testDir = "crates/$PACKAGE/tests"
if (Test-Path $testDir) {
    $testCount = (Get-ChildItem "$testDir/*.rs" -ErrorAction SilentlyContinue).Count
    if ($testCount -gt 0) {
        Write-Status "Found $testCount integration test files" $true
    } else {
        Write-Warning "No integration test files found"
    }
} else {
    Write-Warning "Test directory not found"
}

Write-Host ""
Write-Host "6. Validating Documentation" -ForegroundColor Blue
Write-Host "==========================="

# Check documentation compilation
Write-Info "Building documentation..."
try {
    $result = & cargo doc -p $PACKAGE --no-deps --quiet 2>$null
    if ($LASTEXITCODE -eq 0) {
        Write-Status "Documentation builds successfully" $true
    } else {
        Write-Warning "Documentation build failed"
    }
} catch {
    Write-Warning "Error building documentation"
}

# Check for README
$readmePath = "crates/$PACKAGE/README.md"
if (Test-Path $readmePath) {
    Write-Status "Package README exists" $true
} else {
    Write-Warning "Package README not found"
}

Write-Host ""
Write-Host "7. Validating Examples" -ForegroundColor Blue
Write-Host "====================="

# Check for examples
$exampleDir = "crates/$PACKAGE/examples"
if (Test-Path $exampleDir) {
    $exampleCount = (Get-ChildItem "$exampleDir/*.rs" -ErrorAction SilentlyContinue).Count
    if ($exampleCount -gt 0) {
        Write-Status "Found $exampleCount example files" $true
        
        # Try to check one example
        Write-Info "Checking basic_usage example..."
        try {
            $result = & cargo check -p $PACKAGE --example basic_usage --features=cli-tools --quiet 2>$null
            if ($LASTEXITCODE -eq 0) {
                Write-Status "Example compilation works" $true
            } else {
                Write-Warning "Example compilation failed (may need features)"
            }
        } catch {
            Write-Warning "Error checking example"
        }
    } else {
        Write-Warning "No example files found"
    }
} else {
    Write-Warning "Examples directory not found"
}

Write-Host ""
Write-Host "8. Validating Development Tools" -ForegroundColor Blue
Write-Host "==============================="

# Check for development tools
$devTools = @("cargo-audit", "cargo-llvm-cov", "cargo-criterion")
foreach ($tool in $devTools) {
    try {
        $null = & $tool --version 2>$null
        if ($LASTEXITCODE -eq 0) {
            Write-Status "Development tool $tool is available" $true
        } else {
            Write-Info "Development tool $tool not found (optional)"
            Write-Info "Install with: cargo install $tool"
        }
    } catch {
        Write-Info "Development tool $tool not found (optional)"
        Write-Info "Install with: cargo install $tool"
    }
}

Write-Host ""
Write-Host "9. Validating Makefile Commands" -ForegroundColor Blue
Write-Host "==============================="

if (Test-Path "Makefile") {
    Write-Status "Makefile exists" $true
    Write-Info "Note: On Windows, you may need GNU Make or use PowerShell equivalents"
} else {
    Write-Warning "Makefile not found"
}

Write-Host ""
Write-Host "10. Security and Dependency Validation" -ForegroundColor Blue
Write-Host "========================================="

# Check for security files
$securityFiles = @("crates/$PACKAGE/deny.toml", "crates/$PACKAGE/clippy.toml")
foreach ($file in $securityFiles) {
    if (Test-Path $file) {
        Write-Status "Security config $file exists" $true
    } else {
        Write-Info "Security config $file not found (optional)"
    }
}

# Check for cargo audit (if available)
try {
    $null = & cargo-audit --version 2>$null
    if ($LASTEXITCODE -eq 0) {
        Write-Info "Running security audit..."
        try {
            $result = & cargo audit --quiet 2>$null
            if ($LASTEXITCODE -eq 0) {
                Write-Status "Security audit passed" $true
            } else {
                Write-Warning "Security audit found issues"
            }
        } catch {
            Write-Warning "Error running security audit"
        }
    } else {
        Write-Info "cargo-audit not available (install with: cargo install cargo-audit)"
    }
} catch {
    Write-Info "cargo-audit not available (install with: cargo install cargo-audit)"
}

Write-Host ""
Write-Host "Summary" -ForegroundColor Blue
Write-Host "======="

# Final validation
Write-Info "CI validation completed!"
Write-Info "Key points for CI setup:"
Write-Host "  • Ensure all workflow files are committed to version control"
Write-Host "  • Configure any required secrets (CRATES_IO_TOKEN, etc.)"
Write-Host "  • Set up branch protection rules if needed"
Write-Host "  • Consider enabling required status checks"
Write-Host ""

$hasWorkflows = Test-Path ".github/workflows"
$hasMakefile = Test-Path "Makefile"
$hasCargo = $null -ne (Get-Command cargo -ErrorAction SilentlyContinue)

if ($hasWorkflows -and $hasMakefile -and $hasCargo) {
    Write-Host "🎉 CI setup looks good! Ready for GitHub Actions." -ForegroundColor Green
    exit 0
} else {
    Write-Host "⚠️  CI setup needs attention. Check warnings above." -ForegroundColor Yellow
    if (-not $Detailed) {
        exit 1
    }
}
