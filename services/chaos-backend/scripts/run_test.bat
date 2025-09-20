@echo off
echo ========================================
echo  Chaos Backend Service - MongoDB Test
echo ========================================
echo.
echo Available scenarios:
echo   1. File → MongoDB (Load configs from files and save to MongoDB)
echo   2. MongoDB → Runtime (Load configs from MongoDB and use in runtime)
echo.
echo Usage: run_test.bat [scenario]
echo Example: run_test.bat 1
echo.

if "%1"=="" (
    echo Running Scenario 1 (default): File → MongoDB
    cargo run --features mongodb-storage -- 1
) else (
    echo Running Scenario %1
    cargo run --features mongodb-storage -- %1
)

echo.
echo Test completed!
pause
