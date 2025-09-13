//! Performance Monitoring System
//!
//! This module provides comprehensive performance monitoring and testing
//! capabilities for the Enhanced Hybrid Resource Manager.

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use std::time::{Duration, Instant};
use crate::ActorCoreResult;
use crate::types::Actor;

/// Performance Monitor
pub struct PerformanceMonitor {
    /// Performance metrics
    metrics: Arc<RwLock<HashMap<String, PerformanceMetric>>>,
    /// Performance tests
    tests: Arc<RwLock<HashMap<String, PerformanceTest>>>,
    /// Configuration
    config: PerformanceConfig,
}

/// Performance Metric
#[derive(Debug, Clone)]
pub struct PerformanceMetric {
    /// Metric name
    pub name: String,
    /// Metric value
    pub value: f64,
    /// Metric unit
    pub unit: String,
    /// Timestamp
    pub timestamp: u64,
    /// Metric category
    pub category: MetricCategory,
}

/// Performance Test
#[derive(Debug, Clone)]
pub struct PerformanceTest {
    /// Test name
    pub name: String,
    /// Test description
    pub description: String,
    /// Test results
    pub results: Vec<TestResult>,
    /// Test configuration
    pub config: TestConfig,
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

/// Test Configuration
#[derive(Debug, Clone)]
pub struct TestConfig {
    /// Number of iterations
    pub iterations: usize,
    /// Warmup iterations
    pub warmup_iterations: usize,
    /// Timeout in milliseconds
    pub timeout_ms: u64,
    /// Memory limit in bytes
    pub memory_limit_bytes: Option<usize>,
    /// Enable memory profiling
    pub enable_memory_profiling: bool,
    /// Enable CPU profiling
    pub enable_cpu_profiling: bool,
}

/// Performance Configuration
#[derive(Debug, Clone)]
pub struct PerformanceConfig {
    /// Enable performance monitoring
    pub enable_monitoring: bool,
    /// Metrics collection interval in seconds
    pub collection_interval: u64,
    /// Maximum metrics history
    pub max_metrics_history: usize,
    /// Enable automatic testing
    pub enable_automatic_testing: bool,
    /// Test execution interval in seconds
    pub test_interval: u64,
}

impl Default for PerformanceConfig {
    fn default() -> Self {
        Self {
            enable_monitoring: true,
            collection_interval: 60, // 1 minute
            max_metrics_history: 1000,
            enable_automatic_testing: false,
            test_interval: 300, // 5 minutes
        }
    }
}

/// Metric Categories
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum MetricCategory {
    /// Resource calculation performance
    ResourceCalculation,
    /// Cache performance
    Cache,
    /// Database performance
    Database,
    /// Memory usage
    Memory,
    /// CPU usage
    Cpu,
    /// Network performance
    Network,
    /// Event processing
    EventProcessing,
}

impl PerformanceMonitor {
    /// Create a new Performance Monitor
    pub fn new(config: PerformanceConfig) -> Self {
        Self {
            metrics: Arc::new(RwLock::new(HashMap::new())),
            tests: Arc::new(RwLock::new(HashMap::new())),
            config,
        }
    }
    
    /// Record a performance metric
    pub async fn record_metric(&self, metric: PerformanceMetric) -> ActorCoreResult<()> {
        if !self.config.enable_monitoring {
            return Ok(());
        }
        
        let mut metrics = self.metrics.write().await;
        metrics.insert(metric.name.clone(), metric);
        
        // Trim metrics if we exceed the limit
        if metrics.len() > self.config.max_metrics_history {
            let excess = metrics.len() - self.config.max_metrics_history;
            let keys_to_remove: Vec<String> = metrics.keys().take(excess).cloned().collect();
            for key in keys_to_remove {
                metrics.remove(&key);
            }
        }
        
        Ok(())
    }
    
