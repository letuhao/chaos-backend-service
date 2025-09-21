use axum::{
    extract::{Request, State},
    http::{HeaderMap, StatusCode},
    middleware::Next,
    response::Response,
};
use serde_json::json;
use std::sync::Arc;
use std::collections::HashMap;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;

use crate::config::UserServiceConfig;
use crate::database::DatabaseManager;

/// Rate limit entry for tracking requests
#[derive(Debug, Clone)]
struct RateLimitEntry {
    count: u32,
    reset_time: Instant,
}

/// Rate limiter using in-memory storage
pub struct RateLimiter {
    limits: Arc<RwLock<HashMap<String, RateLimitEntry>>>,
    max_requests: u32,
    window_duration: Duration,
}

impl RateLimiter {
    pub fn new(max_requests: u32, window_duration_seconds: u64) -> Self {
        Self {
            limits: Arc::new(RwLock::new(HashMap::new())),
            max_requests,
            window_duration: Duration::from_secs(window_duration_seconds),
        }
    }

    /// Check if request is allowed and update counter
    pub async fn is_allowed(&self, key: &str) -> (bool, u32, u32) {
        let mut limits = self.limits.write().await;
        let now = Instant::now();

        // Clean up expired entries
        limits.retain(|_, entry| entry.reset_time > now);

        let entry = limits.get_mut(key);
        match entry {
            Some(entry) => {
                if entry.reset_time <= now {
                    // Window expired, reset
                    entry.count = 1;
                    entry.reset_time = now + self.window_duration;
                    (true, 1, self.max_requests - 1)
                } else if entry.count < self.max_requests {
                    // Within limit, increment
                    entry.count += 1;
                    let remaining = self.max_requests - entry.count;
                    (true, entry.count, remaining)
                } else {
                    // Rate limit exceeded
                    (false, entry.count, 0)
                }
            }
            None => {
                // First request
                let entry = RateLimitEntry {
                    count: 1,
                    reset_time: now + self.window_duration,
                };
                limits.insert(key.to_string(), entry);
                let remaining = self.max_requests - 1;
                (true, 1, remaining)
            }
        }
    }
}

// Unused general rate limiting middleware removed for cleaner code

/// IP-based rate limiting middleware
pub async fn ip_rate_limit_middleware(
    State((_config, _db_manager)): State<(Arc<UserServiceConfig>, Arc<DatabaseManager>)>,
    headers: HeaderMap,
    request: Request,
    next: Next,
) -> Result<Response, (StatusCode, axum::Json<serde_json::Value>)> {
    // Create rate limiter (50 requests per 5 minutes for IP-based)
    let rate_limiter = RateLimiter::new(50, 300); // 5 minutes = 300 seconds

    // Get client IP from headers
    let client_ip = headers
        .get("x-forwarded-for")
        .or_else(|| headers.get("x-real-ip"))
        .or_else(|| headers.get("x-client-ip"))
        .and_then(|header| header.to_str().ok())
        .unwrap_or("unknown");

    // Check rate limit
    let (allowed, current_count, remaining) = rate_limiter.is_allowed(client_ip).await;

    if !allowed {
        let error_response = json!({
            "success": false,
            "error": "Rate limit exceeded",
            "details": {
                "limit": 50,
                "current": current_count,
                "remaining": remaining,
                "reset_in_seconds": 300
            }
        });

        return Err((StatusCode::TOO_MANY_REQUESTS, axum::Json(error_response)));
    }

    // Add rate limit headers to response
    let mut response = next.run(request).await;
    let headers = response.headers_mut();
    headers.insert("x-ratelimit-limit", "50".parse().unwrap());
    headers.insert("x-ratelimit-remaining", remaining.to_string().parse().unwrap());
    headers.insert("x-ratelimit-reset", "300".parse().unwrap());

    Ok(response)
}

/// User-based rate limiting middleware (requires authentication)
pub async fn user_rate_limit_middleware(
    State((_config, _db_manager)): State<(Arc<UserServiceConfig>, Arc<DatabaseManager>)>,
    headers: HeaderMap,
    request: Request,
    next: Next,
) -> Result<Response, (StatusCode, axum::Json<serde_json::Value>)> {
    // Create rate limiter (200 requests per 10 minutes for authenticated users)
    let rate_limiter = RateLimiter::new(200, 600); // 10 minutes = 600 seconds

    // Get user ID from request extensions (set by auth middleware)
    let user_id = request.extensions().get::<String>();
    let rate_key = match user_id {
        Some(id) => format!("user:{}", id),
        None => {
            // Fallback to IP if no user ID
            let client_ip = headers
                .get("x-forwarded-for")
                .or_else(|| headers.get("x-real-ip"))
                .or_else(|| headers.get("x-client-ip"))
                .and_then(|header| header.to_str().ok())
                .unwrap_or("unknown");
            format!("ip:{}", client_ip)
        }
    };

    // Check rate limit
    let (allowed, current_count, remaining) = rate_limiter.is_allowed(&rate_key).await;

    if !allowed {
        let error_response = json!({
            "success": false,
            "error": "Rate limit exceeded",
            "details": {
                "limit": 200,
                "current": current_count,
                "remaining": remaining,
                "reset_in_seconds": 600
            }
        });

        return Err((StatusCode::TOO_MANY_REQUESTS, axum::Json(error_response)));
    }

    // Add rate limit headers to response
    let mut response = next.run(request).await;
    let headers = response.headers_mut();
    headers.insert("x-ratelimit-limit", "200".parse().unwrap());
    headers.insert("x-ratelimit-remaining", remaining.to_string().parse().unwrap());
    headers.insert("x-ratelimit-reset", "600".parse().unwrap());

    Ok(response)
}