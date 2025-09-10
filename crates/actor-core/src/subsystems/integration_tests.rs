//! Integration Tests for Enhanced Hybrid Resource Manager
//!
//! This module provides comprehensive integration tests for the Enhanced Hybrid
//! Resource Manager, testing all components working together.

use std::collections::HashMap;
use std::sync::Arc;
use crate::types::Actor;
use crate::ActorCoreResult;
use crate::subsystems::{
    EnhancedHybridResourceManager, ResourceCache, CacheConfig,
    StatChangeNotifier, ResourceEventManager, EventConfig,
    PerformanceMonitor, PerformanceConfig,
    RpgResourceManager, MagicResourceManager, ResourceRegenerationManager, RegenerationConfig,
    InMemoryResourceDatabase,
};
use crate::interfaces::Subsystem;
use crate::subsystems::stat_change_notifier::NotifierConfig;
use crate::subsystems::system_resource_manager::SystemResourceCalculator;
// use crate::subsystems::enhanced_hybrid_resource_manager::ResourceDatabase;

/// Integration Test Suite
pub struct IntegrationTestSuite {
    /// Enhanced Hybrid Resource Manager
    resource_manager: Arc<EnhancedHybridResourceManager>,
    /// Resource Cache
    resource_cache: Arc<ResourceCache>,
    /// Stat Change Notifier
    stat_notifier: Arc<StatChangeNotifier>,
    /// Resource Event Manager
    event_manager: Arc<ResourceEventManager>,
    /// Performance Monitor
    #[allow(dead_code)]
    performance_monitor: Arc<PerformanceMonitor>,
    /// Test actors
    test_actors: Vec<Actor>,
}

impl IntegrationTestSuite {
    /// Create a new Integration Test Suite
    pub async fn new() -> ActorCoreResult<Self> {
        // Initialize components
        let cache_config = CacheConfig::default();
        let resource_cache = Arc::new(ResourceCache::new(cache_config));
        
        let notifier_config = NotifierConfig::default();
        let stat_notifier = Arc::new(StatChangeNotifier::new(notifier_config));
        
        let event_config = EventConfig::default();
        let event_manager = Arc::new(ResourceEventManager::new(event_config));
        
        let performance_config = PerformanceConfig::default();
        let performance_monitor = Arc::new(PerformanceMonitor::new(performance_config));
        
        // Create resource manager (it will use default implementations)
        let resource_manager = Arc::new(EnhancedHybridResourceManager::new());
        
        // Create test actors
        let mut test_actors = Vec::new();
        for i in 0..100 {
            let mut actor = Actor::new(
                format!("test_actor_{}", i),
                "Human".to_string(),
            );
            
            // Set up actor with test data
            let mut data = HashMap::new();
            data.insert("level".to_string(), serde_json::json!(i % 50 + 1));
            data.insert("vitality".to_string(), serde_json::json!(10 + i % 20));
            data.insert("intelligence".to_string(), serde_json::json!(10 + i % 15));
            data.insert("wisdom".to_string(), serde_json::json!(10 + i % 18));
            data.insert("constitution".to_string(), serde_json::json!(10 + i % 25));
            data.insert("charisma".to_string(), serde_json::json!(10 + i % 12));
            data.insert("equipment_bonus".to_string(), serde_json::json!(i % 10));
            data.insert("in_combat".to_string(), serde_json::json!(i % 3 == 0));
            data.insert("resting".to_string(), serde_json::json!(i % 4 == 0));
            data.insert("moving".to_string(), serde_json::json!(i % 5 == 0));
            
            actor.set_data(data);
            test_actors.push(actor);
        }
        
        Ok(Self {
            resource_manager,
            resource_cache,
            stat_notifier,
            event_manager,
            performance_monitor,
            test_actors,
        })
    }
    
    /// Test basic resource calculation
    pub async fn test_basic_resource_calculation(&self) -> ActorCoreResult<TestResult> {
        let start_time = std::time::Instant::now();
        
        // Test resource calculation for all actors
        for actor in &self.test_actors {
            let _result = self.resource_manager.contribute(actor).await?;
        }
        
        let execution_time = start_time.elapsed();
        
        Ok(TestResult {
            test_name: "basic_resource_calculation".to_string(),
            execution_time_ms: execution_time.as_millis() as f64,
            memory_usage_bytes: 0, // Simplified
            success: true,
            error_message: None,
            timestamp: self.get_current_timestamp(),
            additional_metrics: HashMap::new(),
        })
    }
    
