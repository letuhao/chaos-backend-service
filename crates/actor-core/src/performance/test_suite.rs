//! Performance test suite for Actor Core.
//!
//! This module provides a comprehensive test suite for validating
//! performance thresholds and detecting regressions.

use std::time::Duration;
use std::sync::Arc;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::ActorCoreResult;
use crate::types::{Actor, Contribution, SubsystemOutput};
use crate::service_factory::ServiceFactory;
use crate::config::manager::ConfigurationManager;
use super::profiler::{PerformanceProfiler, ProfilerConfig, PerformanceReport};

/// Result of a performance test.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceTestResult {
    /// Test name
    pub test_name: String,
    /// Test duration
    pub duration: Duration,
    /// Test passed
    pub passed: bool,
    /// Performance score (0.0 - 100.0)
    pub score: f64,
    /// Test metrics
    pub metrics: HashMap<String, f64>,
    /// Error message if test failed
    pub error_message: Option<String>,
    /// Performance violations detected
    pub violations: Vec<String>,
    /// Performance recommendations
    pub recommendations: Vec<String>,
}

impl PerformanceTestResult {
    /// Convert from profiler's PerformanceTestResult to test suite's PerformanceTestResult
    pub fn from_profiler_result(profiler_result: super::profiler::PerformanceTestResult) -> Self {
        let mut metrics = HashMap::new();
        // Extract metrics from the PerformanceMetrics struct
        metrics.insert("aggregation_ops".to_string(), profiler_result.metrics.aggregation.total_aggregations as f64);
        metrics.insert("cache_hit_rate".to_string(), profiler_result.metrics.cache.hit_rate);
        metrics.insert("cache_miss_rate".to_string(), profiler_result.metrics.cache.miss_rate);
        metrics.insert("system_memory".to_string(), profiler_result.metrics.system.memory_usage as f64);
        metrics.insert("error_count".to_string(), profiler_result.metrics.errors.total_errors as f64);

        Self {
            test_name: profiler_result.test_name,
            duration: profiler_result.duration,
            passed: profiler_result.passed,
            score: profiler_result.score,
            metrics,
            error_message: None, // Profiler result doesn't have error_message field
            violations: profiler_result.violations.into_iter().map(|v| {
                format!("{}: {} (threshold: {})", v.threshold_name, v.actual_value, v.threshold_value)
            }).collect(),
            recommendations: profiler_result.recommendations,
        }
    }
}

/// Performance test suite that runs comprehensive performance tests.
pub struct PerformanceTestSuite {
    /// Performance profiler
    profiler: PerformanceProfiler,
    /// Test configuration
    config: TestSuiteConfig,
    /// Configuration manager
    _config_manager: Arc<ConfigurationManager>,
}

/// Configuration for the performance test suite.
#[derive(Debug, Clone)]
pub struct TestSuiteConfig {
    /// Enable cache performance tests
    pub enable_cache_tests: bool,
    /// Enable aggregation performance tests
    pub enable_aggregation_tests: bool,
    /// Enable memory performance tests
    pub enable_memory_tests: bool,
    /// Enable concurrency performance tests
    pub enable_concurrency_tests: bool,
    /// Number of actors for stress tests
    pub stress_test_actor_count: usize,
    /// Number of contributions per actor
    pub contributions_per_actor: usize,
    /// Test duration for stress tests
    pub stress_test_duration: Duration,
    /// Number of concurrent threads for concurrency tests
    pub concurrency_thread_count: usize,
}

impl Default for TestSuiteConfig {
    fn default() -> Self {
        Self {
            enable_cache_tests: true,
            enable_aggregation_tests: true,
            enable_memory_tests: true,
            enable_concurrency_tests: true,
            stress_test_actor_count: 1000,
            contributions_per_actor: 10,
            stress_test_duration: Duration::from_secs(30),
            concurrency_thread_count: 4,
        }
    }
}

impl PerformanceTestSuite {
    /// Create a new performance test suite.
    pub fn new(config: TestSuiteConfig, profiler_config: ProfilerConfig, config_manager: Arc<ConfigurationManager>) -> Self {
        Self {
            profiler: PerformanceProfiler::new(profiler_config, config_manager.clone()),
            config,
            _config_manager: config_manager,
        }
    }

