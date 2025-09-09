# 10 — Implementation Guide

1) Implement interfaces (Section 04).
2) Implement **CapLayerRegistry** loader.
3) Implement **CapsProvider**:
   - Merge caps **within each layer** → `LayerCaps`
   - Reduce caps **across layers** using registry policy → `EffectiveCapsFinal`
4) Implement **CombinerRegistry** (per-dimension rules).
5) Implement **Aggregator** (Section 06): gather, compute candidate, clamp via `EffectiveCapsFinal`.
6) Testing (Section 14): golden, property, parity.
