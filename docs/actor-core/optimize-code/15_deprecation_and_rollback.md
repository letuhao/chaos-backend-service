# 15 – Deprecation and rollback plan

Goal: Safely evolve APIs with clear timelines and reversible steps.

Deprecation policy
- Use semantic versioning at crate level.
- Mark old re-exports and APIs as deprecated one minor version before removal.
- Provide shims (e.g., temporary `pub use`), documented in `prelude_and_api_surface.md`.

Planned deprecations
- Registry API: return `Arc<dyn Subsystem>` instead of `Box<dyn Subsystem>`.
  - Version N: introduce new signatures; keep shims.
  - Version N+1: remove shims; update changelog.
- Services module layout:
  - Version N: add `services/aggregator.rs` with re-exports from `services.rs`.
  - Version N+1: delete legacy paths.

Changelog checklist (per breaking change)
- Summary of change and rationale.
- Migration steps for consumers (code snippets).
- Feature flags affected.
- Links to relevant docs in `optimize-code/`.

Rollback plan
- If CI/benches regress or incidents occur:
  - Revert specific refactor commits (keep docs; revert code only).
  - Restore shims and re-exports.
  - Disable new features behind flags (`--no-default-features` or removing feature from CI matrix).
  - Cut a patch release with rollback notes.

Release hygiene
- Tag releases; attach generated docs and Criterion summaries.
- Keep a migration guide in `docs/` referencing exact versions.
