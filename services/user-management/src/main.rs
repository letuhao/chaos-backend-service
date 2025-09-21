use axum::{
    routing::{get, post},
    Router,
};
use std::net::SocketAddr;
use std::sync::Arc;
use tracing_subscriber;

mod config;
mod handlers;
mod models;
mod services;
mod database;
mod middleware;
mod metrics;

use config::UserServiceConfig;
use handlers::auth::*;
use database::{DatabaseManager, migrations::initialize_database};
use middleware::auth::auth_middleware;
use metrics::METRICS;
use middleware::rate_limit::{ip_rate_limit_middleware, user_rate_limit_middleware};

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::fmt::init();
    
    // Load configuration
    let config = match UserServiceConfig::from_env() {
        Ok(config) => {
            // Validate configuration
            if let Err(errors) = config.validate() {
                tracing::error!("Configuration validation failed:");
                for error in errors {
                    tracing::error!("  - {}", error);
                }
                std::process::exit(1);
            }
            config
        }
        Err(e) => {
            tracing::error!("Failed to load configuration: {}", e);
            std::process::exit(1);
        }
    };

    let config = Arc::new(config);
    
    // Initialize database
    let db_manager = match DatabaseManager::new(&config).await {
        Ok(db) => {
            tracing::info!("✅ Connected to MongoDB database");
            
            // Initialize database with collections and indexes
            if let Err(e) = initialize_database(&db.database).await {
                tracing::error!("Failed to initialize database: {}", e);
                std::process::exit(1);
            }
            
            Arc::new(db)
        }
        Err(e) => {
            tracing::error!("Failed to connect to database: {}", e);
            std::process::exit(1);
        }
    };
    
    // Create main production router
    let app = Router::new()
        .route("/health", get(health_check))
        .route("/metrics", get(metrics_handler))
        .route("/", get(root))
        .route("/auth/register", post(register).layer(axum::middleware::from_fn_with_state(
            (config.clone(), db_manager.clone()),
            ip_rate_limit_middleware
        )))
        .route("/auth/login", post(login).layer(axum::middleware::from_fn_with_state(
            (config.clone(), db_manager.clone()),
            ip_rate_limit_middleware
        )))
        .route("/auth/me", get(me).layer(axum::middleware::from_fn_with_state(
            (config.clone(), db_manager.clone()),
            auth_middleware
        )).layer(axum::middleware::from_fn_with_state(
            (config.clone(), db_manager.clone()),
            user_rate_limit_middleware
        )))
        .route("/auth/refresh", post(refresh_token).layer(axum::middleware::from_fn_with_state(
            (config.clone(), db_manager.clone()),
            user_rate_limit_middleware
        )))
        .route("/auth/logout", post(logout).layer(axum::middleware::from_fn_with_state(
            (config.clone(), db_manager.clone()),
            user_rate_limit_middleware
        )))
        .route("/auth/logout-all", post(logout_all).layer(axum::middleware::from_fn_with_state(
            (config.clone(), db_manager.clone()),
            user_rate_limit_middleware
        )))
        .with_state((config, db_manager));

    // Debug endpoints are disabled in production for security
    tracing::info!("🚀 Production mode - debug endpoints disabled");
    
    // Start server
    let addr = SocketAddr::from(([0, 0, 0, 0], 8082));
    tracing::info!("🚀 user-management server starting on {}", addr);
    tracing::info!("📋 Available endpoints:");
    tracing::info!("  - POST /auth/register - User registration");
    tracing::info!("  - POST /auth/login - User login");
    tracing::info!("  - GET  /auth/me - Get current user profile");
    tracing::info!("  - POST /auth/refresh - Refresh JWT token");
    tracing::info!("  - POST /auth/logout - User logout");
    tracing::info!("  - POST /auth/logout-all - Logout from all devices");
    tracing::info!("  - GET  /health - Health check");
    tracing::info!("  - GET  /metrics - Prometheus metrics");
    
    // Debug endpoints are disabled for security
    
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app.into_make_service()).await.unwrap();
}

/// Metrics handler for Prometheus
async fn metrics_handler() -> String {
    use prometheus::TextEncoder;
    
    let encoder = TextEncoder::new();
    let metric_families = METRICS.registry.gather();
    
    match encoder.encode_to_string(&metric_families) {
        Ok(metrics) => metrics,
        Err(e) => {
            tracing::error!("Failed to encode metrics: {}", e);
            String::new()
        }
    }
}

async fn health_check() -> &'static str {
    "OK"
}

async fn root() -> &'static str {
    "Hello from user-management! Use /auth endpoints for authentication."
}

