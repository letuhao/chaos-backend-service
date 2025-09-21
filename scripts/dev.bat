@echo off
REM Batch script for development with warnings suppressed

REM Set environment variables to suppress warnings
set RUSTFLAGS=-A unused_variables -A unused_imports -A dead_code -A unused_mut -A unused_assignments

echo 🔧 Development Mode - Warnings Suppressed
echo RUSTFLAGS: %RUSTFLAGS%
echo.

REM Check if command is provided
if "%1"=="" (
    echo Usage: scripts\dev.bat ^<command^>
    echo.
    echo Available commands:
    echo   check     - cargo check
    echo   build     - cargo build
    echo   test      - cargo test
    echo   examples  - cargo run --example ^<name^>
    echo   bench     - cargo bench
    echo   clean     - cargo clean
    echo.
    goto :eof
)

set command=%1

if "%command%"=="check" (
    echo 🔍 Running cargo check...
    cargo check
) else if "%command%"=="build" (
    echo 🔨 Running cargo build...
    cargo build
) else if "%command%"=="test" (
    echo 🧪 Running cargo test...
    cargo test
) else if "%command%"=="examples" (
    echo 📚 Running examples...
    if "%2"=="" (
        echo Available examples:
        echo   new_architecture_demo
        echo   add_new_function_demo
        echo   element_conditions
        echo.
        echo Usage: scripts\dev.bat examples ^<example_name^>
    ) else (
        cargo run --example %2
    )
) else if "%command%"=="bench" (
    echo ⚡ Running cargo bench...
    cargo bench
) else if "%command%"=="clean" (
    echo 🧹 Running cargo clean...
    cargo clean
) else (
    echo ❌ Unknown command: %command%
    echo Run 'scripts\dev.bat' to see available commands
)
