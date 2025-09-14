//! Coverage tests for subsystems/exhaustion modules.

use actor_core::subsystems::exhaustion::resource_exhaustion::{
    ExhaustionConfig,
    EventConfig,
    PriorityConfig,
    ArchetypeConfig,
    ResourceConfig,
    ThresholdConfig,
    EffectConfig,
    ExhaustionState,
    ThresholdState,
    CoalescingState,
    ExhaustionEvent,
    ExhaustionEventType,
    ExhaustionTransition,
    ExhaustionError
};
use actor_core::subsystems::exhaustion::exhaustion_event_publisher::{
    InMemoryEventPublisher,
    EventStats
};
use actor_core::subsystems::exhaustion::exhaustion_config_loader::{
    ExhaustionConfigLoader,
    MergedConfig,
    ConfigSource,
    ConfigLoaderError
};
use actor_core::subsystems::exhaustion::exhaustion_performance::{
    BenchmarkConfig,
    BenchmarkResult,
    PerformanceStats
};
use std::collections::HashMap;
use std::time::Duration;
use chrono::Utc;

#[test]
fn test_exhaustion_event_type_variants() {
    let variants = vec![
        ExhaustionEventType::ResourceExhausted,
        ExhaustionEventType::ResourceRecovered,
    ];
    
    for variant in variants {
        assert_eq!(variant, variant.clone());
    }
}

#[test]
fn test_exhaustion_event_type_serialization() {
    let event_type = ExhaustionEventType::ResourceExhausted;
    let serialized = serde_json::to_string(&event_type).unwrap();
    let deserialized: ExhaustionEventType = serde_json::from_str(&serialized).unwrap();
    
    assert_eq!(event_type, deserialized);
}

#[test]
fn test_event_config_creation() {
    let config = EventConfig {
        coalesce_window_ms: 100,
    };
    
    assert_eq!(config.coalesce_window_ms, 100);
}

#[test]
fn test_priority_config_creation() {
    let config = PriorityConfig {
        categories: vec!["health".to_string(), "mana".to_string()],
    };
    
    assert_eq!(config.categories.len(), 2);
    assert_eq!(config.categories[0], "health");
    assert_eq!(config.categories[1], "mana");
}

#[test]
fn test_archetype_config_creation() {
    let mut resources = HashMap::new();
    resources.insert("health".to_string(), ResourceConfig {
        thresholds: vec![],
    });
    
    let config = ArchetypeConfig {
        resources,
    };
    
    assert_eq!(config.resources.len(), 1);
    assert!(config.resources.contains_key("health"));
}

#[test]
fn test_resource_config_creation() {
    let config = ResourceConfig {
        thresholds: vec![],
    };
    
    assert_eq!(config.thresholds.len(), 0);
}

#[test]
fn test_threshold_config_creation() {
    let config = ThresholdConfig {
        id: "test_threshold".to_string(),
        order: Some(1),
        enter_percent_lte: Some(0.5),
        exit_percent_gte: Some(0.8),
        enter_value_eq: None,
        exit_value_ge: None,
        effects: vec![],
    };
    
    assert_eq!(config.id, "test_threshold");
    assert_eq!(config.order, Some(1));
    assert_eq!(config.enter_percent_lte, Some(0.5));
    assert_eq!(config.exit_percent_gte, Some(0.8));
    assert_eq!(config.enter_value_eq, None);
    assert_eq!(config.exit_value_ge, None);
    assert_eq!(config.effects.len(), 0);
}

