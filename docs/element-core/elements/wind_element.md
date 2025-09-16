# Wind Element

## 📋 Element Overview
- Element ID: `wind`
- Category: `extended_elements`
- Classification: Control/Mobility, displacement, turbulence

## 🌪️ Status Effects

### Shear (Debuff)

```yaml
# elements/configs/wind_element.yaml
status_effects:
  - name: "shear"
    type: "debuff"
    base_probability: 0.12
    base_duration: 5.0
    base_intensity: 1.0
    tick_interval: 1.0
    max_stacks: 5
    stackable: true
    refresh_duration: true
    effects:
      accurate_rate_debuff: 0.04
      defense_point_debuff: 0.03
    dynamics:
      intensity_gain: 0.014
      intensity_damping: 0.012
      decay_rate: 0.05
      refractory_gain: 0.35
      refractory_decay: 0.12
```

### Same-Element: Wind ↔ Wind

```yaml
same_element_effects:
  - pool_id: "gale_guard"
    apply_to: "both"
  - pool_id: "turbulence_aura"
    apply_to: "defender"
```

### Neutral: Wind ↔ Neutral

```yaml
neutral_effects:
  - pool_id: "shear_fallback"
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

## 🧪 Minimal Test Checklist
- Probability: shear p trong expected_range từ `elements/golden_vectors/wind_golden_vectors.json`
- Dynamics: Δ↑ → intensity↑; refractory giảm spam
- Same-element: gale_guard/turbulence_aura áp dụng đúng
- Neutral: shear_fallback kích hoạt khi thiếu cặp
