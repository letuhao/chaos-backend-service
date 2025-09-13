# Resource Exhaustion System

## Overview
The Resource Exhaustion System defines breakpoints and effects that apply when an actor's resources fall to critical thresholds. It operates globally (outside of combat) and inside combat, with deterministic evaluation and event publication.

## Processing Order (Global)
1. Resource values updated (consumption, regeneration, periodic changes)
2. Check exhaustion per resource against thresholds (deterministic order by resource id)
3. Apply or clear effects based on transitions
4. Emit events with coalescing

## Configuration
- Primary config: `docs/combat-core/configs/resource_exhaustion.yaml`
- Per-archetype overrides (e.g., mage/warrior)
- Coalescing window for events

### Schema (excerpt)
```yaml
version: 1
archetypes:
  mage:
    mana:
      thresholds:
        - id: low_mana
          when_percent_lte: 0.10
          effects:
            - disable_tags: ["shield_activation", "buff_activation"]
            - damage_multiplier: { categories: ["magical","elemental","spiritual"], modifier: -0.40 }
            - incoming_multiplier: { categories: ["magical","elemental"], modifier: 0.25 }
            - cast_time_modifier: 0.30
        - id: no_mana
          when_value_eq: 0
          effects:
            - disable_cost_type: ["mana"]
            - break_active_shields: ["MagicShield","ElementalShield","SpiritualShield"]
            - set_flag: { name: "vulnerable_mage", value: true }
```

## Determinism & Hysteresis
- Deterministic evaluation order by resource id then threshold order
- Optional hysteresis to avoid rapid flapping around a threshold (e.g., require 1% recovery before clearing)
- Coalesce repeated transitions within `events.coalesce_window_ms`

## Events
- `ResourceExhaustedEvent { actor_id, resource_type, threshold_id, effects_applied, timestamp }`
- `ResourceRecoveredEvent { actor_id, resource_type, threshold_id, effects_removed, timestamp }`

## Integration Points
- Combat Core: check exhaustion after distribution and before events in the same tick (see `docs/combat-core/04_Damage_Application_System.md`)
- Regeneration: exhaustion modifiers can affect regen rates (optional)
- Action System: exhaustion can disable tags or cost types (e.g., stamina actions)

## Examples
- Mage (mana-centric):
  - `mana <= 10%`: disable shield/buff; -40% magical damage; +25% magical incoming; +30% cast time
  - `mana == 0`: cannot cast mana actions; break mana-based shields; set vulnerable flag
- Warrior (stamina-centric):
  - `stamina <= 15%`: cannot use stamina-cost actions; -30% physical multipliers; -20% move speed; disable block/parry
  - `stamina == 0`: physical action lockout; heavy stagger susceptibility; -50% taunt effectiveness

## Notes
- Lifespan-driven effects are long-term; recovery is expensive (see `docs/combat-core/configs/recovery.yaml`)
- Keep effects idempotent and versioned for safe retries

---

## Formal Threshold Model

### Ordering & Evaluation
- Resources are evaluated in deterministic order: `resource_id` (ASC) → `threshold.order` (ASC) → `threshold.id` (ASC).
- Multiple thresholds can be true simultaneously; the engine applies all whose conditions are met, then resolves effect precedence (see below).

### Hysteresis
- Each threshold supports enter/exit conditions.
  - `enter`: e.g., `when_percent_lte: 0.10`
  - `exit`: e.g., `when_percent_gte: 0.12` (default = `enter + hysteresis_default`)
- Global default hysteresis can be set (e.g., 0.02 = 2%) and overridden per threshold.

### Simultaneous Exhaustion Priority
- Priority by resource category (descending): Health/LifeForce/Lifespan → Qi/Spiritual/Mana/Stamina → Others.
- When multiple thresholds conflict, the highest category priority wins for conflicting fields; non-conflicting effects are merged.

---

## Effect Schema & Precedence

