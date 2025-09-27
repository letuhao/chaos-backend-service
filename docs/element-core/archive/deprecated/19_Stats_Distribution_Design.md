# Stats Distribution Design

## ⚠️ **DEPRECATION NOTICE**

**This document is deprecated and has been merged into [Element Registry Design](04_Element_Registry_Design.md).**

For new implementations, please use the unified registry approach described in the Element Registry Design document. Migration guide: [Migration Guide](21_Migration_Guide.md).

---

## 📋 **Tổng Quan** (DEPRECATED)

Tài liệu này mô tả cách phân phối stats từ các hệ thống khác nhau (RPG Primary Stats, Cultivation Systems) vào Element-Core Derived Stats. **Thiết kế này tuân theo pattern của Actor-Core**, nơi các external systems tự đăng ký và định nghĩa cách contribute, Element-Core chỉ làm nhiệm vụ data hub và tổng hợp.

## 🎯 **Mục Tiêu Thiết Kế**

### **1. External Registration System**
- **RPG System**: Tự implement `ElementContributor` và register với Element-Core
- **Cultivation System**: Tự implement `ElementContributor` và register với Element-Core
- **Item System**: Tự implement `ElementContributor` và register với Element-Core
- **Element-Core**: Chỉ làm data hub, cache và aggregation

### **2. Decoupled Architecture**
- **No Code Changes**: Element-Core không cần sửa code khi thêm system mới
- **Self-Managed**: Mỗi external system tự quản lý logic của mình
- **Single Responsibility**: Element-Core chỉ lo cache + aggregate
- **Extensible**: Dễ dàng thêm Talent System, Destiny System, etc.

## 🏗️ **Stats Distribution Architecture**

### **1. External Registration Pattern (Following Actor-Core)**

#### **A. Element-Core Contributor Interface**
```rust
/// Element-Core Contributor trait for external systems to implement
/// Element-Core贡献者特征，供外部系统实现
#[async_trait]
pub trait ElementContributor: Send + Sync {
    /// Get the unique identifier for this contributor
    fn system_id(&self) -> &str;
    
    /// Get the priority of this contributor (higher = more important)
    fn priority(&self) -> i64;
    
    /// Contribute to element derived stats
    /// This method is called during stat aggregation to generate contributions
    async fn contribute_to_element_stats(
        &self, 
        actor: &Actor, 
        element_type: &str
    ) -> ElementCoreResult<ElementContribution>;
    
    /// Validate contributor output
    fn validate_output(&self, output: &ElementContribution) -> ElementCoreResult<()>;
}

/// Element-Core Registry for managing external contributors
/// Element-Core注册表，用于管理外部贡献者
pub trait ElementContributorRegistry: Send + Sync {
    /// Register an external contributor
    fn register(&self, contributor: Arc<dyn ElementContributor>) -> ElementCoreResult<()>;
    
    /// Unregister a contributor by system ID
    fn unregister(&self, system_id: &str) -> ElementCoreResult<()>;
    
    /// Get a contributor by system ID
    fn get_by_id(&self, system_id: &str) -> Option<Arc<dyn ElementContributor>>;
    
    /// Get all contributors ordered by priority
    fn get_by_priority(&self) -> Vec<Arc<dyn ElementContributor>>;
    
    /// Check if a contributor is registered
    fn is_registered(&self, system_id: &str) -> bool;
    
    /// Get the number of registered contributors
    fn count(&self) -> usize;
}

/// Element contribution from an external system
/// 来自外部系统的元素贡献
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElementContribution {
    /// Stat name being contributed to
    pub stat_name: String,
    /// Contribution value
    pub value: f64,
    /// Contribution type (additive, multiplicative, etc.)
    pub contribution_type: ContributionType,
    /// Source system
    pub source_system: String,
    /// Priority for conflict resolution
    pub priority: i64,
    /// Element type this contribution applies to
    pub element_type: String,
    /// Additional metadata
    pub metadata: HashMap<String, serde_json::Value>,
}


/// Contribution types
/// 贡献类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ContributionType {
    Additive,      // Cộng (加法)
    Multiplicative, // Nhân (乘法)
    Override,      // Ghi đè (覆盖)
    Percentage,    // Phần trăm (百分比)
    Flat,          // Giá trị cố định (固定值)
}

/// Element cap contribution
/// 元素上限贡献
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElementCapContribution {
    /// Stat name
    pub stat_name: String,
    /// Cap type
    pub cap_type: CapType,
    /// Cap value
    pub value: f64,
    /// Source system
    pub source_system: String,
    /// Priority
    pub priority: i64,
    /// Element type
    pub element_type: String,
}

/// Cap types
/// 上限类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CapType {
    HardMax,    // Cứng tối đa (硬上限)
    SoftMax,    // Mềm tối đa (软上限)
    HardMin,    // Cứng tối thiểu (硬下限)
    SoftMin,    // Mềm tối thiểu (软下限)
}

/// Element subsystem metadata
/// 元素子系统元数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElementSubsystemMeta {
    /// System identifier
    pub system_id: String,
    /// System version
    pub version: String,
    /// Processing timestamp
    pub timestamp: chrono::DateTime<chrono::Utc>,
    /// Additional metadata
    pub data: HashMap<String, serde_json::Value>,
}
```

