//! Export the runtime's exact tensor schema for offline checkpoint conversion.
//! cargo run --quiet --example checkpoint_schema -- configs/bs_roformer_hyperace_v2_inst.yaml schema.json
use std::collections::BTreeMap;

use candle_roformer::SeparatorConfig;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 3 {
        return Err("usage: checkpoint_schema <profile.yaml> <schema.json>".into());
    }
    let config = SeparatorConfig::from_yaml_file(&args[1])?;
    config.validate()?;
    let tensors: BTreeMap<_, _> = candle_roformer::model::expected_key_shapes(&config)
        .into_iter()
        .collect();
    std::fs::write(&args[2], serde_json::to_vec_pretty(&tensors)?)?;
    Ok(())
}