    /// Create a new test suite with default configuration.
    pub fn new_default(config_manager: Arc<ConfigurationManager>) -> Self {
        Self::new(TestSuiteConfig::default(), ProfilerConfig::default(), config_manager)
    }

    /// Run all performance tests.
    pub async fn run_all_tests(&self) -> ActorCoreResult<TestSuiteResults> {
        let mut results = TestSuiteResults::new();

        // Run cache performance tests
        if self.config.enable_cache_tests {
            results.cache_tests = self.run_cache_tests().await?;
        }

        // Run aggregation performance tests
        if self.config.enable_aggregation_tests {
            results.aggregation_tests = self.run_aggregation_tests().await?;
        }

        // Run memory performance tests
        if self.config.enable_memory_tests {
            results.memory_tests = self.run_memory_tests().await?;
        }

        // Run concurrency performance tests
        if self.config.enable_concurrency_tests {
            results.concurrency_tests = self.run_concurrency_tests().await?;
        }

        // Calculate overall results
        results.calculate_overall_results();

        Ok(results)
    }

    /// Run cache performance tests.
    async fn run_cache_tests(&self) -> ActorCoreResult<Vec<PerformanceTestResult>> {
        let mut results = Vec::new();

        // Test cache creation performance
        let cache_creation_result = self.profiler.run_performance_test(
            "cache_creation",
            || {
                let cache = ServiceFactory::create_cache()?;
                // Verify cache is working
                cache.set("test_key".to_string(), serde_json::json!("test_value"), Some(60))?;
                let _ = cache.get("test_key");
                Ok(())
            },
        )?;
        results.push(PerformanceTestResult::from_profiler_result(cache_creation_result));

        // Test cache operations performance
        let cache_operations_result = self.profiler.run_performance_test(
            "cache_operations",
            || {
                let cache = ServiceFactory::create_cache()?;
                
                // Perform many cache operations
                for i in 0..1000 {
                    let key = format!("key_{}", i);
                    let value = serde_json::json!(format!("value_{}", i));
                    cache.set(key, value, Some(60))?;
                }

                // Read operations
                for i in 0..1000 {
                    let key = format!("key_{}", i);
                    let _ = cache.get(&key);
                }

                Ok(())
            },
        )?;
        results.push(PerformanceTestResult::from_profiler_result(cache_operations_result));

        // Test cache hit rate performance
        let cache_hit_rate_result = self.profiler.run_performance_test(
            "cache_hit_rate",
            || {
                let cache = ServiceFactory::create_cache()?;
                
                // Pre-populate cache
                for i in 0..100 {
                    let key = format!("key_{}", i);
                    let value = serde_json::json!(format!("value_{}", i));
                    cache.set(key, value, Some(60))?;
                }

                // Read same keys multiple times to test hit rate
                for _ in 0..10 {
                    for i in 0..100 {
                        let key = format!("key_{}", i);
                        let _ = cache.get(&key);
                    }
                }

                Ok(())
            },
        )?;
        results.push(PerformanceTestResult::from_profiler_result(cache_hit_rate_result));

        Ok(results)
    }

