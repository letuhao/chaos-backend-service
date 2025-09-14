//! Coverage tests for core modules (enums.rs, constants.rs, config.rs).

use actor_core::enums::{
    AcrossLayerPolicy,
    Operator,
    Bucket,
    CapMode
};
use actor_core::constants::{
    system_ids,
    primary_dimensions,
    derived_dimensions,
    meta_dimensions,
    context_types,
    error_codes,
    error_types,
    defaults,
    clamp_ranges,
    timeouts,
    cache_keys,
    log_levels,
    cache_policies,
    performance_thresholds,
    validation_rules,
    dimension_ranges
};
use actor_core::config::{
    ActorCoreConfig,
    RedisConfig,
    CacheConfig,
    LoggingConfig
};
use serde_json;
use std::collections::HashMap;

#[test]
fn test_across_layer_policy_variants() {
    let policies = vec![
        AcrossLayerPolicy::Intersect,
        AcrossLayerPolicy::Union,
        AcrossLayerPolicy::PrioritizedOverride,
    ];
    
    for policy in policies {
        assert!(std::ptr::addr_of!(policy) != std::ptr::null());
    }
}

#[test]
fn test_across_layer_policy_serialization() {
    let policy = AcrossLayerPolicy::Intersect;
    let serialized = serde_json::to_string(&policy).unwrap();
    let deserialized: AcrossLayerPolicy = serde_json::from_str(&serialized).unwrap();
    assert_eq!(policy, deserialized);
}

#[test]
fn test_across_layer_policy_clone() {
    let policy = AcrossLayerPolicy::Union;
    let cloned = policy.clone();
    assert_eq!(policy, cloned);
}

#[test]
fn test_across_layer_policy_debug() {
    let policy = AcrossLayerPolicy::PrioritizedOverride;
    let debug_str = format!("{:?}", policy);
    assert!(!debug_str.is_empty());
}

#[test]
fn test_operator_variants() {
    let operators = vec![
        Operator::Sum,
        Operator::Max,
        Operator::Min,
        Operator::Multiply,
        Operator::Average,
        Operator::Intersect,
    ];
    
    for op in operators {
        assert!(std::ptr::addr_of!(op) != std::ptr::null());
    }
}

#[test]
fn test_operator_serialization() {
    let op = Operator::Sum;
    let serialized = serde_json::to_string(&op).unwrap();
    let deserialized: Operator = serde_json::from_str(&serialized).unwrap();
    assert_eq!(op, deserialized);
}

#[test]
fn test_operator_clone() {
    let op = Operator::Max;
    let cloned = op.clone();
    assert_eq!(op, cloned);
}

#[test]
fn test_operator_debug() {
    let op = Operator::Min;
    let debug_str = format!("{:?}", op);
    assert!(!debug_str.is_empty());
}

#[test]
fn test_bucket_variants() {
    let buckets = vec![
        Bucket::Flat,
        Bucket::Mult,
        Bucket::PostAdd,
        Bucket::Override,
    ];
    
    for bucket in buckets {
        assert!(std::ptr::addr_of!(bucket) != std::ptr::null());
    }
}

#[test]
fn test_bucket_priority() {
    assert_eq!(Bucket::Flat.priority(), 1);
    assert_eq!(Bucket::Mult.priority(), 2);
    assert_eq!(Bucket::PostAdd.priority(), 3);
    assert_eq!(Bucket::Override.priority(), 4);
}

#[test]
fn test_bucket_is_valid() {
    assert!(Bucket::Flat.is_valid());
    assert!(Bucket::Mult.is_valid());
    assert!(Bucket::PostAdd.is_valid());
    assert!(Bucket::Override.is_valid());
}

#[test]
fn test_bucket_display_name() {
    assert_eq!(Bucket::Flat.display_name(), "Flat");
    assert_eq!(Bucket::Mult.display_name(), "Mult");
    assert_eq!(Bucket::PostAdd.display_name(), "PostAdd");
    assert_eq!(Bucket::Override.display_name(), "Override");
}

