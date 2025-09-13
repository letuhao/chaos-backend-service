//! Constants for the Actor Core system.
//!
//! This module contains all the constant values used throughout the system,
//! including dimension names, default values, and configuration constants.

/// System identifiers for various game systems.
pub mod system_ids {
    /// Luyen The (Cultivation) system
    pub const LUYEN_THE: &str = "luyen_the";
    /// Kim Dan (Golden Pill) system
    pub const KIM_DAN: &str = "kim_dan";
    /// Combat system
    pub const COMBAT: &str = "combat";
    /// Equipment system
    pub const EQUIPMENT: &str = "equipment";
    /// Buff system
    pub const BUFF: &str = "buff";
    /// Guild system
    pub const GUILD: &str = "guild";
    /// Event system
    pub const EVENT: &str = "event";
    /// World system
    pub const WORLD: &str = "world";
    /// Magic system
    pub const MAGIC: &str = "magic";
    /// Cultivation system
    pub const CULTIVATION: &str = "cultivation";
    /// Experience system
    pub const EXPERIENCE: &str = "experience";
    /// Reputation system
    pub const REPUTATION: &str = "reputation";
    /// Trading system
    pub const TRADING: &str = "trading";
    /// Weather system
    pub const WEATHER: &str = "weather";
    /// Location system
    pub const LOCATION: &str = "location";
    /// Time system
    pub const TIME: &str = "time";
    /// Stealth system
    pub const STEALTH: &str = "stealth";
    /// Perception system
    pub const PERCEPTION: &str = "perception";
}

/// Primary stat dimensions.
pub mod primary_dimensions {
    /// Strength stat
    pub const STRENGTH: &str = "strength";
    /// Agility stat
    pub const AGILITY: &str = "agility";
    /// Intelligence stat
    pub const INTELLIGENCE: &str = "intelligence";
    /// Vitality stat
    pub const VITALITY: &str = "vitality";
    /// Spirit stat
    pub const SPIRIT: &str = "spirit";
    /// Luck stat
    pub const LUCK: &str = "luck";
    /// Health stat
    pub const HEALTH: &str = "health";
    /// Mana stat
    pub const MANA: &str = "mana";
    /// Stamina stat
    pub const STAMINA: &str = "stamina";
    /// Experience stat
    pub const EXPERIENCE: &str = "experience";
    /// Level stat
    pub const LEVEL: &str = "level";
}

/// Derived stat dimensions.
pub mod derived_dimensions {
    /// Attack power
    pub const ATTACK_POWER: &str = "attack_power";
    /// Defense power
    pub const DEFENSE_POWER: &str = "defense_power";
    /// Critical hit chance
    pub const CRITICAL_HIT_CHANCE: &str = "critical_hit_chance";
    /// Critical hit damage
    pub const CRITICAL_HIT_DAMAGE: &str = "critical_hit_damage";
    /// Attack speed
    pub const ATTACK_SPEED: &str = "attack_speed";
    /// Movement speed
    pub const MOVEMENT_SPEED: &str = "movement_speed";
    /// Casting speed
    pub const CASTING_SPEED: &str = "casting_speed";
    /// Cooldown reduction
    pub const COOLDOWN_REDUCTION: &str = "cooldown_reduction";
    /// Life steal
    pub const LIFE_STEAL: &str = "life_steal";
    /// Mana steal
    pub const MANA_STEAL: &str = "mana_steal";
    /// Damage reduction
    pub const DAMAGE_REDUCTION: &str = "damage_reduction";
    /// Elemental resistance
    pub const ELEMENTAL_RESISTANCE: &str = "elemental_resistance";
}

/// Meta/World dimensions.
pub mod meta_dimensions {
    /// Realm ID
    pub const REALM_ID: &str = "realm_id";
    /// World ID
    pub const WORLD_ID: &str = "world_id";
    /// Zone ID
    pub const ZONE_ID: &str = "zone_id";
    /// Guild ID
    pub const GUILD_ID: &str = "guild_id";
    /// Party ID
    pub const PARTY_ID: &str = "party_id";
    /// Event ID
    pub const EVENT_ID: &str = "event_id";
}

/// Context types for temporary effects.
pub mod context_types {
    /// Damage context
    pub const DAMAGE: &str = "damage";
    /// Healing context
    pub const HEALING: &str = "healing";
    /// Experience gain context
    pub const EXPERIENCE_GAIN: &str = "experience_gain";
    /// Item drop context
    pub const ITEM_DROP: &str = "item_drop";
    /// Combat context
    pub const COMBAT: &str = "combat";
    /// Movement context
    pub const MOVEMENT: &str = "movement";
    /// Casting context
    pub const CASTING: &str = "casting";
}

