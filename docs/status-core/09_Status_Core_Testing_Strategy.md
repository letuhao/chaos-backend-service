# Status Core Testing Strategy

## 📋 **Tổng Quan**

Status Core Testing Strategy định nghĩa comprehensive testing approach cho Status Core system, bao gồm unit testing, integration testing, performance testing, và end-to-end testing.

## 🎯 **Nguyên Tắc Thiết Kế**

### **1. Testing Principles**
- **Test Pyramid**: Unit tests > Integration tests > E2E tests
- **Test Coverage**: 90%+ code coverage
- **Test Isolation**: Tests should be independent
- **Test Reliability**: Tests should be deterministic

### **2. Testing Categories**
- **Unit Testing**: Test individual components
- **Integration Testing**: Test component interactions
- **Performance Testing**: Test performance characteristics
- **End-to-End Testing**: Test complete workflows

### **3. Testing Tools**
- **Rust Testing**: Built-in testing framework
- **Mocking**: Mock external dependencies
- **Property Testing**: Property-based testing
- **Load Testing**: Performance testing tools

## 🏗️ **Testing Architecture**

### **1. Unit Testing Framework**

```rust
/// Unit Testing Framework for Status Core
pub struct StatusCoreUnitTestFramework {
    // Test configuration
    config: TestConfig,
    
    // Mock services
    mock_services: MockServices,
    
    // Test data generators
    data_generators: DataGenerators,
    
    // Test utilities
    test_utilities: TestUtilities,
}

impl StatusCoreUnitTestFramework {
    /// Create test status effect manager
    pub async fn create_test_status_effect_manager(
        &self
    ) -> Result<StatusEffectManager, TestError> {
        let config = self.create_test_config();
        let plugin_registry = self.create_mock_plugin_registry().await?;
        let configuration_manager = self.create_mock_configuration_manager().await?;
        
        StatusEffectManager::new(
            plugin_registry,
            configuration_manager,
            config
        ).await
    }
    
    /// Create test immunity manager
    pub async fn create_test_immunity_manager(
        &self
    ) -> Result<ImmunityManager, TestError> {
        let config = self.create_test_config();
        let plugin_registry = self.create_mock_plugin_registry().await?;
        let configuration_manager = self.create_mock_configuration_manager().await?;
        
        ImmunityManager::new(
            plugin_registry,
            configuration_manager,
            config
        ).await
    }
    
    /// Create test status core engine
    pub async fn create_test_status_core_engine(
        &self
    ) -> Result<StatusCoreEngine, TestError> {
        let config = self.create_test_config();
        let plugin_registry = self.create_mock_plugin_registry().await?;
        let configuration_manager = self.create_mock_configuration_manager().await?;
        
        StatusCoreEngine::new(
            config,
            plugin_registry,
            configuration_manager
        ).await
    }
    
    /// Create mock plugin registry
    async fn create_mock_plugin_registry(
        &self
    ) -> Result<Arc<StatusPluginRegistry>, TestError> {
        let mut registry = StatusPluginRegistry::new();
        
        // Register mock plugins
        registry.register_plugin(
            Box::new(MockFirePlugin::new()),
            StatusPluginConfig::default()
        ).await?;
        
        registry.register_plugin(
            Box::new(MockWaterPlugin::new()),
            StatusPluginConfig::default()
        ).await?;
        
        Ok(Arc::new(registry))
    }
    
    /// Create mock configuration manager
    async fn create_mock_configuration_manager(
        &self
    ) -> Result<Arc<StatusCoreConfigurationManager>, TestError> {
        let config_manager = StatusCoreConfigurationManager::new(
            self.create_test_configuration()
        ).await?;
        
        Ok(Arc::new(config_manager))
    }
}

/// Mock Services for Testing
pub struct MockServices {
    // Mock external services
    mock_element_core: MockElementCore,
    mock_action_core: MockActionCore,
    mock_combat_core: MockCombatCore,
    mock_actor_core: MockActorCore,
    
    // Mock internal services
    mock_cache: MockCache,
    mock_memory_pool: MockMemoryPool,
    mock_event_dispatcher: MockEventDispatcher,
}

/// Mock Element Core
pub struct MockElementCore {
    // Mock data
    mock_elements: HashMap<String, ElementDefinition>,
    mock_interactions: HashMap<String, ElementInteraction>,
    mock_derived_stats: HashMap<String, f64>,
}

impl MockElementCore {
    /// Create new mock element core
    pub fn new() -> Self {
        let mut mock_elements = HashMap::new();
        let mut mock_interactions = HashMap::new();
        let mut mock_derived_stats = HashMap::new();
        
        // Add mock fire element
        mock_elements.insert("fire".to_string(), ElementDefinition {
            id: "fire".to_string(),
            name: "Fire".to_string(),
            name_vi: "Hỏa".to_string(),
            category: ElementCategory::Basic,
            properties: HashMap::new(),
            derived_stats: HashMap::new(),
        });
        
        // Add mock fire-water interaction
        mock_interactions.insert("fire:water".to_string(), ElementInteraction {
            source_element: "fire".to_string(),
            target_element: "water".to_string(),
            interaction_type: ElementInteractionType::Overcoming,
            base_probability: 0.8,
            mastery_scaling: 0.1,
        });
        
        // Add mock derived stats
        mock_derived_stats.insert("fire_mastery".to_string(), 100.0);
        mock_derived_stats.insert("fire_resistance".to_string(), 50.0);
        
        Self {
            mock_elements,
            mock_interactions,
            mock_derived_stats,
        }
    }
    
    /// Get element definition
    pub async fn get_element_definition(
        &self,
        element_id: &str
    ) -> Result<Option<ElementDefinition>, StatusError> {
        Ok(self.mock_elements.get(element_id).cloned())
    }
    
    /// Get element interaction
    pub async fn get_element_interaction(
        &self,
        source_element: &str,
        target_element: &str
    ) -> Result<Option<ElementInteraction>, StatusError> {
        let key = format!("{}:{}", source_element, target_element);
        Ok(self.mock_interactions.get(&key).cloned())
    }
    
    /// Get derived stat value
    pub async fn get_derived_stat_value(
        &self,
        actor_id: &str,
        stat_name: &str
    ) -> Result<f64, StatusError> {
        Ok(self.mock_derived_stats.get(stat_name).copied().unwrap_or(0.0))
    }
}

/// Mock Action Core
pub struct MockActionCore {
    // Mock data
    mock_actions: HashMap<String, ActionDefinition>,
    mock_derived_stats: HashMap<String, f64>,
}

impl MockActionCore {
    /// Create new mock action core
    pub fn new() -> Self {
        let mut mock_actions = HashMap::new();
        let mut mock_derived_stats = HashMap::new();
        
        // Add mock fireball action
        mock_actions.insert("fireball".to_string(), ActionDefinition {
            id: "fireball".to_string(),
            name: "Fireball".to_string(),
            name_vi: "Hỏa Cầu".to_string(),
            category: ActionCategory::Combat,
            action_type: ActionType::Attack,
            target_type: TargetType::Single,
            execution_duration: Duration::from_millis(1000),
            cooldown_duration: Duration::from_secs(5),
            resource_consumption: HashMap::new(),
            effects: Vec::new(),
        });
        
        // Add mock derived stats
        mock_derived_stats.insert("attack_skill_effectiveness".to_string(), 1.2);
        mock_derived_stats.insert("skill_execution_speed".to_string(), 1.1);
        
        Self {
            mock_actions,
            mock_derived_stats,
        }
    }
    
    /// Get action definition
    pub async fn get_action_definition(
        &self,
        action_id: &str
    ) -> Result<Option<ActionDefinition>, StatusError> {
        Ok(self.mock_actions.get(action_id).cloned())
    }
    
    /// Get derived stat value
    pub async fn get_derived_stat_value(
        &self,
        actor_id: &str,
        stat_name: &str
    ) -> Result<f64, StatusError> {
        Ok(self.mock_derived_stats.get(stat_name).copied().unwrap_or(0.0))
    }
}

/// Mock Combat Core
pub struct MockCombatCore {
    // Mock data
    mock_damage_calculations: HashMap<String, f64>,
    mock_defense_calculations: HashMap<String, f64>,
}

impl MockCombatCore {
    /// Create new mock combat core
    pub fn new() -> Self {
        let mut mock_damage_calculations = HashMap::new();
        let mut mock_defense_calculations = HashMap::new();
        
        // Add mock damage calculations
        mock_damage_calculations.insert("fire_damage".to_string(), 100.0);
        mock_damage_calculations.insert("water_damage".to_string(), 80.0);
        
        // Add mock defense calculations
        mock_defense_calculations.insert("fire_defense".to_string(), 50.0);
        mock_defense_calculations.insert("water_defense".to_string(), 60.0);
        
        Self {
            mock_damage_calculations,
            mock_defense_calculations,
        }
    }
    
    /// Calculate damage
    pub async fn calculate_damage(
        &self,
        attacker_id: &str,
        target_id: &str,
        damage_input: &DamageInput
    ) -> Result<DamageOutput, StatusError> {
        let base_damage = self.mock_damage_calculations
            .get(&damage_input.element_id)
            .copied()
            .unwrap_or(0.0);
        
        let defense = self.mock_defense_calculations
            .get(&damage_input.element_id)
            .copied()
            .unwrap_or(0.0);
        
        let final_damage = (base_damage - defense).max(0.0);
        
        Ok(DamageOutput {
            total_damage: final_damage,
            element_damage: HashMap::new(),
            critical_hit: false,
            critical_multiplier: 1.0,
            damage_breakdown: Vec::new(),
        })
    }
}

/// Mock Actor Core
pub struct MockActorCore {
    // Mock data
    mock_actors: HashMap<String, Actor>,
    mock_derived_stats: HashMap<String, f64>,
}

impl MockActorCore {
    /// Create new mock actor core
    pub fn new() -> Self {
        let mut mock_actors = HashMap::new();
        let mut mock_derived_stats = HashMap::new();
        
        // Add mock actor
        mock_actors.insert("test_actor".to_string(), Actor {
            id: "test_actor".to_string(),
            name: "Test Actor".to_string(),
            level: 1,
            primary_stats: HashMap::new(),
            derived_stats: HashMap::new(),
            equipment: Vec::new(),
            talents: Vec::new(),
        });
        
        // Add mock derived stats
        mock_derived_stats.insert("hp".to_string(), 1000.0);
        mock_derived_stats.insert("mp".to_string(), 500.0);
        mock_derived_stats.insert("stamina".to_string(), 200.0);
        
        Self {
            mock_actors,
            mock_derived_stats,
        }
    }
    
    /// Get actor
    pub async fn get_actor(
        &self,
        actor_id: &str
    ) -> Result<Option<Actor>, StatusError> {
        Ok(self.mock_actors.get(actor_id).cloned())
    }
    
    /// Get derived stat value
    pub async fn get_derived_stat_value(
        &self,
        actor_id: &str,
        stat_name: &str
    ) -> Result<f64, StatusError> {
        Ok(self.mock_derived_stats.get(stat_name).copied().unwrap_or(0.0))
    }
}
```

