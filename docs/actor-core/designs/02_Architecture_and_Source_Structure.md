# 02 — Architecture & Source Structure

## Components
- **Actor (metadata)**: GUID, Name, Race, LifeSpan, Age, timestamps, Version, Subsystems[]
- **Subsystem (plugin)**: owns formulas; outputs `SubsystemOutput` (Primary/Derived Contributions, Caps, optional Context)
- **CapsProvider**: merges `CapContribution` **within layer**, yields LayerCaps per dimension
- **CapLayerRegistry**: declares layer **order** and **across‑layer policy**
- **CombinerRegistry**: per‑dimension merge rule (pipeline vs operator) and clamp defaults
- **Aggregator**: orchestrates -> Snapshot (Primary, Derived, CapsUsed, Version)

## Repo layout (suggested)
```
/docs                # These design docs (authoritative)
/src/core            # Interfaces only (no formulas)
/src/registry        # Registry loader (YAML/JSON)
/src/caps            # Caps merge (layered engine)
/src/subsystems/*    # Effects, Items, Race, Talent, Fate, Karma, Cultivation, ...
/tests/golden        # [SubsystemOutput] -> Snapshot (truth files)
/tests/property      # Order invariance, clamp invariants
/parity/*            # Cross-language parity vectors
```