### Effect Kinds
- `disable_tags: [string]` – disables action/system tags (e.g., `shield_activation`)
- `disable_cost_type: [string]` – prevents actions using specified resource types
- `damage_multiplier: { categories: [string], modifier: number }` – multiplicative outgoing modifier
- `incoming_multiplier: { categories: [string], modifier: number }` – multiplicative incoming modifier
- `cast_time_modifier: number` – additive fraction (+0.30 = +30%)
- `move_speed_modifier: number` – additive fraction (-0.20 = -20%)
- `break_active_shields: [ShieldType]` – immediately break listed shields
- `set_flag: { name: string, value: bool }` – set actor flag
- `action_lockout: [string]` – lock specific action families (e.g., `physical`)
- `stagger_susceptibility: enum` – `light|medium|heavy`
- `taunt_effectiveness_modifier: number` – additive fraction
- `regen_modifier: { resource: string, modifier: number }` – multiplicative regen modifier

### Precedence & Combination
- Multipliers combine multiplicatively per category: `final = Π(1 + modifier_i)`.
- Additive modifiers on same field combine additively with clamp if configured.
- `disable_*` and `action_lockout` are OR-combined (any true applies).
- `break_active_shields` is executed once per shield type (idempotent).
- Flags are set last; re-setting to same value is idempotent.

### Validation Rules
- Percentages must be within [-1.0, +10.0] unless override enabled.
- Lists must be unique; duplicates are removed on load.
- Unknown categories/resources/shield types cause load-time warnings or errors (configurable strictness).

---

## Runtime API (Subsystem-facing)

```rust
pub struct ExhaustionEngine { /* ... */ }

impl ExhaustionEngine {
  pub fn evaluate(&self, actor_id: &str, snapshot: &Snapshot) -> Vec<ExhaustionTransition> { /* deterministic */ }
  pub fn apply_effects(&mut self, actor_id: &str, transitions: &[ExhaustionTransition]) -> Result<()> { /* idempotent */ }
  pub fn clear_effects(&mut self, actor_id: &str, transitions: &[ExhaustionTransition]) -> Result<()> { /* idempotent */ }
}

pub struct ExhaustionTransition {
  pub resource: String,
  pub threshold_id: String,
  pub entering: bool,
  pub effects: Vec<Effect>,
}
```

### Idempotency & Coalescing
- Each applied effect is tracked with `(actor_id, resource, threshold_id, version)`.
- Transitions within `events.coalesce_window_ms` are coalesced; only edge transitions emit events.

---

## PvP/PvE Overrides & Areas
- PvP templates and areas may override exhaustion thresholds/effects (e.g., reduce harshness in arena).
- Merge precedence: `pvp_template_overrides → area_overrides → global`.

---

## Extended Configuration Example

```yaml
version: 1
hysteresis_default: 0.02
archetypes:
  warrior:
    stamina:
      thresholds:
        - id: low_stamina
          order: 10
          enter_percent_lte: 0.15
          exit_percent_gte: 0.17
          effects:
            - disable_cost_type: ["stamina"]
            - damage_multiplier: { categories: ["physical"], modifier: -0.30 }
            - move_speed_modifier: -0.20
            - disable_tags: ["parry","block"]
        - id: no_stamina
          order: 20
          enter_value_eq: 0
          effects:
            - action_lockout: ["physical"]
            - stagger_susceptibility: heavy
            - taunt_effectiveness_modifier: -0.50
  mage:
    mana:
      thresholds:
        - id: low_mana
          order: 10
          enter_percent_lte: 0.10
          exit_percent_gte: 0.12
          effects:
            - disable_tags: ["shield_activation","buff_activation"]
            - damage_multiplier: { categories: ["magical","elemental"], modifier: -0.40 }
            - incoming_multiplier: { categories: ["magical","elemental"], modifier: 0.25 }
            - cast_time_modifier: 0.30
        - id: no_mana
          order: 20
          enter_value_eq: 0
          effects:
            - disable_cost_type: ["mana"]
            - break_active_shields: ["MagicShield","ElementalShield","SpiritualShield"]
            - set_flag: { name: "vulnerable_mage", value: true }
```

---

