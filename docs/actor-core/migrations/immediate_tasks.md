# Immediate Implementation Tasks

## 🚨 High Priority - Implement First

### Actor Struct - Missing Critical Methods

```rust
// Add these methods to impl Actor in types.rs

/// Get a subsystem by system ID
pub fn get_subsystem(&self, system_id: &str) -> Option<&Subsystem> {
    self.subsystems.iter().find(|s| s.system_id == system_id)
}

/// Check if actor has a specific subsystem
pub fn has_subsystem(&self, system_id: &str) -> bool {
    self.subsystems.iter().any(|s| s.system_id == system_id)
}

/// Get guild ID from actor data
pub fn get_guild_id(&self) -> Option<&str> {
    self.data.get("guild_id")
        .and_then(|v| v.as_str())
}

/// Set guild ID in actor data
pub fn set_guild_id(&mut self, guild_id: String) {
    self.data.insert("guild_id".to_string(), serde_json::Value::String(guild_id));
    self.touch();
}

/// Set combat status
pub fn set_in_combat(&mut self, in_combat: bool) {
    self.data.insert("in_combat".to_string(), serde_json::Value::Bool(in_combat));
    if in_combat {
        self.data.insert("combat_start_time".to_string(), 
            serde_json::Value::Number(serde_json::Number::from(Utc::now().timestamp())));
    }
    self.touch();
}

/// Check if actor has a specific buff
pub fn has_buff(&self, buff_id: &str) -> bool {
    self.data.get("buffs")
        .and_then(|v| v.as_array())
        .map(|buffs| buffs.iter().any(|b| b.as_str() == Some(buff_id)))
        .unwrap_or(false)
}

/// Add a buff to the actor
pub fn add_buff(&mut self, buff_id: String) {
    let buffs = self.data.entry("buffs".to_string())
        .or_insert_with(|| serde_json::Value::Array(Vec::new()));
    if let Some(buffs_array) = buffs.as_array_mut() {
        if !buffs_array.iter().any(|b| b.as_str() == Some(&buff_id)) {
            buffs_array.push(serde_json::Value::String(buff_id));
        }
    }
    self.touch();
}

/// Remove a buff from the actor
pub fn remove_buff(&mut self, buff_id: &str) -> bool {
    if let Some(buffs) = self.data.get_mut("buffs").and_then(|v| v.as_array_mut()) {
        if let Some(pos) = buffs.iter().position(|b| b.as_str() == Some(buff_id)) {
            buffs.remove(pos);
            self.touch();
            return true;
        }
    }
    false
}
```

### Caps Struct - Missing Critical Methods

```rust
// Add these methods to impl Caps in types.rs

/// Check if a value is within the caps range
pub fn contains(&self, value: f64) -> bool {
    value >= self.min && value <= self.max
}

/// Check if the caps range is empty (min > max)
pub fn is_empty(&self) -> bool {
    self.min > self.max
}

/// Get the range size (max - min)
pub fn get_range(&self) -> f64 {
    self.max - self.min
}

/// Get the center point of the range
pub fn get_center(&self) -> f64 {
    (self.min + self.max) / 2.0
}

/// Expand the range by the given amount
pub fn expand(&mut self, amount: f64) {
    self.min -= amount;
    self.max += amount;
}

/// Shrink the range by the given amount
pub fn shrink(&mut self, amount: f64) {
    self.min += amount;
    self.max -= amount;
    // Ensure min doesn't exceed max
    if self.min > self.max {
        let center = (self.min + self.max) / 2.0;
        self.min = center;
        self.max = center;
    }
}

/// Set both min and max values
pub fn set(&mut self, min: f64, max: f64) {
    self.min = min;
    self.max = max;
}
```

### Constants - Missing Critical Constants

