# Actor Core v3 — Golden Test Vectors
**Generated:** 2025-09-07 18:02

Each case contains:
- `subsystems.json`: array of **SubsystemOutput** (design contract)
- `expected.json`: expected **Snapshot**

Assumptions:
- Combiner Registry clamp defaults from design examples (e.g., `crit_rate:[0..1]`, `cdr:[0..0.5]`, `move_speed:[0..12]`).
- Layer order `REALM -> WORLD -> EVENT -> TOTAL`, policy `INTERSECT` (unless a case provides a custom cap-layer registry).

## Cases
01. **Total cap sum** — baseline + additive = 700, value below cap.
02. **Realm + World + Total** — final cap is min(600, 550, 700) = 550.
03. **Contribution OVERRIDE priority** — override wins with higher priority.
04. **MULT stacking (delta %)** — two multiplicative deltas.
05. **CooldownReduction HARD_MAX** — clamp at 0.5 despite higher sum.
06. **lifespan_years operator MAX** — operator mode, choose max, world min bound.
07. **HARD_MIN floor** — move_speed cannot go below 3.
08. **min>max conflict** — corrected to equality at intersection.
09. **PRIORITIZED_OVERRIDE policy** — across-layer policy demo (authoritative WORLD).
10. **Derived & Primary mix with realm min** — realm min raises spirit; derived mp_max unaffected by spirit directly.
