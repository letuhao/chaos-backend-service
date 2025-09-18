# Condition Core Architecture Design

## 📋 **Tổng Quan**

Tài liệu này thiết kế kiến trúc chi tiết cho Condition Core, dựa trên Skyrim's Condition Functions system và các best practices từ game industry.

## 🏗️ **Core Architecture**

### **1. Condition Core Components**

```rust
// Condition Core Main Structure
pub struct ConditionCore {
    // Core components
    condition_registry: ConditionRegistry,
    condition_engine: ConditionEngine,
    condition_cache: ConditionCache,
    condition_validator: ConditionValidator,
    
    // Configuration
    config_manager: ConditionConfigManager,
    
    // Performance
    performance_monitor: PerformanceMonitor,
    
    // Integration
    integration_bridges: IntegrationBridges,
}

// Condition Registry
pub struct ConditionRegistry {
    condition_functions: HashMap<String, Box<dyn ConditionFunction>>,
    condition_templates: HashMap<String, ConditionTemplate>,
    condition_categories: HashMap<String, ConditionCategory>,
    condition_metadata: HashMap<String, ConditionMetadata>,
}

// Condition Engine
pub struct ConditionEngine {
    condition_evaluator: ConditionEvaluator,
    condition_parser: ConditionParser,
    condition_optimizer: ConditionOptimizer,
    condition_scheduler: ConditionScheduler,
}
```

### **2. Condition Definition Structure**

```rust
// Condition Definition (Skyrim-inspired)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConditionDefinition {
    // Basic Info (Skyrim-inspired)
    pub condition_guid: String,           // GUID for uniqueness
    pub condition_id: String,             // Editor ID like Skyrim
    pub condition_name: String,           // Display name
    pub condition_name_vi: String,        // Vietnamese name
    pub world_id: String,                 // World identifier
    
    // Condition Properties
    pub condition_function: String,       // Function name
    pub condition_parameters: Vec<ConditionParameter>, // Function parameters
    pub condition_operator: ConditionOperator, // Comparison operator
    pub condition_value: ConditionValue,  // Target value
    pub condition_logic: ConditionLogic,  // Logic operator (AND, OR, etc.)
    
    // Categories
    pub categories: Vec<String>,          // Condition categories
    
    // Metadata
    pub priority: u32,                    // Evaluation priority
    pub cacheable: bool,                  // Can be cached
    pub cache_ttl: Option<Duration>,      // Cache TTL
    pub performance_impact: PerformanceImpact, // Performance impact
    
    // Timestamps
    pub created_at: SystemTime,
    pub updated_at: SystemTime,
}

// Condition Parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConditionParameter {
    String(String),
    Integer(i64),
    Float(f64),
    Boolean(bool),
    Vector3(Vector3),
    Color(Color),
    Time(Time),
    Date(Date),
    Custom(serde_json::Value),
}

// Condition Operators
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConditionOperator {
    Equal,                             // ==
    NotEqual,                          // !=
    GreaterThan,                       // >
    GreaterThanOrEqual,                // >=
    LessThan,                          // <
    LessThanOrEqual,                   // <=
    Contains,                          // Contains
    NotContains,                       // Not contains
    StartsWith,                        // Starts with
    EndsWith,                          // Ends with
    Regex,                             // Regex match
    In,                                // In list
    NotIn,                             // Not in list
    Between,                           // Between values
    NotBetween,                        // Not between values
}

// Condition Logic
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConditionLogic {
    And,                               // AND
    Or,                                // OR
    Not,                               // NOT
    Xor,                               // XOR
    Nand,                              // NAND
    Nor,                               // NOR
}

// Condition Value
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConditionValue {
    Boolean(bool),
    Integer(i64),
    Float(f64),
    String(String),
    Vector3(Vector3),
    Color(Color),
    Time(Time),
    Date(Date),
    List(Vec<ConditionValue>),
    Custom(serde_json::Value),
}
```

### **3. Condition Function Registry**