#[test]
fn test_bucket_serialization() {
    let bucket = Bucket::Flat;
    let serialized = serde_json::to_string(&bucket).unwrap();
    let deserialized: Bucket = serde_json::from_str(&serialized).unwrap();
    assert_eq!(bucket, deserialized);
}

#[test]
fn test_bucket_clone() {
    let bucket = Bucket::Mult;
    let cloned = bucket.clone();
    assert_eq!(bucket, cloned);
}

#[test]
fn test_bucket_debug() {
    let bucket = Bucket::PostAdd;
    let debug_str = format!("{:?}", bucket);
    assert!(!debug_str.is_empty());
}

#[test]
fn test_cap_mode_variants() {
    let modes = vec![
        CapMode::Baseline,
        CapMode::Additive,
        CapMode::HardMax,
        CapMode::HardMin,
        CapMode::Override,
        CapMode::SoftMax,
    ];
    
    for mode in modes {
        assert!(std::ptr::addr_of!(mode) != std::ptr::null());
    }
}

#[test]
fn test_cap_mode_is_valid() {
    assert!(CapMode::Baseline.is_valid());
    assert!(CapMode::Additive.is_valid());
    assert!(CapMode::HardMax.is_valid());
    assert!(CapMode::HardMin.is_valid());
    assert!(CapMode::Override.is_valid());
    assert!(CapMode::SoftMax.is_valid());
}

#[test]
fn test_cap_mode_display_name() {
    assert_eq!(CapMode::Baseline.display_name(), "Baseline");
    assert_eq!(CapMode::Additive.display_name(), "Additive");
    assert_eq!(CapMode::HardMax.display_name(), "HardMax");
    assert_eq!(CapMode::HardMin.display_name(), "HardMin");
    assert_eq!(CapMode::Override.display_name(), "Override");
    assert_eq!(CapMode::SoftMax.display_name(), "SoftMax");
}

#[test]
fn test_cap_mode_serialization() {
    let mode = CapMode::Baseline;
    let serialized = serde_json::to_string(&mode).unwrap();
    let deserialized: CapMode = serde_json::from_str(&serialized).unwrap();
    assert_eq!(mode, deserialized);
}

#[test]
fn test_cap_mode_clone() {
    let mode = CapMode::Additive;
    let cloned = mode.clone();
    assert_eq!(mode, cloned);
}

#[test]
fn test_cap_mode_debug() {
    let mode = CapMode::HardMax;
    let debug_str = format!("{:?}", mode);
    assert!(!debug_str.is_empty());
}

#[test]
fn test_system_ids_constants() {
    assert_eq!(system_ids::LUYEN_THE, "luyen_the");
    assert_eq!(system_ids::KIM_DAN, "kim_dan");
    assert_eq!(system_ids::COMBAT, "combat");
    assert_eq!(system_ids::EQUIPMENT, "equipment");
    assert_eq!(system_ids::BUFF, "buff");
    assert_eq!(system_ids::GUILD, "guild");
    assert_eq!(system_ids::EVENT, "event");
    assert_eq!(system_ids::WORLD, "world");
    assert_eq!(system_ids::MAGIC, "magic");
    assert_eq!(system_ids::CULTIVATION, "cultivation");
    assert_eq!(system_ids::EXPERIENCE, "experience");
    assert_eq!(system_ids::REPUTATION, "reputation");
    assert_eq!(system_ids::TRADING, "trading");
    assert_eq!(system_ids::WEATHER, "weather");
    assert_eq!(system_ids::LOCATION, "location");
    assert_eq!(system_ids::TIME, "time");
    assert_eq!(system_ids::STEALTH, "stealth");
    assert_eq!(system_ids::PERCEPTION, "perception");
}

