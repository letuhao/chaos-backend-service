//! Performance SIMD coverage tests for Actor Core.

use actor_core::performance::simd::{
    SimdConfig, SimdOptimizer, SimdStats, SimdResult,
    SimdCacheOptimizer, SimdAggregationOptimizer
};
use actor_core::types::{Contribution, CapContribution};
use actor_core::enums::{Bucket, CapMode};
use std::time::Duration;

#[test]
fn test_simd_config_default() {
    let config = SimdConfig::default();
    
    assert!(config.enable_simd);
    assert!(config.enable_crc32_simd);
    assert!(config.enable_hash_simd);
    assert!(config.enable_memcpy_simd);
    assert!(config.enable_cmp_simd);
    assert_eq!(config.batch_size, 1024);
    assert_eq!(config.min_data_size, 64);
    assert!(config.max_concurrency > 0);
}

#[test]
fn test_simd_config_creation() {
    let config = SimdConfig {
        enable_simd: false,
        enable_crc32_simd: false,
        enable_hash_simd: false,
        enable_memcpy_simd: false,
        enable_cmp_simd: false,
        batch_size: 512,
        min_data_size: 128,
        max_concurrency: 4,
    };
    
    assert!(!config.enable_simd);
    assert!(!config.enable_crc32_simd);
    assert!(!config.enable_hash_simd);
    assert!(!config.enable_memcpy_simd);
    assert!(!config.enable_cmp_simd);
    assert_eq!(config.batch_size, 512);
    assert_eq!(config.min_data_size, 128);
    assert_eq!(config.max_concurrency, 4);
}

#[test]
fn test_simd_config_clone() {
    let config = SimdConfig::default();
    let cloned = config.clone();
    
    assert_eq!(cloned.enable_simd, config.enable_simd);
    assert_eq!(cloned.enable_crc32_simd, config.enable_crc32_simd);
    assert_eq!(cloned.enable_hash_simd, config.enable_hash_simd);
    assert_eq!(cloned.enable_memcpy_simd, config.enable_memcpy_simd);
    assert_eq!(cloned.enable_cmp_simd, config.enable_cmp_simd);
    assert_eq!(cloned.batch_size, config.batch_size);
    assert_eq!(cloned.min_data_size, config.min_data_size);
    assert_eq!(cloned.max_concurrency, config.max_concurrency);
}

#[test]
fn test_simd_config_debug() {
    let config = SimdConfig::default();
    let debug_str = format!("{:?}", config);
    
    assert!(debug_str.contains("SimdConfig"));
    assert!(debug_str.contains("enable_simd: true"));
    assert!(debug_str.contains("batch_size: 1024"));
}

#[test]
fn test_simd_stats_default() {
    let stats = SimdStats::default();
    
    assert_eq!(stats.total_operations, 0);
    assert_eq!(stats.simd_operations, 0);
    assert_eq!(stats.fallback_operations, 0);
    assert_eq!(stats.total_simd_time, Duration::from_secs(0));
    assert_eq!(stats.total_fallback_time, Duration::from_secs(0));
    assert_eq!(stats.avg_simd_time, Duration::from_secs(0));
    assert_eq!(stats.avg_fallback_time, Duration::from_secs(0));
    assert_eq!(stats.simd_utilization, 0.0);
}

#[test]
fn test_simd_stats_creation() {
    let stats = SimdStats {
        total_operations: 1000,
        simd_operations: 800,
        fallback_operations: 200,
        total_simd_time: Duration::from_millis(100),
        total_fallback_time: Duration::from_millis(50),
        avg_simd_time: Duration::from_micros(125),
        avg_fallback_time: Duration::from_micros(250),
        simd_utilization: 0.8,
    };
    
    assert_eq!(stats.total_operations, 1000);
    assert_eq!(stats.simd_operations, 800);
    assert_eq!(stats.fallback_operations, 200);
    assert_eq!(stats.total_simd_time, Duration::from_millis(100));
    assert_eq!(stats.total_fallback_time, Duration::from_millis(50));
    assert_eq!(stats.avg_simd_time, Duration::from_micros(125));
    assert_eq!(stats.avg_fallback_time, Duration::from_micros(250));
    assert_eq!(stats.simd_utilization, 0.8);
}

