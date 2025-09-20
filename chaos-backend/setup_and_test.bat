@echo off
echo ========================================
echo  Chaos Backend Service - Setup and Test
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

REM Setup MongoDB
echo Setting up MongoDB database and collections...
python setup_mongodb.py
if %errorlevel% neq 0 (
    echo ❌ Failed to setup MongoDB
    pause
    exit /b 1
)

echo.
echo 🚀 Running Chaos Backend Service with MongoDB integration...
echo.

REM Run the server
cargo run --features mongodb-storage

echo.
echo Test completed!
pause