    /// Run aggregation performance tests.
    async fn run_aggregation_tests(&self) -> ActorCoreResult<Vec<PerformanceTestResult>> {
        let mut results = Vec::new();

        // Test basic aggregation performance
        let basic_aggregation_result = self.profiler.run_performance_test(
            "basic_aggregation",
            || {
                let plugin_registry = ServiceFactory::create_plugin_registry();
                let combiner_registry = ServiceFactory::create_combiner_registry();
                let caps_registry = ServiceFactory::create_cap_layer_registry();
                let caps_provider = ServiceFactory::create_caps_provider(caps_registry);
                let cache = ServiceFactory::create_cache()?;
                let aggregator = ServiceFactory::create_aggregator(
                    plugin_registry,
                    combiner_registry,
                    caps_provider,
                    cache,
                );

                // Create a test actor
                let actor = Actor::new("test_actor".to_string(), "human".to_string());
                
                // Create test contributions
                let contributions = vec![
                    Contribution::new("strength".to_string(), crate::Bucket::Flat, 10.0, "test_system".to_string()),
                    Contribution::new("agility".to_string(), crate::Bucket::Flat, 15.0, "test_system".to_string()),
                    Contribution::new("intelligence".to_string(), crate::Bucket::Flat, 12.0, "test_system".to_string()),
                ];

                let _subsystem_output = SubsystemOutput {
                    system_id: "test_system".to_string(),
                    primary: contributions,
                    derived: Vec::new(),
                    caps: Vec::new(),
                    processing_time: 0,
                    context: None,
                    meta: crate::types::SubsystemMeta {
                        system_id: "test_system".to_string(),
                        priority: 0,
                        version: "1.0.0".to_string(),
                        dependencies: Vec::new(),
                        system: "test_system".to_string(),
                        data: std::collections::HashMap::new(),
                        created_at: chrono::Utc::now(),
                    },
                    created_at: chrono::Utc::now(),
                };

                // Run aggregation
                let _snapshot = futures::executor::block_on(aggregator.resolve(&actor))?;

                Ok(())
            },
        )?;
        results.push(PerformanceTestResult::from_profiler_result(basic_aggregation_result));

        // Test stress aggregation performance
        let stress_aggregation_result = self.profiler.run_performance_test(
            "stress_aggregation",
            || {
                let plugin_registry = ServiceFactory::create_plugin_registry();
                let combiner_registry = ServiceFactory::create_combiner_registry();
                let caps_registry = ServiceFactory::create_cap_layer_registry();
                let caps_provider = ServiceFactory::create_caps_provider(caps_registry);
                let cache = ServiceFactory::create_cache()?;
                let aggregator = ServiceFactory::create_aggregator(
                    plugin_registry,
                    combiner_registry,
                    caps_provider,
                    cache,
                );

                // Create many actors and perform aggregations
                for i in 0..self.config.stress_test_actor_count {
                    let actor = Actor::new(format!("actor_{}", i), "human".to_string());
                    
                    let mut contributions = Vec::new();
                    for j in 0..self.config.contributions_per_actor {
                        contributions.push(Contribution::new(
                            format!("stat_{}", j),
                            crate::Bucket::Flat,
                            (i * j) as f64,
                            "test_system".to_string(),
                        ));
                    }

                    let _subsystem_output = SubsystemOutput {
                        system_id: "test_system".to_string(),
                        primary: contributions,
                        derived: Vec::new(),
                        caps: Vec::new(),
                        processing_time: 0,
                        context: None,
                        meta: crate::types::SubsystemMeta {
                            system_id: "test_system".to_string(),
                            priority: 0,
                            version: "1.0.0".to_string(),
                            dependencies: Vec::new(),
                            system: "test_system".to_string(),
                            data: std::collections::HashMap::new(),
                            created_at: chrono::Utc::now(),
                        },
                        created_at: chrono::Utc::now(),
                    };

                    let _snapshot = futures::executor::block_on(aggregator.resolve(&actor))?;
                }

                Ok(())
            },
        )?;
        results.push(PerformanceTestResult::from_profiler_result(stress_aggregation_result));

        Ok(results)
    }

    /// Run memory performance tests.
    async fn run_memory_tests(&self) -> ActorCoreResult<Vec<PerformanceTestResult>> {
        let mut results = Vec::new();

        // Test memory allocation performance
        let memory_allocation_result = self.profiler.run_performance_test(
            "memory_allocation",
            || {
                // Allocate many actors to test memory usage
                let mut actors = Vec::new();
                for i in 0..10000 {
                    let actor = Actor::new(format!("actor_{}", i), "human".to_string());
                    actors.push(actor);
                }

                // Verify actors were created
                assert_eq!(actors.len(), 10000);
                
                Ok(())
            },
        )?;
        results.push(PerformanceTestResult::from_profiler_result(memory_allocation_result));

        // Test memory cleanup performance
        let memory_cleanup_result = self.profiler.run_performance_test(
            "memory_cleanup",
            || {
                // Create and immediately drop many objects
                for i in 0..5000 {
                    let _actor = Actor::new(format!("temp_actor_{}", i), "human".to_string());
                    // Actor is automatically dropped here
                }

                Ok(())
            },
        )?;
        results.push(PerformanceTestResult::from_profiler_result(memory_cleanup_result));

        Ok(results)
    }

