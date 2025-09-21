//! Example of dependency injection for Condition Core

use condition_core::*;
use std::time::SystemTime;

// Mock implementation of ElementDataProvider by Element Core
struct MockElementDataProvider;

#[async_trait::async_trait]
impl ElementDataProvider for MockElementDataProvider {
    async fn get_element_mastery(&self, element_id: &str, _actor_id: &str) -> ConditionResult<f64> {
        // Element Core provides this data
        let mastery = match element_id {
            "fire" => 150.0,
            "water" => 120.0,
            "earth" => 100.0,
            "air" => 80.0,
            "light" => 200.0,
            "dark" => 90.0,
            "ice" => 110.0,
            "lightning" => 130.0,
            "shadow" => 95.0, // New element added by Element Core
            "metal" => 105.0, // New element added by Element Core
            _ => 0.0,
        };
        Ok(mastery)
    }

    async fn get_element_resistance(&self, element_id: &str, _actor_id: &str) -> ConditionResult<f64> {
        let resistance = match element_id {
            "fire" => 0.2,
            "water" => 0.5,
            "earth" => 0.3,
            "air" => 0.1,
            "light" => 0.8,
            "dark" => 0.4,
            "ice" => 0.6,
            "lightning" => 0.0,
            "shadow" => 0.3,
            "metal" => 0.7,
            _ => 0.0,
        };
        Ok(resistance)
    }

    async fn has_element_affinity(&self, element_id: &str, _actor_id: &str) -> ConditionResult<bool> {
        let has_affinity = match element_id {
            "fire" => true,
            "water" => true,
            "earth" => false,
            "air" => false,
            "light" => true,
            "dark" => false,
            "ice" => true,
            "lightning" => true,
            "shadow" => false,
            "metal" => false,
            _ => false,
        };
        Ok(has_affinity)
    }

    async fn is_element_weakness(&self, element_id: &str, _actor_id: &str) -> ConditionResult<bool> {
        let is_weakness = match element_id {
            "fire" => false,
            "water" => false,
            "earth" => false,
            "air" => true,
            "light" => false,
            "dark" => false,
            "ice" => false,
            "lightning" => true,
            "shadow" => false,
            "metal" => false,
            _ => false,
        };
        Ok(is_weakness)
    }

    async fn get_element_interaction(&self, source_element: &str, target_element: &str) -> ConditionResult<String> {
        let interaction = match (source_element, target_element) {
            ("fire", "water") => "suppress",
            ("water", "fire") => "extinguish",
            ("fire", "ice") => "melt",
            ("ice", "fire") => "freeze",
            ("earth", "air") => "block",
            ("air", "earth") => "erode",
            ("light", "dark") => "purify",
            ("dark", "light") => "corrupt",
            ("lightning", "water") => "conduct",
            ("water", "lightning") => "conduct",
            ("shadow", "light") => "hide",
            ("light", "shadow") => "illuminate",
            ("metal", "lightning") => "attract",
            ("lightning", "metal") => "attract",
            _ => "neutral",
        };
        Ok(interaction.to_string())
    }

    async fn list_elements(&self) -> ConditionResult<Vec<String>> {
        Ok(vec![
            "fire".to_string(),
            "water".to_string(),
            "earth".to_string(),
            "air".to_string(),
            "light".to_string(),
            "dark".to_string(),
            "ice".to_string(),
            "lightning".to_string(),
            "shadow".to_string(),
            "metal".to_string(),
        ])
    }    
    // Element interaction functions
    async fn is_element_same_category(&self, element1: &str, element2: &str) -> ConditionResult<bool> {
        Ok(element1 == element2) // Simple mock implementation
    }
    
    async fn is_element_generating(&self, source_element: &str, target_element: &str) -> ConditionResult<bool> {
        Ok(false) // Simple mock implementation
    }
    
    async fn is_element_overcoming(&self, source_element: &str, target_element: &str) -> ConditionResult<bool> {
        Ok(false) // Simple mock implementation
    }
    
    async fn is_element_neutral(&self, source_element: &str, target_element: &str) -> ConditionResult<bool> {
        Ok(true) // Simple mock implementation
    }
    
    // Element status functions
    async fn has_element_status_effect(&self, element_id: &str, status_id: &str, _actor_id: &str) -> ConditionResult<bool> {
        Ok(false) // Simple mock implementation
    }
    
    async fn get_element_status_effect_count(&self, element_id: &str, status_id: &str, _actor_id: &str) -> ConditionResult<i64> {
        Ok(0) // Simple mock implementation
    }
    