#### **B. Contribution Formula System**
```rust
/// Contribution formula definition
/// 贡献公式定义
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContributionFormula {
    /// Formula identifier
    pub formula_id: String,
    /// Target stat name
    pub target_stat: String,
    /// Source stats and their weights
    pub source_stats: HashMap<String, f64>,
    /// Formula type
    pub formula_type: FormulaType,
    /// Element type (if element-specific)
    pub element_type: Option<String>,
    /// Validation rules
    pub validation_rules: Vec<ValidationRule>,
}

/// Formula types
/// 公式类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FormulaType {
    Linear,        // Tuyến tính (线性)
    Exponential,   // Mũ (指数)
    Logarithmic,   // Logarit (对数)
    Polynomial,    // Đa thức (多项式)
    Custom,        // Tùy chỉnh (自定义)
}

/// Validation rule
/// 验证规则
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationRule {
    /// Rule type
    pub rule_type: ValidationRuleType,
    /// Rule parameters
    pub parameters: HashMap<String, serde_json::Value>,
}

/// Validation rule types
/// 验证规则类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ValidationRuleType {
    MinValue,      // Giá trị tối thiểu (最小值)
    MaxValue,      // Giá trị tối đa (最大值)
    Range,         // Khoảng giá trị (值范围)
    Custom,        // Tùy chỉnh (自定义)
}
```

### **2. External System Implementations**

#### **A. RPG System Implementation**
```rust
/// RPG System Element Contributor implementation
/// RPG系统元素贡献者实现
pub struct RPGElementContributor {
    system_id: String,
    priority: i64,
}

impl RPGElementContributor {
    pub fn new() -> Self {
        Self {
            system_id: "rpg".to_string(),
            priority: 100,
        }
    }
}

#[async_trait]
impl ElementContributor for RPGElementContributor {
    fn system_id(&self) -> &str {
        &self.system_id
    }
    
    fn priority(&self) -> i64 {
        self.priority
    }
    
    async fn contribute_to_element_stats(
        &self,
        actor: &Actor,
        element_type: &str,
    ) -> ElementCoreResult<ElementContribution> {
        // Get RPG primary stats from actor
        let intelligence = actor.get_data()
            .get("intelligence")
            .and_then(|v| v.as_f64())
            .unwrap_or(0.0);
        
        // RPG System tự định nghĩa công thức: intelligence × 2 cho fire power
        let fire_power = intelligence * 2.0;
        
        Ok(ElementContribution {
            stat_name: "fire_power_point".to_string(),
            value: fire_power,
            contribution_type: ContributionType::Additive,
            source_system: self.system_id.clone(),
            priority: self.priority,
            element_type: element_type.to_string(),
            metadata: HashMap::new(),
        })
    }
    
    fn validate_output(&self, output: &ElementContribution) -> ElementCoreResult<()> {
        if output.value.is_nan() || output.value.is_infinite() {
            return Err(ElementCoreError::Validation(format!(
                "Invalid RPG contribution value: {}", output.value
            )));
        }
        Ok(())
    }
}
```

