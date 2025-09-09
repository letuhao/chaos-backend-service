# Resource Manager — Real-World MMORPG Patterns (Integrates with Actor Core v3)

## Goals
- Deterministic, server-authoritative resources with predictable stacking.
- Integrate cleanly with Actor Core v3 (Subsystem → Contributions/Caps → Snapshot).
- Provide transactional consumption, robust regen, and layered rules (realm/world/event/total).

## Architectural shape (Actor Core integration)
- Subsystem implementation: `ResourceManagerSubsystem` implements `actor_core::interfaces::Subsystem`.
  - `contribute(actor) -> SubsystemOutput`:
    - Primary/derived resource contributions per dimension.
    - Cap contributions scoped by layers (realm/world/event/total).
  - Use `CombinerRegistry` to choose pipeline or operator per dimension.
  - Use `CapLayerRegistry` for layered cap reduction.
- Snapshot fields: expose `*_current`, `*_max`, `*_regen`, and derived `*_percentage` as needed.
- Versioning: all external mutations (consume/restore) must provide `snapshot_version` precondition.

## Resource taxonomy (dimension rules table skeleton)
Fill this table for each resource you ship.

```
| dimension         | type        | mode       | clamp_default     | layers           | deps                       | events                         |
|-------------------|------------ |----------- |------------------ |----------------- |--------------------------- |------------------------------- |
| hp_current        | primary     | pipeline   | {min:0, max:inf}  | realm,world,total| depends on hp_max          | Changed, Depleted, Thresholds  |
| hp_max            | primary     | pipeline   | {min:1, max:2e6}  | realm,world,total| base from vitality, level  | Changed                         |
| hp_regen          | primary     | pipeline   | {min:0, max:1e5}  | realm,world,total| gated by in_combat         | Changed                         |
| mana_current      | primary     | pipeline   | {min:0, max:inf}  | realm,world,total| depends on mana_max        | Changed, Depleted              |
| mana_max          | primary     | pipeline   | {min:1, max:2e6}  | realm,world,total| base from intelligence     | Changed                         |
| mana_regen        | primary     | pipeline   | {min:0, max:1e5}  | realm,world,total| OOC gate by combat status  | Changed                         |
| stamina_current   | primary     | pipeline   | {min:0, max:inf}  | realm,world,total| move/combat gating         | Changed, Depleted              |
| stamina_max       | primary     | pipeline   | {min:1, max:2e6}  | realm,world,total| base from vitality/gear    | Changed                         |
| stamina_regen     | primary     | pipeline   | {min:0, max:1e5}  | realm,world,total| in_combat penalty          | Changed                         |
| shield_current    | primary     | pipeline   | {min:0, max:inf}  | realm,world,total| decay policy               | Changed, Depleted              |
| shield_max        | primary     | pipeline   | {min:0, max:2e6}  | realm,world,total| buff-based/temporary       | Changed                         |
| hp_percentage     | derived     | operator   | {min:0, max:1}    | -                | hp_current / hp_max        | Changed, Thresholds            |
```
- mode: `pipeline` (FLAT → MULT → POST_ADD → OVERRIDE) or `operator` (SUM|MAX|MIN|AVG|MULTIPLY|INTERSECT).
- MULT semantics: multiplicative factor (1.10 = +10%).
- layers: which layers can cap this dimension.

## Transactional consumption API
- Idempotent operations with version guard.
- Suggested endpoints (server-internal or service API):
  - `consume_resource(actor_id, resource_id, amount, snapshot_version, idempotency_key)`
  - `restore_resource(actor_id, resource_id, amount, snapshot_version, idempotency_key)`
- Guarantees:
  - If `snapshot_version` mismatches → conflict; client refreshes snapshot and retries.
  - `idempotency_key` ensures safe retries.
  - Optionally provide `reserve → commit/rollback` for multi-resource atomic costs.

## Regen and time model
- Single authoritative time source (server monotonic). No client timers.
- Modes: continuous (per frame dt), tick (1s cadence), conditional (OOC only), event-driven.
- Offline regen: per-resource policy with maximum catch-up window; safe zones only.
- Ordering per tick: damage → shields → hp → death checks → regen → percentages → events.

## Dependencies and conflicts
- Explicit dependency DAG: compute `*_max` first, `*_current` clamps, then derived like `*_percentage`.
- Conflicts/overrides: tie-break by priority DESC, then system ID ASC; log override events for tuning.

## Events and batching
- Emit domain events: `ResourceChanged`, `ResourceDepleted`, `ResourceRefilled`, `ThresholdCrossed(<x%)`.
- Debounce and batch per actor per tick to avoid event spam.
- Consumers (Combat/AI/UI) subscribe rather than polling.

## Persistence and recovery
- Persist deltas (consumption/restores) with `idempotency_key` and `cause` (skill_id/item_id).
- Periodic snapshot; on restart, replay deltas since snapshot and apply bounded offline regen.

## Performance & caching blueprint
- L1: per-actor hot map (in-process), L2: memory-mapped per-zone/realm, L3: disk/Redis for cold actors.
- Invalidate by snapshot version; cache keys `actor_id:version`.
- Tick budget per actor; spillover to next frame if exceeded.

## Testing plan
- Golden vectors:
  - Simultaneous damage + heal; shield before hp; threshold events.
  - Multiple OVERRIDEs and priority ties.
  - OOC regen resumes after grace period; offline regen bounded.
- Property tests (examples): current ≤ max; non-negative where required; ordering invariance.

## Configuration mapping
- Combiner rules: define per-dimension `use_pipeline`/`operator`, clamp defaults.
- Cap layer registry: REALM/WORLD/EVENT/TOTAL with `INTERSECT` policy by default.
- Ship example YAMLs and load through `ACTOR_CORE_CONFIG_DIR` (already supported).

## Minimal subsystem sketch (Rust)
```rust
use actor_core::interfaces::Subsystem;
use actor_core::types::{SubsystemOutput, Contribution};
use actor_core::enums::Bucket;
use actor_core::ActorCoreResult;

pub struct ResourceManagerSubsystem { system_id: String, priority: i64 }

#[async_trait::async_trait]
impl Subsystem for ResourceManagerSubsystem {
    fn system_id(&self) -> &str { &self.system_id }
    fn priority(&self) -> i64 { self.priority }
    async fn contribute(&self, actor: &actor_core::types::Actor) -> ActorCoreResult<SubsystemOutput> {
        let mut out = SubsystemOutput::new(self.system_id.clone());
        // Example: compute hp_max and hp_current
        let base_hp = 100.0; // compute_base_hp(actor)
        out.add_primary(Contribution::new("hp_max".into(), Bucket::Flat, base_hp, self.system_id.clone()));
        out.add_primary(Contribution::new("hp_current".into(), Bucket::Flat, base_hp, self.system_id.clone()));
        Ok(out)
    }
}
```

## Rollout checklist
- Finalize per-dimension rules table.
- Add transactional consume/restore handlers with idempotency.
- Implement regen scheduler using server monotonic time.
- Add event pipeline and debouncing.
- Wire persistence (deltas + periodic snapshots).
- Add golden vectors and property tests; validate with Actor Core aggregator.
