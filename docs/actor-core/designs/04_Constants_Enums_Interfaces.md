# 04 — Constants, Enums, Interfaces

## Dimension Keys
- Snake_case strings: `strength`, `hp_max`, `crit_rate`, `lifespan_years`, ...

## Buckets (Contribution.bucket)
- `FLAT`: additive before multipliers
- `MULT`: **delta percent**; result ×(1 + value) — factor k ⇒ value = k - 1
- `POST_ADD`: additive after multipliers
- `OVERRIDE`: set absolute value (highest `priority` wins); use sparingly

## Cap Modes (CapContribution.mode)
- `BASELINE`, `ADDITIVE`, `HARD_MAX`, `HARD_MIN`, `OVERRIDE`

## Cap Scopes (layers)
- `scope: string` — one of the configured **layers** (e.g., `REALM`, `WORLD`, `EVENT`, `TOTAL`)
- `realm?: string` — required when `scope="REALM"`
- `tags?: string[]` — optional metadata

## Interfaces (language‑agnostic)
- **Subsystem**
  - `systemId(): string`
  - `priority(): number`
  - `contribute(actor, ctx): SubsystemOutput | Error`
- **CapsProvider**
  - `effectiveCaps(actor, outputs[], layer: string): EffectiveCaps | Error` (merge **within layer**)
- **CapLayerRegistry**
  - `order: string[]` (e.g., `[REALM, WORLD, EVENT, TOTAL]`)
  - `across_policy: 'INTERSECT' | 'PRIORITIZED_OVERRIDE'`
- **CombinerRegistry**
  - `ruleFor(dimension): MergeRule`
- **Aggregator**
  - `resolve(actor): Snapshot | Error`

## Data Contracts
- **Contribution**: `{ dimension, bucket, value, system, priority? }`
- **SubsystemOutput**:
  - `primary: Contribution[]`
  - `derived: Contribution[]`
  - `caps: CapContribution[]`
  - `context?: Record<string, ModifierPack>`
  - `meta: { system, stage?, version? }`
- **CapContribution**:
  - `{ system, dimension, mode, kind: 'max'|'min', value, priority?, scope?, realm?, tags? }`
- **MergeRule**:
  - `{ usePipeline: boolean, operator?: 'SUM'|'MAX'|'MIN', clampDefault: { min, max } }`
- **EffectiveCaps**: `dimension -> { Min, Max }`
- **ModifierPack**: `{ additive_percent?, multipliers?, post_add? }`
