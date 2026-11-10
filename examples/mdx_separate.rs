//! Separate an audio file with the pinned MDX-Net ONNX profile on the CPU.
//!
//! ```sh
//! cargo run --release --example mdx_separate -- \
//!   input.wav /path/to/mdx-models out_dir
//! ```
//!
//! The model directory must contain the published
//! `UVR-MDX-NET-Inst_HQ_3.onnx` (66,759,214 bytes). Its SHA-256 is verified
//! before inference; no network access is performed and weights are never
//! bundled or downloaded.

use std::path::PathBuf;
use std::time::Instant;

use candle_roformer::mdx::{uvr_mdx_net_inst_hq_3, MdxSeparator};
use candle_roformer::wav;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.len() < 3 {
        eprintln!("usage: mdx_separate <input.wav> <model_dir> <out_dir>");
        std::process::exit(2);
    }
    let input = PathBuf::from(&args[0]);
    let model_dir = PathBuf::from(&args[1]);
    let out_dir = PathBuf::from(&args[2]);
    let _ = std::fs::create_dir_all(&out_dir);

    let profile = uvr_mdx_net_inst_hq_3();
    println!(
        "mdx profile=\"{}\" model={} sha256={}",
        profile.name, profile.model_file, profile.model_sha256
    );

    let separator = match MdxSeparator::load_from_dir(profile, &model_dir) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("load error: {}", e);
            std::process::exit(1);
        }
    };
    let audio = match wav::read_audio(&input) {
        Ok(a) => a,
        Err(e) => {
            eprintln!("read error: {}", e);
            std::process::exit(1);
        }
    };

    let started = Instant::now();
    let stems = match separator.separate(&audio) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("separation error: {}", e);
            std::process::exit(1);
        }
    };
    let elapsed = started.elapsed();

    for (name, buffer) in stems.iter() {
        let path = out_dir.join(format!("{}.wav", name.to_lowercase()));
        if let Err(e) = wav::write_wav_f32(&path, buffer) {
            eprintln!("write error ({}): {}", path.display(), e);
            std::process::exit(1);
        }
        println!(
            "stem={} frames={} channels={} sample_rate={} path={}",
            name,
            buffer.frames(),
            buffer.channels,
            buffer.sample_rate,
            path.display()
        );
    }
    println!(
        "mdx_separate elapsed_secs={:.3} cpu_only=true",
        elapsed.as_secs_f64()
    );
}
