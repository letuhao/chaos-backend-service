use axum::{
    extract::State,
    http::StatusCode,
    response::Json,
    routing::get,
    Router,
};
use prometheus::{
    Counter, Histogram, IntCounter, IntGauge, Registry, TextEncoder,
};
use serde::Serialize;
use std::sync::Arc;
use tokio::net::TcpListener;

#[derive(Debug, Serialize)]
pub struct HealthStatus {
    pub status: String,
    pub timestamp: String,
    pub version: String,
    pub services: ServiceHealth,
}

#[derive(Debug, Serialize)]
pub struct ServiceHealth {
    pub database: ServiceStatus,
    pub cache: ServiceStatus,
    pub storage: ServiceStatus,
}

#[derive(Debug, Serialize)]
pub struct ServiceStatus {
    pub status: String,
    pub response_time_ms: Option<u64>,
    pub error: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct MetricsInfo {
    pub requests_total: u64,
    pub requests_per_second: f64,
    pub response_time_avg_ms: f64,
    pub response_time_p95_ms: f64,
    pub error_rate: f64,
    pub active_connections: u64,
}

pub struct MetricsCollector {
    pub requests_total: Counter,
    #[allow(dead_code)]
    pub request_duration: Histogram,
    pub active_connections: IntGauge,
    pub errors_total: IntCounter,
    #[allow(dead_code)]
    pub database_queries: IntCounter,
    #[allow(dead_code)]
    pub cache_hits: IntCounter,
    #[allow(dead_code)]
    pub cache_misses: IntCounter,
}

impl MetricsCollector {
    pub fn new(registry: &Registry) -> Self {
        let requests_total = Counter::new(
            "cms_requests_total",
            "Total number of HTTP requests",
        )
        .unwrap();

        let request_duration = Histogram::with_opts(
            prometheus::HistogramOpts::new(
                "cms_request_duration_seconds",
                "HTTP request duration in seconds",
            )
            .buckets(vec![0.001, 0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1.0, 2.5, 5.0, 10.0]),
        )
        .unwrap();

        let active_connections = IntGauge::new(
            "cms_active_connections",
            "Number of active connections",
        )
        .unwrap();

        let errors_total = IntCounter::new(
            "cms_errors_total",
            "Total number of errors",
        )
        .unwrap();

        let database_queries = IntCounter::new(
            "cms_database_queries_total",
            "Total number of database queries",
        )
        .unwrap();

        let cache_hits = IntCounter::new(
            "cms_cache_hits_total",
            "Total number of cache hits",
        )
        .unwrap();

        let cache_misses = IntCounter::new(
            "cms_cache_misses_total",
            "Total number of cache misses",
        )
        .unwrap();

        // Register metrics
        registry.register(Box::new(requests_total.clone())).unwrap();
        registry.register(Box::new(request_duration.clone())).unwrap();
        registry.register(Box::new(active_connections.clone())).unwrap();
        registry.register(Box::new(errors_total.clone())).unwrap();
        registry.register(Box::new(database_queries.clone())).unwrap();
        registry.register(Box::new(cache_hits.clone())).unwrap();
        registry.register(Box::new(cache_misses.clone())).unwrap();

        Self {
            requests_total,
            request_duration,
            active_connections,
            errors_total,
            database_queries,
            cache_hits,
            cache_misses,
        }
    }

    #[allow(dead_code)]
    pub fn increment_requests(&self) {
        self.requests_total.inc();
    }

    #[allow(dead_code)]
    pub fn record_request_duration(&self, duration: f64) {
        self.request_duration.observe(duration);
    }

    #[allow(dead_code)]
    pub fn set_active_connections(&self, count: i64) {
        self.active_connections.set(count);
    }

    #[allow(dead_code)]
    pub fn increment_errors(&self) {
        self.errors_total.inc();
    }

    #[allow(dead_code)]
    pub fn increment_database_queries(&self) {
        self.database_queries.inc();
    }

    #[allow(dead_code)]
    pub fn increment_cache_hits(&self) {
        self.cache_hits.inc();
    }

    #[allow(dead_code)]
    pub fn increment_cache_misses(&self) {
        self.cache_misses.inc();
    }
}

pub struct MonitoringService {
    pub metrics_collector: Arc<MetricsCollector>,
    pub registry: Arc<Registry>,
}

impl MonitoringService {
    pub fn new() -> Self {
        let registry = Arc::new(Registry::new());
        let metrics_collector = Arc::new(MetricsCollector::new(&registry));

        Self {
            metrics_collector,
            registry,
        }
    }

