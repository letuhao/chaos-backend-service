//! Condition Builder Example - Demonstrating programmatic condition creation

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
            "shadow" => 95.0,
            "metal" => 105.0,
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
            "sanity" => 100.0,
            "karma" => 0.0,
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
            "magic_item" => true,
            "artifact" => false,
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
            "pvp" => true,
            "raid" => false,
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
            "sanity" => 100.0,
            "karma" => 0.0,
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

fn create_test_resolver() -> ConditionResolver {
    let mut data_registry = DataProviderRegistry::new();
    data_registry.register_element_provider(Box::new(MockElementDataProvider));
    data_registry.register_resource_provider(Box::new(MockResourceDataProvider));
    data_registry.register_category_provider(Box::new(MockCategoryDataProvider));
    data_registry.register_actor_provider(Box::new(MockActorDataProvider));
    
    ConditionResolver::new(data_registry)
}

fn create_test_context() -> ConditionContext {
    ConditionContext {
        target: ActorTarget { id: "player_1".to_string() },
        world_id: "test_world".to_string(),
        current_time: SystemTime::now(),
        current_weather: WeatherType::Clear,
        world_state: WorldState {
            time_of_day: 12.0,
            season: "summer".to_string(),
            temperature: 25.0,
            humidity: 0.5,
        },
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Condition Builder Example ===\n");

    let resolver = create_test_resolver();
    let context = create_test_context();

    // 1. Basic Condition Building
    println!("1. Basic Condition Building:");
    
    let health_condition = ConditionBuilder::new()
        .id("check_health")
        .function("get_actor_resource")
        .parameter("health")
        .operator(ConditionOperator::GreaterThan)
        .value(ConditionValue::Float(75.0))
        .build()?;
    
    let result = resolver.resolve_condition(&health_condition, &context).await?;
    println!("   Health > 75: {}", result);

    // 2. Factory Methods
    println!("\n2. Factory Methods:");
    
    let health_check = ConditionBuilderFactory::health_check(50.0).build()?;
    let mana_check = ConditionBuilderFactory::mana_check(25.0).build()?;
    let fire_mastery_check = ConditionBuilderFactory::element_mastery_check("fire", 100.0).build()?;
    let weapon_check = ConditionBuilderFactory::has_category_item("weapon").build()?;
    
    let health_result = resolver.resolve_condition(&health_check, &context).await?;
    let mana_result = resolver.resolve_condition(&mana_check, &context).await?;
    let fire_result = resolver.resolve_condition(&fire_mastery_check, &context).await?;
    let weapon_result = resolver.resolve_condition(&weapon_check, &context).await?;
    
    println!("   Health > 50: {}", health_result);
    println!("   Mana > 25: {}", mana_result);
    println!("   Fire mastery > 100: {}", fire_result);
    println!("   Has weapon: {}", weapon_result);

    // 3. Condition Chain Building
    println!("\n3. Condition Chain Building:");
    
    let chain = ConditionChainBuilder::new()
        .id("complex_condition")
        .logic(ChainLogic::And)
        .condition(health_check)
        .condition(mana_check)
        .condition(fire_mastery_check)
        .condition(weapon_check)
        .build()?;
    
    let chain_result = resolver.resolve_condition_chain(&chain, &context).await?;
    println!("   Complex condition (AND): {}", chain_result);

    // 4. Factory Chain Methods
    println!("\n4. Factory Chain Methods:");
    
    let health_mana_chain = ConditionBuilderFactory::health_and_mana_check(50.0, 25.0)?;
    let resource_chain = ConditionBuilderFactory::resource_check_chain(50.0, 25.0, 50.0)?;
    let element_chain = ConditionBuilderFactory::element_mastery_chain(&[
        ("fire", 100.0),
        ("water", 80.0),
        ("earth", 60.0),
    ])?;
    
    let health_mana_result = resolver.resolve_condition_chain(&health_mana_chain, &context).await?;
    let resource_result = resolver.resolve_condition_chain(&resource_chain, &context).await?;
    let element_result = resolver.resolve_condition_chain(&element_chain, &context).await?;
    
    println!("   Health and Mana chain: {}", health_mana_result);
    println!("   Resource chain: {}", resource_result);
    println!("   Element mastery chain: {}", element_result);

    // 5. Helper Functions
    println!("\n5. Helper Functions:");
    
    let health_helper = helpers::resource_check("health", 75.0).build()?;
    let fire_helper = helpers::element_check("fire", 120.0).build()?;
    let weapon_helper = helpers::category_check("weapon").build()?;
    
    let health_helper_result = resolver.resolve_condition(&health_helper, &context).await?;
    let fire_helper_result = resolver.resolve_condition(&fire_helper, &context).await?;
    let weapon_helper_result = resolver.resolve_condition(&weapon_helper, &context).await?;
    
    println!("   Health helper (75): {}", health_helper_result);
    println!("   Fire helper (120): {}", fire_helper_result);
    println!("   Weapon helper: {}", weapon_helper_result);

    // 6. Dynamic Condition Building
    println!("\n6. Dynamic Condition Building:");
    
    let thresholds = vec![(50.0, 25.0), (75.0, 50.0), (100.0, 75.0)];
    
        for (i, (health_threshold, _mana_threshold)) in thresholds.iter().enumerate() {
        let dynamic_condition = ConditionBuilder::new()
            .id(format!("dynamic_condition_{}", i))
            .function("get_actor_resource")
            .parameter("health")
            .operator(ConditionOperator::GreaterThan)
            .value(ConditionValue::Float(*health_threshold))
            .build()?;
        
        let result = resolver.resolve_condition(&dynamic_condition, &context).await?;
        println!("   Dynamic condition {} (health > {}): {}", i, health_threshold, result);
    }

    // 7. Error Handling
    println!("\n7. Error Handling:");
    
    // Missing required fields
    let incomplete_condition = ConditionBuilder::new()
        .id("incomplete_condition")
        .function("get_actor_resource")
        // Missing operator and value
        .build();
    
    match incomplete_condition {
        Ok(_) => println!("   Unexpected: Incomplete condition built successfully"),
        Err(e) => println!("   Expected error: {}", e),
    }

    // 8. Performance Comparison
    println!("\n8. Performance Comparison:");
    
    // YAML parsing
    let yaml = r#"
    condition_id: "yaml_condition"
    function_name: "get_actor_resource"
    operator: GreaterThan
    value: !Float 75.0
    parameters:
      - !String "health"
    "#;
    
    let start = std::time::Instant::now();
    let _yaml_condition = parse_condition_config(yaml)?;
    let yaml_parse_time = start.elapsed();
    
    // Builder construction
    let start = std::time::Instant::now();
    let _builder_condition = ConditionBuilder::new()
        .id("builder_condition")
        .function("get_actor_resource")
        .parameter("health")
        .operator(ConditionOperator::GreaterThan)
        .value(ConditionValue::Float(75.0))
        .build()?;
    let builder_construct_time = start.elapsed();
    
    println!("   YAML parse time: {:?}", yaml_parse_time);
    println!("   Builder construct time: {:?}", builder_construct_time);
    println!("   Builder is {}x faster", yaml_parse_time.as_nanos() as f64 / builder_construct_time.as_nanos() as f64);

    // 9. Type Safety Demonstration
    println!("\n9. Type Safety Demonstration:");
    
    // This would not compile with wrong types
    let type_safe_condition = ConditionBuilder::new()
        .id("type_safe_condition")
        .function("get_actor_resource")
        .parameter("health") // String parameter
        .operator(ConditionOperator::GreaterThan) // Valid operator
        .value(ConditionValue::Float(75.0)) // Valid value type
        .build()?;
    
    let type_safe_result = resolver.resolve_condition(&type_safe_condition, &context).await?;
    println!("   Type safe condition result: {}", type_safe_result);

    println!("\n=== Condition Builder Example Complete ===");
    
    Ok(())
}

