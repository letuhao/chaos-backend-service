# Redis Setup Guide

This guide will help you connect your Rust application to Redis Cloud using the credentials from your C# example.

## Your Redis Cloud Credentials

Based on your C# example, here are your Redis Cloud connection details:
- **Host**: `redis-17425.c295.ap-southeast-1-1.ec2.redns.redis-cloud.com`
- **Port**: `17425`
- **Username**: `default`
- **Password**: `QSpjGhxjutEqsJj1PeFKpLCLFfMoeoZX`

## Quick Setup

### Option 1: Using PowerShell (Windows)
```powershell
# Run the setup script
.\setup_redis_env.ps1
```

### Option 2: Using Bash (Linux/macOS)
```bash
# Make the script executable and run it
chmod +x setup_redis_env.sh
./setup_redis_env.sh
```

### Option 3: Manual Setup
Create a `.env` file in the `chaos-backend-service` directory with the following content:

```env
# Redis Configuration
ACTOR_CORE_REDIS_URL=redis://default:QSpjGhxjutEqsJj1PeFKpLCLFfMoeoZX@redis-17425.c295.ap-southeast-1-1.ec2.redns.redis-cloud.com:17425

# Redis Connection Settings
ACTOR_CORE_REDIS_CONNECTION_TIMEOUT=10
ACTOR_CORE_REDIS_COMMAND_TIMEOUT=5
ACTOR_CORE_REDIS_MAX_CONNECTIONS=20
ACTOR_CORE_REDIS_USE_TLS=false

# Cache Configuration
ACTOR_CORE_CACHE_DEFAULT_TTL=1800
ACTOR_CORE_CACHE_MAX_ENTRIES=1000000
ACTOR_CORE_CACHE_ENABLE_REDIS=true

# Cache Layer Sizes
ACTOR_CORE_CACHE_L1_SIZE=50000
ACTOR_CORE_CACHE_L2_SIZE=200000
ACTOR_CORE_CACHE_L3_SIZE=500000

# Logging Configuration
RUST_LOG=info,actor_core=debug
ACTOR_CORE_LOG_STRUCTURED=true
ACTOR_CORE_LOG_JSON=false
```

## Testing the Connection

After setting up the `.env` file, test your Redis connection:

```bash
# Test with the example
cargo run --example redis_connection_example --features redis-cache

# Or test with a simple build
cargo build --features redis-cache
```

## Security Notes

⚠️ **IMPORTANT**: 
- The `.env` file is already in `.gitignore` and will NOT be committed to version control
- Never commit Redis credentials to your repository
- Consider using environment variables in production instead of `.env` files
- For production, consider using TLS (rediss://) instead of redis://

## Configuration Options

### Redis URL Formats

**Standard Redis:**
```
redis://username:password@host:port
```

**Redis with TLS (recommended for production):**
```
rediss://username:password@host:port
```

### Environment Variables

| Variable | Description | Default |
|----------|-------------|---------|
| `ACTOR_CORE_REDIS_URL` | Redis connection URL | `redis://localhost:6379` |
| `ACTOR_CORE_REDIS_CONNECTION_TIMEOUT` | Connection timeout (seconds) | `5` |
| `ACTOR_CORE_REDIS_COMMAND_TIMEOUT` | Command timeout (seconds) | `3` |
| `ACTOR_CORE_REDIS_MAX_CONNECTIONS` | Max connections in pool | `10` |
| `ACTOR_CORE_REDIS_USE_TLS` | Use TLS/SSL | `false` |
| `ACTOR_CORE_CACHE_DEFAULT_TTL` | Default cache TTL (seconds) | `1800` |
| `ACTOR_CORE_CACHE_MAX_ENTRIES` | Max entries in memory cache | `1000000` |
| `ACTOR_CORE_CACHE_ENABLE_REDIS` | Enable Redis cache | `false` |

## Usage in Code

```rust
use actor_core::{
    cache::{Cache, CacheFactory},
    config::ActorCoreConfig,
};

// Load configuration
let config = ActorCoreConfig::from_env()?;

// Create cache with Redis
let cache = if config.cache.enable_redis {
    CacheFactory::create_distributed_cache(&config.get_redis_url(), config.cache.default_ttl)?
} else {
    CacheFactory::create_in_memory_cache(config.cache.max_entries, config.cache.default_ttl)
};

// Use the cache
cache.set("key".to_string(), serde_json::json!("value"), Some(300))?;
let value = cache.get("key");
```

## Troubleshooting

### Connection Issues
1. **Check Redis Cloud status**: Ensure your Redis Cloud instance is running
2. **Verify credentials**: Double-check username, password, host, and port
3. **Network connectivity**: Ensure your network allows connections to Redis Cloud
4. **Firewall**: Check if any firewall is blocking the connection

### Common Errors
- **"Connection refused"**: Redis server is not running or wrong host/port
- **"Authentication failed"**: Wrong username or password
- **"Timeout"**: Network issues or Redis server overloaded

### Debug Mode
Enable debug logging to see detailed connection information:
```env
RUST_LOG=debug,actor_core=debug
```

## Next Steps

1. ✅ Set up `.env` file with your Redis credentials
2. ✅ Test the connection with the example
3. 🔄 Integrate Redis cache into your application
4. 🔄 Consider using multi-layer cache for better performance
5. 🔄 Set up monitoring and alerting for Redis

## Support

If you encounter any issues:
1. Check the logs for detailed error messages
2. Verify your Redis Cloud instance is accessible
3. Test with a simple Redis client (like Redis CLI) to confirm connectivity
4. Review the configuration options above