/// Error codes for the system.
pub mod error_codes {
    /// Invalid actor
    pub const INVALID_ACTOR: &str = "INVALID_ACTOR";
    /// Invalid contribution
    pub const INVALID_CONTRIBUTION: &str = "INVALID_CONTRIBUTION";
    /// Invalid cap
    pub const INVALID_CAP: &str = "INVALID_CAP";
    /// Subsystem error
    pub const SUBSYSTEM_ERROR: &str = "SUBSYSTEM_ERROR";
    /// Cache error
    pub const CACHE_ERROR: &str = "CACHE_ERROR";
    /// Registry error
    pub const REGISTRY_ERROR: &str = "REGISTRY_ERROR";
    /// Aggregation error
    pub const AGGREGATION_ERROR: &str = "AGGREGATION_ERROR";
    /// Configuration error
    pub const CONFIGURATION_ERROR: &str = "CONFIGURATION_ERROR";
}

/// Error types for categorization.
pub mod error_types {
    /// Validation error
    pub const VALIDATION: &str = "VALIDATION";
    /// System error
    pub const SYSTEM: &str = "SYSTEM";
    /// Network error
    pub const NETWORK: &str = "NETWORK";
    /// Database error
    pub const DATABASE: &str = "DATABASE";
    /// Cache error
    pub const CACHE: &str = "CACHE";
    /// Configuration error
    pub const CONFIGURATION: &str = "CONFIGURATION";
}

/// Default values for various configurations.
pub mod defaults {
    /// Default actor lifespan (1 year in seconds)
    pub const ACTOR_LIFESPAN: i64 = 365 * 24 * 60 * 60;
    /// Default actor age (0 seconds)
    pub const ACTOR_AGE: i64 = 0;
    /// Default subsystem priority
    pub const SUBSYSTEM_PRIORITY: i64 = 100;
    /// Default contribution priority
    pub const CONTRIBUTION_PRIORITY: i64 = 100;
    /// Default cap priority
    pub const CAP_PRIORITY: i64 = 100;
    /// Default cache TTL (1 hour in seconds)
    pub const CACHE_TTL: u64 = 3600;
    /// Default batch size
    pub const BATCH_SIZE: usize = 100;
    /// Default max retries
    pub const MAX_RETRIES: u32 = 3;
}

/// Clamp ranges for different dimensions.
pub mod clamp_ranges {
    use super::*;

    /// Get the default clamp range for a primary dimension.
    pub fn primary_dimension_range(dimension: &str) -> Option<(f64, f64)> {
        match dimension {
            primary_dimensions::STRENGTH => Some((0.0, 10000.0)),
            primary_dimensions::AGILITY => Some((0.0, 10000.0)),
            primary_dimensions::INTELLIGENCE => Some((0.0, 10000.0)),
            primary_dimensions::VITALITY => Some((0.0, 10000.0)),
            primary_dimensions::SPIRIT => Some((0.0, 10000.0)),
            primary_dimensions::LUCK => Some((0.0, 10000.0)),
            primary_dimensions::HEALTH => Some((0.0, 1000000.0)),
            primary_dimensions::MANA => Some((0.0, 1000000.0)),
            primary_dimensions::STAMINA => Some((0.0, 1000000.0)),
            primary_dimensions::EXPERIENCE => Some((0.0, f64::MAX)),
            primary_dimensions::LEVEL => Some((1.0, 1000.0)),
            _ => None,
        }
    }

    /// Get the default clamp range for a derived dimension.
    pub fn derived_dimension_range(dimension: &str) -> Option<(f64, f64)> {
        match dimension {
            derived_dimensions::ATTACK_POWER => Some((0.0, 100000.0)),
            derived_dimensions::DEFENSE_POWER => Some((0.0, 100000.0)),
            derived_dimensions::CRITICAL_HIT_CHANCE => Some((0.0, 100.0)),
            derived_dimensions::CRITICAL_HIT_DAMAGE => Some((0.0, 1000.0)),
            derived_dimensions::ATTACK_SPEED => Some((0.1, 10.0)),
            derived_dimensions::MOVEMENT_SPEED => Some((0.1, 50.0)),
            derived_dimensions::CASTING_SPEED => Some((0.1, 10.0)),
            derived_dimensions::COOLDOWN_REDUCTION => Some((0.0, 90.0)),
            derived_dimensions::LIFE_STEAL => Some((0.0, 100.0)),
            derived_dimensions::MANA_STEAL => Some((0.0, 100.0)),
            derived_dimensions::DAMAGE_REDUCTION => Some((0.0, 95.0)),
            derived_dimensions::ELEMENTAL_RESISTANCE => Some((0.0, 100.0)),
            _ => None,
        }
    }

