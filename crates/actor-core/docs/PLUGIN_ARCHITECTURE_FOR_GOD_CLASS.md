# 🔌 Plugin Architecture for Actor God Class

## 📋 Problem Statement

Chúng ta có nhiều leveling systems khác nhau:
- **RPG Leveling System** (8 primary attributes)
- **Jindan System** (cultivation/immortal system) 
- **Dao System** (philosophy/comprehension system)
- **Magic System** (mana/spells)
- **Combat System** (damage/resistances)
- **Crafting System** (skills/recipes)

**Vấn đề**: Không thể hardcode tất cả vào God Class vì:
1. Quá nhiều systems → God Class quá lớn
2. Không thể biết trước tất cả systems
3. Modders cần thêm systems mới
4. Performance sẽ bị ảnh hưởng

## 🎯 Solution: Hybrid Plugin Architecture

### **Core Principle:**
- **God Class**: Chỉ chứa core stats (health, mana, stamina, level, experience)
- **Plugin Systems**: Dynamic loading và management của specialized systems
- **Performance**: Core stats = hardcoded, Plugin stats = optimized access

## 🏗️ Architecture Design

### **1. Core God Class Structure**

```rust
pub struct ActorGodClass {
    // === CORE FIELDS (Hardcoded for Performance) ===
    pub id: EntityId,
    pub name: String,
    pub race: String,
    pub lifespan: i64,
    pub age: i64,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
    pub version: Version,
    
    // === CORE STATS (Hardcoded for Performance) ===
    pub health: f64,
    pub mana: f64,
    pub stamina: f64,
    pub level: i32,
    pub experience: f64,
    
    // === CORE DERIVED STATS (Hardcoded for Performance) ===
    pub health_max: f64,
    pub mana_max: f64,
    pub stamina_max: f64,
    pub strength: f64,
    pub intelligence: f64,
    pub agility: f64,
    pub endurance: f64,
    
    // === PLUGIN SYSTEM REGISTRY ===
    pub plugin_systems: HashMap<String, Box<dyn PluginSystem>>,
    
    // === PLUGIN STATS CACHE (Optimized Access) ===
    pub plugin_stats: HashMap<String, f64>,
    
    // === COMPATIBILITY LAYER ===
    pub subsystems: Vec<Subsystem>,
    pub data: HashMap<String, serde_json::Value>,
}
```

### **2. Plugin System Trait**

```rust
/// Base trait for all plugin systems
pub trait PluginSystem: Send + Sync {
    /// Unique identifier for this plugin system
    fn system_id(&self) -> &str;
    
    /// Priority for processing (higher = processed first)
    fn priority(&self) -> i32;
    
    /// Initialize the plugin system
    fn initialize(&mut self, actor: &mut ActorGodClass) -> ActorCoreResult<()>;
    
    /// Process the plugin system and return contributions
    fn process(&self, actor: &ActorGodClass, context: &PluginContext) -> ActorCoreResult<PluginOutput>;
    
    /// Get stats provided by this plugin system
    fn get_stats(&self) -> HashMap<String, f64>;
    
    /// Set stats for this plugin system
    fn set_stats(&mut self, stats: HashMap<String, f64>) -> ActorCoreResult<()>;
    
    /// Get configuration for this plugin system
    fn get_config(&self) -> PluginConfig;
    
    /// Update configuration for this plugin system
    fn update_config(&mut self, config: PluginConfig) -> ActorCoreResult<()>;
}

/// Context passed to plugin systems
pub struct PluginContext {
    pub delta_time: f64,
    pub events: Vec<PluginEvent>,
    pub global_modifiers: HashMap<String, f64>,
}

/// Output from plugin systems
pub struct PluginOutput {
    pub stat_contributions: HashMap<String, f64>,
    pub cap_contributions: Vec<CapContribution>,
    pub context_modifiers: HashMap<String, f64>,
    pub events: Vec<PluginEvent>,
}

/// Configuration for plugin systems
pub struct PluginConfig {
    pub enabled: bool,
    pub priority: i32,
    pub settings: HashMap<String, serde_json::Value>,
}
```