```rust
// Add these to constants.rs

/// System identifiers for various game systems.
pub mod system_ids {
    // Existing constants...
    pub const LUYEN_THE: &str = "luyen_the";
    pub const KIM_DAN: &str = "kim_dan";
    pub const COMBAT: &str = "combat";
    pub const EQUIPMENT: &str = "equipment";
    pub const BUFF: &str = "buff";
    pub const GUILD: &str = "guild";
    pub const EVENT: &str = "event";
    pub const WORLD: &str = "world";
    
    // Missing constants that need to be added
    pub const MAGIC: &str = "magic";
    pub const CULTIVATION: &str = "cultivation";
    pub const EXPERIENCE: &str = "experience";
    pub const REPUTATION: &str = "reputation";
    pub const TRADING: &str = "trading";
    pub const WEATHER: &str = "weather";
    pub const LOCATION: &str = "location";
    pub const TIME: &str = "time";
    pub const STEALTH: &str = "stealth";
    pub const PERCEPTION: &str = "perception";
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
}
```

### Service Methods - Missing Critical Methods

```rust
// Add these methods to AggregatorImpl in services.rs

impl Aggregator for AggregatorImpl {
    async fn resolve_with_context(&self, actor: &Actor, context: &str) -> ActorCoreResult<Snapshot> {
        // Implementation for contextual resolution
        // This would apply context-specific modifiers
        let mut snapshot = self.resolve(actor).await?;
        
        // Apply context-specific modifications
        if let Some(context_modifier) = self.get_context_modifier(context).await? {
            snapshot = context_modifier.apply(snapshot);
        }
        
        Ok(snapshot)
    }

    async fn resolve_batch(&self, actors: &[Actor]) -> ActorCoreResult<Vec<Snapshot>> {
        // Implementation for batch processing
        let mut snapshots = Vec::with_capacity(actors.len());
        
        for actor in actors {
            let snapshot = self.resolve(actor).await?;
            snapshots.push(snapshot);
        }
        
        Ok(snapshots)
    }
}

impl AggregatorImpl {
    fn get_cached_snapshot(&self, actor_id: &Uuid) -> Option<Snapshot> {
        let cache_key = format!("{}", actor_id);
        self.cache.get(&cache_key)
            .and_then(|v| serde_json::from_value::<Snapshot>(v).ok())
    }

    fn invalidate_cache(&self, actor_id: &Uuid) {
        let cache_key = format!("{}", actor_id);
        self.cache.remove(&cache_key);
    }

    fn clear_cache(&self) {
        self.cache.clear();
    }

    fn get_metrics(&self) -> AggregatorMetrics {
        self.metrics.read().unwrap().clone()
    }
    
    async fn get_context_modifier(&self, context: &str) -> ActorCoreResult<Option<ContextModifier>> {
        // Implementation for getting context-specific modifiers
        todo!("Implement context modifier lookup")
    }
}
```

## 🧪 Test Files to Create

### tests/actor_tests.rs
```rust
use actor_core::Actor;
use actor_core::enums::Bucket;

#[tokio::test]
async fn test_actor_creation() {
    let actor = Actor::new("TestActor".to_string(), "Human".to_string());
    assert!(actor.is_valid());
    assert_eq!(actor.name, "TestActor");
    assert_eq!(actor.race, "Human");
}

#[tokio::test]
async fn test_actor_subsystem_management() {
    let mut actor = Actor::new("TestActor".to_string(), "Human".to_string());
    
    // Test adding subsystem
    let subsystem = actor_core::Subsystem::new("combat".to_string(), 1);
    actor.add_subsystem(subsystem);
    assert!(actor.has_subsystem("combat"));
    
    // Test removing subsystem
    assert!(actor.remove_subsystem("combat"));
    assert!(!actor.has_subsystem("combat"));
}

#[tokio::test]
async fn test_actor_guild_management() {
    let mut actor = Actor::new("TestActor".to_string(), "Human".to_string());
    
    // Test setting guild ID
    actor.set_guild_id("guild_123".to_string());
    assert_eq!(actor.get_guild_id(), Some("guild_123"));
    
    // Test clearing guild ID
    actor.set_guild_id("".to_string());
    assert_eq!(actor.get_guild_id(), Some(""));
}

#[tokio::test]
async fn test_actor_combat_management() {
    let mut actor = Actor::new("TestActor".to_string(), "Human".to_string());
    
    // Test setting combat status
    actor.set_in_combat(true);
    assert!(actor.is_in_combat());
    
    actor.set_in_combat(false);
    assert!(!actor.is_in_combat());
}

#[tokio::test]
async fn test_actor_buff_management() {
    let mut actor = Actor::new("TestActor".to_string(), "Human".to_string());
    
    // Test adding buffs
    actor.add_buff("strength_buff".to_string());
    assert!(actor.has_buff("strength_buff"));
    
    // Test removing buffs
    assert!(actor.remove_buff("strength_buff"));
    assert!(!actor.has_buff("strength_buff"));
}
```

