# 16 – Cursor AI Execution Plan (Phases)

Use these phases sequentially. Each phase has a matching `.cursor/rules/actor-core-phase-XX-*.mdc` file that constrains edits and lists exact tasks.

How to run a phase (suggested)
- Announce: "Run phase XX" in PR description.
- Ensure working tree clean; run: `cargo test -p actor-core` once before starting.
- After AI applies edits: run tests, lints, benches; commit with the provided message template.

Phases
- Phase 01 – Registry Arc<dyn Subsystem>
  - Rule: `.cursor/rules/actor-core-phase-01-registry.mdc`
  - Goal: signature changes + remove wrapper; update imports.
- Phase 02 – Split services
  - Rule: `.cursor/rules/actor-core-phase-02-services.mdc`
  - Goal: move aggregator/caps provider to dedicated modules.
- Phase 03 – Cache multi-layer refactor
  - Rule: `.cursor/rules/actor-core-phase-03-cache.mdc`
  - Goal: split multi_layer.rs into policy/metrics/layers/manager/backends/warming.
- Phase 07 – Constants/Enums/Interfaces split
  - Rule: `.cursor/rules/actor-core-phase-07-constants.mdc`
  - Goal: move metrics/enums, keep interfaces trait-only.
- Phase 12 – Prelude & API surface
  - Rule: `.cursor/rules/actor-core-phase-12-prelude.mdc`
  - Goal: add prelude; tighten re-exports.
- Phase 06 – Tests layout
  - Rule: `.cursor/rules/actor-core-phase-06-tests.mdc`
  - Goal: update tests to new layout and paths.
- Phase 08 – Coding rules migration
  - Rule: `.cursor/rules/actor-core-phase-08-coding-rules.mdc`
  - Goal: types/locks/logging/errors/docs updates.
- Phase 09 – Performance profiling thresholds
  - Rule: `.cursor/rules/actor-core-phase-09-profiling.mdc`
  - Goal: run benches; validate thresholds.
- Phase 10 – CI feature matrix
  - Rule: `.cursor/rules/actor-core-phase-10-ci.mdc`
  - Goal: add jobs and gates.
- Phase 14 – Observability & SLOs
  - Rule: `.cursor/rules/actor-core-phase-14-observability.mdc`
  - Goal: instrument spans/metrics; validate counters.
- Phase 15 – Deprecation & rollback
  - Rule: `.cursor/rules/actor-core-phase-15-deprecation.mdc`
  - Goal: document deprecations; add shims/timeline.

Commit template
- `feat(actor-core): phase XX – <title>\n\nSummary: <what changed>\nDocs: docs/actor-core/optimize-code/XX_*.md\nValidation: tests green; clippy clean; benches within threshold.`
