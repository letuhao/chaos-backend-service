//! Benchmark tests for Condition Core

use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};
use condition_core::*;
use std::time::SystemTime;

fn create_benchmark_context() -> ConditionContext {
    ConditionContext {
        target: ActorTarget {
            id: "benchmark_actor".to_string(),
        },
        world_id: "benchmark_world".to_string(),
        current_time: SystemTime::now(),
        current_weather: WeatherType::Clear,
        world_state: WorldState {
            time_of_day: 12.0,
            season: "spring".to_string(),
            temperature: 20.0,
            humidity: 0.5,
        },
    }
}

fn create_benchmark_conditions() -> Vec<ConditionConfig> {
    vec![
        ConditionConfig {
            condition_id: "health_check".to_string(),
            function_name: "get_actor_resource".to_string(),
            operator: ConditionOperator::GreaterThan,
            value: ConditionValue::Float(50.0),
            parameters: vec![ConditionParameter::String("health".to_string())],
        },
        ConditionConfig {
            condition_id: "mana_check".to_string(),
            function_name: "get_actor_resource".to_string(),
            operator: ConditionOperator::GreaterThan,
            value: ConditionValue::Float(25.0),
            parameters: vec![ConditionParameter::String("mana".to_string())],
        },
        ConditionConfig {
            condition_id: "stamina_check".to_string(),
            function_name: "get_actor_resource".to_string(),
            operator: ConditionOperator::GreaterThan,
            value: ConditionValue::Float(30.0),
            parameters: vec![ConditionParameter::String("stamina".to_string())],
        },
        ConditionConfig {
            condition_id: "fire_mastery_check".to_string(),
            function_name: "get_element_mastery".to_string(),
            operator: ConditionOperator::GreaterThanOrEqual,
            value: ConditionValue::Float(100.0),
            parameters: vec![ConditionParameter::String("fire".to_string())],
        },
        ConditionConfig {
            condition_id: "weapon_check".to_string(),
            function_name: "has_category_item".to_string(),
            operator: ConditionOperator::Equal,
            value: ConditionValue::Boolean(true),
            parameters: vec![ConditionParameter::String("weapon".to_string())],
        },
    ]
}