    async fn is_element_status_effect_active(&self, element_id: &str, status_id: &str, actor_id: &str) -> ConditionResult<bool> {
        self.has_element_status_effect(element_id, status_id, actor_id).await
    }
    
    // Element resource functions
    async fn has_element_resource(&self, element_id: &str, resource_type: &str, _actor_id: &str) -> ConditionResult<bool> {
        Ok(false) // Simple mock implementation
    }
    
    async fn get_element_resource_value(&self, element_id: &str, resource_type: &str, _actor_id: &str) -> ConditionResult<f64> {
        Ok(0.0) // Simple mock implementation
    }
    
    async fn is_element_resource_below_threshold(&self, element_id: &str, resource_type: &str, threshold: f64, actor_id: &str) -> ConditionResult<bool> {
        let value = self.get_element_resource_value(element_id, resource_type, actor_id).await?;
        Ok(value < threshold)
    }
    
    async fn is_element_resource_above_threshold(&self, element_id: &str, resource_type: &str, threshold: f64, actor_id: &str) -> ConditionResult<bool> {
        let value = self.get_element_resource_value(element_id, resource_type, actor_id).await?;
        Ok(value > threshold)
    }
    
    // Hybrid element functions
    async fn has_hybrid_element(&self, hybrid_id: &str, _actor_id: &str) -> ConditionResult<bool> {
        Ok(false) // Simple mock implementation
    }
    
    async fn is_hybrid_element_activated(&self, hybrid_id: &str, _actor_id: &str) -> ConditionResult<bool> {
        Ok(false) // Simple mock implementation
    }
    
    async fn get_hybrid_element_parents(&self, hybrid_id: &str) -> ConditionResult<Vec<String>> {
        Ok(vec![]) // Simple mock implementation
    }
    
    async fn list_hybrid_elements(&self) -> ConditionResult<Vec<String>> {
        Ok(vec![]) // Simple mock implementation
    }
    
    // Element derived stats functions
    async fn get_element_derived_stat(&self, element_id: &str, stat_name: &str, _actor_id: &str) -> ConditionResult<f64> {
        Ok(0.0) // Simple mock implementation
    }
    
    async fn has_element_derived_stat(&self, element_id: &str, stat_name: &str, actor_id: &str) -> ConditionResult<bool> {
        let value = self.get_element_derived_stat(element_id, stat_name, actor_id).await?;
        Ok(value > 0.0)
    }
    
    async fn list_element_derived_stats(&self, element_id: &str) -> ConditionResult<Vec<String>> {
        Ok(vec![]) // Simple mock implementation
    }
}

// Mock implementation of ResourceDataProvider by Resource Core
struct MockResourceDataProvider;

#[async_trait::async_trait]
impl ResourceDataProvider for MockResourceDataProvider {
    async fn get_resource_value(&self, resource_id: &str, _actor_id: &str) -> ConditionResult<f64> {
        let value = match resource_id {
            "health" => 100.0,
            "mana" => 50.0,
            "stamina" => 75.0,
            "lifespan" => 1000.0,
            "energy" => 80.0,
            "focus" => 60.0,
            "chi" => 40.0,
            "sanity" => 100.0, // New resource added by Resource Core
            "karma" => 0.0,    // New resource added by Resource Core
            "luck" => 50.0,    // New resource added by Resource Core
            _ => 0.0,
        };
        Ok(value)
    }

    async fn get_resource_max(&self, resource_id: &str, _actor_id: &str) -> ConditionResult<f64> {
        let max_value = match resource_id {
            "health" => 100.0,
            "mana" => 50.0,
            "stamina" => 100.0,
            "lifespan" => 1000.0,
            "energy" => 100.0,
            "focus" => 100.0,
            "chi" => 100.0,
            "sanity" => 100.0,
            "karma" => 100.0,
            "luck" => 100.0,
            _ => 100.0,
        };
        Ok(max_value)
    }

    async fn get_resource_percentage(&self, resource_id: &str, _actor_id: &str) -> ConditionResult<f64> {
        let (current, max) = match resource_id {
            "health" => (100.0, 100.0),
            "mana" => (15.0, 50.0), // Low mana
            "stamina" => (75.0, 100.0),
            _ => (0.0, 100.0),
        };
        Ok(current / max)
    }

    async fn is_resource_empty(&self, resource_id: &str, _actor_id: &str) -> ConditionResult<bool> {
        let current = match resource_id {
            "health" => 100.0,
            "mana" => 15.0,
            "stamina" => 75.0,
            _ => 0.0,
        };
        Ok(current <= 0.0)
    }