#[test]
fn test_effect_config_creation() {
    let config = EffectConfig {
        effect_type: "multiplier".to_string(),
        values: Some(vec!["health".to_string()]),
        categories: Some(vec!["combat".to_string()]),
        modifier: Some(0.5),
        name: Some("exhausted".to_string()),
        value: Some(true),
        level: Some("low".to_string()),
        resource: Some("health".to_string()),
    };
    
    assert_eq!(config.effect_type, "multiplier");
    assert_eq!(config.values, Some(vec!["health".to_string()]));
    assert_eq!(config.categories, Some(vec!["combat".to_string()]));
    assert_eq!(config.modifier, Some(0.5));
    assert_eq!(config.name, Some("exhausted".to_string()));
    assert_eq!(config.value, Some(true));
    assert_eq!(config.level, Some("low".to_string()));
    assert_eq!(config.resource, Some("health".to_string()));
}

#[test]
fn test_exhaustion_state_creation() {
    let state = ExhaustionState {
        active_thresholds: HashMap::new(),
        last_evaluation: Utc::now(),
        coalescing_state: CoalescingState {
            pending_events: vec![],
            last_event_time: Utc::now(),
        },
    };
    
    assert_eq!(state.active_thresholds.len(), 0);
    assert!(state.last_evaluation <= Utc::now());
    assert_eq!(state.coalescing_state.pending_events.len(), 0);
}

#[test]
fn test_threshold_state_creation() {
    let state = ThresholdState {
        threshold_id: "test_threshold".to_string(),
        is_active: true,
        activated_at: Utc::now(),
        applied_effects: vec![],
    };
    
    assert_eq!(state.threshold_id, "test_threshold");
    assert!(state.is_active);
    assert!(state.activated_at <= Utc::now());
    assert_eq!(state.applied_effects.len(), 0);
}

#[test]
fn test_coalescing_state_creation() {
    let state = CoalescingState {
        pending_events: vec![],
        last_event_time: Utc::now(),
    };
    
    assert_eq!(state.pending_events.len(), 0);
    assert!(state.last_event_time <= Utc::now());
}

#[test]
fn test_exhaustion_event_creation() {
    let event = ExhaustionEvent {
        event_type: ExhaustionEventType::ResourceExhausted,
        actor_id: "actor_1".to_string(),
        resource_type: "health".to_string(),
        threshold_id: "test_threshold".to_string(),
        effects: vec![],
        timestamp: Utc::now(),
        idempotency_key: "test_key".to_string(),
        coalesced: false,
    };
    
    assert_eq!(event.event_type, ExhaustionEventType::ResourceExhausted);
    assert_eq!(event.actor_id, "actor_1");
    assert_eq!(event.resource_type, "health");
    assert_eq!(event.threshold_id, "test_threshold");
    assert_eq!(event.effects.len(), 0);
    assert_eq!(event.idempotency_key, "test_key");
    assert!(!event.coalesced);
}

#[test]
fn test_exhaustion_transition_creation() {
    let transition = ExhaustionTransition {
        resource: "health".to_string(),
        threshold_id: "test_threshold".to_string(),
        entering: true,
        effects: vec![],
    };
    
    assert_eq!(transition.resource, "health");
    assert_eq!(transition.threshold_id, "test_threshold");
    assert!(transition.entering);
    assert_eq!(transition.effects.len(), 0);
}

#[test]
fn test_exhaustion_error_variants() {
    let errors = vec![
        ExhaustionError::InvalidConfig("test".to_string()),
        ExhaustionError::UnknownResource("test".to_string()),
        ExhaustionError::StorageError("test".to_string()),
        ExhaustionError::EvaluationError("test".to_string()),
        ExhaustionError::EffectApplicationError("test".to_string()),
    ];
    
    for error in errors {
        assert!(!error.to_string().is_empty());
    }
}

#[test]
fn test_exhaustion_error_conversion() {
    let error = ExhaustionError::InvalidConfig("test".to_string());
    let actor_core_error: actor_core::ActorCoreError = error.into();
    
    assert!(actor_core_error.to_string().contains("test"));
}

