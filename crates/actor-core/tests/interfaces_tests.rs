//! Interface Tests
//!
//! This module contains tests for interfaces, traits, and core data structures
//! used throughout the actor-core system.

use actor_core::prelude::*;
use chrono;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_subsystem_metrics_default() {
        let metrics = SubsystemMetrics::default();
        assert_eq!(metrics.contributions_count, 0);
        assert_eq!(metrics.avg_processing_time, 0);
        assert_eq!(metrics.max_processing_time, 0);
        assert_eq!(metrics.error_count, 0);
        assert_eq!(metrics.last_contribution, None);
    }

    #[test]
    fn test_subsystem_metrics_creation() {
        let now = chrono::Utc::now();
        let metrics = SubsystemMetrics {
            contributions_count: 42,
            avg_processing_time: 1000,
            max_processing_time: 5000,
            error_count: 2,
            last_contribution: Some(now),
        };

        assert_eq!(metrics.contributions_count, 42);
        assert_eq!(metrics.avg_processing_time, 1000);
        assert_eq!(metrics.max_processing_time, 5000);
        assert_eq!(metrics.error_count, 2);
        assert_eq!(metrics.last_contribution, Some(now));
    }

    #[test]
    fn test_subsystem_metrics_serialization() {
        let metrics = SubsystemMetrics {
            contributions_count: 10,
            avg_processing_time: 500,
            max_processing_time: 2000,
            error_count: 1,
            last_contribution: Some(chrono::Utc::now()),
        };

        let serialized = serde_json::to_string(&metrics).unwrap();
        let deserialized: SubsystemMetrics = serde_json::from_str(&serialized).unwrap();
        
        assert_eq!(metrics.contributions_count, deserialized.contributions_count);
        assert_eq!(metrics.avg_processing_time, deserialized.avg_processing_time);
        assert_eq!(metrics.max_processing_time, deserialized.max_processing_time);
        assert_eq!(metrics.error_count, deserialized.error_count);
    }

    #[test]
    fn test_aggregator_metrics_default() {
        let metrics = AggregatorMetrics::default();
        assert_eq!(metrics.total_resolutions, 0);
        assert_eq!(metrics.cache_hits, 0);
        assert_eq!(metrics.cache_misses, 0);
        assert_eq!(metrics.avg_resolution_time, 0);
        assert_eq!(metrics.max_resolution_time, 0);
        assert_eq!(metrics.error_count, 0);
        assert_eq!(metrics.active_subsystems, 0);
    }

    #[test]
    fn test_aggregator_metrics_creation() {
        let metrics = AggregatorMetrics {
            total_resolutions: 100,
            cache_hits: 80,
            cache_misses: 20,
            avg_resolution_time: 1500,
            max_resolution_time: 10000,
            error_count: 5,
            active_subsystems: 10,
        };

        assert_eq!(metrics.total_resolutions, 100);
        assert_eq!(metrics.cache_hits, 80);
        assert_eq!(metrics.cache_misses, 20);
        assert_eq!(metrics.avg_resolution_time, 1500);
        assert_eq!(metrics.max_resolution_time, 10000);
        assert_eq!(metrics.error_count, 5);
        assert_eq!(metrics.active_subsystems, 10);
    }

    #[test]
    fn test_aggregator_metrics_serialization() {
        let metrics = AggregatorMetrics {
            total_resolutions: 50,
            cache_hits: 40,
            cache_misses: 10,
            avg_resolution_time: 2000,
            max_resolution_time: 8000,
            error_count: 2,
            active_subsystems: 5,
        };

        let serialized = serde_json::to_string(&metrics).unwrap();
        let deserialized: AggregatorMetrics = serde_json::from_str(&serialized).unwrap();
        
        assert_eq!(metrics.total_resolutions, deserialized.total_resolutions);
        assert_eq!(metrics.cache_hits, deserialized.cache_hits);
        assert_eq!(metrics.cache_misses, deserialized.cache_misses);
        assert_eq!(metrics.avg_resolution_time, deserialized.avg_resolution_time);
        assert_eq!(metrics.max_resolution_time, deserialized.max_resolution_time);
        assert_eq!(metrics.error_count, deserialized.error_count);
        assert_eq!(metrics.active_subsystems, deserialized.active_subsystems);
    }

    #[test]
    fn test_across_layer_policy_variants() {
        let intersect = AcrossLayerPolicy::Intersect;
        let union = AcrossLayerPolicy::Union;
        let prioritized_override = AcrossLayerPolicy::PrioritizedOverride;

        assert_eq!(intersect, AcrossLayerPolicy::Intersect);
        assert_eq!(union, AcrossLayerPolicy::Union);
        assert_eq!(prioritized_override, AcrossLayerPolicy::PrioritizedOverride);
        assert_ne!(intersect, union);
        assert_ne!(union, prioritized_override);
        assert_ne!(intersect, prioritized_override);
    }

    #[test]
    fn test_across_layer_policy_serialization() {
        let policy = AcrossLayerPolicy::Intersect;
        let serialized = serde_json::to_string(&policy).unwrap();
        let deserialized: AcrossLayerPolicy = serde_json::from_str(&serialized).unwrap();
        assert_eq!(policy, deserialized);

        let policy = AcrossLayerPolicy::Union;
        let serialized = serde_json::to_string(&policy).unwrap();
        let deserialized: AcrossLayerPolicy = serde_json::from_str(&serialized).unwrap();
        assert_eq!(policy, deserialized);

        let policy = AcrossLayerPolicy::PrioritizedOverride;
        let serialized = serde_json::to_string(&policy).unwrap();
        let deserialized: AcrossLayerPolicy = serde_json::from_str(&serialized).unwrap();
        assert_eq!(policy, deserialized);
    }

    #[test]
    fn test_cap_statistics_default() {
        let stats = CapStatistics::default();
        assert_eq!(stats.total_calculations, 0);
        assert_eq!(stats.dimensions_with_caps, 0);
        assert_eq!(stats.avg_calculation_time, 0);
        assert_eq!(stats.max_calculation_time, 0);
    }

    #[test]
    fn test_cap_statistics_creation() {
        let stats = CapStatistics {
            total_calculations: 25,
            dimensions_with_caps: 5,
            avg_calculation_time: 300,
            max_calculation_time: 1500,
        };

        assert_eq!(stats.total_calculations, 25);
        assert_eq!(stats.dimensions_with_caps, 5);
        assert_eq!(stats.avg_calculation_time, 300);
        assert_eq!(stats.max_calculation_time, 1500);
    }

    #[test]
    fn test_cap_statistics_serialization() {
        let stats = CapStatistics {
            total_calculations: 15,
            dimensions_with_caps: 3,
            avg_calculation_time: 200,
            max_calculation_time: 1000,
        };

        let serialized = serde_json::to_string(&stats).unwrap();
        let deserialized: CapStatistics = serde_json::from_str(&serialized).unwrap();
        
        assert_eq!(stats.total_calculations, deserialized.total_calculations);
        assert_eq!(stats.dimensions_with_caps, deserialized.dimensions_with_caps);
        assert_eq!(stats.avg_calculation_time, deserialized.avg_calculation_time);
        assert_eq!(stats.max_calculation_time, deserialized.max_calculation_time);
    }

    #[test]
    fn test_merge_rule_creation() {
        let rule = MergeRule {
            use_pipeline: true,
            operator: Operator::Sum,
            clamp_default: Some(Caps {
                min: 0.0,
                max: 100.0,
            }),
        };

        assert!(rule.use_pipeline);
        assert_eq!(rule.operator, Operator::Sum);
        assert!(rule.clamp_default.is_some());
    }

    #[test]
    fn test_merge_rule_serialization() {
        let rule = MergeRule {
            use_pipeline: false,
            operator: Operator::Max,
            clamp_default: None,
        };

        let serialized = serde_json::to_string(&rule).unwrap();
        let deserialized: MergeRule = serde_json::from_str(&serialized).unwrap();
        
        assert_eq!(rule.use_pipeline, deserialized.use_pipeline);
        assert_eq!(rule.operator, deserialized.operator);
        assert_eq!(rule.clamp_default, deserialized.clamp_default);
    }

    #[test]
    fn test_cache_stats_default() {
        let stats = CacheStats::default();
        assert_eq!(stats.hits, 0);
        assert_eq!(stats.misses, 0);
        assert_eq!(stats.sets, 0);
        assert_eq!(stats.deletes, 0);
        assert_eq!(stats.memory_usage, 0);
        assert_eq!(stats.max_memory_usage, 0);
    }

    #[test]
    fn test_cache_stats_creation() {
        let stats = CacheStats {
            hits: 100,
            misses: 20,
            sets: 50,
            deletes: 10,
            memory_usage: 1024,
            max_memory_usage: 2048,
        };

        assert_eq!(stats.hits, 100);
        assert_eq!(stats.misses, 20);
        assert_eq!(stats.sets, 50);
        assert_eq!(stats.deletes, 10);
        assert_eq!(stats.memory_usage, 1024);
        assert_eq!(stats.max_memory_usage, 2048);
    }

    #[test]
    fn test_cache_stats_serialization() {
        let stats = CacheStats {
            hits: 75,
            misses: 15,
            sets: 30,
            deletes: 5,
            memory_usage: 512,
            max_memory_usage: 1024,
        };

        let serialized = serde_json::to_string(&stats).unwrap();
        let deserialized: CacheStats = serde_json::from_str(&serialized).unwrap();
        
        assert_eq!(stats.hits, deserialized.hits);
        assert_eq!(stats.misses, deserialized.misses);
        assert_eq!(stats.sets, deserialized.sets);
        assert_eq!(stats.deletes, deserialized.deletes);
        assert_eq!(stats.memory_usage, deserialized.memory_usage);
        assert_eq!(stats.max_memory_usage, deserialized.max_memory_usage);
    }


    #[test]
    fn test_all_structs_debug() {
        let metrics = SubsystemMetrics::default();
        let aggregator_metrics = AggregatorMetrics::default();
        let cap_stats = CapStatistics::default();
        let cache_stats = CacheStats::default();
        let rule = MergeRule {
            use_pipeline: true,
            operator: Operator::Sum,
            clamp_default: None,
        };

        assert!(format!("{:?}", metrics).contains("SubsystemMetrics"));
        assert!(format!("{:?}", aggregator_metrics).contains("AggregatorMetrics"));
        assert!(format!("{:?}", cap_stats).contains("CapStatistics"));
        assert!(format!("{:?}", cache_stats).contains("CacheStats"));
        assert!(format!("{:?}", rule).contains("MergeRule"));
    }

    #[test]
    fn test_all_structs_clone() {
        let metrics = SubsystemMetrics {
            contributions_count: 10,
            avg_processing_time: 500,
            max_processing_time: 2000,
            error_count: 1,
            last_contribution: Some(chrono::Utc::now()),
        };

        let cloned = metrics.clone();
        assert_eq!(metrics.contributions_count, cloned.contributions_count);
        assert_eq!(metrics.avg_processing_time, cloned.avg_processing_time);
        assert_eq!(metrics.max_processing_time, cloned.max_processing_time);
        assert_eq!(metrics.error_count, cloned.error_count);
        assert_eq!(metrics.last_contribution, cloned.last_contribution);
    }
}