    /// Get the default clamp range for any dimension.
    pub fn get_range(dimension: &str) -> Option<(f64, f64)> {
        primary_dimension_range(dimension)
            .or_else(|| derived_dimension_range(dimension))
    }
}

/// Timeouts and intervals for various operations.
pub mod timeouts {
    /// Default aggregation timeout (5 seconds)
    pub const AGGREGATION_TIMEOUT: u64 = 5000;
    /// Default cache operation timeout (1 second)
    pub const CACHE_TIMEOUT: u64 = 1000;
    /// Default database operation timeout (10 seconds)
    pub const DATABASE_TIMEOUT: u64 = 10000;
    /// Default network operation timeout (30 seconds)
    pub const NETWORK_TIMEOUT: u64 = 30000;
    /// Default subsystem timeout (2 seconds)
    pub const SUBSYSTEM_TIMEOUT: u64 = 2000;
    /// Default batch processing interval (100 milliseconds)
    pub const BATCH_INTERVAL: u64 = 100;
    /// Default cache cleanup interval (1 hour)
    pub const CACHE_CLEANUP_INTERVAL: u64 = 3600;
}

/// Cache keys for different types of data.
pub mod cache_keys {
    /// Actor snapshot cache key prefix
    pub const ACTOR_SNAPSHOT_PREFIX: &str = "actor_snapshot:";
    /// Subsystem output cache key prefix
    pub const SUBSYSTEM_OUTPUT_PREFIX: &str = "subsystem_output:";
    /// Effective caps cache key prefix
    pub const EFFECTIVE_CAPS_PREFIX: &str = "effective_caps:";
    /// Registry cache key prefix
    pub const REGISTRY_PREFIX: &str = "registry:";
    /// Configuration cache key prefix
    pub const CONFIG_PREFIX: &str = "config:";
}

/// Log levels for different components.
pub mod log_levels {
    /// Trace level
    pub const TRACE: &str = "trace";
    /// Debug level
    pub const DEBUG: &str = "debug";
    /// Info level
    pub const INFO: &str = "info";
    /// Warn level
    pub const WARN: &str = "warn";
    /// Error level
    pub const ERROR: &str = "error";
}

/// Cache policies for different scenarios.
pub mod cache_policies {
    /// LRU (Least Recently Used) policy
    pub const LRU: &str = "lru";
    /// LFU (Least Frequently Used) policy
    pub const LFU: &str = "lfu";
    /// TTL (Time To Live) policy
    pub const TTL: &str = "ttl";
    /// FIFO (First In, First Out) policy
    pub const FIFO: &str = "fifo";
}

/// Performance thresholds for monitoring.
pub mod performance_thresholds {
    /// Maximum acceptable aggregation time (1 second)
    pub const MAX_AGGREGATION_TIME: u64 = 1_000_000; // microseconds
    /// Maximum acceptable cache operation time (1 millisecond)
    pub const MAX_CACHE_TIME: u64 = 1_000; // microseconds
    /// Maximum acceptable subsystem time (100 milliseconds)
    pub const MAX_SUBSYSTEM_TIME: u64 = 100_000; // microseconds
    /// Maximum memory usage per actor (1 MB)
    pub const MAX_MEMORY_PER_ACTOR: u64 = 1_048_576; // bytes
    /// Maximum cache size (100 MB)
    pub const MAX_CACHE_SIZE: u64 = 100_048_576; // bytes
}

/// Validation rules for different data types.
pub mod validation_rules {
    /// Minimum actor name length
    pub const MIN_ACTOR_NAME_LENGTH: usize = 3;
    /// Maximum actor name length
    pub const MAX_ACTOR_NAME_LENGTH: usize = 32;
    /// Minimum dimension name length
    pub const MIN_DIMENSION_NAME_LENGTH: usize = 1;
    /// Maximum dimension name length
    pub const MAX_DIMENSION_NAME_LENGTH: usize = 64;
    /// Minimum system ID length
    pub const MIN_SYSTEM_ID_LENGTH: usize = 1;
    /// Maximum system ID length
    pub const MAX_SYSTEM_ID_LENGTH: usize = 32;
    /// Maximum number of subsystems per actor
    pub const MAX_SUBSYSTEMS_PER_ACTOR: usize = 100;
    /// Maximum number of contributions per subsystem
    pub const MAX_CONTRIBUTIONS_PER_SUBSYSTEM: usize = 1000;
}

