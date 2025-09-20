@echo off
echo Starting Chaos World Backend Services...

REM Check if running as administrator
net session >nul 2>&1
if %errorLevel% neq 0 (
    echo This script must be run as Administrator!
    echo Right-click and select "Run as administrator"
    pause
    exit /b 1
)

echo Starting API Gateway Service...
net start "ChaosWorld-API-Gateway"
if %errorLevel% equ 0 (
    echo ✅ API Gateway started successfully
) else (
    echo ❌ Failed to start API Gateway
)

echo.
echo Starting Chaos Backend Service...
net start "ChaosWorld-Backend"
if %errorLevel% equ 0 (
    echo ✅ Chaos Backend started successfully
) else (
    echo ❌ Failed to start Chaos Backend
)

echo.
echo Checking service status...
sc query "ChaosWorld-API-Gateway" | find "RUNNING" >nul
if %errorLevel% equ 0 (
    echo ✅ API Gateway is running
) else (
    echo ❌ API Gateway is not running
)

sc query "ChaosWorld-Backend" | find "RUNNING" >nul
if %errorLevel% equ 0 (
    echo ✅ Chaos Backend is running
) else (
    echo ❌ Chaos Backend is not running
)

echo.
echo Services should now be accessible at:
echo - API Gateway: http://localhost:8080
echo - Chaos Backend: http://localhost:8081
echo.
echo To check logs: C:\ChaosWorld\logs\
echo To stop services: Run stop_services.bat

pause
