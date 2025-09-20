//! Simple Condition Core Integration Example
//! 
//! This example demonstrates basic integration between Actor Core and Condition Core
//! using mock data providers.

use condition_core::*;
use actor_core::registry::{RegistryManager, ResourceDefinition, CategoryDefinition, TagDefinition, ResourceDefinitionBuilder, CategoryDefinitionBuilder, TagDefinitionBuilder};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Simple Actor Core + Condition Core Integration ===\n");

    // Create runtime registry manager
    let registry_manager = RegistryManager::new();
    let resource_registry = registry_manager.get_resource_registry();
    let category_registry = registry_manager.get_category_registry();
    let tag_registry = registry_manager.get_tag_registry();

    // Register some basic resources and categories
    println!("📦 Setting up runtime registries...");
    setup_runtime_registries(&resource_registry, &category_registry, &tag_registry).await?;

    // Create mock data providers with runtime registries
    let mut data_registry = DataProviderRegistry::new();
    data_registry.register_actor_provider(Box::new(MockActorDataProvider));
    data_registry.register_resource_provider(Box::new(MockResourceDataProvider::new(resource_registry)));
    data_registry.register_category_provider(Box::new(MockCategoryDataProvider::new(category_registry)));

    // Create condition resolver
    let condition_resolver = ConditionResolver::new(data_registry);

    // Create test context
    let context = ConditionContext {
        target: ActorTarget {
            id: "player1".to_string(),
        },
        world_id: "test_world".to_string(),
        current_time: std::time::SystemTime::now(),
        current_weather: WeatherType::Clear,
        world_state: WorldState {
            time_of_day: 12.0,
            season: "spring".to_string(),
            temperature: 20.0,
            humidity: 0.5,
        },
    };

    // Test 1: Health condition
    println!("1. Testing health condition...");
    let health_condition = ConditionConfig {
        condition_id: "health_check".to_string(),
        function_name: "get_actor_resource".to_string(),
        operator: ConditionOperator::GreaterThan,
        value: ConditionValue::Float(50.0),
        parameters: vec![ConditionParameter::String("health".to_string())],
    };

    let health_result = condition_resolver.resolve_condition(&health_condition, &context).await?;
    println!("   Health > 50: {}", health_result);

    // Test 2: Mana condition
    println!("\n2. Testing mana condition...");
    let mana_condition = ConditionConfig {
        condition_id: "mana_check".to_string(),
        function_name: "get_actor_resource".to_string(),
        operator: ConditionOperator::LessThan,
        value: ConditionValue::Float(30.0),
        parameters: vec![ConditionParameter::String("mana".to_string())],
    };

    let mana_result = condition_resolver.resolve_condition(&mana_condition, &context).await?;
    println!("   Mana < 30: {}", mana_result);

    // Test 3: Resource percentage condition
    println!("\n3. Testing resource percentage condition...");
    let resource_percentage_condition = ConditionConfig {
        condition_id: "low_health".to_string(),
        function_name: "is_resource_below_percentage".to_string(),
        operator: ConditionOperator::Equal,
        value: ConditionValue::Boolean(true),
        parameters: vec![
            ConditionParameter::String("health".to_string()),
            ConditionParameter::Float(25.0), // 25%
        ],
    };

    let low_health_result = condition_resolver.resolve_condition(&resource_percentage_condition, &context).await?;
    println!("   Health < 25%: {}", low_health_result);

    // Test 4: Status effect condition
    println!("\n4. Testing status effect condition...");
    let buff_condition = ConditionConfig {
        condition_id: "has_buffs".to_string(),
        function_name: "has_actor_status_effects".to_string(),
        operator: ConditionOperator::Equal,
        value: ConditionValue::Boolean(true),
        parameters: vec![ConditionParameter::String("buffs".to_string())],
    };

    let buff_result = condition_resolver.resolve_condition(&buff_condition, &context).await?;
    println!("   Has buffs: {}", buff_result);

    // Test 5: Category condition
    println!("\n5. Testing category condition...");
    let weapon_condition = ConditionConfig {
        condition_id: "has_weapon".to_string(),
        function_name: "has_category_item".to_string(),
        operator: ConditionOperator::Equal,
        value: ConditionValue::Boolean(true),
        parameters: vec![ConditionParameter::String("weapon".to_string())],
    };

    let weapon_result = condition_resolver.resolve_condition(&weapon_condition, &context).await?;
    println!("   Has weapon: {}", weapon_result);

    // Test 6: Multiple conditions with chain
    println!("\n6. Testing condition chain...");
    let chain_config = ConditionChainConfig {
        chain_id: "can_cast_spell".to_string(),
        logic: ChainLogic::And,
        conditions: vec![
            ConditionConfig {
                condition_id: "has_mana".to_string(),
                function_name: "get_actor_resource".to_string(),
                operator: ConditionOperator::GreaterThan,
                value: ConditionValue::Float(25.0),
                parameters: vec![ConditionParameter::String("mana".to_string())],
            },
            ConditionConfig {
                condition_id: "has_weapon".to_string(),
                function_name: "has_category_item".to_string(),
                operator: ConditionOperator::Equal,
                value: ConditionValue::Boolean(true),
                parameters: vec![ConditionParameter::String("weapon".to_string())],
            },
        ],
    };

    let chain_result = condition_resolver.resolve_condition_chain(&chain_config, &context).await?;
    println!("   Can cast spell (mana > 25 AND has weapon): {}", chain_result);

    println!("\n=== Integration Test Complete ===");
    println!("✅ All condition evaluations working");
    println!("✅ Data providers integrated successfully");
    println!("✅ Condition chains working");
    println!("✅ Actor Core + Condition Core integration successful");

    Ok(())
}