## Testing Guidance
- Golden vectors: craft inputs crossing thresholds (enter/exit) with hysteresis to validate determinism.
- Load/area/template overrides: verify precedence and merged outcomes.
- Performance: measure evaluation cost per actor; keep under microsecond budgets where possible.

---

## Resolution Algorithm (Overrides)

```pseudo
function resolve_config(actor_archetype, area_ctx, pvp_template):
  cfg_global = load(global_config)
  cfg_area   = area_ctx.exhaustion_overrides or {}
  cfg_pvp    = pvp_template.exhaustion_overrides or {}

  # deep merge order: global ← area ← pvp
  cfg_merged = deep_merge(cfg_global, cfg_area)
  cfg_merged = deep_merge(cfg_merged, cfg_pvp)

  # validate merged
  validate(cfg_merged, resource_exhaustion.schema.json)

  return cfg_merged

function deep_merge(dst, src):
  for each key in src:
    if key not in dst: dst[key] = src[key]
    else if both are maps: deep_merge(dst[key], src[key])
    else: dst[key] = src[key]  # scalars/lists replace by default
  return dst
```

Notes:
- Threshold `order` is preserved; duplicates by `id` within the same resource replace by last writer (area/pvp).
- Lists replace by default; if additive behavior is required, mark and handle in loader.

---

## Event Payload Schemas (JSON)

```json
{
  "ResourceExhaustedEvent": {
    "actor_id": "string",
    "resource_type": "string",
    "threshold_id": "string",
    "effects_applied": ["EffectSpec"],
    "timestamp": "number",
    "idempotency_key": "string"
  },
  "ResourceRecoveredEvent": {
    "actor_id": "string",
    "resource_type": "string",
    "threshold_id": "string",
    "effects_removed": ["EffectSpec"],
    "timestamp": "number",
    "idempotency_key": "string"
  }
}
```

Idempotency key suggestion: `hash(actor_id, resource_type, threshold_id, edge, window_bucket)`.

Telemetry fields (optional): `coalesced: bool`, `hysteresis_enter: number`, `hysteresis_exit: number`.

---

## Canonical Tags & Cost Types

- Action tags (disable_tags): `shield_activation`, `buff_activation`, `parry`, `block`, `cast`, `sprint`
- Cost types (disable_cost_type): `mana`, `stamina`, `qi`, `spiritual_energy`, `lifeforce`
- Categories (multipliers): `physical`, `magical`, `elemental`, `spiritual`, `qi`, `all`

Keep tag lists centralized to avoid drift; unknown entries trigger warnings in validator (strict mode: error).

---

## Validation & CLI

Validate configs against the JSON Schema:

```bash
# Using ajv-cli (Node)
npm i -g ajv-cli
ajv validate -s docs/resource-manager/configs/resource_exhaustion.schema.json \
  -d docs/combat-core/configs/resource_exhaustion.yaml --spec=draft2020 --strict=false
```

Checklist:
- Unique threshold ids per resource
- Exactly one enter condition; exit not lower than enter
- Recognized effect types and fields present
- Values in safe ranges (unless overrides)

---

## Performance Targets

