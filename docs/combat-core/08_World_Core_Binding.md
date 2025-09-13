# World-core Binding: Area-to-Mode API (Realtime ↔ Turn-based)

## Overview

World-core is the source of truth for combat mode per area/instance. This document specifies the authoritative hooks, payloads, and boundary behavior for attaching actors to Combat Core in Realtime or Turn-based modes, including safe mode switching and cross-border artifact policies.

## Concepts

- **CombatMode**: `realtime` | `turn_based`
- **Area/Instance**: A world-core defined space with a bound `combat_mode`
- **Encounter (TB)**: Shared turn-based instance managing rounds for bound actors
- **Shard (RT)**: Realtime processing partition for an area

## Events (World-core → Combat-core)

- `AreaModeChanged(area_id, mode, version, ts)`
- `ActorEnteredArea(area_id, actor_id, pos, ts)`
- `ActorLeftArea(area_id, actor_id, pos, ts)`

Events are idempotent via `(area_id, version)`. Timestamps are monotonic per area.

## Attach/Detach APIs (Combat-core)

- `attach_actor_realtime(area_id, actor_id, snapshot_ref, seed) -> Ack`
- `attach_actor_turn_based(encounter_id, actor_id, snapshot_ref, seed) -> Ack`
- `detach_actor(actor_id, reason) -> Ack`

`Ack` includes `(ok, version, ts)` for idempotent retries.

## Snapshot Handoff

World-core provides a minimal but deterministic snapshot reference on attach:

```rust
pub struct ActorSnapshotRef {
    pub actor_id: String,
    pub snapshot_id: String,              // reference to Actor Core Snapshot
    pub position: (f64, f64, f64),
    pub orientation: (f64, f64, f64),
    pub resources: std::collections::HashMap<String, f64>,
    pub shields: Vec<String>,
    pub statuses: Vec<String>,
    pub pending_action_rt: Option<String>,         // realtime queued action id (if any)
    pub declared_action_tb: Option<String>,        // turn-based declared action id (if any)
    pub last_tick_id: Option<u64>,
    pub last_turn_id: Option<u64>,
    pub rng_seed: u64,
    pub initiative_seed: Option<u64>,
}
```

## Mode Switching (Boundary Rules)

- Realtime → Turn-based
  - Finish current tick for the actor; close RT queues
  - World-core sends `ActorSnapshotRef`
  - Combat-core enqueues actor at next `TurnStarted`

- Turn-based → Realtime
  - Finish current turn if the actor is acting
  - World-core sends `ActorSnapshotRef`
  - Combat-core attaches actor at next RT tick

### Cross-border Artifact Policy (per area, world-core configured)

| Artifact | Options | Default |
|---------|---------|---------|
| Projectiles | `cancel`, `defer_to_entry`, `resolve_on_entry` | `defer_to_entry` |
| Statuses | `persist`, `remap`, `purge` | `persist` |
| Queued Actions | `execute_before_exit`, `cancel`, `remap_to_wait` | `remap_to_wait` |

## Turn-based Encounter Lifecycle (Combat-core)

- `create_encounter(area_id, seed, caps) -> encounter_id`
- `bind_actor(encounter_id, actor_id) / unbind_actor`
- Emits: `TurnStarted`, `ActorTurnStarted`, `ActorTurnEnded`, `TurnEnded`, `EncounterEnded`

## Realtime Shard Lifecycle (Combat-core)

- `bind_actor_to_shard(area_id, shard_id, actor_id)`
- Tick batching per `configs/server_timing.yaml`
- Ability queue window per `ability_queue_window_ms`

## Initiative & AP (Turn-based)

- Initiative per round: `(speed * 1.0) + (haste * 0.5) + seeded_tiebreaker`
- AP per round: `base_ap_per_round = 1.0` (default)
- AP cost per action: `ap_cost_formula` from `configs/turn_based.yaml` using `duration_s` and `cooldown_s`
- Minor action threshold: `minor_action_threshold_ap`

## Phases & Timeouts

- Input window: `turn_based.input_window_ms` (default 1500 ms)
- AFK timeout: `turn_based.afk_timeout_ms` (default 5000 ms)
- Round time budget: `turn_based.round_time_ms`

## Idempotency & Retries

- All mode/attach/detach messages carry `(version, ts)`
- Retries with same `(area_id, version)` must be safe to ignore (acknowledge duplicates)

## Failure & Backpressure

- If attach fails: remain in current mode until next safe boundary, retry later
- Batch attach/detach supported for mass movements

## Security & Validation

