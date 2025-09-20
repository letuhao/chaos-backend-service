@echo off
echo ========================================
echo  Chaos Backend Service - Game Server
echo ========================================
echo.

REM Check if MongoDB is running
echo Checking MongoDB connection...
python -c "import pymongo; pymongo.MongoClient('mongodb://localhost:27017').admin.command('ping')" 2>nul
if %errorlevel% neq 0 (
    echo ❌ MongoDB is not running or not accessible
    echo Please start MongoDB first:
    echo   mongod --dbpath C:\data\db
    echo.
    pause
    exit /b 1
)

echo ✅ MongoDB is running
echo.

REM Setup MongoDB if needed
echo Setting up MongoDB...
python setup_mongodb.py
if %errorlevel% neq 0 (
    echo ❌ Failed to setup MongoDB
    pause
    exit /b 1
)

echo.
echo 🚀 Starting Chaos Backend Service...
echo.
echo Available endpoints:
echo   Health: http://localhost:8080/health
echo   Actors: http://localhost:8080/actors
echo   Metrics: http://localhost:8080/metrics
echo   Config: http://localhost:8080/config/info
echo.
echo Press Ctrl+C to stop the server
echo.

REM Run the server
cargo run --features mongodb-storage

echo.
echo Server stopped.
pause
