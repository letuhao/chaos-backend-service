# CMS Service API Test Script
# This script tests the basic functionality of the CMS service

Write-Host "🧪 Testing CMS Service API..." -ForegroundColor Green

$baseUrl = "http://localhost:8080"
$adminUsername = "admin"
$adminPassword = "admin123"

# Test 1: Basic health check
Write-Host "`n1. Testing basic health check..." -ForegroundColor Yellow
try {
    $response = Invoke-RestMethod -Uri "$baseUrl/health" -Method GET
    Write-Host "✅ Health check: $response" -ForegroundColor Green
} catch {
    Write-Host "❌ Health check failed: $($_.Exception.Message)" -ForegroundColor Red
}

# Test 2: Root endpoint
Write-Host "`n2. Testing root endpoint..." -ForegroundColor Yellow
try {
    $response = Invoke-RestMethod -Uri "$baseUrl/" -Method GET
    Write-Host "✅ Root endpoint: $($response.data)" -ForegroundColor Green
} catch {
    Write-Host "❌ Root endpoint failed: $($_.Exception.Message)" -ForegroundColor Red
}

# Test 3: Admin login
Write-Host "`n3. Testing admin login..." -ForegroundColor Yellow
try {
    $loginData = @{
        username = $adminUsername
        password = $adminPassword
    } | ConvertTo-Json

    $response = Invoke-RestMethod -Uri "$baseUrl/api/v1/auth/login" -Method POST -Body $loginData -ContentType "application/json"
    $token = $response.data.token
    Write-Host "✅ Login successful! Token: $($token.Substring(0, 20))..." -ForegroundColor Green
} catch {
    Write-Host "❌ Login failed: $($_.Exception.Message)" -ForegroundColor Red
    exit 1
}

# Test 4: Get user info
Write-Host "`n4. Testing user info..." -ForegroundColor Yellow
try {
    $headers = @{
        "Authorization" = "Bearer $token"
    }
    $response = Invoke-RestMethod -Uri "$baseUrl/api/v1/auth/me" -Method GET -Headers $headers
    Write-Host "✅ User info: $($response.data.username) ($($response.data.role))" -ForegroundColor Green
} catch {
    Write-Host "❌ User info failed: $($_.Exception.Message)" -ForegroundColor Red
}

# Test 5: Access protected admin endpoint
Write-Host "`n5. Testing protected admin endpoint..." -ForegroundColor Yellow
try {
    $response = Invoke-RestMethod -Uri "$baseUrl/api/v1/admin" -Method GET -Headers $headers
    Write-Host "✅ Admin endpoint: $($response.message)" -ForegroundColor Green
} catch {
    Write-Host "❌ Admin endpoint failed: $($_.Exception.Message)" -ForegroundColor Red
}

# Test 6: Health monitoring
Write-Host "`n6. Testing health monitoring..." -ForegroundColor Yellow
try {
    $response = Invoke-RestMethod -Uri "$baseUrl/api/v1/health" -Method GET
    Write-Host "✅ Health monitoring: $($response.data.status)" -ForegroundColor Green
    Write-Host "   Database: $($response.data.services.database.status)" -ForegroundColor Cyan
    Write-Host "   Cache: $($response.data.services.cache.status)" -ForegroundColor Cyan
    Write-Host "   Storage: $($response.data.services.storage.status)" -ForegroundColor Cyan
} catch {
    Write-Host "❌ Health monitoring failed: $($_.Exception.Message)" -ForegroundColor Red
}

# Test 7: Metrics info
Write-Host "`n7. Testing metrics info..." -ForegroundColor Yellow
try {
    $response = Invoke-RestMethod -Uri "$baseUrl/api/v1/metrics/info" -Method GET
    Write-Host "✅ Metrics info:" -ForegroundColor Green
    Write-Host "   Requests: $($response.data.requests_total)" -ForegroundColor Cyan
    Write-Host "   Active connections: $($response.data.active_connections)" -ForegroundColor Cyan
    Write-Host "   Error rate: $($response.data.error_rate)" -ForegroundColor Cyan
} catch {
    Write-Host "❌ Metrics info failed: $($_.Exception.Message)" -ForegroundColor Red
}

Write-Host "`n🎉 API testing completed!" -ForegroundColor Green