### **3. Plugin Registry**

```rust
/// Registry for managing plugin systems
pub struct PluginRegistry {
    pub systems: HashMap<String, Box<dyn PluginSystem>>,
    pub loaders: HashMap<String, Box<dyn PluginLoader>>,
    pub configs: HashMap<String, PluginConfig>,
}

impl PluginRegistry {
    /// Register a plugin system
    pub fn register_plugin(&mut self, system: Box<dyn PluginSystem>) -> ActorCoreResult<()>;
    
    /// Load plugin from configuration
    pub fn load_plugin(&mut self, config: PluginConfig) -> ActorCoreResult<()>;
    
    /// Unload plugin system
    pub fn unload_plugin(&mut self, system_id: &str) -> ActorCoreResult<()>;
    
    /// Get plugin system
    pub fn get_plugin(&self, system_id: &str) -> Option<&dyn PluginSystem>;
    
    /// Get plugin system mutably
    pub fn get_plugin_mut(&mut self, system_id: &str) -> Option<&mut dyn PluginSystem>;
    
    /// Process all plugin systems
    pub fn process_all(&self, actor: &ActorGodClass, context: &PluginContext) -> ActorCoreResult<Vec<PluginOutput>>;
}
```

## 🔧 Implementation Strategy

### **Phase 1: Core Plugin Infrastructure**

1. **Plugin System Trait**: Define base interface
2. **Plugin Registry**: Manage plugin lifecycle
3. **Plugin Context**: Communication between systems
4. **Plugin Output**: Standardized output format

### **Phase 2: Core Stats Optimization**

1. **Hardcoded Core Stats**: Health, Mana, Stamina, Level, Experience
2. **Optimized Access**: Direct field access for core stats
3. **Plugin Stats Cache**: HashMap for plugin-provided stats
4. **Hybrid Access**: Core stats = direct, Plugin stats = cached

### **Phase 3: Plugin System Implementations**

1. **RPG Leveling Plugin**: 8 primary attributes
2. **Jindan System Plugin**: Cultivation stats
3. **Dao System Plugin**: Philosophy/comprehension
4. **Magic System Plugin**: Spells/mana management
5. **Combat System Plugin**: Damage/resistances

### **Phase 4: Performance Optimization**

1. **Plugin Stats Caching**: Cache frequently accessed plugin stats
2. **Lazy Loading**: Load plugins only when needed
3. **Batch Processing**: Process multiple plugins together
4. **Memory Pooling**: Reuse plugin instances

## 📊 Performance Analysis

### **Core Stats (Hardcoded)**
- **Access Time**: ~1-2ns (direct field access)
- **Memory**: Fixed size, cache-friendly
- **Compilation**: Full optimization possible

### **Plugin Stats (Cached)**
- **Access Time**: ~5-10ns (HashMap lookup + cache)
- **Memory**: Dynamic, configurable
- **Compilation**: Partial optimization

### **Hybrid Approach Benefits**
- **Core stats**: Maximum performance (1-2ns)
- **Plugin stats**: Good performance (5-10ns)
- **Flexibility**: Unlimited plugin systems
- **Moddability**: Easy to add new systems

## 🎮 Usage Examples

### **1. Basic Usage**

```rust
// Create Actor God Class
let mut actor = ActorGodClass::new("Hero".to_string(), "Human".to_string());

// Register plugin systems
actor.register_plugin(Box::new(RpgLevelingPlugin::new()));
actor.register_plugin(Box::new(JindanSystemPlugin::new()));
actor.register_plugin(Box::new(DaoSystemPlugin::new()));

// Access core stats (fast)
let health = actor.health;
let mana = actor.mana;
let level = actor.level;

// Access plugin stats (cached)
let strength = actor.get_plugin_stat("rpg_leveling", "strength");
let vital_essence = actor.get_plugin_stat("jindan_system", "vital_essence");
let dao_comprehension = actor.get_plugin_stat("dao_system", "dao_comprehension");
```

### **2. Plugin System Implementation**

