//! Data Providers for Condition Core Integration
//! 
//! This module implements data providers that allow Condition Core to access
//! Actor Core data for condition evaluation.

use condition_core::*;
use std::sync::Arc;

// Error conversion helper functions
fn map_actor_core_error(err: Box<dyn std::error::Error + Send + Sync>) -> ConditionError {
    ConditionError::ConfigError {
        message: err.to_string(),
    }
}

/// Actor data provider for Condition Core
/// 
/// This provider allows Condition Core to access actor-specific data
/// such as resources, stats, and status effects.
pub struct ActorDataProvider {
    actor_repository: Arc<dyn ActorRepository>,
    stat_cache: Arc<dyn StatCache>,
}

impl ActorDataProvider {
    /// Create a new Actor data provider
    pub fn new(
        actor_repository: Arc<dyn ActorRepository>,
        stat_cache: Arc<dyn StatCache>,
    ) -> Self {
        Self {
            actor_repository,
            stat_cache,
        }
    }
}

#[async_trait::async_trait]
impl condition_core::ActorDataProvider for ActorDataProvider {
    /// Get actor resource value (generic) - không hard-code specific resources
    async fn get_actor_resource(&self, resource_type: &str, actor_id: &str) -> ConditionResult<f64> {
        let _actor = self.actor_repository.get_actor(actor_id).await
            .map_err(|e| ConditionError::ConfigError { message: e.to_string() })?;
        let snapshot = self.stat_cache.get_snapshot(actor_id).await
            .map_err(|e| ConditionError::ConfigError { message: e.to_string() })?;
        
        // Lấy resource value từ snapshot, fallback về 0.0 nếu không có
        Ok(snapshot.primary.get(resource_type).unwrap_or(&0.0).clone())
    }
    
    /// Get actor stat value (generic) - không hard-code specific stats
    async fn get_actor_stat(&self, stat_name: &str, actor_id: &str) -> ConditionResult<f64> {
        let _actor = self.actor_repository.get_actor(actor_id).await
            .map_err(map_actor_core_error)?;
        let snapshot = self.stat_cache.get_snapshot(actor_id).await
            .map_err(map_actor_core_error)?;
        
        // Lấy stat value từ snapshot, fallback về 0.0 nếu không có
        Ok(snapshot.primary.get(stat_name).unwrap_or(&0.0).clone())
    }
    
    /// Get actor derived stat value
    async fn get_actor_derived_stat(&self, stat_name: &str, actor_id: &str) -> ConditionResult<f64> {
        let _actor = self.actor_repository.get_actor(actor_id).await
            .map_err(map_actor_core_error)?;
        let snapshot = self.stat_cache.get_snapshot(actor_id).await
            .map_err(map_actor_core_error)?;
        
        // Lấy derived stat value từ snapshot, fallback về 0.0 nếu không có
        Ok(snapshot.derived.get(stat_name).unwrap_or(&0.0).clone())
    }
    
    /// Actor metadata - không phụ thuộc vào specific cultivation system
    async fn get_actor_race(&self, actor_id: &str) -> ConditionResult<String> {
        let actor = self.actor_repository.get_actor(actor_id).await
            .map_err(map_actor_core_error)?;
        Ok(actor.race.clone())
    }
    
    /// Combat state - generic state check
    async fn is_actor_in_combat(&self, actor_id: &str) -> ConditionResult<bool> {
        let actor = self.actor_repository.get_actor(actor_id).await
            .map_err(map_actor_core_error)?;
        Ok(actor.is_in_combat())
    }
    
    /// Generic status effects - không hard-code "buffs"
    async fn has_actor_status_effects(&self, status_type: &str, actor_id: &str) -> ConditionResult<bool> {
        let actor = self.actor_repository.get_actor(actor_id).await
            .map_err(map_actor_core_error)?;
        
        // Check trong actor.data với key tương ứng với status_type
        let has_status = actor.data.get(status_type)
            .and_then(|v| v.as_array())
            .map(|statuses| !statuses.is_empty())
            .unwrap_or(false);
        
        Ok(has_status)
    }
    
    /// Generic status effect count
    async fn get_actor_status_effect_count(&self, status_type: &str, actor_id: &str) -> ConditionResult<i64> {
        let actor = self.actor_repository.get_actor(actor_id).await
            .map_err(map_actor_core_error)?;
        
        let count = actor.data.get(status_type)
            .and_then(|v| v.as_array())
            .map(|statuses| statuses.len())
            .unwrap_or(0);
        
        Ok(count as i64)
    }
    