- World-core is authoritative for position/area membership
- Combat-core validates LoS/range and enforces cooldowns per `configs/validation.yaml`
- Audit logs on mode changes and attach/detach

## Configuration References

- `configs/turn_based.yaml` – initiative formula, AP model, phases, events
- `configs/server_timing.yaml` – tick interval, latency compensation, TB defaults
- `configs/protections.yaml` – immunity/cleanse taxonomy for cross-border behaviors
 - `configs/shields.yaml` – global shield semantics; merge precedence supports PvP template and area overrides
 - `configs/pvp_templates.yaml` – per-mode overrides (e.g., shield penetration caps, stack limits)

## Example Sequence: RT → TB transition

1. World-core emits `AreaModeChanged(area_id=X, mode=turn_based, version=N)`
2. Actor crosses boundary: `ActorEnteredArea(area_id=X, actor_id=A)`
3. Combat-core finishes actor A’s current tick; world-core sends `ActorSnapshotRef`
4. Combat-core attaches A to encounter E, queues at next `TurnStarted`
5. A participates in TB rounds until world-core moves A out

---

This binding keeps math, shields, resources, exhaustion identical across modes while only swapping the scheduler and phases, preserving determinism and performance.

## Appendix A: BG3 Parity and Differences

### Parity
- Turn order by initiative with full action resolution per actor.
- Round phases with end-of-round ticks and status duration handling.
- Reaction/interrupt concepts (modeled as sub-steps during the acting unit’s turn).

### Differences
- Dual-mode engine: realtime batching and turn-based lockstep; BG3 is turn-based only.
- Action economy: variable AP derived from action duration/cooldown vs BG3’s 5e-like action economy.
- Alternate kill paths: Lifespan/resource exhaustion and sacrifice true damage beyond HP-only defeat.
- MMO-scale performance: seeded determinism, batching, telemetry budgets for large encounters.

Implication: Designers can tune toward BG3 feel in TB areas by constraining AP to ~1 major action + minor actions and tightening reaction windows, while keeping MMO RT elsewhere.

## Appendix B: Event Payload Schemas (Types/Validation)

### Common Types
```rust
pub type AreaId = String;
pub type ActorId = String;
pub type EncounterId = String;
pub type ShardId = String;

pub enum CombatMode { Realtime, TurnBased }

pub struct Vec3 { pub x: f64, pub y: f64, pub z: f64 }

pub struct TimestampMs(pub u64);
pub struct Version(pub u64);
```

### Events (World-core → Combat-core)
```rust
pub struct AreaModeChanged {
    pub area_id: AreaId,
    pub mode: CombatMode,
    pub version: Version,
    pub ts: TimestampMs,
}

pub struct ActorEnteredArea {
    pub area_id: AreaId,
    pub actor_id: ActorId,
    pub pos: Vec3,
    pub ts: TimestampMs,
}

pub struct ActorLeftArea {
    pub area_id: AreaId,
    pub actor_id: ActorId,
    pub pos: Vec3,
    pub ts: TimestampMs,
}
```

### Attach/Detach Requests (World-core → Combat-core)
```rust
pub struct AttachRealtime {
    pub area_id: AreaId,
    pub actor_id: ActorId,
    pub snapshot: ActorSnapshotRef,
    pub seed: u64,
    pub version: Version,
    pub ts: TimestampMs,
}

pub struct AttachTurnBased {
    pub encounter_id: EncounterId,
    pub actor_id: ActorId,
    pub snapshot: ActorSnapshotRef,
    pub seed: u64,
    pub version: Version,
    pub ts: TimestampMs,
}

pub struct DetachActor {
    pub actor_id: ActorId,
    pub reason: String,
    pub version: Version,
    pub ts: TimestampMs,
}

pub struct Ack { pub ok: bool, pub version: Version, pub ts: TimestampMs }
```

### Validation Rules
- `version`: strictly increasing per area and per actor; duplicates are idempotent.
- `ts`: monotonic per area; reject if clock skew > configured tolerance.
- `pos`: finite numbers; within area bounds (world-core authoritative), else reject.
- `snapshot.snapshot_id`: must exist in Actor Core cache/store; reject otherwise.
- `mode` transitions: only RT→TB or TB→RT; reject redundant transitions.
- Attach during mode switch: queue until boundary; ack with `ok=true` and note `deferred` in log.

### Error Codes (non-exhaustive)
- `ERR_INVALID_VERSION`
- `ERR_SNAPSHOT_NOT_FOUND`
- `ERR_MODE_MISMATCH`
- `ERR_AREA_NOT_FOUND`
- `ERR_ENCOUNTER_CAP_EXCEEDED`
