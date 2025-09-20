//! Configuration system tests

use actor_core::config::*;
use actor_core::config::providers::*;
use actor_core::config::loaders::*;
use std::sync::Arc;
use std::path::PathBuf;

#[tokio::test]
async fn test_configuration_registry() -> Result<(), Box<dyn std::error::Error>> {
    let registry = Arc::new(ConfigurationRegistryImpl::new());
    
    // Test registering a provider
    let provider = Arc::new(ExampleConfigurationProvider::new());
    registry.register_provider(provider).await?;
    
    // Test getting provider
    let retrieved_provider = registry.get_provider("example_provider").await;
    assert!(retrieved_provider.is_some());
    
    // Test getting all providers
    let providers = registry.get_providers_by_priority().await;
    assert_eq!(providers.len(), 1);
    
    // Test validation
    registry.validate_all_providers().await?;
    
    Ok(())
}

#[tokio::test]
async fn test_configuration_combiner() -> Result<(), Box<dyn std::error::Error>> {
    let combiner = Arc::new(ConfigurationCombinerImpl::new());
    combiner.load_default_rules()?;
    
    // Test getting merge rule
    let rule = combiner.get_merge_rule("element_affinities", "fire_affinity").await;
    assert!(rule.is_some());
    
    // Test merging values
    let values = vec![
        ConfigurationValue::new(
            serde_json::Value::Number(0.5.into()),
            ConfigurationValueType::Float,
            "provider1".to_string(),
            100,
        ),
        ConfigurationValue::new(
            serde_json::Value::Number(0.3.into()),
            ConfigurationValueType::Float,
            "provider2".to_string(),
            200,
        ),
    ];
    
    let merge_rule = ConfigurationMergeRule {
        strategy: ConfigurationMergeStrategy::Sum,
        use_pipeline: true,
        default_value: Some(serde_json::Value::Number(0.0.into())),
        validation_rules: vec![],
    };
    
    let merged = combiner.merge_values(values, &merge_rule).await?;
    assert_eq!(merged.value.as_f64().unwrap(), 0.8);
    
    Ok(())
}

#[tokio::test]
async fn test_configuration_aggregator() -> Result<(), Box<dyn std::error::Error>> {
    let registry = Arc::new(ConfigurationRegistryImpl::new());
    let combiner = Arc::new(ConfigurationCombinerImpl::new());
    combiner.load_default_rules()?;
    let aggregator = Arc::new(ConfigurationAggregatorImpl::new(registry.clone(), combiner.clone()));
    
    // Register a provider
    let provider = Arc::new(ExampleConfigurationProvider::new());
    registry.register_provider(provider).await?;
    
    // Test getting configuration value
    let value = aggregator.get_config_value("element_affinities", "fire_affinity").await?;
    assert!(value.is_some());
    
    // Test getting category configuration
    let category_config = aggregator.get_category_config("element_affinities").await?;
    assert!(!category_config.is_empty());
    
    // Test getting all configuration
    let all_config = aggregator.get_all_config().await?;
    assert!(!all_config.is_empty());
    
    Ok(())
}

#[tokio::test]
async fn test_configuration_manager() -> Result<(), Box<dyn std::error::Error>> {
    let registry = Arc::new(ConfigurationRegistryImpl::new());
    let combiner = Arc::new(ConfigurationCombinerImpl::new());
    combiner.load_default_rules()?;
    let aggregator = Arc::new(ConfigurationAggregatorImpl::new(registry.clone(), combiner.clone()));
    
    let mut loader = ConfigurationLoader::new(registry.clone(), combiner.clone(), aggregator.clone());
    let provider = Arc::new(ExampleConfigurationProvider::new());
    loader.add_provider(provider);
    
    let manager = ConfigurationManager::new(registry, combiner, aggregator, Arc::new(loader));
    manager.initialize().await?;
    
    // Test getting configuration
    let value = manager.get_config("element_affinities", "fire_affinity").await?;
    assert!(value.is_some());
    
    // Test getting category configuration
    let category_config = manager.get_category_config("element_affinities").await?;
    assert!(!category_config.is_empty());
    
    // Test getting all configuration
    let all_config = manager.get_all_config().await?;
    assert!(!all_config.is_empty());
    
    // Test health status
    let health = manager.get_health_status().await;
    assert!(health.registry_health);
    assert!(health.combiner_health);
    assert!(health.aggregator_health);
    
    Ok(())
}

