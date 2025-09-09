# Phase 8 — Benchmarks

## Goal
Compare programmatic vs file-loaded registry overhead.

### Ideas
- Criterion benchmark attaching N layers and M rules.
- Bench regressions in PRs.

## Commands
```powershell
git checkout -b bench/registries
cargo add criterion@0.5 --dev
# Create benches/registries.rs
cargo bench
```
