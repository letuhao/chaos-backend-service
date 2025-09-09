# 08 — Combiner Registry

Example YAML:
```yaml
dimensions:
  strength:            { use_pipeline: true,  clamp_default: { min: 0, max: 999999 } }
  hp_max:              { use_pipeline: true,  clamp_default: { min: 1, max: 2000000 } }
  crit_rate:           { use_pipeline: true,  clamp_default: { min: 0, max: 1 } }
  cooldown_reduction:  { use_pipeline: true,  clamp_default: { min: 0, max: 0.5 } }
  poise_rank:          { use_pipeline: false, operator: MAX, clamp_default: { min: 0, max: 10 } }
```

### Operator Mode
- When `use_pipeline: false`, the Aggregator uses `operator` to merge raw contribution values for that dimension.
- Supported: `SUM | MAX | MIN | AVERAGE | MULTIPLY | INTERSECT`.
- Result clamping precedence: EffectiveCaps → `clamp_default` → constants clamp ranges.

### Pipeline Mode and Buckets
- Within-bucket stable sort: priority DESC, then system ASC (deterministic).
- OVERRIDE tie-break: highest priority wins, then system ASC.
- MULT is treated as a multiplicative factor (e.g., 1.10 = +10%).
