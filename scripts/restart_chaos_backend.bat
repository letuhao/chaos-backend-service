@echo off
echo Restarting Chaos Backend Service...

echo Stopping service...
nssm stop ChaosWorld-Backend

echo Waiting 3 seconds...
timeout /t 3 /nobreak >nul

echo Starting service...
nssm start ChaosWorld-Backend

echo Service restart completed!
pause