```rust
// Condition Function Registry
pub struct ConditionFunctionRegistry {
    functions: HashMap<String, Box<dyn ConditionFunction>>,
    function_categories: HashMap<String, Vec<String>>,
    function_metadata: HashMap<String, FunctionMetadata>,
    function_cache: HashMap<String, CachedFunction>,
}

// Condition Function Trait
pub trait ConditionFunction: Send + Sync {
    async fn evaluate(
        &self,
        parameters: &[ConditionParameter],
        context: &ConditionContext
    ) -> Result<ConditionValue, ConditionError>;
    
    fn get_parameter_types(&self) -> Vec<ParameterType>;
    fn get_return_type(&self) -> ReturnType;
    fn get_description(&self) -> String;
    fn get_description_vi(&self) -> String;
    fn is_cacheable(&self) -> bool;
    fn get_cache_ttl(&self) -> Option<Duration>;
    fn get_performance_impact(&self) -> PerformanceImpact;
}

// Function Metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionMetadata {
    pub function_id: String,
    pub function_name: String,
    pub function_name_vi: String,
    pub category: String,
    pub description: String,
    pub description_vi: String,
    pub parameter_types: Vec<ParameterType>,
    pub return_type: ReturnType,
    pub is_async: bool,
    pub cacheable: bool,
    pub cache_ttl: Option<Duration>,
    pub performance_impact: PerformanceImpact,
    pub version: String,
    pub author: String,
    pub created_at: SystemTime,
    pub updated_at: SystemTime,
}
```

## 🔧 **Condition Functions Implementation**

### **1. Actor Condition Functions**

```rust
// Actor Condition Functions
pub struct GetActorValueFunction;

impl ConditionFunction for GetActorValueFunction {
    async fn evaluate(
        &self,
        parameters: &[ConditionParameter],
        context: &ConditionContext
    ) -> Result<ConditionValue, ConditionError> {
        let stat_name = parameters.get(0)
            .ok_or(ConditionError::MissingParameter("stat_name"))?
            .as_string()?;
        
        let actor_id = context.target.id.clone();
        let actor_value = context.actor_manager
            .get_actor_value(&actor_id, &stat_name)
            .await?;
        
        Ok(ConditionValue::Float(actor_value))
    }
    
    fn get_parameter_types(&self) -> Vec<ParameterType> {
        vec![ParameterType::String] // stat_name
    }
    
    fn get_return_type(&self) -> ReturnType {
        ReturnType::Float
    }
    
    fn get_description(&self) -> String {
        "Get actor's stat value".to_string()
    }
    
    fn get_description_vi(&self) -> String {
        "Lấy giá trị thống kê của diễn viên".to_string()
    }
    
    fn is_cacheable(&self) -> bool {
        true
    }
    
    fn get_cache_ttl(&self) -> Option<Duration> {
        Some(Duration::from_secs(30))
    }
    
    fn get_performance_impact(&self) -> PerformanceImpact {
        PerformanceImpact::Low
    }
}

pub struct IsInCombatFunction;

impl ConditionFunction for IsInCombatFunction {
    async fn evaluate(
        &self,
        parameters: &[ConditionParameter],
        context: &ConditionContext
    ) -> Result<ConditionValue, ConditionError> {
        let actor_id = context.target.id.clone();
        let is_in_combat = context.combat_manager
            .is_actor_in_combat(&actor_id)
            .await?;
        
        Ok(ConditionValue::Boolean(is_in_combat))
    }
    
    fn get_parameter_types(&self) -> Vec<ParameterType> {
        vec![] // No parameters
    }
    
    fn get_return_type(&self) -> ReturnType {
        ReturnType::Boolean
    }
    
    fn get_description(&self) -> String {
        "Check if actor is in combat".to_string()
    }
    
    fn get_description_vi(&self) -> String {
        "Kiểm tra diễn viên có đang trong chiến đấu".to_string()
    }
    
    fn is_cacheable(&self) -> bool {
        true
    }
    
    fn get_cache_ttl(&self) -> Option<Duration> {
        Some(Duration::from_secs(10))
    }
    
    fn get_performance_impact(&self) -> PerformanceImpact {
        PerformanceImpact::Low
    }
}

pub struct GetLevelFunction;

impl ConditionFunction for GetLevelFunction {
    async fn evaluate(
        &self,
        parameters: &[ConditionParameter],
        context: &ConditionContext
    ) -> Result<ConditionValue, ConditionError> {
        let actor_id = context.target.id.clone();
        let level = context.actor_manager
            .get_actor_level(&actor_id)
            .await?;
        
        Ok(ConditionValue::Integer(level as i64))
    }
    
    fn get_parameter_types(&self) -> Vec<ParameterType> {
        vec![] // No parameters
    }
    
    fn get_return_type(&self) -> ReturnType {
        ReturnType::Integer
    }
    
    fn get_description(&self) -> String {
        "Get actor's level".to_string()
    }
    
    fn get_description_vi(&self) -> String {
        "Lấy cấp độ của diễn viên".to_string()
    }
    
    fn is_cacheable(&self) -> bool {
        true
    }
    
    fn get_cache_ttl(&self) -> Option<Duration> {
        Some(Duration::from_secs(300)) // 5 minutes
    }
    
    fn get_performance_impact(&self) -> PerformanceImpact {
        PerformanceImpact::Low
    }
}
```

