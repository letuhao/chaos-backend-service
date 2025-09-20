//! Rate limiting for API Gateway

use crate::config::{Config, RateLimitingConfig, RateLimitRule};
use crate::errors::{ApiGatewayError, Result};
use crate::types::{RequestContext, RateLimit};
use redis::Client as RedisClient;
use redis::Commands;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH, Duration};
use tracing::{info, debug, warn, error};

/// Rate limiter service
#[derive(Debug, Clone)]
pub struct RateLimiter {
    config: RateLimitingConfig,
    redis_client: Arc<RedisClient>,
    rules: HashMap<String, RateLimitRule>,
}

/// Initialize rate limiter
pub async fn init(config: &Config) -> Result<RateLimiter> {
    RateLimiter::new(config.clone())
}

impl RateLimiter {
    /// Create a new rate limiter
    pub fn new(config: Config) -> Result<Self> {
        let rate_limiting_config = config.rate_limiting.clone();
        
        // Create Redis client
        let redis_url = format!("redis://{}:{}", 
            rate_limiting_config.redis.host, 
            rate_limiting_config.redis.port
        );
        
        let redis_client = RedisClient::open(redis_url)
            .map_err(|e| ApiGatewayError::Database(format!("Failed to connect to Redis: {}", e)))?;

        // Build rules map
        let mut rules = HashMap::new();
        for rule in &rate_limiting_config.rules {
            rules.insert(rule.name.clone(), rule.clone());
        }

        Ok(Self {
            config: rate_limiting_config,
            redis_client: Arc::new(redis_client),
            rules,
        })
    }

    /// Check if request is within rate limit
    pub async fn check_rate_limit(&self, context: &RequestContext, rule_name: &str) -> Result<RateLimit> {
        debug!("Checking rate limit for rule: {} and IP: {}", rule_name, context.ip_address);

        let rule = self.rules.get(rule_name)
            .ok_or_else(|| ApiGatewayError::RateLimit(format!("Unknown rate limit rule: {}", rule_name)))?;

        // Create Redis connection
        let mut conn = self.redis_client.get_connection()
            .map_err(|e| ApiGatewayError::Database(format!("Failed to get Redis connection: {}", e)))?;

        // Create rate limit key
        let key = self.create_rate_limit_key(rule_name, &context.ip_address);
        
        // Get current count
        let current: u32 = conn.get(&key).unwrap_or(0);
        
        // Check if limit exceeded
        if current >= rule.limit {
            warn!("Rate limit exceeded for rule: {} and IP: {}", rule_name, context.ip_address);
            
            let rate_limit = RateLimit {
                rule_name: rule_name.to_string(),
                limit: rule.limit,
                window: rule.window,
                current,
                remaining: 0,
                reset_time: SystemTime::now() + Duration::from_secs(rule.window),
                burst: rule.burst,
            };

            return Ok(rate_limit);
        }

        // Increment counter
        let new_count = current + 1;
        
        // Set expiration if this is the first request in the window
        if current == 0 {
            let _: () = conn.set_ex(&key, new_count, rule.window as u64)
                .map_err(|e| ApiGatewayError::Database(format!("Failed to set Redis key: {}", e)))?;
        } else {
            let _: () = conn.set(&key, new_count)
                .map_err(|e| ApiGatewayError::Database(format!("Failed to set Redis key: {}", e)))?;
        }

        let rate_limit = RateLimit {
            rule_name: rule_name.to_string(),
            limit: rule.limit,
            window: rule.window,
            current: new_count,
            remaining: rule.limit - new_count,
            reset_time: SystemTime::now() + Duration::from_secs(rule.window),
            burst: rule.burst,
        };

        debug!("Rate limit check passed for rule: {} and IP: {}", rule_name, context.ip_address);
        Ok(rate_limit)
    }

    /// Check rate limit for multiple rules
    pub async fn check_multiple_rate_limits(&self, context: &RequestContext, rule_names: &[String]) -> Result<Vec<RateLimit>> {
        let mut results = Vec::new();

        for rule_name in rule_names {
            match self.check_rate_limit(context, rule_name).await {
                Ok(rate_limit) => results.push(rate_limit),
                Err(e) => {
                    error!("Rate limit check failed for rule {}: {}", rule_name, e);
                    return Err(e);
                }
            }
        }

        Ok(results)
    }

    /// Check rate limit based on path
    pub async fn check_path_rate_limit(&self, context: &RequestContext, path: &str) -> Result<Option<RateLimit>> {
        debug!("Checking path-based rate limit for path: {}", path);

        // Find applicable rules for this path
        let applicable_rules = self.find_applicable_rules(path);
        
        if applicable_rules.is_empty() {
            debug!("No applicable rate limit rules for path: {}", path);
            return Ok(None);
        }

        // Check the most restrictive rule
        let mut most_restrictive_rule = None;
        let mut most_restrictive_limit = u32::MAX;

        for rule_name in applicable_rules {
            if let Some(rule) = self.rules.get(&rule_name) {
                if rule.limit < most_restrictive_limit {
                    most_restrictive_limit = rule.limit;
                    most_restrictive_rule = Some(rule_name);
                }
            }
        }

        if let Some(rule_name) = most_restrictive_rule {
            self.check_rate_limit(context, &rule_name).await.map(Some)
        } else {
            Ok(None)
        }
    }

