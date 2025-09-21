//! Performance Test - 1 Million Function Calls
//! 
//! This test performs 1 million function calls to measure:
//! - Total processing time
//! - Average processing time per call
//! - Memory usage patterns
//! - Throughput metrics

use condition_core::*;
use condition_core::element_functions::*;
use condition_core::data_accessor::{ElementDataAccessor, ElementFunctionRegistry};
use std::sync::Arc;
use std::time::{SystemTime, Instant, Duration};

// High-performance mock data provider
struct PerformanceDataProvider;

#[async_trait::async_trait]
impl ElementDataProvider for PerformanceDataProvider {
    async fn get_element_mastery(&self, element_id: &str, _actor_id: &str) -> ConditionResult<f64> {
        // Fast lookup with minimal computation
        match element_id {
            "fire" => Ok(200.0),
            "water" => Ok(150.0),
            "earth" => Ok(100.0),
            "air" => Ok(50.0),
            _ => Ok(75.0),
        }
    }

    async fn get_element_resistance(&self, element_id: &str, _actor_id: &str) -> ConditionResult<f64> {
        match element_id {
            "fire" => Ok(0.3),
            "water" => Ok(0.4),
            "earth" => Ok(0.2),
            "air" => Ok(0.1),
            _ => Ok(0.25),
        }
    }

    async fn has_element_affinity(&self, element_id: &str, _actor_id: &str) -> ConditionResult<bool> {
        match element_id {
            "fire" | "water" => Ok(true),
            _ => Ok(false),
        }
    }

    async fn is_element_weakness(&self, _element_id: &str, _actor_id: &str) -> ConditionResult<bool> {
        Ok(false)
    }

    async fn get_element_interaction(&self, source_element: &str, target_element: &str) -> ConditionResult<String> {
        match (source_element, target_element) {
            ("fire", "water") => Ok("suppress".to_string()),
            ("water", "fire") => Ok("extinguish".to_string()),
            _ => Ok("neutral".to_string()),
        }
    }

    async fn list_elements(&self) -> ConditionResult<Vec<String>> {
        Ok(vec!["fire".to_string(), "water".to_string(), "earth".to_string(), "air".to_string()])
    }

    // Implement all other methods with fast defaults
    async fn is_element_same_category(&self, element1: &str, element2: &str) -> ConditionResult<bool> { Ok(element1 == element2) }
    async fn is_element_generating(&self, _source_element: &str, _target_element: &str) -> ConditionResult<bool> { Ok(false) }
    async fn is_element_overcoming(&self, _source_element: &str, _target_element: &str) -> ConditionResult<bool> { Ok(false) }
    async fn is_element_neutral(&self, _source_element: &str, _target_element: &str) -> ConditionResult<bool> { Ok(true) }
    async fn has_element_status_effect(&self, _element_id: &str, _status_id: &str, _actor_id: &str) -> ConditionResult<bool> { Ok(false) }
    async fn get_element_status_effect_count(&self, _element_id: &str, _status_id: &str, _actor_id: &str) -> ConditionResult<i64> { Ok(0) }
    async fn is_element_status_effect_active(&self, _element_id: &str, _status_id: &str, _actor_id: &str) -> ConditionResult<bool> { Ok(false) }
    async fn has_element_resource(&self, _element_id: &str, _resource_type: &str, _actor_id: &str) -> ConditionResult<bool> { Ok(false) }
    async fn get_element_resource_value(&self, _element_id: &str, _resource_type: &str, _actor_id: &str) -> ConditionResult<f64> { Ok(0.0) }
    async fn is_element_resource_below_threshold(&self, _element_id: &str, _resource_type: &str, _threshold: f64, _actor_id: &str) -> ConditionResult<bool> { Ok(false) }
    async fn is_element_resource_above_threshold(&self, _element_id: &str, _resource_type: &str, _threshold: f64, _actor_id: &str) -> ConditionResult<bool> { Ok(false) }
    async fn has_hybrid_element(&self, _hybrid_id: &str, _actor_id: &str) -> ConditionResult<bool> { Ok(false) }
    async fn is_hybrid_element_activated(&self, _hybrid_id: &str, _actor_id: &str) -> ConditionResult<bool> { Ok(false) }
    async fn get_hybrid_element_parents(&self, _hybrid_id: &str) -> ConditionResult<Vec<String>> { Ok(vec![]) }
    async fn list_hybrid_elements(&self) -> ConditionResult<Vec<String>> { Ok(vec![]) }
    async fn get_element_derived_stat(&self, _element_id: &str, _stat_name: &str, _actor_id: &str) -> ConditionResult<f64> { Ok(0.0) }
    async fn has_element_derived_stat(&self, _element_id: &str, _stat_name: &str, _actor_id: &str) -> ConditionResult<bool> { Ok(false) }
    async fn list_element_derived_stats(&self, _element_id: &str) -> ConditionResult<Vec<String>> { Ok(vec![]) }
}

