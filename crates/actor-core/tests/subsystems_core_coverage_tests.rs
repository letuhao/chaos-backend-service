//! Coverage tests for subsystems/core modules.

use actor_core::subsystems::core::resource_events::{
    ResourceEventManager, 
    ResourceEvent, 
    ResourceEventType, 
    EventPriority, 
    EventFilter, 
    EventConfig, 
    EventStats,
    ResourceEventListener,
    DefaultResourceEventListener
};
use actor_core::subsystems::core::stat_change_notifier::{
    StatChangeNotifier, 
    StatChangeEvent, 
    StatChangeListener, 
    NotifierConfig
};
use actor_core::ActorCoreResult;
use std::collections::HashMap;
use std::sync::Arc;

// Mock implementations for testing
struct MockResourceEventListener {
    listener_id: String,
    interested_event_types: Vec<ResourceEventType>,
    event_priority: EventPriority,
}

impl MockResourceEventListener {
    fn new(
        listener_id: String,
        interested_event_types: Vec<ResourceEventType>,
        event_priority: EventPriority,
    ) -> Self {
        Self {
            listener_id,
            interested_event_types,
            event_priority,
        }
    }
}

#[async_trait::async_trait]
impl ResourceEventListener for MockResourceEventListener {
    async fn handle_event(&self, _event: &ResourceEvent) -> ActorCoreResult<()> {
        Ok(())
    }
    
    fn listener_id(&self) -> &str {
        &self.listener_id
    }
    
    fn interested_event_types(&self) -> Vec<ResourceEventType> {
        self.interested_event_types.clone()
    }
    
    fn event_priority(&self) -> EventPriority {
        self.event_priority.clone()
    }
}

struct MockStatChangeListener {
    listener_id: String,
    interested_stats: Vec<String>,
}

impl MockStatChangeListener {
    fn new(listener_id: String, interested_stats: Vec<String>) -> Self {
        Self {
            listener_id,
            interested_stats,
        }
    }
}

#[async_trait::async_trait]
impl StatChangeListener for MockStatChangeListener {
    async fn on_stat_change(&self, _event: &StatChangeEvent) -> ActorCoreResult<()> {
        Ok(())
    }
    
    fn listener_id(&self) -> &str {
        &self.listener_id
    }
    
    fn interested_stats(&self) -> Vec<String> {
        self.interested_stats.clone()
    }
}

#[test]
fn test_resource_event_type_variants() {
    let variants = vec![
        ResourceEventType::ResourceChanged,
        ResourceEventType::ResourceRegenerated,
        ResourceEventType::ResourceDepleted,
        ResourceEventType::ResourceFullyRestored,
        ResourceEventType::ResourceMaxChanged,
        ResourceEventType::ResourceRateChanged,
        ResourceEventType::ResourceCategoryChanged,
        ResourceEventType::ResourceSystemActivated,
        ResourceEventType::ResourceSystemDeactivated,
        ResourceEventType::ResourceCalculationCompleted,
        ResourceEventType::ResourceCacheUpdated,
        ResourceEventType::ResourceDatabaseStored,
        ResourceEventType::ResourceDatabaseLoaded,
    ];
    
    for variant in variants {
        assert_eq!(variant, variant.clone());
    }
}

#[test]
fn test_event_priority_variants() {
    let variants = vec![
        EventPriority::Low,
        EventPriority::Normal,
        EventPriority::High,
        EventPriority::Critical,
    ];
    
    for variant in variants {
        assert_eq!(variant, variant.clone());
    }
}

#[test]
fn test_event_config_default() {
    let config = EventConfig::default();
    
    assert_eq!(config.max_history_size, 10000);
    assert!(config.enable_batching);
    assert_eq!(config.batch_size, 100);
    assert_eq!(config.batch_timeout_ms, 100);
    assert!(config.enable_filtering);
    assert!(config.enable_persistence);
    assert!(config.enable_monitoring);
}

#[test]
fn test_event_config_new() {
    let config = EventConfig {
        max_history_size: 5000,
        enable_batching: false,
        batch_size: 50,
        batch_timeout_ms: 200,
        enable_filtering: false,
        enable_persistence: false,
        enable_monitoring: false,
    };
    
    assert_eq!(config.max_history_size, 5000);
    assert!(!config.enable_batching);
    assert_eq!(config.batch_size, 50);
    assert_eq!(config.batch_timeout_ms, 200);
    assert!(!config.enable_filtering);
    assert!(!config.enable_persistence);
    assert!(!config.enable_monitoring);
}