    /// Generic status effect count by category/tag
    async fn get_actor_status_effect_count_by_category(&self, status_type: &str, category: &str, actor_id: &str) -> ConditionResult<i64> {
        let actor = self.actor_repository.get_actor(actor_id).await
            .map_err(map_actor_core_error)?;
        
        let count = actor.data.get(status_type)
            .and_then(|v| v.as_array())
            .map(|statuses| {
                statuses.iter()
                    .filter(|status| {
                        status.get("category")
                            .and_then(|c| c.as_str())
                            .map(|c| c == category)
                            .unwrap_or(false)
                    })
                    .count()
            })
            .unwrap_or(0);
        
        Ok(count as i64)
    }
}

/// Resource data provider for Condition Core
/// 
/// This provider allows Condition Core to access resource data
/// such as current values, max values, and thresholds.
pub struct ResourceDataProvider {
    resource_manager: Arc<dyn ResourceManager>,
    #[allow(dead_code)]
    stat_cache: Arc<dyn StatCache>,
}

impl ResourceDataProvider {
    /// Create a new Resource data provider
    pub fn new(
        resource_manager: Arc<dyn ResourceManager>,
        stat_cache: Arc<dyn StatCache>,
    ) -> Self {
        Self {
            resource_manager,
            stat_cache,
        }
    }
}

#[async_trait::async_trait]
impl condition_core::ResourceDataProvider for ResourceDataProvider {
    async fn get_resource_value(&self, resource_type: &str, actor_id: &str) -> ConditionResult<f64> {
        let resource = self.resource_manager.get_resource(actor_id, resource_type).await
            .map_err(map_actor_core_error)?;
        Ok(resource.current_value)
    }
    
    async fn get_resource_max(&self, resource_type: &str, actor_id: &str) -> ConditionResult<f64> {
        let resource = self.resource_manager.get_resource(actor_id, resource_type).await
            .map_err(map_actor_core_error)?;
        Ok(resource.max_value)
    }
    
    async fn get_resource_percentage(&self, resource_type: &str, actor_id: &str) -> ConditionResult<f64> {
        let resource = self.resource_manager.get_resource(actor_id, resource_type).await
            .map_err(map_actor_core_error)?;
        if resource.max_value > 0.0 {
            Ok(resource.current_value / resource.max_value)
        } else {
            Ok(0.0)
        }
    }
    
    async fn is_resource_empty(&self, resource_type: &str, actor_id: &str) -> ConditionResult<bool> {
        let resource = self.resource_manager.get_resource(actor_id, resource_type).await
            .map_err(map_actor_core_error)?;
        Ok(resource.current_value <= 0.0)
    }
    
    /// Generic resource state check với custom threshold
    async fn is_resource_below_threshold(&self, resource_type: &str, threshold: f64, actor_id: &str) -> ConditionResult<bool> {
        let resource = self.resource_manager.get_resource(actor_id, resource_type).await
            .map_err(map_actor_core_error)?;
        Ok(resource.current_value < threshold)
    }
    
    /// Generic resource state check với custom threshold (above)
    async fn is_resource_above_threshold(&self, resource_type: &str, threshold: f64, actor_id: &str) -> ConditionResult<bool> {
        let resource = self.resource_manager.get_resource(actor_id, resource_type).await
            .map_err(map_actor_core_error)?;
        Ok(resource.current_value > threshold)
    }
    
    /// Generic resource state check với percentage threshold (below)
    async fn is_resource_below_percentage(&self, resource_type: &str, percentage: f64, actor_id: &str) -> ConditionResult<bool> {
        let resource = self.resource_manager.get_resource(actor_id, resource_type).await
            .map_err(map_actor_core_error)?;
        if resource.max_value > 0.0 {
            let threshold = resource.max_value * (percentage / 100.0);
            Ok(resource.current_value < threshold)
        } else {
            Ok(false)
        }
    }
    
    /// Generic resource state check với percentage threshold (above)
    async fn is_resource_above_percentage(&self, resource_type: &str, percentage: f64, actor_id: &str) -> ConditionResult<bool> {
        let resource = self.resource_manager.get_resource(actor_id, resource_type).await
            .map_err(map_actor_core_error)?;
        if resource.max_value > 0.0 {
            let threshold = resource.max_value * (percentage / 100.0);
            Ok(resource.current_value > threshold)
        } else {
            Ok(false)
        }
    }
    
    async fn list_resources(&self) -> ConditionResult<Vec<String>> {
        // This would typically come from a resource registry
        // For now, return common resource types
        Ok(vec![
            "health".to_string(),
            "mana".to_string(),
            "stamina".to_string(),
            "sanity".to_string(),
        ])
    }
}

