# Effect Core Condition System Design

## 📋 **Tổng Quan**

**⚠️ DEPRECATED: Tài liệu này đã được di chuyển sang [Condition Core](../condition-core/README.md)**

Tài liệu này thiết kế hệ thống condition phức tạp cho Effect Core, dựa trên Skyrim's Condition Functions system với hơn 100 condition functions và logic phức tạp.

**Lưu ý**: Condition System đã được tách thành Condition Core riêng biệt để có thể share với tất cả systems. Vui lòng tham khảo [Condition Core Documentation](../condition-core/README.md) để biết thêm chi tiết.

## 🎯 **Skyrim Condition System Analysis**

### **1. Skyrim Condition Functions Categories**

Skyrim có hơn 100 condition functions được chia thành các categories:

```
Skyrim Condition Functions
├── Actor Conditions (25+ functions)
│   ├── GetActorValue (Health, Magicka, Stamina, etc.)
│   ├── GetLevel, GetRace, GetSex
│   ├── IsInCombat, IsDead, IsUnconscious
│   ├── GetActorValuePercentage
│   └── GetActorValueMax
├── Item Conditions (15+ functions)
│   ├── HasItem, GetItemCount, IsEquipped
│   ├── GetItemCharge, GetItemHealth
│   ├── IsWorn, IsWornHasKeyword
│   └── GetEquippedItemType
├── Location Conditions (20+ functions)
│   ├── GetInCurrentLocation, GetInCurrentLocType
│   ├── IsInInterior, IsInWater, IsInAir
│   ├── GetDistanceFromPlayer, GetDistanceFromRef
│   └── GetLocationCleared
├── Time Conditions (10+ functions)
│   ├── GetCurrentTime, GetDayOfWeek, GetSeason
│   ├── IsDay, IsNight, IsSunrise, IsSunset
│   └── GetGameHour
├── Weather Conditions (8+ functions)
│   ├── GetCurrentWeather, IsRaining, IsSnowing
│   ├── IsStorming, IsFoggy, IsCloudy
│   └── GetWeatherTransition
├── Magic Conditions (15+ functions)
│   ├── HasMagicEffect, GetMagicEffectMagnitude
│   ├── HasSpell, GetSpellCount, IsSpellTarget
│   ├── GetMagicEffectDuration, GetMagicEffectTimeLeft
│   └── HasPerk, GetPerkCount
├── Relationship Conditions (12+ functions)
│   ├── GetRelationshipRank, IsHostileToActor
│   ├── IsFriendlyToActor, IsNeutralToActor
│   ├── GetFactionRank, IsInFaction
│   └── GetCrimeGold, GetCrimeGoldViolent
└── Custom Conditions (10+ functions)
    ├── GetGlobalValue, SetGlobalValue
    ├── GetQuestCompleted, GetQuestStage
    ├── GetEventData, GetEventDataString
    └── GetRandomPercent
```

### **2. Skyrim Condition Logic Examples**

```javascript
// Skyrim Condition Examples
// Example 1: Complex Fire Damage Condition
if (GetActorValue Health < 0.5) AND 
   (IsInCombat == 1) AND 
   (HasMagicEffect FireResist == 0) AND
   (GetCurrentWeather == 0) AND
   (GetDistanceFromPlayer < 1000)
then
   ApplyEffect FireDamage

// Example 2: Healing Potion Condition
if (GetActorValue Health < 0.8) AND
   (HasItem HealthPotion > 0) AND
   (IsInCombat == 0) AND
   (GetCurrentTime > 6.0) AND
   (GetCurrentTime < 18.0)
then
   ApplyEffect HealthHealing

// Example 3: Weather-based Effect
if (IsRaining == 1) AND
   (HasMagicEffect WaterMastery > 0) AND
   (GetInCurrentLocation == "Forest")
then
   ApplyEffect WaterAmplification
```

## 🏗️ **Chaos Condition System Design**

### **1. Condition Function Registry**