/// All supported dimensions in the system.
pub fn all_dimensions() -> Vec<&'static str> {
    vec![
        // Primary dimensions
        primary_dimensions::STRENGTH,
        primary_dimensions::AGILITY,
        primary_dimensions::INTELLIGENCE,
        primary_dimensions::VITALITY,
        primary_dimensions::SPIRIT,
        primary_dimensions::LUCK,
        primary_dimensions::HEALTH,
        primary_dimensions::MANA,
        primary_dimensions::STAMINA,
        primary_dimensions::EXPERIENCE,
        primary_dimensions::LEVEL,
        // Derived dimensions
        derived_dimensions::ATTACK_POWER,
        derived_dimensions::DEFENSE_POWER,
        derived_dimensions::CRITICAL_HIT_CHANCE,
        derived_dimensions::CRITICAL_HIT_DAMAGE,
        derived_dimensions::ATTACK_SPEED,
        derived_dimensions::MOVEMENT_SPEED,
        derived_dimensions::CASTING_SPEED,
        derived_dimensions::COOLDOWN_REDUCTION,
        derived_dimensions::LIFE_STEAL,
        derived_dimensions::MANA_STEAL,
        derived_dimensions::DAMAGE_REDUCTION,
        derived_dimensions::ELEMENTAL_RESISTANCE,
        // Meta dimensions
        meta_dimensions::REALM_ID,
        meta_dimensions::WORLD_ID,
        meta_dimensions::ZONE_ID,
        meta_dimensions::GUILD_ID,
        meta_dimensions::PARTY_ID,
        meta_dimensions::EVENT_ID,
    ]
}

/// All supported context types.
pub fn all_context_types() -> Vec<&'static str> {
    vec![
        context_types::DAMAGE,
        context_types::HEALING,
        context_types::EXPERIENCE_GAIN,
        context_types::ITEM_DROP,
        context_types::COMBAT,
        context_types::MOVEMENT,
        context_types::CASTING,
    ]
}

/// All supported system IDs.
pub fn all_system_ids() -> Vec<&'static str> {
    vec![
        system_ids::LUYEN_THE,
        system_ids::KIM_DAN,
        system_ids::COMBAT,
        system_ids::EQUIPMENT,
        system_ids::BUFF,
        system_ids::GUILD,
        system_ids::EVENT,
        system_ids::WORLD,
        system_ids::MAGIC,
        system_ids::CULTIVATION,
        system_ids::EXPERIENCE,
        system_ids::REPUTATION,
        system_ids::TRADING,
        system_ids::WEATHER,
        system_ids::LOCATION,
        system_ids::TIME,
        system_ids::STEALTH,
        system_ids::PERCEPTION,
    ]
}

/// Dimension range constants for validation
pub mod dimension_ranges {
    // Primary stats
    pub const MIN_STRENGTH: f64 = 1.0;
    pub const MAX_STRENGTH: f64 = 1000.0;
    pub const MIN_AGILITY: f64 = 1.0;
    pub const MAX_AGILITY: f64 = 1000.0;
    pub const MIN_INTELLIGENCE: f64 = 1.0;
    pub const MAX_INTELLIGENCE: f64 = 1000.0;
    pub const MIN_VITALITY: f64 = 1.0;
    pub const MAX_VITALITY: f64 = 1000.0;
    pub const MIN_SPIRIT: f64 = 1.0;
    pub const MAX_SPIRIT: f64 = 1000.0;
    pub const MIN_LUCK: f64 = 1.0;
    pub const MAX_LUCK: f64 = 1000.0;
    
    // Health/Mana/Stamina
    pub const MIN_HEALTH: f64 = 1.0;
    pub const MAX_HEALTH: f64 = 10000.0;
    pub const MIN_MANA: f64 = 0.0;
    pub const MAX_MANA: f64 = 10000.0;
    pub const MIN_STAMINA: f64 = 0.0;
    pub const MAX_STAMINA: f64 = 10000.0;
    
