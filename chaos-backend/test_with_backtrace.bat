@echo off
echo ========================================
echo  Chaos Backend Service - Test With Backtrace
echo ========================================
echo.
echo Running Chaos Backend Service with backtrace enabled...
echo.

set RUST_BACKTRACE=1
set RUST_BACKTRACE=full

cargo run --features mongodb-storage

echo.
echo Test completed!
pause
