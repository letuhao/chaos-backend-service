# 07 — Caps & Layers (Total + Realm/World/Event)

## Within-Layer Merge (per layer, per dimension)
Same algorithm as v2:
- `baselineMax += BASELINE(max)`
- `addMax += ADDITIVE(max)`
- `hardMax = min(hardMax, HARD_MAX)`
- `overrideMax` by highest `priority`
- Mirror for min
- `candidateMax = overrideMax ?? (baselineMax + addMax)`
- `finalMax = min(candidateMax, hardMax, registryDefault.max)`
- `finalMin = max(candidateMin, hardMin, registryDefault.min)`
- Enforce `finalMin ≤ finalMax`

Output: `LayerCaps[layer][dimension] = {Min, Max}`

## Across-Layer Reduction
Use **CapLayerRegistry**:
```yaml
order: [REALM, WORLD, EVENT, TOTAL]
across_policy: INTERSECT
```

- Start with infinite range; intersect with each present layer in order:
  - `range.Min = max(range.Min, caps.Min)`
  - `range.Max = min(range.Max, caps.Max)`
- Finally intersect with **registry clampDefault**.
- Optional policy `PRIORITIZED_OVERRIDE`: if a layer is authoritative and sets an OVERRIDE bound, you may short‑circuit.

The result is `EffectiveCapsFinal[dimension]`, guaranteed to honor all active layers and never exceed defaults.
