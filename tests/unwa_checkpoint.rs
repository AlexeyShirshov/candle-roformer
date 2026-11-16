//! Opt-in smoke test: no weights downloaded or bundled.
use candle_roformer::{AudioBuffer, DeviceChoice, LocalSeparationAssets, Separator};

#[test]
#[ignore = "requires CANDLE_ROFORMER_UNWA_SAFETENSORS (converted local checkpoint)"]
fn real_unwa_checkpoint_loads_and_separates() {
    let checkpoint = std::env::var("CANDLE_ROFORMER_UNWA_SAFETENSORS")
        .expect("set CANDLE_ROFORMER_UNWA_SAFETENSORS");
    let config = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("configs/unwa_inst_v1e_plus.yaml");
    let separator = Separator::load(
        &LocalSeparationAssets::new(checkpoint, config),
        DeviceChoice::Cpu,
    ).unwrap();
    let audio = AudioBuffer {
        sample_rate: 44100,
        channels: 2,
        planar: (0..4410 * 2)
            .map(|i| (i as f32 * 0.05).sin() * 0.1)
            .collect(),
    };
    let stems = separator.separate(&audio).unwrap();
    for stem in [&stems.vocals, &stems.instrumental] {
        assert_eq!(stem.planar.len(), audio.planar.len());
        assert_eq!(stem.channels, audio.channels);
        assert_eq!(stem.sample_rate, audio.sample_rate);
        assert!(stem.planar.iter().all(|v| v.is_finite()));
    }
}
