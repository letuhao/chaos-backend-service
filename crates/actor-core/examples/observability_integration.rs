//! Examples demonstrating how to integrate the observability system.
//!
//! This module shows how to use the standardized tracing and metrics
//! across different actor-core components.

use std::time::Instant;
use actor_core::prelude::*;

/// Example: Integrating observability into a cache operation.
pub struct ExampleCache {
    manager: ObservabilityManager,
    component_name: String,
}

impl ExampleCache {
    pub fn new() -> Self {
        let config = ObservabilityConfig::default();
        Self {
            manager: ObservabilityManager::new(config),
            component_name: "example_cache".to_string(),
        }
    }

    /// Example cache get operation with observability.
    pub fn get(&self, key: &str) -> Option<String> {
        let start_time = Instant::now();
        
        // Simulate cache operation
        let hit = key.len() > 5; // Simple hit/miss logic
        let result = if hit { Some("cached_value".to_string()) } else { None };
        
        let duration_us = start_time.elapsed().as_micros() as u64;
        
        // Record the operation with standardized metrics and tracing
        self.manager.record_component_cache_operation(
            &self.component_name,
            "get",
            hit,
            duration_us
        );
        
        // Use standardized tracing macro
        crate::trace_cache_operation!(
            &self.manager,
            &self.component_name,
            "get",
            hit,
            duration_us,
            key = key,
            cache_layer = "l1"
        );
        
        result
    }

    /// Example cache set operation with observability.
    pub fn set(&self, key: &str, value: &str) -> Result<(), String> {
        let start_time = Instant::now();
        
        // Simulate cache operation
        if key.is_empty() {
            let duration_us = start_time.elapsed().as_micros() as u64;
            self.manager.record_component_error(&self.component_name, "empty_key");
            self.manager.record_component_operation(&self.component_name, "get_error", duration_us);
            
            crate::trace_error!(
                &self.manager,
                &self.component_name,
                "empty_key",
                "Key cannot be empty",
                key = key
            );
            
            return Err("Key cannot be empty".to_string());
        }
        
        let duration_us = start_time.elapsed().as_micros() as u64;
        
        // Record the operation
        self.manager.record_component_cache_operation(
            &self.component_name,
            "set",
            true, // set operations are always "successful"
            duration_us
        );
        
        crate::trace_cache_operation!(
            &self.manager,
            &self.component_name,
            "set",
            true,
            duration_us,
            key = key,
            value_length = value.len(),
            cache_layer = "l1"
        );
        
        Ok(())
    }

    /// Get metrics for this cache component.
    pub fn get_metrics(&self) -> Option<StandardMetrics> {
        self.manager.get_component_metrics(&self.component_name)
    }
}

/// Example: Integrating observability into a subsystem.
pub struct ExampleSubsystem {
    manager: ObservabilityManager,
    subsystem_id: String,
}

impl ExampleSubsystem {
    pub fn new(subsystem_id: String) -> Self {
        let config = ObservabilityConfig::default();
        Self {
            manager: ObservabilityManager::new(config),
            subsystem_id,
        }
    }

    /// Example subsystem contribution with observability.
    pub async fn contribute(&self, actor_id: &str) -> Result<Vec<f64>, String> {
        let start_time = Instant::now();
        
        // Simulate subsystem processing
        let success = actor_id.len() > 3;
        let contributions = if success {
            vec![10.0, 20.0, 30.0]
        } else {
            vec![]
        };
        
        let duration_us = start_time.elapsed().as_micros() as u64;
        
        // Record the subsystem execution
        self.manager.record_subsystem_execution(
            &self.subsystem_id,
            success,
            duration_us
        );
        
        if success {
            crate::trace_with_timing!(
                &self.manager,
                &self.subsystem_id,
                "contribute",
                duration_us,
                actor_id = actor_id,
                contribution_count = contributions.len()
            );
        } else {
            crate::trace_error!(
                &self.manager,
                &self.subsystem_id,
                "invalid_actor_id",
                "Actor ID too short",
                actor_id = actor_id
            );
        }
        
        if success {
            Ok(contributions)
        } else {
            Err("Invalid actor ID".to_string())
        }
    }

    /// Get metrics for this subsystem.
    pub fn get_metrics(&self) -> Option<StandardMetrics> {
        self.manager.get_component_metrics(&self.subsystem_id)
    }
}

/// Example: Demonstrating structured logging with consistent fields.
pub fn demonstrate_structured_logging() {
    use tracing::{info, warn, error};
    
    // Example of standardized structured logging
    let actor_id = "player_123";
    let dimension = "strength";
    let operation = "aggregate";
    let duration_us = 1500;
    let subsystem_count = 5;
    
    // Use consistent field names from tracing_fields
    info!(
        actor_id = actor_id,
        dimension = dimension,
        operation = operation,
        duration_us = duration_us,
        subsystem_count = subsystem_count,
        "Actor stat aggregation completed"
    );
    
    // Example warning with structured fields
    warn!(
        actor_id = actor_id,
        dimension = dimension,
        processing_time_us = duration_us,
        "Aggregation took longer than expected"
    );
    
    // Example error with structured fields
    error!(
        actor_id = actor_id,
        dimension = dimension,
        error = "Invalid contribution value",
        "Failed to process contribution"
    );
}

