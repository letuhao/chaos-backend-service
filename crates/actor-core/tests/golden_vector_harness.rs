use actor_core::{RegistryFactory, ServiceFactory, CacheFactory};
use actor_core::interfaces::Subsystem as SubsystemTrait;
use actor_core::types::{SubsystemOutput, Contribution};
use actor_core::enums::Bucket;
use serde::Deserialize;
use std::fs;
use std::path::Path;

#[derive(Deserialize)]
struct InputContribution { #[allow(dead_code)] dimension: String, #[allow(dead_code)] bucket: String, #[allow(dead_code)] value: f64, #[allow(dead_code)] system: String }

#[derive(Deserialize)]
struct VectorFile { #[allow(dead_code)] actor_id: String, #[allow(dead_code)] version: i32, #[allow(dead_code)] inputs: Vec<InputContribution> }

struct InlineSubsystem { id: String, prio: i64, contribs: Vec<Contribution> }

#[async_trait::async_trait]
impl SubsystemTrait for InlineSubsystem {
    fn system_id(&self) -> &str { &self.id }
    fn priority(&self) -> i64 { self.prio }
    async fn contribute(&self, _actor: &actor_core::types::Actor) -> actor_core::ActorCoreResult<SubsystemOutput> {
        let mut out = SubsystemOutput::new(self.id.clone());
        for c in &self.contribs { out.add_primary(c.clone()); }
        Ok(out)
    }
}

#[test]
fn run_golden_vector_case01_damage_heal() {
    run_case("docs/resource-manager/golden_vectors/case01_damage_and_heal_same_tick");
}

#[test]
fn run_golden_vector_case02_ooc_regen() {
    run_case("docs/resource-manager/golden_vectors/case02_ooc_regen");
}

fn run_case(dir: &str) {
    let subs_path = Path::new(dir).join("subsystems.json");
    let expected_path = Path::new(dir).join("expected.json");

    let subs = fs::read_to_string(&subs_path).expect("read subsystems.json");
    let expected = fs::read_to_string(&expected_path).expect("read expected.json");

    let parsed: serde_json::Value = serde_json::from_str(&subs).expect("subsystems.json invalid");
    let parsed_expected: serde_json::Value = serde_json::from_str(&expected).expect("expected.json invalid");

    // Build minimal aggregator (registry + caps + cache)
    let plugin = RegistryFactory::create_plugin_registry();
    let combiner = RegistryFactory::create_combiner_registry();
    let cap_layers = RegistryFactory::create_cap_layer_registry();
    let caps = ServiceFactory::create_caps_provider(cap_layers);
    let cache = CacheFactory::create_in_memory_cache(10_000, 600);
    let aggregator = ServiceFactory::create_aggregator(plugin.clone(), combiner, caps, cache);

    // Map inputs into contributions, with optional tick/decay/offline policies
    let mut contribs: Vec<Contribution> = Vec::new();
    if let Some(inputs) = parsed.get("inputs").and_then(|v| v.as_array()) {
        for it in inputs {
            let dim = it.get("dimension").and_then(|v| v.as_str()).unwrap().to_string();
            let bucket_s = it.get("bucket").and_then(|v| v.as_str()).unwrap().to_string();
            let value = it.get("value").and_then(|v| v.as_f64()).unwrap();
            let system = it.get("system").and_then(|v| v.as_str()).unwrap().to_string();
            let bucket = match bucket_s.as_str() { "FLAT"=>Bucket::Flat, "MULT"=>Bucket::Mult, "POST_ADD"=>Bucket::PostAdd, "OVERRIDE"=>Bucket::Override, _=>Bucket::Flat };
            contribs.push(Contribution{ dimension: dim, bucket, value, system, priority: Some(100), tags: None });
        }
    }

    // Tick-based adjustments
    let delta = parsed.get("tick").and_then(|t| t.get("delta_seconds")).and_then(|v| v.as_f64()).unwrap_or(0.0);
    if delta > 0.0 {
        // Regen add to current values (simple model for test harness)
        let regen_dims = ["hp_regen","mana_regen","stamina_regen"]; // map to respective current dims
        for rd in regen_dims.iter() {
            if let Some(regen_val) = find_input_value(&parsed, rd) {
                let target_dim = match *rd { "hp_regen"=>"hp_current", "mana_regen"=>"mana_current", "stamina_regen"=>"stamina_current", _=>"" };
                if !target_dim.is_empty() {
                    contribs.push(Contribution{ dimension: target_dim.to_string(), bucket: Bucket::Flat, value: regen_val*delta, system: "tick".into(), priority: Some(90), tags: None });
                }
            }
        }
        // Shield decay
        if let Some(decay) = parsed.get("decay").and_then(|d| d.get("shield_per_second")).and_then(|v| v.as_f64()) {
            contribs.push(Contribution{ dimension: "shield_current".into(), bucket: Bucket::Flat, value: -decay*delta, system: "decay".into(), priority: Some(95), tags: None });
        }
    }

    // Offline catch-up
    if let Some(off_secs) = parsed.get("metadata").and_then(|m| m.get("offline_seconds")).and_then(|v| v.as_f64()) {
        let max_secs = parsed.get("policy").and_then(|p| p.get("offline_regen_max_seconds")).and_then(|v| v.as_f64()).unwrap_or(0.0);
        let window = off_secs.min(max_secs);
        if window > 0.0 {
            if let Some(regen_val) = find_input_value(&parsed, "mana_regen") {
                contribs.push(Contribution{ dimension: "mana_current".into(), bucket: Bucket::Flat, value: regen_val*window, system: "offline".into(), priority: Some(85), tags: None });
            }
        }
    }

    // Register inline subsystem
    let inline = InlineSubsystem{ id: "inline_res".into(), prio: 100, contribs };
    plugin.register(Box::new(inline)).expect("register subsystem");

    // Build dummy actor
    let actor = actor_core::types::Actor::new("TestActor".into(), "Human".into());
    // Resolve
    let rt = tokio::runtime::Runtime::new().unwrap();
    let snapshot = rt.block_on(aggregator.resolve(&actor)).expect("resolve snapshot");

    // Assert primary values numerically if present
    if let Some(primary) = parsed_expected.get("primary").and_then(|v| v.as_object()) {
        for (k,v) in primary.iter() {
            let expected_val = v.as_f64().unwrap();
            let got = snapshot.primary.get(k.as_str()).copied().unwrap_or(0.0);
            assert!((got-expected_val).abs() < 1e-6, "primary {} expected {}, got {}", k, expected_val, got);
        }
    }

    // Derived: if hp_percentage expected, compute from primary
    if let Some(derived) = parsed_expected.get("derived").and_then(|v| v.as_object()) {
        if let (Some(exp), Some(cur), Some(maxv)) = (
            derived.get("hp_percentage").and_then(|v| v.as_f64()),
            snapshot.primary.get("hp_current"),
            snapshot.primary.get("hp_max")
        ) {
            let pct = if *maxv > 0.0 { cur / maxv } else { 0.0 };
            assert!((pct-exp).abs() < 1e-6, "derived hp_percentage expected {}, got {}", exp, pct);
        }
    }
}

fn find_input_value(parsed: &serde_json::Value, dim: &str) -> Option<f64> {
    parsed.get("inputs")?.as_array()?.iter()
        .find(|it| it.get("dimension").and_then(|v| v.as_str()) == Some(dim))
        .and_then(|it| it.get("value").and_then(|v| v.as_f64()))
}


