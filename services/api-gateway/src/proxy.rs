use crate::config::{ApiGatewayConfig, ServiceConfig, RouteConfig};
use axum::{
    body::Bytes,
    extract::{Path, State},
    http::{HeaderMap, Method, StatusCode},
    response::Response,
};
use reqwest::Client;
use std::collections::HashMap;
use tracing::{error, info, warn};

/// Proxy handler for routes with path parameters (e.g., /auth/:path)
pub async fn proxy_request_with_path(
    State(config): State<ApiGatewayConfig>,
    Path(path): Path<String>,
    method: Method,
    headers: HeaderMap,
    body: Bytes,
) -> Result<Response, StatusCode> {
    // Determine which route this came from based on the request path
    let route = determine_route_from_path(&config, &path);
    proxy_request_internal_with_route(&config, Some(path), method, headers, body, route).await
}

/// Proxy handler for health route
pub async fn proxy_request_health(
    State(config): State<ApiGatewayConfig>,
    method: Method,
    headers: HeaderMap,
    body: Bytes,
) -> Result<Response, StatusCode> {
    let route = config.routing.routes.iter().find(|r| r.path == "/health");
    proxy_request_internal_with_route(&config, None, method, headers, body, route).await
}

/// Proxy handler for API root route
pub async fn proxy_request_api_root(
    State(config): State<ApiGatewayConfig>,
    method: Method,
    headers: HeaderMap,
    body: Bytes,
) -> Result<Response, StatusCode> {
    let route = config.routing.routes.iter().find(|r| r.path == "/api");
    // For API root, we want to send "/" to the backend service
    proxy_request_internal_with_route(&config, Some("/".to_string()), method, headers, body, route).await
}

/// Generic proxy handler that can route to any service
pub async fn proxy_request(
    State(config): State<ApiGatewayConfig>,
    Path(path): Path<String>,
    method: Method,
    headers: HeaderMap,
    body: Bytes,
) -> Result<Response, StatusCode> {
    let route = determine_route_from_path(&config, &path);
    proxy_request_internal_with_route(&config, Some(path), method, headers, body, route).await
}

/// Determine which route a path belongs to
fn determine_route_from_path<'a>(config: &'a ApiGatewayConfig, path: &str) -> Option<&'a RouteConfig> {
    // Check if this path matches any wildcard routes
    for route in &config.routing.routes {
        if route.path.ends_with("/*") {
            let prefix = route.path.trim_end_matches("/*").trim_start_matches('/');
            // For auth routes, check if the path is one of the known auth endpoints
            if prefix == "auth" {
                if path == "register" || path == "login" || path == "me" || path == "refresh" || path == "logout" || path == "logout-all" {
                    return Some(route);
                }
            } else if prefix == "api" {
                // For API routes, any path should match
                return Some(route);
            } else if path.starts_with(prefix) {
                return Some(route);
            }
        }
    }
    None
}

