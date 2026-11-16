use std::collections::BTreeMap;

use candle_core::{DType, Device, Tensor};
use candle_nn::VarBuilder;
use candle_roformer::config::{InferenceConfig, ModelFamily, SeparatorConfig};
use candle_roformer::model::{bs_roformer, hyperace::HyperAce};

fn config() -> SeparatorConfig {
    SeparatorConfig::from_yaml_str(include_str!("../configs/bs_roformer_hyperace_v2_inst.yaml"))
        .unwrap()
}

#[test]
fn published_profile_selects_hyperace_not_standard_bs() {
    let cfg = config();
    cfg.validate().unwrap();
    assert_eq!(cfg.family(), ModelFamily::BSRoformerHyperAceV2);
    assert_eq!((cfg.dim, cfg.depth, cfg.stft_hop_length), (256, 12, 512));
    assert_eq!(cfg.freqs_per_bands.as_ref().unwrap().len(), 62);
    let infer = InferenceConfig::from_yaml_str(include_str!(
        "../configs/bs_roformer_hyperace_v2_inst.yaml"
    ))
    .unwrap();
    assert_eq!(infer.segment_size, Some(960000));
    assert_eq!((infer.batch_size, infer.num_overlap), (2, 4));
}

#[test]
fn schema_preserves_trunk_and_mlp_and_adds_complete_head() {
    let mut cfg = config();
    let hyper: BTreeMap<_, _> = bs_roformer::expected_key_shapes(&cfg).into_iter().collect();
    assert_eq!(hyper.len(), 1170, "published HyperACE v2 Inst state dict");
    cfg.family = Some(ModelFamily::BSRoformer);
    let ordinary = bs_roformer::expected_key_shapes(&cfg);
    for (name, shape) in &ordinary {
        assert_eq!(hyper.get(name), Some(shape), "{name}");
    }
    assert!(hyper.len() > ordinary.len());
    assert_eq!(
        hyper["mask_estimators.0.segm.hyperace.high_order_branch.0.ahc.adaptive_hyperedge_gen.global_proto"],
        vec![32, 256],
    );
    assert_eq!(
        hyper["mask_estimators.0.segm.upsample_head.block4.out_conv.blocks.1.tdf.5.weight"],
        vec![992, 248],
    );
    assert_eq!(
        hyper["mask_estimators.0.segm.upsample_head.final_conv.weight"],
        vec![4, 16, 3, 3],
    );
    assert!(!hyper
        .keys()
        .any(|k| k.contains("running_mean") || k.contains("running_var")));
    cfg.family = Some(ModelFamily::BSRoformerHyperAceV2);
    cfg.stereo = false;
    assert!(cfg.validate().is_err());
}

/// Generate the fixture with tools/hyperace_reference.py using the pinned
/// author's source and converted real weights. No synthetic self-comparison.
#[test]
#[ignore = "requires HYPERACE_SAFETENSORS and HYPERACE_REFERENCE"]
fn real_checkpoint_and_head_match_pytorch() {
    let weights = std::env::var("HYPERACE_SAFETENSORS").expect("HYPERACE_SAFETENSORS");
    let reference = std::env::var("HYPERACE_REFERENCE").expect("HYPERACE_REFERENCE");
    let cfg = config();
    let mmap = unsafe { candle_core::safetensors::MmapedSafetensors::new(&weights) }.unwrap();
    let actual: BTreeMap<_, _> = mmap
        .tensors()
        .into_iter()
        .map(|(name, tensor)| (name, tensor.shape().to_vec()))
        .collect();
    let expected: BTreeMap<_, _> = bs_roformer::expected_key_shapes(&cfg).into_iter().collect();
    assert_eq!(
        actual, expected,
        "real checkpoint must exactly match schema"
    );
    let vb = unsafe { VarBuilder::from_mmaped_safetensors(&[weights], DType::F32, &Device::Cpu) }
        .unwrap();
    let model = HyperAce::new(256, 62, 1025, vb.pp("mask_estimators.0.segm")).unwrap();
    let fixture = candle_core::safetensors::load(reference, &Device::Cpu).unwrap();
    for suffix in ["even", "odd"] {
        let input = &fixture[&format!("input_{suffix}")];
        let expected = &fixture[&format!("output_{suffix}")];
        let output = model.forward(input).unwrap();
        compare(expected, &output, suffix);
    }
    let full = candle_roformer::model::BSRoformer::new(&cfg, vb).unwrap();
    let output = full.forward(&fixture["audio"]).unwrap();
    compare(&fixture["separated"], &output, "full forward");
    drop(full);
    drop(model);
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("configs/bs_roformer_hyperace_v2_inst.yaml");
    let assets = candle_roformer::LocalSeparationAssets::new(
        std::env::var("HYPERACE_SAFETENSORS").unwrap(),
        path,
    );
    let separator = candle_roformer::Separator::load(&assets, candle_roformer::DeviceChoice::Cpu)
        .expect("public loader must select the changed head and accept instrument target");
    assert_eq!(
        separator.config().family(),
        ModelFamily::BSRoformerHyperAceV2
    );
}

fn compare(expected: &Tensor, output: &Tensor, label: &str) {
    assert_eq!(expected.dims(), output.dims());
    let a = expected.flatten_all().unwrap().to_vec1::<f32>().unwrap();
    let b = output.flatten_all().unwrap().to_vec1::<f32>().unwrap();
    let mut max = 0f64;
    let mut sum = 0f64;
    let mut energy = 0f64;
    for (a, b) in a.iter().zip(&b) {
        assert!(b.is_finite());
        let d = *a as f64 - *b as f64;
        max = max.max(d.abs());
        sum += d * d;
        energy += (*a as f64).powi(2);
    }
    let rmse = (sum / a.len() as f64).sqrt();
    let relative = (sum / energy.max(1e-20)).sqrt();
    eprintln!("{label}: max_abs={max:.8e}, RMSE={rmse:.8e}, relative_L2={relative:.8e}");
    assert!(
        max < 1e-3 && relative < 1e-3,
        "{label}: numerical parity failed"
    );
}