    /// Get performance metrics
    pub async fn get_metrics(&self, category: Option<&MetricCategory>) -> ActorCoreResult<Vec<PerformanceMetric>> {
        let metrics = self.metrics.read().await;
        let mut result: Vec<PerformanceMetric> = metrics.values().cloned().collect();
        
        // Filter by category if specified
        if let Some(category) = category {
            result.retain(|metric| &metric.category == category);
        }
        
        // Sort by timestamp (newest first)
        result.sort_by_key(|metric| std::cmp::Reverse(metric.timestamp));
        
        Ok(result)
    }
    
    /// Run a performance test
    pub async fn run_test<F>(&self, test_name: &str, test_func: F) -> ActorCoreResult<TestResult>
    where
        F: Fn() -> ActorCoreResult<()>,
    {
        let start_time = Instant::now();
        let start_memory = self.get_memory_usage();
        
        let result = test_func();
        let success = result.is_ok();
        let error_message = if let Err(e) = result {
            Some(format!("{}", e))
        } else {
            None
        };
        
        let execution_time = start_time.elapsed();
        let end_memory = self.get_memory_usage();
        let memory_usage = end_memory.saturating_sub(start_memory);
        
        let test_result = TestResult {
            test_name: test_name.to_string(),
            execution_time_ms: execution_time.as_millis() as f64,
            memory_usage_bytes: memory_usage,
            success,
            error_message,
            timestamp: self.get_current_timestamp(),
            additional_metrics: HashMap::new(),
        };
        
        // Store the test result
        self.store_test_result(test_name, test_result.clone()).await?;
        
        Ok(test_result)
    }
    
    /// Run a benchmark test
    pub async fn run_benchmark<F>(&self, test_name: &str, config: TestConfig, test_func: F) -> ActorCoreResult<Vec<TestResult>>
    where
        F: Fn() -> ActorCoreResult<()>,
    {
        let mut results = Vec::new();
        
        // Warmup iterations
        for _ in 0..config.warmup_iterations {
            let _ = test_func();
        }
        
        // Actual test iterations
        for i in 0..config.iterations {
            let start_time = Instant::now();
            let start_memory = self.get_memory_usage();
            
            let result = test_func();
            let success = result.is_ok();
            let error_message = if let Err(e) = result {
                Some(format!("{}", e))
            } else {
                None
            };
            
            let execution_time = start_time.elapsed();
            let end_memory = self.get_memory_usage();
            let memory_usage = end_memory.saturating_sub(start_memory);
            
            let test_result = TestResult {
                test_name: format!("{}_iteration_{}", test_name, i),
                execution_time_ms: execution_time.as_millis() as f64,
                memory_usage_bytes: memory_usage,
                success,
                error_message,
                timestamp: self.get_current_timestamp(),
                additional_metrics: HashMap::new(),
            };
            
            results.push(test_result);
        }
        
        // Store all results
        for result in &results {
            self.store_test_result(test_name, result.clone()).await?;
        }
        
        Ok(results)
    }
    
    /// Get test results
    pub async fn get_test_results(&self, test_name: Option<&str>) -> ActorCoreResult<Vec<TestResult>> {
        let tests = self.tests.read().await;
        let mut all_results = Vec::new();
        
        if let Some(test_name) = test_name {
            if let Some(test) = tests.get(test_name) {
                all_results.extend(test.results.clone());
            }
        } else {
            for test in tests.values() {
                all_results.extend(test.results.clone());
            }
        }
        
        // Sort by timestamp (newest first)
        all_results.sort_by_key(|result| std::cmp::Reverse(result.timestamp));
        
        Ok(all_results)
    }
    
