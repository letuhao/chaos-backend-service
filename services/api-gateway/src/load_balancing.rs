//! Load balancing for API Gateway

use crate::config::{Config, LoadBalancingConfig, ServiceConfig};
use crate::errors::{ApiGatewayError, Result};
use crate::types::{Service, ServiceStatus, LoadBalancerStats, ServiceStats};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{SystemTime, Duration};
use tracing::{info, debug, warn, error};
use tokio::sync::RwLock;

/// Load balancer service
#[derive(Debug, Clone)]
pub struct LoadBalancer {
    config: LoadBalancingConfig,
    services: Arc<RwLock<HashMap<String, Vec<Service>>>>,
    stats: Arc<RwLock<LoadBalancerStats>>,
    current_index: Arc<RwLock<HashMap<String, usize>>>,
}

/// Load balancing algorithm
#[derive(Debug, Clone, PartialEq)]
pub enum LoadBalancingAlgorithm {
    RoundRobin,
    LeastConnections,
    WeightedRoundRobin,
    IpHash,
}

/// Initialize load balancer
pub async fn init(config: &Config) -> Result<LoadBalancer> {
    LoadBalancer::new(config.clone())
}

impl LoadBalancer {
    /// Create a new load balancer
    pub fn new(config: Config) -> Result<Self> {
        let load_balancing_config = config.load_balancing.clone();
        
        Ok(Self {
            config: load_balancing_config,
            services: Arc::new(RwLock::new(HashMap::new())),
            stats: Arc::new(RwLock::new(LoadBalancerStats {
                total_requests: 0,
                successful_requests: 0,
                failed_requests: 0,
                average_response_time: 0.0,
                active_connections: 0,
                service_stats: HashMap::new(),
            })),
            current_index: Arc::new(RwLock::new(HashMap::new())),
        })
    }

    /// Add a service to the load balancer
    pub async fn add_service(&self, service_name: String, service: Service) -> Result<()> {
        debug!("Adding service: {} to load balancer", service_name);
        
        let mut services = self.services.write().await;
        services.entry(service_name.clone()).or_insert_with(Vec::new).push(service);
        
        info!("Service added to load balancer: {}", service_name);
        Ok(())
    }

    /// Remove a service from the load balancer
    pub async fn remove_service(&self, service_name: &str, service_id: &str) -> Result<()> {
        debug!("Removing service: {} with ID: {} from load balancer", service_name, service_id);
        
        let mut services = self.services.write().await;
        if let Some(service_list) = services.get_mut(service_name) {
            service_list.retain(|s| s.name != service_id);
            if service_list.is_empty() {
                services.remove(service_name);
            }
        }
        
        info!("Service removed from load balancer: {} - {}", service_name, service_id);
        Ok(())
    }

    /// Get the next available service for a request
    pub async fn get_next_service(&self, service_name: &str) -> Result<Option<Service>> {
        debug!("Getting next service for: {}", service_name);
        
        let services = self.services.read().await;
        let service_list = services.get(service_name);
        
        if let Some(service_list) = service_list {
            if service_list.is_empty() {
                warn!("No services available for: {}", service_name);
                return Ok(None);
            }
            
            // Filter healthy services
            let healthy_services: Vec<&Service> = service_list.iter()
                .filter(|s| s.status == ServiceStatus::Healthy)
                .collect();
            
            if healthy_services.is_empty() {
                warn!("No healthy services available for: {}", service_name);
                return Ok(None);
            }
            
            // Select service based on algorithm
            let selected_service = self.select_service(healthy_services, service_name).await?;
            
            debug!("Selected service: {} for request", selected_service.name);
            Ok(Some(selected_service.clone()))
        } else {
            warn!("Service not found: {}", service_name);
            Ok(None)
        }
    }

