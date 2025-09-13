# Damage Application Engine

## Overview
Describes the engine orchestration: shield processing, resource distribution, exhaustion checks, and event emission.

## Flow
1. Process Shields (see `09_Shields_and_Protections.md`)
2. Distribute Damage to Resources (see `10_Resource_Damage_Distribution.md`)
3. Apply Resource Protections (config-driven)
4. Update Resource Values and Check Exhaustion (see `resource_exhaustion.yaml`)
5. Emit Events (DamageApplied, ShieldBroken, ResourceDepleted, ResourceExhausted/Recovered)

## Batch & Performance
- Batch per target shard (realtime) or per encounter round (turn-based)
- Reuse caches; avoid allocations in hot paths; measure p95/p99 against budgets

## Caching & Replays
- Optional result caching for repeated patterns (short TTL)
- Idempotent event emission via idempotency keys

## Reference: Engine Internals

```rust
// Main damage application engine
pub struct DamageApplicationEngine {
    shield_processor: Arc<ShieldOrderProcessor>,
    resource_distributor: Arc<ResourceDamageDistributor>,
    protection_system: Arc<ResourceProtectionSystem>,
    event_system: Arc<EventSystem>,
}

impl DamageApplicationEngine {
    /// Apply damage to actor
    pub async fn apply_damage(
        &mut self,
        actor_id: &str,
        damage: &mut DamageResult,
    ) -> ActorCoreResult<DamageApplicationResult> {
        let start_time = current_timestamp();
        let shield_result = self.shield_processor.process_damage_through_shields(actor_id, damage).await?;
        let protection_result = self.protection_system.apply_resource_protections(actor_id, damage).await?;
        let resource_result = self.resource_distributor.distribute_damage_with_protection(actor_id, damage, &protection_result).await?;
        self.handle_depletion_effects(actor_id, &resource_result).await?;
        self.generate_damage_events(actor_id, damage, &shield_result, &resource_result, &protection_result).await?;
        let processing_time = current_timestamp() - start_time;
        Ok(DamageApplicationResult { actor_id: actor_id.to_string(), original_damage: damage.final_damage + shield_result.total_absorbed + resource_result.total_damage_applied, final_damage: damage.final_damage, shield_result, resource_result, protection_result, processing_time, timestamp: current_timestamp(), })
    }
}
```

## Reference: Event Structures

```rust
#[derive(Debug, Clone)]
pub struct DamageAppliedEvent { pub actor_id: String, pub damage: DamageResult, pub shield_result: ShieldProcessingResult, pub resource_result: ResourceDamageResult, pub timestamp: u64 }
#[derive(Debug, Clone)]
pub struct ShieldBrokenEvent { pub actor_id: String, pub shield_id: String, pub timestamp: u64 }
#[derive(Debug, Clone)]
pub struct ResourceDepletedEvent { pub actor_id: String, pub resource_id: String, pub timestamp: u64 }
#[derive(Debug, Clone)]
pub struct ResourceExhaustedEvent { pub actor_id: String, pub resource_type: ResourceType, pub threshold_id: String, pub effects_applied: Vec<String>, pub timestamp: u64 }
#[derive(Debug, Clone)]
pub struct ResourceRecoveredEvent { pub actor_id: String, pub resource_type: ResourceType, pub threshold_id: String, pub effects_removed: Vec<String>, pub timestamp: u64 }
```