    /// Get performance statistics
    pub async fn get_performance_stats(&self) -> ActorCoreResult<PerformanceStats> {
        let metrics = self.metrics.read().await;
        let tests = self.tests.read().await;
        
        let mut total_metrics = 0;
        let mut category_counts = HashMap::new();
        let mut total_tests = 0;
        let mut successful_tests = 0;
        let mut total_execution_time = 0.0;
        let mut total_memory_usage = 0;
        
        // Process metrics
        for metric in metrics.values() {
            total_metrics += 1;
            *category_counts.entry(metric.category.clone()).or_insert(0) += 1;
        }
        
        // Process test results
        for test in tests.values() {
            for result in &test.results {
                total_tests += 1;
                if result.success {
                    successful_tests += 1;
                }
                total_execution_time += result.execution_time_ms;
                total_memory_usage += result.memory_usage_bytes;
            }
        }
        
        let success_rate = if total_tests > 0 {
            successful_tests as f64 / total_tests as f64
        } else {
            0.0
        };
        
        let average_execution_time = if total_tests > 0 {
            total_execution_time / total_tests as f64
        } else {
            0.0
        };
        
        let average_memory_usage = if total_tests > 0 {
            total_memory_usage / total_tests
        } else {
            0
        };
        
        Ok(PerformanceStats {
            total_metrics,
            category_counts,
            total_tests,
            successful_tests,
            success_rate,
            average_execution_time,
            average_memory_usage,
        })
    }
    
    /// Store test result
    async fn store_test_result(&self, test_name: &str, result: TestResult) -> ActorCoreResult<()> {
        let mut tests = self.tests.write().await;
        
        let test = tests.entry(test_name.to_string()).or_insert_with(|| PerformanceTest {
            name: test_name.to_string(),
            description: String::new(),
            results: Vec::new(),
            config: TestConfig {
                iterations: 1,
                warmup_iterations: 0,
                timeout_ms: 1000,
                memory_limit_bytes: None,
                enable_memory_profiling: false,
                enable_cpu_profiling: false,
            },
        });
        
        test.results.push(result);
        
        // Keep only the last 100 results per test
        if test.results.len() > 100 {
            test.results.drain(0..test.results.len() - 100);
        }
        
        Ok(())
    }
    
    /// Get current memory usage
    fn get_memory_usage(&self) -> usize {
        // This is a simplified implementation
        // In practice, you'd use a proper memory profiling library
        0
    }
    
    /// Get current timestamp
    fn get_current_timestamp(&self) -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }
}

/// Performance Statistics
#[derive(Debug, Clone)]
pub struct PerformanceStats {
    /// Total number of metrics
    pub total_metrics: usize,
    /// Metric counts by category
    pub category_counts: HashMap<MetricCategory, usize>,
    /// Total number of tests
    pub total_tests: usize,
    /// Number of successful tests
    pub successful_tests: usize,
    /// Test success rate
    pub success_rate: f64,
    /// Average execution time in milliseconds
    pub average_execution_time: f64,
    /// Average memory usage in bytes
    pub average_memory_usage: usize,
}

/// Load Testing Suite
pub struct LoadTestingSuite {
    /// Performance monitor
    monitor: Arc<PerformanceMonitor>,
    /// Test actors
    test_actors: Vec<Actor>,
}

impl LoadTestingSuite {
    /// Create a new Load Testing Suite
    pub fn new(monitor: Arc<PerformanceMonitor>) -> Self {
        Self {
            monitor,
            test_actors: Vec::new(),
        }
    }
    
    /// Generate test actors
    pub fn generate_test_actors(&mut self, count: usize) -> ActorCoreResult<()> {
        self.test_actors.clear();
        
        for i in 0..count {
            let actor = Actor::new(
                format!("test_actor_{}", i),
                "Human".to_string(),
            );
            self.test_actors.push(actor);
        }
        
        Ok(())
    }
    
    /// Run resource calculation load test
    pub async fn run_resource_calculation_load_test(&self, iterations: usize) -> ActorCoreResult<Vec<TestResult>> {
        let config = TestConfig {
            iterations,
            warmup_iterations: 10,
            timeout_ms: 5000,
            memory_limit_bytes: Some(100 * 1024 * 1024), // 100MB
            enable_memory_profiling: true,
            enable_cpu_profiling: true,
        };
        
        let results = self.monitor.run_benchmark(
            "resource_calculation_load_test",
            config,
            || {
                // Simulate resource calculation for all test actors
                for _actor in &self.test_actors {
                    // This would call the actual resource calculation
                    // For now, just simulate some work
                    std::thread::sleep(Duration::from_micros(100));
                }
                Ok(())
            },
        ).await?;
        
        Ok(results)
    }
    