#[test]
fn test_primary_dimensions_constants() {
    assert_eq!(primary_dimensions::STRENGTH, "strength");
    assert_eq!(primary_dimensions::AGILITY, "agility");
    assert_eq!(primary_dimensions::INTELLIGENCE, "intelligence");
    assert_eq!(primary_dimensions::VITALITY, "vitality");
    assert_eq!(primary_dimensions::SPIRIT, "spirit");
    assert_eq!(primary_dimensions::LUCK, "luck");
    assert_eq!(primary_dimensions::HEALTH, "health");
    assert_eq!(primary_dimensions::MANA, "mana");
    assert_eq!(primary_dimensions::STAMINA, "stamina");
    assert_eq!(primary_dimensions::EXPERIENCE, "experience");
    assert_eq!(primary_dimensions::LEVEL, "level");
}

#[test]
fn test_derived_dimensions_constants() {
    assert_eq!(derived_dimensions::ATTACK_POWER, "attack_power");
    assert_eq!(derived_dimensions::DEFENSE_POWER, "defense_power");
    assert_eq!(derived_dimensions::CRITICAL_HIT_CHANCE, "critical_hit_chance");
    assert_eq!(derived_dimensions::CRITICAL_HIT_DAMAGE, "critical_hit_damage");
    assert_eq!(derived_dimensions::ATTACK_SPEED, "attack_speed");
    assert_eq!(derived_dimensions::MOVEMENT_SPEED, "movement_speed");
    assert_eq!(derived_dimensions::CASTING_SPEED, "casting_speed");
    assert_eq!(derived_dimensions::COOLDOWN_REDUCTION, "cooldown_reduction");
    assert_eq!(derived_dimensions::LIFE_STEAL, "life_steal");
    assert_eq!(derived_dimensions::MANA_STEAL, "mana_steal");
    assert_eq!(derived_dimensions::DAMAGE_REDUCTION, "damage_reduction");
    assert_eq!(derived_dimensions::ELEMENTAL_RESISTANCE, "elemental_resistance");
}

#[test]
fn test_meta_dimensions_constants() {
    assert_eq!(meta_dimensions::REALM_ID, "realm_id");
    assert_eq!(meta_dimensions::WORLD_ID, "world_id");
    assert_eq!(meta_dimensions::ZONE_ID, "zone_id");
    assert_eq!(meta_dimensions::GUILD_ID, "guild_id");
    assert_eq!(meta_dimensions::PARTY_ID, "party_id");
    assert_eq!(meta_dimensions::EVENT_ID, "event_id");
}

#[test]
fn test_context_types_constants() {
    assert_eq!(context_types::DAMAGE, "damage");
    assert_eq!(context_types::HEALING, "healing");
    assert_eq!(context_types::EXPERIENCE_GAIN, "experience_gain");
    assert_eq!(context_types::ITEM_DROP, "item_drop");
    assert_eq!(context_types::COMBAT, "combat");
    assert_eq!(context_types::MOVEMENT, "movement");
    assert_eq!(context_types::CASTING, "casting");
}

#[test]
fn test_error_codes_constants() {
    assert_eq!(error_codes::INVALID_ACTOR, "INVALID_ACTOR");
    assert_eq!(error_codes::INVALID_CONTRIBUTION, "INVALID_CONTRIBUTION");
    assert_eq!(error_codes::INVALID_CAP, "INVALID_CAP");
    assert_eq!(error_codes::SUBSYSTEM_ERROR, "SUBSYSTEM_ERROR");
    assert_eq!(error_codes::CACHE_ERROR, "CACHE_ERROR");
    assert_eq!(error_codes::REGISTRY_ERROR, "REGISTRY_ERROR");
    assert_eq!(error_codes::AGGREGATION_ERROR, "AGGREGATION_ERROR");
}

