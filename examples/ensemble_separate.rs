//! Separate a song with an ensemble preset (multiple checkpoints combined by a
//! waveform operator).
//!
//! The command-line surface of this crate is its examples (the batch CLI lives
//! in the separate `karaoke-cli` project); this one runs a documented preset
//! end to end and writes exactly the preset's stem set.
//!
//! ```sh
//! cargo run --release --example ensemble_separate -- \
//!   --preset vocal_balanced --input song.wav --out out_dir \
//!   --member model_bs_roformer_ep_317_sdr_12.9755.safetensors:configs/bs_roformer_viperx_1297.yaml \
//!   --member voc_fv7.safetensors:configs/voc_fv7.yaml
//! ```
//!
//! `--member` is repeatable and must be given once per preset model, in the
//! preset's documented order (`checkpoint.safetensors:profile.yaml`). Presets
//! and their exact composition are documented in `docs/ensemble-presets.md`;
//! the built-in ids are listed by `--list-presets`.

use std::path::PathBuf;
use std::time::Instant;

use candle_roformer::{
    separate_with_preset, DeviceChoice, EnsemblePreset, LocalSeparationAssets, Separator,
};

fn usage() -> &'static str {
    "usage: ensemble_separate --preset <id> --input <input.wav> --out <out_dir> \
     --member <checkpoint.safetensors>:<profile.yaml> [--member ...]\n\
     \x20      ensemble_separate --list-presets"
}

fn die(message: &str, code: i32) -> ! {
    eprintln!("{}", message);
    eprintln!("{}", usage());
    std::process::exit(code);
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.iter().any(|a| a == "--list-presets") {
        for id in EnsemblePreset::BUILTINS {
            match EnsemblePreset::builtin(id) {
                Ok(preset) => println!(
                    "{} ({}) — {}; algorithm={}; stems={}; models={}",
                    preset.id,
                    preset.name,
                    preset.description,
                    preset.algorithm.as_str(),
                    preset.stems.join("+"),
                    preset
                        .models
                        .iter()
                        .map(|m| m.checkpoint.as_str())
                        .collect::<Vec<_>>()
                        .join(", ")
                ),
                Err(e) => println!("{id}: {e}"),
            }
        }
        return;
    }

    let mut preset_id: Option<String> = None;
    let mut input: Option<PathBuf> = None;
    let mut out_dir: Option<PathBuf> = None;
    let mut members: Vec<(PathBuf, PathBuf)> = Vec::new();

    let mut i = 0;
    while i < args.len() {
        let take = |i: usize, flag: &str| -> String {
            args.get(i + 1)
                .map(|v| v.to_string())
                .unwrap_or_else(|| die(&format!("{flag} needs a value"), 2))
        };
        match args[i].as_str() {
            "--preset" => preset_id = Some(take(i, "--preset")),
            "--input" => input = Some(PathBuf::from(take(i, "--input"))),
            "--out" => out_dir = Some(PathBuf::from(take(i, "--out"))),
            "--member" => {
                let spec = take(i, "--member");
                let (checkpoint, profile) = spec.split_once(':').unwrap_or_else(|| {
                    die(
                        "--member must be <checkpoint.safetensors>:<profile.yaml>",
                        2,
                    )
                });
                members.push((PathBuf::from(checkpoint), PathBuf::from(profile)));
            }
            other => die(&format!("unknown argument `{}`", other), 2),
        }
        i += 2;
    }

    let preset_id = preset_id.unwrap_or_else(|| die("--preset is required", 2));
    let input = input.unwrap_or_else(|| die("--input is required", 2));
    let out_dir = out_dir.unwrap_or_else(|| die("--out is required", 2));

    let preset = match EnsemblePreset::builtin(&preset_id) {
        Ok(p) => p,
        Err(e) => die(&format!("preset error: {}", e), 2),
    };
    if members.len() != preset.models.len() {
        die(
            &format!(
                "preset `{}` needs {} --member arguments ({}) but {} were supplied",
                preset.id,
                preset.models.len(),
                preset
                    .models
                    .iter()
                    .map(|m| m.checkpoint.as_str())
                    .collect::<Vec<_>>()
                    .join(", "),
                members.len()
            ),
            2,
        );
    }

    println!(
        "preset={} name={:?} algorithm={} weights={:?} stems={:?}",
        preset.id,
        preset.name,
        preset.algorithm.as_str(),
        preset.weights,
        preset.stems
    );
    for (member, (checkpoint, profile)) in preset.models.iter().zip(members.iter()) {
        println!(
            "member checkpoint={} profile={} yaml={} role={} bytes={}",
            member.checkpoint,
            member.profile,
            profile.display(),
            member.role,
            std::fs::metadata(checkpoint).map(|m| m.len()).unwrap_or(0)
        );
    }

    let mut separators = Vec::with_capacity(members.len());
    for (checkpoint, profile) in &members {
        let assets = LocalSeparationAssets::new(checkpoint, profile);
        let t0 = Instant::now();
        match Separator::load(&assets, DeviceChoice::Cpu) {
            Ok(s) => {
                let cfg = s.config();
                println!(
                    "loaded family={:?} dim={} depth={} sample_rate={} in {:.3}s",
                    cfg.family(),
                    cfg.dim,
                    cfg.depth,
                    cfg.sample_rate,
                    t0.elapsed().as_secs_f64()
                );
                separators.push(s);
            }
            Err(e) => {
                eprintln!("load error ({}): {}", checkpoint.display(), e);
                std::process::exit(1);
            }
        }
    }

    let audio = match candle_roformer::wav::read_audio(&input) {
        Ok(a) => a,
        Err(e) => {
            eprintln!("read error: {}", e);
            std::process::exit(1);
        }
    };
    println!(
        "input={} frames={} channels={} sample_rate={}",
        input.display(),
        audio.frames(),
        audio.channels,
        audio.sample_rate
    );

    let t1 = Instant::now();
    let stems = match separate_with_preset(&separators, &preset, &audio) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("ensemble separation error: {}", e);
            std::process::exit(1);
        }
    };
    let secs = t1.elapsed().as_secs_f64();

    if let Err(e) = std::fs::create_dir_all(&out_dir) {
        eprintln!("cannot create {}: {}", out_dir.display(), e);
        std::process::exit(1);
    }
    // Exactly the preset's documented stem set, nothing else.
    let produced: Vec<&str> = stems.names();
    let expected: Vec<&str> = preset.stems.iter().map(|s| s.as_str()).collect();
    if produced != expected {
        eprintln!(
            "stem set mismatch: produced {:?}, preset documents {:?}",
            produced, expected
        );
        std::process::exit(1);
    }
    for (name, buffer) in stems.iter() {
        let path = out_dir.join(format!("{}.wav", name));
        if let Err(e) = candle_roformer::wav::write_wav_f32(&path, buffer) {
            eprintln!("write error ({}): {}", path.display(), e);
            std::process::exit(1);
        }
        println!(
            "stem={} path={} frames={} channels={} sample_rate={}",
            name,
            path.display(),
            buffer.frames(),
            buffer.channels,
            buffer.sample_rate
        );
    }
    println!("separation_secs={:.3}", secs);
}
