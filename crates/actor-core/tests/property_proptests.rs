//! Proptest-based property tests for Actor Core bucket processing.
//! These validate ordering invariance, clamp invariants, and OVERRIDE determinism.

use actor_core::bucket_processor::process_contributions_in_order;
use actor_core::enums::Bucket;
use actor_core::types::Contribution;
use actor_core::types::Caps;
use proptest::prelude::*;
use rand::SeedableRng;

fn approx_eq(a: f64, b: f64, eps: f64) -> bool {
    (a - b).abs() <= eps
}

proptest! {
    /// Ordering invariance: shuffling contributions should not change the result
    /// because the processor sorts deterministically within buckets.
    #[test]
    fn prop_bucket_ordering_invariance(contribs in prop_contributions_strategy(0, 40)) {
        let mut v1 = contribs.clone();
        let mut v2 = contribs.clone();

        // Shuffle in two different orders
        use rand::seq::SliceRandom;
        let mut rng = rand::rngs::StdRng::seed_from_u64(12345);
        v1.as_mut_slice().shuffle(&mut rng);
        v2.as_mut_slice().shuffle(&mut rng);

        // Process both
        let r1 = process_contributions_in_order(v1, 0.0, None).unwrap();
        let r2 = process_contributions_in_order(v2, 0.0, None).unwrap();

        prop_assert!(approx_eq(r1, r2, 1e-9));
    }

    /// Clamp invariant: result is within the provided caps.
    #[test]
    fn prop_clamp_invariant(
        contribs in prop_contributions_strategy(1, 25),
        min in -10_000.0f64..=0.0,
        max in 0.0f64..=10_000.0,
    ) {
        let (min, max) = if min <= max { (min, max) } else { (max, min) };
        let caps = Caps::new(min, max);

        let r = process_contributions_in_order(contribs, 0.0, Some(&caps)).unwrap();

        prop_assert!(r >= min - 1e-9 && r <= max + 1e-9);
    }

    /// OVERRIDE determinism: among multiple OVERRIDE entries, the chosen value
    /// is independent of input order (deterministic tiebreak rules).
    #[test]
    fn prop_override_deterministic(
        mut flat_vals in proptest::collection::vec(-1000.0f64..=1000.0, 0..10),
        overrides in prop_override_contributions_strategy(1, 10),
    ) {
        // Build a mixed vector with some FLATs and many OVERRIDEs
        let mut contribs: Vec<Contribution> = Vec::new();
        for (i, v) in flat_vals.drain(..).enumerate() {
            let c = Contribution {
                dimension: "stat".to_string(),
                bucket: Bucket::Flat,
                value: v,
                system: format!("flat_{i}"),
                priority: Some(((i % 3) as i64) * 10),
                tags: None,
            };
            contribs.push(c);
        }
        contribs.extend(overrides.clone());

        // Two random shuffles
        use rand::seq::SliceRandom;
        let mut v1 = contribs.clone();
        let mut v2 = contribs.clone();
        let mut rng = rand::rngs::StdRng::seed_from_u64(98765);
        v1.as_mut_slice().shuffle(&mut rng);
        v2.as_mut_slice().shuffle(&mut rng);

        let r1 = process_contributions_in_order(v1, 0.0, None).unwrap();
        let r2 = process_contributions_in_order(v2, 0.0, None).unwrap();

        prop_assert!(approx_eq(r1, r2, 1e-9));
    }
}

// Strategy helpers

fn prop_contributions_strategy(min_len: usize, max_len: usize) -> impl Strategy<Value = Vec<Contribution>> {
    // Limit ranges to keep values finite and avoid extreme magnitudes
    let bucket_strategy = prop_oneof![
        Just(Bucket::Flat),
        Just(Bucket::Mult),
        Just(Bucket::PostAdd),
        Just(Bucket::Override),
    ];

    proptest::collection::vec(
        (bucket_strategy, any_finite_value(), any_system_id(), any_priority_opt())
            .prop_map(|(bucket, val, system, pri)| Contribution {
                dimension: "stat".to_string(),
                bucket,
                value: match bucket {
                    Bucket::Mult => val_mult_factor(val),
                    _ => val,
                },
                system,
                priority: pri,
                tags: None,
            }),
        min_len..=max_len,
    )
}

fn prop_override_contributions_strategy(min_len: usize, max_len: usize) -> impl Strategy<Value = Vec<Contribution>> {
    proptest::collection::vec(
        (any_finite_value(), any_system_id(), any_priority_opt())
            .prop_map(|(val, system, pri)| Contribution {
                dimension: "stat".to_string(),
                bucket: Bucket::Override,
                value: val,
                system,
                priority: pri,
                tags: None,
            }),
        min_len..=max_len,
    )
}

fn any_system_id() -> impl Strategy<Value = String> {
    // Short deterministic-friendly identifiers
    proptest::collection::vec("[a-z]{1,6}", 1..=1).prop_map(|v| v[0].to_string())
}

fn any_priority_opt() -> impl Strategy<Value = Option<i64>> {
    prop_oneof![
        Just(None),
        (0i64..=1000i64).prop_map(Some),
    ]
}

fn any_finite_value() -> impl Strategy<Value = f64> {
    // Avoid NaN/Inf by limiting to a reasonable range
    -10_000.0f64..=10_000.0
}

fn val_mult_factor(val: f64) -> f64 {
    // Map arbitrary value to a multiplicative factor within [0.5, 2.0]
    // to prevent extreme magnitudes and keep determinism robust.
    let clamped = if val.is_finite() { val } else { 0.0 };
    let norm = (clamped / 10_000.0).max(-1.0).min(1.0); // [-1,1]
    1.0 + norm * 0.5 // [0.5,1.5]
}