/// Category data provider for Condition Core
/// 
/// This provider allows Condition Core to access category data
/// such as items, availability, and counts.
pub struct CategoryDataProvider {
    #[allow(dead_code)]
    category_registry: Arc<dyn CategoryRegistry>,
    actor_inventory: Arc<dyn ActorInventory>,
}

impl CategoryDataProvider {
    /// Create a new Category data provider
    pub fn new(
        category_registry: Arc<dyn CategoryRegistry>,
        actor_inventory: Arc<dyn ActorInventory>,
    ) -> Self {
        Self {
            category_registry,
            actor_inventory,
        }
    }
}

#[async_trait::async_trait]
impl condition_core::CategoryDataProvider for CategoryDataProvider {
    async fn has_category_item(&self, category: &str, actor_id: &str) -> ConditionResult<bool> {
        let items = self.actor_inventory.get_items_by_category(actor_id, category).await
            .map_err(map_actor_core_error)?;
        Ok(!items.is_empty())
    }
    
    async fn get_category_item_count(&self, category: &str, actor_id: &str) -> ConditionResult<i64> {
        let items = self.actor_inventory.get_items_by_category(actor_id, category).await
            .map_err(map_actor_core_error)?;
        Ok(items.len() as i64)
    }
    
    async fn is_category_available(&self, _category: &str, _actor_id: &str) -> ConditionResult<bool> {
        // Check if category is available for the actor
        // This could involve checking actor stats, level, etc.
        Ok(true) // Simplified for now
    }
    
    async fn is_category_blocked(&self, _category: &str, _actor_id: &str) -> ConditionResult<bool> {
        // Check if category is blocked for the actor
        // This could involve checking debuffs, restrictions, etc.
        Ok(false) // Simplified for now
    }
    
    async fn list_categories(&self) -> ConditionResult<Vec<String>> {
        // This would typically come from a category registry
        // For now, return common category types
        Ok(vec![
            "weapon".to_string(),
            "armor".to_string(),
            "potion".to_string(),
            "combat".to_string(),
            "magic".to_string(),
        ])
    }
}

// Placeholder traits for Actor Core interfaces
// These would be defined in the actual Actor Core implementation

/// Trait for actor repository
#[async_trait::async_trait]
pub trait ActorRepository: Send + Sync {
    async fn get_actor(&self, actor_id: &str) -> Result<Actor, Box<dyn std::error::Error + Send + Sync>>;
}

/// Trait for stat cache
#[async_trait::async_trait]
pub trait StatCache: Send + Sync {
    async fn get_snapshot(&self, actor_id: &str) -> Result<Snapshot, Box<dyn std::error::Error + Send + Sync>>;
}

/// Trait for resource manager
#[async_trait::async_trait]
pub trait ResourceManager: Send + Sync {
    async fn get_resource(&self, actor_id: &str, resource_type: &str) -> Result<Resource, Box<dyn std::error::Error + Send + Sync>>;
}

/// Trait for category registry
#[async_trait::async_trait]
pub trait CategoryRegistry: Send + Sync {
    async fn get_required_stats(&self, category: &str) -> Result<Vec<(String, f64)>, Box<dyn std::error::Error + Send + Sync>>;
}

/// Trait for actor inventory
#[async_trait::async_trait]
pub trait ActorInventory: Send + Sync {
    async fn get_items_by_category(&self, actor_id: &str, category: &str) -> Result<Vec<Item>, Box<dyn std::error::Error + Send + Sync>>;
}

// Placeholder types for Actor Core
#[derive(Debug, Clone)]
pub struct Actor {
    pub id: String,
    pub race: String,
    pub data: serde_json::Value,
}

impl Actor {
    pub fn new(id: String, race: String) -> Self {
        Self {
            id,
            race,
            data: serde_json::Value::Object(serde_json::Map::new()),
        }
    }
    
    pub fn is_in_combat(&self) -> bool {
        self.data.get("in_combat")
            .and_then(|v| v.as_bool())
            .unwrap_or(false)
    }
}

#[derive(Debug, Clone)]
pub struct Snapshot {
    pub primary: std::collections::HashMap<String, f64>,
    pub derived: std::collections::HashMap<String, f64>,
}

impl Default for Snapshot {
    fn default() -> Self {
        Self {
            primary: std::collections::HashMap::new(),
            derived: std::collections::HashMap::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Resource {
    pub name: String,
    pub current_value: f64,
    pub max_value: f64,
}

#[derive(Debug, Clone)]
pub struct Item {
    pub id: String,
    pub category: String,
}