### **2. Item Condition Functions**

```rust
// Item Condition Functions
pub struct HasItemFunction;

impl ConditionFunction for HasItemFunction {
    async fn evaluate(
        &self,
        parameters: &[ConditionParameter],
        context: &ConditionContext
    ) -> Result<ConditionValue, ConditionError> {
        let item_id = parameters.get(0)
            .ok_or(ConditionError::MissingParameter("item_id"))?
            .as_string()?;
        
        let actor_id = context.target.id.clone();
        let has_item = context.inventory_manager
            .has_item(&actor_id, &item_id)
            .await?;
        
        Ok(ConditionValue::Boolean(has_item))
    }
    
    fn get_parameter_types(&self) -> Vec<ParameterType> {
        vec![ParameterType::String] // item_id
    }
    
    fn get_return_type(&self) -> ReturnType {
        ReturnType::Boolean
    }
    
    fn get_description(&self) -> String {
        "Check if actor has item".to_string()
    }
    
    fn get_description_vi(&self) -> String {
        "Kiểm tra diễn viên có vật phẩm".to_string()
    }
    
    fn is_cacheable(&self) -> bool {
        true
    }
    
    fn get_cache_ttl(&self) -> Option<Duration> {
        Some(Duration::from_secs(60))
    }
    
    fn get_performance_impact(&self) -> PerformanceImpact {
        PerformanceImpact::Low
    }
}

pub struct GetItemCountFunction;

impl ConditionFunction for GetItemCountFunction {
    async fn evaluate(
        &self,
        parameters: &[ConditionParameter],
        context: &ConditionContext
    ) -> Result<ConditionValue, ConditionError> {
        let item_id = parameters.get(0)
            .ok_or(ConditionError::MissingParameter("item_id"))?
            .as_string()?;
        
        let actor_id = context.target.id.clone();
        let count = context.inventory_manager
            .get_item_count(&actor_id, &item_id)
            .await?;
        
        Ok(ConditionValue::Integer(count as i64))
    }
    
    fn get_parameter_types(&self) -> Vec<ParameterType> {
        vec![ParameterType::String] // item_id
    }
    
    fn get_return_type(&self) -> ReturnType {
        ReturnType::Integer
    }
    
    fn get_description(&self) -> String {
        "Get item count".to_string()
    }
    
    fn get_description_vi(&self) -> String {
        "Lấy số lượng vật phẩm".to_string()
    }
    
    fn is_cacheable(&self) -> bool {
        true
    }
    
    fn get_cache_ttl(&self) -> Option<Duration> {
        Some(Duration::from_secs(60))
    }
    
    fn get_performance_impact(&self) -> PerformanceImpact {
        PerformanceImpact::Low
    }
}

pub struct IsEquippedFunction;

impl ConditionFunction for IsEquippedFunction {
    async fn evaluate(
        &self,
        parameters: &[ConditionParameter],
        context: &ConditionContext
    ) -> Result<ConditionValue, ConditionError> {
        let item_id = parameters.get(0)
            .ok_or(ConditionError::MissingParameter("item_id"))?
            .as_string()?;
        
        let actor_id = context.target.id.clone();
        let is_equipped = context.equipment_manager
            .is_item_equipped(&actor_id, &item_id)
            .await?;
        
        Ok(ConditionValue::Boolean(is_equipped))
    }
    
    fn get_parameter_types(&self) -> Vec<ParameterType> {
        vec![ParameterType::String] // item_id
    }
    
    fn get_return_type(&self) -> ReturnType {
        ReturnType::Boolean
    }
    
    fn get_description(&self) -> String {
        "Check if item is equipped".to_string()
    }
    
    fn get_description_vi(&self) -> String {
        "Kiểm tra vật phẩm có được trang bị".to_string()
    }
    
    fn is_cacheable(&self) -> bool {
        true
    }
    
    fn get_cache_ttl(&self) -> Option<Duration> {
        Some(Duration::from_secs(30))
    }
    
    fn get_performance_impact(&self) -> PerformanceImpact {
        PerformanceImpact::Low
    }
}
```

