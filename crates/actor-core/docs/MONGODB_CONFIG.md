# MongoDB Configuration for Actor Core

This document describes the MongoDB configuration feature for Actor Core, which allows you to load and save configuration from MongoDB database instead of just files.

## Overview

The MongoDB configuration system provides:

- **Database Storage**: Store configuration in MongoDB collections
- **Auto-sync**: Automatic synchronization between files and database
- **Fallback Support**: Fallback to file-based configuration when database is unavailable
- **CLI Tools**: Command-line tools for managing configuration sync
- **High Priority**: MongoDB configs have high priority (50) to override file configs

## Features

### 1. MongoDB Configuration Provider

The `MongoDBConfigurationProvider` implements the `ConfigurationProvider` trait and provides:

- Connection to MongoDB database
- Loading configuration from database collections
- Saving configuration to database
- Automatic type conversion between MongoDB documents and Actor Core configuration types

### 2. MongoDB Configuration Manager

The `MongoDBConfigManager` provides high-level management:

- Auto-sync daemon with configurable intervals
- Manual sync operations (files to DB, DB to files, bidirectional)
- Sync status monitoring
- Error handling and recovery

### 3. CLI Tools

Command-line interface for configuration management:

```bash
# Sync configuration from files to MongoDB
actor-core-mongodb-config sync-to-db --config-path configs/ --mongodb-config configs/mongodb_config.yaml

# Load configuration from MongoDB
actor-core-mongodb-config load-from-db --mongodb-config configs/mongodb_config.yaml --output configs/loaded_config.yaml

# Start auto-sync daemon
actor-core-mongodb-config start-daemon --mongodb-config configs/mongodb_config.yaml --interval 300

# Check sync status
actor-core-mongodb-config status --mongodb-config configs/mongodb_config.yaml

# Stop auto-sync daemon
actor-core-mongodb-config stop-daemon --mongodb-config configs/mongodb_config.yaml
```

## Configuration

### MongoDB Configuration File

Create `configs/mongodb_config.yaml`:

```yaml
# MongoDB connection settings
connection_string: "mongodb://localhost:27017"
database_name: "actor_core_config"
collection_name: "configuration"

# Auto-sync settings
enable_auto_sync: true
sync_interval_seconds: 300  # 5 minutes

# Fallback settings
enable_fallback_to_file: true
fallback_file_path: "configs/"

# Performance settings
max_connection_pool_size: 10
connection_timeout_seconds: 30
server_selection_timeout_seconds: 5

# Security settings
enable_ssl: false
ssl_cert_file: ""
ssl_key_file: ""
ssl_ca_file: ""

# Authentication settings
username: ""
password: ""
auth_database: "admin"

# Index settings
create_indexes: true
indexes:
  - fields: ["category", "key"]
    unique: true
  - fields: ["category"]
  - fields: ["source_provider"]
  - fields: ["timestamp"]

# Backup settings
enable_backup: false
backup_interval_hours: 24
backup_retention_days: 7

# Monitoring settings
enable_metrics: true
metrics_collection_interval_seconds: 60
```

### Database Schema

Configuration documents are stored in MongoDB with the following schema:

```json
{
  "_id": "category:key",
  "category": "string",
  "key": "string", 
  "value": "any JSON value",
  "value_type": "string|integer|float|number|boolean|array|object|duration|size|percentage",
  "source_provider": "string",
  "priority": "number",
  "timestamp": "ISO 8601 datetime",
  "can_override": "boolean",
  "can_merge": "boolean",
  "version": "number"
}
```

## Usage

### 1. Enable MongoDB Configuration in Builder

```rust
use actor_core::prelude::*;

#[tokio::main]
async fn main() -> ActorCoreResult<()> {
    // Build Actor Core with MongoDB configuration
    let actor_core = ActorCoreBuilder::new()
        .with_mongodb_config(true)  // Enable MongoDB configuration
        .with_metrics(true)
        .with_caching(true)
        .build()
        .await?;

    // Use the actor core...
    Ok(())
}
```

### 2. Manual Configuration Management

