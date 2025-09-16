# Metal Element

## 📋 Element Overview
- Element ID: `metal`
- Category: `five_elements`
- Classification: Yang — sharpness, resilience

## 🪙 Status Effects

### Brittle (Debuff)

```yaml
# elements/configs/metal_element.yaml
status_effects:
  - name: "brittle"
    type: "debuff"
    base_probability: 0.1
    base_duration: 6.0
    base_intensity: 1.0
    tick_interval: 1.0
    max_stacks: 5
    stackable: true
    refresh_duration: true
    effects:
      defense_reduction: 0.05
    dynamics:
      intensity_gain: 0.014
      intensity_damping: 0.012
      decay_rate: 0.04
      refractory_gain: 0.35
      refractory_decay: 0.1
```

### Same-Element: Metal ↔ Metal

```yaml
same_element_effects:
  - pool_id: "temper_resonance"
    apply_to: "both"
```

### Neutral: Metal ↔ Neutral

```yaml
neutral_effects:
  - pool_id: "brittle_fallback"
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