### **3. Location Condition Functions**

```rust
// Location Condition Functions
pub struct GetInCurrentLocationFunction;

impl ConditionFunction for GetInCurrentLocationFunction {
    async fn evaluate(
        &self,
        parameters: &[ConditionParameter],
        context: &ConditionContext
    ) -> Result<ConditionValue, ConditionError> {
        let location_id = parameters.get(0)
            .ok_or(ConditionError::MissingParameter("location_id"))?
            .as_string()?;
        
        let actor_id = context.target.id.clone();
        let current_location = context.location_manager
            .get_actor_location(&actor_id)
            .await?;
        
        let is_in_location = current_location.id == location_id;
        Ok(ConditionValue::Boolean(is_in_location))
    }
    
    fn get_parameter_types(&self) -> Vec<ParameterType> {
        vec![ParameterType::String] // location_id
    }
    
    fn get_return_type(&self) -> ReturnType {
        ReturnType::Boolean
    }
    
    fn get_description(&self) -> String {
        "Check if actor is in location".to_string()
    }
    
    fn get_description_vi(&self) -> String {
        "Kiểm tra diễn viên có trong vị trí".to_string()
    }
    
    fn is_cacheable(&self) -> bool {
        true
    }
    
    fn get_cache_ttl(&self) -> Option<Duration> {
        Some(Duration::from_secs(60))
    }
    
    fn get_performance_impact(&self) -> PerformanceImpact {
        PerformanceImpact::Low
    }
}

pub struct IsInInteriorFunction;

impl ConditionFunction for IsInInteriorFunction {
    async fn evaluate(
        &self,
        parameters: &[ConditionParameter],
        context: &ConditionContext
    ) -> Result<ConditionValue, ConditionError> {
        let actor_id = context.target.id.clone();
        let current_location = context.location_manager
            .get_actor_location(&actor_id)
            .await?;
        
        let is_interior = current_location.location_type == LocationType::Interior;
        Ok(ConditionValue::Boolean(is_interior))
    }
    
    fn get_parameter_types(&self) -> Vec<ParameterType> {
        vec![] // No parameters
    }
    
    fn get_return_type(&self) -> ReturnType {
        ReturnType::Boolean
    }
    
    fn get_description(&self) -> String {
        "Check if actor is in interior".to_string()
    }
    
    fn get_description_vi(&self) -> String {
        "Kiểm tra diễn viên có trong nội thất".to_string()
    }
    
    fn is_cacheable(&self) -> bool {
        true
    }
    
    fn get_cache_ttl(&self) -> Option<Duration> {
        Some(Duration::from_secs(60))
    }
    
    fn get_performance_impact(&self) -> PerformanceImpact {
        PerformanceImpact::Low
    }
}

pub struct GetDistanceFromPlayerFunction;

impl ConditionFunction for GetDistanceFromPlayerFunction {
    async fn evaluate(
        &self,
        parameters: &[ConditionParameter],
        context: &ConditionContext
    ) -> Result<ConditionValue, ConditionError> {
        let actor_id = context.target.id.clone();
        let player_id = context.player_id.clone();
        
        let distance = context.location_manager
            .get_distance_between_actors(&actor_id, &player_id)
            .await?;
        
        Ok(ConditionValue::Float(distance))
    }
    
    fn get_parameter_types(&self) -> Vec<ParameterType> {
        vec![] // No parameters
    }
    
    fn get_return_type(&self) -> ReturnType {
        ReturnType::Float
    }
    
    fn get_description(&self) -> String {
        "Get distance from player".to_string()
    }
    
    fn get_description_vi(&self) -> String {
        "Lấy khoảng cách từ người chơi".to_string()
    }
    
    fn is_cacheable(&self) -> bool {
        true
    }
    
    fn get_cache_ttl(&self) -> Option<Duration> {
        Some(Duration::from_secs(10))
    }
    
    fn get_performance_impact(&self) -> PerformanceImpact {
        PerformanceImpact::Medium
    }
}
```

