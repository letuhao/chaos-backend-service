//! Data provider interfaces for Condition Core

use super::error::*;
use super::types::{StatusEffectHistory, StatusEffectTimeline};
use std::sync::Arc;

/// Trait for providing element data to Condition Core
#[async_trait::async_trait]
pub trait ElementDataProvider: Send + Sync {
    /// Get element mastery level
    async fn get_element_mastery(&self, element_id: &str, actor_id: &str) -> ConditionResult<f64>;
    
    /// Get element resistance
    async fn get_element_resistance(&self, element_id: &str, actor_id: &str) -> ConditionResult<f64>;
    
    /// Check if actor has element affinity
    async fn has_element_affinity(&self, element_id: &str, actor_id: &str) -> ConditionResult<bool>;
    
    /// Check if actor has element weakness
    async fn is_element_weakness(&self, element_id: &str, actor_id: &str) -> ConditionResult<bool>;
    
    /// Get element interaction result
    async fn get_element_interaction(&self, source_element: &str, target_element: &str) -> ConditionResult<String>;
    
    /// List all available elements
    async fn list_elements(&self) -> ConditionResult<Vec<String>>;
    
    // Element interaction functions
    async fn is_element_same_category(&self, element1: &str, element2: &str) -> ConditionResult<bool>;
    async fn is_element_generating(&self, source_element: &str, target_element: &str) -> ConditionResult<bool>;
    async fn is_element_overcoming(&self, source_element: &str, target_element: &str) -> ConditionResult<bool>;
    async fn is_element_neutral(&self, source_element: &str, target_element: &str) -> ConditionResult<bool>;
    
    // Element status functions
    async fn has_element_status_effect(&self, element_id: &str, status_id: &str, actor_id: &str) -> ConditionResult<bool>;
    async fn get_element_status_effect_count(&self, element_id: &str, status_id: &str, actor_id: &str) -> ConditionResult<i64>;
    async fn is_element_status_effect_active(&self, element_id: &str, status_id: &str, actor_id: &str) -> ConditionResult<bool>;
    
    // Element resource functions
    async fn has_element_resource(&self, element_id: &str, resource_type: &str, actor_id: &str) -> ConditionResult<bool>;
    async fn get_element_resource_value(&self, element_id: &str, resource_type: &str, actor_id: &str) -> ConditionResult<f64>;
    async fn is_element_resource_below_threshold(&self, element_id: &str, resource_type: &str, threshold: f64, actor_id: &str) -> ConditionResult<bool>;
    async fn is_element_resource_above_threshold(&self, element_id: &str, resource_type: &str, threshold: f64, actor_id: &str) -> ConditionResult<bool>;
    
    // Hybrid element functions
    async fn has_hybrid_element(&self, hybrid_id: &str, actor_id: &str) -> ConditionResult<bool>;
    async fn is_hybrid_element_activated(&self, hybrid_id: &str, actor_id: &str) -> ConditionResult<bool>;
    async fn get_hybrid_element_parents(&self, hybrid_id: &str) -> ConditionResult<Vec<String>>;
    async fn list_hybrid_elements(&self) -> ConditionResult<Vec<String>>;
    
    // Element derived stats functions
    async fn get_element_derived_stat(&self, element_id: &str, stat_name: &str, actor_id: &str) -> ConditionResult<f64>;
    async fn has_element_derived_stat(&self, element_id: &str, stat_name: &str, actor_id: &str) -> ConditionResult<bool>;
    async fn list_element_derived_stats(&self, element_id: &str) -> ConditionResult<Vec<String>>;
}

/// Trait for providing actor data to Condition Core
#[async_trait::async_trait]
pub trait ActorDataProvider: Send + Sync {
    /// Get actor resource value (generic)
    async fn get_actor_resource(&self, resource_type: &str, actor_id: &str) -> ConditionResult<f64>;
    