// Mock data providers for benchmarking
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
            _ => 0.0,
        };
        Ok(mastery)
    }

    async fn get_element_resistance(&self, element_id: &str, _actor_id: &str) -> ConditionResult<f64> {
        let resistance = match element_id {
            "fire" => 0.2,
            "water" => 0.5,
            "earth" => 0.3,
            _ => 0.0,
        };
        Ok(resistance)
    }

    async fn has_element_affinity(&self, element_id: &str, _actor_id: &str) -> ConditionResult<bool> {
        let has_affinity = match element_id {
            "fire" => true,
            "water" => true,
            "earth" => false,
            _ => false,
        };
        Ok(has_affinity)
    }

    async fn is_element_weakness(&self, _element_id: &str, _actor_id: &str) -> ConditionResult<bool> {
        Ok(false)
    }

    async fn get_element_interaction(&self, source_element: &str, target_element: &str) -> ConditionResult<String> {
        let interaction = match (source_element, target_element) {
            ("fire", "water") => "suppress",
            ("water", "fire") => "extinguish",
            _ => "neutral",
        };
        Ok(interaction.to_string())
    }

    async fn list_elements(&self) -> ConditionResult<Vec<String>> {
        Ok(vec!["fire".to_string(), "water".to_string(), "earth".to_string()])
    }
    
    // Element interaction functions
    async fn is_element_same_category(&self, element1: &str, element2: &str) -> ConditionResult<bool> {
        let fire_elements = vec!["fire", "lava", "magma"];
        let water_elements = vec!["water", "ice", "snow"];
        let earth_elements = vec!["earth", "stone", "metal"];
        
        let category1 = if fire_elements.contains(&element1) { "fire" }
            else if water_elements.contains(&element1) { "water" }
            else if earth_elements.contains(&element1) { "earth" }
            else { "unknown" };
            
        let category2 = if fire_elements.contains(&element2) { "fire" }
            else if water_elements.contains(&element2) { "water" }
            else if earth_elements.contains(&element2) { "earth" }
            else { "unknown" };
            
        Ok(category1 == category2)
    }
    
    async fn is_element_generating(&self, source_element: &str, target_element: &str) -> ConditionResult<bool> {
        Ok(matches!((source_element, target_element), 
            ("water", "fire") | ("fire", "earth") | ("earth", "metal") | ("metal", "water")))
    }
    
    async fn is_element_overcoming(&self, source_element: &str, target_element: &str) -> ConditionResult<bool> {
        Ok(matches!((source_element, target_element), 
            ("fire", "water") | ("water", "earth") | ("earth", "metal") | ("metal", "fire")))
    }
    
    async fn is_element_neutral(&self, source_element: &str, target_element: &str) -> ConditionResult<bool> {
        Ok(!self.is_element_generating(source_element, target_element).await? && 
           !self.is_element_overcoming(source_element, target_element).await?)
    }
    
    // Element status functions
    async fn has_element_status_effect(&self, element_id: &str, status_id: &str, _actor_id: &str) -> ConditionResult<bool> {
        match (element_id, status_id) {
            ("fire", "burning") => Ok(true),
            ("water", "wet") => Ok(false),
            ("earth", "rooted") => Ok(true),
            _ => Ok(false),
        }
    }
    
    async fn get_element_status_effect_count(&self, element_id: &str, status_id: &str, _actor_id: &str) -> ConditionResult<i64> {
        match (element_id, status_id) {
            ("fire", "burning") => Ok(2),
            ("water", "wet") => Ok(0),
            ("earth", "rooted") => Ok(1),
            _ => Ok(0),
        }
    }
    
    async fn is_element_status_effect_active(&self, element_id: &str, status_id: &str, actor_id: &str) -> ConditionResult<bool> {
        self.has_element_status_effect(element_id, status_id, actor_id).await
    }
    
    // Element resource functions
    async fn has_element_resource(&self, element_id: &str, resource_type: &str, _actor_id: &str) -> ConditionResult<bool> {
        match (element_id, resource_type) {
            ("fire", "mana") => Ok(true),
            ("water", "mana") => Ok(true),
            ("earth", "mana") => Ok(false),
            _ => Ok(false),
        }
    }
    
    async fn get_element_resource_value(&self, element_id: &str, resource_type: &str, _actor_id: &str) -> ConditionResult<f64> {
        match (element_id, resource_type) {
            ("fire", "mana") => Ok(100.0),
            ("water", "mana") => Ok(80.0),
            ("earth", "mana") => Ok(0.0),
            _ => Ok(0.0),
        }
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
        match hybrid_id {
            "steam" => Ok(true),
            "mud" => Ok(false),
            "lava" => Ok(true),
            _ => Ok(false),
        }
    }
    
    async fn is_hybrid_element_activated(&self, hybrid_id: &str, _actor_id: &str) -> ConditionResult<bool> {
        match hybrid_id {
            "steam" => Ok(true),
            "mud" => Ok(false),
            "lava" => Ok(true),
            _ => Ok(false),
        }
    }
    
    async fn get_hybrid_element_parents(&self, hybrid_id: &str) -> ConditionResult<Vec<String>> {
        match hybrid_id {
            "steam" => Ok(vec!["fire".to_string(), "water".to_string()]),
            "mud" => Ok(vec!["earth".to_string(), "water".to_string()]),
            "lava" => Ok(vec!["fire".to_string(), "earth".to_string()]),
            _ => Ok(vec![]),
        }
    }
    
    async fn list_hybrid_elements(&self) -> ConditionResult<Vec<String>> {
        Ok(vec!["steam".to_string(), "mud".to_string(), "lava".to_string()])
    }
    
    // Element derived stats functions
    async fn get_element_derived_stat(&self, element_id: &str, stat_name: &str, _actor_id: &str) -> ConditionResult<f64> {
        match (element_id, stat_name) {
            ("fire", "power") => Ok(200.0),
            ("fire", "defense") => Ok(50.0),
            ("water", "power") => Ok(120.0),
            ("water", "defense") => Ok(180.0),
            ("earth", "power") => Ok(150.0),
            ("earth", "defense") => Ok(250.0),
            _ => Ok(0.0),
        }
    }
    
    async fn has_element_derived_stat(&self, element_id: &str, stat_name: &str, actor_id: &str) -> ConditionResult<bool> {
        let value = self.get_element_derived_stat(element_id, stat_name, actor_id).await?;
        Ok(value > 0.0)
    }
    
    async fn list_element_derived_stats(&self, element_id: &str) -> ConditionResult<Vec<String>> {
        match element_id {
            "fire" => Ok(vec!["power".to_string(), "defense".to_string(), "crit_rate".to_string()]),
            "water" => Ok(vec!["power".to_string(), "defense".to_string(), "healing".to_string()]),
            "earth" => Ok(vec!["power".to_string(), "defense".to_string(), "stability".to_string()]),
            _ => Ok(vec![]),
        }
    }
}