## 🔧 **Condition Evaluation Engine**

### **1. Condition Evaluator**

```rust
// Condition Evaluator
pub struct ConditionEvaluator {
    function_registry: Arc<ConditionFunctionRegistry>,
    condition_cache: Arc<ConditionCache>,
    evaluation_strategies: HashMap<String, EvaluationStrategy>,
    performance_monitor: Arc<PerformanceMonitor>,
}

impl ConditionEvaluator {
    // Evaluate single condition
    pub async fn evaluate_condition(
        &self,
        condition: &ConditionDefinition,
        context: &ConditionContext
    ) -> Result<ConditionResult, ConditionError> {
        let start_time = SystemTime::now();
        
        // Check cache first
        let cache_key = self.generate_cache_key(condition, context);
        if let Some(cached_result) = self.condition_cache.get(&cache_key) {
            self.performance_monitor.record_cache_hit(&condition.condition_id);
            return Ok(cached_result.clone());
        }
        
        // Get condition function
        let function = self.function_registry
            .get_function(&condition.condition_function)
            .ok_or(ConditionError::FunctionNotFound(condition.condition_function.clone()))?;
        
        // Evaluate condition
        let condition_value = function.evaluate(&condition.condition_parameters, context).await?;
        
        // Apply operator
        let result = self.apply_operator(
            &condition_value,
            &condition.condition_operator,
            &condition.condition_value
        )?;
        
        // Create result
        let condition_result = ConditionResult {
            condition_id: condition.condition_id.clone(),
            passed: result,
            value: condition_value,
            evaluated_at: SystemTime::now(),
            evaluation_time: start_time.elapsed().unwrap_or_default(),
        };
        
        // Cache result if cacheable
        if condition.cacheable {
            self.condition_cache.cache(cache_key, condition_result.clone());
        }
        
        // Record performance metrics
        self.performance_monitor.record_evaluation(&condition.condition_id, start_time.elapsed().unwrap_or_default());
        
        Ok(condition_result)
    }
    
    // Evaluate condition chain
    pub async fn evaluate_condition_chain(
        &self,
        conditions: &[ConditionDefinition],
        context: &ConditionContext
    ) -> Result<ConditionChainResult, ConditionError> {
        let start_time = SystemTime::now();
        let mut results = Vec::new();
        
        // Sort conditions by priority
        let mut sorted_conditions = conditions.to_vec();
        sorted_conditions.sort_by_key(|c| c.priority);
        
        for condition in sorted_conditions {
            let result = self.evaluate_condition(&condition, context).await?;
            results.push(result);
            
            // Early exit if condition fails and logic is AND
            if !result.passed && condition.condition_logic == ConditionLogic::And {
                break;
            }
        }
        
        // Apply condition logic
        let final_result = self.apply_condition_logic(&results, &conditions[0].condition_logic)?;
        
        Ok(ConditionChainResult {
            conditions: results,
            final_result,
            evaluated_at: SystemTime::now(),
            evaluation_time: start_time.elapsed().unwrap_or_default(),
        })
    }
    
    // Apply operator
    fn apply_operator(
        &self,
        value: &ConditionValue,
        operator: &ConditionOperator,
        target_value: &ConditionValue
    ) -> Result<bool, ConditionError> {
        match operator {
            ConditionOperator::Equal => Ok(value == target_value),
            ConditionOperator::NotEqual => Ok(value != target_value),
            ConditionOperator::GreaterThan => Ok(value > target_value),
            ConditionOperator::GreaterThanOrEqual => Ok(value >= target_value),
            ConditionOperator::LessThan => Ok(value < target_value),
            ConditionOperator::LessThanOrEqual => Ok(value <= target_value),
            ConditionOperator::Contains => {
                if let (ConditionValue::String(s), ConditionValue::String(t)) = (value, target_value) {
                    Ok(s.contains(t))
                } else {
                    Err(ConditionError::InvalidOperator("Contains".to_string()))
                }
            },
            ConditionOperator::NotContains => {
                if let (ConditionValue::String(s), ConditionValue::String(t)) = (value, target_value) {
                    Ok(!s.contains(t))
                } else {
                    Err(ConditionError::InvalidOperator("NotContains".to_string()))
                }
            },
            ConditionOperator::StartsWith => {
                if let (ConditionValue::String(s), ConditionValue::String(t)) = (value, target_value) {
                    Ok(s.starts_with(t))
                } else {
                    Err(ConditionError::InvalidOperator("StartsWith".to_string()))
                }
            },
            ConditionOperator::EndsWith => {
                if let (ConditionValue::String(s), ConditionValue::String(t)) = (value, target_value) {
                    Ok(s.ends_with(t))
                } else {
                    Err(ConditionError::InvalidOperator("EndsWith".to_string()))
                }
            },
            ConditionOperator::Regex => {
                if let (ConditionValue::String(s), ConditionValue::String(t)) = (value, target_value) {
                    let regex = Regex::new(t)?;
                    Ok(regex.is_match(s))
                } else {
                    Err(ConditionError::InvalidOperator("Regex".to_string()))
                }
            },
            ConditionOperator::In => {
                if let ConditionValue::List(list) = target_value {
                    Ok(list.contains(value))
                } else {
                    Err(ConditionError::InvalidOperator("In".to_string()))
                }
            },
            ConditionOperator::NotIn => {
                if let ConditionValue::List(list) = target_value {
                    Ok(!list.contains(value))
                } else {
                    Err(ConditionError::InvalidOperator("NotIn".to_string()))
                }
            },
            ConditionOperator::Between => {
                if let ConditionValue::List(list) = target_value {
                    if list.len() == 2 {
                        Ok(value >= &list[0] && value <= &list[1])
                    } else {
                        Err(ConditionError::InvalidOperator("Between requires exactly 2 values".to_string()))
                    }
                } else {
                    Err(ConditionError::InvalidOperator("Between".to_string()))
                }
            },
            ConditionOperator::NotBetween => {
                if let ConditionValue::List(list) = target_value {
                    if list.len() == 2 {
                        Ok(value < &list[0] || value > &list[1])
                    } else {
                        Err(ConditionError::InvalidOperator("NotBetween requires exactly 2 values".to_string()))
                    }
                } else {
                    Err(ConditionError::InvalidOperator("NotBetween".to_string()))
                }
            },
        }
    }
    
    // Apply condition logic
    fn apply_condition_logic(
        &self,
        results: &[ConditionResult],
        logic: &ConditionLogic
    ) -> Result<bool, ConditionError> {
        match logic {
            ConditionLogic::And => {
                Ok(results.iter().all(|r| r.passed))
            },
            ConditionLogic::Or => {
                Ok(results.iter().any(|r| r.passed))
            },
            ConditionLogic::Not => {
                if results.len() != 1 {
                    return Err(ConditionError::InvalidLogic("Not requires exactly one condition".to_string()));
                }
                Ok(!results[0].passed)
            },
            ConditionLogic::Xor => {
                let passed_count = results.iter().filter(|r| r.passed).count();
                Ok(passed_count == 1)
            },
            ConditionLogic::Nand => {
                Ok(!results.iter().all(|r| r.passed))
            },
            ConditionLogic::Nor => {
                Ok(!results.iter().any(|r| r.passed))
            },
        }
    }
}
```