    /// Select a service based on the configured algorithm
    async fn select_service(&self, services: Vec<&Service>, service_name: &str) -> Result<Service> {
        let algorithm = self.parse_algorithm(&self.config.algorithm)?;
        
        match algorithm {
            LoadBalancingAlgorithm::RoundRobin => {
                self.round_robin_selection(services, service_name).await
            }
            LoadBalancingAlgorithm::LeastConnections => {
                self.least_connections_selection(services).await
            }
            LoadBalancingAlgorithm::WeightedRoundRobin => {
                self.weighted_round_robin_selection(services, service_name).await
            }
            LoadBalancingAlgorithm::IpHash => {
                self.ip_hash_selection(services).await
            }
        }
    }

    /// Round robin selection
    async fn round_robin_selection(&self, services: Vec<&Service>, service_name: &str) -> Result<Service> {
        let mut current_index = self.current_index.write().await;
        let index = current_index.entry(service_name.to_string()).or_insert(0);
        
        let selected_service = services[*index % services.len()].clone();
        *index = (*index + 1) % services.len();
        
        Ok(selected_service)
    }

    /// Least connections selection
    async fn least_connections_selection(&self, services: Vec<&Service>) -> Result<Service> {
        let mut least_connections = u32::MAX;
        let mut selected_service = services[0].clone();
        
        for service in services {
            // TODO: Get actual connection count from service
            let connection_count = 0; // Placeholder
            
            if connection_count < least_connections {
                least_connections = connection_count;
                selected_service = service.clone();
            }
        }
        
        Ok(selected_service)
    }

    /// Weighted round robin selection
    async fn weighted_round_robin_selection(&self, services: Vec<&Service>, service_name: &str) -> Result<Service> {
        // TODO: Implement weighted round robin
        // For now, fall back to round robin
        self.round_robin_selection(services, service_name).await
    }

    /// IP hash selection
    async fn ip_hash_selection(&self, services: Vec<&Service>) -> Result<Service> {
        // TODO: Implement IP hash selection
        // For now, select the first service
        Ok(services[0].clone())
    }

    /// Update service health status
    pub async fn update_service_health(&self, service_name: &str, service_id: &str, status: ServiceStatus) -> Result<()> {
        debug!("Updating service health: {} - {} to {:?}", service_name, service_id, status);
        
        let mut services = self.services.write().await;
        if let Some(service_list) = services.get_mut(service_name) {
            for service in service_list.iter_mut() {
                if service.name == service_id {
                    service.status = status;
                    service.last_health_check = Some(SystemTime::now());
                    break;
                }
            }
        }
        
        Ok(())
    }

    /// Update service response time
    pub async fn update_service_response_time(&self, service_name: &str, service_id: &str, response_time: u64) -> Result<()> {
        debug!("Updating service response time: {} - {} to {}ms", service_name, service_id, response_time);
        
        let mut services = self.services.write().await;
        if let Some(service_list) = services.get_mut(service_name) {
            for service in service_list.iter_mut() {
                if service.name == service_id {
                    service.response_time = Some(response_time);
                    break;
                }
            }
        }
        
        // Update statistics
        self.update_stats(service_name, true, response_time).await?;
        
        Ok(())
    }

    /// Record service error
    pub async fn record_service_error(&self, service_name: &str, service_id: &str) -> Result<()> {
        debug!("Recording service error: {} - {}", service_name, service_id);
        
        let mut services = self.services.write().await;
        if let Some(service_list) = services.get_mut(service_name) {
            for service in service_list.iter_mut() {
                if service.name == service_id {
                    service.error_count += 1;
                    break;
                }
            }
        }
        
        // Update statistics
        self.update_stats(service_name, false, 0).await?;
        
        Ok(())
    }

    /// Record service success
    pub async fn record_service_success(&self, service_name: &str, service_id: &str) -> Result<()> {
        debug!("Recording service success: {} - {}", service_name, service_id);
        
        let mut services = self.services.write().await;
        if let Some(service_list) = services.get_mut(service_name) {
            for service in service_list.iter_mut() {
                if service.name == service_id {
                    service.success_count += 1;
                    break;
                }
            }
        }
        
        Ok(())
    }