    /// Run concurrency performance tests.
    async fn run_concurrency_tests(&self) -> ActorCoreResult<Vec<PerformanceTestResult>> {
        let mut results = Vec::new();

        // Test concurrent cache operations
        let concurrent_cache_result = self.profiler.run_performance_test(
            "concurrent_cache_operations",
            || {
                let cache = Arc::new(ServiceFactory::create_cache()?);
                let mut handles = Vec::new();

                // Spawn concurrent threads
                for thread_id in 0..self.config.concurrency_thread_count {
                    let cache_clone = Arc::clone(&cache);
                    let handle = std::thread::spawn(move || {
                        for i in 0..100 {
                            let key = format!("thread_{}_key_{}", thread_id, i);
                            let value = serde_json::json!(format!("thread_{}_value_{}", thread_id, i));
                            let _ = cache_clone.set(key, value, Some(60));
                        }
                    });
                    handles.push(handle);
                }

                // Wait for all threads to complete
                for handle in handles {
                    handle.join().unwrap();
                }

                Ok(())
            },
        )?;
        results.push(PerformanceTestResult::from_profiler_result(concurrent_cache_result));

        // Test concurrent aggregation operations
        let concurrent_aggregation_result = self.profiler.run_performance_test(
            "concurrent_aggregation",
            || {
                let plugin_registry = ServiceFactory::create_plugin_registry();
                let combiner_registry = ServiceFactory::create_combiner_registry();
                let caps_registry = ServiceFactory::create_cap_layer_registry();
                let caps_provider = ServiceFactory::create_caps_provider(caps_registry);
                let cache = ServiceFactory::create_cache()?;
                let aggregator = Arc::new(ServiceFactory::create_aggregator(
                    plugin_registry,
                    combiner_registry,
                    caps_provider,
                    cache,
                ));

                let mut handles = Vec::new();

                // Spawn concurrent threads for aggregation
                for thread_id in 0..self.config.concurrency_thread_count {
                    let aggregator_clone = Arc::clone(&aggregator);
                    let handle = std::thread::spawn(move || {
                        for i in 0..100 {
                            let actor = Actor::new(format!("thread_{}_actor_{}", thread_id, i), "human".to_string());
                            let contributions = vec![
                                Contribution::new(
                                    "strength".to_string(),
                                    crate::Bucket::Flat,
                                    (thread_id * i) as f64,
                                    "test_system".to_string(),
                                ),
                            ];
                            let _subsystem_output = SubsystemOutput {
                                system_id: "test_system".to_string(),
                                primary: contributions,
                                derived: Vec::new(),
                                caps: Vec::new(),
                                processing_time: 0,
                                context: None,
                                meta: crate::types::SubsystemMeta {
                                    system_id: "test_system".to_string(),
                                    priority: 0,
                                    version: "1.0.0".to_string(),
                                    dependencies: Vec::new(),
                                    system: "test_system".to_string(),
                                    data: std::collections::HashMap::new(),
                                    created_at: chrono::Utc::now(),
                                },
                                created_at: chrono::Utc::now(),
                            };
                            let _ = futures::executor::block_on(aggregator_clone.resolve(&actor));
                        }
                    });
                    handles.push(handle);
                }

                // Wait for all threads to complete
                for handle in handles {
                    handle.join().unwrap();
                }

                Ok(())
            },
        )?;
        results.push(PerformanceTestResult::from_profiler_result(concurrent_aggregation_result));

        Ok(results)
    }

    /// Get the performance profiler.
    pub fn profiler(&self) -> &PerformanceProfiler {
        &self.profiler
    }

    /// Generate a comprehensive performance report.
    pub fn generate_report(&self) -> PerformanceReport {
        self.profiler.generate_report()
    }
}

