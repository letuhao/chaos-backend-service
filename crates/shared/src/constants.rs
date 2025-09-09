//! Constants used across the Chaos World backend.

/// Maximum number of characters per player account.
pub const MAX_CHARACTERS_PER_ACCOUNT: usize = 10;

/// Maximum length of character names.
pub const MAX_CHARACTER_NAME_LENGTH: usize = 32;

/// Minimum length of character names.
pub const MIN_CHARACTER_NAME_LENGTH: usize = 3;

/// Maximum level for characters.
pub const MAX_CHARACTER_LEVEL: u32 = 1000;

/// Maximum inventory slots.
pub const MAX_INVENTORY_SLOTS: usize = 200;

/// Maximum guild members.
pub const MAX_GUILD_MEMBERS: usize = 1000;

/// Maximum party members.
pub const MAX_PARTY_MEMBERS: usize = 8;

/// Maximum raid members.
pub const MAX_RAID_MEMBERS: usize = 40;

/// Default session timeout in seconds.
pub const DEFAULT_SESSION_TIMEOUT: u64 = 3600; // 1 hour

/// Default cache TTL in seconds.
pub const DEFAULT_CACHE_TTL: u64 = 300; // 5 minutes

/// Maximum number of concurrent connections per IP.
pub const MAX_CONNECTIONS_PER_IP: usize = 10;

/// Maximum message size in bytes.
pub const MAX_MESSAGE_SIZE: usize = 1024 * 1024; // 1MB

/// Maximum batch size for database operations.
pub const MAX_BATCH_SIZE: usize = 1000;

/// Default database connection pool size.
pub const DEFAULT_DB_POOL_SIZE: u32 = 10;

/// Default Redis connection pool size.
pub const DEFAULT_REDIS_POOL_SIZE: u32 = 10;

/// Game world constants.
pub mod world {
    /// Maximum world width in units.
    pub const MAX_WORLD_WIDTH: f64 = 10000.0;
    
    /// Maximum world height in units.
    pub const MAX_WORLD_HEIGHT: f64 = 10000.0;
    
    /// Default zone size in units.
    pub const DEFAULT_ZONE_SIZE: f64 = 1000.0;
    
    /// Maximum number of zones.
    pub const MAX_ZONES: usize = 100;
}

/// Combat constants.
pub mod combat {
    /// Maximum combat range in units.
    pub const MAX_COMBAT_RANGE: f64 = 50.0;
    
    /// Default attack speed in attacks per second.
    pub const DEFAULT_ATTACK_SPEED: f64 = 1.0;
    
    /// Maximum skill cooldown in seconds.
    pub const MAX_SKILL_COOLDOWN: u64 = 300; // 5 minutes
    
    /// Maximum buff duration in seconds.
    pub const MAX_BUFF_DURATION: u64 = 3600; // 1 hour
}

/// Economy constants.
pub mod economy {
    /// Maximum gold amount.
    pub const MAX_GOLD: u64 = 1_000_000_000; // 1 billion
    
    /// Default starting gold.
    pub const DEFAULT_STARTING_GOLD: u64 = 1000;
    
    /// Maximum item stack size.
    pub const MAX_ITEM_STACK: u32 = 9999;
    
    /// Maximum auction house listings per player.
    pub const MAX_AUCTION_LISTINGS: usize = 50;
}