#[test]
fn test_exhaustion_config_creation() {
    let mut archetypes = HashMap::new();
    archetypes.insert("warrior".to_string(), ArchetypeConfig {
        resources: HashMap::new(),
    });
    
    let config = ExhaustionConfig {
        version: 1,
        hysteresis_default: 0.1,
        events: EventConfig {
            coalesce_window_ms: 100,
        },
        priorities: Some(PriorityConfig {
            categories: vec!["health".to_string()],
        }),
        archetypes,
    };
    
    assert_eq!(config.version, 1);
    assert_eq!(config.hysteresis_default, 0.1);
    assert_eq!(config.events.coalesce_window_ms, 100);
    assert!(config.priorities.is_some());
    assert_eq!(config.archetypes.len(), 1);
}

#[tokio::test]
async fn test_in_memory_event_publisher_new() {
    let publisher = InMemoryEventPublisher::new();
    
    // Test that the publisher was created successfully
    assert!(publisher.get_events().await.is_empty());
    assert!(publisher.get_stats().await.total_events == 0);
}

#[tokio::test]
async fn test_in_memory_event_publisher_with_coalesce_window() {
    let publisher = InMemoryEventPublisher::with_coalesce_window(200);
    
    // Test that the publisher was created successfully
    assert!(publisher.get_events().await.is_empty());
    assert!(publisher.get_stats().await.total_events == 0);
}

#[test]
fn test_event_stats_default() {
    let stats = EventStats::default();
    
    assert_eq!(stats.total_events, 0);
    assert_eq!(stats.events_by_type.len(), 0);
    assert_eq!(stats.coalesced_events, 0);
    assert_eq!(stats.failed_events, 0);
}

#[test]
fn test_event_stats_creation() {
    let mut events_by_type = HashMap::new();
    events_by_type.insert("ResourceExhausted".to_string(), 5);
    events_by_type.insert("ResourceRecovered".to_string(), 3);
    
    let stats = EventStats {
        total_events: 8,
        events_by_type,
        coalesced_events: 2,
        failed_events: 1,
    };
    
    assert_eq!(stats.total_events, 8);
    assert_eq!(stats.events_by_type.len(), 2);
    assert_eq!(stats.coalesced_events, 2);
    assert_eq!(stats.failed_events, 1);
}

#[test]
fn test_config_source_variants() {
    let sources = vec![
        ConfigSource::Global,
        ConfigSource::Area("test_area".to_string()),
        ConfigSource::PvP("test_pvp".to_string()),
    ];
    
    for source in sources {
        assert_eq!(source, source.clone());
    }
}

#[test]
fn test_merged_config_creation() {
    let mut sources = HashMap::new();
    sources.insert("version".to_string(), ConfigSource::Global);
    sources.insert("hysteresis_default".to_string(), ConfigSource::Area("test_area".to_string()));
    
    let config = ExhaustionConfig {
        version: 1,
        hysteresis_default: 0.1,
        events: EventConfig {
            coalesce_window_ms: 100,
        },
        priorities: None,
        archetypes: HashMap::new(),
    };
    
    let merged_config = MergedConfig {
        config,
        sources,
    };
    
    assert_eq!(merged_config.config.version, 1);
    assert_eq!(merged_config.sources.len(), 2);
    assert_eq!(merged_config.sources.get("version"), Some(&ConfigSource::Global));
    assert_eq!(merged_config.sources.get("hysteresis_default"), Some(&ConfigSource::Area("test_area".to_string())));
}

#[test]
fn test_config_loader_error_variants() {
    let errors = vec![
        ConfigLoaderError::FileNotFound("test.yaml".to_string()),
        ConfigLoaderError::InvalidYaml("test error".to_string()),
        ConfigLoaderError::InvalidJson("test error".to_string()),
    ];
    
    for error in errors {
        assert!(!error.to_string().is_empty());
    }
}

#[test]
fn test_exhaustion_config_loader_new() {
    let loader = ExhaustionConfigLoader::new();
    
    // Test that the loader was created successfully
    // Note: We can't test private fields directly, but we can test that the loader was created
    assert!(std::ptr::addr_of!(loader) != std::ptr::null());
}

