@echo off
echo Installing Chaos World Backend Services...

REM Check if running as administrator
net session >nul 2>&1
if %errorLevel% neq 0 (
    echo This script must be run as Administrator!
    echo Right-click and select "Run as administrator"
    pause
    exit /b 1
)

REM Set paths
set NSSM_PATH=C:\ProgramData\chocolatey\bin\nssm.exe
set SERVICE_DIR=C:\ChaosWorld\services
set LOG_DIR=C:\ChaosWorld\logs

REM Check if NSSM exists
if not exist "%NSSM_PATH%" (
    echo NSSM not found at %NSSM_PATH%
    echo Please run install_nssm.bat first
    pause
    exit /b 1
)

REM Check if services exist
if not exist "%SERVICE_DIR%\api-gateway.exe" (
    echo API Gateway not found at %SERVICE_DIR%\api-gateway.exe
    echo Please run setup_windows_services.bat first
    pause
    exit /b 1
)

REM Install API Gateway Service
echo Installing API Gateway Service...
"%NSSM_PATH%" install "ChaosWorld-API-Gateway" "%SERVICE_DIR%\api-gateway.exe"
"%NSSM_PATH%" set "ChaosWorld-API-Gateway" AppDirectory "%SERVICE_DIR%"
"%NSSM_PATH%" set "ChaosWorld-API-Gateway" AppStdout "%LOG_DIR%\api-gateway.log"
"%NSSM_PATH%" set "ChaosWorld-API-Gateway" AppStderr "%LOG_DIR%\api-gateway-error.log"
"%NSSM_PATH%" set "ChaosWorld-API-Gateway" AppRotateFiles 1
"%NSSM_PATH%" set "ChaosWorld-API-Gateway" AppRotateOnline 1
"%NSSM_PATH%" set "ChaosWorld-API-Gateway" AppRotateBytes 1048576
"%NSSM_PATH%" set "ChaosWorld-API-Gateway" Start SERVICE_AUTO_START
"%NSSM_PATH%" set "ChaosWorld-API-Gateway" DisplayName "Chaos World API Gateway"
"%NSSM_PATH%" set "ChaosWorld-API-Gateway" Description "API Gateway for Chaos World Game Backend"

REM Install Chaos Backend Service
echo Installing Chaos Backend Service...
"%NSSM_PATH%" install "ChaosWorld-Backend" "%SERVICE_DIR%\chaos-backend.exe"
"%NSSM_PATH%" set "ChaosWorld-Backend" AppDirectory "%SERVICE_DIR%"
"%NSSM_PATH%" set "ChaosWorld-Backend" AppStdout "%LOG_DIR%\chaos-backend.log"
"%NSSM_PATH%" set "ChaosWorld-Backend" AppStderr "%LOG_DIR%\chaos-backend-error.log"
"%NSSM_PATH%" set "ChaosWorld-Backend" AppRotateFiles 1
"%NSSM_PATH%" set "ChaosWorld-Backend" AppRotateOnline 1
"%NSSM_PATH%" set "ChaosWorld-Backend" AppRotateBytes 1048576
"%NSSM_PATH%" set "ChaosWorld-Backend" Start SERVICE_AUTO_START
"%NSSM_PATH%" set "ChaosWorld-Backend" DisplayName "Chaos World Backend"
"%NSSM_PATH%" set "ChaosWorld-Backend" Description "Main Game Backend for Chaos World"

echo.
echo Services installed successfully!
echo.
echo Services created:
echo - ChaosWorld-API-Gateway
echo - ChaosWorld-Backend
echo.
echo Logs will be saved to: %LOG_DIR%
echo.
echo Next steps:
echo 1. Run start_services.bat to start the services
echo 2. Check Windows Services (services.msc) to see the services
echo 3. Run stop_services.bat to stop the services
echo 4. Run uninstall_services.bat to remove the services

pause
