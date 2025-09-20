//! Integration tests for the Actor Core system.
//!
//! This module contains end-to-end tests that verify the complete
//! stat aggregation pipeline works correctly.

use std::collections::HashMap;
use std::sync::Arc;
use tokio;

use crate::{
    ActorCore, ActorCoreBuilder, ActorCoreResult,
    config::{
        manager::ConfigurationManager,
        providers::{
            DefaultConfigProvider, EnvironmentConfigurationProvider,
            FileConfigurationProvider, ExampleConfigurationProvider,
        },
    },
    registry::RegistryManager,
    aggregator::Aggregator,
    types::*,
    enums::*,
};

/// Integration test configuration
#[derive(Debug, Clone)]
pub struct IntegrationTestConfig {
    /// Test timeout in seconds
    pub timeout_seconds: u64,
    /// Number of test iterations
    pub iterations: usize,
    /// Enable performance testing
    pub enable_performance_tests: bool,
    /// Test data directory
    pub test_data_dir: String,
    /// Enable verbose logging
    pub verbose_logging: bool,
}

impl Default for IntegrationTestConfig {
    fn default() -> Self {
        Self::load_config().unwrap_or_else(|_| {
            tracing::warn!("Failed to load integration test config, using hardcoded defaults");
            Self {
                timeout_seconds: 30,
                iterations: 100,
                enable_performance_tests: true,
                test_data_dir: "test_data".to_string(),
                verbose_logging: false,
            }
        })
    }
}

impl IntegrationTestConfig {
    /// Load integration test configuration from config file
    pub fn load_config() -> ActorCoreResult<Self> {
        // Try to load from integration_test_config.yaml first
        let config_path = std::path::Path::new("configs/integration_test_config.yaml");
            
        if config_path.exists() {
            match Self::load_config_from_file(config_path) {
                Ok(config) => return Ok(config),
                Err(e) => {
                    tracing::warn!("Failed to load integration test config from file: {}. Using hardcoded defaults.", e);
                }
            }
        }
        
        // Fallback to hardcoded defaults
        Ok(Self {
            timeout_seconds: 30,
            iterations: 100,
            enable_performance_tests: true,
            test_data_dir: "test_data".to_string(),
            verbose_logging: false,
        })
    }

    /// Load integration test configuration from file
    fn load_config_from_file(path: &std::path::Path) -> ActorCoreResult<Self> {
        let content = std::fs::read_to_string(path)?;
        let config: IntegrationTestConfig = serde_yaml::from_str(&content)?;
        Ok(config)
    }
}

/// Integration test suite for Actor Core
pub struct IntegrationTestSuite {
    config: IntegrationTestConfig,
    actor_core: Option<Arc<ActorCore>>,
}

impl IntegrationTestSuite {
    /// Create a new integration test suite
    pub fn new() -> Self {
        Self {
            config: IntegrationTestConfig::default(),
            actor_core: None,
        }
    }

    /// Create a new integration test suite with custom configuration
    pub fn with_config(config: IntegrationTestConfig) -> Self {
        Self {
            config,
            actor_core: None,
        }
    }

    /// Initialize the test suite
    pub async fn initialize(&mut self) -> ActorCoreResult<()> {
        tracing::info!("Initializing integration test suite");

        // Create ActorCore instance
        let actor_core = ActorCoreBuilder::new()
            .with_config_manager(Arc::new(ConfigurationManager::new()))
            .with_registry_manager(Arc::new(RegistryManager::new()))
            .build()
            .await?;

        self.actor_core = Some(Arc::new(actor_core));
        Ok(())
    }

    /// Run all integration tests
    pub async fn run_all_tests(&self) -> ActorCoreResult<TestResults> {
        let mut results = TestResults::new();

        // Test basic functionality
        if let Err(e) = self.test_basic_functionality().await {
            results.add_failure("basic_functionality", e);
        } else {
            results.add_success("basic_functionality");
        }

        // Test configuration loading
        if let Err(e) = self.test_configuration_loading().await {
            results.add_failure("configuration_loading", e);
        } else {
            results.add_success("configuration_loading");
        }

        // Test registry operations
        if let Err(e) = self.test_registry_operations().await {
            results.add_failure("registry_operations", e);
        } else {
            results.add_success("registry_operations");
        }

        // Test aggregator functionality
        if let Err(e) = self.test_aggregator_functionality().await {
            results.add_failure("aggregator_functionality", e);
        } else {
            results.add_success("aggregator_functionality");
        }

        // Performance tests (if enabled)
        if self.config.enable_performance_tests {
            if let Err(e) = self.test_performance().await {
                results.add_failure("performance", e);
            } else {
                results.add_success("performance");
            }
        }

        Ok(results)
    }

    /// Test basic functionality
    async fn test_basic_functionality(&self) -> ActorCoreResult<()> {
        tracing::info!("Running basic functionality test");

        let actor_core = self.actor_core.as_ref()
            .ok_or_else(|| crate::ActorCoreError::InvalidInput("ActorCore not initialized".to_string()))?;

        // Test basic operations
        let actor = Actor::new("test_actor".to_string());
        let snapshot = actor_core.aggregator.resolve(&actor).await?;

        // Verify snapshot is created
        if snapshot.is_empty() {
            return Err(crate::ActorCoreError::InvalidInput("Empty snapshot created".to_string()));
        }

        tracing::info!("Basic functionality test passed");
        Ok(())
    }

