#!/bin/bash

echo "========================================"
echo " Chaos Backend Service - MongoDB Test"
echo "========================================"
echo ""
echo "Available scenarios:"
echo "  1. File → MongoDB (Load configs from files and save to MongoDB)"
echo "  2. MongoDB → Runtime (Load configs from MongoDB and use in runtime)"
echo ""
echo "Usage: ./run_test.sh [scenario]"
echo "Example: ./run_test.sh 1"
echo ""

if [ -z "$1" ]; then
    echo "Running Scenario 1 (default): File → MongoDB"
    cargo run --features mongodb-storage -- 1
else
    echo "Running Scenario $1"
    cargo run --features mongodb-storage -- $1
fi

echo ""
echo "Test completed!"
