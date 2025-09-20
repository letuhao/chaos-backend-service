@echo off
echo ========================================
echo  Chaos Backend Service - Flag Manager
echo ========================================
echo.

if "%1"=="" (
    echo Usage: manage_flags.bat [command] [args]
    echo.
    echo Commands:
    echo   list                    - List all runtime flags
    echo   set [name] [value]      - Set a runtime flag
    echo   get [name]              - Get a runtime flag value
    echo   reset                   - Reset to default flags
    echo.
    echo Examples:
    echo   manage_flags.bat list
    echo   manage_flags.bat set server_port 9090
    echo   manage_flags.bat set max_connections 2000
    echo   manage_flags.bat get server_port
    echo   manage_flags.bat reset
    echo.
    pause
    exit /b 0
)

if "%1"=="list" (
    python update_flags.py
) else if "%1"=="set" (
    if "%2"=="" (
        echo ❌ Error: Flag name required
        echo Usage: manage_flags.bat set [name] [value]
        pause
        exit /b 1
    )
    if "%3"=="" (
        echo ❌ Error: Flag value required
        echo Usage: manage_flags.bat set [name] [value]
        pause
        exit /b 1
    )
    python update_flags.py %2 %3
) else if "%1"=="get" (
    if "%2"=="" (
        echo ❌ Error: Flag name required
        echo Usage: manage_flags.bat get [name]
        pause
        exit /b 1
    )
    python -c "import pymongo; client=pymongo.MongoClient('mongodb://localhost:27017'); db=client['chaos_game']; flags=db.runtime_flags.find_one({'_id': 'runtime_config'}); print(f'{'%2'}: {flags.get('%2', 'Not found')}')"
) else if "%1"=="reset" (
    echo 🔄 Resetting to default flags...
    python setup_mongodb.py
) else (
    echo ❌ Unknown command: %1
    echo Use 'manage_flags.bat' without arguments to see usage
)

echo.
pause