#[test]
fn test_simd_stats_clone() {
    let stats = SimdStats {
        total_operations: 1000,
        simd_operations: 800,
        fallback_operations: 200,
        total_simd_time: Duration::from_millis(100),
        total_fallback_time: Duration::from_millis(50),
        avg_simd_time: Duration::from_micros(125),
        avg_fallback_time: Duration::from_micros(250),
        simd_utilization: 0.8,
    };
    
    let cloned = stats.clone();
    assert_eq!(cloned.total_operations, stats.total_operations);
    assert_eq!(cloned.simd_operations, stats.simd_operations);
    assert_eq!(cloned.fallback_operations, stats.fallback_operations);
    assert_eq!(cloned.total_simd_time, stats.total_simd_time);
    assert_eq!(cloned.total_fallback_time, stats.total_fallback_time);
    assert_eq!(cloned.avg_simd_time, stats.avg_simd_time);
    assert_eq!(cloned.avg_fallback_time, stats.avg_fallback_time);
    assert_eq!(cloned.simd_utilization, stats.simd_utilization);
}

#[test]
fn test_simd_stats_debug() {
    let stats = SimdStats {
        total_operations: 1000,
        simd_operations: 800,
        fallback_operations: 200,
        total_simd_time: Duration::from_millis(100),
        total_fallback_time: Duration::from_millis(50),
        avg_simd_time: Duration::from_micros(125),
        avg_fallback_time: Duration::from_micros(250),
        simd_utilization: 0.8,
    };
    
    let debug_str = format!("{:?}", stats);
    assert!(debug_str.contains("SimdStats"));
    assert!(debug_str.contains("total_operations: 1000"));
    assert!(debug_str.contains("simd_utilization: 0.8"));
}

#[test]
fn test_simd_result_creation() {
    let result = SimdResult {
        value: 12345,
        is_simd: true,
    };
    
    assert_eq!(result.value, 12345);
    assert!(result.is_simd);
}

#[test]
fn test_simd_result_clone() {
    let result = SimdResult {
        value: 12345,
        is_simd: true,
    };
    
    let cloned = result.clone();
    assert_eq!(cloned.value, result.value);
    assert_eq!(cloned.is_simd, result.is_simd);
}

#[test]
fn test_simd_result_debug() {
    let result = SimdResult {
        value: 12345,
        is_simd: true,
    };
    
    let debug_str = format!("{:?}", result);
    assert!(debug_str.contains("SimdResult"));
    assert!(debug_str.contains("value: 12345"));
    assert!(debug_str.contains("is_simd: true"));
}

#[test]
fn test_simd_optimizer_new() {
    let config = SimdConfig::default();
    let optimizer = SimdOptimizer::new(config);
    
    let stats = optimizer.get_stats();
    assert_eq!(stats.total_operations, 0);
    assert_eq!(stats.simd_operations, 0);
    assert_eq!(stats.fallback_operations, 0);
}

#[test]
fn test_simd_optimizer_new_default() {
    let optimizer = SimdOptimizer::new_default();
    
    let stats = optimizer.get_stats();
    assert_eq!(stats.total_operations, 0);
    assert_eq!(stats.simd_operations, 0);
    assert_eq!(stats.fallback_operations, 0);
}

#[test]
fn test_simd_optimizer_crc32() {
    let optimizer = SimdOptimizer::new_default();
    let data = b"test data for crc32";
    
    let result = optimizer.crc32(data);
    assert!(result > 0);
    
    let stats = optimizer.get_stats();
    assert_eq!(stats.total_operations, 1);
    assert!(stats.simd_operations > 0 || stats.fallback_operations > 0);
}