#### **B. Cultivation System Implementation**
```rust
/// Cultivation System Element Contributor implementation
/// 修炼系统元素贡献者实现
pub struct CultivationElementContributor {
    system_id: String,
    priority: i64,
}

impl CultivationElementContributor {
    pub fn new() -> Self {
        Self {
            system_id: "cultivation".to_string(),
            priority: 200,
        }
    }
}

#[async_trait]
impl ElementContributor for CultivationElementContributor {
    fn system_id(&self) -> &str {
        &self.system_id
    }
    
    fn priority(&self) -> i64 {
        self.priority
    }
    
    async fn contribute_to_element_stats(
        &self,
        actor: &Actor,
        element_type: &str,
    ) -> ElementCoreResult<ElementContribution> {
        // Get cultivation stats from actor
        let qi = actor.get_data()
            .get("qi_amount")
            .and_then(|v| v.as_f64())
            .unwrap_or(0.0);
        
        // Cultivation System tự định nghĩa công thức: qi × 3 cho fire power
        let fire_power = qi * 3.0;
        
        Ok(ElementContribution {
            stat_name: "fire_power_point".to_string(),
            value: fire_power,
            contribution_type: ContributionType::Additive,
            source_system: self.system_id.clone(),
            priority: self.priority,
            element_type: element_type.to_string(),
            metadata: HashMap::new(),
        })
    }
    
    fn validate_output(&self, output: &ElementContribution) -> ElementCoreResult<()> {
        if output.value < 0.0 {
            return Err(ElementCoreError::Validation(format!(
                "Cultivation contribution cannot be negative: {}", output.value
            )));
        }
        Ok(())
    }
}
```

#### **C. Item System Implementation**
```rust
/// Item System Element Contributor implementation
/// 物品系统元素贡献者实现
pub struct ItemElementContributor {
    system_id: String,
    priority: i64,
}

impl ItemElementContributor {
    pub fn new() -> Self {
        Self {
            system_id: "items".to_string(),
            priority: 300,
        }
    }
}

#[async_trait]
impl ElementContributor for ItemElementContributor {
    fn system_id(&self) -> &str {
        &self.system_id
    }
    
    fn priority(&self) -> i64 {
        self.priority
    }
    
    async fn contribute_to_element_stats(
        &self,
        actor: &Actor,
        element_type: &str,
    ) -> ElementCoreResult<ElementContribution> {
        // Get item bonus from actor equipment
        let fire_sword_bonus = actor.get_data()
            .get("fire_sword_bonus")
            .and_then(|v| v.as_f64())
            .unwrap_or(0.0);
        
        // Item System cộng trực tiếp: +1000 từ fire sword
        let fire_power = fire_sword_bonus; // +1000
        
        Ok(ElementContribution {
            stat_name: "fire_power_point".to_string(),
            value: fire_power,
            contribution_type: ContributionType::Additive,
            source_system: self.system_id.clone(),
            priority: self.priority,
            element_type: element_type.to_string(),
            metadata: HashMap::new(),
        })
    }
    
    fn validate_output(&self, output: &ElementContribution) -> ElementCoreResult<()> {
        if output.value < 0.0 {
            return Err(ElementCoreError::Validation(format!(
                "Item contribution cannot be negative: {}", output.value
            )));
        }
        Ok(())
    }
}
```

#### **D. Talent System Implementation**
```rust
/// Talent System Element Contributor implementation
/// 天赋系统元素贡献者实现
pub struct TalentElementContributor {
    system_id: String,
    priority: i64,
}

impl TalentElementContributor {
    pub fn new() -> Self {
        Self {
            system_id: "talents".to_string(),
            priority: 400,
        }
    }
}

#[async_trait]
impl ElementContributor for TalentElementContributor {
    fn system_id(&self) -> &str {
        &self.system_id
    }
    
    fn priority(&self) -> i64 {
        self.priority
    }
    
    async fn contribute_to_element_stats(
        &self,
        actor: &Actor,
        element_type: &str,
    ) -> ElementCoreResult<ElementContribution> {
        // Get talent bonus from actor
        let talent_bonus = actor.get_data()
            .get("talent_A_bonus")
            .and_then(|v| v.as_f64())
            .unwrap_or(0.0);
        
        // Talent System cộng trực tiếp: +200 từ talent_A
        let fire_power = talent_bonus; // +200
        
        Ok(ElementContribution {
            stat_name: "fire_power_point".to_string(),
            value: fire_power,
            contribution_type: ContributionType::Additive,
            source_system: self.system_id.clone(),
            priority: self.priority,
            element_type: element_type.to_string(),
            metadata: HashMap::new(),
        })
    }
    
    fn validate_output(&self, output: &ElementContribution) -> ElementCoreResult<()> {
        if output.value < 0.0 {
            return Err(ElementCoreError::Validation(format!(
                "Talent contribution cannot be negative: {}", output.value
            )));
        }
        Ok(())
    }
}
```

