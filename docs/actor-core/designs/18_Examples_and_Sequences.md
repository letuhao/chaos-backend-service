# 18 — Examples & Sequences

## Example A — Strength with Realm & World
- Contributions: `strength FLAT +650`.
- Caps:
  - TOTAL: BASELINE +200 (kim_dan), ADDITIVE +500 (luyen_the) → 700
  - REALM (luyen_the:T2): HARD_MAX 600
  - WORLD: HARD_MAX 550
- Across-layer (order: REALM → WORLD → TOTAL, policy: INTERSECT):
  - final cap = min(600, 550, 700) = **550**
- Final value: clamp(650, [..550]) = **550**.

```mermaid
flowchart LR
  A[Contributions] -->|FLAT/MULT/POST_ADD/OVERRIDE| B[Candidate v]
  B --> C[Within-Layer Caps]
  C --> D[Across-Layer Reduction (INTERSECT)]
  D --> E[Clamp to EffectiveCapsFinal]
  E --> F[Snapshot]
```
