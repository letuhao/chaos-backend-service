@echo off
echo ========================================
echo Chaos World Backend Services Setup
echo ========================================
echo.
echo This script will set up Windows services for:
echo - API Gateway (Port 8080)
echo - Chaos Backend (Port 8081)
echo.
echo Requirements:
echo - Administrator privileges
echo - Internet connection (to download NSSM)
echo - Rust toolchain installed
echo.

REM Check if running as administrator
net session >nul 2>&1
if %errorLevel% neq 0 (
    echo ❌ This script must be run as Administrator!
    echo Please right-click and select "Run as administrator"
    pause
    exit /b 1
)

echo ✅ Running as Administrator

echo.
echo Step 1: Installing NSSM...
call install_nssm.bat
if %errorLevel% neq 0 (
    echo ❌ Failed to install NSSM
    pause
    exit /b 1
)

echo.
echo Step 2: Building and preparing services...
call setup_windows_services.bat
if %errorLevel% neq 0 (
    echo ❌ Failed to build services
    pause
    exit /b 1
)

echo.
echo Step 3: Installing Windows services...
call install_services.bat
if %errorLevel% neq 0 (
    echo ❌ Failed to install services
    pause
    exit /b 1
)

echo.
echo Step 4: Starting services...
call start_services.bat
if %errorLevel% neq 0 (
    echo ❌ Failed to start services
    pause
    exit /b 1
)

echo.
echo ========================================
echo Setup Complete!
echo ========================================
echo.
echo Your Chaos World Backend services are now running as Windows services:
echo.
echo ✅ API Gateway: http://localhost:8080
echo ✅ Chaos Backend: http://localhost:8081
echo.
echo Service Management:
echo - Check status: check_services.bat
echo - Stop services: stop_services.bat
echo - Start services: start_services.bat
echo - Uninstall: uninstall_services.bat
echo.
echo Logs location: C:\ChaosWorld\logs\
echo Service files: C:\ChaosWorld\services\
echo.
echo The services will automatically start when Windows boots.
echo They will restart automatically if they crash.
echo.

pause
