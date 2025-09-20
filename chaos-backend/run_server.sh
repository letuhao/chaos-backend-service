#!/bin/bash

echo "========================================"
echo " Chaos Backend Service - Game Server"
echo "========================================"
echo ""

# Check if MongoDB is running
echo "Checking MongoDB connection..."
python3 -c "import pymongo; pymongo.MongoClient('mongodb://localhost:27017').admin.command('ping')" 2>/dev/null
if [ $? -ne 0 ]; then
    echo "❌ MongoDB is not running or not accessible"
    echo "Please start MongoDB first:"
    echo "  mongod --dbpath /data/db"
    echo ""
    exit 1
fi

echo "✅ MongoDB is running"
echo ""

# Setup MongoDB if needed
echo "Setting up MongoDB..."
python3 setup_mongodb.py
if [ $? -ne 0 ]; then
    echo "❌ Failed to setup MongoDB"
    exit 1
fi

echo ""
echo "🚀 Starting Chaos Backend Service..."
echo ""
echo "Available endpoints:"
echo "  Health: http://localhost:8080/health"
echo "  Actors: http://localhost:8080/actors"
echo "  Metrics: http://localhost:8080/metrics"
echo "  Config: http://localhost:8080/config/info"
echo ""
echo "Press Ctrl+C to stop the server"
echo ""

# Run the server
cargo run --features mongodb-storage

echo ""
echo "Server stopped."
