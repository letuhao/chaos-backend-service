# 01 — Executive Summary

Actor Core v3 is a **metadata‑only aggregator**. Domain logic is implemented by **Subsystems**
(Race, Items, Effects, Talent, Fate, Karma, Cultivation, …). Core collects **Contributions** and **CapContributions**,
merges caps (including **layered caps**: REALM/WORLD/EVENT/TOTAL), and clamps once per dimension to produce a deterministic **Snapshot**.