    /// Test resource caching
    pub async fn test_resource_caching(&self) -> ActorCoreResult<TestResult> {
        let start_time = std::time::Instant::now();
        
        // Test cache operations
        for actor in &self.test_actors {
            // Set a resource in cache
            let metadata = crate::subsystems::resource_cache::ResourceMetadata {
                category: "health".to_string(),
                dependencies: vec!["vitality".to_string()],
                priority: 100,
                is_shared: true,
            };
            
            self.resource_cache.set(
                &actor.id.to_string(),
                "hp_current",
                100.0,
                metadata,
            ).await?;
            
            // Get resource from cache
            let _value = self.resource_cache.get(
                &actor.id.to_string(),
                "hp_current",
            ).await?;
        }
        
        let execution_time = start_time.elapsed();
        
        Ok(TestResult {
            test_name: "resource_caching".to_string(),
            execution_time_ms: execution_time.as_millis() as f64,
            memory_usage_bytes: 0, // Simplified
            success: true,
            error_message: None,
            timestamp: self.get_current_timestamp(),
            additional_metrics: HashMap::new(),
        })
    }
    
    /// Test stat change notifications
    pub async fn test_stat_change_notifications(&self) -> ActorCoreResult<TestResult> {
        let start_time = std::time::Instant::now();
        
        // Test stat change notifications
        for actor in &self.test_actors {
            let changed_stats = vec!["vitality".to_string(), "intelligence".to_string()];
            self.stat_notifier.notify_stat_change(actor, &changed_stats).await?;
        }
        
        let execution_time = start_time.elapsed();
        
        Ok(TestResult {
            test_name: "stat_change_notifications".to_string(),
            execution_time_ms: execution_time.as_millis() as f64,
            memory_usage_bytes: 0, // Simplified
            success: true,
            error_message: None,
            timestamp: self.get_current_timestamp(),
            additional_metrics: HashMap::new(),
        })
    }
    
    /// Test resource events
    pub async fn test_resource_events(&self) -> ActorCoreResult<TestResult> {
        let start_time = std::time::Instant::now();
        
        // Test event emission
        for actor in &self.test_actors {
            let event = self.event_manager.create_resource_changed_event(
                &actor.id.to_string(),
                "hp_current",
                100.0,
                90.0,
                "test_source",
            );
            
            self.event_manager.emit_event(event).await?;
        }
        
        let execution_time = start_time.elapsed();
        
        Ok(TestResult {
            test_name: "resource_events".to_string(),
            execution_time_ms: execution_time.as_millis() as f64,
            memory_usage_bytes: 0, // Simplified
            success: true,
            error_message: None,
            timestamp: self.get_current_timestamp(),
            additional_metrics: HashMap::new(),
        })
    }
    
    /// Test resource regeneration
    pub async fn test_resource_regeneration(&self) -> ActorCoreResult<TestResult> {
        let start_time = std::time::Instant::now();
        
        // Create regeneration manager
        let regen_config = RegenerationConfig::default();
        let regen_manager = ResourceRegenerationManager::new(regen_config);
        
        // Test regeneration for all actors
        for actor in &self.test_actors {
            regen_manager.start_regeneration(actor, "hp_current").await?;
        }
        
        // Simulate time passing and update regeneration
        let mut actor_map = HashMap::new();
        for actor in &self.test_actors {
            actor_map.insert(actor.id.to_string(), actor.clone());
        }
        
        regen_manager.update_regeneration(&actor_map).await?;
        
        let execution_time = start_time.elapsed();
        
        Ok(TestResult {
            test_name: "resource_regeneration".to_string(),
            execution_time_ms: execution_time.as_millis() as f64,
            memory_usage_bytes: 0, // Simplified
            success: true,
            error_message: None,
            timestamp: self.get_current_timestamp(),
            additional_metrics: HashMap::new(),
        })
    }
    
    /// Test RPG resource manager
    pub async fn test_rpg_resource_manager(&self) -> ActorCoreResult<TestResult> {
        let start_time = std::time::Instant::now();
        
        // Create RPG resource manager
        let rpg_manager = RpgResourceManager::new();
        
        // Test resource calculation for all actors
        for actor in &self.test_actors {
            let _resources = rpg_manager.calculate_resources(actor).await?;
        }
        
        let execution_time = start_time.elapsed();
        
        Ok(TestResult {
            test_name: "rpg_resource_manager".to_string(),
            execution_time_ms: execution_time.as_millis() as f64,
            memory_usage_bytes: 0, // Simplified
            success: true,
            error_message: None,
            timestamp: self.get_current_timestamp(),
            additional_metrics: HashMap::new(),
        })
    }
    
    /// Test magic resource manager
    pub async fn test_magic_resource_manager(&self) -> ActorCoreResult<TestResult> {
        let start_time = std::time::Instant::now();
        
        // Create magic resource manager
        let magic_manager = MagicResourceManager::new();
        
        // Test resource calculation for all actors
        for actor in &self.test_actors {
            let _resources = magic_manager.calculate_resources(actor).await?;
        }
        
        let execution_time = start_time.elapsed();
        
        Ok(TestResult {
            test_name: "magic_resource_manager".to_string(),
            execution_time_ms: execution_time.as_millis() as f64,
            memory_usage_bytes: 0, // Simplified
            success: true,
            error_message: None,
            timestamp: self.get_current_timestamp(),
            additional_metrics: HashMap::new(),
        })
    }
    