    /// Get actor stat value (generic)
    async fn get_actor_stat(&self, stat_name: &str, actor_id: &str) -> ConditionResult<f64>;
    
    /// Get actor derived stat value
    async fn get_actor_derived_stat(&self, stat_name: &str, actor_id: &str) -> ConditionResult<f64>;
    
    /// Get actor race
    async fn get_actor_race(&self, actor_id: &str) -> ConditionResult<String>;
    
    /// Check if actor is in combat
    async fn is_actor_in_combat(&self, actor_id: &str) -> ConditionResult<bool>;
    
    /// Check if actor has status effects of specific type
    async fn has_actor_status_effects(&self, status_type: &str, actor_id: &str) -> ConditionResult<bool>;
    
    /// Get actor status effect count
    async fn get_actor_status_effect_count(&self, status_type: &str, actor_id: &str) -> ConditionResult<i64>;
    
    /// Get actor status effect count by category
    async fn get_actor_status_effect_count_by_category(&self, status_type: &str, category: &str, actor_id: &str) -> ConditionResult<i64>;
}

/// Trait for providing resource data to Condition Core
#[async_trait::async_trait]
pub trait ResourceDataProvider: Send + Sync {
    /// Get current resource value
    async fn get_resource_value(&self, resource_type: &str, actor_id: &str) -> ConditionResult<f64>;
    
    /// Get maximum resource value
    async fn get_resource_max(&self, resource_type: &str, actor_id: &str) -> ConditionResult<f64>;
    
    /// Get resource percentage
    async fn get_resource_percentage(&self, resource_type: &str, actor_id: &str) -> ConditionResult<f64>;
    
    
    /// Check if resource is empty
    async fn is_resource_empty(&self, resource_type: &str, actor_id: &str) -> ConditionResult<bool>;
    
    /// Check if resource is below custom threshold
    async fn is_resource_below_threshold(&self, resource_type: &str, threshold: f64, actor_id: &str) -> ConditionResult<bool>;
    
    /// Check if resource is above custom threshold
    async fn is_resource_above_threshold(&self, resource_type: &str, threshold: f64, actor_id: &str) -> ConditionResult<bool>;
    
    /// Check if resource is below percentage threshold
    async fn is_resource_below_percentage(&self, resource_type: &str, percentage: f64, actor_id: &str) -> ConditionResult<bool>;
    
    /// Check if resource is above percentage threshold
    async fn is_resource_above_percentage(&self, resource_type: &str, percentage: f64, actor_id: &str) -> ConditionResult<bool>;
    
    /// List all available resources
    async fn list_resources(&self) -> ConditionResult<Vec<String>>;
}

/// Trait for providing category data to Condition Core
#[async_trait::async_trait]
pub trait CategoryDataProvider: Send + Sync {
    /// Check if actor has items from category
    async fn has_category_item(&self, category_id: &str, actor_id: &str) -> ConditionResult<bool>;
    
    /// Get count of items from category
    async fn get_category_item_count(&self, category_id: &str, actor_id: &str) -> ConditionResult<i64>;
    
    /// Check if category is available
    async fn is_category_available(&self, category_id: &str, actor_id: &str) -> ConditionResult<bool>;
    
    /// Check if category is blocked
    async fn is_category_blocked(&self, category_id: &str, actor_id: &str) -> ConditionResult<bool>;
    
    /// List all available categories
    async fn list_categories(&self) -> ConditionResult<Vec<String>>;
}

/// Trait for providing status effect data to Condition Core
#[async_trait::async_trait]
pub trait StatusDataProvider: Send + Sync {
    // Basic Status Functions
    async fn has_status_effect(&self, actor_id: &str, effect_id: &str) -> ConditionResult<bool>;
    async fn get_status_effect_count(&self, actor_id: &str, effect_id: &str) -> ConditionResult<u32>;
    async fn get_status_effect_magnitude(&self, actor_id: &str, effect_id: &str) -> ConditionResult<f64>;
    async fn is_status_effect_active(&self, actor_id: &str, effect_id: &str) -> ConditionResult<bool>;
    async fn is_status_effect_expired(&self, actor_id: &str, effect_id: &str) -> ConditionResult<bool>;
    