#[test]
fn test_simd_optimizer_crc32_small_data() {
    let config = SimdConfig {
        enable_simd: true,
        enable_crc32_simd: true,
        enable_hash_simd: true,
        enable_memcpy_simd: true,
        enable_cmp_simd: true,
        batch_size: 1024,
        min_data_size: 100, // Larger than test data
        max_concurrency: 4,
    };
    let optimizer = SimdOptimizer::new(config);
    let data = b"small"; // Less than min_data_size
    
    let result = optimizer.crc32(data);
    assert!(result > 0);
    
    let stats = optimizer.get_stats();
    assert_eq!(stats.total_operations, 1);
    assert_eq!(stats.fallback_operations, 1);
    assert_eq!(stats.simd_operations, 0);
}

#[test]
fn test_simd_optimizer_hash() {
    let optimizer = SimdOptimizer::new_default();
    let data = b"test data for hash";
    
    let result = optimizer.hash(data);
    assert!(result > 0);
    
    let stats = optimizer.get_stats();
    assert_eq!(stats.total_operations, 1);
    assert!(stats.simd_operations > 0 || stats.fallback_operations > 0);
}

#[test]
fn test_simd_optimizer_memcpy() {
    let optimizer = SimdOptimizer::new_default();
    let src = b"test data for memcpy";
    let mut dst = vec![0u8; src.len()];
    
    let result = optimizer.memcpy(&mut dst, src);
    assert!(result.is_ok());
    assert_eq!(dst, src);
    
    let stats = optimizer.get_stats();
    assert_eq!(stats.total_operations, 1);
    assert!(stats.simd_operations > 0 || stats.fallback_operations > 0);
}

#[test]
fn test_simd_optimizer_memcpy_length_mismatch() {
    let optimizer = SimdOptimizer::new_default();
    let src = b"test data";
    let mut dst = vec![0u8; 5]; // Different length
    
    let result = optimizer.memcpy(&mut dst, src);
    assert!(result.is_err());
    
    let stats = optimizer.get_stats();
    assert_eq!(stats.total_operations, 0); // No operation performed due to error
}

#[test]
fn test_simd_optimizer_memcmp() {
    let optimizer = SimdOptimizer::new_default();
    let a = b"test data";
    let b = b"test data";
    
    let result = optimizer.memcmp(a, b);
    assert_eq!(result, 0); // Equal
    
    let stats = optimizer.get_stats();
    assert_eq!(stats.total_operations, 1);
    assert!(stats.simd_operations > 0 || stats.fallback_operations > 0);
}

#[test]
fn test_simd_optimizer_memcmp_different_lengths() {
    let optimizer = SimdOptimizer::new_default();
    let a = b"short";
    let b = b"longer data";
    
    let result = optimizer.memcmp(a, b);
    assert_eq!(result, -1); // a is shorter than b
    
    let stats = optimizer.get_stats();
    assert_eq!(stats.total_operations, 0); // No operation performed due to length mismatch
}

#[test]
fn test_simd_optimizer_process_batch() {
    let optimizer = SimdOptimizer::new_default();
    let items = vec![1, 2, 3, 4, 5];
    
    let results = optimizer.process_batch(&items, |&x| SimdResult {
        value: x as u64 * 2,
        is_simd: true,
    }).unwrap();
    
    assert_eq!(results.len(), 5);
    assert_eq!(results[0].value, 2);
    assert_eq!(results[1].value, 4);
    assert_eq!(results[2].value, 6);
    assert_eq!(results[3].value, 8);
    assert_eq!(results[4].value, 10);
    
    let stats = optimizer.get_stats();
    assert_eq!(stats.total_operations, 1); // One batch operation
}

#[test]
fn test_simd_optimizer_process_batch_empty() {
    let optimizer = SimdOptimizer::new_default();
    let items: Vec<i32> = vec![];
    
    let results = optimizer.process_batch(&items, |&x| SimdResult {
        value: x as u64 * 2,
        is_simd: true,
    }).unwrap();
    
    assert_eq!(results.len(), 0);
    
    let stats = optimizer.get_stats();
    assert_eq!(stats.total_operations, 0);
}

