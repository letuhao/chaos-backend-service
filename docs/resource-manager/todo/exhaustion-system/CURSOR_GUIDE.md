# Cursor AI Guide — Complete the Resource Exhaustion System

This guide lists concrete, incremental edits to finish the Exhaustion System according to the spec in `docs/resource-manager/08_Resource_Exhaustion_System.md`.

References
- Design: `docs/resource-manager/08_Resource_Exhaustion_System.md`
- Schema: `docs/resource-manager/configs/resource_exhaustion.schema.json`
- Config: `docs/combat-core/configs/resource_exhaustion.yaml`
- Golden vectors: `docs/resource-manager/golden_vectors/case05_*`, `case06_*`

## 1) Config Loader — Hysteresis & Merge

File: `crates/actor-core/src/subsystems/exhaustion/exhaustion_config_loader.rs`

- After deep-merge and before validation completes, if a threshold has:
  - `enter_percent_lte` set and `exit_percent_gte` missing, set:
    - `exit_percent_gte = (enter_percent_lte + hysteresis_default).min(1.0)`
  - `enter_value_eq` set and `exit_value_ge` missing, set:
    - `exit_value_ge = enter_value_eq + 1` (or a config-driven minimal step)
- Keep existing validation: exactly one enter condition; exit ≥ enter
- Ensure precedence order in the loader call path: `global ← area ← PvP`
- Keep thresholds sorted by `order` (already implemented)

Suggested helper:
```rust
fn fill_hysteresis_defaults(cfg: &mut ExhaustionConfig) {
    let h = cfg.hysteresis_default;
    for (_, arch) in &mut cfg.archetypes {
        for (_, res) in &mut arch.resources {
            for th in &mut res.thresholds {
                if let Some(enter_p) = th.enter_percent_lte {
                    if th.exit_percent_gte.is_none() {
                        th.exit_percent_gte = Some((enter_p + h).min(1.0));
                    }
                }
                if let Some(enter_v) = th.enter_value_eq {
                    if th.exit_value_ge.is_none() {
                        th.exit_value_ge = Some(enter_v + 1.0);
                    }
                }
            }
        }
    }
}
```
Call `fill_hysteresis_defaults` inside `deep_merge_configs` before `validate_config`.

## 2) Event Publisher — Idempotency & Coalescing

File: `crates/actor-core/src/subsystems/exhaustion/exhaustion_event_publisher.rs`

- Add coalescing and idempotency key per `08_Resource_Exhaustion_System.md`.
- Maintain `last_emit: HashMap<String, u64>` keyed by `hash(actor_id, resource, threshold_id, edge)`.
- If `now - last_emit[key] < coalesce_window_ms`, emit a coalesced marker (or skip publishing) and update telemetry.
- Include telemetry fields: `coalesced`, `hysteresis_enter`, `hysteresis_exit` (if available from config/transition).

Signature idea:
```rust
impl ExhaustionEventPublisher for InMemoryEventPublisher {
    fn publish(&self, evt: &ExhaustionEvent) -> Result<(), ExhaustionError> { /* coalesce & publish */ }
}
```

## 3) Engine API — evaluate/apply/clear

File: `crates/actor-core/src/subsystems/exhaustion/resource_exhaustion.rs`

- Expose public API on the subsystem/engine:
```rust
pub trait ExhaustionProvider {
    fn evaluate(&self, actor_id: &str, snapshot: &Snapshot) -> Vec<ExhaustionTransition>;
    fn apply(&mut self, actor_id: &str, transitions: &[ExhaustionTransition]) -> Result<(), ExhaustionError>;
    fn clear(&mut self, actor_id: &str, transitions: &[ExhaustionTransition]) -> Result<(), ExhaustionError>;
}
```
- Store applied effects (for idempotency and clear) using a versioned key:
  - `(actor_id, resource, threshold_id, version)` in a map/set
- Ensure deterministic evaluation order and respect hysteresis (enter/exit logic)

## 4) Debug — Merged Final Config Snapshot

File: `crates/actor-core/src/subsystems/exhaustion/exhaustion_config_loader.rs`

- Add a debug util:
```rust
impl ExhaustionConfigLoader {
    pub fn pretty_print_merged(&self, merged: &MergedConfig) -> String { /* include sources */ }
}
```
- Optional: add a CLI flag to `crates/actor-core/examples/exhaustion_cli.rs` to print merged for an actor/area/pvp template

## 5) Canonical Enums

File: `docs/resource-manager/configs/enums.yaml`
```yaml
action_tags: [shield_activation, buff_activation, parry, block, cast, sprint]
cost_types: [mana, stamina, qi, spiritual_energy, lifeforce]
categories: [physical, magical, elemental, spiritual, qi, all]
```
- Loader/validator: warn (or error, strict mode) on unknown items

## 6) Tests — Hysteresis & Precedence

File: `crates/actor-core/tests/exhaustion_system_tests.rs`

- Add tests mirroring `case05` and `case06` timelines:
  - assert first exhausted event triggers at enter; recovered at exit
  - assert coalescing within window suppresses duplicates
  - assert merged precedence (PvP > area > global) yields expected exit thresholds/effects

## 7) CI — Schema & Golden Runner

- Add workflow: `.github/workflows/validate-exhaustion.yml` (AJV validation)
- Add optional step: run Node runner in `docs/resource-manager/golden_vectors/runner` if present

## 8) Performance

- Pre-sort thresholds per resource at load time
- Cache last edge per `(actor, resource, threshold)` to short-circuit unchanged frames
- Target ≤ 1–3 μs per actor evaluation

---

When implementing, keep code deterministic, idempotent, and instrumented (counters for exhausted/recovered, coalescing hits, top thresholds).