/// Results from running the performance test suite.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestSuiteResults {
    /// Cache performance test results
    pub cache_tests: Vec<PerformanceTestResult>,
    /// Aggregation performance test results
    pub aggregation_tests: Vec<PerformanceTestResult>,
    /// Memory performance test results
    pub memory_tests: Vec<PerformanceTestResult>,
    /// Concurrency performance test results
    pub concurrency_tests: Vec<PerformanceTestResult>,
    /// Overall test suite results
    pub overall: OverallTestResults,
}

impl TestSuiteResults {
    /// Create new test suite results.
    pub fn new() -> Self {
        Self {
            cache_tests: Vec::new(),
            aggregation_tests: Vec::new(),
            memory_tests: Vec::new(),
            concurrency_tests: Vec::new(),
            overall: OverallTestResults::default(),
        }
    }

    /// Calculate overall test results.
    pub fn calculate_overall_results(&mut self) {
        let all_tests: Vec<&PerformanceTestResult> = self.cache_tests
            .iter()
            .chain(self.aggregation_tests.iter())
            .chain(self.memory_tests.iter())
            .chain(self.concurrency_tests.iter())
            .collect();

        let total_tests = all_tests.len();
        let passed_tests = all_tests.iter().filter(|t| t.passed).count();
        let failed_tests = total_tests - passed_tests;

        let total_score: f64 = all_tests.iter().map(|t| t.score).sum();
        let average_score = if total_tests > 0 { total_score / total_tests as f64 } else { 0.0 };

        let total_violations: usize = all_tests.iter().map(|t| t.violations.len()).sum();
        let total_duration: Duration = all_tests.iter().map(|t| t.duration).sum();

        self.overall = OverallTestResults {
            total_tests,
            passed_tests,
            failed_tests,
            pass_rate: if total_tests > 0 { (passed_tests as f64 / total_tests as f64) * 100.0 } else { 0.0 },
            average_score,
            total_violations,
            total_duration,
            overall_status: if failed_tests == 0 {
                TestStatus::Passed
            } else if failed_tests <= total_tests / 4 {
                TestStatus::Warning
            } else {
                TestStatus::Failed
            },
        };
    }

    /// Get all test results.
    pub fn all_tests(&self) -> Vec<&PerformanceTestResult> {
        self.cache_tests
            .iter()
            .chain(self.aggregation_tests.iter())
            .chain(self.memory_tests.iter())
            .chain(self.concurrency_tests.iter())
            .collect()
    }

    /// Get failed tests.
    pub fn failed_tests(&self) -> Vec<&PerformanceTestResult> {
        self.all_tests().into_iter().filter(|t| !t.passed).collect()
    }

    /// Get all recommendations from failed tests.
    pub fn get_recommendations(&self) -> Vec<String> {
        let mut recommendations = Vec::new();
        for test in self.failed_tests() {
            recommendations.extend(test.recommendations.clone());
        }
        recommendations.sort();
        recommendations.dedup();
        recommendations
    }
}

/// Overall test results summary.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OverallTestResults {
    /// Total number of tests
    pub total_tests: usize,
    /// Number of passed tests
    pub passed_tests: usize,
    /// Number of failed tests
    pub failed_tests: usize,
    /// Pass rate percentage
    pub pass_rate: f64,
    /// Average performance score
    pub average_score: f64,
    /// Total threshold violations
    pub total_violations: usize,
    /// Total test duration
    pub total_duration: Duration,
    /// Overall test status
    pub overall_status: TestStatus,
}

/// Test status levels.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TestStatus {
    /// All tests passed
    Passed,
    /// Some tests failed but within acceptable limits
    Warning,
    /// Significant number of tests failed
    Failed,
}

impl Default for TestStatus {
    fn default() -> Self {
        TestStatus::Passed
    }
}

impl std::fmt::Display for TestStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TestStatus::Passed => write!(f, "PASSED"),
            TestStatus::Warning => write!(f, "WARNING"),
            TestStatus::Failed => write!(f, "FAILED"),
        }
    }
}