#[test]
fn test_simd_optimizer_reset_stats() {
    let optimizer = SimdOptimizer::new_default();
    let data = b"test data";
    
    // Perform some operations
    optimizer.crc32(data);
    optimizer.hash(data);
    
    let stats_before = optimizer.get_stats();
    assert!(stats_before.total_operations > 0);
    
    // Reset stats
    optimizer.reset_stats();
    
    let stats_after = optimizer.get_stats();
    assert_eq!(stats_after.total_operations, 0);
    assert_eq!(stats_after.simd_operations, 0);
    assert_eq!(stats_after.fallback_operations, 0);
}

#[test]
fn test_simd_cache_optimizer_new() {
    let config = SimdConfig::default();
    let optimizer = SimdCacheOptimizer::new(config);
    
    let stats = optimizer.get_stats();
    assert_eq!(stats.total_operations, 0);
}

#[test]
fn test_simd_cache_optimizer_optimize_cache_key() {
    let optimizer = SimdCacheOptimizer::new(SimdConfig::default());
    let key = "test_cache_key";
    
    let result = optimizer.optimize_cache_key(key);
    assert!(result > 0);
    
    let stats = optimizer.get_stats();
    assert_eq!(stats.total_operations, 1);
}

#[test]
fn test_simd_cache_optimizer_optimize_serialization() {
    let optimizer = SimdCacheOptimizer::new(SimdConfig::default());
    let data = b"test data for serialization";
    
    let result = optimizer.optimize_serialization(data).unwrap();
    assert_eq!(result, data);
    
    let stats = optimizer.get_stats();
    assert_eq!(stats.total_operations, 0); // No SIMD operations performed
}

#[test]
fn test_simd_cache_optimizer_optimize_deserialization() {
    let optimizer = SimdCacheOptimizer::new(SimdConfig::default());
    let data = b"test data for deserialization";
    
    let result = optimizer.optimize_deserialization(data).unwrap();
    assert_eq!(result, data);
    
    let stats = optimizer.get_stats();
    assert_eq!(stats.total_operations, 0); // No SIMD operations performed
}

#[test]
fn test_simd_cache_optimizer_optimize_comparison() {
    let optimizer = SimdCacheOptimizer::new(SimdConfig::default());
    let a = b"test data a";
    let b = b"test data b";
    
    let result = optimizer.optimize_comparison(a, b);
    assert!(result != 0); // Not equal
    
    let stats = optimizer.get_stats();
    assert_eq!(stats.total_operations, 1);
}

#[test]
fn test_simd_aggregation_optimizer_new() {
    let config = SimdConfig::default();
    let optimizer = SimdAggregationOptimizer::new(config);
    
    let stats = optimizer.get_stats();
    assert_eq!(stats.total_operations, 0);
}

#[test]
fn test_simd_aggregation_optimizer_optimize_contribution_aggregation_empty() {
    let optimizer = SimdAggregationOptimizer::new(SimdConfig::default());
    let contributions: Vec<Contribution> = vec![];
    
    let result = optimizer.optimize_contribution_aggregation(&contributions).unwrap();
    assert_eq!(result, 0.0);
    
    let stats = optimizer.get_stats();
    assert_eq!(stats.total_operations, 0);
}

#[test]
fn test_simd_aggregation_optimizer_optimize_contribution_aggregation_flat() {
    let optimizer = SimdAggregationOptimizer::new(SimdConfig::default());
    let contributions = vec![
        Contribution { dimension: "test".to_string(), bucket: Bucket::Flat, value: 10.0, system: "test_system".to_string(), priority: None, tags: None },
        Contribution { dimension: "test".to_string(), bucket: Bucket::Flat, value: 20.0, system: "test_system".to_string(), priority: None, tags: None },
        Contribution { dimension: "test".to_string(), bucket: Bucket::Flat, value: 30.0, system: "test_system".to_string(), priority: None, tags: None },
    ];
    
    let result = optimizer.optimize_contribution_aggregation(&contributions).unwrap();
    assert_eq!(result, 60.0); // 10 + 20 + 30
    
    let stats = optimizer.get_stats();
    assert_eq!(stats.total_operations, 0); // No SIMD operations performed
}

