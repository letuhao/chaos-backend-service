# Run API Gateway Script
# This script runs the API Gateway service

Write-Host "========================================" -ForegroundColor Cyan
Write-Host "  API Gateway Run Script" -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan

# Set environment variables
$env:ENV = "development"
$env:RUST_LOG = "debug"
$env:RUST_BACKTRACE = "1"

# Check if API Gateway is already running
$existingProcess = Get-Process -Name "api-gateway" -ErrorAction SilentlyContinue
if ($existingProcess) {
    Write-Host "⚠️  API Gateway is already running. Stopping existing process..." -ForegroundColor Yellow
    Stop-Process -Id $existingProcess.Id -Force
    Start-Sleep -Seconds 2
}

# Build the API Gateway
Write-Host "`n🔨 Building API Gateway..." -ForegroundColor Yellow
cargo build --release

if ($LASTEXITCODE -ne 0) {
    Write-Host "❌ Build failed!" -ForegroundColor Red
    exit 1
}

Write-Host "✅ Build successful!" -ForegroundColor Green

# Start the API Gateway
Write-Host "`n🚀 Starting API Gateway..." -ForegroundColor Yellow
Write-Host "Environment: $env:ENV" -ForegroundColor Cyan
Write-Host "Log Level: $env:RUST_LOG" -ForegroundColor Cyan
Write-Host "Debug: $env:RUST_BACKTRACE" -ForegroundColor Cyan
Write-Host "`nPress Ctrl+C to stop the server" -ForegroundColor Yellow
Write-Host "========================================" -ForegroundColor Cyan

# Run the API Gateway
try {
    & "target\release\api-gateway.exe" --env development --debug --hot-reload
} catch {
    Write-Host "`n❌ API Gateway failed to start: $($_.Exception.Message)" -ForegroundColor Red
    exit 1
}

Write-Host "`n✅ API Gateway stopped" -ForegroundColor Green