    // Derived stats
    pub const MIN_ATTACK_POWER: f64 = 0.0;
    pub const MAX_ATTACK_POWER: f64 = 5000.0;
    pub const MIN_DEFENSE_POWER: f64 = 0.0;
    pub const MAX_DEFENSE_POWER: f64 = 5000.0;
    pub const MIN_CRITICAL_HIT_CHANCE: f64 = 0.0;
    pub const MAX_CRITICAL_HIT_CHANCE: f64 = 1.0;
    pub const MIN_CRITICAL_HIT_DAMAGE: f64 = 1.0;
    pub const MAX_CRITICAL_HIT_DAMAGE: f64 = 5.0;
    pub const MIN_ATTACK_SPEED: f64 = 0.1;
    pub const MAX_ATTACK_SPEED: f64 = 10.0;
    pub const MIN_MOVEMENT_SPEED: f64 = 0.1;
    pub const MAX_MOVEMENT_SPEED: f64 = 20.0;
    pub const MIN_CASTING_SPEED: f64 = 0.1;
    pub const MAX_CASTING_SPEED: f64 = 10.0;
    pub const MIN_COOLDOWN_REDUCTION: f64 = 0.0;
    pub const MAX_COOLDOWN_REDUCTION: f64 = 0.8;
    
    // Experience and Level
    pub const MIN_EXPERIENCE: f64 = 0.0;
    pub const MAX_EXPERIENCE: f64 = 999999999.0;
    pub const MIN_LEVEL: f64 = 1.0;
    pub const MAX_LEVEL: f64 = 1000.0;
    
    // Resistance stats
    pub const MIN_FIRE_RESISTANCE: f64 = 0.0;
    pub const MAX_FIRE_RESISTANCE: f64 = 1.0;
    pub const MIN_WATER_RESISTANCE: f64 = 0.0;
    pub const MAX_WATER_RESISTANCE: f64 = 1.0;
    pub const MIN_EARTH_RESISTANCE: f64 = 0.0;
    pub const MAX_EARTH_RESISTANCE: f64 = 1.0;
    pub const MIN_AIR_RESISTANCE: f64 = 0.0;
    pub const MAX_AIR_RESISTANCE: f64 = 1.0;
    pub const MIN_LIGHT_RESISTANCE: f64 = 0.0;
    pub const MAX_LIGHT_RESISTANCE: f64 = 1.0;
    pub const MIN_DARK_RESISTANCE: f64 = 0.0;
    pub const MAX_DARK_RESISTANCE: f64 = 1.0;
    
    // Special stats
    pub const MIN_LUCK_FACTOR: f64 = 0.0;
    pub const MAX_LUCK_FACTOR: f64 = 2.0;
    pub const MIN_CRITICAL_RESISTANCE: f64 = 0.0;
    pub const MAX_CRITICAL_RESISTANCE: f64 = 1.0;
    pub const MIN_DODGE_CHANCE: f64 = 0.0;
    pub const MAX_DODGE_CHANCE: f64 = 0.95;
    pub const MIN_BLOCK_CHANCE: f64 = 0.0;
    pub const MAX_BLOCK_CHANCE: f64 = 0.95;
    pub const MIN_PARRY_CHANCE: f64 = 0.0;
    pub const MAX_PARRY_CHANCE: f64 = 0.95;
}

#[cfg(test)]
mod tests {
    use super::*;

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
        assert_eq!(error_codes::CONFIGURATION_ERROR, "CONFIGURATION_ERROR");
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
    fn test_clamp_ranges_primary_dimension_range() {
        assert_eq!(clamp_ranges::primary_dimension_range("strength"), Some((0.0, 10000.0)));
        assert_eq!(clamp_ranges::primary_dimension_range("agility"), Some((0.0, 10000.0)));
        assert_eq!(clamp_ranges::primary_dimension_range("intelligence"), Some((0.0, 10000.0)));
        assert_eq!(clamp_ranges::primary_dimension_range("vitality"), Some((0.0, 10000.0)));
        assert_eq!(clamp_ranges::primary_dimension_range("spirit"), Some((0.0, 10000.0)));
        assert_eq!(clamp_ranges::primary_dimension_range("luck"), Some((0.0, 10000.0)));
        assert_eq!(clamp_ranges::primary_dimension_range("health"), Some((0.0, 1000000.0)));
        assert_eq!(clamp_ranges::primary_dimension_range("mana"), Some((0.0, 1000000.0)));
        assert_eq!(clamp_ranges::primary_dimension_range("stamina"), Some((0.0, 1000000.0)));
        assert_eq!(clamp_ranges::primary_dimension_range("experience"), Some((0.0, f64::MAX)));
        assert_eq!(clamp_ranges::primary_dimension_range("level"), Some((1.0, 1000.0)));
        assert_eq!(clamp_ranges::primary_dimension_range("unknown"), None);
    }