// Mock implementations for demonstration
struct MockActorDataProvider;

struct MockResourceDataProvider {
    resource_registry: std::sync::Arc<dyn actor_core::registry::ResourceRegistry>,
}

impl MockResourceDataProvider {
    fn new(resource_registry: std::sync::Arc<dyn actor_core::registry::ResourceRegistry>) -> Self {
        Self { resource_registry }
    }
}

struct MockCategoryDataProvider {
    category_registry: std::sync::Arc<dyn actor_core::registry::CategoryRegistry>,
}

impl MockCategoryDataProvider {
    fn new(category_registry: std::sync::Arc<dyn actor_core::registry::CategoryRegistry>) -> Self {
        Self { category_registry }
    }
}

#[async_trait::async_trait]
impl condition_core::ActorDataProvider for MockActorDataProvider {
    async fn get_actor_resource(&self, resource_type: &str, _actor_id: &str) -> ConditionResult<f64> {
        let value = match resource_type {
            "health" => 100.0,
            "mana" => 50.0,
            "stamina" => 75.0,
            _ => 0.0,
        };
        Ok(value)
    }

    async fn get_actor_stat(&self, stat_name: &str, _actor_id: &str) -> ConditionResult<f64> {
        let value = match stat_name {
            "strength" => 80.0,
            "intelligence" => 90.0,
            "agility" => 70.0,
            _ => 50.0,
        };
        Ok(value)
    }

    async fn get_actor_derived_stat(&self, stat_name: &str, _actor_id: &str) -> ConditionResult<f64> {
        let value = match stat_name {
            "damage" => 120.0,
            "defense" => 100.0,
            "speed" => 85.0,
            _ => 50.0,
        };
        Ok(value)
    }

    async fn get_actor_race(&self, _actor_id: &str) -> ConditionResult<String> {
        Ok("human".to_string())
    }

    async fn is_actor_in_combat(&self, _actor_id: &str) -> ConditionResult<bool> {
        Ok(false)
    }

    async fn has_actor_status_effects(&self, status_type: &str, _actor_id: &str) -> ConditionResult<bool> {
        let has_status = match status_type {
            "buffs" => true,
            "debuffs" => false,
            "ailments" => false,
            _ => false,
        };
        Ok(has_status)
    }

    async fn get_actor_status_effect_count(&self, status_type: &str, _actor_id: &str) -> ConditionResult<i64> {
        let count = match status_type {
            "buffs" => 2,
            "debuffs" => 0,
            "ailments" => 0,
            _ => 0,
        };
        Ok(count)
    }

    async fn get_actor_status_effect_count_by_category(&self, status_type: &str, category: &str, _actor_id: &str) -> ConditionResult<i64> {
        let count = match (status_type, category) {
            ("buffs", "combat") => 1,
            ("buffs", "magic") => 1,
            _ => 0,
        };
        Ok(count)
    }
}

#[async_trait::async_trait]
impl condition_core::ResourceDataProvider for MockResourceDataProvider {
    async fn get_resource_value(&self, resource_type: &str, _actor_id: &str) -> ConditionResult<f64> {
        let value = match resource_type {
            "health" => 100.0,
            "mana" => 50.0,
            "stamina" => 75.0,
            _ => 0.0,
        };
        Ok(value)
    }

    async fn get_resource_max(&self, resource_type: &str, _actor_id: &str) -> ConditionResult<f64> {
        let max_value = match resource_type {
            "health" => 100.0,
            "mana" => 100.0,
            "stamina" => 100.0,
            _ => 100.0,
        };
        Ok(max_value)
    }

    async fn get_resource_percentage(&self, resource_type: &str, _actor_id: &str) -> ConditionResult<f64> {
        let (current, max) = match resource_type {
            "health" => (100.0, 100.0),
            "mana" => (50.0, 100.0),
            "stamina" => (75.0, 100.0),
            _ => (0.0, 100.0),
        };
        Ok(current / max)
    }

