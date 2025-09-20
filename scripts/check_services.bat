@echo off
echo Checking Chaos World Backend Services Status...

echo.
echo ========================================
echo Service Status
echo ========================================

echo.
echo API Gateway Service:
sc query "ChaosWorld-API-Gateway" 2>nul | find "STATE" || echo Service not installed

echo.
echo Chaos Backend Service:
sc query "ChaosWorld-Backend" 2>nul | find "STATE" || echo Service not installed

echo.
echo ========================================
echo Port Status
echo ========================================

echo.
echo Checking if ports are in use:
netstat -an | findstr ":8080" >nul
if %errorLevel%==0 (
    echo ✅ Port 8080 (API Gateway) is in use
) else (
    echo ❌ Port 8080 (API Gateway) is not in use
)

netstat -an | findstr ":8081" >nul
if %errorLevel%==0 (
    echo ✅ Port 8081 (Chaos Backend) is in use
) else (
    echo ❌ Port 8081 (Chaos Backend) is not in use
)

echo.
echo ========================================
echo Log Files
echo ========================================

echo.
echo Recent log entries:
if exist "C:\ChaosWorld\logs\api-gateway.log" (
    echo.
    echo API Gateway Log (last 5 lines):
    powershell -Command "Get-Content 'C:\ChaosWorld\logs\api-gateway.log' | Select-Object -Last 5"
) else (
    echo ❌ API Gateway log not found
)

if exist "C:\ChaosWorld\logs\chaos-backend.log" (
    echo.
    echo Chaos Backend Log (last 5 lines):
    powershell -Command "Get-Content 'C:\ChaosWorld\logs\chaos-backend.log' | Select-Object -Last 5"
) else (
    echo ❌ Chaos Backend log not found
)

echo.
echo ========================================
echo Quick Tests
echo ========================================

echo.
echo Testing API Gateway...
curl -s http://localhost:8080/health >nul 2>&1
if %errorLevel%==0 (
    echo ✅ API Gateway is responding
) else (
    echo ❌ API Gateway is not responding
)

echo.
echo Testing Chaos Backend...
curl -s http://localhost:8081/health >nul 2>&1
if %errorLevel%==0 (
    echo ✅ Chaos Backend is responding
) else (
    echo ❌ Chaos Backend is not responding
)

echo.
echo ========================================
echo Management Commands
echo ========================================
echo.
echo To start services: start_services.bat
echo To stop services: stop_services.bat
echo To restart services: stop_services.bat && start_services.bat
echo To uninstall: uninstall_services.bat

pause
