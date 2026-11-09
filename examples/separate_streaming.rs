//! Separate an audio file into vocals/instrumental using bounded-memory
//! chunked processing.
//!
//! ```sh
//! cargo run --release --example separate -- \
//!   --chunk_duration 8 input.wav model.safetensors out_dir
//! ```
//!
//! `model.safetensors` must have a sidecar `model.yaml` (or pass config via
//! the checkpoint's `.yaml`). No network access is performed.

use std::path::PathBuf;

use candle_roformer::{DeviceChoice, LocalSeparationAssets, Separator};

fn main() {
    let mut args: Vec<String> = std::env::args().skip(1).collect();
    let mut chunk_duration: f64 = 8.0;
    // Parse --chunk_duration flag (anywhere).
    let mut i = 0;
    while i < args.len() {
        if args[i] == "--chunk_duration" {
            if i + 1 >= args.len() {
                eprintln!("--chunk_duration requires a value");
                std::process::exit(2);
            }
            chunk_duration = match args[i + 1].parse() {
                Ok(v) => v,
                Err(_) => {
                    eprintln!("invalid --chunk_duration: {}", args[i + 1]);
                    std::process::exit(2);
                }
            };
            args.drain(i..i + 2);
        } else {
            i += 1;
        }
    }
    if args.len() < 3 {
        eprintln!(
            "usage: separate [--chunk_duration <seconds>] <input.wav> <model.safetensors> <out_dir>"
        );
        std::process::exit(2);
    }
    let input = PathBuf::from(&args[0]);
    let ckpt = PathBuf::from(&args[1]);
    let out_dir = PathBuf::from(&args[2]);
    let _ = std::fs::create_dir_all(&out_dir);

    let assets = LocalSeparationAssets::new(&ckpt, ckpt.with_extension("yaml"));
    let separator = match Separator::load(&assets, DeviceChoice::Cpu) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("load error: {}", e);
            std::process::exit(1);
        }
    };

    let vocals = out_dir.join("vocals.wav");
    let instrumental = out_dir.join("instrumental.wav");

    match separator.separate_file(&input, &vocals, &instrumental, chunk_duration) {
        Ok(()) => {
            println!("wrote {} and {}", vocals.display(), instrumental.display());
        }
        Err(e) => {
            eprintln!("separation error: {}", e);
            std::process::exit(1);
        }
    }
}
