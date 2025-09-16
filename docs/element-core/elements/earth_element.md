# Earth Element

## 📋 Element Overview
- Element ID: `earth`
- Category: `five_elements`
- Classification: Yin–balanced — stability, fortification

## 🪨 Status Effects

### Quagmire (Debuff)

```yaml
# elements/configs/earth_element.yaml
status_effects:
  - name: "quagmire"
    type: "debuff"
    base_probability: 0.1
    base_duration: 6.0
    base_intensity: 1.0
    tick_interval: 1.0
    max_stacks: 5
    stackable: true
    refresh_duration: true
    effects:
      move_speed_slow: 0.05
      accurate_rate_debuff: 0.02
    dynamics:
      intensity_gain: 0.015
      intensity_damping: 0.012
      decay_rate: 0.05
      refractory_gain: 0.35
      refractory_decay: 0.1
```

### Same-Element: Earth ↔ Earth

```yaml
same_element_effects:
  - pool_id: "stone_resonance"
    apply_to: "both"
```

### Neutral: Earth ↔ Neutral

```yaml
neutral_effects:
  - pool_id: "quagmire_fallback"
    apply_to: "defender"
    probability:
      base: "from_element"
      use_probability_engine: true
      scaling_factor_key: "status_probability"
    dynamics_override: {}
```

## 🔗 References
- `configs/status_pool.yaml`
- `configs/interaction_config.yaml`
- `configs/probability_config.yaml`
- `11_Advanced_Derived_Stats_Design.md`