### **2. Condition Cache**

```rust
// Condition Cache
pub struct ConditionCache {
    condition_cache: DashMap<String, CachedConditionResult>,
    function_cache: DashMap<String, CachedFunctionResult>,
    evaluation_cache: DashMap<String, CachedEvaluationResult>,
    cache_metrics: Arc<Mutex<CacheMetrics>>,
    max_size: usize,
    default_ttl: Duration,
}

impl ConditionCache {
    // Get cached condition result
    pub fn get_condition_result(&self, key: &str) -> Option<ConditionResult> {
        if let Some(cached_result) = self.condition_cache.get(key) {
            if cached_result.is_valid() {
                self.record_cache_hit();
                return Some(cached_result.result.clone());
            } else {
                self.condition_cache.remove(key);
            }
        }
        
        self.record_cache_miss();
        None
    }
    
    // Cache condition result
    pub fn cache_condition_result(
        &self,
        key: String,
        result: ConditionResult,
        ttl: Option<Duration>
    ) {
        let ttl = ttl.unwrap_or(self.default_ttl);
        let cached_result = CachedConditionResult {
            result,
            cached_at: SystemTime::now(),
            ttl,
        };
        
        // Check cache size
        if self.condition_cache.len() >= self.max_size {
            self.evict_oldest_entries();
        }
        
        self.condition_cache.insert(key, cached_result);
    }
    
    // Get cached function result
    pub fn get_function_result(&self, key: &str) -> Option<ConditionValue> {
        if let Some(cached_result) = self.function_cache.get(key) {
            if cached_result.is_valid() {
                self.record_cache_hit();
                return Some(cached_result.result.clone());
            } else {
                self.function_cache.remove(key);
            }
        }
        
        self.record_cache_miss();
        None
    }
    
    // Cache function result
    pub fn cache_function_result(
        &self,
        key: String,
        result: ConditionValue,
        ttl: Option<Duration>
    ) {
        let ttl = ttl.unwrap_or(self.default_ttl);
        let cached_result = CachedFunctionResult {
            result,
            cached_at: SystemTime::now(),
            ttl,
        };
        
        // Check cache size
        if self.function_cache.len() >= self.max_size {
            self.evict_oldest_function_entries();
        }
        
        self.function_cache.insert(key, cached_result);
    }
    
    // Evict oldest entries
    fn evict_oldest_entries(&self) {
        let mut entries: Vec<_> = self.condition_cache.iter().collect();
        entries.sort_by_key(|(_, result)| result.cached_at);
        
        let evict_count = self.condition_cache.len() - self.max_size + 1;
        for (key, _) in entries.iter().take(evict_count) {
            self.condition_cache.remove(*key);
        }
    }
    
    // Evict oldest function entries
    fn evict_oldest_function_entries(&self) {
        let mut entries: Vec<_> = self.function_cache.iter().collect();
        entries.sort_by_key(|(_, result)| result.cached_at);
        
        let evict_count = self.function_cache.len() - self.max_size + 1;
        for (key, _) in entries.iter().take(evict_count) {
            self.function_cache.remove(*key);
        }
    }
    
    // Record cache hit
    fn record_cache_hit(&self) {
        if let Ok(mut metrics) = self.cache_metrics.lock() {
            metrics.hits += 1;
        }
    }
    
    // Record cache miss
    fn record_cache_miss(&self) {
        if let Ok(mut metrics) = self.cache_metrics.lock() {
            metrics.misses += 1;
        }
    }
    
    // Get cache metrics
    pub fn get_cache_metrics(&self) -> CacheMetrics {
        self.cache_metrics.lock().unwrap().clone()
    }
    
    // Clear cache
    pub fn clear(&self) {
        self.condition_cache.clear();
        self.function_cache.clear();
        self.evaluation_cache.clear();
    }
}
```