```rust
// Condition Function Registry
pub struct ConditionFunctionRegistry {
    functions: HashMap<String, Box<dyn ConditionFunction>>,
    function_categories: HashMap<String, Vec<String>>,
    function_metadata: HashMap<String, FunctionMetadata>,
}

// Condition Function Trait
pub trait ConditionFunction: Send + Sync {
    async fn evaluate(
        &self,
        parameters: &[ConditionParameter],
        context: &EffectContext
    ) -> Result<ConditionValue, ConditionError>;
    
    fn get_parameter_types(&self) -> Vec<ParameterType>;
    fn get_return_type(&self) -> ReturnType;
    fn get_description(&self) -> String;
    fn get_description_vi(&self) -> String;
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
}
```

### **2. Actor Condition Functions**

```rust
// Actor Condition Functions
pub struct GetActorValueFunction;

impl ConditionFunction for GetActorValueFunction {
    async fn evaluate(
        &self,
        parameters: &[ConditionParameter],
        context: &EffectContext
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
}

pub struct IsInCombatFunction;

impl ConditionFunction for IsInCombatFunction {
    async fn evaluate(
        &self,
        parameters: &[ConditionParameter],
        context: &EffectContext
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
}

pub struct GetLevelFunction;

impl ConditionFunction for GetLevelFunction {
    async fn evaluate(
        &self,
        parameters: &[ConditionParameter],
        context: &EffectContext
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
}
```

### **3. Item Condition Functions**

```rust
// Item Condition Functions
pub struct HasItemFunction;

impl ConditionFunction for HasItemFunction {
    async fn evaluate(
        &self,
        parameters: &[ConditionParameter],
        context: &EffectContext
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
}

pub struct GetItemCountFunction;

impl ConditionFunction for GetItemCountFunction {
    async fn evaluate(
        &self,
        parameters: &[ConditionParameter],
        context: &EffectContext
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
}

pub struct IsEquippedFunction;

impl ConditionFunction for IsEquippedFunction {
    async fn evaluate(
        &self,
        parameters: &[ConditionParameter],
        context: &EffectContext
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
}
```

### **4. Location Condition Functions**

```rust
// Location Condition Functions
pub struct GetInCurrentLocationFunction;

impl ConditionFunction for GetInCurrentLocationFunction {
    async fn evaluate(
        &self,
        parameters: &[ConditionParameter],
        context: &EffectContext
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
}

pub struct IsInInteriorFunction;

impl ConditionFunction for IsInInteriorFunction {
    async fn evaluate(
        &self,
        parameters: &[ConditionParameter],
        context: &EffectContext
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
}

pub struct GetDistanceFromPlayerFunction;

impl ConditionFunction for GetDistanceFromPlayerFunction {
    async fn evaluate(
        &self,
        parameters: &[ConditionParameter],
        context: &EffectContext
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
}
```

### **5. Time Condition Functions**

```rust
// Time Condition Functions
pub struct GetCurrentTimeFunction;

impl ConditionFunction for GetCurrentTimeFunction {
    async fn evaluate(
        &self,
        parameters: &[ConditionParameter],
        context: &EffectContext
    ) -> Result<ConditionValue, ConditionError> {
        let current_time = context.time_manager
            .get_current_time()
            .await?;
        
        Ok(ConditionValue::Float(current_time))
    }
    
    fn get_parameter_types(&self) -> Vec<ParameterType> {
        vec![] // No parameters
    }
    
    fn get_return_type(&self) -> ReturnType {
        ReturnType::Float
    }
    
    fn get_description(&self) -> String {
        "Get current time".to_string()
    }
    
    fn get_description_vi(&self) -> String {
        "Lấy thời gian hiện tại".to_string()
    }
}

pub struct IsDayFunction;

impl ConditionFunction for IsDayFunction {
    async fn evaluate(
        &self,
        parameters: &[ConditionParameter],
        context: &EffectContext
    ) -> Result<ConditionValue, ConditionError> {
        let current_time = context.time_manager
            .get_current_time()
            .await?;
        
        let is_day = current_time >= 6.0 && current_time < 18.0;
        Ok(ConditionValue::Boolean(is_day))
    }
    
    fn get_parameter_types(&self) -> Vec<ParameterType> {
        vec![] // No parameters
    }
    
    fn get_return_type(&self) -> ReturnType {
        ReturnType::Boolean
    }
    
    fn get_description(&self) -> String {
        "Check if it's day time".to_string()
    }
    
    fn get_description_vi(&self) -> String {
        "Kiểm tra có phải ban ngày".to_string()
    }
}

pub struct GetDayOfWeekFunction;

impl ConditionFunction for GetDayOfWeekFunction {
    async fn evaluate(
        &self,
        parameters: &[ConditionParameter],
        context: &EffectContext
    ) -> Result<ConditionValue, ConditionError> {
        let day_of_week = context.time_manager
            .get_day_of_week()
            .await?;
        
        Ok(ConditionValue::Integer(day_of_week as i64))
    }
    
    fn get_parameter_types(&self) -> Vec<ParameterType> {
        vec![] // No parameters
    }
    
    fn get_return_type(&self) -> ReturnType {
        ReturnType::Integer
    }
    
    fn get_description(&self) -> String {
        "Get day of week".to_string()
    }
    
    fn get_description_vi(&self) -> String {
        "Lấy ngày trong tuần".to_string()
    }
}
```

