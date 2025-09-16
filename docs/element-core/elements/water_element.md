# Water Element

## 📋 Element Overview

- Element ID: `water`
- Category: `five_elements`
- Classification: Yin-leaning — fluid, adaptable, control

## 🌊 Status Effects

### Soaked (Debuff)

#### Mechanics
- Type: Control/slow + accuracy damp
- Stackable: Yes (up to 5), refresh duration

#### Properties (excerpt)
```yaml
# elements/configs/water_element.yaml
status_effects:
  - name: "soaked"
    type: "debuff"
    base_probability: 0.12
    base_duration: 6.0
    base_intensity: 1.0
    tick_interval: 1.0
    max_stacks: 5
    stackable: true
    refresh_duration: true
    effects:
      move_speed_slow: 0.08
      accurate_rate_debuff: 0.04
    dynamics:
      intensity_gain: 0.015
      intensity_damping: 0.012
      decay_rate: 0.04
      refractory_gain: 0.4
      refractory_decay: 0.1
```

#### Behavior
- Slow and accuracy debuff scale with intensity.
- Probability uses the same engine as Fire (sigmoid, clamped math bound).

### Same-Element: Water ↔ Water

Concepts (from status pool):
- Flow Resonance: minor skill execution speed smoothing and dodge stabilizing for both sides; costs small resource efficiency.
- Aqua Veil: defender receives brief damage smoothing layer that decays rapidly.

```yaml
# elements/configs/water_element.yaml
same_element_effects:
  - pool_id: "flow_resonance"
    apply_to: "both"
  - pool_id: "aqua_veil"
    apply_to: "defender"
```

### Neutral: Water ↔ Neutral

- When not in `pairs`, Water applies its base control status: Soaked.
- Uses probability engine; no special interaction buffs/debuffs.

```yaml
# elements/configs/water_element.yaml
neutral_effects:
  - pool_id: "soaked_fallback"
    apply_to: "defender"
    probability:
      base: "from_element"
      use_probability_engine: true
      scaling_factor_key: "status_probability"
    dynamics_override: {}
```

### Environmental Modifiers (optional)

```yaml
# elements/configs/water_element.yaml
environment_mods:
  rain:
    status_probability_add: +0.05
    intensity_gain_mod: +0.005
  dry:
    status_probability_add: -0.05
    intensity_gain_mod: -0.005
```

## 🔗 References
- `configs/status_pool.yaml`
- `configs/interaction_config.yaml`
- `configs/probability_config.yaml`
- `11_Advanced_Derived_Stats_Design.md`

## 🧪 Minimal Test Checklist

- Probability engine
  - [ ] Soaked probability within expected_range from `elements/golden_vectors/water_golden_vectors.json`.
  - [ ] p ∈ [0,1] (mathematical bound), monotonic w.r.t mastery delta.
- Dynamics
  - [ ] Increasing Δ raises intensity trajectory; damping causes decay over time.
  - [ ] Refractory reduces immediate retriggers after a trigger.
- Same-element
  - [ ] `flow_resonance` and `aqua_veil` apply as defined; no cross-element effects.
- Neutral-by-element
  - [ ] When pair is missing, `soaked_fallback` triggers via probability engine.
- Environment
  - [ ] `rain` increases, `dry` decreases status_probability as configured.

## 📦 Golden Vector Loading (Pseudo-code)

```rust
// Pseudo-code for QA to validate ranges from golden vectors
struct GoldenVectors { /* ... */ }

fn load_json<T>(path: &str) -> T { /* ... */ }

fn test_soaked_probability_against_gv() {
    let gv: serde_json::Value = load_json("elements/golden_vectors/water_golden_vectors.json");
    let expected = gv["probability_engine"]["soaked_status_probability"]["expected_range"].as_array().unwrap();
    let lo = expected[0].as_f64().unwrap();
    let hi = expected[1].as_f64().unwrap();

    // Build attacker/defender stats from gv (omni/element sums) and run probability engine
    let p = compute_status_probability(/* attacker, defender, scaling from probability_config */);

    assert!(p >= lo && p <= hi, "Soaked probability {:?} not within [{:?}, {:?}]", p, lo, hi);
}
```