### **2. Integration Testing Framework**

```rust
/// Integration Testing Framework for Status Core
pub struct StatusCoreIntegrationTestFramework {
    // Test configuration
    config: IntegrationTestConfig,
    
    // Test services
    test_services: TestServices,
    
    // Test data
    test_data: TestData,
    
    // Test utilities
    test_utilities: TestUtilities,
}

impl StatusCoreIntegrationTestFramework {
    /// Create test environment
    pub async fn create_test_environment(
        &self
    ) -> Result<TestEnvironment, TestError> {
        let status_core_engine = self.create_status_core_engine().await?;
        let element_core = self.create_element_core().await?;
        let action_core = self.create_action_core().await?;
        let combat_core = self.create_combat_core().await?;
        let actor_core = self.create_actor_core().await?;
        
        Ok(TestEnvironment {
            status_core_engine,
            element_core,
            action_core,
            combat_core,
            actor_core,
        })
    }
    
    /// Test status effect application
    pub async fn test_status_effect_application(
        &self,
        environment: &TestEnvironment
    ) -> Result<(), TestError> {
        let actor_id = "test_actor";
        let status_effect = self.create_test_status_effect();
        let context = self.create_test_status_context();
        
        // Apply status effect
        let result = environment.status_core_engine
            .apply_status_effect(actor_id, status_effect, &context)
            .await?;
        
        // Verify result
        assert!(result.success);
        assert_eq!(result.effect_id, "test_effect");
        
        // Verify effect is active
        let active_effects = environment.status_core_engine
            .get_actor_status_effects(actor_id)
            .await?;
        
        assert_eq!(active_effects.len(), 1);
        assert_eq!(active_effects[0].effect_id, "test_effect");
        
        Ok(())
    }
    
    /// Test status effect processing
    pub async fn test_status_effect_processing(
        &self,
        environment: &TestEnvironment
    ) -> Result<(), TestError> {
        let actor_id = "test_actor";
        let context = self.create_test_status_context();
        
        // Apply multiple status effects
        let effects = vec![
            self.create_test_status_effect_with_id("effect1"),
            self.create_test_status_effect_with_id("effect2"),
            self.create_test_status_effect_with_id("effect3"),
        ];
        
        for effect in effects {
            environment.status_core_engine
                .apply_status_effect(actor_id, effect, &context)
                .await?;
        }
        
        // Process status effects
        let results = environment.status_core_engine
            .process_actor_status_effects(actor_id, &context)
            .await?;
        
        // Verify results
        assert_eq!(results.len(), 3);
        for result in results {
            assert!(result.success);
        }
        
        Ok(())
    }
    
    /// Test immunity system
    pub async fn test_immunity_system(
        &self,
        environment: &TestEnvironment
    ) -> Result<(), TestError> {
        let actor_id = "test_actor";
        let context = self.create_test_status_context();
        
        // Apply immunity
        let immunity = self.create_test_immunity();
        let immunity_result = environment.status_core_engine
            .immunity_manager
            .apply_immunity(actor_id, immunity, &context)
            .await?;
        
        assert!(immunity_result.success);
        
        // Try to apply effect that should be blocked
        let status_effect = self.create_test_status_effect();
        let result = environment.status_core_engine
            .apply_status_effect(actor_id, status_effect, &context)
            .await?;
        
        assert!(!result.success);
        assert_eq!(result.reason, StatusEffectFailureReason::Immunity);
        
        Ok(())
    }
    
    /// Test element interaction system
    pub async fn test_element_interaction_system(
        &self,
        environment: &TestEnvironment
    ) -> Result<(), TestError> {
        let actor_id = "test_actor";
        let context = self.create_test_status_context();
        
        // Apply fire effect
        let fire_effect = self.create_fire_status_effect();
        let fire_result = environment.status_core_engine
            .apply_status_effect(actor_id, fire_effect, &context)
            .await?;
        
        assert!(fire_result.success);
        
        // Apply water effect (should interact with fire)
        let water_effect = self.create_water_status_effect();
        let water_result = environment.status_core_engine
            .apply_status_effect(actor_id, water_effect, &context)
            .await?;
        
        assert!(water_result.success);
        
        // Verify interaction occurred
        let active_effects = environment.status_core_engine
            .get_actor_status_effects(actor_id)
            .await?;
        
        // Should have both effects or interaction result
        assert!(!active_effects.is_empty());
        
        Ok(())
    }
}

/// Test Environment
pub struct TestEnvironment {
    pub status_core_engine: StatusCoreEngine,
    pub element_core: ElementCore,
    pub action_core: ActionCore,
    pub combat_core: CombatCore,
    pub actor_core: ActorCore,
}
```

