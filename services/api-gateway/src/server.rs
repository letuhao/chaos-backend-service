//! API Gateway server implementation

use crate::config::Config;
use crate::errors::{ApiGatewayError, Result};
use axum::Router;
use crate::monitoring::MonitoringService;
use axum::{
    Router as AxumRouter,
    middleware,
};
use std::net::SocketAddr;
use tower::ServiceBuilder;
use tower_http::{
    cors::CorsLayer,
    trace::TraceLayer,
    compression::CompressionLayer,
};
use tracing::{info, error};

/// API Gateway server
pub struct ApiGatewayServer {
    config: Config,
    router: axum::Router,
    monitoring: MonitoringService,
}

impl ApiGatewayServer {
    /// Create a new API Gateway server
    pub fn new(config: Config, router: axum::Router) -> Result<Self> {
        let monitoring = MonitoringService::new(&config)?;
        
        Ok(Self {
            config,
            router,
            monitoring,
        })
    }

    /// Run the API Gateway server
    pub async fn run(self) -> Result<()> {
        let addr = self.config.server_address().parse::<SocketAddr>()
            .map_err(|e| ApiGatewayError::Config(format!("Invalid server address: {}", e)))?;

        info!("🚀 Starting API Gateway server on {}", addr);

        // Build the Axum router
        let app = self.build_router().await?;

        // Start the server
        let listener = tokio::net::TcpListener::bind(addr).await
            .map_err(|e| ApiGatewayError::Server(format!("Failed to bind to address: {}", e)))?;

        info!("✅ API Gateway server started successfully on {}", addr);

        // Start monitoring in background
        let monitoring = self.monitoring.clone();
        tokio::spawn(async move {
            if let Err(e) = monitoring.start().await {
                error!("❌ Monitoring service failed: {}", e);
            }
        });

        // Serve the application
        axum::serve(listener, app).await
            .map_err(|e| ApiGatewayError::Server(format!("Server error: {}", e)))?;

        Ok(())
    }

    /// Build the Axum router with middleware
    async fn build_router(&self) -> Result<AxumRouter> {
        // Create the base router from our routing system
        let base_router = self.router.clone();

        // Build middleware stack
        let middleware_stack = self.build_middleware_stack().await?;

        // Apply middleware to the router
        let app = base_router
            .layer(middleware_stack)
            .layer(middleware::from_fn(crate::middleware::request_logging))
            .layer(middleware::from_fn(crate::middleware::error_handling));

        Ok(app)
    }

    /// Build the middleware stack
    async fn build_middleware_stack(&self) -> Result<ServiceBuilder<()>> {
        let mut stack = ServiceBuilder::new();

        // CORS middleware
        if self.config.security.cors.enabled {
            let cors = CorsLayer::new()
                .allow_origin(self.config.security.cors.origins.iter().map(|s| s.parse().unwrap()).collect::<Vec<_>>())
                .allow_methods(self.config.security.cors.methods.iter().map(|s| s.parse().unwrap()).collect::<Vec<_>>())
                .allow_headers(self.config.security.cors.headers.iter().map(|s| s.parse().unwrap()).collect::<Vec<_>>())
                .allow_credentials(self.config.security.cors.credentials)
                .max_age(std::time::Duration::from_secs(self.config.security.cors.max_age));

            stack = stack.layer(cors);
        }

        // Compression middleware
        if self.config.is_production() && self.config.production.compress_responses {
            stack = stack.layer(CompressionLayer::new());
        }

        // Timeout middleware - skip for now as TimeoutLayer is not available
        // let timeout_duration = std::time::Duration::from_secs(self.config.server.timeout);
        // stack = stack.layer(TimeoutLayer::new(timeout_duration));

        // Request body limit middleware
        let body_limit = 10 * 1024 * 1024; // 10MB
        // stack = stack.layer(RequestBodyLimitLayer::new(body_limit)); // Skip for now

        // Tracing middleware
        if self.config.monitoring.tracing.enabled {
            let trace_layer = TraceLayer::new_for_http()
                .make_span_with(|request: &axum::http::Request<_>| {
                    tracing::info_span!(
                        "http_request",
                        method = %request.method(),
                        uri = %request.uri(),
                        version = ?request.version(),
                    )
                })
                .on_request(|request: &axum::http::Request<_>, _span: &tracing::Span| {
                    tracing::info!("Started processing request: {} {}", request.method(), request.uri());
                })
                .on_response(|response: &axum::http::Response<_>, latency: std::time::Duration, _span: &tracing::Span| {
                    tracing::info!(
                        "Finished processing request: {}ms, status: {}",
                        latency.as_millis(),
                        response.status()
                    );
                });

            stack = stack.layer(trace_layer);
        }

        // Security headers middleware
        stack = stack.layer(middleware::from_fn(crate::middleware::security_headers));

        // Rate limiting middleware
        if self.config.rate_limiting.enabled {
            stack = stack.layer(middleware::from_fn(crate::middleware::rate_limiting));
        }

        // Authentication middleware
        stack = stack.layer(middleware::from_fn(crate::middleware::authentication));

        // Authorization middleware
        stack = stack.layer(middleware::from_fn(crate::middleware::authorization));

        // Circuit breaker middleware
        if self.config.routing.circuit_breaker.enabled {
            stack = stack.layer(middleware::from_fn(crate::middleware::circuit_breaker));
        }

        // Load balancing middleware
        stack = stack.layer(middleware::from_fn(crate::middleware::load_balancing));

        // Caching middleware
        if self.config.caching.enabled {
            stack = stack.layer(middleware::from_fn(crate::middleware::caching));
        }

        // Metrics middleware
        if self.config.metrics.enabled {
            stack = stack.layer(middleware::from_fn(crate::middleware::metrics));
        }

        Ok(stack)
    }

    /// Get server configuration
    pub fn config(&self) -> &Config {
        &self.config
    }

    /// Get router
    pub fn router(&self) -> &axum::Router {
        &self.router
    }

    /// Get monitoring service
    pub fn monitoring(&self) -> &MonitoringService {
        &self.monitoring
    }

    /// Graceful shutdown
    pub async fn shutdown(&self) -> Result<()> {
        info!("🛑 Shutting down API Gateway server...");
        
        // Stop monitoring
        self.monitoring.stop().await?;
        
        info!("✅ API Gateway server shutdown complete");
        Ok(())
    }
}

impl Clone for ApiGatewayServer {
    fn clone(&self) -> Self {
        Self {
            config: self.config.clone(),
            router: self.router.clone(),
            monitoring: self.monitoring.clone(),
        }
    }
}

