# Chaos Backend Service - Game Server

Real-time game server with MongoDB integration and runtime configuration management.

## 🚀 Quick Start

### Prerequisites
- Rust 1.70+
- MongoDB 4.4+
- Python 3.8+ (for setup scripts)

### 1. Start MongoDB
```bash
# Windows
mongod --dbpath C:\data\db

# Linux/Mac
mongod --dbpath /data/db
```

### 2. Setup Database
```bash
# Windows
python setup_mongodb.py

# Linux/Mac
python3 setup_mongodb.py
```

### 3. Run Server
```bash
# Windows
run_server.bat

# Linux/Mac
chmod +x run_server.sh
./run_server.sh
```

## 🔧 Runtime Flags Management

The server uses MongoDB to store runtime flags that can be changed without restarting.

### Available Flags
- `server_port` - HTTP server port (default: 8080)
- `max_connections` - Maximum concurrent connections (default: 1000)
- `tick_rate` - Game loop frequency in Hz (default: 60)
- `enable_mongodb_sync` - Enable MongoDB config sync (default: true)
- `mongodb_connection` - MongoDB connection string
- `config_sync_interval` - Config sync interval in seconds (default: 300)
- `log_level` - Logging level (default: info)
- `enable_metrics` - Enable performance metrics (default: true)
- `enable_health_checks` - Enable health check endpoints (default: true)
- `world_size` - Game world size (default: 10000)
- `max_actors` - Maximum number of actors (default: 10000)

### Managing Flags

#### List all flags
```bash
# Windows
manage_flags.bat list

# Linux/Mac
python3 update_flags.py
```

#### Set a flag
```bash
# Windows
manage_flags.bat set server_port 9090
manage_flags.bat set max_connections 2000
manage_flags.bat set enable_mongodb_sync false

# Linux/Mac
python3 update_flags.py server_port 9090
python3 update_flags.py max_connections 2000
python3 update_flags.py enable_mongodb_sync false
```

#### Get a flag value
```bash
# Windows
manage_flags.bat get server_port

# Linux/Mac
python3 -c "import pymongo; client=pymongo.MongoClient('mongodb://localhost:27017'); db=client['chaos_game']; flags=db.runtime_flags.find_one({'_id': 'runtime_config'}); print(f'server_port: {flags.get(\"server_port\", \"Not found\")}')"
```

#### Reset to defaults
```bash
# Windows
manage_flags.bat reset

# Linux/Mac
python3 setup_mongodb.py
```

## 🌐 API Endpoints

### Health Checks
- `GET /health` - Basic health check
- `GET /health/detailed` - Detailed health information
- `GET /ready` - Readiness check
- `GET /live` - Liveness check

### Actor Management
- `GET /actors` - List all actors
- `POST /actors` - Create new actor
- `GET /actors/:id` - Get actor by ID
- `PUT /actors/:id` - Update actor
- `DELETE /actors/:id` - Delete actor
- `GET /actors/race/:race` - Get actors by race
- `GET /actors/level/:min/:max` - Get actors by level range

### Configuration
- `GET /config/info` - Get configuration information
- `GET /config/:category` - Get configs by category
- `GET /config/:category/:key` - Get specific config value
- `POST /config/sync` - Trigger config sync

### Metrics
- `GET /metrics` - Performance metrics
- `GET /metrics/actors` - Actor statistics
- `GET /metrics/status` - Server status
- `GET /metrics/report` - Detailed performance report

## 🏗️ Architecture

### Single Process, Multi-threaded
- **Real World Examples**: WoW, LoL, Fortnite, CS:GO
- **Performance**: Ultra-low latency, shared memory
- **Scalability**: Horizontal scaling via multiple instances

### Components
- **Game Server** - Main server runtime
- **Actor Manager** - Actor lifecycle management
- **Config Sync** - MongoDB configuration synchronization
- **Performance Monitor** - Real-time metrics collection
- **HTTP API** - REST endpoints for management

### Database Integration
- **MongoDB** - Primary database for flags and configs
- **Runtime Flags** - Dynamic configuration without restart
- **Config Sync** - Automatic synchronization between files and database

## 🔄 Configuration Flow

### Scenario 1: File → MongoDB
1. Server starts
2. Loads runtime flags from MongoDB
3. Loads configurations from files
4. Syncs configurations to MongoDB
5. Runs with MongoDB-backed configs

### Scenario 2: MongoDB → Runtime
1. Server starts
2. Loads runtime flags from MongoDB
3. Loads configurations from MongoDB
4. Runs with MongoDB-backed configs

## 🛠️ Development

### Build
```bash
cargo build --features mongodb-storage
```

### Run
```bash
cargo run --features mongodb-storage
```

### Test
```bash
cargo test --features mongodb-storage
```

## 📊 Monitoring

The server provides comprehensive monitoring through:
- **Health endpoints** for load balancer integration
- **Metrics endpoints** for performance monitoring
- **Real-time statistics** for operational insights
- **MongoDB integration** for persistent metrics storage

## 🚀 Production Deployment

1. **Setup MongoDB** with proper authentication and security
2. **Configure runtime flags** for your environment
3. **Deploy server** with appropriate resource allocation
4. **Setup monitoring** using the provided endpoints
5. **Configure load balancing** using health check endpoints

## 📝 Notes

- All configuration changes are applied without server restart
- MongoDB connection is required for full functionality
- Server falls back to default values if MongoDB is unavailable
- Performance is optimized for real-time game requirements