    async fn is_resource_below_threshold(&self, resource_id: &str, threshold: f64, _actor_id: &str) -> ConditionResult<bool> {
        let current = match resource_id {
            "health" => 100.0,
            "mana" => 15.0,
            "stamina" => 75.0,
            _ => 0.0,
        };
        Ok(current < threshold)
    }

    async fn is_resource_above_threshold(&self, resource_id: &str, threshold: f64, _actor_id: &str) -> ConditionResult<bool> {
        let current = match resource_id {
            "health" => 100.0,
            "mana" => 15.0,
            "stamina" => 75.0,
            _ => 0.0,
        };
        Ok(current > threshold)
    }

    async fn is_resource_below_percentage(&self, resource_id: &str, percentage: f64, _actor_id: &str) -> ConditionResult<bool> {
        let (current, max) = match resource_id {
            "health" => (100.0, 100.0),
            "mana" => (15.0, 50.0), // Low mana
            "stamina" => (75.0, 100.0),
            _ => (0.0, 100.0),
        };
        let current_percentage = (current / max) * 100.0;
        Ok(current_percentage < percentage)
    }

    async fn is_resource_above_percentage(&self, resource_id: &str, percentage: f64, _actor_id: &str) -> ConditionResult<bool> {
        let (current, max) = match resource_id {
            "health" => (100.0, 100.0),
            "mana" => (15.0, 50.0), // Low mana
            "stamina" => (75.0, 100.0),
            _ => (0.0, 100.0),
        };
        let current_percentage = (current / max) * 100.0;
        Ok(current_percentage > percentage)
    }

    async fn list_resources(&self) -> ConditionResult<Vec<String>> {
        Ok(vec![
            "health".to_string(),
            "mana".to_string(),
            "stamina".to_string(),
            "lifespan".to_string(),
            "energy".to_string(),
            "focus".to_string(),
            "chi".to_string(),
            "sanity".to_string(),
            "karma".to_string(),
            "luck".to_string(),
        ])
    }
}

// Mock implementation of CategoryDataProvider by Category Core
struct MockCategoryDataProvider;

#[async_trait::async_trait]
impl CategoryDataProvider for MockCategoryDataProvider {
    async fn has_category_item(&self, category_id: &str, _actor_id: &str) -> ConditionResult<bool> {
        let has_item = match category_id {
            "weapon" => true,
            "armor" => true,
            "potion" => true,
            "scroll" => false,
            "gem" => true,
            "food" => false,
            "tool" => true,
            "material" => true,
            "magic_item" => true, // New category added by Category Core
            "artifact" => false,  // New category added by Category Core
            _ => false,
        };
        Ok(has_item)
    }

    async fn get_category_item_count(&self, category_id: &str, _actor_id: &str) -> ConditionResult<i64> {
        let count = match category_id {
            "weapon" => 3,
            "armor" => 5,
            "potion" => 10,
            "scroll" => 0,
            "gem" => 2,
            "food" => 0,
            "tool" => 1,
            "material" => 15,
            "magic_item" => 1,
            "artifact" => 0,
            _ => 0,
        };
        Ok(count)
    }

    async fn is_category_available(&self, category_id: &str, _actor_id: &str) -> ConditionResult<bool> {
        let is_available = match category_id {
            "combat" => true,
            "magic" => true,
            "movement" => true,
            "crafting" => false,
            "trading" => true,
            "quest" => true,
            "social" => true,
            "admin" => false,
            "pvp" => true,      // New category added by Category Core
            "raid" => false,    // New category added by Category Core
            _ => true,
        };
        Ok(is_available)
    }

    async fn is_category_blocked(&self, category_id: &str, _actor_id: &str) -> ConditionResult<bool> {
        let is_blocked = match category_id {
            "combat" => false,
            "magic" => false,
            "movement" => false,
            "crafting" => true,
            "trading" => false,
            "quest" => false,
            "social" => false,
            "admin" => true,
            "pvp" => false,
            "raid" => true,
            _ => false,
        };
        Ok(is_blocked)
    }

