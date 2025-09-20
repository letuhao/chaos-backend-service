mod config;
mod auth;
mod monitoring;
mod handlers;

use axum::{
    middleware,
    routing::get,
    Router,
};
use std::net::SocketAddr;
use std::sync::Arc;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use config::Config;
use auth::{AuthService, auth_middleware};
use monitoring::MonitoringService;
use handlers::{
    create_auth_routes, create_monitoring_routes, create_basic_routes, create_protected_routes,
    status_handler,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "content_management_service=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load configuration
    let config = Config::load()?;
    tracing::info!("📋 Configuration loaded successfully");

    // Initialize services
    let auth_service = Arc::new(AuthService::new(
        config.auth.jwt_secret.clone(),
        config.auth.jwt_expiry,
        config.auth.admin_username.clone(),
        config.auth.admin_password.clone(),
    ));

    let monitoring_service = Arc::new(MonitoringService::new());
    tracing::info!("🔧 Services initialized successfully");

    // Create application router
    let app = Router::new()
        // Basic routes (no auth required)
        .merge(create_basic_routes())
        .route("/health", get(status_handler))
        
        // Auth routes (no auth required)
        .nest("/api/v1/auth", create_auth_routes().with_state(auth_service.clone()))
        
        // Monitoring routes (no auth required)
        .nest("/api/v1", create_monitoring_routes().with_state(monitoring_service.clone()))
        
        // Protected routes (auth required) - apply auth middleware only to these routes
        .nest("/api/v1", create_protected_routes()
            .with_state(auth_service.clone())
            .route_layer(middleware::from_fn_with_state(
                auth_service.clone(),
                auth_middleware,
            ))
        )
        
        // Add middleware
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http());

    // Start metrics server if enabled
    if config.monitoring.metrics_enabled {
        let metrics_service = monitoring_service.clone();
        let metrics_port = config.monitoring.metrics_port;
        
        tokio::spawn(async move {
            if let Err(e) = metrics_service.start_metrics_server(metrics_port).await {
                tracing::error!("Failed to start metrics server: {}", e);
            }
        });
        
        tracing::info!("📊 Metrics server will start on port {}", metrics_port);
    }

    // Start main server
    let addr = SocketAddr::from(([0, 0, 0, 0], config.server.port));
    tracing::info!("🚀 CMS Service starting on {}", addr);
    tracing::info!("📝 Admin login: {} / {}", config.auth.admin_username, config.auth.admin_password);
    
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}