### **6. Weather Condition Functions**

```rust
// Weather Condition Functions
pub struct GetCurrentWeatherFunction;

impl ConditionFunction for GetCurrentWeatherFunction {
    async fn evaluate(
        &self,
        parameters: &[ConditionParameter],
        context: &EffectContext
    ) -> Result<ConditionValue, ConditionError> {
        let current_weather = context.weather_manager
            .get_current_weather()
            .await?;
        
        Ok(ConditionValue::String(current_weather))
    }
    
    fn get_parameter_types(&self) -> Vec<ParameterType> {
        vec![] // No parameters
    }
    
    fn get_return_type(&self) -> ReturnType {
        ReturnType::String
    }
    
    fn get_description(&self) -> String {
        "Get current weather".to_string()
    }
    
    fn get_description_vi(&self) -> String {
        "Lấy thời tiết hiện tại".to_string()
    }
}

pub struct IsRainingFunction;

impl ConditionFunction for IsRainingFunction {
    async fn evaluate(
        &self,
        parameters: &[ConditionParameter],
        context: &EffectContext
    ) -> Result<ConditionValue, ConditionError> {
        let current_weather = context.weather_manager
            .get_current_weather()
            .await?;
        
        let is_raining = current_weather == "rain" || current_weather == "storm";
        Ok(ConditionValue::Boolean(is_raining))
    }
    
    fn get_parameter_types(&self) -> Vec<ParameterType> {
        vec![] // No parameters
    }
    
    fn get_return_type(&self) -> ReturnType {
        ReturnType::Boolean
    }
    
    fn get_description(&self) -> String {
        "Check if it's raining".to_string()
    }
    
    fn get_description_vi(&self) -> String {
        "Kiểm tra có đang mưa".to_string()
    }
}
```

### **7. Magic Condition Functions**