/// Example: Demonstrating metrics collection.
pub fn demonstrate_metrics_collection() {
    let config = ObservabilityConfig::default();
    let manager = ObservabilityManager::new(config);
    
    // Simulate some operations
    for i in 0..10 {
        let start_time = Instant::now();
        
        // Simulate some work
        std::thread::sleep(std::time::Duration::from_millis(1));
        
        let duration_us = start_time.elapsed().as_micros() as u64;
        
        manager.record_component_operation("test_component", "test_operation", duration_us);
        
        if i % 3 == 0 {
            manager.record_component_error("test_component", "test_error");
        }
    }
    
    // Get and display metrics
    let global_metrics = manager.get_global_metrics();
    println!("Global metrics:");
    println!("  Total operations: {}", global_metrics.total_operations());
    println!("  Total errors: {}", global_metrics.total_errors());
    println!("  Error rate: {:.2}%", global_metrics.error_rate() * 100.0);
    
    let component_metrics = manager.get_component_metrics("test_component");
    if let Some(metrics) = component_metrics {
        println!("Component metrics:");
        println!("  Operations: {:?}", metrics.operations);
        println!("  Errors: {:?}", metrics.errors);
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🎮 Actor Core - Observability Integration Example");
    println!("================================================");

    // Example 1: Cache Operations with Observability
    println!("\n1. Cache Operations with Observability:");
    let cache = ExampleCache::new();
    
    // Test cache operations
    let result = cache.get("test_key");
    println!("Cache get result: {:?}", result);
    
    let set_result = cache.set("test_key", "test_value");
    println!("Cache set result: {:?}", set_result);
    
    // Test error case
    let error_result = cache.set("", "test_value");
    println!("Cache set error: {:?}", error_result);
    
    // Display metrics
    if let Some(metrics) = cache.get_metrics() {
        println!("Cache metrics:");
        println!("  Cache hits: {}", metrics.cache_stats.hits);
        println!("  Cache misses: {}", metrics.cache_stats.misses);
        println!("  Errors: {:?}", metrics.errors);
    }

    // Example 2: Subsystem Operations with Observability
    println!("\n2. Subsystem Operations with Observability:");
    let subsystem = ExampleSubsystem::new("example_subsystem".to_string());
    
    // Test successful contribution
    let result = subsystem.contribute("valid_actor").await?;
    println!("Subsystem contribution result: {:?}", result);
    
    // Test failed contribution
    let error_result = subsystem.contribute("abc").await;
    println!("Subsystem contribution error: {:?}", error_result);
    
    // Display metrics
    if let Some(metrics) = subsystem.get_metrics() {
        println!("Subsystem metrics:");
        println!("  Total executions: {}", metrics.subsystem_stats.total_executions);
        println!("  Successful executions: {}", metrics.subsystem_stats.successful_executions);
        println!("  Failed executions: {}", metrics.subsystem_stats.failed_executions);
    }

    // Example 3: Structured Logging
    println!("\n3. Structured Logging Demonstration:");
    demonstrate_structured_logging();

    // Example 4: Metrics Collection
    println!("\n4. Metrics Collection Demonstration:");
    demonstrate_metrics_collection();

    println!("\n🎉 Observability integration example completed successfully!");
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_cache_operations() {
        let cache = ExampleCache::new();
        
        // Test cache get
        let result = cache.get("test_key");
        assert!(result.is_some());
        
        // Test cache set
        let set_result = cache.set("test_key", "test_value");
        assert!(set_result.is_ok());
        
        // Test error case
        let error_result = cache.set("", "test_value");
        assert!(error_result.is_err());
        
        // Check metrics
        let metrics = cache.get_metrics();
        assert!(metrics.is_some());
        let metrics = metrics.unwrap();
        assert!(metrics.cache_stats.hits > 0);
        assert!(metrics.errors.len() > 0);
    }

    #[tokio::test]
    async fn test_example_subsystem_operations() {
        let subsystem = ExampleSubsystem::new("test_subsystem".to_string());
        
        // Test successful contribution
        let result = subsystem.contribute("valid_actor").await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 3);
        
        // Test failed contribution
        let error_result = subsystem.contribute("abc").await;
        assert!(error_result.is_err());
        
        // Check metrics
        let metrics = subsystem.get_metrics();
        assert!(metrics.is_some());
        let metrics = metrics.unwrap();
        assert_eq!(metrics.subsystem_stats.total_executions, 2);
        assert_eq!(metrics.subsystem_stats.successful_executions, 1);
        assert_eq!(metrics.subsystem_stats.failed_executions, 1);
    }

    #[test]
    fn test_metrics_collection() {
        demonstrate_metrics_collection();
    }
}