- Budget: ≤ 1–3 μs per actor per evaluation on hot path
- Complexity: O(#resources + #thresholds)
- Hints: pre-sort thresholds per resource; cache last known edge to fast-path no-change frames

---

## YAML Schema Specification (authoritative)

```yaml
# docs/combat-core/configs/resource_exhaustion.yaml
version: 1                      # integer, required
hysteresis_default: 0.02        # number in [0, 1], optional (default 0.0)
events:
  coalesce_window_ms: 200       # integer >= 0, optional (default 0)

# Optional global priorities when simultaneous thresholds fire
priorities:
  categories: ["health","lifeforce","lifespan","qi","spiritual","mana","stamina","other"]

archetypes:                     # map<string, ArchetypeSpec>
  warrior:
    stamina:                    # map<string, ResourceSpec> — resource key
      thresholds:               # list<ThresholdSpec>
        - id: low_stamina       # string, required, unique per resource
          order: 10             # integer, optional; default 0; ascending applied
          # Enter/Exit conditions (percent or absolute)
          enter_percent_lte: 0.15      # number in [0, 1]
          exit_percent_gte: 0.17       # number in [0, 1]; default enter + hysteresis_default
          # Alternatively, absolute values
          # enter_value_eq: 0
          # exit_value_ge: 1
          effects:              # list<EffectSpec>
            - type: disable_cost_type
              values: ["stamina"]
            - type: damage_multiplier
              categories: ["physical"]
              modifier: -0.30
            - type: move_speed_modifier
              modifier: -0.20
            - type: disable_tags
              values: ["parry","block"]

        - id: no_stamina
          order: 20
          enter_value_eq: 0
          effects:
            - type: action_lockout
              values: ["physical"]
            - type: stagger_susceptibility
              level: heavy             # enum: light|medium|heavy
            - type: taunt_effectiveness_modifier
              modifier: -0.50

  mage:
    mana:
      thresholds:
        - id: low_mana
          order: 10
          enter_percent_lte: 0.10
          exit_percent_gte: 0.12
          effects:
            - type: disable_tags
              values: ["shield_activation","buff_activation"]
            - type: damage_multiplier
              categories: ["magical","elemental"]
              modifier: -0.40
            - type: incoming_multiplier
              categories: ["magical","elemental"]
              modifier: 0.25
            - type: cast_time_modifier
              modifier: 0.30

        - id: no_mana
          order: 20
          enter_value_eq: 0
          effects:
            - type: disable_cost_type
              values: ["mana"]
            - type: break_active_shields
              values: ["MagicShield","ElementalShield","SpiritualShield"]
            - type: set_flag
              name: vulnerable_mage
              value: true
```

### EffectSpec (normalized)
- `type` (string, required): one of
  - `disable_tags` (values: list<string>)
  - `disable_cost_type` (values: list<string>)
  - `damage_multiplier` (categories: list<string>, modifier: number)
  - `incoming_multiplier` (categories: list<string>, modifier: number)
  - `cast_time_modifier` (modifier: number)
  - `move_speed_modifier` (modifier: number)
  - `break_active_shields` (values: list<ShieldType>)
  - `set_flag` (name: string, value: bool)
  - `action_lockout` (values: list<string>)
  - `stagger_susceptibility` (level: enum[light|medium|heavy])
  - `taunt_effectiveness_modifier` (modifier: number)
  - `regen_modifier` (resource: string, modifier: number)

---

## Validation Checklist

- Top-level
  - `version` present and integer
  - `hysteresis_default` ∈ [0,1]
  - `events.coalesce_window_ms` ≥ 0

- Thresholds
  - Each threshold has unique `id` within its resource
  - Exactly one enter condition provided: (`enter_percent_lte` | `enter_value_eq`)
  - Exit condition optional; if provided, `exit_percent_gte ≥ enter_percent_lte` or `exit_value_ge ≥ enter_value_eq`
  - `order` non-negative integer

- Effects
  - `type` is recognized; required fields present per type
  - Multipliers within safe ranges (e.g., `[-1.0, +10.0]`) unless overrides enabled
  - Lists deduplicated; unknown categories/resources/shield types flagged

- Overrides
  - Precedence respected: `pvp_template_overrides → area_overrides → global`
  - Final merged config passes the same validation

- Determinism
  - Evaluation order is stable across runs for identical inputs
  - Coalescing window prevents event spam near thresholds

---

## Minimal Runtime Contracts

```rust
pub trait ExhaustionProvider {
  fn evaluate(&self, actor_id: &str, snapshot: &Snapshot) -> Vec<ExhaustionTransition>;
  fn apply(&mut self, actor_id: &str, transitions: &[ExhaustionTransition]) -> Result<(), ExhaustionError>;
  fn clear(&mut self, actor_id: &str, transitions: &[ExhaustionTransition]) -> Result<(), ExhaustionError>;
}

pub enum ExhaustionError { InvalidConfig, UnknownResource, StorageError(String) }

pub struct IdempotencyKey { pub actor_id: String, pub resource: String, pub threshold_id: String, pub version: u64 }
```
