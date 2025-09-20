@echo off
echo ========================================
echo  MongoDB Connection Test
echo ========================================
echo.

echo Testing MongoDB connection...
python -c "import pymongo; pymongo.MongoClient('mongodb://localhost:27017').admin.command('ping'); print('✅ MongoDB is running')" 2>nul
if %errorlevel% neq 0 (
    echo ❌ MongoDB is not running
    echo.
    echo Please start MongoDB first:
    echo   mongod --dbpath C:\data\db
    echo.
    echo Or install MongoDB if not installed:
    echo   https://www.mongodb.com/try/download/community
    echo.
    pause
    exit /b 1
)

echo.
echo Setting up database...
python setup_mongodb.py
if %errorlevel% neq 0 (
    echo ❌ Failed to setup MongoDB
    pause
    exit /b 1
)

echo.
echo Running Chaos Backend Service...
cargo run --features mongodb-storage

pause
