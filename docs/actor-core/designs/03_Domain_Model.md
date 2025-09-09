# 03 — Domain Model

**Actor**
- GUID, Name, Race, LifeSpan, Age, CreatedAt, UpdatedAt, Version
- Subsystems[]: registered plugins

**Snapshot**
- Primary: map<dimension, number>
- Derived: map<dimension, number>
- CapsUsed: map<dimension, {Min, Max}>
- Version: integer
