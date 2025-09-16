# Lightning Element

## 📋 Element Overview
- Element ID: `lightning`
- Category: `extended_elements`
- Classification: Burst/Chain, risk–reward control

## ⚡ Status Effects

### Shocked (Debuff)

```yaml
# elements/configs/lightning_element.yaml
status_effects:
  - name: "shocked"
    type: "debuff"
    base_probability: 0.14
    base_duration: 5.0
    base_intensity: 1.0
    tick_interval: 1.0
    max_stacks: 5
    stackable: true
    refresh_duration: true
    effects:
      paralyze_micro: true
      damage_taken_amp: 0.04
    dynamics:
      intensity_gain: 0.016
      intensity_damping: 0.013
      decay_rate: 0.05
      refractory_gain: 0.45
      refractory_decay: 0.12
```

#### Behavior
- Tăng damage nhận nhẹ; có khả năng “đơ ngắn” theo xác suất (không ICD cứng, dùng refractory).

### Same-Element: Lightning ↔ Lightning

```yaml
same_element_effects:
  - pool_id: "arc_resonance"
    apply_to: "both"
  - pool_id: "overcharge_boost"
    apply_to: "attacker"
```

### Neutral: Lightning ↔ Neutral

```yaml
neutral_effects:
  - pool_id: "shocked_fallback"
    apply_to: "defender"
    probability:
      base: "from_element"
      use_probability_engine: true
      scaling_factor_key: "status_probability"
    dynamics_override: {}
```

## 🌩️ Suggested Interactions
- Với Water: Conduction (lan theo Soaked)
- Với Metal: Overload (crit_damage↑, self-short risk)
- Với Earth/Crystal: Grounding/Resonance (giảm hiệu lực, shatter risk)
- Với Wind: Storm Charge (chain↑), Thunderclap (displace trên shocked)

## 🔗 References
- `configs/status_pool.yaml`
- `configs/interaction_config.yaml`
- `configs/probability_config.yaml`
- `11_Advanced_Derived_Stats_Design.md`

## 🧪 Minimal Test Checklist
- Probability engine: shocked p trong expected_range từ `elements/golden_vectors/lightning_golden_vectors.json`
- Dynamics: Δ↑ → intensity↑; refractory hoạt động (không spam paralyze)
- Same-element: arc_resonance/overcharge_boost áp dụng đúng
- Neutral: shocked_fallback kích hoạt khi thiếu cặp