    // Status Immunity Functions
    async fn has_status_immunity(&self, actor_id: &str, effect_id: &str) -> ConditionResult<bool>;
    async fn get_status_immunity_count(&self, actor_id: &str, effect_id: &str) -> ConditionResult<u32>;
    async fn is_status_immunity_active(&self, actor_id: &str, effect_id: &str) -> ConditionResult<bool>;
    
    // Status Category Functions
    async fn has_status_category(&self, actor_id: &str, category: &str) -> ConditionResult<bool>;
    async fn get_status_category_count(&self, actor_id: &str, category: &str) -> ConditionResult<u32>;
    async fn list_status_categories(&self, actor_id: &str) -> ConditionResult<Vec<String>>;
    
    // Status Interaction Functions
    async fn is_status_effect_stackable(&self, effect_id: &str) -> ConditionResult<bool>;
    async fn can_status_effect_stack(&self, actor_id: &str, effect_id: &str) -> ConditionResult<bool>;
    async fn get_status_effect_interaction(&self, effect_id: &str, target_effect_id: &str) -> ConditionResult<String>;
    async fn get_status_effect_priority(&self, effect_id: &str) -> ConditionResult<i32>;
    async fn get_status_effect_source(&self, actor_id: &str, effect_id: &str) -> ConditionResult<String>;
    async fn get_status_effect_target(&self, actor_id: &str, effect_id: &str) -> ConditionResult<String>;
    
    // Status Movement Functions
    async fn has_status_movement_restriction(&self, actor_id: &str, restriction_type: &str) -> ConditionResult<bool>;
    async fn get_status_movement_restriction(&self, actor_id: &str, restriction_type: &str) -> ConditionResult<f64>;
    
    // Status Visual/Audio Functions
    async fn has_status_visual_effect(&self, actor_id: &str, effect_id: &str) -> ConditionResult<bool>;
    async fn get_status_visual_effect(&self, actor_id: &str, effect_id: &str) -> ConditionResult<String>;
    async fn has_status_audio_effect(&self, actor_id: &str, effect_id: &str) -> ConditionResult<bool>;
    async fn get_status_audio_effect(&self, actor_id: &str, effect_id: &str) -> ConditionResult<String>;
    
    // Status Properties Functions
    async fn get_status_effect_properties(&self, actor_id: &str, effect_id: &str) -> ConditionResult<std::collections::HashMap<String, serde_json::Value>>;
    async fn has_status_effect_property(&self, actor_id: &str, effect_id: &str, property: &str) -> ConditionResult<bool>;
    async fn get_status_effect_property(&self, actor_id: &str, effect_id: &str, property: &str) -> ConditionResult<serde_json::Value>;
    
    // Status History Functions
    async fn get_status_effect_history(&self, actor_id: &str, effect_id: &str) -> ConditionResult<Vec<StatusEffectHistory>>;
    async fn get_status_effect_timeline(&self, actor_id: &str, effect_id: &str) -> ConditionResult<Vec<StatusEffectTimeline>>;
    
    // Legacy functions for backward compatibility
    async fn get_status_effect_level(&self, status_id: &str, actor_id: &str) -> ConditionResult<i64>;
    async fn has_category_status(&self, category_id: &str, actor_id: &str) -> ConditionResult<bool>;
    async fn list_status_effects(&self) -> ConditionResult<Vec<String>>;
}

/// Trait for providing action data to Condition Core
#[async_trait::async_trait]
pub trait ActionDataProvider: Send + Sync {
    /// Check if actor has action
    async fn has_action(&self, action_id: &str, actor_id: &str) -> ConditionResult<bool>;
    