## 📊 **Performance Monitoring**

### **1. Performance Monitor**

```rust
// Performance Monitor
pub struct PerformanceMonitor {
    evaluation_metrics: DashMap<String, EvaluationMetrics>,
    function_metrics: DashMap<String, FunctionMetrics>,
    cache_metrics: Arc<Mutex<CacheMetrics>>,
    performance_alerts: Vec<PerformanceAlert>,
}

impl PerformanceMonitor {
    // Record evaluation
    pub fn record_evaluation(&self, condition_id: &str, duration: Duration) {
        let mut metrics = self.evaluation_metrics
            .entry(condition_id.to_string())
            .or_insert(EvaluationMetrics::new());
        
        metrics.total_evaluations += 1;
        metrics.total_time += duration;
        metrics.average_time = metrics.total_time / metrics.total_evaluations as u32;
        
        if duration > metrics.max_time {
            metrics.max_time = duration;
        }
        
        if duration < metrics.min_time {
            metrics.min_time = duration;
        }
        
        // Check for performance alerts
        if duration > Duration::from_millis(100) {
            self.record_performance_alert(condition_id, PerformanceAlertType::SlowEvaluation, duration);
        }
    }
    
    // Record function execution
    pub fn record_function_execution(&self, function_id: &str, duration: Duration) {
        let mut metrics = self.function_metrics
            .entry(function_id.to_string())
            .or_insert(FunctionMetrics::new());
        
        metrics.total_executions += 1;
        metrics.total_time += duration;
        metrics.average_time = metrics.total_time / metrics.total_executions as u32;
        
        if duration > metrics.max_time {
            metrics.max_time = duration;
        }
        
        if duration < metrics.min_time {
            metrics.min_time = duration;
        }
    }
    
    // Record cache hit
    pub fn record_cache_hit(&self, condition_id: &str) {
        if let Ok(mut metrics) = self.cache_metrics.lock() {
            metrics.hits += 1;
        }
    }
    
    // Record cache miss
    pub fn record_cache_miss(&self, condition_id: &str) {
        if let Ok(mut metrics) = self.cache_metrics.lock() {
            metrics.misses += 1;
        }
    }
    
    // Record performance alert
    fn record_performance_alert(
        &self,
        condition_id: &str,
        alert_type: PerformanceAlertType,
        duration: Duration
    ) {
        let alert = PerformanceAlert {
            condition_id: condition_id.to_string(),
            alert_type,
            duration,
            timestamp: SystemTime::now(),
        };
        
        // In a real implementation, this would be sent to a monitoring system
        println!("Performance Alert: {:?}", alert);
    }
    
    // Get performance report
    pub fn get_performance_report(&self) -> PerformanceReport {
        let evaluation_metrics: HashMap<String, EvaluationMetrics> = self.evaluation_metrics
            .iter()
            .map(|entry| (entry.key().clone(), entry.value().clone()))
            .collect();
        
        let function_metrics: HashMap<String, FunctionMetrics> = self.function_metrics
            .iter()
            .map(|entry| (entry.key().clone(), entry.value().clone()))
            .collect();
        
        let cache_metrics = self.cache_metrics.lock().unwrap().clone();
        
        PerformanceReport {
            evaluation_metrics,
            function_metrics,
            cache_metrics,
            generated_at: SystemTime::now(),
        }
    }
}
```

