//! Performance Testing Example
//!
//! This example demonstrates comprehensive performance testing and monitoring
//! capabilities of the Enhanced Hybrid Resource Manager.

use chaos_backend_service::crates::actor_core::subsystems::*;
use chaos_backend_service::crates::actor_core::types::Actor;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Instant;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Enhanced Hybrid Resource Manager - Performance Testing Example");
    
    // 1. Set up performance monitoring system
    let performance_monitor = setup_performance_monitoring().await?;
    
    // 2. Create load testing suite
    let mut load_test_suite = LoadTestingSuite::new(performance_monitor.clone());
    
    // 3. Generate test actors
    println!("\n👥 Generating test actors...");
    load_test_suite.generate_test_actors(1000)?;
    println!("   Generated 1000 test actors");
    
    // 4. Run individual performance tests
    println!("\n📊 Running individual performance tests...");
    run_individual_tests(&performance_monitor).await?;
    
    // 5. Run load tests
    println!("\n⚡ Running load tests...");
    run_load_tests(&mut load_test_suite).await?;
    
    // 6. Run stress tests
    println!("\n🔥 Running stress tests...");
    run_stress_tests(&performance_monitor).await?;
    
    // 7. Run memory tests
    println!("\n💾 Running memory tests...");
    run_memory_tests(&performance_monitor).await?;
    
    // 8. Generate performance report
    println!("\n📈 Generating performance report...");
    generate_performance_report(&performance_monitor).await?;
    
    println!("\n✅ Performance testing example completed successfully!");
    Ok(())
}

/// Set up comprehensive performance monitoring
async fn setup_performance_monitoring() -> Result<Arc<PerformanceMonitor>, Box<dyn std::error::Error>> {
    println!("🔧 Setting up performance monitoring system...");
    
    let performance_config = PerformanceConfig {
        enable_monitoring: true,
        collection_interval: 10, // 10 seconds
        max_metrics_history: 50000,
        enable_automatic_testing: true,
        test_interval: 60, // 1 minute
    };
    
    let monitor = Arc::new(PerformanceMonitor::new(performance_config));
    
    println!("✅ Performance monitoring setup complete");
    Ok(monitor)
}

/// Run individual performance tests
async fn run_individual_tests(monitor: &Arc<PerformanceMonitor>) -> Result<(), Box<dyn std::error::Error>> {
    // Test 1: Resource calculation performance
    println!("   Testing resource calculation performance...");
    let result1 = monitor.run_test("resource_calculation", || {
        // Simulate resource calculation
        std::thread::sleep(std::time::Duration::from_millis(1));
        Ok(())
    }).await?;
    
    println!("     Execution time: {:.2}ms", result1.execution_time_ms);
    println!("     Success: {}", result1.success);
    
    // Test 2: Cache performance
    println!("   Testing cache performance...");
    let result2 = monitor.run_test("cache_operations", || {
        // Simulate cache operations
        std::thread::sleep(std::time::Duration::from_micros(500));
        Ok(())
    }).await?;
    
    println!("     Execution time: {:.2}ms", result2.execution_time_ms);
    println!("     Success: {}", result2.success);
    
    // Test 3: Event processing performance
    println!("   Testing event processing performance...");
    let result3 = monitor.run_test("event_processing", || {
        // Simulate event processing
        std::thread::sleep(std::time::Duration::from_micros(200));
        Ok(())
    }).await?;
    
    println!("     Execution time: {:.2}ms", result3.execution_time_ms);
    println!("     Success: {}", result3.success);
    
    // Test 4: Database operations performance
    println!("   Testing database operations performance...");
    let result4 = monitor.run_test("database_operations", || {
        // Simulate database operations
        std::thread::sleep(std::time::Duration::from_millis(5));
        Ok(())
    }).await?;
    
    println!("     Execution time: {:.2}ms", result4.execution_time_ms);
    println!("     Success: {}", result4.success);
    
    Ok(())
}