    /// Update load balancer statistics
    async fn update_stats(&self, service_name: &str, success: bool, response_time: u64) -> Result<()> {
        let mut stats = self.stats.write().await;
        
        stats.total_requests += 1;
        if success {
            stats.successful_requests += 1;
        } else {
            stats.failed_requests += 1;
        }
        
        // Update average response time
        if response_time > 0 {
            let total_time = stats.average_response_time * (stats.total_requests - 1) as f64 + response_time as f64;
            stats.average_response_time = total_time / stats.total_requests as f64;
        }
        
        // Update service-specific stats
        let service_stats = stats.service_stats.entry(service_name.to_string()).or_insert(ServiceStats {
            requests: 0,
            successful_requests: 0,
            failed_requests: 0,
            average_response_time: 0.0,
            error_rate: 0.0,
            last_request: None,
        });
        
        service_stats.requests += 1;
        if success {
            service_stats.successful_requests += 1;
        } else {
            service_stats.failed_requests += 1;
        }
        
        if response_time > 0 {
            let total_time = service_stats.average_response_time * (service_stats.requests - 1) as f64 + response_time as f64;
            service_stats.average_response_time = total_time / service_stats.requests as f64;
        }
        
        service_stats.error_rate = service_stats.failed_requests as f64 / service_stats.requests as f64;
        service_stats.last_request = Some(SystemTime::now());
        
        Ok(())
    }

    /// Get load balancer statistics
    pub async fn get_stats(&self) -> Result<LoadBalancerStats> {
        let stats = self.stats.read().await;
        Ok(stats.clone())
    }

    /// Get service statistics
    pub async fn get_service_stats(&self, service_name: &str) -> Result<Option<ServiceStats>> {
        let stats = self.stats.read().await;
        Ok(stats.service_stats.get(service_name).cloned())
    }

    /// Get all services
    pub async fn get_services(&self) -> Result<HashMap<String, Vec<Service>>> {
        let services = self.services.read().await;
        Ok(services.clone())
    }

    /// Get services for a specific service name
    pub async fn get_services_for_name(&self, service_name: &str) -> Result<Vec<Service>> {
        let services = self.services.read().await;
        Ok(services.get(service_name).cloned().unwrap_or_default())
    }

    /// Check if service is available
    pub async fn is_service_available(&self, service_name: &str) -> Result<bool> {
        let services = self.services.read().await;
        if let Some(service_list) = services.get(service_name) {
            Ok(service_list.iter().any(|s| s.status == ServiceStatus::Healthy))
        } else {
            Ok(false)
        }
    }

    /// Parse load balancing algorithm from string
    fn parse_algorithm(&self, algorithm: &str) -> Result<LoadBalancingAlgorithm> {
        match algorithm.to_lowercase().as_str() {
            "round_robin" => Ok(LoadBalancingAlgorithm::RoundRobin),
            "least_connections" => Ok(LoadBalancingAlgorithm::LeastConnections),
            "weighted_round_robin" => Ok(LoadBalancingAlgorithm::WeightedRoundRobin),
            "ip_hash" => Ok(LoadBalancingAlgorithm::IpHash),
            _ => Err(ApiGatewayError::LoadBalancing(format!("Unknown load balancing algorithm: {}", algorithm))),
        }
    }

    /// Start health checking
    pub async fn start_health_checking(&self) -> Result<()> {
        if !self.config.health_check.enabled {
            return Ok(());
        }
        
        info!("Starting health checking for load balancer");
        
        // TODO: Implement health checking
        // This is a placeholder implementation
        
        Ok(())
    }

    /// Stop health checking
    pub async fn stop_health_checking(&self) -> Result<()> {
        info!("Stopping health checking for load balancer");
        
        // TODO: Implement health checking stop
        // This is a placeholder implementation
        
        Ok(())
    }
}