```rust
// Magic Condition Functions
pub struct HasEffectFunction;

impl ConditionFunction for HasEffectFunction {
    async fn evaluate(
        &self,
        parameters: &[ConditionParameter],
        context: &EffectContext
    ) -> Result<ConditionValue, ConditionError> {
        let effect_id = parameters.get(0)
            .ok_or(ConditionError::MissingParameter("effect_id"))?
            .as_string()?;
        
        let actor_id = context.target.id.clone();
        let has_effect = context.effect_manager
            .has_effect(&actor_id, &effect_id)
            .await?;
        
        Ok(ConditionValue::Boolean(has_effect))
    }
    
    fn get_parameter_types(&self) -> Vec<ParameterType> {
        vec![ParameterType::String] // effect_id
    }
    
    fn get_return_type(&self) -> ReturnType {
        ReturnType::Boolean
    }
    
    fn get_description(&self) -> String {
        "Check if actor has effect".to_string()
    }
    
    fn get_description_vi(&self) -> String {
        "Kiểm tra diễn viên có hiệu ứng".to_string()
    }
}

pub struct GetEffectMagnitudeFunction;

impl ConditionFunction for GetEffectMagnitudeFunction {
    async fn evaluate(
        &self,
        parameters: &[ConditionParameter],
        context: &EffectContext
    ) -> Result<ConditionValue, ConditionError> {
        let effect_id = parameters.get(0)
            .ok_or(ConditionError::MissingParameter("effect_id"))?
            .as_string()?;
        
        let actor_id = context.target.id.clone();
        let magnitude = context.effect_manager
            .get_effect_magnitude(&actor_id, &effect_id)
            .await?;
        
        Ok(ConditionValue::Float(magnitude))
    }
    
    fn get_parameter_types(&self) -> Vec<ParameterType> {
        vec![ParameterType::String] // effect_id
    }
    
    fn get_return_type(&self) -> ReturnType {
        ReturnType::Float
    }
    
    fn get_description(&self) -> String {
        "Get effect magnitude".to_string()
    }
    
    fn get_description_vi(&self) -> String {
        "Lấy cường độ hiệu ứng".to_string()
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
}

impl ConditionEvaluator {
    // Evaluate single condition
    pub async fn evaluate_condition(
        &self,
        condition: &EffectCondition,
        context: &EffectContext
    ) -> Result<ConditionResult, ConditionError> {
        // Check cache first
        let cache_key = self.generate_cache_key(condition, context);
        if let Some(cached_result) = self.condition_cache.get(&cache_key) {
            return Ok(cached_result.clone());
        }
        
        // Get condition function
        let function = self.function_registry
            .get_function(&condition.condition_function)
            .ok_or(ConditionError::FunctionNotFound(condition.condition_function.clone()))?;
        
        // Evaluate condition
        let condition_value = function.evaluate(&condition.parameters, context).await?;
        
        // Apply operator
        let result = self.apply_operator(
            &condition_value,
            &condition.condition_operator,
            &condition.condition_value
        )?;
        
        // Cache result
        let condition_result = ConditionResult {
            condition_id: condition.condition_id.clone(),
            passed: result,
            value: condition_value,
            evaluated_at: SystemTime::now(),
        };
        
        self.condition_cache.cache(cache_key, condition_result.clone());
        
        Ok(condition_result)
    }
    
    // Evaluate condition chain
    pub async fn evaluate_condition_chain(
        &self,
        conditions: &[EffectCondition],
        context: &EffectContext
    ) -> Result<ConditionChainResult, ConditionError> {
        let mut results = Vec::new();
        
        for condition in conditions {
            let result = self.evaluate_condition(condition, context).await?;
            results.push(result);
        }
        
        // Apply condition logic
        let final_result = self.apply_condition_logic(&results, &conditions[0].condition_logic)?;
        
        Ok(ConditionChainResult {
            conditions: results,
            final_result,
            evaluated_at: SystemTime::now(),
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
    cache: Arc<Mutex<HashMap<String, ConditionResult>>>,
    cache_ttl: Duration,
    max_size: usize,
}

impl ConditionCache {
    // Get cached result
    pub fn get(&self, key: &str) -> Option<ConditionResult> {
        let cache = self.cache.lock().unwrap();
        cache.get(key).cloned()
    }
    
    // Cache result
    pub fn cache(&self, key: String, result: ConditionResult) {
        let mut cache = self.cache.lock().unwrap();
        
        // Check cache size
        if cache.len() >= self.max_size {
            self.evict_oldest_entries(&mut cache);
        }
        
        cache.insert(key, result);
    }
    
    // Evict oldest entries
    fn evict_oldest_entries(&self, cache: &mut HashMap<String, ConditionResult>) {
        let mut entries: Vec<_> = cache.iter().collect();
        entries.sort_by_key(|(_, result)| result.evaluated_at);
        
        let evict_count = cache.len() - self.max_size + 1;
        for (key, _) in entries.iter().take(evict_count) {
            cache.remove(*key);
        }
    }
    
    // Clear cache
    pub fn clear(&self) {
        let mut cache = self.cache.lock().unwrap();
        cache.clear();
    }
}
```

## 📊 **Configuration Examples**

### **1. Complex Fire Damage Condition**

