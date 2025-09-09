# Actor-Core — Cursor AI Implementation Bundle
_Date: 2025-09-09_

This bundle gives **step-by-step, command-first** instructions to finish the Actor Core per the design docs.
It’s organized into **phases**, each with:
- a brief **Goal**
- **Exact Commands** to run (Windows PowerShell & Git Bash)
- **Cursor AI Prompts** you can copy-paste
- **Acceptance Criteria**

> Tip: Create a new git branch for each phase to keep changes focused and reviewable.

## Contents
- `00_README.md` — This file (overview + entry points)
- `10_Cursor_Prompts.md` — Copy‑paste prompts for Cursor AI (per phase)
- `11_Commands_Windows.md` — All commands (PowerShell‑first)
- `12_Commands_GitBash.md` — All commands (Git Bash)
- `01_Tasks_RegistryLoader.md` — Implement YAML/JSON registry loader
- `02_Tasks_EffectiveCaps_Alias.md` — Add EffectiveCaps alias
- `03_Tasks_FeatureFlags_ExtraBuckets.md` — Gate extra buckets behind feature flags
- `04_Tasks_BucketOrder_Algorithm.md` — Enforce bucket ordering + clamping
- `05_Tasks_Config_Samples.md` — Sample YAML/JSON configs
- `06_Tasks_Tests.md` — Unit + property tests
- `07_Tasks_CI_Checks.md` — CI: fmt, clippy, deny, doctests
- `08_Tasks_Benchmarks.md` — Benchmarks for registry sources
- `09_Tasks_Docs_Sync.md` — Update docs to reflect code
- `configs/` — Example configs (edit + version control them)
- `schemas/` — Optional JSON schemas for validation
- `templates/` — Issue/PR templates

Start with **`11_Commands_Windows.md`** or **`12_Commands_GitBash.md`**.
Then follow the matching task file for that Phase.