#[test]
fn test_simd_aggregation_optimizer_optimize_contribution_aggregation_mult() {
    let optimizer = SimdAggregationOptimizer::new(SimdConfig::default());
    let contributions = vec![
        Contribution { dimension: "test".to_string(), bucket: Bucket::Mult, value: 2.0, system: "test_system".to_string(), priority: None, tags: None },
        Contribution { dimension: "test".to_string(), bucket: Bucket::Mult, value: 3.0, system: "test_system".to_string(), priority: None, tags: None },
        Contribution { dimension: "test".to_string(), bucket: Bucket::Mult, value: 4.0, system: "test_system".to_string(), priority: None, tags: None },
    ];
    
    let result = optimizer.optimize_contribution_aggregation(&contributions).unwrap();
    assert_eq!(result, 0.0); // 0 * 2 * 3 * 4 = 0
    
    let stats = optimizer.get_stats();
    assert_eq!(stats.total_operations, 0); // No SIMD operations performed
}

#[test]
fn test_simd_aggregation_optimizer_optimize_contribution_aggregation_override() {
    let optimizer = SimdAggregationOptimizer::new(SimdConfig::default());
    let contributions = vec![
        Contribution { dimension: "test".to_string(), bucket: Bucket::Override, value: 10.0, system: "test_system".to_string(), priority: None, tags: None },
        Contribution { dimension: "test".to_string(), bucket: Bucket::Override, value: 20.0, system: "test_system".to_string(), priority: None, tags: None },
        Contribution { dimension: "test".to_string(), bucket: Bucket::Override, value: 30.0, system: "test_system".to_string(), priority: None, tags: None },
    ];
    
    let result = optimizer.optimize_contribution_aggregation(&contributions).unwrap();
    assert_eq!(result, 30.0); // Last override value
    
    let stats = optimizer.get_stats();
    assert_eq!(stats.total_operations, 0); // No SIMD operations performed
}

#[test]
fn test_simd_aggregation_optimizer_optimize_cap_calculation_empty() {
    let optimizer = SimdAggregationOptimizer::new(SimdConfig::default());
    let caps: Vec<CapContribution> = vec![];
    
    let result = optimizer.optimize_cap_calculation(&caps).unwrap();
    assert_eq!(result.min, f64::NEG_INFINITY);
    assert_eq!(result.max, f64::INFINITY);
    
    let stats = optimizer.get_stats();
    assert_eq!(stats.total_operations, 0);
}

#[test]
fn test_simd_aggregation_optimizer_optimize_cap_calculation_baseline() {
    let optimizer = SimdAggregationOptimizer::new(SimdConfig::default());
    let caps = vec![
        CapContribution { system: "test_system".to_string(), dimension: "test".to_string(), mode: CapMode::Baseline, kind: "min".to_string(), value: 10.0, priority: None, scope: None, realm: None, tags: None },
        CapContribution { system: "test_system".to_string(), dimension: "test".to_string(), mode: CapMode::Baseline, kind: "max".to_string(), value: 100.0, priority: None, scope: None, realm: None, tags: None },
    ];
    
    let result = optimizer.optimize_cap_calculation(&caps).unwrap();
    assert_eq!(result.min, 10.0);
    assert_eq!(result.max, 100.0);
    
    let stats = optimizer.get_stats();
    assert_eq!(stats.total_operations, 0); // No SIMD operations performed
}

#[test]
fn test_simd_aggregation_optimizer_optimize_cap_calculation_additive() {
    let optimizer = SimdAggregationOptimizer::new(SimdConfig::default());
    let caps = vec![
        CapContribution { system: "test_system".to_string(), dimension: "test".to_string(), mode: CapMode::Additive, kind: "min".to_string(), value: 5.0, priority: None, scope: None, realm: None, tags: None },
        CapContribution { system: "test_system".to_string(), dimension: "test".to_string(), mode: CapMode::Additive, kind: "max".to_string(), value: 10.0, priority: None, scope: None, realm: None, tags: None },
    ];
    
    let result = optimizer.optimize_cap_calculation(&caps).unwrap();
    assert_eq!(result.min, f64::NEG_INFINITY); // No baseline min set
    assert_eq!(result.max, f64::INFINITY); // No baseline max set
    
    let stats = optimizer.get_stats();
    assert_eq!(stats.total_operations, 0); // No SIMD operations performed
}