### **3. Performance Testing Framework**

```rust
/// Performance Testing Framework for Status Core
pub struct StatusCorePerformanceTestFramework {
    // Performance test configuration
    config: PerformanceTestConfig,
    
    // Load testing tools
    load_tester: LoadTester,
    
    // Performance monitoring
    performance_monitor: PerformanceMonitor,
    
    // Benchmark tools
    benchmark_tools: BenchmarkTools,
}

impl StatusCorePerformanceTestFramework {
    /// Run throughput test
    pub async fn run_throughput_test(
        &self,
        environment: &TestEnvironment
    ) -> Result<ThroughputTestResult, TestError> {
        let start_time = Instant::now();
        let mut handles = Vec::new();
        
        // Create test tasks
        for i in 0..self.config.throughput_test_config.task_count {
            let environment = environment.clone();
            let task = self.create_throughput_test_task(i);
            
            let handle = tokio::spawn(async move {
                environment.status_core_engine
                    .process_actor_status_effects(&task.actor_id, &task.context)
                    .await
            });
            
            handles.push(handle);
        }
        
        // Wait for all tasks to complete
        let mut success_count = 0;
        let mut error_count = 0;
        
        for handle in handles {
            match handle.await? {
                Ok(_) => success_count += 1,
                Err(_) => error_count += 1,
            }
        }
        
        let total_time = start_time.elapsed();
        let throughput = success_count as f64 / total_time.as_secs_f64();
        
        Ok(ThroughputTestResult {
            total_tasks: self.config.throughput_test_config.task_count,
            success_count,
            error_count,
            total_time,
            throughput,
            success_rate: success_count as f64 / self.config.throughput_test_config.task_count as f64,
        })
    }
    
    /// Run latency test
    pub async fn run_latency_test(
        &self,
        environment: &TestEnvironment
    ) -> Result<LatencyTestResult, TestError> {
        let mut latencies = Vec::new();
        
        for _ in 0..self.config.latency_test_config.sample_count {
            let start_time = Instant::now();
            
            let result = environment.status_core_engine
                .apply_status_effect(
                    "test_actor",
                    self.create_test_status_effect(),
                    &self.create_test_status_context()
                )
                .await?;
            
            let latency = start_time.elapsed();
            latencies.push(latency);
            
            // Verify result
            assert!(result.success);
        }
        
        // Calculate statistics
        latencies.sort();
        let p50 = latencies[latencies.len() / 2];
        let p95 = latencies[(latencies.len() * 95) / 100];
        let p99 = latencies[(latencies.len() * 99) / 100];
        let average = latencies.iter().sum::<Duration>() / latencies.len() as u32;
        
        Ok(LatencyTestResult {
            sample_count: latencies.len(),
            average_latency: average,
            p50_latency: p50,
            p95_latency: p95,
            p99_latency: p99,
            min_latency: *latencies.first().unwrap(),
            max_latency: *latencies.last().unwrap(),
        })
    }
    
    /// Run memory usage test
    pub async fn run_memory_usage_test(
        &self,
        environment: &TestEnvironment
    ) -> Result<MemoryUsageTestResult, TestError> {
        let initial_memory = self.get_memory_usage();
        
        // Apply many status effects
        let mut effects = Vec::new();
        for i in 0..self.config.memory_test_config.effect_count {
            let effect = self.create_test_status_effect_with_id(&format!("effect_{}", i));
            effects.push(effect);
        }
        
        for effect in effects {
            environment.status_core_engine
                .apply_status_effect("test_actor", effect, &self.create_test_status_context())
                .await?;
        }
        
        let peak_memory = self.get_memory_usage();
        let memory_increase = peak_memory - initial_memory;
        
        // Clean up
        for i in 0..self.config.memory_test_config.effect_count {
            environment.status_core_engine
                .remove_status_effect("test_actor", &format!("effect_{}", i))
                .await?;
        }
        
        let final_memory = self.get_memory_usage();
        let memory_after_cleanup = final_memory - initial_memory;
        
        Ok(MemoryUsageTestResult {
            initial_memory,
            peak_memory,
            final_memory,
            memory_increase,
            memory_after_cleanup,
            effect_count: self.config.memory_test_config.effect_count,
        })
    }
    
    /// Run stress test
    pub async fn run_stress_test(
        &self,
        environment: &TestEnvironment
    ) -> Result<StressTestResult, TestError> {
        let start_time = Instant::now();
        let mut handles = Vec::new();
        
        // Create stress test tasks
        for i in 0..self.config.stress_test_config.task_count {
            let environment = environment.clone();
            let task = self.create_stress_test_task(i);
            
            let handle = tokio::spawn(async move {
                // Run multiple operations
                for _ in 0..task.operation_count {
                    environment.status_core_engine
                        .apply_status_effect(&task.actor_id, task.effect.clone(), &task.context)
                        .await?;
                    
                    environment.status_core_engine
                        .process_actor_status_effects(&task.actor_id, &task.context)
                        .await?;
                }
                
                Ok::<(), StatusError>(())
            });
            
            handles.push(handle);
        }
        
        // Wait for all tasks to complete
        let mut success_count = 0;
        let mut error_count = 0;
        
        for handle in handles {
            match handle.await? {
                Ok(_) => success_count += 1,
                Err(_) => error_count += 1,
            }
        }
        
        let total_time = start_time.elapsed();
        
        Ok(StressTestResult {
            total_tasks: self.config.stress_test_config.task_count,
            success_count,
            error_count,
            total_time,
            success_rate: success_count as f64 / self.config.stress_test_config.task_count as f64,
        })
    }
}

/// Throughput Test Result
#[derive(Debug, Clone)]
pub struct ThroughputTestResult {
    pub total_tasks: usize,
    pub success_count: usize,
    pub error_count: usize,
    pub total_time: Duration,
    pub throughput: f64,
    pub success_rate: f64,
}

/// Latency Test Result
#[derive(Debug, Clone)]
pub struct LatencyTestResult {
    pub sample_count: usize,
    pub average_latency: Duration,
    pub p50_latency: Duration,
    pub p95_latency: Duration,
    pub p99_latency: Duration,
    pub min_latency: Duration,
    pub max_latency: Duration,
}

/// Memory Usage Test Result
#[derive(Debug, Clone)]
pub struct MemoryUsageTestResult {
    pub initial_memory: u64,
    pub peak_memory: u64,
    pub final_memory: u64,
    pub memory_increase: u64,
    pub memory_after_cleanup: u64,
    pub effect_count: usize,
}

/// Stress Test Result
#[derive(Debug, Clone)]
pub struct StressTestResult {
    pub total_tasks: usize,
    pub success_count: usize,
    pub error_count: usize,
    pub total_time: Duration,
    pub success_rate: f64,
}
```