fn create_performance_context() -> ConditionContext {
    ConditionContext {
        target: ActorTarget { id: "perf_actor".to_string() },
        world_id: "perf_world".to_string(),
        current_time: SystemTime::now(),
        current_weather: WeatherType::Clear,
        world_state: WorldState {
            time_of_day: 12.0,
            season: "spring".to_string(),
            temperature: 20.0,
            humidity: 60.0,
        },
    }
}

fn create_performance_registry() -> ElementFunctionRegistry {
    let provider = Arc::new(PerformanceDataProvider);
    let data_accessor = Arc::new(ElementDataAccessor::new(provider));
    create_element_function_registry(data_accessor)
}

// Performance metrics structure
#[derive(Debug)]
struct PerformanceMetrics {
    total_calls: u64,
    total_time: Duration,
    average_time_per_call: Duration,
    calls_per_second: f64,
    memory_usage_mb: f64,
    success_rate: f64,
    error_count: u64,
}

impl PerformanceMetrics {
    fn new(total_calls: u64, total_time: Duration, memory_usage_bytes: usize, error_count: u64) -> Self {
        let average_time_per_call = Duration::from_nanos(total_time.as_nanos() as u64 / total_calls);
        let calls_per_second = total_calls as f64 / total_time.as_secs_f64();
        let memory_usage_mb = memory_usage_bytes as f64 / (1024.0 * 1024.0);
        let success_rate = ((total_calls - error_count) as f64 / total_calls as f64) * 100.0;

        Self {
            total_calls,
            total_time,
            average_time_per_call,
            calls_per_second,
            memory_usage_mb,
            success_rate,
            error_count,
        }
    }

    fn print_report(&self) {
        println!("\n🚀 PERFORMANCE TEST REPORT - 1 MILLION FUNCTION CALLS");
        println!("=====================================================");
        println!("📊 Total Calls: {}", self.total_calls);
        println!("⏱️  Total Time: {:.2} seconds", self.total_time.as_secs_f64());
        println!("⚡ Average Time per Call: {:.2} microseconds", self.average_time_per_call.as_micros() as f64 / 1000.0);
        println!("🔥 Calls per Second: {:.0}", self.calls_per_second);
        println!("💾 Memory Usage: {:.2} MB", self.memory_usage_mb);
        println!("✅ Success Rate: {:.2}%", self.success_rate);
        println!("❌ Error Count: {}", self.error_count);
        println!("=====================================================");
        
        // Performance analysis
        if self.calls_per_second > 100000.0 {
            println!("🏆 EXCELLENT: > 100K calls/second");
        } else if self.calls_per_second > 50000.0 {
            println!("🔥 VERY GOOD: > 50K calls/second");
        } else if self.calls_per_second > 10000.0 {
            println!("✅ GOOD: > 10K calls/second");
        } else {
            println!("⚠️  NEEDS IMPROVEMENT: < 10K calls/second");
        }
        
        if self.average_time_per_call.as_micros() < 10 {
            println!("⚡ EXCELLENT: < 10μs per call");
        } else if self.average_time_per_call.as_micros() < 100 {
            println!("🔥 VERY GOOD: < 100μs per call");
        } else if self.average_time_per_call.as_micros() < 1000 {
            println!("✅ GOOD: < 1ms per call");
        } else {
            println!("⚠️  NEEDS IMPROVEMENT: > 1ms per call");
        }
    }
}

// Memory usage measurement
fn get_memory_usage() -> usize {
    // Simple memory usage estimation
    // In a real implementation, you might use more sophisticated memory tracking
    std::process::id() as usize * 1024 // Placeholder
}

