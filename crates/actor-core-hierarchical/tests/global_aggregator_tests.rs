//! # Global Aggregator Tests
//! 
//! Integration tests for the global aggregator functionality.

use actor_core_hierarchical::{GlobalAggregator, HierarchicalActor, AggregationStrategy};
use chrono::Utc;

#[test]
fn test_global_aggregator_creation() {
    let aggregator = GlobalAggregator::new();
    assert!(!aggregator.aggregation_strategies.is_empty());
    assert_eq!(aggregator.aggregation_cache.len(), 0);
}

#[test]
fn test_aggregation_strategies() {
    let mut aggregator = GlobalAggregator::new();
    
    // Test Sum strategy
    let sum_strategy = AggregationStrategy::Sum;
    let values = vec![10.0, 20.0, 30.0];
    let result = aggregator.apply_aggregation_strategy(&sum_strategy, &values);
    assert_eq!(result, 60.0);
    
    // Test Max strategy
    let max_strategy = AggregationStrategy::Max;
    let result = aggregator.apply_aggregation_strategy(&max_strategy, &values);
    assert_eq!(result, 30.0);
    
    // Test Min strategy
    let min_strategy = AggregationStrategy::Min;
    let result = aggregator.apply_aggregation_strategy(&min_strategy, &values);
    assert_eq!(result, 10.0);
    
    // Test Average strategy
    let avg_strategy = AggregationStrategy::Average;
    let result = aggregator.apply_aggregation_strategy(&avg_strategy, &values);
    assert_eq!(result, 20.0);
    
    // Test Multiply strategy
    let mul_strategy = AggregationStrategy::Multiply;
    let result = aggregator.apply_aggregation_strategy(&mul_strategy, &values);
    assert_eq!(result, 6000.0);
}

#[test]
fn test_actor_stats_aggregation() {
    let mut aggregator = GlobalAggregator::new();
    let mut actor = HierarchicalActor::new();
    
    // Add system contributions
    let contribution1 = actor_core_hierarchical::SystemContribution {
        system_name: "elemental".to_string(),
        stat_name: "health".to_string(),
        value: 100.0,
        priority: 1,
        timestamp: Utc::now(),
    };
    
    let contribution2 = actor_core_hierarchical::SystemContribution {
        system_name: "cultivation".to_string(),
        stat_name: "health".to_string(),
        value: 50.0,
        priority: 2,
        timestamp: Utc::now(),
    };
    
    actor.add_system_contribution(contribution1);
    actor.add_system_contribution(contribution2);
    
    // Aggregate stats
    let stats = aggregator.aggregate_actor_stats(&actor);
    
    // Health should be summed (100 + 50 = 150)
    assert_eq!(stats.get("health").unwrap(), &150.0);
}

#[test]
fn test_cache_operations() {
    let mut aggregator = GlobalAggregator::new();
    let actor = HierarchicalActor::new();
    
    // First aggregation should populate cache
    let _stats1 = aggregator.aggregate_actor_stats(&actor);
    assert_eq!(aggregator.aggregation_cache.len(), 1);
    
    // Second aggregation should use cache
    let _stats2 = aggregator.aggregate_actor_stats(&actor);
    assert_eq!(aggregator.aggregation_cache.len(), 1);
    
    // Invalidate cache
    aggregator.invalidate_actor_cache(actor.get_id());
    assert_eq!(aggregator.aggregation_cache.len(), 0);
}

#[test]
fn test_custom_aggregation_strategy() {
    let mut aggregator = GlobalAggregator::new();
    
    // Set custom strategy for a stat
    let custom_strategy = AggregationStrategy::Custom(|values| {
        values.iter().sum::<f64>() * 2.0
    });
    
    aggregator.set_aggregation_strategy("custom_stat".to_string(), custom_strategy);
    
    let strategy = aggregator.get_aggregation_strategy("custom_stat");
    let values = vec![10.0, 20.0];
    let result = aggregator.apply_aggregation_strategy(&strategy, &values);
    
    // Custom function: (10 + 20) * 2 = 60
    assert_eq!(result, 60.0);
}

#[test]
fn test_multiple_stats_aggregation() {
    let mut aggregator = GlobalAggregator::new();
    let mut actor = HierarchicalActor::new();
    
    // Add multiple contributions for different stats
    let health_contributions = vec![
        actor_core_hierarchical::SystemContribution {
            system_name: "race".to_string(),
            stat_name: "health".to_string(),
            value: 100.0,
            priority: 1,
            timestamp: Utc::now(),
        },
        actor_core_hierarchical::SystemContribution {
            system_name: "class".to_string(),
            stat_name: "health".to_string(),
            value: 50.0,
            priority: 2,
            timestamp: Utc::now(),
        },
    ];
    
    let mana_contributions = vec![
        actor_core_hierarchical::SystemContribution {
            system_name: "race".to_string(),
            stat_name: "mana".to_string(),
            value: 80.0,
            priority: 1,
            timestamp: Utc::now(),
        },
        actor_core_hierarchical::SystemContribution {
            system_name: "class".to_string(),
            stat_name: "mana".to_string(),
            value: 30.0,
            priority: 2,
            timestamp: Utc::now(),
        },
    ];
    
    for contribution in health_contributions {
        actor.add_system_contribution(contribution);
    }
    
    for contribution in mana_contributions {
        actor.add_system_contribution(contribution);
    }
    
    let stats = aggregator.aggregate_actor_stats(&actor);
    
    // Both stats should be summed
    assert_eq!(stats.get("health").unwrap(), &150.0); // 100 + 50
    assert_eq!(stats.get("mana").unwrap(), &110.0);   // 80 + 30
}

#[test]
fn test_cache_stats() {
    let mut aggregator = GlobalAggregator::new();
    let actor1 = HierarchicalActor::new();
    let actor2 = HierarchicalActor::new();
    
    // Aggregate stats for two actors
    let _stats1 = aggregator.aggregate_actor_stats(&actor1);
    let _stats2 = aggregator.aggregate_actor_stats(&actor2);
    
    let cache_stats = aggregator.get_cache_stats();
    assert_eq!(cache_stats.cached_actors, 2);
}