    pub async fn check_health(&self) -> HealthStatus {
        let timestamp = chrono::Utc::now().to_rfc3339();
        let version = env!("CARGO_PKG_VERSION").to_string();

        // Check database health
        let database_status = self.check_database_health().await;
        
        // Check cache health
        let cache_status = self.check_cache_health().await;
        
        // Check storage health
        let storage_status = self.check_storage_health().await;

        let services = ServiceHealth {
            database: database_status,
            cache: cache_status,
            storage: storage_status,
        };

        let overall_status = if services.database.status == "healthy" 
            && services.cache.status == "healthy" 
            && services.storage.status == "healthy" {
            "healthy"
        } else {
            "unhealthy"
        };

        HealthStatus {
            status: overall_status.to_string(),
            timestamp,
            version,
            services,
        }
    }

    async fn check_database_health(&self) -> ServiceStatus {
        // For now, we'll simulate database health check
        // In a real implementation, you would ping the database
        ServiceStatus {
            status: "healthy".to_string(),
            response_time_ms: Some(15),
            error: None,
        }
    }

    async fn check_cache_health(&self) -> ServiceStatus {
        // For now, we'll simulate cache health check
        // In a real implementation, you would ping Redis
        ServiceStatus {
            status: "healthy".to_string(),
            response_time_ms: Some(5),
            error: None,
        }
    }

    async fn check_storage_health(&self) -> ServiceStatus {
        // For now, we'll simulate storage health check
        // In a real implementation, you would check file system
        ServiceStatus {
            status: "healthy".to_string(),
            response_time_ms: Some(2),
            error: None,
        }
    }

    pub fn get_metrics_info(&self) -> MetricsInfo {
        // Get metrics values
        let requests_total = self.metrics_collector.requests_total.get() as u64;
        let active_connections = self.metrics_collector.active_connections.get() as u64;
        let errors_total = self.metrics_collector.errors_total.get() as u64;
        
        // Calculate derived metrics
        let error_rate = if requests_total > 0 {
            errors_total as f64 / requests_total as f64
        } else {
            0.0
        };

        // For now, return simulated values
        // In a real implementation, you would calculate these from actual metrics
        MetricsInfo {
            requests_total,
            requests_per_second: 25.5,
            response_time_avg_ms: 120.0,
            response_time_p95_ms: 250.0,
            error_rate,
            active_connections,
        }
    }

    pub async fn start_metrics_server(&self, port: u16) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let metrics_router = Router::new()
            .route("/metrics", get(metrics_handler))
            .with_state(self.registry.clone());

        let addr = format!("0.0.0.0:{}", port);
        let listener = TcpListener::bind(&addr).await?;
        
        tracing::info!("📊 Metrics server starting on {}", addr);
        
        axum::serve(listener, metrics_router).await?;
        Ok(())
    }
}

async fn metrics_handler(
    State(registry): State<Arc<Registry>>,
) -> Result<String, StatusCode> {
    let encoder = TextEncoder::new();
    let metric_families = registry.gather();
    
    match encoder.encode_to_string(&metric_families) {
        Ok(metrics) => Ok(metrics),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

#[allow(dead_code)]
pub fn create_monitoring_routes() -> Router<Arc<MonitoringService>> {
    Router::new()
        .route("/health", get(health_handler))
        .route("/health/detailed", get(detailed_health_handler))
        .route("/metrics/info", get(metrics_info_handler))
}

#[allow(dead_code)]
async fn health_handler(
    State(monitoring): State<Arc<MonitoringService>>,
) -> Result<Json<HealthStatus>, StatusCode> {
    let health = monitoring.check_health().await;
    Ok(Json(health))
}

#[allow(dead_code)]
async fn detailed_health_handler(
    State(monitoring): State<Arc<MonitoringService>>,
) -> Result<Json<HealthStatus>, StatusCode> {
    let health = monitoring.check_health().await;
    Ok(Json(health))
}

#[allow(dead_code)]
async fn metrics_info_handler(
    State(monitoring): State<Arc<MonitoringService>>,
) -> Result<Json<MetricsInfo>, StatusCode> {
    let metrics_info = monitoring.get_metrics_info();
    Ok(Json(metrics_info))
}
