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
