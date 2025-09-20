@echo off
echo Stopping Chaos World Backend Services...

REM Check if running as administrator
net session >nul 2>&1
if %errorLevel% neq 0 (
    echo This script must be run as Administrator!
    echo Right-click and select "Run as administrator"
    pause
    exit /b 1
)

echo Stopping API Gateway Service...
net stop "ChaosWorld-API-Gateway"
if %errorLevel% equ 0 (
    echo ✅ API Gateway stopped successfully
) else (
    echo ❌ Failed to stop API Gateway (may not be running)
)

echo.
echo Stopping Chaos Backend Service...
net stop "ChaosWorld-Backend"
if %errorLevel% equ 0 (
    echo ✅ Chaos Backend stopped successfully
) else (
    echo ❌ Failed to stop Chaos Backend (may not be running)
)

echo.
echo Checking service status...
sc query "ChaosWorld-API-Gateway" | find "STOPPED" >nul
if %errorLevel% equ 0 (
    echo ✅ API Gateway is stopped
) else (
    echo ⚠️  API Gateway status unknown
)

sc query "ChaosWorld-Backend" | find "STOPPED" >nul
if %errorLevel% equ 0 (
    echo ✅ Chaos Backend is stopped
) else (
    echo ⚠️  Chaos Backend status unknown
)

echo.
echo All services have been stopped.
echo To start them again: Run start_services.bat

pause
