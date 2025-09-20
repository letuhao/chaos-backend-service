Write-Host "Checking Chaos World Backend Services Status..." -ForegroundColor Green

Write-Host ""
Write-Host "========================================" -ForegroundColor Yellow
Write-Host "Service Status" -ForegroundColor Yellow
Write-Host "========================================" -ForegroundColor Yellow

Write-Host ""
Write-Host "API Gateway Service:" -ForegroundColor Cyan
$apiGatewayStatus = sc query "ChaosWorld-API-Gateway" 2>$null | Select-String "STATE"
if ($apiGatewayStatus) {
    Write-Host $apiGatewayStatus -ForegroundColor White
} else {
    Write-Host "Service not installed" -ForegroundColor Red
}

Write-Host ""
Write-Host "Chaos Backend Service:" -ForegroundColor Cyan
$backendStatus = sc query "ChaosWorld-Backend" 2>$null | Select-String "STATE"
if ($backendStatus) {
    Write-Host $backendStatus -ForegroundColor White
} else {
    Write-Host "Service not installed" -ForegroundColor Red
}

Write-Host ""
Write-Host "========================================" -ForegroundColor Yellow
Write-Host "Port Status" -ForegroundColor Yellow
Write-Host "========================================" -ForegroundColor Yellow

Write-Host ""
Write-Host "Checking if ports are in use:" -ForegroundColor Cyan

$port8080 = netstat -an | Select-String ":8080"
if ($port8080) {
    Write-Host "✅ Port 8080 (API Gateway) is in use" -ForegroundColor Green
} else {
    Write-Host "❌ Port 8080 (API Gateway) is not in use" -ForegroundColor Red
}

$port8081 = netstat -an | Select-String ":8081"
if ($port8081) {
    Write-Host "✅ Port 8081 (Chaos Backend) is in use" -ForegroundColor Green
} else {
    Write-Host "❌ Port 8081 (Chaos Backend) is not in use" -ForegroundColor Red
}

Write-Host ""
Write-Host "========================================" -ForegroundColor Yellow
Write-Host "Quick Tests" -ForegroundColor Yellow
Write-Host "========================================" -ForegroundColor Yellow

Write-Host ""
Write-Host "Testing API Gateway..." -ForegroundColor Cyan
try {
    $response = Invoke-WebRequest -Uri "http://localhost:8080/health" -TimeoutSec 5 -UseBasicParsing
    if ($response.StatusCode -eq 200) {
        Write-Host "✅ API Gateway is responding" -ForegroundColor Green
    } else {
        Write-Host "❌ API Gateway is not responding (Status: $($response.StatusCode))" -ForegroundColor Red
    }
} catch {
    Write-Host "❌ API Gateway is not responding" -ForegroundColor Red
}

Write-Host ""
Write-Host "Testing Chaos Backend..." -ForegroundColor Cyan
try {
    $response = Invoke-WebRequest -Uri "http://localhost:8081/health" -TimeoutSec 5 -UseBasicParsing
    if ($response.StatusCode -eq 200) {
        Write-Host "✅ Chaos Backend is responding" -ForegroundColor Green
    } else {
        Write-Host "❌ Chaos Backend is not responding (Status: $($response.StatusCode))" -ForegroundColor Red
    }
} catch {
    Write-Host "❌ Chaos Backend is not responding" -ForegroundColor Red
}

Write-Host ""
Write-Host "========================================" -ForegroundColor Yellow
Write-Host "Management Commands" -ForegroundColor Yellow
Write-Host "========================================" -ForegroundColor Yellow
Write-Host ""
Write-Host "To start services: .\scripts\start_services.bat (Run as Administrator)" -ForegroundColor White
Write-Host "To stop services: .\scripts\stop_services.bat (Run as Administrator)" -ForegroundColor White
Write-Host "To restart services: .\scripts\stop_services.bat && .\scripts\start_services.bat" -ForegroundColor White
Write-Host "To uninstall: .\scripts\uninstall_services.bat (Run as Administrator)" -ForegroundColor White

Read-Host "Press Enter to continue"