    #[test]
    fn test_clamp_ranges_derived_dimension_range() {
        assert_eq!(clamp_ranges::derived_dimension_range("attack_power"), Some((0.0, 100000.0)));
        assert_eq!(clamp_ranges::derived_dimension_range("defense_power"), Some((0.0, 100000.0)));
        assert_eq!(clamp_ranges::derived_dimension_range("critical_hit_chance"), Some((0.0, 100.0)));
        assert_eq!(clamp_ranges::derived_dimension_range("critical_hit_damage"), Some((0.0, 1000.0)));
        assert_eq!(clamp_ranges::derived_dimension_range("attack_speed"), Some((0.1, 10.0)));
        assert_eq!(clamp_ranges::derived_dimension_range("movement_speed"), Some((0.1, 50.0)));
        assert_eq!(clamp_ranges::derived_dimension_range("casting_speed"), Some((0.1, 10.0)));
        assert_eq!(clamp_ranges::derived_dimension_range("cooldown_reduction"), Some((0.0, 90.0)));
        assert_eq!(clamp_ranges::derived_dimension_range("life_steal"), Some((0.0, 100.0)));
        assert_eq!(clamp_ranges::derived_dimension_range("mana_steal"), Some((0.0, 100.0)));
        assert_eq!(clamp_ranges::derived_dimension_range("damage_reduction"), Some((0.0, 95.0)));
        assert_eq!(clamp_ranges::derived_dimension_range("elemental_resistance"), Some((0.0, 100.0)));
        assert_eq!(clamp_ranges::derived_dimension_range("unknown"), None);
    }

    #[test]
    fn test_clamp_ranges_get_range() {
        assert_eq!(clamp_ranges::get_range("strength"), Some((0.0, 10000.0)));
        assert_eq!(clamp_ranges::get_range("attack_power"), Some((0.0, 100000.0)));
        assert_eq!(clamp_ranges::get_range("unknown"), None);
    }

    #[test]
    fn test_timeouts_constants() {
        assert_eq!(timeouts::AGGREGATION_TIMEOUT, 5000);
        assert_eq!(timeouts::CACHE_TIMEOUT, 1000);
        assert_eq!(timeouts::DATABASE_TIMEOUT, 10000);
        assert_eq!(timeouts::NETWORK_TIMEOUT, 30000);
        assert_eq!(timeouts::SUBSYSTEM_TIMEOUT, 2000);
        assert_eq!(timeouts::BATCH_INTERVAL, 100);
        assert_eq!(timeouts::CACHE_CLEANUP_INTERVAL, 3600);
    }

    #[test]
    fn test_cache_keys_constants() {
        assert_eq!(cache_keys::ACTOR_SNAPSHOT_PREFIX, "actor_snapshot:");
        assert_eq!(cache_keys::SUBSYSTEM_OUTPUT_PREFIX, "subsystem_output:");
        assert_eq!(cache_keys::EFFECTIVE_CAPS_PREFIX, "effective_caps:");
        assert_eq!(cache_keys::REGISTRY_PREFIX, "registry:");
        assert_eq!(cache_keys::CONFIG_PREFIX, "config:");
    }

    #[test]
    fn test_log_levels_constants() {
        assert_eq!(log_levels::TRACE, "trace");
        assert_eq!(log_levels::DEBUG, "debug");
        assert_eq!(log_levels::INFO, "info");
        assert_eq!(log_levels::WARN, "warn");
        assert_eq!(log_levels::ERROR, "error");
    }

    #[test]
    fn test_cache_policies_constants() {
        assert_eq!(cache_policies::LRU, "lru");
        assert_eq!(cache_policies::LFU, "lfu");
        assert_eq!(cache_policies::TTL, "ttl");
        assert_eq!(cache_policies::FIFO, "fifo");
    }

    #[test]
    fn test_performance_thresholds_constants() {
        assert_eq!(performance_thresholds::MAX_AGGREGATION_TIME, 1_000_000);
        assert_eq!(performance_thresholds::MAX_CACHE_TIME, 1_000);
        assert_eq!(performance_thresholds::MAX_SUBSYSTEM_TIME, 100_000);
        assert_eq!(performance_thresholds::MAX_MEMORY_PER_ACTOR, 1_048_576);
        assert_eq!(performance_thresholds::MAX_CACHE_SIZE, 100_048_576);
    }