```rust
use actor_core::config::mongodb_manager::*;

#[tokio::main]
async fn main() -> ActorCoreResult<()> {
    // Load MongoDB configuration
    let mongodb_config = MongoDBConfigurationProvider::load_mongodb_config("configs/mongodb_config.yaml")?;
    
    // Create MongoDB manager
    let mongodb_manager = MongoDBConfigManager::new(mongodb_config).await?;
    
    // Start auto-sync
    mongodb_manager.start_auto_sync().await?;
    
    // Manual sync from files to DB
    mongodb_manager.sync_from_files_to_db().await?;
    
    // Check sync status
    let status = mongodb_manager.get_sync_status().await;
    println!("Sync enabled: {}, In progress: {}", status.enabled, status.in_progress);
    
    Ok(())
}
```

### 3. Direct MongoDB Provider Usage

```rust
use actor_core::config::mongodb::*;

#[tokio::main]
async fn main() -> ActorCoreResult<()> {
    // Load MongoDB configuration
    let mongodb_config = MongoDBConfigurationProvider::load_mongodb_config("configs/mongodb_config.yaml")?;
    
    // Create MongoDB provider
    let mongodb_provider = MongoDBConfigurationProvider::new(
        "my_mongodb_provider".to_string(),
        50, // High priority
        mongodb_config,
    ).await?;
    
    // Load configuration from database
    let config_value = mongodb_provider.get_config_value("database", "connection_pool_size").await?;
    
    // Save configuration to database
    let new_value = ConfigurationValue {
        value: serde_json::Value::Number(20.into()),
        value_type: ConfigurationValueType::Integer,
        source_provider: "my_provider".to_string(),
        priority: 100,
        timestamp: chrono::Utc::now(),
        can_override: true,
        can_merge: true,
    };
    
    mongodb_provider.save_to_database("database", "connection_pool_size", &new_value).await?;
    
    Ok(())
}
```

## Sync Operations

### Files to MongoDB

This operation loads all configuration files and saves them to MongoDB:

```rust
// Manual sync
mongodb_manager.sync_from_files_to_db().await?;

// Or using CLI
actor-core-mongodb-config sync-to-db --force
```

### MongoDB to Files

This operation loads configuration from MongoDB and saves it to files:

```rust
// Using CLI
actor-core-mongodb-config load-from-db --output configs/loaded_config.yaml
```

### Bidirectional Sync

This operation synchronizes both directions, resolving conflicts based on priority and timestamps:

```rust
// Using CLI (not yet implemented)
actor-core-mongodb-config sync-bidirectional
```

## Error Handling

The MongoDB configuration system includes comprehensive error handling:

- **Connection Errors**: Automatic fallback to file-based configuration
- **Sync Errors**: Logged with retry mechanisms
- **Validation Errors**: Configuration validation before saving
- **Timeout Errors**: Configurable timeouts for all operations

## Performance Considerations

- **Connection Pooling**: MongoDB connection pooling for better performance
- **Indexing**: Automatic index creation for fast queries
- **Caching**: In-memory caching of frequently accessed configurations
- **Batch Operations**: Batch operations for bulk configuration updates

## Security

- **SSL/TLS Support**: Encrypted connections to MongoDB
- **Authentication**: Username/password authentication
- **Access Control**: Database-level access control
- **Audit Logging**: Configuration change audit logs

## Monitoring

- **Sync Status**: Real-time sync status monitoring
- **Performance Metrics**: Sync performance metrics
- **Error Tracking**: Comprehensive error tracking and reporting
- **Health Checks**: Database connectivity health checks

## Examples

See the following examples:

- `examples/mongodb_config_demo.rs` - Complete MongoDB configuration demo
- `examples/mongodb_sync_example.rs` - Sync operations example
- `examples/mongodb_cli_demo.rs` - CLI usage example

## Troubleshooting

### Common Issues

1. **Connection Failed**: Check MongoDB connection string and network connectivity
2. **Sync Failed**: Check file permissions and MongoDB write access
3. **Type Conversion Error**: Ensure configuration value types match expected types
4. **Priority Conflicts**: Check configuration priorities and merge rules

### Debug Mode

Enable debug logging to troubleshoot issues:

```bash
RUST_LOG=debug actor-core-mongodb-config status
```

### Health Check

Check MongoDB configuration health:

```rust
let status = mongodb_manager.get_sync_status().await;
println!("MongoDB Config Health: {:?}", status);
```
