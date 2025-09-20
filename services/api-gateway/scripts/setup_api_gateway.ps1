# Setup API Gateway Script
# This script sets up the API Gateway service

Write-Host "========================================" -ForegroundColor Cyan
Write-Host "  API Gateway Setup Script" -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan

# Check if Rust is installed
Write-Host "`n🔍 Checking Rust installation..." -ForegroundColor Yellow
try {
    $rustVersion = rustc --version
    Write-Host "✅ Rust is installed: $rustVersion" -ForegroundColor Green
} catch {
    Write-Host "❌ Rust is not installed. Please install Rust first." -ForegroundColor Red
    Write-Host "Visit: https://rustup.rs/" -ForegroundColor Yellow
    exit 1
}

# Check if Cargo is installed
Write-Host "`n🔍 Checking Cargo installation..." -ForegroundColor Yellow
try {
    $cargoVersion = cargo --version
    Write-Host "✅ Cargo is installed: $cargoVersion" -ForegroundColor Green
} catch {
    Write-Host "❌ Cargo is not installed. Please install Cargo first." -ForegroundColor Red
    exit 1
}

# Check if Redis is running (optional)
Write-Host "`n🔍 Checking Redis connection..." -ForegroundColor Yellow
try {
    $redisTest = redis-cli ping 2>$null
    if ($redisTest -eq "PONG") {
        Write-Host "✅ Redis is running" -ForegroundColor Green
    } else {
        Write-Host "⚠️  Redis is not running. Some features may not work." -ForegroundColor Yellow
    }
} catch {
    Write-Host "⚠️  Redis is not available. Some features may not work." -ForegroundColor Yellow
}

# Check if MongoDB is running (optional)
Write-Host "`n🔍 Checking MongoDB connection..." -ForegroundColor Yellow
try {
    $mongoTest = mongosh --eval "db.runCommand('ping')" --quiet 2>$null
    if ($mongoTest -like "*ok*") {
        Write-Host "✅ MongoDB is running" -ForegroundColor Green
    } else {
        Write-Host "⚠️  MongoDB is not running. Some features may not work." -ForegroundColor Yellow
    }
} catch {
    Write-Host "⚠️  MongoDB is not available. Some features may not work." -ForegroundColor Yellow
}

# Create necessary directories
Write-Host "`n📁 Creating necessary directories..." -ForegroundColor Yellow
$directories = @("logs", "data", "cache", "temp")
foreach ($dir in $directories) {
    if (!(Test-Path $dir)) {
        New-Item -ItemType Directory -Path $dir -Force | Out-Null
        Write-Host "✅ Created directory: $dir" -ForegroundColor Green
    } else {
        Write-Host "✅ Directory already exists: $dir" -ForegroundColor Green
    }
}

# Copy environment file if it doesn't exist
Write-Host "`n📄 Setting up environment file..." -ForegroundColor Yellow
if (!(Test-Path ".env")) {
    if (Test-Path "env.example") {
        Copy-Item "env.example" ".env"
        Write-Host "✅ Created .env file from env.example" -ForegroundColor Green
    } else {
        Write-Host "⚠️  env.example not found. Please create .env file manually." -ForegroundColor Yellow
    }
} else {
    Write-Host "✅ .env file already exists" -ForegroundColor Green
}

# Build the API Gateway
Write-Host "`n🔨 Building API Gateway..." -ForegroundColor Yellow
cargo build

if ($LASTEXITCODE -ne 0) {
    Write-Host "❌ Build failed!" -ForegroundColor Red
    exit 1
}

Write-Host "✅ Build successful!" -ForegroundColor Green

# Run tests
Write-Host "`n🧪 Running tests..." -ForegroundColor Yellow
cargo test

if ($LASTEXITCODE -ne 0) {
    Write-Host "⚠️  Some tests failed. Check the output above." -ForegroundColor Yellow
} else {
    Write-Host "✅ All tests passed!" -ForegroundColor Green
}

# Check configuration files
Write-Host "`n📋 Checking configuration files..." -ForegroundColor Yellow
$configFiles = @("configs/api-gateway.yaml", "configs/api-gateway-dev.yaml", "configs/api-gateway-prod.yaml")
foreach ($configFile in $configFiles) {
    if (Test-Path $configFile) {
        Write-Host "✅ Configuration file exists: $configFile" -ForegroundColor Green
    } else {
        Write-Host "❌ Configuration file missing: $configFile" -ForegroundColor Red
    }
}

# Display next steps
Write-Host "`n========================================" -ForegroundColor Cyan
Write-Host "  Setup Complete!" -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan
Write-Host "`nNext steps:" -ForegroundColor Yellow
Write-Host "1. Configure .env file with your settings" -ForegroundColor White
Write-Host "2. Start Redis (optional): redis-server" -ForegroundColor White
Write-Host "3. Start MongoDB (optional): mongod" -ForegroundColor White
Write-Host "4. Run API Gateway: .\scripts\run_api_gateway.ps1" -ForegroundColor White
Write-Host "5. Test API Gateway: .\scripts\test_api_gateway.ps1" -ForegroundColor White
Write-Host "`nFor more information, see the README.md file." -ForegroundColor Cyan