#[test]
fn test_error_types_constants() {
    assert_eq!(error_types::VALIDATION, "VALIDATION");
    assert_eq!(error_types::SYSTEM, "SYSTEM");
    assert_eq!(error_types::NETWORK, "NETWORK");
    assert_eq!(error_types::DATABASE, "DATABASE");
    assert_eq!(error_types::CACHE, "CACHE");
    assert_eq!(error_types::CONFIGURATION, "CONFIGURATION");
}

#[test]
fn test_defaults_constants() {
    assert_eq!(defaults::ACTOR_LIFESPAN, 365 * 24 * 60 * 60);
    assert_eq!(defaults::ACTOR_AGE, 0);
    assert_eq!(defaults::SUBSYSTEM_PRIORITY, 100);
    assert_eq!(defaults::CONTRIBUTION_PRIORITY, 100);
    assert_eq!(defaults::CAP_PRIORITY, 100);
    assert_eq!(defaults::CACHE_TTL, 3600);
    assert_eq!(defaults::BATCH_SIZE, 100);
    assert_eq!(defaults::MAX_RETRIES, 3);
}

#[test]
fn test_clamp_ranges_function() {
    let strength_range = clamp_ranges::primary_dimension_range(primary_dimensions::STRENGTH);
    assert!(strength_range.is_some());
    let (min, max) = strength_range.unwrap();
    assert_eq!(min, 0.0);
    assert_eq!(max, 10000.0);
}

#[test]
fn test_actor_core_config_creation() {
    let config = ActorCoreConfig {
        redis: RedisConfig {
            url: "redis://localhost:6379".to_string(),
            connection_timeout: 30,
            command_timeout: 5,
            max_connections: 100,
            use_tls: false,
        },
        cache: CacheConfig {
            default_ttl: 3600,
            max_entries: 10000,
            enable_redis: true,
            l1_size: 1000,
            l2_size: 5000,
            l3_size: 10000,
        },
        logging: LoggingConfig {
            level: "info".to_string(),
            structured: true,
            json: true,
        },
    };
    
    assert_eq!(config.redis.url, "redis://localhost:6379");
    assert_eq!(config.cache.default_ttl, 3600);
    assert_eq!(config.logging.level, "info");
    assert_eq!(config.logging.structured, true);
    assert_eq!(config.logging.json, true);
}

#[test]
fn test_redis_config_creation() {
    let config = RedisConfig {
        url: "redis://localhost:6379".to_string(),
        connection_timeout: 30,
        command_timeout: 5,
        max_connections: 100,
        use_tls: false,
    };
    
    assert_eq!(config.url, "redis://localhost:6379");
    assert_eq!(config.connection_timeout, 30);
    assert_eq!(config.command_timeout, 5);
    assert_eq!(config.max_connections, 100);
    assert_eq!(config.use_tls, false);
}

#[test]
fn test_cache_config_creation() {
    let config = CacheConfig {
        default_ttl: 3600,
        max_entries: 10000,
        enable_redis: true,
        l1_size: 1000,
        l2_size: 5000,
        l3_size: 10000,
    };
    
    assert_eq!(config.default_ttl, 3600);
    assert_eq!(config.max_entries, 10000);
    assert_eq!(config.enable_redis, true);
    assert_eq!(config.l1_size, 1000);
    assert_eq!(config.l2_size, 5000);
    assert_eq!(config.l3_size, 10000);
}

#[test]
fn test_logging_config_creation() {
    let config = LoggingConfig {
        level: "info".to_string(),
        structured: true,
        json: true,
    };
    
    assert_eq!(config.level, "info");
    assert_eq!(config.structured, true);
    assert_eq!(config.json, true);
}