    /// Check if actor can perform action
    async fn can_perform_action(&self, action_id: &str, actor_id: &str) -> ConditionResult<bool>;
    
    /// List all available actions
    async fn list_actions(&self) -> ConditionResult<Vec<String>>;
}

/// Trait for providing location data to Condition Core
#[async_trait::async_trait]
pub trait LocationDataProvider: Send + Sync {
    /// Check if actor is in location
    async fn is_in_location(&self, location_id: &str, actor_id: &str) -> ConditionResult<bool>;
    
    /// Get location type
    async fn get_location_type(&self, actor_id: &str) -> ConditionResult<String>;
    
    /// List all available locations
    async fn list_locations(&self) -> ConditionResult<Vec<String>>;
}

/// Trait for providing event data to Condition Core
#[async_trait::async_trait]
pub trait EventDataProvider: Send + Sync {
    /// Check if actor has active event
    async fn has_active_event(&self, event_id: &str, actor_id: &str) -> ConditionResult<bool>;
    
    /// Check if event is active
    async fn is_event_active(&self, event_id: &str) -> ConditionResult<bool>;
    
    /// List all available events
    async fn list_events(&self) -> ConditionResult<Vec<String>>;
}

/// Trait for providing quest data to Condition Core
#[async_trait::async_trait]
pub trait QuestDataProvider: Send + Sync {
    /// Check if actor has quest
    async fn has_quest(&self, quest_id: &str, actor_id: &str) -> ConditionResult<bool>;
    
    /// Check if quest is completed
    async fn is_quest_completed(&self, quest_id: &str, actor_id: &str) -> ConditionResult<bool>;
    
    /// List all available quests
    async fn list_quests(&self) -> ConditionResult<Vec<String>>;
}


/// Trait for providing item data to Condition Core
#[async_trait::async_trait]
pub trait ItemDataProvider: Send + Sync {
    /// Check if actor has item
    async fn has_item(&self, item_id: &str, actor_id: &str) -> ConditionResult<bool>;
    
    /// Get item count
    async fn get_item_count(&self, item_id: &str, actor_id: &str) -> ConditionResult<i64>;
    
    /// List all available items
    async fn list_items(&self) -> ConditionResult<Vec<String>>;
}

/// Trait for providing shield data to Condition Core
#[async_trait::async_trait]
pub trait ShieldDataProvider: Send + Sync {
    /// Check if actor has shield
    async fn has_shield(&self, actor_id: &str) -> ConditionResult<bool>;
    
    /// Get shield strength
    async fn get_shield_strength(&self, actor_id: &str) -> ConditionResult<f64>;
}

/// Data provider registry for managing all data providers
pub struct DataProviderRegistry {
    element_provider: Option<Arc<dyn ElementDataProvider>>,
    resource_provider: Option<Arc<dyn ResourceDataProvider>>,
    category_provider: Option<Arc<dyn CategoryDataProvider>>,
    status_provider: Option<Arc<dyn StatusDataProvider>>,
    action_provider: Option<Arc<dyn ActionDataProvider>>,
    location_provider: Option<Arc<dyn LocationDataProvider>>,
    event_provider: Option<Arc<dyn EventDataProvider>>,
    quest_provider: Option<Arc<dyn QuestDataProvider>>,
    actor_provider: Option<Arc<dyn ActorDataProvider>>,
    item_provider: Option<Arc<dyn ItemDataProvider>>,
    shield_provider: Option<Arc<dyn ShieldDataProvider>>,
}

impl DataProviderRegistry {
    /// Create a new data provider registry
    pub fn new() -> Self {
        Self {
            element_provider: None,
            resource_provider: None,
            category_provider: None,
            status_provider: None,
            action_provider: None,
            location_provider: None,
            event_provider: None,
            quest_provider: None,
            actor_provider: None,
            item_provider: None,
            shield_provider: None,
        }
    }

