# 13 – Validation and input sanitization

Goal: Centralize validation for contributions, caps, and configuration.

Files
- `crates/actor-core/src/bucket/validation.rs` (new)
  - Functions:
    - `validate_contributions(contribs: &[Contribution]) -> ActorCoreResult<()>`
      - Check dimension names non-empty, bucket valid, values finite (no NaN/Inf), priorities non-negative.
    - `validate_dimension_merge_rule(dimension: &str, rule: &MergeRule) -> ActorCoreResult<()>`
    - `validate_caps(dimension: &str, caps: &Caps) -> ActorCoreResult<()>`
- `crates/actor-core/src/services/aggregator.rs`
  - Ensure calls to `validate_contributions` occur before processing.
- `crates/actor-core/src/registry/loader.rs`
  - Validate loaded rules and cap layers via validation functions.

Invariants
- Dimension names are lowercase snake_case.
- Bucket sequences must be one of the allowed orders when using pipeline.
- Caps satisfy `min <= max`, and clamp ranges cover expected domain if defaults apply.

Validation
- Add unit tests for invalid inputs in `tests/property_proptests.rs` and targeted negative tests per module.
