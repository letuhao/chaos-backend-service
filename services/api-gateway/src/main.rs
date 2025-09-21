use axum::{
    extract::{Path, State},
    http::Method,
    response::Response,
    routing::{get, post, put, delete, options},
    Router,
    body::Bytes,
};
use tower_http::cors::CorsLayer;
use std::net::SocketAddr;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod config;
mod proxy;

use config::ApiGatewayConfig;
use proxy::{proxy_request, proxy_request_with_path, proxy_request_health, proxy_request_api_root, get_services_health};

#[tokio::main]
async fn main() {
    // Initialize tracing
    // Create logs directory if it doesn't exist
    std::fs::create_dir_all("C:\\ChaosWorld\\logs").unwrap_or_else(|e| {
        eprintln!("Warning: Could not create logs directory: {}", e);
    });
    
    let file = std::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open("C:\\ChaosWorld\\logs\\api-gateway.log")
        .unwrap_or_else(|e| {
            eprintln!("Warning: Could not open log file: {}", e);
            std::fs::File::create("api-gateway.log").unwrap()
        });
    
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "api_gateway=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer().with_writer(file))
        .with(tracing_subscriber::fmt::layer()) // Also output to console
        .init();

    // Load configuration
    let config_path = std::env::var("CONFIG_PATH")
        .unwrap_or_else(|_| "configs/api-gateway.yaml".to_string());
    tracing::info!("🔍 Looking for config file at: {}", config_path);
    tracing::info!("🔍 Current working directory: {:?}", std::env::current_dir());
    tracing::info!("🔍 Config file exists: {}", std::path::Path::new(&config_path).exists());
    
    let config = match ApiGatewayConfig::from_env() {
        Ok(config) => {
            tracing::info!("✅ Configuration loaded successfully from file");
            config
        }
        Err(e) => {
            tracing::error!("❌ Failed to load configuration: {}", e);
            tracing::info!("Using default configuration");
            ApiGatewayConfig::default()
        }
    };

    // Log detailed configuration
    tracing::info!("🚀 API Gateway Configuration:");
    tracing::info!("  Server: {}:{}", config.server.host, config.server.port);
    tracing::info!("  Services:");
    for (name, service) in &config.routing.service_discovery.static_services {
        tracing::info!("    {}: {}:{} (health: {:?})", name, service.host, service.port, service.health_check);
    }
    tracing::info!("  Routes:");
    for route in &config.routing.routes {
        tracing::info!("    {} -> {} (methods: {:?})", route.path, route.service, route.methods);
    }

    // Check services health
    let _health_status = get_services_health(&config).await;

    // Create router with routes from configuration
    let mut app = Router::new()
        .route("/", get(root))
        .route("/services/health", get(services_health_handler));
    
    // Add routes from configuration
    for route in &config.routing.routes {
        let path_pattern = &route.path;
        
        // Handle routes differently based on whether they have path parameters
        if path_pattern.ends_with("/*") {
            // Wildcard routes - convert to Axum format with path parameter
            let prefix = path_pattern.trim_end_matches("/*");
            let axum_pattern = format!("{}/:path", prefix);
            
            tracing::info!("🔧 Registering wildcard route: {} -> {}", path_pattern, axum_pattern);
            
            // Add route with all specified methods
            for method in &route.methods {
                match method.as_str() {
                    "GET" => app = app.route(&axum_pattern, get(proxy_request_with_path)),
                    "POST" => app = app.route(&axum_pattern, post(proxy_request_with_path)),
                    "PUT" => app = app.route(&axum_pattern, put(proxy_request_with_path)),
                    "DELETE" => app = app.route(&axum_pattern, delete(proxy_request_with_path)),
                    "OPTIONS" => app = app.route(&axum_pattern, options(proxy_request_with_path)),
                    _ => continue,
                }
            }
        } else {
            // Direct routes without path parameters
            tracing::info!("🔧 Registering direct route: {}", path_pattern);
            
            for method in &route.methods {
                match method.as_str() {
                    "GET" => {
                        if path_pattern == "/health" {
                            app = app.route(path_pattern, get(proxy_request_health));
                        } else if path_pattern == "/api" {
                            app = app.route(path_pattern, get(proxy_request_api_root));
                        }
                    },
                    "POST" => {
                        if path_pattern == "/health" {
                            app = app.route(path_pattern, post(proxy_request_health));
                        } else if path_pattern == "/api" {
                            app = app.route(path_pattern, post(proxy_request_api_root));
                        }
                    },
                    "PUT" => {
                        if path_pattern == "/health" {
                            app = app.route(path_pattern, put(proxy_request_health));
                        } else if path_pattern == "/api" {
                            app = app.route(path_pattern, put(proxy_request_api_root));
                        }
                    },
                    "DELETE" => {
                        if path_pattern == "/health" {
                            app = app.route(path_pattern, delete(proxy_request_health));
                        } else if path_pattern == "/api" {
                            app = app.route(path_pattern, delete(proxy_request_api_root));
                        }
                    },
                    "OPTIONS" => {
                        if path_pattern == "/health" {
                            app = app.route(path_pattern, options(proxy_request_health));
                        } else if path_pattern == "/api" {
                            app = app.route(path_pattern, options(proxy_request_api_root));
                        }
                    },
                    _ => continue,
                }
            }
        }
    }
    
    let app = app
        .with_state(config.clone())
        .layer(
            CorsLayer::new()
                .allow_origin("http://localhost:3200".parse::<axum::http::HeaderValue>().unwrap())
                .allow_methods([axum::http::Method::GET, axum::http::Method::POST, axum::http::Method::PUT, axum::http::Method::DELETE, axum::http::Method::OPTIONS])
                .allow_headers([axum::http::header::CONTENT_TYPE, axum::http::header::AUTHORIZATION])
        );

    // Start server
    let addr = SocketAddr::from(([0, 0, 0, 0], config.server.port));
    tracing::info!("🚀 API Gateway server starting on {}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app.into_make_service()).await.unwrap();
}

async fn root() -> &'static str {
    "Hello from API Gateway!"
}

async fn services_health_handler(State(config): State<ApiGatewayConfig>) -> String {
    let health_status = get_services_health(&config).await;
    
    let mut response = String::from("Services Health Status:\n");
    for (service_name, is_healthy) in health_status {
        response.push_str(&format!("  {}: {}\n", service_name, if is_healthy { "✅ Healthy" } else { "❌ Unhealthy" }));
    }
    
    response
}