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

### Determinism & Semantics (Implementation Notes)
- Within-bucket sort: priority DESC, then system ASC; deterministic across runs.
- OVERRIDE: highest priority wins; if tie, system ASC.
- MULT semantics: treated as multiplicative factor (e.g., 1.10 = +10%).
- Clamp precedence: EffectiveCaps → Combiner clamp_default → constants clamp ranges.