    /// Register element data provider
    pub fn register_element_provider(&mut self, provider: Box<dyn ElementDataProvider>) {
        self.element_provider = Some(Arc::from(provider));
    }

    /// Register resource data provider
    pub fn register_resource_provider(&mut self, provider: Box<dyn ResourceDataProvider>) {
        self.resource_provider = Some(Arc::from(provider));
    }

    /// Register category data provider
    pub fn register_category_provider(&mut self, provider: Box<dyn CategoryDataProvider>) {
        self.category_provider = Some(Arc::from(provider));
    }

    /// Register actor data provider
    pub fn register_actor_provider(&mut self, provider: Box<dyn ActorDataProvider>) {
        self.actor_provider = Some(Arc::from(provider));
    }

    /// Register status data provider
    pub fn register_status_provider(&mut self, provider: Box<dyn StatusDataProvider>) {
        self.status_provider = Some(Arc::from(provider));
    }

    /// Register action data provider
    pub fn register_action_provider(&mut self, provider: Box<dyn ActionDataProvider>) {
        self.action_provider = Some(Arc::from(provider));
    }

    /// Register location data provider
    pub fn register_location_provider(&mut self, provider: Box<dyn LocationDataProvider>) {
        self.location_provider = Some(Arc::from(provider));
    }

    /// Register event data provider
    pub fn register_event_provider(&mut self, provider: Box<dyn EventDataProvider>) {
        self.event_provider = Some(Arc::from(provider));
    }

    /// Register quest data provider
    pub fn register_quest_provider(&mut self, provider: Box<dyn QuestDataProvider>) {
        self.quest_provider = Some(Arc::from(provider));
    }

    /// Register item data provider
    pub fn register_item_provider(&mut self, provider: Box<dyn ItemDataProvider>) {
        self.item_provider = Some(Arc::from(provider));
    }

    /// Register shield data provider
    pub fn register_shield_provider(&mut self, provider: Box<dyn ShieldDataProvider>) {
        self.shield_provider = Some(Arc::from(provider));
    }

    /// Get element data provider
    pub fn get_element_provider(&self) -> Option<Arc<dyn ElementDataProvider>> {
        self.element_provider.clone()
    }

    /// Get resource data provider
    pub fn get_resource_provider(&self) -> Option<Arc<dyn ResourceDataProvider>> {
        self.resource_provider.clone()
    }

    /// Get category data provider
    pub fn get_category_provider(&self) -> Option<Arc<dyn CategoryDataProvider>> {
        self.category_provider.clone()
    }

    /// Get actor data provider
    pub fn get_actor_provider(&self) -> Option<Arc<dyn ActorDataProvider>> {
        self.actor_provider.clone()
    }

    /// Get status data provider
    pub fn get_status_provider(&self) -> Option<Arc<dyn StatusDataProvider>> {
        self.status_provider.clone()
    }

    /// Get action data provider
    pub fn get_action_provider(&self) -> Option<Arc<dyn ActionDataProvider>> {
        self.action_provider.clone()
    }

    /// Get location data provider
    pub fn get_location_provider(&self) -> Option<Arc<dyn LocationDataProvider>> {
        self.location_provider.clone()
    }

    /// Get event data provider
    pub fn get_event_provider(&self) -> Option<Arc<dyn EventDataProvider>> {
        self.event_provider.clone()
    }

    /// Get quest data provider
    pub fn get_quest_provider(&self) -> Option<Arc<dyn QuestDataProvider>> {
        self.quest_provider.clone()
    }

    /// Get item data provider
    pub fn get_item_provider(&self) -> Option<Arc<dyn ItemDataProvider>> {
        self.item_provider.clone()
    }

    /// Get shield data provider
    pub fn get_shield_provider(&self) -> Option<Arc<dyn ShieldDataProvider>> {
        self.shield_provider.clone()
    }
}

impl Default for DataProviderRegistry {
    fn default() -> Self {
        Self::new()
    }
}
