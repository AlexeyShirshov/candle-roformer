//! Separate a stereo file with a converted VR Arch (UVR) checkpoint.
//!
//! ```sh
//! cargo run --release --example vr_separate -- \
//!   model.safetensors input.wav out_dir
//! ```
//!
//! The checkpoint must be converted offline first:
//! `python tools/convert_vr_arch.py 2_HP-UVR.pth --out models/`.

use std::path::{Path, PathBuf};

use candle_roformer::model::vr_pipeline::{VrProfile, VrSeparator};
use candle_roformer::wav::{read_audio, write_wav_f32, AudioBuffer};

fn usage() -> &'static str {
    "usage: vr_separate <checkpoint.safetensors> <input.wav> <out_dir>"
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.len() != 3 {
        eprintln!("{}", usage());
        std::process::exit(2);
    }
    let checkpoint = PathBuf::from(&args[0]);
    let input = PathBuf::from(&args[1]);
    let out_dir = PathBuf::from(&args[2]);

    let audio = match read_audio(&input) {
        Ok(audio) => audio,
        Err(err) => {
            eprintln!("read error: {err}");
            std::process::exit(1);
        }
    };
    if audio.channels != 2 {
        eprintln!("input must be stereo, got {} channels", audio.channels);
        std::process::exit(2);
    }
    if audio.sample_rate != 44_100 {
        eprintln!("input must be 44100 Hz, got {}", audio.sample_rate);
        std::process::exit(2);
    }

    let separator = match VrSeparator::load(&checkpoint, VrProfile::four_band_v2()) {
        Ok(separator) => separator,
        Err(err) => {
            eprintln!("load error: {err}");
            std::process::exit(1);
        }
    };

    let frames = audio.frames();
    let (left, right) = audio.planar.split_at(frames);
    let started = std::time::Instant::now();
    let (primary, secondary) = match separator.separate(left, right) {
        Ok(stems) => stems,
        Err(err) => {
            eprintln!("separation error: {err}");
            std::process::exit(1);
        }
    };
    let seconds = started.elapsed().as_secs_f64();

    if let Err(err) = std::fs::create_dir_all(&out_dir) {
        eprintln!("cannot create {}: {err}", out_dir.display());
        std::process::exit(1);
    }
    for (name, interleaved) in [("instrumental", primary), ("vocals", secondary)] {
        let mut planar = Vec::with_capacity(interleaved.len());
        for ch in 0..2 {
            planar.extend(interleaved.iter().skip(ch).step_by(2));
        }
        let buffer = AudioBuffer {
            sample_rate: 44_100,
            channels: 2,
            planar,
        };
        let path = Path::new(&out_dir).join(format!("{name}.wav"));
        if let Err(err) = write_wav_f32(&path, &buffer) {
            eprintln!("write error {}: {err}", path.display());
            std::process::exit(1);
        }
        println!("{name}: {}", path.display());
    }
    println!("separation_secs={seconds:.3}");
}
