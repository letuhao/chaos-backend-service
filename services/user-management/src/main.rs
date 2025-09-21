use axum::{
    routing::{get, post},
    Router,
};
use tower_http::cors::CorsLayer;
use std::net::SocketAddr;
use std::sync::Arc;
use tracing_subscriber::{self, prelude::*};
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
    // Initialize tracing with both console and file output
    // Create logs directory if it doesn't exist
    std::fs::create_dir_all("C:\\ChaosWorld\\logs").unwrap_or_else(|e| {
        eprintln!("Warning: Could not create logs directory: {}", e);
    });
    
    let file = std::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open("C:\\ChaosWorld\\logs\\user-management.log")
        .unwrap_or_else(|e| {
            eprintln!("Warning: Could not open log file: {}", e);
            std::fs::File::create("user-management.log").unwrap()
        });
    
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "user_management=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer().with_writer(file))
        .with(tracing_subscriber::fmt::layer()) // Also output to console
        .init();

    tracing::info!("🚀 Starting User Management Service...");
    tracing::info!("📁 Current working directory: {:?}", std::env::current_dir());
    tracing::info!("🔍 CONFIG_PATH environment variable: {:?}", std::env::var("CONFIG_PATH"));
    
    // Load configuration from file or environment variables
    tracing::info!("📋 Loading configuration...");
    let config = match UserServiceConfig::load() {
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
    
    // Log configuration details
    tracing::info!("🔧 Configuration loaded successfully:");
    tracing::info!("  Server: {}:{}", config.server.host, config.server.port);
    tracing::info!("  Database: {}", config.database.url);
    tracing::info!("  Redis: {}", config.redis.url);
    
    // Initialize database
    tracing::info!("🗄️ Connecting to MongoDB...");
    tracing::info!("  Database URL: {}", config.database.url);
    tracing::info!("  Timeout: {} seconds", config.database.timeout_seconds);
    
    // Add timeout to database connection
    let db_connection_result = tokio::time::timeout(
        std::time::Duration::from_secs(config.database.timeout_seconds),
        DatabaseManager::new(&config)
    ).await;
    
    let db_manager = match db_connection_result {
        Ok(Ok(db)) => {
            tracing::info!("✅ Connected to MongoDB database");
            
            // Initialize database with collections and indexes
            if let Err(e) = initialize_database(&db.database).await {
                tracing::error!("Failed to initialize database: {}", e);
                std::process::exit(1);
            }
            
            Arc::new(db)
        }
        Ok(Err(e)) => {
            tracing::error!("❌ Failed to connect to database: {}", e);
            std::process::exit(1);
        }
        Err(_) => {
            tracing::error!("❌ Database connection timed out after {} seconds", config.database.timeout_seconds);
            tracing::error!("❌ Please check if MongoDB is running on {}", config.database.url);
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
        .layer(
            CorsLayer::new()
                .allow_origin("http://localhost:3200".parse::<axum::http::HeaderValue>().unwrap())
                .allow_methods([axum::http::Method::GET, axum::http::Method::POST, axum::http::Method::OPTIONS])
                .allow_headers([axum::http::header::CONTENT_TYPE, axum::http::header::AUTHORIZATION])
        )
        .with_state((config.clone(), db_manager));

    // Debug endpoints are disabled in production for security
    tracing::info!("🚀 Production mode - debug endpoints disabled");
    
    // Start server
    let addr = SocketAddr::from(([0, 0, 0, 0], config.clone().server.port));
    tracing::info!("🚀 Starting HTTP server...");
    tracing::info!("  Address: {}", addr);
    tracing::info!("  Workers: {}", config.server.workers);
    tracing::info!("  Max connections: {}", config.server.max_connections);
    tracing::info!("📋 Available endpoints:");
    tracing::info!("  - GET  /health - Health check");
    tracing::info!("  - GET  /metrics - Metrics");
    tracing::info!("  - POST /auth/register - User registration");
    tracing::info!("  - POST /auth/login - User login");
    tracing::info!("  - GET  /auth/me - Get current user");
    tracing::info!("  - POST /auth/refresh - Refresh token");
    tracing::info!("  - POST /auth/logout - Logout");
    tracing::info!("  - POST /auth/logout-all - Logout all sessions");
    tracing::info!("  - GET  /metrics - Prometheus metrics");
    
    // Debug endpoints are disabled for security
    
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    tracing::info!("✅ Server successfully bound to {}", addr);
    tracing::info!("🚀 User Management Service is now running!");
    tracing::info!("📝 Logs are being written to: C:\\ChaosWorld\\logs\\user-management.log");
    
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