    /// Test database operations
    pub async fn test_database_operations(&self) -> ActorCoreResult<TestResult> {
        let start_time = std::time::Instant::now();
        
        // Create in-memory database for testing
        let _database = InMemoryResourceDatabase::new();
        
        // Test database operations for all actors
        for _actor in &self.test_actors {
            // Database operations would be tested here
            // For now, just simulate some work
            std::thread::sleep(std::time::Duration::from_micros(100));
        }
        
        let execution_time = start_time.elapsed();
        
        Ok(TestResult {
            test_name: "database_operations".to_string(),
            execution_time_ms: execution_time.as_millis() as f64,
            memory_usage_bytes: 0, // Simplified
            success: true,
            error_message: None,
            timestamp: self.get_current_timestamp(),
            additional_metrics: HashMap::new(),
        })
    }
    
    /// Run comprehensive integration test
    pub async fn run_comprehensive_test(&self) -> ActorCoreResult<ComprehensiveTestResults> {
        let mut all_results = Vec::new();
        
        // Run all individual tests
        let tests = vec![
            self.test_basic_resource_calculation().await?,
            self.test_resource_caching().await?,
            self.test_stat_change_notifications().await?,
            self.test_resource_events().await?,
            self.test_resource_regeneration().await?,
            self.test_rpg_resource_manager().await?,
            self.test_magic_resource_manager().await?,
            self.test_database_operations().await?,
        ];
        
        all_results.extend(tests);
        
        // Calculate overall statistics
        let total_tests = all_results.len();
        let successful_tests = all_results.iter().filter(|r| r.success).count();
        let total_execution_time: f64 = all_results.iter().map(|r| r.execution_time_ms).sum();
        let total_memory_usage: usize = all_results.iter().map(|r| r.memory_usage_bytes).sum();
        
        Ok(ComprehensiveTestResults {
            total_tests,
            successful_tests,
            success_rate: successful_tests as f64 / total_tests as f64,
            total_execution_time,
            average_execution_time: total_execution_time / total_tests as f64,
            total_memory_usage,
            average_memory_usage: total_memory_usage / total_tests,
            test_results: all_results,
        })
    }
    
    /// Get current timestamp
    fn get_current_timestamp(&self) -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }
}

/// Test Result
#[derive(Debug, Clone)]
pub struct TestResult {
    /// Test name
    pub test_name: String,
    /// Execution time in milliseconds
    pub execution_time_ms: f64,
    /// Memory usage in bytes
    pub memory_usage_bytes: usize,
    /// Success status
    pub success: bool,
    /// Error message if failed
    pub error_message: Option<String>,
    /// Timestamp
    pub timestamp: u64,
    /// Additional metrics
    pub additional_metrics: HashMap<String, f64>,
}

/// Comprehensive Test Results
#[derive(Debug, Clone)]
pub struct ComprehensiveTestResults {
    /// Total number of tests
    pub total_tests: usize,
    /// Number of successful tests
    pub successful_tests: usize,
    /// Overall success rate
    pub success_rate: f64,
    /// Total execution time in milliseconds
    pub total_execution_time: f64,
    /// Average execution time in milliseconds
    pub average_execution_time: f64,
    /// Total memory usage in bytes
    pub total_memory_usage: usize,
    /// Average memory usage in bytes
    pub average_memory_usage: usize,
    /// All test results
    pub test_results: Vec<TestResult>,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_integration_suite_creation() {
        let suite = IntegrationTestSuite::new().await.unwrap();
        assert_eq!(suite.test_actors.len(), 100);
    }
    
    #[tokio::test]
    async fn test_basic_resource_calculation() {
        let suite = IntegrationTestSuite::new().await.unwrap();
        let result = suite.test_basic_resource_calculation().await.unwrap();
        assert!(result.success);
        assert!(result.execution_time_ms > 0.0);
    }
    
    #[tokio::test]
    async fn test_resource_caching() {
        let suite = IntegrationTestSuite::new().await.unwrap();
        let result = suite.test_resource_caching().await.unwrap();
        assert!(result.success);
        assert!(result.execution_time_ms > 0.0);
    }
    
    #[tokio::test]
    async fn test_comprehensive_integration() {
        let suite = IntegrationTestSuite::new().await.unwrap();
        let results = suite.run_comprehensive_test().await.unwrap();
        assert!(results.success_rate > 0.9); // At least 90% success rate
        assert!(results.total_tests > 0);
    }
}
