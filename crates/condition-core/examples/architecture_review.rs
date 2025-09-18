//! Architecture Review Example - Showing the evolution of Condition Core

use condition_core::*;
use std::time::SystemTime;

// Mock implementations for demonstration
struct MockElementDataProvider;
struct MockResourceDataProvider;
struct MockCategoryDataProvider;
struct MockActorDataProvider;

#[async_trait::async_trait]
impl ElementDataProvider for MockElementDataProvider {
    async fn get_element_mastery(&self, element_id: &str, _actor_id: &str) -> ConditionResult<f64> {
        let mastery = match element_id {
            "fire" => 150.0,
            "water" => 120.0,
            "earth" => 100.0,
            "shadow" => 95.0, // New element
            "metal" => 105.0, // New element
            _ => 0.0,
        };
        Ok(mastery)
    }

    async fn get_element_resistance(&self, element_id: &str, _actor_id: &str) -> ConditionResult<f64> {
        let resistance = match element_id {
            "fire" => 0.2,
            "water" => 0.5,
            "earth" => 0.3,
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
            "shadow".to_string(),
            "metal".to_string(),
        ])
    }
}

#[async_trait::async_trait]
impl ResourceDataProvider for MockResourceDataProvider {
    async fn get_resource_value(&self, resource_id: &str, _actor_id: &str) -> ConditionResult<f64> {
        let value = match resource_id {
            "health" => 100.0,
            "mana" => 50.0,
            "stamina" => 75.0,
            "sanity" => 100.0, // New resource
            "karma" => 0.0,    // New resource
            _ => 0.0,
        };
        Ok(value)
    }

    async fn get_resource_max(&self, resource_id: &str, _actor_id: &str) -> ConditionResult<f64> {
        let max_value = match resource_id {
            "health" => 100.0,
            "mana" => 50.0,
            "stamina" => 100.0,
            "sanity" => 100.0,
            "karma" => 100.0,
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
            "sanity".to_string(),
            "karma".to_string(),
        ])
    }
}

#[async_trait::async_trait]
impl CategoryDataProvider for MockCategoryDataProvider {
    async fn has_category_item(&self, category_id: &str, _actor_id: &str) -> ConditionResult<bool> {
        let has_item = match category_id {
            "weapon" => true,
            "armor" => true,
            "potion" => true,
            "magic_item" => true, // New category
            "artifact" => false,  // New category
            _ => false,
        };
        Ok(has_item)
    }

    async fn get_category_item_count(&self, category_id: &str, _actor_id: &str) -> ConditionResult<i64> {
        let count = match category_id {
            "weapon" => 3,
            "armor" => 5,
            "potion" => 10,
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
            "pvp" => true,      // New category
            "raid" => false,    // New category
            _ => true,
        };
        Ok(is_available)
    }

    async fn is_category_blocked(&self, category_id: &str, _actor_id: &str) -> ConditionResult<bool> {
        let is_blocked = match category_id {
            "combat" => false,
            "magic" => false,
            "movement" => false,
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
            "magic_item".to_string(),
            "artifact".to_string(),
            "combat".to_string(),
            "magic".to_string(),
            "movement".to_string(),
            "pvp".to_string(),
            "raid".to_string(),
        ])
    }
}

#[async_trait::async_trait]
impl ActorDataProvider for MockActorDataProvider {
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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Condition Core Architecture Review ===\n");

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

    // 1. OLD ARCHITECTURE: Hard-coded functions (removed)
    println!("1. OLD ARCHITECTURE (Removed):");
    println!("   ❌ Hard-coded functions have been removed");
    println!("   ❌ No fallback mockup implementations");
    println!("   ✅ Clean architecture with proper error handling\n");

    // 2. NEW ARCHITECTURE: Data provider injection
    println!("2. NEW ARCHITECTURE (Data Provider Injection):");
    
    // Create data provider registry
    let mut data_registry = DataProviderRegistry::new();
    data_registry.register_element_provider(Box::new(MockElementDataProvider));
    data_registry.register_resource_provider(Box::new(MockResourceDataProvider));
    data_registry.register_category_provider(Box::new(MockCategoryDataProvider));
    data_registry.register_actor_provider(Box::new(MockActorDataProvider));

    // Create condition resolver with data providers
    let resolver = ConditionResolver::new(data_registry);

    // Test with existing elements
    if let Some(element_provider) = resolver.get_data_registry().get_element_provider() {
        let fire_mastery = element_provider.get_element_mastery("fire", "player_1").await?;
        println!("   Fire mastery: {}", fire_mastery);

        // Test with new elements (no code changes needed!)
        let shadow_mastery = element_provider.get_element_mastery("shadow", "player_1").await?;
        println!("   Shadow mastery (new element): {}", shadow_mastery);

        let metal_mastery = element_provider.get_element_mastery("metal", "player_1").await?;
        println!("   Metal mastery (new element): {}", metal_mastery);

        // Test new interactions
        let shadow_light_interaction = element_provider.get_element_interaction("shadow", "light").await?;
        println!("   Shadow vs Light interaction: {}", shadow_light_interaction);
    }

    // Test with resources
    if let Some(resource_provider) = resolver.get_data_registry().get_resource_provider() {
        let mana_value = resource_provider.get_resource_value("mana", "player_1").await?;
        println!("   Mana value: {}", mana_value);

        // Test with new resources
        let sanity_value = resource_provider.get_resource_value("sanity", "player_1").await?;
        println!("   Sanity value (new resource): {}", sanity_value);

        let karma_value = resource_provider.get_resource_value("karma", "player_1").await?;
        println!("   Karma value (new resource): {}", karma_value);
    }

    // Test with categories
    if let Some(category_provider) = resolver.get_data_registry().get_category_provider() {
        let has_weapon = category_provider.has_category_item("weapon", "player_1").await?;
        println!("   Has weapon: {}", has_weapon);

        // Test with new categories
        let has_magic_item = category_provider.has_category_item("magic_item", "player_1").await?;
        println!("   Has magic item (new category): {}", has_magic_item);

        let is_pvp_available = category_provider.is_category_available("pvp", "player_1").await?;
        println!("   Is PvP available (new category): {}", is_pvp_available);
    }

    println!("\n=== Architecture Comparison ===");
    println!("OLD ARCHITECTURE:");
    println!("❌ Hard-coded data in functions");
    println!("❌ Must modify code to add new elements/resources/categories");
    println!("❌ Violates Single Responsibility Principle");
    println!("❌ Violates Dependency Inversion Principle");
    println!("❌ Not scalable for plugin system");

    println!("\nNEW ARCHITECTURE:");
    println!("✅ Data providers inject real data");
    println!("✅ Can add new elements/resources/categories without code changes");
    println!("✅ Follows Single Responsibility Principle");
    println!("✅ Follows Dependency Inversion Principle");
    println!("✅ Scalable for plugin system");
    println!("✅ Each system manages its own data");
    println!("✅ Condition Core only resolves conditions");

    println!("\n=== Key Benefits ===");
    println!("1. PLUGIN READY: New elements/resources/categories can be added via YAML configs");
    println!("2. SEPARATION OF CONCERNS: Each system manages its own data");
    println!("3. TESTABLE: Easy to mock data providers for testing");
    println!("4. EXTENSIBLE: Easy to add new data providers for new systems");
    println!("5. MAINTAINABLE: Changes to data don't affect condition logic");

    Ok(())
}