```rust
pub struct RpgLevelingPlugin {
    pub strength: f64,
    pub intelligence: f64,
    pub willpower: f64,
    pub agility: f64,
    pub speed: f64,
    pub endurance: f64,
    pub personality: f64,
    pub luck: f64,
    pub level: i32,
    pub experience: f64,
    pub points_unspent: i32,
}

impl PluginSystem for RpgLevelingPlugin {
    fn system_id(&self) -> &str { "rpg_leveling" }
    
    fn priority(&self) -> i32 { 100 }
    
    fn get_stats(&self) -> HashMap<String, f64> {
        let mut stats = HashMap::new();
        stats.insert("strength".to_string(), self.strength);
        stats.insert("intelligence".to_string(), self.intelligence);
        stats.insert("willpower".to_string(), self.willpower);
        stats.insert("agility".to_string(), self.agility);
        stats.insert("speed".to_string(), self.speed);
        stats.insert("endurance".to_string(), self.endurance);
        stats.insert("personality".to_string(), self.personality);
        stats.insert("luck".to_string(), self.luck);
        stats.insert("level".to_string(), self.level as f64);
        stats.insert("experience".to_string(), self.experience);
        stats
    }
    
    fn process(&self, actor: &ActorGodClass, context: &PluginContext) -> ActorCoreResult<PluginOutput> {
        // Process RPG leveling logic
        let mut output = PluginOutput::new();
        
        // Calculate derived stats
        let hp_max = self.strength * 10.0 + self.endurance * 5.0;
        let mana_max = self.intelligence * 8.0 + self.willpower * 6.0;
        let stamina_max = self.endurance * 12.0 + self.agility * 4.0;
        
        // Add contributions
        output.stat_contributions.insert("hp_max".to_string(), hp_max);
        output.stat_contributions.insert("mana_max".to_string(), mana_max);
        output.stat_contributions.insert("stamina_max".to_string(), stamina_max);
        
        Ok(output)
    }
}
```

### **3. Configuration-Driven Loading**

```yaml
# config/plugins.yaml
plugins:
  rpg_leveling:
    enabled: true
    priority: 100
    settings:
      points_per_level_min: 3
      points_per_level_max: 10
      xp_base: 1000
      xp_growth: 1.2
  
  jindan_system:
    enabled: true
    priority: 200
    settings:
      realm: "QiRefining"
      stage: "Early"
      vital_essence: 100.0
      qi_control: 80.0
  
  dao_system:
    enabled: true
    priority: 300
    settings:
      dao_comprehension: 50.0
      profound_meaning: 30.0
```

## 🚀 Benefits

### **1. Performance**
- **Core stats**: 1-2ns access time
- **Plugin stats**: 5-10ns access time
- **Memory efficient**: Only load needed plugins
- **Cache friendly**: Core stats in contiguous memory

### **2. Flexibility**
- **Unlimited systems**: Add any number of plugin systems
- **Dynamic loading**: Load/unload plugins at runtime
- **Configuration**: Easy to configure plugin behavior
- **Moddability**: Modders can create custom plugins

### **3. Maintainability**
- **Separation of concerns**: Each system is independent
- **Easy testing**: Test plugins in isolation
- **Version control**: Track plugin changes separately
- **Documentation**: Each plugin can have its own docs

### **4. Scalability**
- **Horizontal scaling**: Add more plugin systems
- **Vertical scaling**: Optimize individual plugins
- **Memory scaling**: Load only needed plugins
- **Performance scaling**: Cache frequently accessed stats

## 🔮 Future Enhancements

1. **Plugin Hot Reloading**: Reload plugins without restart
2. **Plugin Dependencies**: Manage plugin dependencies
3. **Plugin Versioning**: Version control for plugins
4. **Plugin Marketplace**: Share plugins between projects
5. **Plugin Analytics**: Monitor plugin performance
6. **Plugin Security**: Sandbox plugin execution

---

*This architecture provides the perfect balance between performance (hardcoded core stats) and flexibility (plugin systems) for the Actor God Class.*
