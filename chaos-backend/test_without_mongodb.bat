@echo off
echo ========================================
echo  Chaos Backend Service - Test Without MongoDB
echo ========================================
echo.

echo Running Chaos Backend Service with default flags...
echo (MongoDB connection will be skipped, using default values)
echo.

cargo run --features mongodb-storage

echo.
echo Test completed!
pause