    #[test]
    fn test_validation_rules_constants() {
        assert_eq!(validation_rules::MIN_ACTOR_NAME_LENGTH, 3);
        assert_eq!(validation_rules::MAX_ACTOR_NAME_LENGTH, 32);
        assert_eq!(validation_rules::MIN_DIMENSION_NAME_LENGTH, 1);
        assert_eq!(validation_rules::MAX_DIMENSION_NAME_LENGTH, 64);
        assert_eq!(validation_rules::MIN_SYSTEM_ID_LENGTH, 1);
        assert_eq!(validation_rules::MAX_SYSTEM_ID_LENGTH, 32);
        assert_eq!(validation_rules::MAX_SUBSYSTEMS_PER_ACTOR, 100);
        assert_eq!(validation_rules::MAX_CONTRIBUTIONS_PER_SUBSYSTEM, 1000);
    }

    #[test]
    fn test_all_dimensions_function() {
        let dimensions = all_dimensions();
        assert!(dimensions.contains(&"strength"));
        assert!(dimensions.contains(&"agility"));
        assert!(dimensions.contains(&"intelligence"));
        assert!(dimensions.contains(&"vitality"));
        assert!(dimensions.contains(&"spirit"));
        assert!(dimensions.contains(&"luck"));
        assert!(dimensions.contains(&"health"));
        assert!(dimensions.contains(&"mana"));
        assert!(dimensions.contains(&"stamina"));
        assert!(dimensions.contains(&"experience"));
        assert!(dimensions.contains(&"level"));
        assert!(dimensions.contains(&"attack_power"));
        assert!(dimensions.contains(&"defense_power"));
        assert!(dimensions.contains(&"realm_id"));
        assert!(dimensions.contains(&"world_id"));
        assert!(dimensions.contains(&"zone_id"));
        assert!(dimensions.contains(&"guild_id"));
        assert!(dimensions.contains(&"party_id"));
        assert!(dimensions.contains(&"event_id"));
    }

    #[test]
    fn test_all_context_types_function() {
        let context_types = all_context_types();
        assert!(context_types.contains(&"damage"));
        assert!(context_types.contains(&"healing"));
        assert!(context_types.contains(&"experience_gain"));
        assert!(context_types.contains(&"item_drop"));
        assert!(context_types.contains(&"combat"));
        assert!(context_types.contains(&"movement"));
        assert!(context_types.contains(&"casting"));
    }

    #[test]
    fn test_all_system_ids_function() {
        let system_ids = all_system_ids();
        assert!(system_ids.contains(&"luyen_the"));
        assert!(system_ids.contains(&"kim_dan"));
        assert!(system_ids.contains(&"combat"));
        assert!(system_ids.contains(&"equipment"));
        assert!(system_ids.contains(&"buff"));
        assert!(system_ids.contains(&"guild"));
        assert!(system_ids.contains(&"event"));
        assert!(system_ids.contains(&"world"));
        assert!(system_ids.contains(&"magic"));
        assert!(system_ids.contains(&"cultivation"));
        assert!(system_ids.contains(&"experience"));
        assert!(system_ids.contains(&"reputation"));
        assert!(system_ids.contains(&"trading"));
        assert!(system_ids.contains(&"weather"));
        assert!(system_ids.contains(&"location"));
        assert!(system_ids.contains(&"time"));
        assert!(system_ids.contains(&"stealth"));
        assert!(system_ids.contains(&"perception"));
    }

