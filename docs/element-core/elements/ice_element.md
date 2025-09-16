# Ice Element

## 📋 Element Overview
- Element ID: `ice`
- Category: `extended_elements`
- Classification: Control/Slow/Freeze

## ❄️ Status Effects

### Chill (Debuff)

```yaml
# elements/configs/ice_element.yaml
status_effects:
  - name: "chill"
    type: "debuff"
    base_probability: 0.13
    base_duration: 6.0
    base_intensity: 1.0
    tick_interval: 1.0
    max_stacks: 5
    stackable: true
    refresh_duration: true
    effects:
      move_speed_slow: 0.06
      skill_speed_slow: 0.05
    dynamics:
      intensity_gain: 0.013
      intensity_damping: 0.012
      decay_rate: 0.05
      refractory_gain: 0.35
      refractory_decay: 0.12
```

### Freeze (Control)
- Đạt được qua conversion từ Chill khi intensity vượt ngưỡng.

```yaml
conversions:
  - pool_id: "freeze_conversion"
```

### Same-Element: Ice ↔ Ice

```yaml
same_element_effects:
  - pool_id: "frost_resonance"
    apply_to: "both"
```

### Neutral: Ice ↔ Neutral

```yaml
neutral_effects:
  - pool_id: "chilled_fallback"
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
- Probability: chill p trong expected_range từ `elements/golden_vectors/ice_golden_vectors.json`
- Conversion: threshold → freeze rồi decay
- Same-element: frost_resonance áp dụng đúng
- Neutral: chilled_fallback kích hoạt khi thiếu cặp
