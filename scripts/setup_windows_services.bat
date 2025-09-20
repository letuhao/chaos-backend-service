@echo off
echo Setting up Chaos World Backend Services for Windows...

REM Create services directory
if not exist "C:\ChaosWorld\services" mkdir "C:\ChaosWorld\services"
if not exist "C:\ChaosWorld\logs" mkdir "C:\ChaosWorld\logs"

REM Download NSSM if not exists
if not exist "C:\ChaosWorld\nssm.exe" (
    echo Downloading NSSM...
    powershell -Command "Invoke-WebRequest -Uri 'https://nssm.cc/release/nssm-2.24.zip' -OutFile 'C:\ChaosWorld\nssm.zip'"
    powershell -Command "Expand-Archive -Path 'C:\ChaosWorld\nssm.zip' -DestinationPath 'C:\ChaosWorld' -Force"
    copy "C:\ChaosWorld\nssm-2.24\win64\nssm.exe" "C:\ChaosWorld\"
    rmdir /s /q "C:\ChaosWorld\nssm-2.24"
    del "C:\ChaosWorld\nssm.zip"
)

REM Build the services
echo Building services...
cargo build --release -p api-gateway
cargo build --release -p chaos-backend

REM Copy executables
copy "target\release\api-gateway.exe" "C:\ChaosWorld\services\"
copy "target\release\chaos-backend.exe" "C:\ChaosWorld\services\"

echo Services built and copied to C:\ChaosWorld\services\
echo.
echo Next steps:
echo 1. Run install_services.bat as Administrator
echo 2. Run start_services.bat to start the services
echo 3. Run stop_services.bat to stop the services
echo 4. Run uninstall_services.bat to remove the services

pause
