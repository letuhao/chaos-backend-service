# Chaos Backend Service

## Overview
This is the main Chaos Backend service that handles the core game logic and integrates with Actor Core.

## Features
- Game logic processing
- Actor management
- MongoDB integration
- Configuration management
- Health checks

## Development

### Prerequisites
- Rust 1.70+
- MongoDB
- Redis

### Running the service
`ash
cargo run --bin chaos-backend
`

### Running MongoDB checker
`ash
cargo run --bin check_mongodb
`

### Configuration
Configuration files are located in the configs/ directory.

### Testing
`ash
cargo test
`

## Scripts
- manage_flags.bat - Manage MongoDB flags
- un_server.bat - Start the server
- 	est_mongodb.bat - Test MongoDB connection
- setup_mongodb.py - Setup MongoDB data
