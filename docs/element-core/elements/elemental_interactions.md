# Elemental Interactions (Aggregate Overview)

This document summarizes elemental interactions for designers and implementers. It is narrative-only and references the authoritative configs to prevent duplication.

- Single source of truth:
  - `docs/element-core/configs/interaction_config.yaml`
  - `docs/element-core/configs/status_pool.yaml`
  - `docs/element-core/configs/probability_config.yaml`
  - Overview: `docs/element-core/elements/overview/five_elements_overview.md`
- Design principles:
  - No hard caps; yin–yang counterbalance via dynamics (`intensity_gain`, `intensity_damping`, `decay_rate`, `refractory_gain`, `refractory_decay`).
  - Probabilities are computed via sigmoid and clamped to [0, 1] as a math bound (not a gameplay cap).
  - Same-element interactions (X ↔ X) are documented in each element’s file (e.g., `elements/fire_element.md`).

> VI: Tài liệu này chỉ tóm tắt và trỏ đến file cấu hình gốc để tránh trùng lặp.  
> 中文: 本文仅提供概要，具体实现以配置文件为准，避免重复。

---

## Relationships & Base Triggers

Defined centrally; do not duplicate numbers here. Example excerpt:

```yaml
# docs/element-core/configs/interaction_config.yaml (excerpt)
relationships:
  same: 0.0
  neutral: 0.1
  generating: 0.3
  overcoming: 0.8

dynamics:
  trigger_weight: 0.2
  intensity_weight: 0.1
  trigger_scale: 50.0
  steepness: 1.0
```

- Interpretation:
  - same: baseline only; specialized same-element effects live in each element doc.  
    VI: hiệu ứng cùng nguyên tố được mô tả trong file nguyên tố tương ứng.  
    中文: 同元素效果在对应元素文档中说明。
  - generating/overcoming/neutral: provide base trigger; final trigger = base + scaled(sigmoid), then clamp to [0,1].

## Pairs (Who interacts with whom)

Pairs are listed in config to avoid scattering rules. Example excerpt:

```yaml
# docs/element-core/configs/interaction_config.yaml (excerpt)
pairs:
  fire:
    generating: ["wood"]
    overcoming: ["metal"]
    neutral: ["water", "earth"]
  water:
    generating: ["metal"]
    overcoming: ["fire"]
    neutral: ["wood", "earth"]
```

> VI: Chỉ cập nhật danh sách pairs trong config, không sửa ở tài liệu này.  
> 中文: 仅在配置中维护配对列表，不在本文重复。

## Effects Catalog (References to Status Pool)

Define effects once in `status_pool.yaml`, then reference by `pool_id` from `interaction_config.yaml`.

- Shared pool entries (examples):

```yaml
# docs/element-core/configs/status_pool.yaml (excerpt)
effects:
  heat_resonance:
    category: "same_element"
    applies_to: ["attacker", "defender"]
    dynamics:
      intensity_gain: 0.015
      intensity_damping: 0.012
      refractory_gain: 0.4
      refractory_decay: 0.1
    stat_hooks:
      skill_execution_speed: { delta_weight: 0.12 }
      crit_rate: { delta_weight: 0.06 }
      resource_cost: { delta_weight: 0.05 }

  ember_shield:
    category: "same_element"
    applies_to: ["defender"]
    dynamics:
      intensity_gain: 0.02
      intensity_damping: 0.02
    reflect:
      type: "burning_thorns"
      proportion_of_damage: 0.08
```

- Referencing effects for a specific matchup (examples):

```yaml
# docs/element-core/configs/interaction_config.yaml (excerpt)
effects:
  fire_vs_metal_heat_soften:
    when: { attacker: "fire", defender: "metal", relationship: "overcoming" }
    apply_to: "defender"
    pool_id: "heat_resonance"
    dynamics_mod:
      intensity_gain_mod: +0.02
      intensity_damping_mod: +0.02
    stat_hooks:
      parry_rate: { damping_mod: +0.01 }
      defense_point: { effective_delta_weight: +0.1 }

  wood_generates_fire_oxygen_surge:
    when: { attacker: "wood", defender: "fire", relationship: "generating" }
    apply_to: "defender"
    pool_id: "heat_resonance"
    dynamics_mod:
      intensity_gain_mod: +0.03
      refractory_gain_mod: +0.2
    stat_hooks:
      power_point: { delta_weight: +0.15 }
      self_dot_risk: { base: "light", scales_with_intensity: true }

  water_overcomes_fire_quench:
    when: { attacker: "water", defender: "fire", relationship: "overcoming" }
    apply_to: "defender"
    pool_id: "ember_shield"
    dynamics_mod:
      intensity_damping_mod: +0.03
      refractory_gain_mod: +0.3
    aura:
      steam_veil:
        accuracy_mod: -0.08
        move_speed_mod: +0.06
```

> VI: Logic hiệu ứng nằm ở pool; interaction chỉ tham chiếu bằng `pool_id` và áp dụng `*_mod`.  
> 中文: 效果逻辑在池中定义；交互仅通过 `pool_id` 引用并覆写参数。

## Same-Element Interactions (Where to find them)

- Each element maintains its own same-element section, e.g. `elements/fire_element.md` for Fire↔Fire.  
- This aggregate keeps only cross-element interactions to avoid duplication.

## Implementation Notes

- Use probability mechanics from `01_Probability_Mechanics_Design.md` and dynamics from `06_Implementation_Notes.md`.
- Engine flow (conceptual):

```rust
// 1) Determine relationship and base trigger from config
// 2) Compute normed stat delta and sigmoid(scale)
// 3) final_trigger = clamp(base + scaled, 0.0, 1.0)
// 4) If triggered, materialize effect by resolving pool_id -> effect template
// 5) Apply dynamics (gain/damping/decay/refractory) over time
```

> VI: Không dùng cooldown cứng; dùng refractory để giảm xác suất tạm thời sau khi kích hoạt.  
> 中文: 不使用固定冷却；通过不应期降低短时触发概率。

### Damage Composition (Order)

- For damage resolution order with parry/block, see `docs/combat-core/02_Damage_System_Design.md`.
- Short order reference: `Hit → Parry → Block → Penetration/Defense → Reflection → Shields → Resources`.

## How to Add a New Interaction

1) Add or reuse a pool effect in `status_pool.yaml` (id, dynamics, hooks).  
2) Reference it in `interaction_config.yaml/effects` with `when`, `apply_to`, `pool_id`, and optional `*_mod`.  
3) If it is a same-element effect, document it in the element’s markdown (not here).  
4) Add/adjust golden vectors if needed under `elements/golden_vectors/`.  
5) Telemetry: ensure engine logs `(Δ, I, R, p)` for tuning.

> VI: Quy trình trên giúp mở rộng mà không tạo trùng lặp.  
> 中文: 按上述流程扩展，避免重复定义。

---

## Neutral-by-Element (Not in Pairs)

If a pair is not defined, handle it within the attacker element’s markdown/config as a Neutral-by-Element rule. For Fire, see `elements/fire_element.md` → “Neutral Interaction: Fire ↔ Neutral”. That entry references `status_pool.yaml` (`burning_fallback`) and uses the probability engine.

> VI: Trường hợp không có cặp, quy định tại file nguyên tố tấn công (ví dụ Fire↔Neutral).  
> 中文: 当配对缺失时，在进攻元素文档中定义为“中立交互”。

---

Last Updated: 2025-09-16  
Maintainer: Chaos World Team
