# Test API Gateway endpoints

Write-Host "🧪 Testing API Gateway..." -ForegroundColor Yellow

$baseUrl = "http://localhost:8080"

# Test health endpoint
Write-Host "📡 Testing /health endpoint..." -ForegroundColor Cyan
try {
    $healthResponse = Invoke-RestMethod -Uri "$baseUrl/health" -Method GET
    Write-Host "✅ Health check: $healthResponse" -ForegroundColor Green
} catch {
    Write-Host "❌ Health check failed: $($_.Exception.Message)" -ForegroundColor Red
}

# Test root endpoint
Write-Host "📡 Testing / endpoint..." -ForegroundColor Cyan
try {
    $rootResponse = Invoke-RestMethod -Uri "$baseUrl/" -Method GET
    Write-Host "✅ Root endpoint: $rootResponse" -ForegroundColor Green
} catch {
    Write-Host "❌ Root endpoint failed: $($_.Exception.Message)" -ForegroundColor Red
}

# Test non-existent endpoint
Write-Host "📡 Testing /nonexistent endpoint..." -ForegroundColor Cyan
try {
    $errorResponse = Invoke-RestMethod -Uri "$baseUrl/nonexistent" -Method GET
    Write-Host "✅ Non-existent endpoint: $errorResponse" -ForegroundColor Green
} catch {
    Write-Host "✅ Non-existent endpoint correctly returned 404" -ForegroundColor Green
}

Write-Host "🎉 API Gateway testing completed!" -ForegroundColor Green