#### **E. Destiny System Implementation**
```rust
/// Destiny System Element Contributor implementation
/// 命运系统元素贡献者实现
pub struct DestinyElementContributor {
    system_id: String,
    priority: i64,
}

impl DestinyElementContributor {
    pub fn new() -> Self {
        Self {
            system_id: "destiny".to_string(),
            priority: 500,
        }
    }
}

#[async_trait]
impl ElementContributor for DestinyElementContributor {
    fn system_id(&self) -> &str {
        &self.system_id
    }
    
    fn priority(&self) -> i64 {
        self.priority
    }
    
    async fn contribute_to_element_stats(
        &self,
        actor: &Actor,
        element_type: &str,
    ) -> ElementCoreResult<ElementContribution> {
        // Get destiny bonus from actor
        let destiny_bonus = actor.get_data()
            .get("destiny_A_bonus")
            .and_then(|v| v.as_f64())
            .unwrap_or(0.0);
        
        // Destiny System cộng trực tiếp: +300 từ destiny_A
        let fire_power = destiny_bonus; // +300
        
        Ok(ElementContribution {
            stat_name: "fire_power_point".to_string(),
            value: fire_power,
            contribution_type: ContributionType::Additive,
            source_system: self.system_id.clone(),
            priority: self.priority,
            element_type: element_type.to_string(),
            metadata: HashMap::new(),
        })
    }
    
    fn validate_output(&self, output: &ElementContribution) -> ElementCoreResult<()> {
        if output.value < 0.0 {
            return Err(ElementCoreError::Validation(format!(
                "Destiny contribution cannot be negative: {}", output.value
            )));
        }
        Ok(())
    }
}
```

### **3. Element-Core Data Hub Implementation**

#### **A. Element-Core Registry Implementation**
```rust
/// Element-Core Registry implementation for managing external contributors
/// Element-Core注册表实现，用于管理外部贡献者
pub struct ElementContributorRegistryImpl {
    /// Map of system ID to contributor
    contributors: Arc<RwLock<HashMap<String, Arc<dyn ElementContributor>>>>,
    /// Cache for performance
    cache: Arc<RwLock<HashMap<String, ElementDerivedStats>>>,
    /// Metrics for monitoring
    metrics: Arc<RwLock<RegistryMetrics>>,
}

impl ElementContributorRegistryImpl {
    pub fn new() -> Self {
        Self {
            contributors: Arc::new(RwLock::new(HashMap::new())),
            cache: Arc::new(RwLock::new(HashMap::new())),
            metrics: Arc::new(RwLock::new(RegistryMetrics::default())),
        }
    }
}

impl ElementContributorRegistry for ElementContributorRegistryImpl {
    fn register(&self, contributor: Arc<dyn ElementContributor>) -> ElementCoreResult<()> {
        let system_id = contributor.system_id().to_string();
        
        if system_id.is_empty() {
            return Err(ElementCoreError::Validation("System ID cannot be empty".to_string()));
        }

        let mut contributors = self.contributors.write();
        
        if contributors.contains_key(&system_id) {
            warn!("Overwriting existing contributor: {}", system_id);
        }
        
        contributors.insert(system_id.clone(), contributor);
        
        info!("Registered contributor: {}", system_id);
        Ok(())
    }
    
    fn unregister(&self, system_id: &str) -> ElementCoreResult<()> {
        let mut contributors = self.contributors.write();
        
        if contributors.remove(system_id).is_some() {
            info!("Unregistered contributor: {}", system_id);
            Ok(())
        } else {
            Err(ElementCoreError::Validation(format!("Contributor not found: {}", system_id)))
        }
    }
    
    fn get_by_id(&self, system_id: &str) -> Option<Arc<dyn ElementContributor>> {
        let contributors = self.contributors.read();
        contributors.get(system_id).cloned()
    }
    
    fn get_by_priority(&self) -> Vec<Arc<dyn ElementContributor>> {
        let contributors = self.contributors.read();
        let mut contributor_list: Vec<Arc<dyn ElementContributor>> = contributors.values().cloned().collect();
        
        // Sort by priority (higher priority first)
        contributor_list.sort_by(|a, b| b.priority().cmp(&a.priority()));
        contributor_list
    }
    
    fn is_registered(&self, system_id: &str) -> bool {
        let contributors = self.contributors.read();
        contributors.contains_key(system_id)
    }
    
    fn count(&self) -> usize {
        let contributors = self.contributors.read();
        contributors.len()
    }
}
```