/// Internal proxy logic with explicit route
async fn proxy_request_internal_with_route(
    config: &ApiGatewayConfig,
    path: Option<String>,
    method: Method,
    headers: HeaderMap,
    body: Bytes,
    route: Option<&RouteConfig>,
) -> Result<Response, StatusCode> {
    let route = match route {
        Some(route) => route,
        None => {
            let path_str = path.as_deref().unwrap_or("(direct)");
            info!("❌ No route found for path: /{}", path_str);
            return Err(StatusCode::NOT_FOUND);
        }
    };

    // Check if method is allowed
    if !config.is_method_allowed(route, method.as_str()) {
        warn!("❌ Method {} not allowed for route: {}", method, route.path);
        return Err(StatusCode::METHOD_NOT_ALLOWED);
    }

    // Get service configuration
    let service = match config.get_service(&route.service) {
        Some(service) => service,
        None => {
            error!("❌ Service not found: {}", route.service);
            return Err(StatusCode::BAD_GATEWAY);
        }
    };

    // Build target URL
    let target_path = if route.strip_prefix {
        // Remove the route prefix from the path
        let prefix = route.path.trim_end_matches("/*");
        if prefix == "/health" {
            // Special case for health - route directly to /health
            "health".to_string()
        } else if prefix == "/api" {
            // Special case for API root - route to /
            match &path {
                Some(path) if path == "/" => "".to_string(), // Empty path for root
                Some(path) => path.trim_start_matches('/').to_string(),
                None => "".to_string(), // Empty path for root
            }
        } else {
            match &path {
                Some(path) => path.trim_start_matches(prefix.trim_start_matches('/')).to_string(),
                None => "health".to_string(), // Fallback for direct routes
            }
        }
    } else {
        // Keep the full path - reconstruct it from the route prefix and path
        match &path {
            Some(path) => {
                let prefix = route.path.trim_end_matches("/*").trim_start_matches('/');
                format!("{}/{}", prefix, path)
            },
            None => "health".to_string(), // Fallback for direct routes
        }
    };

    let target_url = format!("http://{}:{}/{}", service.host, service.port, target_path);

    info!("🔍 PROXY REQUEST:");
    info!("  Method: {}", method);
    info!("  Source Path: /{}", path.as_deref().unwrap_or("(direct)"));
    info!("  Target URL: {}", target_url);
    info!("  Service: {}", route.service);
    info!("  Strip Prefix: {}", route.strip_prefix);

    // Create HTTP client
    let client = Client::new();

    // Convert Axum method to Reqwest method
    let reqwest_method = match method.as_str() {
        "GET" => reqwest::Method::GET,
        "POST" => reqwest::Method::POST,
        "PUT" => reqwest::Method::PUT,
        "DELETE" => reqwest::Method::DELETE,
        "PATCH" => reqwest::Method::PATCH,
        "HEAD" => reqwest::Method::HEAD,
        "OPTIONS" => reqwest::Method::OPTIONS,
        _ => {
            error!("❌ Unsupported HTTP method: {}", method);
            return Err(StatusCode::METHOD_NOT_ALLOWED);
        }
    };

    // Build request
    let mut request = client.request(reqwest_method, &target_url);

    // Forward headers (excluding host)
    for (key, value) in headers.iter() {
        if key.as_str() != "host" {
            if let Ok(value_str) = value.to_str() {
                request = request.header(key.as_str(), value_str);
            }
        }
    }

    // Add custom headers if configured
    if let Some(add_headers) = &route.add_headers {
        for (key, value) in add_headers {
            request = request.header(key, value);
        }
    }

    // Add body if present
    if !body.is_empty() {
        request = request.body(body.to_vec());
    }

    info!("🚀 SENDING REQUEST to {}", target_url);

    // Send request
    match request.send().await {
        Ok(response) => {
            let status = response.status().as_u16();
            let response_headers = response.headers().clone();
            let response_body = response.bytes().await.unwrap_or_default();

            info!("✅ RESPONSE RECEIVED:");
            info!("  Status: {}", status);
            info!("  Body Length: {}", response_body.len());

            // Build response
            let mut response_builder = Response::builder()
                .status(StatusCode::from_u16(status).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR));

            // Forward response headers
            for (key, value) in response_headers.iter() {
                if let Ok(value_str) = value.to_str() {
                    response_builder = response_builder.header(key.as_str(), value_str);
                }
            }

            Ok(response_builder
                .body(axum::body::Body::from(response_body))
                .unwrap())
        }
        Err(e) => {
            error!("❌ PROXY REQUEST FAILED:");
            error!("  Error: {}", e);
            error!("  Target URL: {}", target_url);
            error!("  Service: {}", route.service);
            Err(StatusCode::BAD_GATEWAY)
        }
    }
}

/// Health check for a specific service
pub async fn check_service_health(service: &ServiceConfig) -> bool {
    if let Some(health_path) = &service.health_check {
        let health_url = format!("http://{}:{}{}", service.host, service.port, health_path);
        
        match Client::new().get(&health_url).send().await {
            Ok(response) => response.status().is_success(),
            Err(e) => {
                warn!("Health check failed for {}: {}", health_url, e);
                false
            }
        }
    } else {
        // If no health check path is configured, assume the service is healthy
        true
    }
}

/// Get all services and their health status
pub async fn get_services_health(config: &ApiGatewayConfig) -> HashMap<String, bool> {
    let mut health_status = HashMap::new();
    
    for (service_name, service_config) in &config.routing.service_discovery.static_services {
        let is_healthy = check_service_health(service_config).await;
        health_status.insert(service_name.clone(), is_healthy);
        
        info!("🏥 Service {} health: {}", service_name, if is_healthy { "✅ Healthy" } else { "❌ Unhealthy" });
    }
    
    health_status
}