    async fn list_categories(&self) -> ConditionResult<Vec<String>> {
        Ok(vec![
            "weapon".to_string(),
            "armor".to_string(),
            "potion".to_string(),
            "scroll".to_string(),
            "gem".to_string(),
            "food".to_string(),
            "tool".to_string(),
            "material".to_string(),
            "magic_item".to_string(),
            "artifact".to_string(),
            "combat".to_string(),
            "magic".to_string(),
            "movement".to_string(),
            "crafting".to_string(),
            "trading".to_string(),
            "quest".to_string(),
            "social".to_string(),
            "admin".to_string(),
            "pvp".to_string(),
            "raid".to_string(),
        ])
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Dependency Injection Example ===\n");

    // Create data provider registry
    let mut data_registry = DataProviderRegistry::new();

    // Element Core injects its data provider
    data_registry.register_element_provider(Box::new(MockElementDataProvider));
    println!("✅ Element Core registered element data provider");

    // Resource Core injects its data provider
    data_registry.register_resource_provider(Box::new(MockResourceDataProvider));
    println!("✅ Resource Core registered resource data provider");

    // Category Core injects its data provider
    data_registry.register_category_provider(Box::new(MockCategoryDataProvider));
    println!("✅ Category Core registered category data provider");

    // Create condition resolver with data providers
    let resolver = ConditionResolver::new(data_registry);

    // Create test context
    let _context = ConditionContext {
        target: ActorTarget {
            id: "player_1".to_string(),
        },
        world_id: "test_world".to_string(),
        current_time: SystemTime::now(),
        current_weather: WeatherType::Rain,
        world_state: WorldState {
            time_of_day: 14.0,
            season: "summer".to_string(),
            temperature: 25.0,
            humidity: 0.8,
        },
    };

    // Example 1: Element functions using Element Core data
    println!("\n1. Element Functions (using Element Core data):");
    
    if let Some(element_provider) = resolver.get_data_registry().get_element_provider() {
        let fire_mastery = element_provider.get_element_mastery("fire", "player_1").await?;
        println!("   Fire mastery: {}", fire_mastery);

        let shadow_mastery = element_provider.get_element_mastery("shadow", "player_1").await?;
        println!("   Shadow mastery (new element): {}", shadow_mastery);

        let fire_water_interaction = element_provider.get_element_interaction("fire", "water").await?;
        println!("   Fire vs Water: {}", fire_water_interaction);

        let shadow_light_interaction = element_provider.get_element_interaction("shadow", "light").await?;
        println!("   Shadow vs Light (new interaction): {}", shadow_light_interaction);
    }

    // Example 2: Resource functions using Resource Core data
    println!("\n2. Resource Functions (using Resource Core data):");
    
    if let Some(resource_provider) = resolver.get_data_registry().get_resource_provider() {
        let mana_value = resource_provider.get_resource_value("mana", "player_1").await?;
        println!("   Mana value: {}", mana_value);

        let sanity_value = resource_provider.get_resource_value("sanity", "player_1").await?;
        println!("   Sanity value (new resource): {}", sanity_value);

        let is_mana_low = resource_provider.is_resource_below_percentage("mana", 30.0, "player_1").await?;
        println!("   Is mana low: {}", is_mana_low);

        let is_sanity_low = resource_provider.is_resource_below_percentage("sanity", 30.0, "player_1").await?;
        println!("   Is sanity low (new resource): {}", is_sanity_low);
    }

    // Example 3: Category functions using Category Core data
    println!("\n3. Category Functions (using Category Core data):");
    
    if let Some(category_provider) = resolver.get_data_registry().get_category_provider() {
        let has_weapon = category_provider.has_category_item("weapon", "player_1").await?;
        println!("   Has weapon: {}", has_weapon);

        let has_magic_item = category_provider.has_category_item("magic_item", "player_1").await?;
        println!("   Has magic item (new category): {}", has_magic_item);

        let is_pvp_available = category_provider.is_category_available("pvp", "player_1").await?;
        println!("   Is PvP available (new category): {}", is_pvp_available);

        let is_raid_blocked = category_provider.is_category_blocked("raid", "player_1").await?;
        println!("   Is raid blocked (new category): {}", is_raid_blocked);
    }

    // Example 4: List all available data
    println!("\n4. Available Data (dynamically loaded):");
    
    if let Some(element_provider) = resolver.get_data_registry().get_element_provider() {
        let elements = element_provider.list_elements().await?;
        println!("   Elements: {:?}", elements);
    }

    if let Some(resource_provider) = resolver.get_data_registry().get_resource_provider() {
        let resources = resource_provider.list_resources().await?;
        println!("   Resources: {:?}", resources);
    }

    if let Some(category_provider) = resolver.get_data_registry().get_category_provider() {
        let categories = category_provider.list_categories().await?;
        println!("   Categories: {:?}", categories);
    }

    println!("\n=== Key Benefits ===");
    println!("✅ Condition Core doesn't know about specific elements/resources/categories");
    println!("✅ Each system manages its own data");
    println!("✅ New elements/resources/categories can be added without changing Condition Core");
    println!("✅ Condition Core focuses only on condition resolution logic");
    println!("✅ Follows Single Responsibility Principle");
    println!("✅ Follows Dependency Inversion Principle");

    Ok(())
}