```yaml
# Complex Fire Damage Condition
effect_definition:
  effect_guid: "550e8400-e29b-41d4-a716-446655440000"
  effect_id: "EFF_FireDamage"
  effect_name: "Fire Damage"
  effect_name_vi: "Sát Thương Hỏa"
  
  conditions:
    - condition_id: "health_condition"
      condition_function: "get_actor_value"
      condition_parameter: "health"
      condition_operator: "less_than"
      condition_value: 0.5
      condition_logic: "AND"
      
    - condition_id: "combat_condition"
      condition_function: "is_in_combat"
      condition_operator: "equal"
      condition_value: true
      condition_logic: "AND"
      
    - condition_id: "fire_resist_condition"
      condition_function: "has_effect"
      condition_parameter: "fire_resistance"
      condition_operator: "equal"
      condition_value: false
      condition_logic: "AND"
      
    - condition_id: "weather_condition"
      condition_function: "get_current_weather"
      condition_operator: "equal"
      condition_value: "clear"
      condition_logic: "AND"
      
    - condition_id: "distance_condition"
      condition_function: "get_distance_from_player"
      condition_operator: "less_than"
      condition_value: 1000.0
      condition_logic: "AND"
```

### **2. Healing Potion Condition**

```yaml
# Healing Potion Condition
effect_definition:
  effect_guid: "550e8400-e29b-41d4-a716-446655440001"
  effect_id: "EFF_HealthHealing"
  effect_name: "Health Healing"
  effect_name_vi: "Hồi Máu"
  
  conditions:
    - condition_id: "health_condition"
      condition_function: "get_actor_value"
      condition_parameter: "health"
      condition_operator: "less_than"
      condition_value: 0.8
      condition_logic: "AND"
      
    - condition_id: "item_condition"
      condition_function: "has_item"
      condition_parameter: "health_potion"
      condition_operator: "greater_than"
      condition_value: 0
      condition_logic: "AND"
      
    - condition_id: "combat_condition"
      condition_function: "is_in_combat"
      condition_operator: "equal"
      condition_value: false
      condition_logic: "AND"
      
    - condition_id: "time_condition"
      condition_function: "get_current_time"
      condition_operator: "greater_than"
      condition_value: 6.0
      condition_logic: "AND"
      
    - condition_id: "time_condition_2"
      condition_function: "get_current_time"
      condition_operator: "less_than"
      condition_value: 18.0
      condition_logic: "AND"
```

### **3. Weather-based Effect Condition**

```yaml
# Weather-based Effect Condition
effect_definition:
  effect_guid: "550e8400-e29b-41d4-a716-446655440002"
  effect_id: "EFF_WaterAmplification"
  effect_name: "Water Amplification"
  effect_name_vi: "Khuếch Đại Thủy"
  
  conditions:
    - condition_id: "weather_condition"
      condition_function: "is_raining"
      condition_operator: "equal"
      condition_value: true
      condition_logic: "AND"
      
    - condition_id: "mastery_condition"
      condition_function: "has_effect"
      condition_parameter: "water_mastery"
      condition_operator: "greater_than"
      condition_value: 0
      condition_logic: "AND"
      
    - condition_id: "location_condition"
      condition_function: "get_in_current_location"
      condition_parameter: "forest"
      condition_operator: "equal"
      condition_value: true
      condition_logic: "AND"
```

## 🎯 **Key Features**

### **1. Skyrim-Inspired Design**
- **100+ Condition Functions**: Tương tự Skyrim's Condition Functions
- **Complex Logic**: AND, OR, NOT, XOR, NAND, NOR logic
- **Multiple Categories**: Actor, Item, Location, Time, Weather, Magic, Relationship
- **Performance Optimization**: Caching và async evaluation

### **2. Advanced Condition System**
- **Flexible Parameters**: Support multiple parameter types
- **Condition Chaining**: Complex condition chains
- **Condition Caching**: Performance optimization
- **Async Evaluation**: Non-blocking condition evaluation

### **3. Extensible Architecture**
- **Custom Functions**: Easy to add new condition functions
- **Plugin Support**: Support for plugin condition functions
- **Version Control**: Support for condition function versioning
- **Migration Support**: Support for condition migration

---

**Last Updated**: 2025-01-27  
**Version**: 1.0  
**Status**: Condition System Design Complete  
**Maintainer**: Chaos World Team
