# Exhaustion System - Implementation Plan

## Phases & Tasks

### Phase 1: Validation + CI
- Add JSON Schema validation (ajv) for `resource_exhaustion.yaml`.
- CI step: run schema validation on PRs; fail on errors.
- Golden-vector runner for `case05` and `case06`.

### Phase 2: Loader/Merger + Debug Snapshot
- Implement deep-merge (global ← area ← PvP) per spec.
- Provide CLI/API to output `merged_final_config` for an actor.
- Annotate overridden keys with source (global/area/pvp) in debug output.

### Phase 3: API Endpoints/Hooks
- `evaluate(actor_id, snapshot) -> Vec<ExhaustionTransition>`.
- `apply(actor_id, transitions) -> Result<()>`.
- `clear(actor_id, transitions) -> Result<()>`.
- Publish `ResourceExhaustedEvent`/`ResourceRecoveredEvent` with idempotency keys.

### Phase 4: Canonical Enums + Cross-links
- Centralize action tags, cost types, categories (enums file).
- Reference from schema and validate unknowns.
- Add "Exhaustion" pointers in combat docs and action docs (disable-able tags/costs).

### Phase 5: More Golden Vectors
- `case07`: multi-threshold same resource.
- `case08`: hysteresis edge (10%↔12%).

### Phase 6: Telemetry + Performance
- Counters: exhausted/recovered per resource, coalescing hits, top thresholds.
- Pre-sort thresholds per resource; cache last edge per actor.
- Target ≤ 1–3 μs per actor evaluation.

## Acceptance Criteria
- CI validates schema; runner passes case05/06.
- Merged_final_config endpoint/CLI shows precedence and source of overrides.
- Events carry idempotency keys; telemetry fields present.
- Enums enforced; cross-doc references added.
- New golden vectors pass; performance within budget.
