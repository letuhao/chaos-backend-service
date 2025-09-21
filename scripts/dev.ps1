# PowerShell script for development with warnings suppressed

# Set environment variables to suppress warnings
$env:RUSTFLAGS = "-A unused_variables -A unused_imports -A dead_code -A unused_mut -A unused_assignments"

Write-Host "🔧 Development Mode - Warnings Suppressed" -ForegroundColor Green
Write-Host "RUSTFLAGS: $env:RUSTFLAGS" -ForegroundColor Yellow
Write-Host ""

# Check if command is provided
if ($args.Count -eq 0) {
    Write-Host "Usage: .\scripts\dev.ps1 <command>" -ForegroundColor Cyan
    Write-Host ""
    Write-Host "Available commands:" -ForegroundColor Cyan
    Write-Host "  check     - cargo check"
    Write-Host "  build     - cargo build"
    Write-Host "  test      - cargo test"
    Write-Host "  examples  - cargo run --example <name>"
    Write-Host "  bench     - cargo bench"
    Write-Host "  clean     - cargo clean"
    Write-Host ""
    exit 0
}

$command = $args[0]

switch ($command) {
    "check" {
        Write-Host "🔍 Running cargo check..." -ForegroundColor Blue
        cargo check
    }
    "build" {
        Write-Host "🔨 Running cargo build..." -ForegroundColor Blue
        cargo build
    }
    "test" {
        Write-Host "🧪 Running cargo test..." -ForegroundColor Blue
        cargo test
    }
    "examples" {
        Write-Host "📚 Running examples..." -ForegroundColor Blue
        if ($args.Count -gt 1) {
            $example = $args[1]
            cargo run --example $example
        } else {
            Write-Host "Available examples:" -ForegroundColor Cyan
            Write-Host "  new_architecture_demo"
            Write-Host "  add_new_function_demo"
            Write-Host "  element_conditions"
            Write-Host ""
            Write-Host "Usage: .\scripts\dev.ps1 examples <example_name>"
        }
    }
    "bench" {
        Write-Host "⚡ Running cargo bench..." -ForegroundColor Blue
        cargo bench
    }
    "clean" {
        Write-Host "🧹 Running cargo clean..." -ForegroundColor Blue
        cargo clean
    }
    default {
        Write-Host "❌ Unknown command: $command" -ForegroundColor Red
        Write-Host "Run '.\scripts\dev.ps1' to see available commands"
    }
}