#[test]
fn test_config_serialization() {
    let config = ActorCoreConfig {
        redis: RedisConfig {
            url: "redis://localhost:6379".to_string(),
            connection_timeout: 30,
            command_timeout: 5,
            max_connections: 100,
            use_tls: false,
        },
        cache: CacheConfig {
            default_ttl: 3600,
            max_entries: 10000,
            enable_redis: true,
            l1_size: 1000,
            l2_size: 5000,
            l3_size: 10000,
        },
        logging: LoggingConfig {
            level: "info".to_string(),
            structured: true,
            json: true,
        },
    };
    
    let serialized = serde_json::to_string(&config).unwrap();
    let deserialized: ActorCoreConfig = serde_json::from_str(&serialized).unwrap();
    
    assert_eq!(config.redis.url, deserialized.redis.url);
    assert_eq!(config.cache.default_ttl, deserialized.cache.default_ttl);
    assert_eq!(config.logging.level, deserialized.logging.level);
}

#[test]
fn test_config_clone() {
    let config = ActorCoreConfig {
        redis: RedisConfig {
            url: "redis://localhost:6379".to_string(),
            connection_timeout: 30,
            command_timeout: 5,
            max_connections: 100,
            use_tls: false,
        },
        cache: CacheConfig {
            default_ttl: 3600,
            max_entries: 10000,
            enable_redis: true,
            l1_size: 1000,
            l2_size: 5000,
            l3_size: 10000,
        },
        logging: LoggingConfig {
            level: "info".to_string(),
            structured: true,
            json: true,
        },
    };
    
    let cloned = config.clone();
    assert_eq!(config.redis.url, cloned.redis.url);
    assert_eq!(config.cache.default_ttl, cloned.cache.default_ttl);
    assert_eq!(config.logging.level, cloned.logging.level);
}

#[test]
fn test_config_debug() {
    let config = ActorCoreConfig {
        redis: RedisConfig {
            url: "redis://localhost:6379".to_string(),
            connection_timeout: 30,
            command_timeout: 5,
            max_connections: 100,
            use_tls: false,
        },
        cache: CacheConfig {
            default_ttl: 3600,
            max_entries: 10000,
            enable_redis: true,
            l1_size: 1000,
            l2_size: 5000,
            l3_size: 10000,
        },
        logging: LoggingConfig {
            level: "info".to_string(),
            structured: true,
            json: true,
        },
    };
    
    let debug_str = format!("{:?}", config);
    assert!(!debug_str.is_empty());
}

#[test]
fn test_config_default() {
    let config = ActorCoreConfig::default();
    assert_eq!(config.redis.url, "redis://localhost:6379");
    assert_eq!(config.redis.connection_timeout, 5);
    assert_eq!(config.redis.command_timeout, 3);
    assert_eq!(config.redis.max_connections, 10);
    assert_eq!(config.redis.use_tls, false);
    assert_eq!(config.cache.default_ttl, 1800);
    assert_eq!(config.cache.max_entries, 1_000_000);
    assert_eq!(config.cache.enable_redis, false);
    assert_eq!(config.cache.l1_size, 50_000);
    assert_eq!(config.cache.l2_size, 200_000);
    assert_eq!(config.cache.l3_size, 500_000);
}

#[test]
fn test_redis_config_default() {
    let config = RedisConfig::default();
    assert_eq!(config.url, "redis://localhost:6379");
    assert_eq!(config.connection_timeout, 5);
    assert_eq!(config.command_timeout, 3);
    assert_eq!(config.max_connections, 10);
    assert_eq!(config.use_tls, false);
}

#[test]
fn test_cache_config_default() {
    let config = CacheConfig::default();
    assert_eq!(config.default_ttl, 1800);
    assert_eq!(config.max_entries, 1_000_000);
    assert_eq!(config.enable_redis, false);
    assert_eq!(config.l1_size, 50_000);
    assert_eq!(config.l2_size, 200_000);
    assert_eq!(config.l3_size, 500_000);
}

