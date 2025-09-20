//! Caching service for API Gateway

use crate::config::{Config, CachingConfig, TtlConfig};
use crate::errors::{ApiGatewayError, Result};
use crate::types::CacheEntry;
use redis::Client as RedisClient;
use redis::Commands;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{SystemTime, Duration};
use tracing::{info, debug, warn, error};

/// Cache service
#[derive(Debug, Clone)]
pub struct CacheService {
    config: CachingConfig,
    redis_client: Arc<RedisClient>,
    ttl_config: TtlConfig,
    local_cache: Arc<tokio::sync::RwLock<HashMap<String, CacheEntry<String>>>>,
}

/// Cache key types
#[derive(Debug, Clone, PartialEq)]
pub enum CacheKeyType {
    Auth,
    User,
    Game,
    Inventory,
    Chat,
    Guild,
    World,
    Matchmaking,
    Events,
    Content,
    Notifications,
    Payments,
    AntiCheat,
    Analytics,
    Default,
}

/// Initialize cache service
pub async fn init(config: &Config) -> Result<CacheService> {
    CacheService::new(config.clone())
}

impl CacheService {
    /// Create a new cache service
    pub fn new(config: Config) -> Result<Self> {
        let caching_config = config.caching.clone();
        let ttl_config = config.caching.ttl.clone();
        
        // Create Redis client
        let redis_url = format!("redis://{}:{}", 
            caching_config.redis.host, 
            caching_config.redis.port
        );
        
        let redis_client = RedisClient::open(redis_url)
            .map_err(|e| ApiGatewayError::Database(format!("Failed to connect to Redis: {}", e)))?;

        Ok(Self {
            config: caching_config,
            redis_client: Arc::new(redis_client),
            ttl_config,
            local_cache: Arc::new(tokio::sync::RwLock::new(HashMap::new())),
        })
    }

    /// Get a value from cache
    pub async fn get<T>(&self, key: &str, key_type: CacheKeyType) -> Result<Option<T>>
    where
        T: for<'de> Deserialize<'de> + Clone,
    {
        debug!("Getting cache value for key: {} with type: {:?}", key, key_type);

        // Try local cache first
        if let Some(value) = self.get_from_local_cache(key).await? {
            debug!("Cache hit in local cache for key: {}", key);
            return Ok(Some(value));
        }

        // Try Redis cache
        if self.config.enabled {
            if let Some(value) = self.get_from_redis_cache(key).await? {
                debug!("Cache hit in Redis for key: {}", key);
                return Ok(Some(value));
            }
        }

        debug!("Cache miss for key: {}", key);
        Ok(None)
    }

    /// Set a value in cache
    pub async fn set<T>(&self, key: &str, value: T, key_type: CacheKeyType) -> Result<()>
    where
        T: Serialize + Clone,
    {
        debug!("Setting cache value for key: {} with type: {:?}", key, key_type);

        let ttl = self.get_ttl_for_key_type(&key_type);
        
        // Set in local cache
        self.set_in_local_cache(key, value.clone(), ttl).await?;

        // Set in Redis cache if enabled
        if self.config.enabled && ttl > 0 {
            self.set_in_redis_cache(key, value, ttl).await?;
        }

        debug!("Cache value set successfully for key: {}", key);
        Ok(())
    }

    /// Delete a value from cache
    pub async fn delete(&self, key: &str) -> Result<()> {
        debug!("Deleting cache value for key: {}", key);

        // Delete from local cache
        self.delete_from_local_cache(key).await?;

        // Delete from Redis cache if enabled
        if self.config.enabled {
            self.delete_from_redis_cache(key).await?;
        }

        debug!("Cache value deleted successfully for key: {}", key);
        Ok(())
    }

    /// Clear all cache
    pub async fn clear(&self) -> Result<()> {
        debug!("Clearing all cache");

        // Clear local cache
        self.clear_local_cache().await?;

        // Clear Redis cache if enabled
        if self.config.enabled {
            self.clear_redis_cache().await?;
        }

        info!("All cache cleared successfully");
        Ok(())
    }

    /// Get cache statistics
    pub async fn get_stats(&self) -> Result<CacheStats> {
        let local_cache = self.local_cache.read().await;
        let local_size = local_cache.len();

        let mut redis_size = 0;
        if self.config.enabled {
            let mut conn = self.redis_client.get_connection()
                .map_err(|e| ApiGatewayError::Database(format!("Failed to get Redis connection: {}", e)))?;
            
            redis_size = redis::cmd("DBSIZE").query_async::<_, u64>(&mut conn).await
                .map_err(|e| ApiGatewayError::Database(format!("Failed to get Redis size: {}", e)))?;
        }

        Ok(CacheStats {
            local_cache_size: local_size,
            redis_cache_size: redis_size,
            total_size: local_size + redis_size as usize,
        })
    }

    /// Get value from local cache
    async fn get_from_local_cache<T>(&self, key: &str) -> Result<Option<T>>
    where
        T: for<'de> Deserialize<'de>,
    {
        let local_cache = self.local_cache.read().await;
        
        if let Some(entry) = local_cache.get(key) {
            // Check if entry is expired
            if entry.expires_at > SystemTime::now() {
                let value: T = serde_json::from_str(&entry.value)
                    .map_err(|e| ApiGatewayError::Serialization(format!("Failed to deserialize cache value: {}", e)))?;
                return Ok(Some(value));
            } else {
                // Entry is expired, remove it
                drop(local_cache);
                self.delete_from_local_cache(key).await?;
            }
        }

        Ok(None)
    }