    #[test]
    fn test_dimension_ranges_constants() {
        assert_eq!(dimension_ranges::MIN_STRENGTH, 1.0);
        assert_eq!(dimension_ranges::MAX_STRENGTH, 1000.0);
        assert_eq!(dimension_ranges::MIN_AGILITY, 1.0);
        assert_eq!(dimension_ranges::MAX_AGILITY, 1000.0);
        assert_eq!(dimension_ranges::MIN_INTELLIGENCE, 1.0);
        assert_eq!(dimension_ranges::MAX_INTELLIGENCE, 1000.0);
        assert_eq!(dimension_ranges::MIN_VITALITY, 1.0);
        assert_eq!(dimension_ranges::MAX_VITALITY, 1000.0);
        assert_eq!(dimension_ranges::MIN_SPIRIT, 1.0);
        assert_eq!(dimension_ranges::MAX_SPIRIT, 1000.0);
        assert_eq!(dimension_ranges::MIN_LUCK, 1.0);
        assert_eq!(dimension_ranges::MAX_LUCK, 1000.0);
        assert_eq!(dimension_ranges::MIN_HEALTH, 1.0);
        assert_eq!(dimension_ranges::MAX_HEALTH, 10000.0);
        assert_eq!(dimension_ranges::MIN_MANA, 0.0);
        assert_eq!(dimension_ranges::MAX_MANA, 10000.0);
        assert_eq!(dimension_ranges::MIN_STAMINA, 0.0);
        assert_eq!(dimension_ranges::MAX_STAMINA, 10000.0);
        assert_eq!(dimension_ranges::MIN_ATTACK_POWER, 0.0);
        assert_eq!(dimension_ranges::MAX_ATTACK_POWER, 5000.0);
        assert_eq!(dimension_ranges::MIN_DEFENSE_POWER, 0.0);
        assert_eq!(dimension_ranges::MAX_DEFENSE_POWER, 5000.0);
        assert_eq!(dimension_ranges::MIN_CRITICAL_HIT_CHANCE, 0.0);
        assert_eq!(dimension_ranges::MAX_CRITICAL_HIT_CHANCE, 1.0);
        assert_eq!(dimension_ranges::MIN_CRITICAL_HIT_DAMAGE, 1.0);
        assert_eq!(dimension_ranges::MAX_CRITICAL_HIT_DAMAGE, 5.0);
        assert_eq!(dimension_ranges::MIN_ATTACK_SPEED, 0.1);
        assert_eq!(dimension_ranges::MAX_ATTACK_SPEED, 10.0);
        assert_eq!(dimension_ranges::MIN_MOVEMENT_SPEED, 0.1);
        assert_eq!(dimension_ranges::MAX_MOVEMENT_SPEED, 20.0);
        assert_eq!(dimension_ranges::MIN_CASTING_SPEED, 0.1);
        assert_eq!(dimension_ranges::MAX_CASTING_SPEED, 10.0);
        assert_eq!(dimension_ranges::MIN_COOLDOWN_REDUCTION, 0.0);
        assert_eq!(dimension_ranges::MAX_COOLDOWN_REDUCTION, 0.8);
        assert_eq!(dimension_ranges::MIN_EXPERIENCE, 0.0);
        assert_eq!(dimension_ranges::MAX_EXPERIENCE, 999999999.0);
        assert_eq!(dimension_ranges::MIN_LEVEL, 1.0);
        assert_eq!(dimension_ranges::MAX_LEVEL, 1000.0);
        assert_eq!(dimension_ranges::MIN_FIRE_RESISTANCE, 0.0);
        assert_eq!(dimension_ranges::MAX_FIRE_RESISTANCE, 1.0);
        assert_eq!(dimension_ranges::MIN_WATER_RESISTANCE, 0.0);
        assert_eq!(dimension_ranges::MAX_WATER_RESISTANCE, 1.0);
        assert_eq!(dimension_ranges::MIN_EARTH_RESISTANCE, 0.0);
        assert_eq!(dimension_ranges::MAX_EARTH_RESISTANCE, 1.0);
        assert_eq!(dimension_ranges::MIN_AIR_RESISTANCE, 0.0);
        assert_eq!(dimension_ranges::MAX_AIR_RESISTANCE, 1.0);
        assert_eq!(dimension_ranges::MIN_LIGHT_RESISTANCE, 0.0);
        assert_eq!(dimension_ranges::MAX_LIGHT_RESISTANCE, 1.0);
        assert_eq!(dimension_ranges::MIN_DARK_RESISTANCE, 0.0);
        assert_eq!(dimension_ranges::MAX_DARK_RESISTANCE, 1.0);
        assert_eq!(dimension_ranges::MIN_LUCK_FACTOR, 0.0);
        assert_eq!(dimension_ranges::MAX_LUCK_FACTOR, 2.0);
        assert_eq!(dimension_ranges::MIN_CRITICAL_RESISTANCE, 0.0);
        assert_eq!(dimension_ranges::MAX_CRITICAL_RESISTANCE, 1.0);
        assert_eq!(dimension_ranges::MIN_DODGE_CHANCE, 0.0);
        assert_eq!(dimension_ranges::MAX_DODGE_CHANCE, 0.95);
        assert_eq!(dimension_ranges::MIN_BLOCK_CHANCE, 0.0);
        assert_eq!(dimension_ranges::MAX_BLOCK_CHANCE, 0.95);
        assert_eq!(dimension_ranges::MIN_PARRY_CHANCE, 0.0);
        assert_eq!(dimension_ranges::MAX_PARRY_CHANCE, 0.95);
    }
}