#[test]
fn test_resource_event_creation() {
    let event = ResourceEvent {
        event_id: "test_event_1".to_string(),
        actor_id: "actor_1".to_string(),
        event_type: ResourceEventType::ResourceChanged,
        resource_name: "health".to_string(),
        old_value: 100.0,
        new_value: 80.0,
        timestamp: 1234567890,
        source: "test_source".to_string(),
        metadata: HashMap::new(),
        priority: EventPriority::Normal,
    };
    
    assert_eq!(event.event_id, "test_event_1");
    assert_eq!(event.actor_id, "actor_1");
    assert_eq!(event.event_type, ResourceEventType::ResourceChanged);
    assert_eq!(event.resource_name, "health");
    assert_eq!(event.old_value, 100.0);
    assert_eq!(event.new_value, 80.0);
    assert_eq!(event.timestamp, 1234567890);
    assert_eq!(event.source, "test_source");
    assert_eq!(event.priority, EventPriority::Normal);
}

#[test]
fn test_event_filter_creation() {
    let filter = EventFilter {
        name: "test_filter".to_string(),
        event_types: vec![ResourceEventType::ResourceChanged, ResourceEventType::ResourceDepleted],
        actor_ids: Some(vec!["actor_1".to_string(), "actor_2".to_string()]),
        resource_names: Some(vec!["health".to_string(), "mana".to_string()]),
        priorities: vec![EventPriority::High, EventPriority::Critical],
        min_value_change: Some(10.0),
        max_value_change: Some(100.0),
    };
    
    assert_eq!(filter.name, "test_filter");
    assert_eq!(filter.event_types.len(), 2);
    assert_eq!(filter.actor_ids.as_ref().unwrap().len(), 2);
    assert_eq!(filter.resource_names.as_ref().unwrap().len(), 2);
    assert_eq!(filter.priorities.len(), 2);
    assert_eq!(filter.min_value_change, Some(10.0));
    assert_eq!(filter.max_value_change, Some(100.0));
}

#[test]
fn test_event_stats_creation() {
    let mut event_type_counts = HashMap::new();
    event_type_counts.insert("ResourceChanged".to_string(), 10);
    event_type_counts.insert("ResourceDepleted".to_string(), 5);
    
    let mut actor_event_counts = HashMap::new();
    actor_event_counts.insert("actor_1".to_string(), 8);
    actor_event_counts.insert("actor_2".to_string(), 7);
    
    let mut resource_event_counts = HashMap::new();
    resource_event_counts.insert("health".to_string(), 12);
    resource_event_counts.insert("mana".to_string(), 3);
    
    let stats = EventStats {
        total_events: 15,
        event_type_counts,
        actor_event_counts,
        resource_event_counts,
    };
    
    assert_eq!(stats.total_events, 15);
    assert_eq!(stats.event_type_counts.len(), 2);
    assert_eq!(stats.actor_event_counts.len(), 2);
    assert_eq!(stats.resource_event_counts.len(), 2);
}

#[test]
fn test_default_resource_event_listener_new() {
    let listener = DefaultResourceEventListener::new(
        "test_listener".to_string(),
        vec![ResourceEventType::ResourceChanged],
        EventPriority::Normal,
    );
    
    assert_eq!(listener.listener_id(), "test_listener");
    assert_eq!(listener.interested_event_types().len(), 1);
    assert_eq!(listener.event_priority(), EventPriority::Normal);
}

#[tokio::test]
async fn test_resource_event_manager_new() {
    let config = EventConfig::default();
    let manager = ResourceEventManager::new(config);
    
    // Test that the manager was created successfully
    assert!(manager.get_event_history(None, None, None).await.is_ok());
}