    async fn is_resource_empty(&self, resource_type: &str, _actor_id: &str) -> ConditionResult<bool> {
        let current = match resource_type {
            "health" => 100.0,
            "mana" => 50.0,
            "stamina" => 75.0,
            _ => 0.0,
        };
        Ok(current <= 0.0)
    }

    async fn is_resource_below_threshold(&self, resource_type: &str, threshold: f64, _actor_id: &str) -> ConditionResult<bool> {
        let current = match resource_type {
            "health" => 100.0,
            "mana" => 50.0,
            "stamina" => 75.0,
            _ => 0.0,
        };
        Ok(current < threshold)
    }

    async fn is_resource_above_threshold(&self, resource_type: &str, threshold: f64, _actor_id: &str) -> ConditionResult<bool> {
        let current = match resource_type {
            "health" => 100.0,
            "mana" => 50.0,
            "stamina" => 75.0,
            _ => 0.0,
        };
        Ok(current > threshold)
    }

    async fn is_resource_below_percentage(&self, resource_type: &str, percentage: f64, _actor_id: &str) -> ConditionResult<bool> {
        let (current, max) = match resource_type {
            "health" => (100.0, 100.0),
            "mana" => (50.0, 100.0),
            "stamina" => (75.0, 100.0),
            _ => (0.0, 100.0),
        };
        let current_percentage = (current / max) * 100.0;
        Ok(current_percentage < percentage)
    }

    async fn is_resource_above_percentage(&self, resource_type: &str, percentage: f64, _actor_id: &str) -> ConditionResult<bool> {
        let (current, max) = match resource_type {
            "health" => (100.0, 100.0),
            "mana" => (50.0, 100.0),
            "stamina" => (75.0, 100.0),
            _ => (0.0, 100.0),
        };
        let current_percentage = (current / max) * 100.0;
        Ok(current_percentage > percentage)
    }

    async fn list_resources(&self) -> ConditionResult<Vec<String>> {
        // Use the runtime registry instead of hardcoded values
        let resources = self.resource_registry.get_all_resources().await
            .map_err(|e| ConditionError::ConfigError { message: e.to_string() })?;
        Ok(resources.into_iter().map(|r| r.id).collect())
    }
}

#[async_trait::async_trait]
impl condition_core::CategoryDataProvider for MockCategoryDataProvider {
    async fn has_category_item(&self, category: &str, _actor_id: &str) -> ConditionResult<bool> {
        let has_item = match category {
            "weapon" => true,
            "armor" => true,
            "potion" => false,
            _ => false,
        };
        Ok(has_item)
    }

    async fn get_category_item_count(&self, category: &str, _actor_id: &str) -> ConditionResult<i64> {
        let count = match category {
            "weapon" => 1,
            "armor" => 2,
            "potion" => 0,
            _ => 0,
        };
        Ok(count)
    }

    async fn is_category_available(&self, _category: &str, _actor_id: &str) -> ConditionResult<bool> {
        Ok(true)
    }

    async fn is_category_blocked(&self, _category: &str, _actor_id: &str) -> ConditionResult<bool> {
        Ok(false)
    }

    async fn list_categories(&self) -> ConditionResult<Vec<String>> {
        // Use the runtime registry instead of hardcoded values
        let categories = self.category_registry.get_all_categories().await
            .map_err(|e| ConditionError::ConfigError { message: e.to_string() })?;
        Ok(categories.into_iter().map(|c| c.id).collect())
    }
}

async fn setup_runtime_registries(
    resource_registry: &std::sync::Arc<dyn actor_core::registry::ResourceRegistry>,
    category_registry: &std::sync::Arc<dyn actor_core::registry::CategoryRegistry>,
    tag_registry: &std::sync::Arc<dyn actor_core::registry::TagRegistry>,
) -> Result<(), Box<dyn std::error::Error>> {
    // Register basic resources
    let health = ResourceDefinition::create_health_resource("mock_subsystem");
    resource_registry.register_resource(health).await?;
    
    let mana = ResourceDefinition::create_mana_resource("mock_subsystem");
    resource_registry.register_resource(mana).await?;
    
    let stamina = ResourceDefinition::create_stamina_resource("mock_subsystem");
    resource_registry.register_resource(stamina).await?;

    // Register basic categories
    let weapon = CategoryDefinition::create_weapon_category("mock_subsystem");
    category_registry.register_category(weapon).await?;
    
    let armor = CategoryDefinition::create_armor_category("mock_subsystem");
    category_registry.register_category(armor).await?;
    
    let potion = CategoryDefinition::create_potion_category("mock_subsystem");
    category_registry.register_category(potion).await?;

    // Register basic tags
    let vital = TagDefinition::create_vital_tag("mock_subsystem");
    tag_registry.register_tag(vital).await?;
    
    let magic = TagDefinition::create_magic_tag("mock_subsystem");
    tag_registry.register_tag(magic).await?;

    println!("  ✅ Registered resources, categories, and tags");
    Ok(())
}