#[tokio::test]
async fn test_example_configuration_provider() -> Result<(), Box<dyn std::error::Error>> {
    let provider = ExampleConfigurationProvider::new();
    
    // Test provider metadata
    assert_eq!(provider.provider_id(), "example_provider");
    assert_eq!(provider.priority(), 100);
    assert_eq!(provider.get_supported_categories().len(), 2);
    
    // Test getting configuration value
    let value = provider.get_config_value("element_affinities", "fire_affinity").await?;
    assert!(value.is_some());
    assert_eq!(value.unwrap().value.as_f64().unwrap(), 0.0);
    
    // Test getting category configuration
    let category_config = provider.get_category_config("element_affinities").await?;
    assert!(!category_config.is_empty());
    
    // Test validation
    provider.validate_config().await?;
    
    Ok(())
}

#[tokio::test]
async fn test_environment_configuration_provider() -> Result<(), Box<dyn std::error::Error>> {
    let mut provider = EnvironmentConfigurationProvider::new(
        "env_provider".to_string(),
        200,
        "TEST_ACTOR_CORE".to_string(),
    );
    
    // Set some environment variables for testing
    std::env::set_var("TEST_ACTOR_CORE_ELEMENT_FIRE_AFFINITY", "0.8");
    std::env::set_var("TEST_ACTOR_CORE_STAT_STRENGTH", "15");
    std::env::set_var("TEST_ACTOR_CORE_FLAG_ENABLE_CACHING", "true");
    
    provider.load_from_environment()?;
    
    // Test provider metadata
    assert_eq!(provider.provider_id(), "env_provider");
    assert_eq!(provider.priority(), 200);
    
    // Test getting configuration value
    let value = provider.get_config_value("element", "fire_affinity").await?;
    assert!(value.is_some());
    assert_eq!(value.unwrap().value.as_f64().unwrap(), 0.8);
    
    // Test getting category configuration
    let category_config = provider.get_category_config("element").await?;
    assert!(!category_config.is_empty());
    
    // Test validation
    provider.validate_config().await?;
    
    // Clean up environment variables
    std::env::remove_var("TEST_ACTOR_CORE_ELEMENT_FIRE_AFFINITY");
    std::env::remove_var("TEST_ACTOR_CORE_STAT_STRENGTH");
    std::env::remove_var("TEST_ACTOR_CORE_FLAG_ENABLE_CACHING");
    
    Ok(())
}

#[tokio::test]
async fn test_database_configuration_provider() -> Result<(), Box<dyn std::error::Error>> {
    let mut provider = DatabaseConfigurationProvider::new(
        "db_provider".to_string(),
        300,
    );
    
    provider.load_from_database().await?;
    
    // Test provider metadata
    assert_eq!(provider.provider_id(), "db_provider");
    assert_eq!(provider.priority(), 300);
    
    // Test getting configuration value
    let value = provider.get_config_value("database", "connection_pool_size").await?;
    assert!(value.is_some());
    assert_eq!(value.unwrap().value.as_i64().unwrap(), 10);
    
    // Test getting category configuration
    let category_config = provider.get_category_config("database").await?;
    assert!(!category_config.is_empty());
    
    // Test validation
    provider.validate_config().await?;
    
    Ok(())
}

#[tokio::test]
async fn test_file_configuration_provider() -> Result<(), Box<dyn std::error::Error>> {
    // Create a temporary configuration file
    let config_content = r#"
categories:
  test:
    value1: 42
    value2: "hello"
    value3: true
"#;
    
    let temp_dir = std::env::temp_dir();
    let config_path = temp_dir.join("test_config.yaml");
    std::fs::write(&config_path, config_content)?;
    
    let provider = FileConfigurationProvider::new(
        "file_provider".to_string(),
        100,
        config_path.clone(),
    );
    
    provider.load_from_file().await?;
    
    // Test provider metadata
    assert_eq!(provider.provider_id(), "file_provider");
    assert_eq!(provider.priority(), 100);
    
    // Test getting configuration value
    let value = provider.get_config_value("test", "value1").await?;
    assert!(value.is_some());
    assert_eq!(value.unwrap().value.as_i64().unwrap(), 42);
    
    // Test getting category configuration
    let category_config = provider.get_category_config("test").await?;
    assert!(!category_config.is_empty());
    
    // Test validation
    provider.validate_config().await?;
    
    // Clean up
    std::fs::remove_file(&config_path)?;
    
    Ok(())
}

#[tokio::test]
async fn test_default_configuration_provider() -> Result<(), Box<dyn std::error::Error>> {
    let config_path = PathBuf::from("configs/actor_core_defaults.yaml");
    let provider = DefaultConfigProvider::new(config_path)?;
    
    // Test provider metadata
    assert_eq!(provider.provider_id(), "default_config_provider");
    assert_eq!(provider.priority(), 1000);
    
    // Test getting configuration value
    let value = provider.get_config_value("defaults", "resources").await?;
    assert!(value.is_some());
    
    // Test getting category configuration
    let category_config = provider.get_category_config("defaults").await?;
    assert!(!category_config.is_empty());
    
    // Test validation
    provider.validate_config().await?;
    
    Ok(())
}