#[async_trait::async_trait]
impl ResourceDataProvider for MockResourceDataProvider {
    async fn get_resource_value(&self, resource_id: &str, _actor_id: &str) -> ConditionResult<f64> {
        let value = match resource_id {
            "health" => 100.0,
            "mana" => 50.0,
            "stamina" => 75.0,
            _ => 0.0,
        };
        Ok(value)
    }

    async fn get_resource_max(&self, resource_id: &str, _actor_id: &str) -> ConditionResult<f64> {
        let max_value = match resource_id {
            "health" => 100.0,
            "mana" => 50.0,
            "stamina" => 100.0,
            _ => 100.0,
        };
        Ok(max_value)
    }

    async fn get_resource_percentage(&self, resource_id: &str, _actor_id: &str) -> ConditionResult<f64> {
        let (current, max) = match resource_id {
            "health" => (100.0, 100.0),
            "mana" => (15.0, 50.0),
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
            "mana" => (15.0, 50.0),
            "stamina" => (75.0, 100.0),
            _ => (0.0, 100.0),
        };
        let current_percentage = (current / max) * 100.0;
        Ok(current_percentage < percentage)
    }

    async fn is_resource_above_percentage(&self, resource_id: &str, percentage: f64, _actor_id: &str) -> ConditionResult<bool> {
        let (current, max) = match resource_id {
            "health" => (100.0, 100.0),
            "mana" => (15.0, 50.0),
            "stamina" => (75.0, 100.0),
            _ => (0.0, 100.0),
        };
        let current_percentage = (current / max) * 100.0;
        Ok(current_percentage > percentage)
    }

    async fn list_resources(&self) -> ConditionResult<Vec<String>> {
        Ok(vec!["health".to_string(), "mana".to_string(), "stamina".to_string()])
    }
}

#[async_trait::async_trait]
impl CategoryDataProvider for MockCategoryDataProvider {
    async fn has_category_item(&self, category_id: &str, _actor_id: &str) -> ConditionResult<bool> {
        let has_item = match category_id {
            "weapon" => true,
            "armor" => true,
            "potion" => true,
            _ => false,
        };
        Ok(has_item)
    }

    async fn get_category_item_count(&self, category_id: &str, _actor_id: &str) -> ConditionResult<i64> {
        let count = match category_id {
            "weapon" => 3,
            "armor" => 5,
            "potion" => 10,
            _ => 0,
        };
        Ok(count)
    }

    async fn is_category_available(&self, _category_id: &str, _actor_id: &str) -> ConditionResult<bool> {
        Ok(true)
    }

    async fn is_category_blocked(&self, _category_id: &str, _actor_id: &str) -> ConditionResult<bool> {
        Ok(false)
    }