/// Run comprehensive load tests
async fn run_load_tests(load_test_suite: &mut LoadTestingSuite) -> Result<(), Box<dyn std::error::Error>> {
    // Load test 1: Resource calculation with 1000 actors
    println!("   Running resource calculation load test...");
    let results1 = load_test_suite.run_resource_calculation_load_test(100).await?;
    
    let successful1 = results1.iter().filter(|r| r.success).count();
    let avg_time1 = results1.iter().map(|r| r.execution_time_ms).sum::<f64>() / results1.len() as f64;
    
    println!("     Tests: {}, Success: {}, Avg time: {:.2}ms", 
        results1.len(), successful1, avg_time1);
    
    // Load test 2: Cache performance with 1000 actors
    println!("   Running cache performance load test...");
    let results2 = load_test_suite.run_cache_performance_test(100).await?;
    
    let successful2 = results2.iter().filter(|r| r.success).count();
    let avg_time2 = results2.iter().map(|r| r.execution_time_ms).sum::<f64>() / results2.len() as f64;
    
    println!("     Tests: {}, Success: {}, Avg time: {:.2}ms", 
        results2.len(), successful2, avg_time2);
    
    // Load test 3: Database operations with 1000 actors
    println!("   Running database operations load test...");
    let results3 = load_test_suite.run_database_performance_test(50).await?;
    
    let successful3 = results3.iter().filter(|r| r.success).count();
    let avg_time3 = results3.iter().map(|r| r.execution_time_ms).sum::<f64>() / results3.len() as f64;
    
    println!("     Tests: {}, Success: {}, Avg time: {:.2}ms", 
        results3.len(), successful3, avg_time3);
    
    // Run comprehensive test suite
    println!("   Running comprehensive test suite...");
    let comprehensive_results = load_test_suite.run_comprehensive_test_suite().await?;
    
    println!("     Comprehensive results:");
    println!("       Total tests: {}", comprehensive_results.total_tests);
    println!("       Successful: {}", comprehensive_results.successful_tests);
    println!("       Success rate: {:.2}%", comprehensive_results.success_rate * 100.0);
    println!("       Total execution time: {:.2}ms", comprehensive_results.total_execution_time);
    println!("       Average execution time: {:.2}ms", comprehensive_results.average_execution_time);
    println!("       Total memory usage: {} bytes", comprehensive_results.total_memory_usage);
    println!("       Average memory usage: {} bytes", comprehensive_results.average_memory_usage);
    
    Ok(())
}