#[tokio::test]
async fn test_resource_event_manager_add_listener() {
    let config = EventConfig::default();
    let manager = ResourceEventManager::new(config);
    
    let listener = Arc::new(MockResourceEventListener::new(
        "test_listener".to_string(),
        vec![ResourceEventType::ResourceChanged],
        EventPriority::Normal,
    ));
    
    let result = manager.add_listener(listener).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_resource_event_manager_remove_listener() {
    let config = EventConfig::default();
    let manager = ResourceEventManager::new(config);
    
    let result = manager.remove_listener("test_listener").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_resource_event_manager_emit_event() {
    let config = EventConfig::default();
    let manager = ResourceEventManager::new(config);
    
    let event = ResourceEvent {
        event_id: "test_event".to_string(),
        actor_id: "actor_1".to_string(),
        event_type: ResourceEventType::ResourceChanged,
        resource_name: "health".to_string(),
        old_value: 100.0,
        new_value: 80.0,
        timestamp: 1234567890,
        source: "test_source".to_string(),
        metadata: HashMap::new(),
        priority: EventPriority::Normal,
    };
    
    let result = manager.emit_event(event).await;
    assert!(result.is_ok());
}

#[test]
fn test_resource_event_manager_create_resource_changed_event() {
    let config = EventConfig::default();
    let manager = ResourceEventManager::new(config);
    
    let event = manager.create_resource_changed_event(
        "actor_1",
        "health",
        100.0,
        80.0,
        "test_source",
    );
    
    assert_eq!(event.actor_id, "actor_1");
    assert_eq!(event.resource_name, "health");
    assert_eq!(event.old_value, 100.0);
    assert_eq!(event.new_value, 80.0);
    assert_eq!(event.event_type, ResourceEventType::ResourceChanged);
    assert_eq!(event.source, "test_source");
    assert_eq!(event.priority, EventPriority::Normal);
}

#[test]
fn test_resource_event_manager_create_resource_regenerated_event() {
    let config = EventConfig::default();
    let manager = ResourceEventManager::new(config);
    
    let event = manager.create_resource_regenerated_event(
        "actor_1",
        "health",
        20.0,
        100.0,
        "test_source",
    );
    
    assert_eq!(event.actor_id, "actor_1");
    assert_eq!(event.resource_name, "health");
    assert_eq!(event.old_value, 80.0); // new_value - amount
    assert_eq!(event.new_value, 100.0);
    assert_eq!(event.event_type, ResourceEventType::ResourceRegenerated);
    assert_eq!(event.source, "test_source");
    assert_eq!(event.priority, EventPriority::Low);
}

#[test]
fn test_resource_event_manager_create_resource_depleted_event() {
    let config = EventConfig::default();
    let manager = ResourceEventManager::new(config);
    
    let event = manager.create_resource_depleted_event(
        "actor_1",
        "health",
        "test_source",
    );
    
    assert_eq!(event.actor_id, "actor_1");
    assert_eq!(event.resource_name, "health");
    assert_eq!(event.old_value, 0.0);
    assert_eq!(event.new_value, 0.0);
    assert_eq!(event.event_type, ResourceEventType::ResourceDepleted);
    assert_eq!(event.source, "test_source");
    assert_eq!(event.priority, EventPriority::High);
}

#[test]
fn test_resource_event_manager_create_resource_fully_restored_event() {
    let config = EventConfig::default();
    let manager = ResourceEventManager::new(config);
    
    let event = manager.create_resource_fully_restored_event(
        "actor_1",
        "health",
        100.0,
        "test_source",
    );
    
    assert_eq!(event.actor_id, "actor_1");
    assert_eq!(event.resource_name, "health");
    assert_eq!(event.old_value, 0.0);
    assert_eq!(event.new_value, 100.0);
    assert_eq!(event.event_type, ResourceEventType::ResourceFullyRestored);
    assert_eq!(event.source, "test_source");
    assert_eq!(event.priority, EventPriority::Normal);
}

#[tokio::test]
async fn test_resource_event_manager_add_event_filter() {
    let config = EventConfig::default();
    let manager = ResourceEventManager::new(config);
    
    let filter = EventFilter {
        name: "test_filter".to_string(),
        event_types: vec![ResourceEventType::ResourceChanged],
        actor_ids: None,
        resource_names: None,
        priorities: vec![EventPriority::High],
        min_value_change: None,
        max_value_change: None,
    };
    
    let result = manager.add_event_filter(filter).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_resource_event_manager_remove_event_filter() {
    let config = EventConfig::default();
    let manager = ResourceEventManager::new(config);
    
    let result = manager.remove_event_filter("test_filter").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_resource_event_manager_get_event_history() {
    let config = EventConfig::default();
    let manager = ResourceEventManager::new(config);
    
    let result = manager.get_event_history(None, None, None).await;
    assert!(result.is_ok());
    
    let events = result.unwrap();
    assert_eq!(events.len(), 0);
}

#[tokio::test]
async fn test_resource_event_manager_get_event_history_with_filters() {
    let config = EventConfig::default();
    let manager = ResourceEventManager::new(config);
    
    let result = manager.get_event_history(
        Some("actor_1"),
        Some(&ResourceEventType::ResourceChanged),
        Some(10),
    ).await;
    assert!(result.is_ok());
    
    let events = result.unwrap();
    assert_eq!(events.len(), 0);
}

#[test]
fn test_stat_change_event_creation() {
    let event = StatChangeEvent {
        actor_id: "actor_1".to_string(),
        changed_stats: vec!["health".to_string(), "mana".to_string()],
        timestamp: 1234567890,
        source: "test_source".to_string(),
        metadata: HashMap::new(),
    };
    
    assert_eq!(event.actor_id, "actor_1");
    assert_eq!(event.changed_stats.len(), 2);
    assert_eq!(event.timestamp, 1234567890);
    assert_eq!(event.source, "test_source");
}

#[test]
fn test_notifier_config_default() {
    let config = NotifierConfig::default();
    
    assert_eq!(config.max_history_size, 10000);
    assert!(config.enable_batching);
    assert_eq!(config.batch_timeout_ms, 100);
    assert!(config.enable_dependency_tracking);
}

#[test]
fn test_notifier_config_new() {
    let config = NotifierConfig {
        max_history_size: 5000,
        enable_batching: false,
        batch_timeout_ms: 200,
        enable_dependency_tracking: false,
    };
    
    assert_eq!(config.max_history_size, 5000);
    assert!(!config.enable_batching);
    assert_eq!(config.batch_timeout_ms, 200);
    assert!(!config.enable_dependency_tracking);
}

#[tokio::test]
async fn test_stat_change_notifier_new() {
    let config = NotifierConfig::default();
    let notifier = StatChangeNotifier::new(config);
    
    // Test that the notifier was created successfully
    assert!(notifier.get_history(None, None).await.is_ok());
}

#[tokio::test]
async fn test_stat_change_notifier_add_listener() {
    let config = NotifierConfig::default();
    let notifier = StatChangeNotifier::new(config);
    
    let listener = Arc::new(MockStatChangeListener::new(
        "test_listener".to_string(),
        vec!["health".to_string()],
    ));
    
    let result = notifier.add_listener(listener).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_stat_change_notifier_remove_listener() {
    let config = NotifierConfig::default();
    let notifier = StatChangeNotifier::new(config);
    
    let result = notifier.remove_listener("test_listener").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_stat_change_notifier_notify_stat_change() {
    let config = NotifierConfig::default();
    let notifier = StatChangeNotifier::new(config);
    
    let _event = StatChangeEvent {
        actor_id: "actor_1".to_string(),
        changed_stats: vec!["health".to_string()],
        timestamp: 1234567890,
        source: "test_source".to_string(),
        metadata: HashMap::new(),
    };
    
    // Create a mock Actor for testing
    let actor = actor_core::types::Actor::new("actor_1".to_string(), "human".to_string());
    let changed_stats = vec!["health".to_string()];
    let result = notifier.notify_stat_change(&actor, &changed_stats).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_stat_change_notifier_get_change_history() {
    let config = NotifierConfig::default();
    let notifier = StatChangeNotifier::new(config);
    
    let result = notifier.get_history(None, None).await;
    assert!(result.is_ok());
    
    let events = result.unwrap();
    assert_eq!(events.len(), 0);
}

#[tokio::test]
async fn test_stat_change_notifier_get_change_history_with_filters() {
    let config = NotifierConfig::default();
    let notifier = StatChangeNotifier::new(config);
    
    let result = notifier.get_history(
        Some("actor_1"),
        Some(10),
    ).await;
    assert!(result.is_ok());
    
    let events = result.unwrap();
    assert_eq!(events.len(), 0);
}

#[tokio::test]
async fn test_stat_change_notifier_add_resource_dependency() {
    let config = NotifierConfig::default();
    let notifier = StatChangeNotifier::new(config);
    
    let result = notifier.add_resource_dependency(
        "health".to_string(),
        "mana".to_string(),
    ).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_stat_change_notifier_remove_resource_dependency() {
    let config = NotifierConfig::default();
    let notifier = StatChangeNotifier::new(config);
    
    let result = notifier.remove_resource_dependency(
        "health",
        "mana",
    ).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_stat_change_notifier_get_dependent_resources() {
    let config = NotifierConfig::default();
    let notifier = StatChangeNotifier::new(config);
    
    let changed_stats = vec!["health".to_string()];
    let result = notifier.get_affected_resources(&changed_stats).await;
    assert!(result.is_ok());
    
    let dependencies = result.unwrap();
    assert_eq!(dependencies.len(), 0);
}

#[test]
fn test_resource_event_serialization() {
    let event = ResourceEvent {
        event_id: "test_event".to_string(),
        actor_id: "actor_1".to_string(),
        event_type: ResourceEventType::ResourceChanged,
        resource_name: "health".to_string(),
        old_value: 100.0,
        new_value: 80.0,
        timestamp: 1234567890,
        source: "test_source".to_string(),
        metadata: HashMap::new(),
        priority: EventPriority::Normal,
    };
    
    let serialized = serde_json::to_string(&event).unwrap();
    let deserialized: ResourceEvent = serde_json::from_str(&serialized).unwrap();
    
    assert_eq!(event.event_id, deserialized.event_id);
    assert_eq!(event.actor_id, deserialized.actor_id);
    assert_eq!(event.event_type, deserialized.event_type);
    assert_eq!(event.resource_name, deserialized.resource_name);
    assert_eq!(event.old_value, deserialized.old_value);
    assert_eq!(event.new_value, deserialized.new_value);
    assert_eq!(event.timestamp, deserialized.timestamp);
    assert_eq!(event.source, deserialized.source);
    assert_eq!(event.priority, deserialized.priority);
}

#[test]
fn test_stat_change_event_serialization() {
    let event = StatChangeEvent {
        actor_id: "actor_1".to_string(),
        changed_stats: vec!["health".to_string()],
        timestamp: 1234567890,
        source: "test_source".to_string(),
        metadata: HashMap::new(),
    };
    
    let serialized = serde_json::to_string(&event).unwrap();
    let deserialized: StatChangeEvent = serde_json::from_str(&serialized).unwrap();
    
    assert_eq!(event.actor_id, deserialized.actor_id);
    assert_eq!(event.changed_stats, deserialized.changed_stats);
    assert_eq!(event.timestamp, deserialized.timestamp);
    assert_eq!(event.source, deserialized.source);
}

#[test]
fn test_event_priority_equality() {
    assert_eq!(EventPriority::Low, EventPriority::Low);
    assert_eq!(EventPriority::Normal, EventPriority::Normal);
    assert_eq!(EventPriority::High, EventPriority::High);
    assert_eq!(EventPriority::Critical, EventPriority::Critical);
}

#[test]
fn test_resource_event_type_display() {
    let event_type = ResourceEventType::ResourceChanged;
    let display = format!("{:?}", event_type);
    assert!(display.contains("ResourceChanged"));
}

#[test]
fn test_event_priority_display() {
    let priority = EventPriority::High;
    let display = format!("{:?}", priority);
    assert!(display.contains("High"));
}

#[test]
fn test_stat_change_event_display() {
    let event = StatChangeEvent {
        actor_id: "actor_1".to_string(),
        changed_stats: vec!["health".to_string()],
        timestamp: 1234567890,
        source: "test_source".to_string(),
        metadata: HashMap::new(),
    };
    
    let display = format!("{:?}", event);
    assert!(display.contains("actor_1"));
    assert!(display.contains("health"));
}

#[test]
fn test_event_config_clone() {
    let config1 = EventConfig::default();
    let config2 = config1.clone();
    
    assert_eq!(config1.max_history_size, config2.max_history_size);
    assert_eq!(config1.enable_batching, config2.enable_batching);
    assert_eq!(config1.batch_size, config2.batch_size);
    assert_eq!(config1.batch_timeout_ms, config2.batch_timeout_ms);
    assert_eq!(config1.enable_filtering, config2.enable_filtering);
    assert_eq!(config1.enable_persistence, config2.enable_persistence);
    assert_eq!(config1.enable_monitoring, config2.enable_monitoring);
}

#[test]
fn test_notifier_config_clone() {
    let config1 = NotifierConfig::default();
    let config2 = config1.clone();
    
    assert_eq!(config1.max_history_size, config2.max_history_size);
    assert_eq!(config1.enable_batching, config2.enable_batching);
    assert_eq!(config1.batch_timeout_ms, config2.batch_timeout_ms);
    assert_eq!(config1.enable_dependency_tracking, config2.enable_dependency_tracking);
}

#[test]
fn test_event_filter_clone() {
    let filter1 = EventFilter {
        name: "test_filter".to_string(),
        event_types: vec![ResourceEventType::ResourceChanged],
        actor_ids: None,
        resource_names: None,
        priorities: vec![EventPriority::High],
        min_value_change: None,
        max_value_change: None,
    };
    
    let filter2 = filter1.clone();
    
    assert_eq!(filter1.name, filter2.name);
    assert_eq!(filter1.event_types, filter2.event_types);
    assert_eq!(filter1.priorities, filter2.priorities);
}