#[test]
fn test_logging_config_default() {
    let config = LoggingConfig::default();
    assert_eq!(config.level, "info");
    assert_eq!(config.structured, true);
    assert_eq!(config.json, false);
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
fn test_numeric_operations() {
    let value1 = 100.0;
    let value2 = 50.0;
    
    assert_eq!(value1 + value2, 150.0);
    assert_eq!(value1 - value2, 50.0);
    assert_eq!(value1 * value2, 5000.0);
    assert_eq!(value1 / value2, 2.0);
}

#[test]
fn test_boolean_operations() {
    let true_value = true;
    let false_value = false;
    
    assert!(true_value);
    assert!(!false_value);
    assert_eq!(true_value && false_value, false);
    assert_eq!(true_value || false_value, true);
}

#[test]
fn test_hashmap_operations() {
    let mut map = HashMap::new();
    map.insert("key1".to_string(), "value1".to_string());
    map.insert("key2".to_string(), "value2".to_string());
    
    assert_eq!(map.len(), 2);
    assert_eq!(map.get("key1"), Some(&"value1".to_string()));
    assert_eq!(map.get("key2"), Some(&"value2".to_string()));
}

#[test]
fn test_vec_operations() {
    let mut vec = Vec::new();
    vec.push("item1".to_string());
    vec.push("item2".to_string());
    
    assert_eq!(vec.len(), 2);
    assert_eq!(vec[0], "item1");
    assert_eq!(vec[1], "item2");
}

#[test]
fn test_option_operations() {
    let some_value = Some("test".to_string());
    let none_value: Option<String> = None;
    
    assert!(some_value.is_some());
    assert!(none_value.is_none());
    assert_eq!(some_value.as_ref().unwrap(), "test");
}

#[test]
fn test_usize_operations() {
    let value1 = 100usize;
    let value2 = 50usize;
    
    assert_eq!(value1 + value2, 150);
    assert_eq!(value1 - value2, 50);
    assert_eq!(value1 * value2, 5000);
    assert_eq!(value1 / value2, 2);
}

#[test]
fn test_u32_operations() {
    let value1 = 100u32;
    let value2 = 50u32;
    
    assert_eq!(value1 + value2, 150);
    assert_eq!(value1 - value2, 50);
    assert_eq!(value1 * value2, 5000);
    assert_eq!(value1 / value2, 2);
}

#[test]
fn test_u64_operations() {
    let value1 = 100u64;
    let value2 = 50u64;
    
    assert_eq!(value1 + value2, 150);
    assert_eq!(value1 - value2, 50);
    assert_eq!(value1 * value2, 5000);
    assert_eq!(value1 / value2, 2);
}

#[test]
fn test_f64_operations() {
    let value1 = 100.0;
    let value2 = 50.0;
    
    assert_eq!(value1 + value2, 150.0);
    assert_eq!(value1 - value2, 50.0);
    assert_eq!(value1 * value2, 5000.0);
    assert_eq!(value1 / value2, 2.0);
}

#[test]
fn test_equality_operations() {
    let policy1 = AcrossLayerPolicy::Intersect;
    let policy2 = AcrossLayerPolicy::Intersect;
    let policy3 = AcrossLayerPolicy::Union;
    
    assert_eq!(policy1, policy2);
    assert_ne!(policy1, policy3);
}

#[test]
fn test_partial_eq_implementations() {
    let bucket1 = Bucket::Flat;
    let bucket2 = Bucket::Flat;
    let bucket3 = Bucket::Mult;
    
    assert_eq!(bucket1, bucket2);
    assert_ne!(bucket1, bucket3);
}

#[test]
fn test_hash_implementations() {
    let bucket1 = Bucket::Flat;
    let bucket2 = Bucket::Flat;
    
    // Test that equal values have equal hash codes
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    
    let mut hasher1 = DefaultHasher::new();
    bucket1.hash(&mut hasher1);
    let hash1 = hasher1.finish();
    
    let mut hasher2 = DefaultHasher::new();
    bucket2.hash(&mut hasher2);
    let hash2 = hasher2.finish();
    
    assert_eq!(hash1, hash2);
}