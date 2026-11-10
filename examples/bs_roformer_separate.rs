//! Separate a song with a local BS-RoFormer checkpoint (safetensors + YAML).
//!
//! The checkpoint must be the offline-converted form of the published UVR
//! `.ckpt` (see `tools/convert_bs_roformer.py`); the profile is the UVR model
//! YAML (`model:` section with `freqs_per_bands`, `stft_*`, `inference:`).
//!
//! ```sh
//! cargo run --release --example bs_roformer_separate -- \
//!   model_bs_roformer.safetensors model_bs_roformer.yaml song.wav out_dir
//! ```
//!
//! The loader picks the architecture from the profile (`family:` or
//! `freqs_per_bands`), validates every tensor name and shape against the
//! profile, then separates on CPU. No network access is performed.

use std::path::PathBuf;
use std::time::Instant;

use candle_roformer::{DeviceChoice, LocalSeparationAssets, Separator};

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.len() != 4 {
        eprintln!(
            "usage: bs_roformer_separate <checkpoint.safetensors> <config.yaml> <input.wav> <out_dir>"
        );
        std::process::exit(2);
    }
    let ckpt = PathBuf::from(&args[0]);
    let config = PathBuf::from(&args[1]);
    let input = PathBuf::from(&args[2]);
    let out_dir = PathBuf::from(&args[3]);
    let _ = std::fs::create_dir_all(&out_dir);

    let bytes = std::fs::metadata(&ckpt).map(|m| m.len()).unwrap_or(0);
    println!(
        "checkpoint={} bytes={} config={}",
        ckpt.display(),
        bytes,
        config.display()
    );

    let assets = LocalSeparationAssets::new(&ckpt, &config);
    let t0 = Instant::now();
    let separator = match Separator::load(&assets, DeviceChoice::Cpu) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("load error: {}", e);
            std::process::exit(1);
        }
    };
    let load_secs = t0.elapsed().as_secs_f64();
    let cfg = separator.config();
    let band_sum: usize = cfg
        .freqs_per_bands
        .as_ref()
        .map(|v| v.iter().sum())
        .unwrap_or(0);
    println!(
        "family={:?} dim={} depth={} bands={} freqs_per_bands_sum={} stems={} sample_rate={} n_fft={} hop={} win={}",
        cfg.family(),
        cfg.dim,
        cfg.depth,
        cfg.num_bands,
        band_sum,
        cfg.num_stems,
        cfg.sample_rate,
        cfg.stft_n_fft,
        cfg.stft_hop_length,
        cfg.stft_win_length
    );
    println!("load_secs={:.3}", load_secs);
    let inf = separator.inference_config();
    println!(
        "inference num_overlap={} normalization={} amplification={} chunk_hops={} step_size={:?}",
        inf.num_overlap, inf.normalization, inf.amplification, inf.chunk_hops, inf.step_size
    );

    let audio = match candle_roformer::wav::read_audio(&input) {
        Ok(a) => a,
        Err(e) => {
            eprintln!("read error: {}", e);
            std::process::exit(1);
        }
    };
    println!(
        "input frames={} channels={} sample_rate={}",
        audio.frames(),
        audio.channels,
        audio.sample_rate
    );

    let t1 = Instant::now();
    let stems = match separator.separate(&audio) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("separation error: {}", e);
            std::process::exit(1);
        }
    };
    let sep_secs = t1.elapsed().as_secs_f64();

    let vocals = out_dir.join("vocals.wav");
    let instrumental = out_dir.join("instrumental.wav");
    if let Err(e) = candle_roformer::wav::write_wav_f32(&vocals, &stems.vocals) {
        eprintln!("write error: {}", e);
        std::process::exit(1);
    }
    if let Err(e) = candle_roformer::wav::write_wav_f32(&instrumental, &stems.instrumental) {
        eprintln!("write error: {}", e);
        std::process::exit(1);
    }
    for (name, buf) in stems.iter() {
        println!(
            "stem={} frames={} channels={} sample_rate={}",
            name,
            buf.frames(),
            buf.channels,
            buf.sample_rate
        );
    }
    println!("separation_secs={:.3}", sep_secs);
    println!("vocals={}", vocals.display());
    println!("instrumental={}", instrumental.display());
}