    /// Check rate limit based on user roles
    pub async fn check_role_rate_limit(&self, context: &RequestContext, roles: &[String]) -> Result<Option<RateLimit>> {
        debug!("Checking role-based rate limit for roles: {:?}", roles);

        // Find applicable rules for these roles
        let applicable_rules = self.find_applicable_rules_for_roles(roles);
        
        if applicable_rules.is_empty() {
            debug!("No applicable rate limit rules for roles: {:?}", roles);
            return Ok(None);
        }

        // Check the most restrictive rule
        let mut most_restrictive_rule = None;
        let mut most_restrictive_limit = u32::MAX;

        for rule_name in applicable_rules {
            if let Some(rule) = self.rules.get(&rule_name) {
                if rule.limit < most_restrictive_limit {
                    most_restrictive_limit = rule.limit;
                    most_restrictive_rule = Some(rule_name);
                }
            }
        }

        if let Some(rule_name) = most_restrictive_rule {
            self.check_rate_limit(context, &rule_name).await.map(Some)
        } else {
            Ok(None)
        }
    }

    /// Reset rate limit for a specific key
    pub async fn reset_rate_limit(&self, rule_name: &str, identifier: &str) -> Result<()> {
        debug!("Resetting rate limit for rule: {} and identifier: {}", rule_name, identifier);

        let mut conn = self.redis_client.get_connection()
            .map_err(|e| ApiGatewayError::Database(format!("Failed to get Redis connection: {}", e)))?;

        let key = self.create_rate_limit_key(rule_name, identifier);
        let _: () = conn.del(&key)
            .map_err(|e| ApiGatewayError::Database(format!("Failed to delete Redis key: {}", e)))?;

        debug!("Rate limit reset successfully");
        Ok(())
    }

    /// Get current rate limit status
    pub async fn get_rate_limit_status(&self, rule_name: &str, identifier: &str) -> Result<RateLimit> {
        debug!("Getting rate limit status for rule: {} and identifier: {}", rule_name, identifier);

        let rule = self.rules.get(rule_name)
            .ok_or_else(|| ApiGatewayError::RateLimit(format!("Unknown rate limit rule: {}", rule_name)))?;

        let mut conn = self.redis_client.get_connection()
            .map_err(|e| ApiGatewayError::Database(format!("Failed to get Redis connection: {}", e)))?;

        let key = self.create_rate_limit_key(rule_name, identifier);
        let current: u32 = conn.get(&key).unwrap_or(0);

        let rate_limit = RateLimit {
            rule_name: rule_name.to_string(),
            limit: rule.limit,
            window: rule.window,
            current,
            remaining: rule.limit.saturating_sub(current),
            reset_time: SystemTime::now() + Duration::from_secs(rule.window),
            burst: rule.burst,
        };

        Ok(rate_limit)
    }

    /// Create rate limit key
    fn create_rate_limit_key(&self, rule_name: &str, identifier: &str) -> String {
        format!("rate_limit:{}:{}", rule_name, identifier)
    }

    /// Find applicable rules for a path
    fn find_applicable_rules(&self, path: &str) -> Vec<String> {
        let mut applicable_rules = Vec::new();

        for (rule_name, rule) in &self.rules {
            if let Some(paths) = &rule.paths {
                for rule_path in paths {
                    if self.path_matches(path, rule_path) {
                        applicable_rules.push(rule_name.clone());
                        break;
                    }
                }
            }
        }

        applicable_rules
    }

    /// Find applicable rules for roles
    fn find_applicable_rules_for_roles(&self, roles: &[String]) -> Vec<String> {
        let mut applicable_rules = Vec::new();

        for (rule_name, rule) in &self.rules {
            if let Some(rule_roles) = &rule.roles {
                for role in roles {
                    if rule_roles.contains(role) {
                        applicable_rules.push(rule_name.clone());
                        break;
                    }
                }
            }
        }

        applicable_rules
    }

    /// Check if path matches pattern
    fn path_matches(&self, path: &str, pattern: &str) -> bool {
        // Simple pattern matching - can be enhanced with more sophisticated matching
        if pattern.ends_with('*') {
            let prefix = &pattern[..pattern.len() - 1];
            path.starts_with(prefix)
        } else {
            path == pattern
        }
    }

    /// Get all rate limit rules
    pub fn get_rules(&self) -> &HashMap<String, RateLimitRule> {
        &self.rules
    }

    /// Add a new rate limit rule
    pub fn add_rule(&mut self, name: String, rule: RateLimitRule) {
        self.rules.insert(name, rule);
    }

    /// Remove a rate limit rule
    pub fn remove_rule(&mut self, name: &str) -> Option<RateLimitRule> {
        self.rules.remove(name)
    }
}