### tests/caps_tests.rs
```rust
use actor_core::Caps;

#[test]
fn test_caps_creation() {
    let caps = Caps::new(10.0, 20.0);
    assert!(caps.is_valid());
    assert_eq!(caps.min, 10.0);
    assert_eq!(caps.max, 20.0);
}

#[test]
fn test_caps_contains() {
    let caps = Caps::new(10.0, 20.0);
    assert!(caps.contains(15.0));
    assert!(!caps.contains(5.0));
    assert!(!caps.contains(25.0));
}

#[test]
fn test_caps_is_empty() {
    let caps = Caps::new(10.0, 20.0);
    assert!(!caps.is_empty());
    
    let empty_caps = Caps::new(20.0, 10.0);
    assert!(empty_caps.is_empty());
}

#[test]
fn test_caps_get_range() {
    let caps = Caps::new(10.0, 20.0);
    assert_eq!(caps.get_range(), 10.0);
}

#[test]
fn test_caps_get_center() {
    let caps = Caps::new(10.0, 20.0);
    assert_eq!(caps.get_center(), 15.0);
}

#[test]
fn test_caps_expand() {
    let mut caps = Caps::new(10.0, 20.0);
    caps.expand(5.0);
    assert_eq!(caps.min, 5.0);
    assert_eq!(caps.max, 25.0);
}

#[test]
fn test_caps_shrink() {
    let mut caps = Caps::new(10.0, 20.0);
    caps.shrink(5.0);
    assert_eq!(caps.min, 15.0);
    assert_eq!(caps.max, 15.0); // Should be equal after shrinking too much
}

#[test]
fn test_caps_set() {
    let mut caps = Caps::new(10.0, 20.0);
    caps.set(5.0, 25.0);
    assert_eq!(caps.min, 5.0);
    assert_eq!(caps.max, 25.0);
}
```

## 🚨 Compilation Fixes

### Fix Unused Field Warnings

1. **Remove unused fields** from structs
2. **Add `#[allow(dead_code)]`** for fields that will be used later
3. **Implement the missing functionality** that uses these fields

### Example Fix for MultiLayerCacheManager

```rust
// In cache/multi_layer.rs, either remove unused fields or implement their usage

pub struct MultiLayerCacheManager {
    l1_cache: Arc<dyn L1Cache>,
    l2_cache: Arc<dyn L2Cache>,
    l3_cache: Arc<dyn L3Cache>,
    config: MultiLayerConfig,
    stats: Arc<RwLock<MultiLayerStats>>,
    // Remove this field if not needed, or implement background sync
    // sync_handle: Option<tokio::task::JoinHandle<()>>,
}
```

## 📋 Implementation Checklist

- [ ] Add 19 missing Actor methods
- [ ] Add 10 missing Caps methods  
- [ ] Add 50+ missing constants
- [ ] Add 7 missing service methods
- [ ] Create comprehensive test suite
- [ ] Fix compilation warnings
- [ ] Update migration tracking document
- [ ] Add integration tests
- [ ] Add performance benchmarks
- [ ] Add API documentation

**Estimated Time: 2-3 weeks for critical items**
