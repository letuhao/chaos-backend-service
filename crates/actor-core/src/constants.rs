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
