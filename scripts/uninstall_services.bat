@echo off
echo Uninstalling Chaos World Backend Services...

REM Check if running as administrator
net session >nul 2>&1
if %errorLevel% neq 0 (
    echo This script must be run as Administrator!
    echo Right-click and select "Run as administrator"
    pause
    exit /b 1
)

echo Stopping services first...
net stop "ChaosWorld-API-Gateway" >nul 2>&1
net stop "ChaosWorld-Backend" >nul 2>&1

echo Uninstalling API Gateway Service...
C:\ChaosWorld\bin\nssm.exe remove "ChaosWorld-API-Gateway" confirm
if %errorLevel% equ 0 (
    echo ✅ API Gateway service removed
) else (
    echo ❌ Failed to remove API Gateway service
)

echo.
echo Uninstalling Chaos Backend Service...
C:\ChaosWorld\bin\nssm.exe remove "ChaosWorld-Backend" confirm
if %errorLevel% equ 0 (
    echo ✅ Chaos Backend service removed
) else (
    echo ❌ Failed to remove Chaos Backend service
)

echo.
echo Services have been uninstalled.
echo.
echo Note: Service files and logs are still in C:\ChaosWorld\
echo To completely remove everything, delete the C:\ChaosWorld\ folder

pause
