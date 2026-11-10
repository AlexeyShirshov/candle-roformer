use std::fs;
use std::path::PathBuf;

fn main() {
    let out = PathBuf::from(std::env::args().nth(1).unwrap_or_else(|| ".".into()));
    fs::create_dir_all(&out).unwrap();
    // Reuse the same config as tests/streaming_chunked.rs::tiny_config
    let cfg = candle_roformer::config::SeparatorConfig {
        dim: 8, depth: 1, stereo: true, num_stems: 1, num_bands: 4,
        dim_head: 4, heads: 1, flash_attn: false, dim_freqs_in: 9,
        sample_rate: 44100, stft_n_fft: 16, stft_hop_length: 4,
        stft_win_length: 16, stft_normalized: false, mask_estimator_depth: 2,
        use_torch_checkpoint: false, time_transformer_depth: 1,
        freq_transformer_depth: 1, mlp_expansion_factor: 4,
        linear_transformer_depth: 0, norm_output: true, skip_connection: false,
        zero_dc: true,
    };
    // write yaml
    let yaml = format!(
        "model:\n  dim: {}\n  depth: {}\n  stereo: true\n  num_stems: 1\n  time_transformer_depth: 1\n  freq_transformer_depth: 1\n  num_bands: {}\n  dim_head: {}\n  heads: {}\n  flash_attn: false\n  dim_freqs_in: {}\n  sample_rate: 44100\n  stft_n_fft: {}\n  stft_hop_length: {}\n  stft_win_length: {}\n  stft_normalized: false\n  mask_estimator_depth: 2\n  use_torch_checkpoint: false\n  mlp_expansion_factor: 4\n  linear_transformer_depth: 0\n  norm_output: true\n  skip_connection: false\n  zero_dc: true\n",
        cfg.dim, cfg.depth, cfg.num_bands, cfg.dim_head, cfg.heads, cfg.dim_freqs_in,
        cfg.stft_n_fft, cfg.stft_hop_length, cfg.stft_win_length
    );
    fs::write(out.join("tiny.yaml"), yaml).unwrap();

    // write safetensors
    use candle_core::{DType, Device};
    use candle_nn::{VarBuilder, VarMap};
    let dev = Device::Cpu;
    let varmap = VarMap::new();
    let vb = VarBuilder::from_varmap(&varmap, DType::F32, &dev);
    let _ = candle_roformer::model::MelBandRoformer::new(&cfg, vb).unwrap();
    varmap.save(out.join("tiny.safetensors")).unwrap();
    println!("wrote {}", out.display());
}