/// Run stress tests to find system limits
async fn run_stress_tests(monitor: &Arc<PerformanceMonitor>) -> Result<(), Box<dyn std::error::Error>> {
    println!("   Running stress tests to find system limits...");
    
    // Stress test 1: High-frequency resource calculations
    println!("     Stress test 1: High-frequency resource calculations");
    let stress_config1 = TestConfig {
        iterations: 1000,
        warmup_iterations: 50,
        timeout_ms: 10000,
        memory_limit_bytes: Some(200 * 1024 * 1024), // 200MB
        enable_memory_profiling: true,
        enable_cpu_profiling: true,
    };
    
    let stress_results1 = monitor.run_benchmark("stress_high_frequency", stress_config1, || {
        // Simulate high-frequency resource calculation
        std::thread::sleep(std::time::Duration::from_micros(100));
        Ok(())
    }).await?;
    
    let stress_success1 = stress_results1.iter().filter(|r| r.success).count();
    let stress_avg1 = stress_results1.iter().map(|r| r.execution_time_ms).sum::<f64>() / stress_results1.len() as f64;
    
    println!("       Tests: {}, Success: {}, Avg time: {:.2}ms", 
        stress_results1.len(), stress_success1, stress_avg1);
    
    // Stress test 2: Memory-intensive operations
    println!("     Stress test 2: Memory-intensive operations");
    let stress_config2 = TestConfig {
        iterations: 500,
        warmup_iterations: 25,
        timeout_ms: 15000,
        memory_limit_bytes: Some(500 * 1024 * 1024), // 500MB
        enable_memory_profiling: true,
        enable_cpu_profiling: true,
    };
    
    let stress_results2 = monitor.run_benchmark("stress_memory_intensive", stress_config2, || {
        // Simulate memory-intensive operation
        let _large_vector: Vec<u8> = vec![0; 1024 * 1024]; // 1MB
        std::thread::sleep(std::time::Duration::from_millis(2));
        Ok(())
    }).await?;
    
    let stress_success2 = stress_results2.iter().filter(|r| r.success).count();
    let stress_avg2 = stress_results2.iter().map(|r| r.execution_time_ms).sum::<f64>() / stress_results2.len() as f64;
    
    println!("       Tests: {}, Success: {}, Avg time: {:.2}ms", 
        stress_results2.len(), stress_success2, stress_avg2);
    
    // Stress test 3: Concurrent operations
    println!("     Stress test 3: Concurrent operations");
    let start_time = Instant::now();
    
    let mut handles = Vec::new();
    for i in 0..10 {
        let monitor_clone = monitor.clone();
        let handle = tokio::spawn(async move {
            let config = TestConfig {
                iterations: 100,
                warmup_iterations: 10,
                timeout_ms: 5000,
                memory_limit_bytes: Some(50 * 1024 * 1024), // 50MB
                enable_memory_profiling: false,
                enable_cpu_profiling: false,
            };
            
            monitor_clone.run_benchmark(
                &format!("concurrent_test_{}", i),
                config,
                || {
                    std::thread::sleep(std::time::Duration::from_millis(1));
                    Ok(())
                }
            ).await
        });
        handles.push(handle);
    }
    
    let mut concurrent_success = 0;
    let mut concurrent_total = 0;
    for handle in handles {
        if let Ok(results) = handle.await? {
            concurrent_success += results.iter().filter(|r| r.success).count();
            concurrent_total += results.len();
        }
    }
    
    let concurrent_duration = start_time.elapsed();
    let concurrent_avg = concurrent_duration.as_millis() as f64 / concurrent_total as f64;
    
    println!("       Tests: {}, Success: {}, Avg time: {:.2}ms", 
        concurrent_total, concurrent_success, concurrent_avg);
    
    Ok(())
}

/// Run memory usage tests
async fn run_memory_tests(monitor: &Arc<PerformanceMonitor>) -> Result<(), Box<dyn std::error::Error>> {
    println!("   Running memory usage tests...");
    
    // Memory test 1: Cache memory usage
    println!("     Memory test 1: Cache memory usage");
    let memory_config1 = TestConfig {
        iterations: 100,
        warmup_iterations: 10,
        timeout_ms: 10000,
        memory_limit_bytes: Some(100 * 1024 * 1024), // 100MB
        enable_memory_profiling: true,
        enable_cpu_profiling: false,
    };
    
    let memory_results1 = monitor.run_benchmark("memory_cache_usage", memory_config1, || {
        // Simulate cache operations with memory allocation
        let _cache_data: Vec<u8> = vec![0; 1024 * 100]; // 100KB
        std::thread::sleep(std::time::Duration::from_micros(500));
        Ok(())
    }).await?;
    
    let memory_success1 = memory_results1.iter().filter(|r| r.success).count();
    let memory_avg1 = memory_results1.iter().map(|r| r.memory_usage_bytes).sum::<usize>() / memory_results1.len();
    
    println!("       Tests: {}, Success: {}, Avg memory: {} bytes", 
        memory_results1.len(), memory_success1, memory_avg1);
    
    // Memory test 2: Event system memory usage
    println!("     Memory test 2: Event system memory usage");
    let memory_config2 = TestConfig {
        iterations: 50,
        warmup_iterations: 5,
        timeout_ms: 15000,
        memory_limit_bytes: Some(200 * 1024 * 1024), // 200MB
        enable_memory_profiling: true,
        enable_cpu_profiling: false,
    };
    
    let memory_results2 = monitor.run_benchmark("memory_event_system", memory_config2, || {
        // Simulate event system operations
        let _event_data: Vec<u8> = vec![0; 1024 * 500]; // 500KB
        std::thread::sleep(std::time::Duration::from_millis(1));
        Ok(())
    }).await?;
    
    let memory_success2 = memory_results2.iter().filter(|r| r.success).count();
    let memory_avg2 = memory_results2.iter().map(|r| r.memory_usage_bytes).sum::<usize>() / memory_results2.len();
    
    println!("       Tests: {}, Success: {}, Avg memory: {} bytes", 
        memory_results2.len(), memory_success2, memory_avg2);
    
    // Memory test 3: Resource calculation memory usage
    println!("     Memory test 3: Resource calculation memory usage");
    let memory_config3 = TestConfig {
        iterations: 200,
        warmup_iterations: 20,
        timeout_ms: 20000,
        memory_limit_bytes: Some(300 * 1024 * 1024), // 300MB
        enable_memory_profiling: true,
        enable_cpu_profiling: false,
    };
    
    let memory_results3 = monitor.run_benchmark("memory_resource_calculation", memory_config3, || {
        // Simulate resource calculation with memory allocation
        let _resource_data: Vec<u8> = vec![0; 1024 * 200]; // 200KB
        std::thread::sleep(std::time::Duration::from_micros(800));
        Ok(())
    }).await?;
    
    let memory_success3 = memory_results3.iter().filter(|r| r.success).count();
    let memory_avg3 = memory_results3.iter().map(|r| r.memory_usage_bytes).sum::<usize>() / memory_results3.len();
    
    println!("       Tests: {}, Success: {}, Avg memory: {} bytes", 
        memory_results3.len(), memory_success3, memory_avg3);
    
    Ok(())
}