### **4. End-to-End Testing Framework**

```rust
/// End-to-End Testing Framework for Status Core
pub struct StatusCoreE2ETestFramework {
    // E2E test configuration
    config: E2ETestConfig,
    
    // Test scenarios
    test_scenarios: TestScenarios,
    
    // Test data
    test_data: TestData,
    
    // Test utilities
    test_utilities: TestUtilities,
}

impl StatusCoreE2ETestFramework {
    /// Run complete status effect workflow test
    pub async fn run_status_effect_workflow_test(
        &self,
        environment: &TestEnvironment
    ) -> Result<(), TestError> {
        // Test complete workflow
        let actor_id = "test_actor";
        let context = self.create_test_status_context();
        
        // 1. Apply status effect
        let status_effect = self.create_test_status_effect();
        let apply_result = environment.status_core_engine
            .apply_status_effect(actor_id, status_effect, &context)
            .await?;
        
        assert!(apply_result.success);
        
        // 2. Process status effect
        let process_results = environment.status_core_engine
            .process_actor_status_effects(actor_id, &context)
            .await?;
        
        assert!(!process_results.is_empty());
        
        // 3. Get active status effects
        let active_effects = environment.status_core_engine
            .get_actor_status_effects(actor_id)
            .await?;
        
        assert_eq!(active_effects.len(), 1);
        
        // 4. Remove status effect
        let remove_result = environment.status_core_engine
            .remove_status_effect(actor_id, "test_effect")
            .await?;
        
        assert!(remove_result.success);
        
        // 5. Verify effect is removed
        let active_effects_after = environment.status_core_engine
            .get_actor_status_effects(actor_id)
            .await?;
        
        assert!(active_effects_after.is_empty());
        
        Ok(())
    }
    
    /// Run multi-element interaction test
    pub async fn run_multi_element_interaction_test(
        &self,
        environment: &TestEnvironment
    ) -> Result<(), TestError> {
        let actor_id = "test_actor";
        let context = self.create_test_status_context();
        
        // Apply fire effect
        let fire_effect = self.create_fire_status_effect();
        let fire_result = environment.status_core_engine
            .apply_status_effect(actor_id, fire_effect, &context)
            .await?;
        
        assert!(fire_result.success);
        
        // Apply water effect
        let water_effect = self.create_water_status_effect();
        let water_result = environment.status_core_engine
            .apply_status_effect(actor_id, water_effect, &context)
            .await?;
        
        assert!(water_result.success);
        
        // Process effects
        let process_results = environment.status_core_engine
            .process_actor_status_effects(actor_id, &context)
            .await?;
        
        // Verify interaction occurred
        assert!(!process_results.is_empty());
        
        Ok(())
    }
    
    /// Run immunity system test
    pub async fn run_immunity_system_test(
        &self,
        environment: &TestEnvironment
    ) -> Result<(), TestError> {
        let actor_id = "test_actor";
        let context = self.create_test_status_context();
        
        // Apply immunity
        let immunity = self.create_test_immunity();
        let immunity_result = environment.status_core_engine
            .immunity_manager
            .apply_immunity(actor_id, immunity, &context)
            .await?;
        
        assert!(immunity_result.success);
        
        // Try to apply effect that should be blocked
        let status_effect = self.create_test_status_effect();
        let result = environment.status_core_engine
            .apply_status_effect(actor_id, status_effect, &context)
            .await?;
        
        assert!(!result.success);
        assert_eq!(result.reason, StatusEffectFailureReason::Immunity);
        
        Ok(())
    }
    
    /// Run performance under load test
    pub async fn run_performance_under_load_test(
        &self,
        environment: &TestEnvironment
    ) -> Result<(), TestError> {
        let start_time = Instant::now();
        let mut handles = Vec::new();
        
        // Create load test tasks
        for i in 0..1000 {
            let environment = environment.clone();
            let task = self.create_load_test_task(i);
            
            let handle = tokio::spawn(async move {
                environment.status_core_engine
                    .apply_status_effect(&task.actor_id, task.effect, &task.context)
                    .await
            });
            
            handles.push(handle);
        }
        
        // Wait for all tasks to complete
        let mut success_count = 0;
        for handle in handles {
            match handle.await? {
                Ok(_) => success_count += 1,
                Err(_) => {},
            }
        }
        
        let total_time = start_time.elapsed();
        let throughput = success_count as f64 / total_time.as_secs_f64();
        
        // Verify performance targets
        assert!(throughput >= 1000.0, "Throughput {} is below target 1000", throughput);
        assert!(total_time.as_secs() < 10, "Total time {}s is above target 10s", total_time.as_secs());
        
        Ok(())
    }
}
```