    async fn list_categories(&self) -> ConditionResult<Vec<String>> {
        Ok(vec!["weapon".to_string(), "armor".to_string(), "potion".to_string()])
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

fn create_benchmark_resolver() -> ConditionResolver {
    let mut data_registry = DataProviderRegistry::new();
    data_registry.register_element_provider(Box::new(MockElementDataProvider));
    data_registry.register_resource_provider(Box::new(MockResourceDataProvider));
    data_registry.register_category_provider(Box::new(MockCategoryDataProvider));
    data_registry.register_actor_provider(Box::new(MockActorDataProvider));
    
    ConditionResolver::new(data_registry)
}

fn bench_single_condition_resolution(c: &mut Criterion) {
    let resolver = create_benchmark_resolver();
    let context = create_benchmark_context();
    let condition = create_benchmark_conditions()[0].clone();

    c.bench_function("single_condition_resolution", |b| {
        b.iter(|| {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(resolver.resolve_condition(&condition, &context))
        })
    });
}

fn bench_multiple_conditions_resolution(c: &mut Criterion) {
    let resolver = create_benchmark_resolver();
    let context = create_benchmark_context();
    let conditions = create_benchmark_conditions();

    c.bench_function("multiple_conditions_resolution", |b| {
        b.iter(|| {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(resolver.resolve_conditions(&conditions, &context))
        })
    });
}

fn bench_condition_chain_resolution(c: &mut Criterion) {
    let resolver = create_benchmark_resolver();
    let context = create_benchmark_context();
    let conditions = create_benchmark_conditions();

    let chain_config = ConditionChainConfig {
        chain_id: "benchmark_chain".to_string(),
        logic: ChainLogic::And,
        conditions: conditions.clone(),
    };

    c.bench_function("condition_chain_resolution", |b| {
        b.iter(|| {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(resolver.resolve_condition_chain(&chain_config, &context))
        })
    });
}

fn bench_different_chain_logics(c: &mut Criterion) {
    let resolver = create_benchmark_resolver();
    let context = create_benchmark_context();
    let conditions = create_benchmark_conditions();

    let mut group = c.benchmark_group("chain_logic_comparison");

    for logic in [ChainLogic::And, ChainLogic::Or, ChainLogic::Xor] {
        let chain_config = ConditionChainConfig {
            chain_id: format!("benchmark_chain_{:?}", logic),
            logic: logic.clone(),
            conditions: conditions.clone(),
        };

        group.bench_with_input(
            BenchmarkId::new("chain_resolution", format!("{:?}", logic)),
            &chain_config,
            |b, chain_config| {
                b.iter(|| {
                    let rt = tokio::runtime::Runtime::new().unwrap();
                    rt.block_on(resolver.resolve_condition_chain(chain_config, &context))
                })
            },
        );
    }

    group.finish();
}

fn bench_yaml_parsing(c: &mut Criterion) {
    let yaml_config = r#"
condition_id: "benchmark_condition"
function_name: "get_actor_resource"
operator: GreaterThan
value: !Float 50.0
parameters:
  - !String "health"
"#;

    c.bench_function("yaml_parsing", |b| {
        b.iter(|| parse_condition_config(yaml_config))
    });
}

fn bench_yaml_serialization(c: &mut Criterion) {
    let config = ConditionConfig {
        condition_id: "benchmark_condition".to_string(),
        function_name: "get_actor_resource".to_string(),
        operator: ConditionOperator::GreaterThan,
        value: ConditionValue::Float(50.0),
        parameters: vec![
            ConditionParameter::String("health".to_string()),
            ConditionParameter::Integer(42),
        ],
    };

    c.bench_function("yaml_serialization", |b| {
        b.iter(|| serde_yaml::to_string(&config))
    });
}

criterion_group!(
    benches,
    bench_single_condition_resolution,
    bench_multiple_conditions_resolution,
    bench_condition_chain_resolution,
    bench_different_chain_logics,
    bench_yaml_parsing,
    bench_yaml_serialization
);

criterion_main!(benches);