#### **B. Element-Core Aggregator Implementation**
```rust
/// Element-Core Aggregator for combining external contributions
/// Element-Core聚合器，用于组合外部贡献
pub struct ElementCoreAggregator {
    /// Registry for managing contributors
    registry: Arc<ElementContributorRegistryImpl>,
    /// Cache for performance
    cache: Arc<RwLock<HashMap<String, ElementDerivedStats>>>,
    /// Configuration
    config: ElementAggregatorConfig,
}

impl ElementCoreAggregator {
    pub fn new(registry: Arc<ElementContributorRegistryImpl>) -> Self {
        Self {
            registry,
            cache: Arc::new(RwLock::new(HashMap::new())),
            config: ElementAggregatorConfig::default(),
        }
    }
    
    /// Calculate final derived stats for an actor and element
    pub async fn calculate_derived_stats(
        &self,
        actor: &Actor,
        element_type: &str,
    ) -> ElementCoreResult<ElementDerivedStats> {
        // Check cache first
        let cache_key = format!("{}_{}", actor.id, element_type);
        if let Some(cached_stats) = self.cache.read().get(&cache_key) {
            return Ok(cached_stats.clone());
        }
        
        // Get all registered contributors
        let contributors = self.registry.get_by_priority();
        
        // Collect contributions from all contributors
        let mut all_contributions = Vec::new();
        
        for contributor in &contributors {
            match contributor.contribute_to_element_stats(actor, element_type).await {
                Ok(contribution) => {
                    // Validate contribution
                    if let Err(e) = contributor.validate_output(&contribution) {
                        warn!("Contributor {} validation failed: {}", contributor.system_id(), e);
                        continue;
                    }
                    
                    all_contributions.push(contribution);
                }
                Err(e) => {
                    warn!("Contributor {} failed to contribute: {}", contributor.system_id(), e);
                    continue;
                }
            }
        }
        
        // Aggregate contributions
        let aggregated_stats = self.aggregate_contributions(all_contributions).await?;
        
        // Cache result
        self.cache.write().insert(cache_key, aggregated_stats.clone());
        
        Ok(aggregated_stats)
    }
    
    /// Aggregate contributions using rules
    async fn aggregate_contributions(
        &self,
        contributions: Vec<ElementContribution>,
    ) -> ElementCoreResult<ElementDerivedStats> {
        let mut aggregated_stats = HashMap::new();
        
        // Group contributions by stat name
        let mut grouped_contributions: HashMap<String, Vec<ElementContribution>> = HashMap::new();
        for contribution in contributions {
            grouped_contributions
                .entry(contribution.stat_name.clone())
                .or_insert_with(Vec::new)
                .push(contribution);
        }
        
        // Apply aggregation rule to each stat
        for (stat_name, stat_contributions) in grouped_contributions {
            let aggregation_rule = self.get_aggregation_rule(&stat_name);
            let final_value = self.apply_aggregation_rule(&aggregation_rule, &stat_contributions)?;
            aggregated_stats.insert(stat_name, final_value);
        }
        
        Ok(ElementDerivedStats::from_hashmap(aggregated_stats))
    }
    
    /// Apply aggregation rule to contributions
    fn apply_aggregation_rule(
        &self,
        rule: &AggregationRule,
        contributions: &[ElementContribution],
    ) -> ElementCoreResult<f64> {
        if contributions.is_empty() {
            return Ok(0.0);
        }
        
        match rule {
            AggregationRule::Additive => {
                let mut total = 0.0;
                for contribution in contributions {
                    total += contribution.value;
                }
                Ok(total)
            }
            AggregationRule::Multiplicative => {
                let mut total = 1.0;
                for contribution in contributions {
                    total *= contribution.value;
                }
                Ok(total)
            }
            AggregationRule::Override => {
                // Use highest priority contribution
                let highest_priority = contributions
                    .iter()
                    .max_by_key(|c| c.priority)
                    .unwrap();
                Ok(highest_priority.value)
            }
            AggregationRule::Maximum => {
                let max_value = contributions
                    .iter()
                    .map(|c| c.value)
                    .fold(f64::NEG_INFINITY, f64::max);
                Ok(max_value)
            }
            AggregationRule::Minimum => {
                let min_value = contributions
                    .iter()
                    .map(|c| c.value)
                    .fold(f64::INFINITY, f64::min);
                Ok(min_value)
            }
        }
    }
    
    /// Get aggregation rule for stat
    fn get_aggregation_rule(&self, stat_name: &str) -> AggregationRule {
        // Default aggregation rules
        match stat_name {
            "fire_power_point" | "water_power_point" | "earth_power_point" |
            "metal_power_point" | "wood_power_point" | "ice_power_point" |
            "lightning_power_point" | "air_power_point" => AggregationRule::Additive,
            
            "element_mastery" | "element_control" | "element_harmony" => AggregationRule::Override,
            
            _ => AggregationRule::Additive,
        }
    }
}
```