    /// Set value in local cache
    async fn set_in_local_cache<T>(&self, key: &str, value: T, ttl: u64) -> Result<()>
    where
        T: Serialize,
    {
        let serialized_value = serde_json::to_string(&value)
            .map_err(|e| ApiGatewayError::Serialization(format!("Failed to serialize cache value: {}", e)))?;

        let now = SystemTime::now();
        let expires_at = now + Duration::from_secs(ttl);

        let entry = CacheEntry {
            key: key.to_string(),
            value: serialized_value,
            ttl,
            created_at: now,
            expires_at,
        };

        let mut local_cache = self.local_cache.write().await;
        local_cache.insert(key.to_string(), entry);

        Ok(())
    }

    /// Delete value from local cache
    async fn delete_from_local_cache(&self, key: &str) -> Result<()> {
        let mut local_cache = self.local_cache.write().await;
        local_cache.remove(key);
        Ok(())
    }

    /// Clear local cache
    async fn clear_local_cache(&self) -> Result<()> {
        let mut local_cache = self.local_cache.write().await;
        local_cache.clear();
        Ok(())
    }

    /// Get value from Redis cache
    async fn get_from_redis_cache<T>(&self, key: &str) -> Result<Option<T>>
    where
        T: for<'de> Deserialize<'de>,
    {
        let mut conn = self.redis_client.get_connection()
            .map_err(|e| ApiGatewayError::Database(format!("Failed to get Redis connection: {}", e)))?;

        let value: Option<String> = conn.get(key)
            .map_err(|e| ApiGatewayError::Database(format!("Failed to get Redis value: {}", e)))?;

        if let Some(value) = value {
            let deserialized: T = serde_json::from_str(&value)
                .map_err(|e| ApiGatewayError::Serialization(format!("Failed to deserialize Redis value: {}", e)))?;
            Ok(Some(deserialized))
        } else {
            Ok(None)
        }
    }

    /// Set value in Redis cache
    async fn set_in_redis_cache<T>(&self, key: &str, value: T, ttl: u64) -> Result<()>
    where
        T: Serialize,
    {
        let mut conn = self.redis_client.get_connection()
            .map_err(|e| ApiGatewayError::Database(format!("Failed to get Redis connection: {}", e)))?;

        let serialized_value = serde_json::to_string(&value)
            .map_err(|e| ApiGatewayError::Serialization(format!("Failed to serialize Redis value: {}", e)))?;

        if ttl > 0 {
            let _: () = conn.set_ex(key, serialized_value, ttl as u64)
                .map_err(|e| ApiGatewayError::Database(format!("Failed to set Redis value with TTL: {}", e)))?;
        } else {
            let _: () = conn.set(key, serialized_value)
                .map_err(|e| ApiGatewayError::Database(format!("Failed to set Redis value: {}", e)))?;
        }

        Ok(())
    }

    /// Delete value from Redis cache
    async fn delete_from_redis_cache(&self, key: &str) -> Result<()> {
        let mut conn = self.redis_client.get_connection()
            .map_err(|e| ApiGatewayError::Database(format!("Failed to get Redis connection: {}", e)))?;

        let _: () = conn.del(key)
            .map_err(|e| ApiGatewayError::Database(format!("Failed to delete Redis value: {}", e)))?;

        Ok(())
    }

    /// Clear Redis cache
    async fn clear_redis_cache(&self) -> Result<()> {
        let mut conn = self.redis_client.get_connection()
            .map_err(|e| ApiGatewayError::Database(format!("Failed to get Redis connection: {}", e)))?;

        let _: () = redis::cmd("FLUSHDB").query_async(&mut conn).await
            .map_err(|e| ApiGatewayError::Database(format!("Failed to clear Redis cache: {}", e)))?;

        Ok(())
    }

    /// Get TTL for key type
    fn get_ttl_for_key_type(&self, key_type: &CacheKeyType) -> u64 {
        match key_type {
            CacheKeyType::Auth => self.ttl_config.auth,
            CacheKeyType::User => self.ttl_config.user,
            CacheKeyType::Game => self.ttl_config.game,
            CacheKeyType::Inventory => self.ttl_config.inventory,
            CacheKeyType::Chat => self.ttl_config.chat,
            CacheKeyType::Guild => self.ttl_config.guild,
            CacheKeyType::World => self.ttl_config.world,
            CacheKeyType::Matchmaking => self.ttl_config.matchmaking,
            CacheKeyType::Events => self.ttl_config.events,
            CacheKeyType::Content => self.ttl_config.content,
            CacheKeyType::Notifications => self.ttl_config.notifications,
            CacheKeyType::Payments => self.ttl_config.payments,
            CacheKeyType::AntiCheat => self.ttl_config.anti_cheat,
            CacheKeyType::Analytics => self.ttl_config.analytics,
            CacheKeyType::Default => self.ttl_config.default,
        }
    }

    /// Create cache key with prefix
    pub fn create_cache_key(&self, prefix: &str, key: &str) -> String {
        format!("{}:{}", prefix, key)
    }

    /// Check if cache is enabled
    pub fn is_enabled(&self) -> bool {
        self.config.enabled
    }

    /// Get cache configuration
    pub fn get_config(&self) -> &CachingConfig {
        &self.config
    }
}

/// Cache statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheStats {
    pub local_cache_size: usize,
    pub redis_cache_size: u64,
    pub total_size: usize,
}

impl CacheStats {
    /// Get cache hit rate
    pub fn get_hit_rate(&self) -> f64 {
        // TODO: Implement actual hit rate calculation
        0.0
    }

    /// Get memory usage
    pub fn get_memory_usage(&self) -> u64 {
        // TODO: Implement actual memory usage calculation
        0
    }
}