## 🎯 **Key Features**

### **1. Skyrim-Inspired Design**
- **100+ Condition Functions**: Tương tự Skyrim's Condition Functions
- **Complex Logic**: AND, OR, NOT, XOR, NAND, NOR logic
- **Multiple Categories**: Actor, Item, Location, Time, Weather, Magic, Relationship
- **Performance Optimization**: Caching và async evaluation

### **2. Multiple Configuration Methods**
- **YAML String-based**: Configuration linh hoạt
- **Class/Interface-based**: Type-safe configuration
- **Hybrid Approach**: Kết hợp cả hai methods
- **Easy Migration**: Dễ dàng migrate giữa methods

### **3. Advanced Architecture**
- **Unified Condition System**: Quản lý tập trung
- **Consistent Interfaces**: Interface thống nhất
- **Centralized Processing**: Xử lý tập trung
- **Cross-System Integration**: Tích hợp với tất cả systems

### **4. Performance Optimization**
- **Centralized Caching**: Cache tập trung
- **Batch Evaluation**: Đánh giá batch
- **Performance Monitoring**: Monitor performance
- **Memory Optimization**: Tối ưu memory

---

**Last Updated**: 2025-01-27  
**Version**: 1.0  
**Status**: Architecture Design Complete  
**Maintainer**: Chaos World Team
