# 06 — Aggregation Algorithm

Per dimension:
1) Gather all `Contribution` from all subsystems.
2) Sort contributions: `(bucket asc, priority desc, system asc)`.
3) If `usePipeline`:
   - `sumFlat = Σ FLAT`
   - `mult = Π (1 + MULT)`
   - `post = Σ POST_ADD`
   - `candidate = (OVERRIDE ? overrideValue : sumFlat * mult + post)`
4) Else (operator mode): `candidate = SUM | MAX | MIN(values)`
5) Compute **EffectiveCapsFinal** (Section 07) and **clamp** `candidate` → `final`.
6) Write to Snapshot.