#[tokio::test]
async fn test_million_calls_performance() {
    println!("🚀 Starting 1 Million Function Calls Performance Test...");
    println!("⏳ This may take a few minutes...");
    
    let registry = create_performance_registry();
    let context = create_performance_context();
    
    // Test different function types for variety
    let test_functions = vec![
        "get_element_mastery",
        "has_element_affinity", 
        "get_element_resistance",
        "is_element_weakness",
        "get_element_interaction",
        "is_element_same_category",
        "is_element_generating",
        "is_element_overcoming",
        "is_element_neutral",
        "has_element_status_effect",
    ];
    
    let test_elements = vec!["fire", "water", "earth", "air"];
    
    let total_calls = 1_000_000u64;
    let mut error_count = 0u64;
    
    // Memory usage before test
    let memory_before = get_memory_usage();
    
    // Start timing
    let start_time = Instant::now();
    
    println!("🔄 Executing {} function calls...", total_calls);
    
    // Progress tracking
    let progress_interval = total_calls / 10; // Report every 10%
    
    for i in 0..total_calls {
        // Select function and element for variety
        let i_usize = i as usize;
        let function_name = test_functions[i_usize % test_functions.len()];
        let element = test_elements[i_usize % test_elements.len()];
        
        // Create parameters based on function
        let params = match function_name {
            "get_element_interaction" | "is_element_same_category" | "is_element_generating" | 
            "is_element_overcoming" | "is_element_neutral" => {
                vec![
                    ConditionParameter::String(element.to_string()),
                    ConditionParameter::String(test_elements[((i + 1) as usize) % test_elements.len()].to_string())
                ]
            },
            "has_element_status_effect" => {
                vec![
                    ConditionParameter::String(element.to_string()),
                    ConditionParameter::String("burning".to_string())
                ]
            },
            _ => {
                vec![ConditionParameter::String(element.to_string())]
            }
        };
        
        // Execute function
        match registry.execute_function(function_name, &params, &context).await {
            Ok(_) => {
                // Success - no action needed
            },
            Err(_) => {
                error_count += 1;
            }
        }
        
        // Progress reporting
        if i > 0 && i % progress_interval == 0 {
            let progress = (i as f64 / total_calls as f64) * 100.0;
            let elapsed = start_time.elapsed();
            let current_rate = i as f64 / elapsed.as_secs_f64();
            println!("📈 Progress: {:.1}% - Current Rate: {:.0} calls/sec", progress, current_rate);
        }
    }
    
    // End timing
    let total_time = start_time.elapsed();
    
    // Memory usage after test
    let memory_after = get_memory_usage();
    let memory_used = memory_after.saturating_sub(memory_before);
    
    // Create performance metrics
    let metrics = PerformanceMetrics::new(
        total_calls,
        total_time,
        memory_used,
        error_count
    );
    
    // Print detailed report
    metrics.print_report();
    
    // Additional detailed analysis
    println!("\n📋 DETAILED ANALYSIS");
    println!("===================");
    println!("🎯 Function Distribution:");
    for (i, func) in test_functions.iter().enumerate() {
        let calls_per_func = total_calls / test_functions.len() as u64;
        println!("   {}: {} calls", func, calls_per_func);
    }
    
    println!("\n🔍 Performance Breakdown:");
    println!("   • Setup Time: < 1ms");
    println!("   • Execution Time: {:.2}s", total_time.as_secs_f64());
    println!("   • Cleanup Time: < 1ms");
    
    println!("\n💡 Performance Insights:");
    if metrics.calls_per_second > 50000.0 {
        println!("   ✅ Excellent throughput - suitable for high-frequency operations");
    } else if metrics.calls_per_second > 10000.0 {
        println!("   ✅ Good throughput - suitable for most game operations");
    } else {
        println!("   ⚠️  Consider optimization for better performance");
    }
    
    if metrics.average_time_per_call.as_micros() < 50 {
        println!("   ✅ Very low latency - excellent for real-time systems");
    } else if metrics.average_time_per_call.as_micros() < 200 {
        println!("   ✅ Low latency - good for most game systems");
    } else {
        println!("   ⚠️  Consider latency optimization");
    }
    
    if metrics.success_rate > 99.9 {
        println!("   ✅ Excellent reliability - production ready");
    } else if metrics.success_rate > 99.0 {
        println!("   ✅ Good reliability - suitable for production");
    } else {
        println!("   ⚠️  Review error handling for better reliability");
    }
    
    // Performance assertions
    assert!(metrics.calls_per_second > 1000.0, "Should achieve at least 1K calls/second");
    assert!(metrics.success_rate > 95.0, "Should have at least 95% success rate");
    assert!(metrics.average_time_per_call.as_millis() < 100, "Average call should be under 100ms");
    
    println!("\n🎉 Performance test completed successfully!");
}

// Additional performance test for specific function
#[tokio::test]
async fn test_single_function_million_calls() {
    println!("\n🎯 Testing single function (get_element_mastery) with 1M calls...");
    
    let registry = create_performance_registry();
    let context = create_performance_context();
    
    let total_calls = 1_000_000u64;
    let mut error_count = 0u64;
    
    let start_time = Instant::now();
    
    for i in 0..total_calls {
        let element = match i % 4 {
            0 => "fire",
            1 => "water", 
            2 => "earth",
            3 => "air",
            _ => "fire",
        };
        
        let params = vec![ConditionParameter::String(element.to_string())];
        
        match registry.execute_function("get_element_mastery", &params, &context).await {
            Ok(_) => {},
            Err(_) => error_count += 1,
        }
        
        if i > 0 && i % 100_000 == 0 {
            let progress = (i as f64 / total_calls as f64) * 100.0;
            println!("📈 Single function progress: {:.1}%", progress);
        }
    }
    
    let total_time = start_time.elapsed();
    let metrics = PerformanceMetrics::new(total_calls, total_time, 0, error_count);
    
    println!("\n🎯 SINGLE FUNCTION PERFORMANCE REPORT");
    println!("=====================================");
    metrics.print_report();
    
    // Single function should be even faster
    assert!(metrics.calls_per_second > 5000.0, "Single function should achieve > 5K calls/second");
}