## 🧪 **Test Data Generators**

```rust
/// Test Data Generators
pub struct TestDataGenerators {
    // Status effect generators
    status_effect_generator: StatusEffectGenerator,
    
    // Immunity generators
    immunity_generator: ImmunityGenerator,
    
    // Context generators
    context_generator: ContextGenerator,
    
    // Actor generators
    actor_generator: ActorGenerator,
}

impl TestDataGenerators {
    /// Generate test status effect
    pub fn generate_status_effect(&self) -> StatusEffect {
        self.status_effect_generator.generate()
    }
    
    /// Generate test status effect with specific ID
    pub fn generate_status_effect_with_id(&self, effect_id: &str) -> StatusEffect {
        self.status_effect_generator.generate_with_id(effect_id)
    }
    
    /// Generate test immunity
    pub fn generate_immunity(&self) -> Immunity {
        self.immunity_generator.generate()
    }
    
    /// Generate test status context
    pub fn generate_status_context(&self) -> StatusContext {
        self.context_generator.generate()
    }
    
    /// Generate test actor
    pub fn generate_actor(&self) -> Actor {
        self.actor_generator.generate()
    }
}

/// Status Effect Generator
pub struct StatusEffectGenerator {
    // Generator configuration
    config: GeneratorConfig,
    
    // Random number generator
    rng: ThreadRng,
}

impl StatusEffectGenerator {
    /// Generate random status effect
    pub fn generate(&mut self) -> StatusEffect {
        let effect_id = self.generate_effect_id();
        let effect_name = self.generate_effect_name();
        let effect_name_vi = self.generate_effect_name_vi();
        let category = self.generate_category();
        let effect_type = self.generate_effect_type();
        let magnitude = self.generate_magnitude();
        let duration = self.generate_duration();
        let target = self.generate_target();
        let source = self.generate_source();
        let conditions = self.generate_conditions();
        let interactions = self.generate_interactions();
        let immunity_list = self.generate_immunity_list();
        let movement_restrictions = self.generate_movement_restrictions();
        let visual_effects = self.generate_visual_effects();
        let audio_effects = self.generate_audio_effects();
        let properties = self.generate_properties();
        let priority = self.generate_priority();
        
        StatusEffect {
            effect_id,
            effect_name,
            effect_name_vi,
            category,
            effect_type,
            magnitude,
            duration,
            target,
            source,
            conditions,
            interactions,
            immunity_list,
            movement_restrictions,
            visual_effects,
            audio_effects,
            properties,
            priority,
        }
    }
    
    /// Generate status effect with specific ID
    pub fn generate_with_id(&mut self, effect_id: &str) -> StatusEffect {
        let mut effect = self.generate();
        effect.effect_id = effect_id.to_string();
        effect
    }
    
    /// Generate effect ID
    fn generate_effect_id(&mut self) -> String {
        let effect_ids = vec![
            "burning", "freezing", "poison", "stun", "root", "silence",
            "blind", "charm", "fear", "rage", "blessing", "curse"
        ];
        effect_ids[self.rng.gen_range(0..effect_ids.len())].to_string()
    }
    
    /// Generate effect name
    fn generate_effect_name(&mut self) -> String {
        let effect_names = vec![
            "Burning", "Freezing", "Poison", "Stun", "Root", "Silence",
            "Blind", "Charm", "Fear", "Rage", "Blessing", "Curse"
        ];
        effect_names[self.rng.gen_range(0..effect_names.len())].to_string()
    }
    
    /// Generate effect name in Vietnamese
    fn generate_effect_name_vi(&mut self) -> String {
        let effect_names_vi = vec![
            "Bỏng", "Đóng Băng", "Độc", "Choáng", "Rễ", "Im Lặng",
            "Mù", "Mê Hoặc", "Sợ Hãi", "Phẫn Nộ", "Phước Lành", "Lời Nguyền"
        ];
        effect_names_vi[self.rng.gen_range(0..effect_names_vi.len())].to_string()
    }
    
    /// Generate category
    fn generate_category(&mut self) -> StatusCategory {
        let categories = vec![
            StatusCategory::Debuff,
            StatusCategory::Buff,
            StatusCategory::Control,
            StatusCategory::Transformation,
        ];
        categories[self.rng.gen_range(0..categories.len())].clone()
    }
    
    /// Generate effect type
    fn generate_effect_type(&mut self) -> StatusEffectType {
        let effect_types = vec![
            StatusEffectType::DamageOverTime,
            StatusEffectType::HealOverTime,
            StatusEffectType::StatModifier,
            StatusEffectType::MovementRestriction,
            StatusEffectType::Control,
        ];
        effect_types[self.rng.gen_range(0..effect_types.len())].clone()
    }
    
    /// Generate magnitude
    fn generate_magnitude(&mut self) -> StatusMagnitude {
        let base_value = self.rng.gen_range(10.0..100.0);
        let scaling_factor = self.rng.gen_range(0.1..2.0);
        let min_value = base_value * 0.5;
        let max_value = base_value * 2.0;
        
        StatusMagnitude {
            base_value,
            scaling_factor,
            min_value,
            max_value,
            scaling_stat: "intelligence".to_string(),
        }
    }
    
    /// Generate duration
    fn generate_duration(&mut self) -> StatusDuration {
        let base_duration = Duration::from_secs(self.rng.gen_range(5..30));
        let scaling_factor = self.rng.gen_range(0.1..1.0);
        let min_duration = base_duration / 2;
        let max_duration = base_duration * 2;
        
        StatusDuration {
            base_duration,
            scaling_factor,
            min_duration,
            max_duration,
            scaling_stat: "wisdom".to_string(),
        }
    }
    
    /// Generate target
    fn generate_target(&mut self) -> StatusTarget {
        let targets = vec![
            StatusTarget::Self,
            StatusTarget::Ally,
            StatusTarget::Enemy,
            StatusTarget::All,
        ];
        targets[self.rng.gen_range(0..targets.len())].clone()
    }
    
    /// Generate source
    fn generate_source(&mut self) -> StatusSource {
        let sources = vec![
            StatusSource::Player,
            StatusSource::NPC,
            StatusSource::Environment,
            StatusSource::Item,
        ];
        sources[self.rng.gen_range(0..sources.len())].clone()
    }
    
    /// Generate conditions
    fn generate_conditions(&mut self) -> Vec<StatusCondition> {
        let mut conditions = Vec::new();
        let condition_count = self.rng.gen_range(0..3);
        
        for _ in 0..condition_count {
            let condition = StatusCondition {
                condition_type: StatusConditionType::HealthBelow,
                value: self.rng.gen_range(0.1..0.9),
                operator: StatusConditionOperator::LessThan,
            };
            conditions.push(condition);
        }
        
        conditions
    }
    
    /// Generate interactions
    fn generate_interactions(&mut self) -> Vec<StatusEffectInteraction> {
        let mut interactions = Vec::new();
        let interaction_count = self.rng.gen_range(0..2);
        
        for _ in 0..interaction_count {
            let interaction = StatusEffectInteraction {
                target_effect_id: self.generate_effect_id(),
                interaction_type: StatusInteractionType::Replace,
                probability: self.rng.gen_range(0.1..1.0),
                magnitude_modifier: self.rng.gen_range(0.5..2.0),
                duration_modifier: self.rng.gen_range(0.5..2.0),
            };
            interactions.push(interaction);
        }
        
        interactions
    }
    
    /// Generate immunity list
    fn generate_immunity_list(&mut self) -> Vec<String> {
        let mut immunity_list = Vec::new();
        let immunity_count = self.rng.gen_range(0..2);
        
        for _ in 0..immunity_count {
            immunity_list.push(self.generate_effect_id());
        }
        
        immunity_list
    }
    
    /// Generate movement restrictions
    fn generate_movement_restrictions(&mut self) -> Vec<MovementRestriction> {
        let mut restrictions = Vec::new();
        let restriction_count = self.rng.gen_range(0..2);
        
        for _ in 0..restriction_count {
            let restriction = MovementRestriction {
                restriction_type: MovementRestrictionType::CannotMove,
                magnitude: self.rng.gen_range(0.1..1.0),
                duration: Duration::from_secs(self.rng.gen_range(1..10)),
            };
            restrictions.push(restriction);
        }
        
        restrictions
    }
    
    /// Generate visual effects
    fn generate_visual_effects(&mut self) -> Vec<VisualEffect> {
        let mut visual_effects = Vec::new();
        let effect_count = self.rng.gen_range(0..2);
        
        for _ in 0..effect_count {
            let visual_effect = VisualEffect {
                effect_type: VisualEffectType::Particle,
                color: self.generate_color(),
                intensity: self.rng.gen_range(0.1..1.0),
                duration: Duration::from_secs(self.rng.gen_range(1..10)),
            };
            visual_effects.push(visual_effect);
        }
        
        visual_effects
    }
    
    /// Generate audio effects
    fn generate_audio_effects(&mut self) -> Vec<AudioEffect> {
        let mut audio_effects = Vec::new();
        let effect_count = self.rng.gen_range(0..2);
        
        for _ in 0..effect_count {
            let audio_effect = AudioEffect {
                sound_id: self.generate_sound_id(),
                volume: self.rng.gen_range(0.1..1.0),
                pitch: self.rng.gen_range(0.5..2.0),
                duration: Duration::from_secs(self.rng.gen_range(1..10)),
            };
            audio_effects.push(audio_effect);
        }
        
        audio_effects
    }
    
    /// Generate properties
    fn generate_properties(&mut self) -> HashMap<String, serde_json::Value> {
        let mut properties = HashMap::new();
        let property_count = self.rng.gen_range(0..3);
        
        for _ in 0..property_count {
            let key = format!("property_{}", self.rng.gen_range(0..100));
            let value = serde_json::Value::Number(
                serde_json::Number::from_f64(self.rng.gen_range(0.0..100.0)).unwrap()
            );
            properties.insert(key, value);
        }
        
        properties
    }
    
    /// Generate priority
    fn generate_priority(&mut self) -> u32 {
        self.rng.gen_range(1..10)
    }
    
    /// Generate color
    fn generate_color(&mut self) -> Color {
        Color {
            red: self.rng.gen_range(0..256),
            green: self.rng.gen_range(0..256),
            blue: self.rng.gen_range(0..256),
            alpha: self.rng.gen_range(0..256),
        }
    }
    
    /// Generate sound ID
    fn generate_sound_id(&mut self) -> String {
        let sound_ids = vec![
            "fire_burn", "ice_crack", "poison_bubble", "stun_ring",
            "root_grow", "silence_quiet", "blind_dark", "charm_melody"
        ];
        sound_ids[self.rng.gen_range(0..sound_ids.len())].to_string()
    }
}
```

## 📝 **Implementation Notes**

### **1. Testing Strategy**
- **Test Pyramid**: Unit tests > Integration tests > E2E tests
- **Test Coverage**: 90%+ code coverage
- **Test Isolation**: Tests should be independent
- **Test Reliability**: Tests should be deterministic

### **2. Testing Categories**
- **Unit Testing**: Test individual components
- **Integration Testing**: Test component interactions
- **Performance Testing**: Test performance characteristics
- **End-to-End Testing**: Test complete workflows

### **3. Testing Tools**
- **Rust Testing**: Built-in testing framework
- **Mocking**: Mock external dependencies
- **Property Testing**: Property-based testing
- **Load Testing**: Performance testing tools

---

**Last Updated**: 2025-01-27  
**Version**: 1.0  
**Status**: Design Complete  
**Maintainer**: Chaos World Team