## 🔧 **Usage Example**

### **1. System Registration**
```rust
// Create Element-Core registry
let registry = Arc::new(ElementContributorRegistryImpl::new());

// Create Element-Core aggregator
let aggregator = ElementCoreAggregator::new(registry.clone());

// Register external systems
registry.register(Arc::new(RPGElementContributor::new()));
registry.register(Arc::new(CultivationElementContributor::new()));
registry.register(Arc::new(ItemElementContributor::new()));
registry.register(Arc::new(TalentElementContributor::new()));
registry.register(Arc::new(DestinyElementContributor::new()));
```

### **2. Calculate Fire Power Point**
```rust
// Calculate fire power point for an actor
let fire_stats = aggregator.calculate_derived_stats(&actor, "fire").await?;

// Result: Total Fire Power Point = 
// RPG: intelligence × 2 = 100 × 2 = 200
// Cultivation: qi × 3 = 50 × 3 = 150  
// Items: fire_sword_bonus = 1000
// Talents: talent_A_bonus = 200
// Destiny: destiny_A_bonus = 300
// Total = 200 + 150 + 1000 + 200 + 300 = 1850
```

## 🎯 **Kết Luận**

### **1. Thiết Kế Nhất Quán với Actor-Core**
- **External Registration**: Các external systems tự register với Element-Core
- **Data Hub Pattern**: Element-Core chỉ làm cache + aggregation
- **No Code Changes**: Không cần sửa Element-Core khi thêm system mới
- **Decoupled Architecture**: Mỗi system tự quản lý logic của mình

### **2. Lợi Ích của Thiết Kế**
- **Extensible**: Dễ dàng thêm Talent System, Destiny System, etc.
- **Maintainable**: Mỗi system có trách nhiệm riêng biệt
- **Testable**: Có thể test từng system độc lập
- **Performance**: Caching và batch processing
- **Flexible**: Mỗi system có thể định nghĩa formulas riêng

### **3. Ví Dụ Tính Toán**
```rust
// Total Fire Power Point = 
// RPG: intelligence × 2 = 100 × 2 = 200
// Cultivation: qi × 3 = 50 × 3 = 150  
// Items: fire_sword_bonus = 1000
// Talents: talent_A_bonus = 200
// Destiny: destiny_A_bonus = 300
// Total = 200 + 150 + 1000 + 200 + 300 = 1850
```

### **4. Tương Lai**
- **More Systems**: Có thể thêm Skill System, Pet System, etc.
- **Advanced Rules**: Có thể thêm conditional contributions
- **Performance**: Có thể optimize caching và batch processing
- **Monitoring**: Có thể thêm metrics và monitoring

## 📚 **Related Documents**

- [00_Element_Core_Overview.md](./00_Element_Core_Overview.md) - Element Core overview
- [02_Multi_System_Integration_Design.md](./02_Multi_System_Integration_Design.md) - Multi-system integration
- [11_Advanced_Derived_Stats_Design.md](./11_Advanced_Derived_Stats_Design.md) - Derived stats system
- [16_Hybrid_Subsystem_Design.md](./16_Hybrid_Subsystem_Design.md) - Hybrid elements system
- [17_Elemental_Category_System_Design.md](./17_Elemental_Category_System_Design.md) - Element categorization
- [18_Universal_Element_Registry_Design.md](./18_Universal_Element_Registry_Design.md) - Universal element registry

---

**Last Updated**: 2024-12-19  
**Version**: 2.0  
**Status**: Updated to match Actor-Core pattern  
**Maintainer**: Chaos World Team