    /// Run cache performance test
    pub async fn run_cache_performance_test(&self, iterations: usize) -> ActorCoreResult<Vec<TestResult>> {
        let config = TestConfig {
            iterations,
            warmup_iterations: 5,
            timeout_ms: 3000,
            memory_limit_bytes: Some(50 * 1024 * 1024), // 50MB
            enable_memory_profiling: true,
            enable_cpu_profiling: false,
        };
        
        let results = self.monitor.run_benchmark(
            "cache_performance_test",
            config,
            || {
                // Simulate cache operations
                for _actor in &self.test_actors {
                    // This would call the actual cache operations
                    // For now, just simulate some work
                    std::thread::sleep(Duration::from_micros(50));
                }
                Ok(())
            },
        ).await?;
        
        Ok(results)
    }
    
    /// Run database performance test
    pub async fn run_database_performance_test(&self, iterations: usize) -> ActorCoreResult<Vec<TestResult>> {
        let config = TestConfig {
            iterations,
            warmup_iterations: 5,
            timeout_ms: 10000,
            memory_limit_bytes: Some(200 * 1024 * 1024), // 200MB
            enable_memory_profiling: true,
            enable_cpu_profiling: true,
        };
        
        let results = self.monitor.run_benchmark(
            "database_performance_test",
            config,
            || {
                // Simulate database operations
                for _actor in &self.test_actors {
                    // This would call the actual database operations
                    // For now, just simulate some work
                    std::thread::sleep(Duration::from_millis(1));
                }
                Ok(())
            },
        ).await?;
        
        Ok(results)
    }
    
    /// Run comprehensive performance test suite
    pub async fn run_comprehensive_test_suite(&self) -> ActorCoreResult<ComprehensiveTestResults> {
        let mut all_results = Vec::new();
        
        // Resource calculation test
        let resource_results = self.run_resource_calculation_load_test(100).await?;
        all_results.extend(resource_results);
        
        // Cache performance test
        let cache_results = self.run_cache_performance_test(100).await?;
        all_results.extend(cache_results);
        
        // Database performance test
        let database_results = self.run_database_performance_test(50).await?;
        all_results.extend(database_results);
        
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
    async fn test_performance_monitor() {
        let config = PerformanceConfig::default();
        let monitor = PerformanceMonitor::new(config);
        
        // Record a metric
        let metric = PerformanceMetric {
            name: "test_metric".to_string(),
            value: 100.0,
            unit: "ms".to_string(),
            timestamp: 1234567890,
            category: MetricCategory::ResourceCalculation,
        };
        
        monitor.record_metric(metric).await.unwrap();
        
        // Get metrics
        let metrics = monitor.get_metrics(None).await.unwrap();
        assert_eq!(metrics.len(), 1);
        assert_eq!(metrics[0].name, "test_metric");
    }
    
    #[tokio::test]
    async fn test_performance_test() {
        let config = PerformanceConfig::default();
        let monitor = PerformanceMonitor::new(config);
        
        // Run a simple test
        let result = monitor.run_test("test_operation", || {
            // Simulate some work
            std::thread::sleep(Duration::from_millis(10));
            Ok(())
        }).await.unwrap();
        
        assert!(result.success);
        assert!(result.execution_time_ms > 0.0);
    }
    
    #[tokio::test]
    async fn test_load_testing_suite() {
        let config = PerformanceConfig::default();
        let monitor = Arc::new(PerformanceMonitor::new(config));
        let mut suite = LoadTestingSuite::new(monitor);
        
        // Generate test actors
        suite.generate_test_actors(10).unwrap();
        assert_eq!(suite.test_actors.len(), 10);
        
        // Run a load test
        let results = suite.run_resource_calculation_load_test(5).await.unwrap();
        assert_eq!(results.len(), 5);
    }
}