/// Generate comprehensive performance report
async fn generate_performance_report(monitor: &Arc<PerformanceMonitor>) -> Result<(), Box<dyn std::error::Error>> {
    println!("   Generating comprehensive performance report...");
    
    // Get performance statistics
    let stats = monitor.get_performance_stats().await?;
    
    println!("\n📊 PERFORMANCE REPORT");
    println!("====================");
    println!("Total Metrics: {}", stats.total_metrics);
    println!("Total Tests: {}", stats.total_tests);
    println!("Successful Tests: {}", stats.successful_tests);
    println!("Success Rate: {:.2}%", stats.success_rate * 100.0);
    println!("Average Execution Time: {:.2}ms", stats.average_execution_time);
    println!("Average Memory Usage: {} bytes", stats.average_memory_usage);
    
    // Get metrics by category
    let metrics = monitor.get_metrics(None).await?;
    let mut category_counts = HashMap::new();
    
    for metric in &metrics {
        *category_counts.entry(metric.category.clone()).or_insert(0) += 1;
    }
    
    println!("\n📈 METRICS BY CATEGORY");
    println!("======================");
    for (category, count) in &category_counts {
        println!("{:?}: {} metrics", category, count);
    }
    
    // Get recent test results
    let recent_tests = monitor.get_test_results(None).await?;
    let recent_tests = recent_tests.into_iter().take(10).collect::<Vec<_>>();
    
    println!("\n🧪 RECENT TEST RESULTS");
    println!("======================");
    for test in &recent_tests {
        println!("{}: {:.2}ms, Success: {}, Memory: {} bytes", 
            test.test_name, test.execution_time_ms, test.success, test.memory_usage_bytes);
    }
    
    // Performance recommendations
    println!("\n💡 PERFORMANCE RECOMMENDATIONS");
    println!("==============================");
    
    if stats.average_execution_time > 100.0 {
        println!("⚠️  High execution time detected. Consider optimizing resource calculations.");
    }
    
    if stats.success_rate < 0.95 {
        println!("⚠️  Low success rate detected. Check error handling and resource validation.");
    }
    
    if stats.average_memory_usage > 1024 * 1024 {
        println!("⚠️  High memory usage detected. Consider implementing memory pooling or caching strategies.");
    }
    
    if stats.total_metrics > 10000 {
        println!("ℹ️  Large number of metrics collected. Consider adjusting collection interval.");
    }
    
    println!("✅ Performance report generated successfully");
    
    Ok(())
}