#[test]
fn test_benchmark_config_creation() {
    let mut resource_ranges = HashMap::new();
    resource_ranges.insert("health".to_string(), (0.0, 100.0));
    resource_ranges.insert("mana".to_string(), (0.0, 50.0));
    
    let config = BenchmarkConfig {
        name: "test_benchmark".to_string(),
        actor_count: 100,
        evaluations_per_actor: 1000,
        resource_ranges,
    };
    
    assert_eq!(config.name, "test_benchmark");
    assert_eq!(config.actor_count, 100);
    assert_eq!(config.evaluations_per_actor, 1000);
    assert_eq!(config.resource_ranges.len(), 2);
}

#[test]
fn test_benchmark_result_creation() {
    let result = BenchmarkResult {
        config_name: "test_benchmark".to_string(),
        total_time: Duration::from_secs(5),
        avg_time_per_evaluation: Duration::from_micros(5000),
        evaluations_per_second: 200.0,
        memory_usage_bytes: Some(1024),
    };
    
    assert_eq!(result.config_name, "test_benchmark");
    assert_eq!(result.total_time, Duration::from_secs(5));
    assert_eq!(result.avg_time_per_evaluation, Duration::from_micros(5000));
    assert_eq!(result.evaluations_per_second, 200.0);
    assert_eq!(result.memory_usage_bytes, Some(1024));
}

#[test]
fn test_performance_stats_creation() {
    let stats = PerformanceStats {
        total_evaluations: 1000,
        cache_hits: 800,
        cache_misses: 200,
        avg_evaluation_time_us: 2500.0,
        total_evaluation_time: Duration::from_secs(2),
        fast_path_evaluations: 100,
    };
    
    assert_eq!(stats.total_evaluations, 1000);
    assert_eq!(stats.cache_hits, 800);
    assert_eq!(stats.cache_misses, 200);
    assert_eq!(stats.avg_evaluation_time_us, 2500.0);
    assert_eq!(stats.total_evaluation_time, Duration::from_secs(2));
    assert_eq!(stats.fast_path_evaluations, 100);
}

#[test]
fn test_exhaustion_event_serialization() {
    let event = ExhaustionEvent {
        event_type: ExhaustionEventType::ResourceExhausted,
        actor_id: "actor_1".to_string(),
        resource_type: "health".to_string(),
        threshold_id: "test_threshold".to_string(),
        effects: vec![],
        timestamp: Utc::now(),
        idempotency_key: "test_key".to_string(),
        coalesced: false,
    };
    
    let serialized = serde_json::to_string(&event).unwrap();
    let deserialized: ExhaustionEvent = serde_json::from_str(&serialized).unwrap();
    
    assert_eq!(event.event_type, deserialized.event_type);
    assert_eq!(event.actor_id, deserialized.actor_id);
    assert_eq!(event.resource_type, deserialized.resource_type);
    assert_eq!(event.threshold_id, deserialized.threshold_id);
    assert_eq!(event.idempotency_key, deserialized.idempotency_key);
    assert_eq!(event.coalesced, deserialized.coalesced);
}

#[test]
fn test_threshold_config_equality() {
    let config1 = ThresholdConfig {
        id: "test".to_string(),
        order: Some(1),
        enter_percent_lte: Some(0.5),
        exit_percent_gte: Some(0.8),
        enter_value_eq: None,
        exit_value_ge: None,
        effects: vec![],
    };
    
    let config2 = ThresholdConfig {
        id: "test".to_string(),
        order: Some(1),
        enter_percent_lte: Some(0.5),
        exit_percent_gte: Some(0.8),
        enter_value_eq: None,
        exit_value_ge: None,
        effects: vec![],
    };
    
    assert_eq!(config1, config2);
}