#[test]
fn test_simd_aggregation_optimizer_optimize_cap_calculation_hard_max() {
    let optimizer = SimdAggregationOptimizer::new(SimdConfig::default());
    let caps = vec![
        CapContribution { system: "test_system".to_string(), dimension: "test".to_string(), mode: CapMode::HardMax, kind: "max".to_string(), value: 50.0, priority: None, scope: None, realm: None, tags: None },
        CapContribution { system: "test_system".to_string(), dimension: "test".to_string(), mode: CapMode::HardMax, kind: "max".to_string(), value: 30.0, priority: None, scope: None, realm: None, tags: None },
    ];
    
    let result = optimizer.optimize_cap_calculation(&caps).unwrap();
    assert_eq!(result.min, f64::NEG_INFINITY);
    assert_eq!(result.max, 30.0); // min(50, 30)
    
    let stats = optimizer.get_stats();
    assert_eq!(stats.total_operations, 0); // No SIMD operations performed
}

#[test]
fn test_simd_aggregation_optimizer_optimize_cap_calculation_hard_min() {
    let optimizer = SimdAggregationOptimizer::new(SimdConfig::default());
    let caps = vec![
        CapContribution { system: "test_system".to_string(), dimension: "test".to_string(), mode: CapMode::HardMin, kind: "min".to_string(), value: 10.0, priority: None, scope: None, realm: None, tags: None },
        CapContribution { system: "test_system".to_string(), dimension: "test".to_string(), mode: CapMode::HardMin, kind: "min".to_string(), value: 20.0, priority: None, scope: None, realm: None, tags: None },
    ];
    
    let result = optimizer.optimize_cap_calculation(&caps).unwrap();
    assert_eq!(result.min, 20.0); // max(10, 20)
    assert_eq!(result.max, f64::INFINITY);
    
    let stats = optimizer.get_stats();
    assert_eq!(stats.total_operations, 0); // No SIMD operations performed
}

#[test]
fn test_simd_aggregation_optimizer_optimize_cap_calculation_override() {
    let optimizer = SimdAggregationOptimizer::new(SimdConfig::default());
    let caps = vec![
        CapContribution { system: "test_system".to_string(), dimension: "test".to_string(), mode: CapMode::Override, kind: "min".to_string(), value: 15.0, priority: None, scope: None, realm: None, tags: None },
        CapContribution { system: "test_system".to_string(), dimension: "test".to_string(), mode: CapMode::Override, kind: "max".to_string(), value: 75.0, priority: None, scope: None, realm: None, tags: None },
    ];
    
    let result = optimizer.optimize_cap_calculation(&caps).unwrap();
    assert_eq!(result.min, 15.0);
    assert_eq!(result.max, 75.0);
    
    let stats = optimizer.get_stats();
    assert_eq!(stats.total_operations, 0); // No SIMD operations performed
}

#[test]
fn test_simd_aggregation_optimizer_optimize_cap_calculation_soft_max() {
    let optimizer = SimdAggregationOptimizer::new(SimdConfig::default());
    let caps = vec![
        CapContribution { system: "test_system".to_string(), dimension: "test".to_string(), mode: CapMode::SoftMax, kind: "max".to_string(), value: 40.0, priority: None, scope: None, realm: None, tags: None },
        CapContribution { system: "test_system".to_string(), dimension: "test".to_string(), mode: CapMode::SoftMax, kind: "max".to_string(), value: 25.0, priority: None, scope: None, realm: None, tags: None },
    ];
    
    let result = optimizer.optimize_cap_calculation(&caps).unwrap();
    assert_eq!(result.min, f64::NEG_INFINITY);
    assert_eq!(result.max, 25.0); // min(40, 25)
    
    let stats = optimizer.get_stats();
    assert_eq!(stats.total_operations, 0); // No SIMD operations performed
}