    /// Test configuration loading
    async fn test_configuration_loading(&self) -> ActorCoreResult<()> {
        tracing::info!("Running configuration loading test");

        // Test loading various configuration providers
        let default_provider = DefaultConfigProvider::new("test_default".to_string(), 100, "configs/default_config.yaml".into())?;
        let env_provider = EnvironmentConfigurationProvider::new("test_env".to_string(), 200, "TEST_ACTOR_CORE".to_string());
        let file_provider = FileConfigurationProvider::new("test_file".to_string(), 300, "configs/test_config.yaml".into());

        // Verify providers are created successfully
        assert_eq!(default_provider.provider_id(), "test_default");
        assert_eq!(env_provider.provider_id(), "test_env");
        assert_eq!(file_provider.provider_id(), "test_file");

        tracing::info!("Configuration loading test passed");
        Ok(())
    }

    /// Test registry operations
    async fn test_registry_operations(&self) -> ActorCoreResult<()> {
        tracing::info!("Running registry operations test");

        let actor_core = self.actor_core.as_ref()
            .ok_or_else(|| crate::ActorCoreError::InvalidInput("ActorCore not initialized".to_string()))?;

        // Test resource registration
        let resource = ResourceDefinition {
            id: "test_resource".to_string(),
            name: "Test Resource".to_string(),
            description: "A test resource".to_string(),
            resource_type: "test".to_string(),
            properties: HashMap::new(),
        };

        actor_core.registry_manager.register_resource(resource.clone()).await?;

        // Verify resource is registered
        let retrieved_resource = actor_core.registry_manager.get_resource("test_resource").await?;
        assert_eq!(retrieved_resource.id, resource.id);

        tracing::info!("Registry operations test passed");
        Ok(())
    }

    /// Test aggregator functionality
    async fn test_aggregator_functionality(&self) -> ActorCoreResult<()> {
        tracing::info!("Running aggregator functionality test");

        let actor_core = self.actor_core.as_ref()
            .ok_or_else(|| crate::ActorCoreError::InvalidInput("ActorCore not initialized".to_string()))?;

        // Create test actor with contributions
        let mut actor = Actor::new("test_actor".to_string());
        
        // Add some test contributions
        let contribution = Contribution {
            id: "test_contribution".to_string(),
            actor_id: "test_actor".to_string(),
            subsystem_id: "test_subsystem".to_string(),
            resource_id: "test_resource".to_string(),
            value: 100.0,
            bucket: Bucket::Flat,
            cap_mode: CapMode::Baseline,
            timestamp: chrono::Utc::now(),
        };

        actor.add_contribution(contribution);

        // Test aggregation
        let snapshot = actor_core.aggregator.resolve(&actor).await?;

        // Verify snapshot contains expected data
        if snapshot.is_empty() {
            return Err(crate::ActorCoreError::InvalidInput("Empty snapshot created".to_string()));
        }

        tracing::info!("Aggregator functionality test passed");
        Ok(())
    }

    /// Test performance
    async fn test_performance(&self) -> ActorCoreResult<()> {
        tracing::info!("Running performance test");

        let actor_core = self.actor_core.as_ref()
            .ok_or_else(|| crate::ActorCoreError::InvalidInput("ActorCore not initialized".to_string()))?;

        let start_time = std::time::Instant::now();

        // Run performance test iterations
        for i in 0..self.config.iterations {
            let actor = Actor::new(format!("test_actor_{}", i));
            let _snapshot = actor_core.aggregator.resolve(&actor).await?;
        }

        let duration = start_time.elapsed();
        let avg_time = duration.as_millis() as f64 / self.config.iterations as f64;

        tracing::info!("Performance test completed: {} iterations in {:?}, avg: {:.2}ms", 
                      self.config.iterations, duration, avg_time);

        // Verify performance is within acceptable limits
        if avg_time > 100.0 { // 100ms per operation
            return Err(crate::ActorCoreError::InvalidInput(
                format!("Performance test failed: average time {:.2}ms exceeds limit", avg_time)
            ));
        }

        Ok(())
    }
}

/// Test results container
#[derive(Debug, Clone)]
pub struct TestResults {
    pub passed: Vec<String>,
    pub failed: Vec<(String, String)>,
    pub total_tests: usize,
    pub success_rate: f64,
}

impl TestResults {
    pub fn new() -> Self {
        Self {
            passed: Vec::new(),
            failed: Vec::new(),
            total_tests: 0,
            success_rate: 0.0,
        }
    }

    pub fn add_success(&mut self, test_name: &str) {
        self.passed.push(test_name.to_string());
        self.update_stats();
    }

    pub fn add_failure(&mut self, test_name: &str, error: crate::ActorCoreError) {
        self.failed.push((test_name.to_string(), error.to_string()));
        self.update_stats();
    }

    fn update_stats(&mut self) {
        self.total_tests = self.passed.len() + self.failed.len();
        if self.total_tests > 0 {
            self.success_rate = self.passed.len() as f64 / self.total_tests as f64;
        }
    }

    pub fn is_successful(&self) -> bool {
        self.failed.is_empty()
    }
}

/// Run integration tests
pub async fn run_integration_tests() -> ActorCoreResult<TestResults> {
    let mut test_suite = IntegrationTestSuite::new();
    test_suite.initialize().await?;
    test_suite.run_all_tests().await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_integration_test_suite_creation() {
        let test_suite = IntegrationTestSuite::new();
        assert_eq!(test_suite.config.timeout_seconds, 30);
        assert_eq!(test_suite.config.iterations, 100);
    }

    #[tokio::test]
    async fn test_integration_test_config_loading() {
        let config = IntegrationTestConfig::load_config();
        assert!(config.is_ok());
    }

    #[tokio::test]
    async fn test_test_results() {
        let mut results = TestResults::new();
        results.add_success("test1");
        results.add_failure("test2", crate::ActorCoreError::InvalidInput("test error".to_string()));
        
        assert_eq!(results.total_tests, 2);
        assert_eq!(results.success_rate, 0.5);
        assert!(!results.is_successful());
    }
}