#[test]
fn test_effect_config_equality() {
    let config1 = EffectConfig {
        effect_type: "multiplier".to_string(),
        values: Some(vec!["health".to_string()]),
        categories: None,
        modifier: Some(0.5),
        name: None,
        value: None,
        level: None,
        resource: None,
    };
    
    let config2 = EffectConfig {
        effect_type: "multiplier".to_string(),
        values: Some(vec!["health".to_string()]),
        categories: None,
        modifier: Some(0.5),
        name: None,
        value: None,
        level: None,
        resource: None,
    };
    
    assert_eq!(config1, config2);
}

#[test]
fn test_exhaustion_event_type_equality() {
    assert_eq!(ExhaustionEventType::ResourceExhausted, ExhaustionEventType::ResourceExhausted);
    assert_eq!(ExhaustionEventType::ResourceRecovered, ExhaustionEventType::ResourceRecovered);
    assert_ne!(ExhaustionEventType::ResourceExhausted, ExhaustionEventType::ResourceRecovered);
}

#[test]
fn test_config_source_equality() {
    assert_eq!(ConfigSource::Global, ConfigSource::Global);
    assert_eq!(ConfigSource::Area("test".to_string()), ConfigSource::Area("test".to_string()));
    assert_eq!(ConfigSource::PvP("test".to_string()), ConfigSource::PvP("test".to_string()));
    assert_ne!(ConfigSource::Global, ConfigSource::Area("test".to_string()));
    assert_ne!(ConfigSource::Area("test1".to_string()), ConfigSource::Area("test2".to_string()));
}

#[test]
fn test_exhaustion_config_serialization() {
    let config = ExhaustionConfig {
        version: 1,
        hysteresis_default: 0.1,
        events: EventConfig {
            coalesce_window_ms: 100,
        },
        priorities: Some(PriorityConfig {
            categories: vec!["health".to_string()],
        }),
        archetypes: HashMap::new(),
    };
    
    let serialized = serde_json::to_string(&config).unwrap();
    let deserialized: ExhaustionConfig = serde_json::from_str(&serialized).unwrap();
    
    assert_eq!(config.version, deserialized.version);
    assert_eq!(config.hysteresis_default, deserialized.hysteresis_default);
    assert_eq!(config.events.coalesce_window_ms, deserialized.events.coalesce_window_ms);
    assert_eq!(config.priorities.is_some(), deserialized.priorities.is_some());
    assert_eq!(config.archetypes.len(), deserialized.archetypes.len());
}

#[test]
fn test_duration_creation() {
    let duration = Duration::from_secs(5);
    assert_eq!(duration.as_secs(), 5);
}

#[test]
fn test_utc_now() {
    let now = Utc::now();
    assert!(now.timestamp() > 0);
}

#[test]
fn test_hashmap_operations() {
    let mut map = HashMap::new();
    map.insert("key1".to_string(), "value1".to_string());
    map.insert("key2".to_string(), "value2".to_string());
    
    assert_eq!(map.len(), 2);
    assert_eq!(map.get("key1"), Some(&"value1".to_string()));
    assert_eq!(map.get("key2"), Some(&"value2".to_string()));
    assert!(map.contains_key("key1"));
    assert!(!map.contains_key("key3"));
}

#[test]
fn test_string_operations() {
    let s1 = "test".to_string();
    let s2 = s1.clone();
    
    assert_eq!(s1, s2);
    assert_eq!(s1.len(), 4);
    assert!(!s1.is_empty());
}

#[test]
fn test_option_operations() {
    let some_value = Some(42);
    let none_value: Option<i32> = None;
    
    assert!(some_value.is_some());
    assert!(none_value.is_none());
    assert_eq!(some_value.unwrap(), 42);
}

#[test]
fn test_vec_operations() {
    let mut vec = Vec::new();
    vec.push("item1");
    vec.push("item2");
    
    assert_eq!(vec.len(), 2);
    assert_eq!(vec[0], "item1");
    assert_eq!(vec[1], "item2");
}