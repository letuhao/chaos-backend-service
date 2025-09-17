# 🔥 Burning Status Combat System — Review & Improvement Notes

## ✅ Strengths
- **Modular & Clear Flow**: ActionCore → CombatCore → StatusCore → ElementCore → ResourceManager. Easy to trace and debug.
- **Deterministic & Balanced**: SPEC v2.0 provides Composition Law, clamps, and invariants → ensures consistent results across runs.
- **Extensible Interfaces**: Supports registering new Damage, Defence, Amplifier types without breaking the system.
- **Complete Status Lifecycle**: Apply → Validate → Store → Tick Process → Expiry & Cleanup, including EventDispatcher.
- **Testing Guidance**: Golden tests, property tests, stacking rules, clamp ranges already defined.

## ⚠️ Risks & Areas to Improve

### 1. Performance & Tick Processing
- Current per-actor iteration may not scale with thousands of active effects.
- **Improve**: Use **time wheel / min-heap** based on `expires_at` / `next_tick_at` to only process due effects.
- Consider **sharded queues** by actor/combat instance.

### 2. Concurrency & Data Contention
- Global `RwLock` on `active_effects` may create contention under load.
- **Improve**: Use **sharded locks**, ECS-style storage, or per-actor containers.

### 3. Effect Idempotency & Race Conditions
- Risk of double-apply or remove conflicts if multiple ticks overlap.
- **Improve**: Define **effect keys** (actorId, effectId, sourceId, instanceId). Use **CAS/versioning** to ensure exact-once semantics.

### 4. Stacking & Refresh Rules
- Non-combat effects have stacking rules; combat effects (Burning, Poison, etc.) need explicit policy.
- **Improve**: Add **stacking law**: {REFRESH, EXTEND, ADD_STACK, OVERRIDE}, `stackId`, `maxStacks`, `priority`.

### 5. Damage & Resistance Consistency
- Duplicate methods (`get_mastery_bonus` vs `get_fire_mastery_bonus`).
- Unclear if DoT damage is resisted per tick or only on apply.
- **Improve**: Standardize **ElementCore::get_mastery_bonus(element)**. Define **DoT resist law**.

### 6. Transactional Safety
- Resource consumption happens before status/damage apply. Failures may desync.
- **Improve**: Introduce **reserve/commit/rollback** semantics.

### 7. Timing & Tickrate
- Need fixed tickrate (e.g., 50–100 ms) with lag compensation.
- **Improve**: Use **server-authoritative clock**, drift detection, resync.

### 8. Observability
- Limited current logging.
- **Improve**: Add metrics & structured logs:
  - Counters: `status_active_total`, `status_apply/sec`, `status_expired/sec`
  - Timers: `tick_duration_ms`
  - Traces: correlate Action → Combat → Status → Resource events

### 9. Testing & Fuzzing
- SPEC covers golden tests but not fuzz/soak tests at scale.
- **Improve**: Add **fuzz tests** for expiry/stacking boundaries and **soak tests** for 100k+ effects.

### 10. Data Model & Snapshotting
- Multiple time fields (applied_at, expires_at) risk desync.
- **Improve**: Use **monotonic tick index** or unified epoch time. Define snapshot schema for save/load/rollback.

## 📐 Suggested Laws (Standardization)
- **Apply Law**:  
  `magnitude_final = base × scaling_by_stats × amplifiers.status × element_mastery`
- **Tick Law**:  
  `tick_damage = magnitude_final × tick_scale × (1 - resist × (1 - piercing))`
- **Stacking**: explicit `mode`, `stackId`, `maxStacks`, `priority`
- **Expiry**: exact deadline, ±1 tick grace, exact-once removal event
- **Crit & DoT**: recommend **non-crit DoT** (simpler). If crit allowed, must define when & cap.

## 🚀 Quick Wins
1. Implement **priority queue/time wheel** for status scheduling.  
2. Add **combat stacking rules** for DoTs.  
3. Shard status storage & lock.  
4. Fix tickrate + lag comp.  
5. Add observability pack (metrics, logs, traces).  
6. Property & fuzz tests for expiry/stacking.  
7. Reserve/commit/rollback for resources + effects.

---
**Status**: Architecture solid; ready for scaling with the improvements above.  
**Maintainer Note**: Validate changes against Actor Core SPEC v2.0 invariants【30†SPEC.md】.
