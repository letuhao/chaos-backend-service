@echo off
echo Installing NSSM (Non-Sucking Service Manager)...

REM Create directory for NSSM
if not exist "C:\ChaosWorld" mkdir "C:\ChaosWorld"
if not exist "C:\ChaosWorld\bin" mkdir "C:\ChaosWorld\bin"

REM Download NSSM
echo Downloading NSSM...
powershell -Command "Invoke-WebRequest -Uri 'https://nssm.cc/release/nssm-2.24.zip' -OutFile 'C:\ChaosWorld\nssm.zip'"

REM Extract NSSM
echo Extracting NSSM...
powershell -Command "Expand-Archive -Path 'C:\ChaosWorld\nssm.zip' -DestinationPath 'C:\ChaosWorld' -Force"

REM Copy NSSM executable
copy "C:\ChaosWorld\nssm-2.24\win64\nssm.exe" "C:\ChaosWorld\bin\"

REM Clean up
rmdir /s /q "C:\ChaosWorld\nssm-2.24"
del "C:\ChaosWorld\nssm.zip"

REM Add to PATH (optional)
echo Adding NSSM to PATH...
setx PATH "%PATH%;C:\ChaosWorld\bin" /M

echo.
echo NSSM installed successfully!
echo Location: C:\ChaosWorld\bin\nssm.exe
echo.
echo You can now run: C:\ChaosWorld\bin\nssm.exe
echo.

pause
